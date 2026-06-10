use anyhow::Result;

use crate::cli::GlobalArgs;
use crate::config::{self, TokenSource};

/// No token could be resolved from flags, env, keychain or config.
#[derive(Debug)]
pub struct MissingToken;

impl std::fmt::Display for MissingToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "no API token found (checked --token, $EDESK_TOKEN, keychain and config)"
        )
    }
}

impl std::error::Error for MissingToken {}

/// Where the active token was resolved from, for `auth status`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTokenSource {
    Flag,
    Env,
    Stored(TokenSource),
}

impl std::fmt::Display for ActiveTokenSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveTokenSource::Flag => write!(f, "--token flag"),
            ActiveTokenSource::Env => write!(f, "EDESK_TOKEN environment variable"),
            ActiveTokenSource::Stored(source) => write!(f, "{source}"),
        }
    }
}

/// Resolved invocation context: global flags plus an authenticated client.
pub struct Context {
    pub global: GlobalArgs,
}

impl Context {
    pub fn new(global: GlobalArgs) -> Self {
        Self { global }
    }

    /// Resolve the token with gh-style precedence:
    /// flag > environment > keychain > config/token file.
    ///
    /// Note: clap's `env = "EDESK_TOKEN"` fills `global.token` from the
    /// environment too, so flag and env are both covered by the first arm.
    pub fn resolve_token(&self) -> Result<(String, ActiveTokenSource)> {
        // An empty token (e.g. `EDESK_TOKEN=""`) counts as unset.
        if let Some(token) = self.global.token.as_deref().filter(|t| !t.is_empty()) {
            // clap doesn't tell us whether the value came from the flag or the
            // env var; distinguish for `auth status` by comparing.
            let from_env = std::env::var("EDESK_TOKEN").is_ok_and(|env| env == token);
            let source = if from_env {
                ActiveTokenSource::Env
            } else {
                ActiveTokenSource::Flag
            };
            return Ok((token.to_string(), source));
        }
        if let Some((token, source)) = config::stored_token() {
            return Ok((token, ActiveTokenSource::Stored(source)));
        }
        Err(MissingToken.into())
    }

    fn resolve_base_url(&self) -> Result<Option<String>> {
        if let Some(url) = &self.global.base_url {
            return Ok(Some(url.clone()));
        }
        let doc = config::load()?;
        Ok(config::get_value(&doc, "base_url"))
    }

    /// Build an authenticated API client.
    pub fn client(&self) -> Result<edesk_client::Client> {
        let (token, _) = self.resolve_token()?;
        self.client_with_token(&token)
    }

    /// Build a client with an explicit token (used by `auth login` to
    /// validate a token before storing it).
    pub fn client_with_token(&self, token: &str) -> Result<edesk_client::Client> {
        let mut builder = edesk_client::Client::builder().token(token);
        if let Some(url) = self.resolve_base_url()? {
            builder = builder.base_url(url);
        }
        Ok(builder.build()?)
    }
}
