use anyhow::{bail, Result};
use clap::Subcommand;

use crate::config;
use crate::context::Context;

/// Keys users may set. Kept explicit so typos fail loudly.
const KNOWN_KEYS: &[&str] = &["base_url"];

#[derive(Debug, Subcommand)]
pub enum ConfigCmd {
    /// Print a configuration value
    Get { key: String },
    /// Set a configuration value
    Set { key: String, value: String },
    /// Remove a configuration value
    Unset { key: String },
    /// Show the whole configuration file
    List,
    /// Print the path of the configuration file
    Path,
}

pub fn run(_ctx: &Context, cmd: ConfigCmd) -> Result<()> {
    match cmd {
        ConfigCmd::Get { key } => {
            let doc = config::load()?;
            match config::get_value(&doc, &key) {
                Some(value) => println!("{value}"),
                None => bail!("`{key}` is not set"),
            }
            Ok(())
        }
        ConfigCmd::Set { key, value } => {
            if !KNOWN_KEYS.contains(&key.as_str()) {
                bail!(
                    "unknown key `{key}` (known keys: {})",
                    KNOWN_KEYS.join(", ")
                );
            }
            let mut doc = config::load()?;
            doc[&key] = toml_edit::value(value);
            config::save(&doc)?;
            eprintln!("✓ {key} updated");
            Ok(())
        }
        ConfigCmd::Unset { key } => {
            let mut doc = config::load()?;
            if doc.remove(&key).is_none() {
                bail!("`{key}` is not set");
            }
            config::save(&doc)?;
            eprintln!("✓ {key} removed");
            Ok(())
        }
        ConfigCmd::List => {
            let doc = config::load()?;
            print!("{doc}");
            Ok(())
        }
        ConfigCmd::Path => {
            println!("{}", config::config_path()?.display());
            Ok(())
        }
    }
}
