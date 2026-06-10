use anyhow::{bail, Context as _, Result};
use clap::Subcommand;

use crate::config;
use crate::context::Context;
use crate::output;

#[derive(Debug, Subcommand)]
pub enum AuthCmd {
    /// Store an API token (validated against the API first)
    ///
    /// Tokens are generated at https://dashboard.edesk.com/api-token and are
    /// stored in the OS keychain when available, otherwise in a 0600 file
    /// under ~/.config/edesk/.
    Login {
        /// Read the token from standard input instead of prompting
        #[arg(long)]
        with_token: bool,
    },
    /// Show the active token's source and identity
    Status,
    /// Remove the stored token
    Logout,
}

pub async fn run(ctx: &Context, cmd: AuthCmd) -> Result<()> {
    match cmd {
        AuthCmd::Login { with_token } => login(ctx, with_token).await,
        AuthCmd::Status => status(ctx).await,
        AuthCmd::Logout => {
            if config::delete_token()? {
                eprintln!("✓ Stored token removed");
            } else {
                eprintln!("No stored token found");
            }
            Ok(())
        }
    }
}

async fn login(ctx: &Context, with_token: bool) -> Result<()> {
    let token = if with_token {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf.trim().to_string()
    } else {
        dialoguer::Password::new()
            .with_prompt("Paste your eDesk API token")
            .interact()
            .context("token prompt failed (pipe the token via `edesk auth login --with-token` in scripts)")?
    };
    if token.is_empty() {
        bail!("empty token");
    }

    // Validate before storing.
    let client = ctx.client_with_token(&token)?;
    let resp = client.whoami().await.context("token validation failed")?;
    let identity = resp.data["user"]["email"]
        .as_str()
        .or_else(|| resp.data["user"]["name"].as_str())
        .unwrap_or("unknown")
        .to_string();

    let location = config::store_token(&token)?;
    eprintln!("✓ Logged in as {identity} (token stored in {location})");
    Ok(())
}

async fn status(ctx: &Context) -> Result<()> {
    let (token, source) = ctx.resolve_token()?;
    let client = ctx.client_with_token(&token)?;
    let resp = client.whoami().await?;

    if ctx.global.json || ctx.global.jq.is_some() {
        return output::print_single(&ctx.global, resp.data);
    }

    let user = &resp.data["user"];
    eprintln!("Token source: {source}");
    eprintln!("Token: {}", redact(&token));
    eprintln!(
        "Logged in as: {} <{}> (user id {})",
        user["name"].as_str().unwrap_or("?"),
        user["email"].as_str().unwrap_or("?"),
        user["id"]
    );
    Ok(())
}

fn redact(token: &str) -> String {
    let chars: Vec<char> = token.chars().collect();
    if chars.len() <= 8 {
        return "*".repeat(chars.len());
    }
    let head: String = chars[..4].iter().collect();
    let tail: String = chars[chars.len() - 4..].iter().collect();
    format!("{head}…{tail}")
}
