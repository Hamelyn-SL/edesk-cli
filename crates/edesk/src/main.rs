mod cli;
mod commands;
mod config;
mod context;
mod jq;
mod output;

use std::process::ExitCode;

use clap::Parser;

/// Exit codes follow the `gh` convention: 0 success, 1 failure,
/// 2 usage error (clap handles it), 4 authentication required.
const EXIT_FAILURE: u8 = 1;
const EXIT_AUTH: u8 = 4;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = cli::Cli::parse();

    match commands::run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            if is_broken_pipe(&err) {
                return ExitCode::SUCCESS;
            }
            report(&err);
            if is_auth_error(&err) {
                ExitCode::from(EXIT_AUTH)
            } else {
                ExitCode::from(EXIT_FAILURE)
            }
        }
    }
}

fn report(err: &anyhow::Error) {
    use owo_colors::OwoColorize;
    let use_color = std::io::IsTerminal::is_terminal(&std::io::stderr())
        && std::env::var_os("NO_COLOR").is_none();
    let mut chain = err.chain();
    if let Some(first) = chain.next() {
        if use_color {
            eprintln!("{} {first}", "error:".red().bold());
        } else {
            eprintln!("error: {first}");
        }
    }
    for cause in chain {
        eprintln!("  caused by: {cause}");
    }
    if is_auth_error(err) {
        eprintln!("\nRun `edesk auth login` to store a valid API token.");
        eprintln!("Tokens are generated at https://dashboard.edesk.com/api-token");
    }
}

fn is_auth_error(err: &anyhow::Error) -> bool {
    err.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<edesk_client::Error>(),
            Some(edesk_client::Error::Auth { .. })
        )
    }) || err.downcast_ref::<context::MissingToken>().is_some()
}

fn is_broken_pipe(err: &anyhow::Error) -> bool {
    err.chain().any(|cause| {
        cause
            .downcast_ref::<std::io::Error>()
            .is_some_and(|io| io.kind() == std::io::ErrorKind::BrokenPipe)
    })
}
