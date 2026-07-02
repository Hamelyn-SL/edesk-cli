use anyhow::{bail, Context as _, Result};
use axoupdater::{AxoUpdater, AxoupdateError};
use clap::Args;

use crate::update_check;

const CURRENT: &str = env!("CARGO_PKG_VERSION");
const RELEASES_URL: &str = "https://github.com/Hamelyn-SL/edesk-cli/releases/latest";

#[derive(Debug, Args)]
pub struct UpgradeArgs {
    /// Only check whether a newer release exists; do not install
    #[arg(long)]
    pub check: bool,
}

pub async fn run(args: UpgradeArgs) -> Result<()> {
    if args.check {
        return check_only().await;
    }

    if installed_via_homebrew() {
        bail!("this edesk is managed by Homebrew; update it with:\n  brew upgrade edesk");
    }

    let mut updater = AxoUpdater::new_for("edesk");
    match updater.load_receipt() {
        Ok(_) => {}
        Err(AxoupdateError::NoReceipt { .. }) => bail!(
            "no install receipt found — this edesk was not installed with the official installer.\n\
             Update it the same way it was installed, or reinstall with:\n  \
             curl -LsSf https://github.com/Hamelyn-SL/edesk-cli/releases/latest/download/edesk-installer.sh | sh"
        ),
        Err(err) => return Err(err).context("could not load the install receipt"),
    }

    eprintln!("Checking {RELEASES_URL} …");
    match updater.run().await.context("update failed")? {
        Some(result) => {
            let from = result
                .old_version
                .map(|v| v.to_string())
                .unwrap_or_else(|| CURRENT.to_string());
            eprintln!(
                "✓ Upgraded edesk {from} → {} (installed in {})",
                result.new_version, result.install_prefix
            );
        }
        None => eprintln!("edesk {CURRENT} is already up to date."),
    }
    Ok(())
}

/// Report-only path. Uses the same GitHub query as the passive daily notice
/// (honors EDESK_UPDATE_CHECK_URL, so it is testable against a mock).
async fn check_only() -> Result<()> {
    let Some(latest) = update_check::fetch_latest().await else {
        bail!("could not reach GitHub to check for releases");
    };
    if update_check::is_newer(&latest, CURRENT) {
        println!("Update available: {CURRENT} → {latest}");
        println!("Run `edesk upgrade` to install it (Homebrew: brew upgrade edesk).");
    } else {
        println!("edesk {CURRENT} is up to date (latest release: {latest}).");
    }
    Ok(())
}

/// Homebrew keeps binaries under a Cellar prefix; self-updating one would
/// fight the package manager.
fn installed_via_homebrew() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.canonicalize().ok())
        .is_some_and(|path| {
            path.components()
                .any(|part| part.as_os_str() == "Cellar" || part.as_os_str() == "homebrew")
        })
}
