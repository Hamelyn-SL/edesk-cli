//! Configuration file and credential storage.
//!
//! Config lives at `~/.config/edesk/config.toml` (XDG-style on every
//! platform, matching uv/ruff). The token is stored in the OS keychain via
//! `keyring` when available, falling back to a `0600` file next to the config
//! — the same strategy `gh` uses for headless environments.

use std::path::PathBuf;

use anyhow::{Context, Result};
use etcetera::BaseStrategy;
use toml_edit::DocumentMut;

const KEYRING_SERVICE: &str = "edesk-cli";
const KEYRING_USER: &str = "default";

pub fn config_dir() -> Result<PathBuf> {
    let strategy =
        etcetera::choose_base_strategy().context("could not determine home directory")?;
    Ok(strategy.config_dir().join("edesk"))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

fn token_file_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("token"))
}

/// Read the config file as an editable TOML document (empty doc if missing).
pub fn load() -> Result<DocumentMut> {
    let path = config_path()?;
    match std::fs::read_to_string(&path) {
        Ok(text) => text
            .parse::<DocumentMut>()
            .with_context(|| format!("invalid TOML in {}", path.display())),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(DocumentMut::new()),
        Err(err) => Err(err).with_context(|| format!("could not read {}", path.display())),
    }
}

pub fn save(doc: &DocumentMut) -> Result<()> {
    let path = config_path()?;
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(&path, doc.to_string())
        .with_context(|| format!("could not write {}", path.display()))
}

pub fn get_value(doc: &DocumentMut, key: &str) -> Option<String> {
    doc.get(key).and_then(|item| {
        item.as_str()
            .map(ToString::to_string)
            .or_else(|| item.as_value().map(|v| v.to_string()))
    })
}

/// Where a stored token was found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenSource {
    Keyring,
    TokenFile,
}

impl std::fmt::Display for TokenSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenSource::Keyring => write!(f, "system keychain"),
            TokenSource::TokenFile => write!(f, "token file"),
        }
    }
}

/// Look up a previously stored token: keychain first, then the token file.
pub fn stored_token() -> Option<(String, TokenSource)> {
    if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER) {
        if let Ok(token) = entry.get_password() {
            return Some((token, TokenSource::Keyring));
        }
    }
    let path = token_file_path().ok()?;
    let token = std::fs::read_to_string(path).ok()?;
    let token = token.trim().to_string();
    if token.is_empty() {
        None
    } else {
        Some((token, TokenSource::TokenFile))
    }
}

/// Store the token in the keychain, falling back to a 0600 file.
/// Returns where it ended up.
pub fn store_token(token: &str) -> Result<TokenSource> {
    if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER) {
        if entry.set_password(token).is_ok() {
            return Ok(TokenSource::Keyring);
        }
    }
    let path = token_file_path()?;
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(&path, token)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(TokenSource::TokenFile)
}

/// Remove the token from every storage location. Returns true if any existed.
pub fn delete_token() -> Result<bool> {
    let mut deleted = false;
    if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER) {
        if entry.delete_credential().is_ok() {
            deleted = true;
        }
    }
    let path = token_file_path()?;
    if path.exists() {
        std::fs::remove_file(&path)?;
        deleted = true;
    }
    Ok(deleted)
}
