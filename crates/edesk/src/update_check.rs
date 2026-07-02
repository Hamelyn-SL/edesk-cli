//! Passive new-version notice, gh-style: at most one HTTP request per day,
//! only when stderr is a TTY, never blocking the command's own output.
//!
//! Opt out with `EDESK_NO_UPDATE_CHECK=1`. Tests point
//! `EDESK_UPDATE_CHECK_URL` at a mock server.

use std::io::IsTerminal;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::config;

const CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);
const FETCH_TIMEOUT: Duration = Duration::from_secs(1);
const DEFAULT_API_BASE: &str = "https://api.github.com";
const REPO: &str = "Hamelyn-SL/edesk-cli";

#[derive(Debug, Default, Serialize, Deserialize)]
struct CheckState {
    checked_at: u64,
    latest: Option<String>,
}

/// Print a one-line notice on stderr if a newer release exists. Never errors,
/// never prints on failure. Call after the main command has finished.
pub async fn maybe_notice(quiet: bool) {
    if quiet
        || std::env::var_os("EDESK_NO_UPDATE_CHECK").is_some()
        || !std::io::stderr().is_terminal()
    {
        return;
    }
    let Some(latest) = latest_version().await else {
        return;
    };
    let current = env!("CARGO_PKG_VERSION");
    if is_newer(&latest, current) {
        eprintln!("\nA new release of edesk is available: {current} → {latest}");
        eprintln!("Update with `edesk upgrade` (or `brew upgrade edesk`).");
    }
}

/// Latest known version, from the daily cache or a fresh (1s-budget) fetch.
async fn latest_version() -> Option<String> {
    let path = config::config_dir().ok()?.join("update-check.json");
    let state: CheckState = std::fs::read_to_string(&path)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default();

    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    if now.saturating_sub(state.checked_at) < CHECK_INTERVAL.as_secs() {
        return state.latest;
    }

    let latest = fetch_latest().await;
    // Record the attempt even on failure so an unreachable network doesn't
    // retry on every invocation.
    let new_state = CheckState {
        checked_at: now,
        latest: latest.clone().or(state.latest),
    };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, serde_json::to_string(&new_state).ok()?);
    new_state.latest
}

/// Query the newest release tag. `None` on any failure.
pub async fn fetch_latest() -> Option<String> {
    let base =
        std::env::var("EDESK_UPDATE_CHECK_URL").unwrap_or_else(|_| DEFAULT_API_BASE.to_string());
    let url = format!(
        "{}/repos/{REPO}/releases/latest",
        base.trim_end_matches('/')
    );
    let client = edesk_client::reqwest::Client::builder()
        .user_agent(edesk_client::USER_AGENT)
        .timeout(FETCH_TIMEOUT)
        .build()
        .ok()?;
    let resp = client.get(url).send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let body: serde_json::Value = resp.json().await.ok()?;
    let tag = body.get("tag_name")?.as_str()?;
    Some(tag.trim_start_matches('v').to_string())
}

/// Compare dotted numeric versions; non-numeric parts compare as 0.
pub fn is_newer(candidate: &str, current: &str) -> bool {
    parse(candidate) > parse(current)
}

fn parse(version: &str) -> Vec<u64> {
    version
        .trim_start_matches('v')
        .split(['.', '-'])
        .map(|part| part.parse().unwrap_or(0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_comparison() {
        assert!(is_newer("0.2.0", "0.1.1"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(is_newer("v0.1.2", "0.1.1"));
        assert!(!is_newer("0.1.1", "0.1.1"));
        assert!(!is_newer("0.1.0", "0.1.1"));
    }
}
