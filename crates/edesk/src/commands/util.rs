use std::future::Future;
use std::io::IsTerminal;

use anyhow::{bail, Context as _, Result};
use clap::Args;
use edesk_client::{ApiResponse, Page, Paginator};
use serde_json::Value;

/// Largest page size to request when batching. The API default is 20; it
/// honors larger values via the undocumented `itemsPerPage` parameter.
const MAX_PER_PAGE: u64 = 100;
/// Hard stop for `--all`, to guard against a server that keeps reporting
/// more pages forever.
const MAX_PAGES: u64 = 10_000;

/// Shared pagination flags for every `list` command.
#[derive(Debug, Clone, Args)]
pub struct ListOpts {
    /// Maximum number of items to fetch
    #[arg(short = 'L', long, default_value_t = 30, conflicts_with_all = ["all", "page"])]
    pub limit: u64,

    /// Fetch every item, paginating as needed
    #[arg(long)]
    pub all: bool,

    /// Fetch one specific page (1-based) instead of batching
    #[arg(long, conflicts_with = "all")]
    pub page: Option<u64>,

    /// Page size when using --page
    #[arg(long, requires = "page", value_name = "N")]
    pub per_page: Option<u64>,
}

/// Fetch items according to the list flags, calling `fetch` once per page.
pub async fn paginate<F, Fut>(opts: &ListOpts, fetch: F) -> Result<(Vec<Value>, Option<Paginator>)>
where
    F: Fn(Page) -> Fut,
    Fut: Future<Output = edesk_client::Result<ApiResponse>>,
{
    if let Some(page) = opts.page {
        let resp = fetch(Page {
            page: Some(page),
            items_per_page: opts.per_page,
        })
        .await?;
        return Ok((into_array(resp.data), resp.paginator));
    }

    let target = if opts.all { None } else { Some(opts.limit) };
    let per_page = target.map_or(MAX_PER_PAGE, |t| t.clamp(1, MAX_PER_PAGE));
    let mut items: Vec<Value> = Vec::new();
    let mut paginator = None;

    for page_number in 1..=MAX_PAGES {
        let resp = fetch(Page {
            page: Some(page_number),
            items_per_page: Some(per_page),
        })
        .await?;
        let batch = into_array(resp.data);
        let batch_was_empty = batch.is_empty();
        items.extend(batch);
        paginator = resp.paginator;

        if let Some(target) = target {
            if items.len() as u64 >= target {
                items.truncate(target as usize);
                break;
            }
        }
        match paginator {
            Some(p) if p.has_more() && !batch_was_empty => {}
            _ => break,
        }
    }
    Ok((items, paginator))
}

fn into_array(data: Value) -> Vec<Value> {
    match data {
        Value::Array(items) => items,
        Value::Null => Vec::new(),
        other => vec![other],
    }
}

/// Ask before a destructive action. `--yes` skips the prompt; outside a TTY
/// the prompt cannot be shown, so `--yes` is required.
pub fn confirm(action: &str, yes: bool) -> Result<()> {
    if yes {
        return Ok(());
    }
    if !std::io::stderr().is_terminal() {
        bail!("refusing to {action} without confirmation; pass --yes");
    }
    let confirmed = dialoguer::Confirm::new()
        .with_prompt(format!("{action}?"))
        .default(false)
        .interact()?;
    if !confirmed {
        bail!("cancelled");
    }
    Ok(())
}

/// Parse a `name=value` pair (for `--field` style flags).
pub fn parse_key_value(raw: &str) -> Result<(String, String)> {
    match raw.split_once('=') {
        Some((key, value)) if !key.is_empty() => Ok((key.to_string(), value.to_string())),
        _ => bail!("expected NAME=VALUE, got `{raw}`"),
    }
}

/// Parse `name=value` pairs into API custom fields.
pub fn parse_custom_fields(raw: &[String]) -> Result<Vec<edesk_client::api::CustomField>> {
    raw.iter()
        .map(|pair| {
            let (name, value) = parse_key_value(pair)?;
            Ok(edesk_client::api::CustomField {
                name,
                value: Value::String(value),
            })
        })
        .collect()
}

/// Resolve a JSON request body from `--body` / `--body-file` (with `-` for stdin).
pub fn read_body(body: Option<&str>, body_file: Option<&std::path::Path>) -> Result<Value> {
    let text = match (body, body_file) {
        (Some(inline), None) => inline.to_string(),
        (None, Some(path)) if path.as_os_str() == "-" => {
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
        (None, Some(path)) => std::fs::read_to_string(path)
            .with_context(|| format!("could not read {}", path.display()))?,
        (None, None) => bail!("provide a request body with --body or --body-file"),
        (Some(_), Some(_)) => bail!("--body and --body-file are mutually exclusive"),
    };
    serde_json::from_str(&text).context("request body is not valid JSON")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_key_value_pairs() {
        assert_eq!(
            parse_key_value("name=urgent").unwrap(),
            ("name".to_string(), "urgent".to_string())
        );
        assert_eq!(
            parse_key_value("note=a=b").unwrap(),
            ("note".to_string(), "a=b".to_string())
        );
        assert!(parse_key_value("no-separator").is_err());
        assert!(parse_key_value("=value").is_err());
    }

    #[test]
    fn read_body_parses_inline_json() {
        let body = read_body(Some(r#"{"a": 1}"#), None).unwrap();
        assert_eq!(body, serde_json::json!({"a": 1}));
        assert!(read_body(Some("not json"), None).is_err());
        assert!(read_body(None, None).is_err());
    }
}
