use anyhow::{Context as _, Result};
use clap::Args;
use edesk_client::reqwest::Method;
use serde_json::Value;

use super::util;
use crate::context::Context;
use crate::output;

/// Make a raw, authenticated request to any eDesk API path.
///
/// Examples:
///   edesk api /whoami
///   edesk api /tickets --query page=2
///   edesk api /tags --method POST --body '{"name":"x","tag_group_id":1}'
///   edesk api /tickets --paginate --jq '.[].id'
#[derive(Debug, Args)]
pub struct ApiArgs {
    /// API path, e.g. /tickets (a full URL is also accepted)
    pub path: String,

    /// HTTP method
    #[arg(short = 'X', long, default_value = "GET")]
    pub method: String,

    /// Query parameter as KEY=VALUE (repeatable)
    #[arg(short = 'F', long = "query", value_name = "KEY=VALUE")]
    pub query: Vec<String>,

    /// Inline JSON request body
    #[arg(long, conflicts_with = "body_file")]
    pub body: Option<String>,

    /// Read the JSON request body from a file (use - for stdin)
    #[arg(long, value_name = "PATH")]
    pub body_file: Option<std::path::PathBuf>,

    /// Follow pagination and combine all pages into one array (GET only)
    #[arg(long)]
    pub paginate: bool,
}

pub async fn run(ctx: &Context, args: ApiArgs) -> Result<()> {
    let client = ctx.client()?;
    let method: Method = args
        .method
        .to_uppercase()
        .parse()
        .with_context(|| format!("invalid HTTP method `{}`", args.method))?;

    let query: Vec<(String, String)> = args
        .query
        .iter()
        .map(|pair| util::parse_key_value(pair))
        .collect::<Result<_>>()?;

    let body: Option<Value> = if args.body.is_some() || args.body_file.is_some() {
        Some(util::read_body(
            args.body.as_deref(),
            args.body_file.as_deref(),
        )?)
    } else {
        None
    };

    let path = normalize_path(&args.path);

    if args.paginate && method == Method::GET {
        let mut items: Vec<Value> = Vec::new();
        let mut page = 1u64;
        loop {
            let mut q = query.clone();
            q.push(("page".into(), page.to_string()));
            let resp = client.request(method.clone(), &path, &q, None).await?;
            match resp.data {
                Value::Array(batch) => {
                    if batch.is_empty() {
                        break;
                    }
                    items.extend(batch);
                }
                other => {
                    items.push(other);
                    break;
                }
            }
            match resp.paginator {
                Some(p) if p.has_more() => page += 1,
                _ => break,
            }
        }
        return finish(ctx, Value::Array(items));
    }

    // Single request: print the full envelope (data + paginator) verbatim.
    let resp = client.request(method, &path, &query, body.as_ref()).await?;
    let envelope = serde_json::to_value(&resp)?;
    finish(ctx, envelope)
}

fn finish(ctx: &Context, value: Value) -> Result<()> {
    if let Some(expr) = &ctx.global.jq {
        let outputs = crate::jq::apply(expr, value)?;
        for output in outputs {
            match output {
                Value::String(s) => println!("{s}"),
                other => output::print_json(&other)?,
            }
        }
        return Ok(());
    }
    output::print_json(&value)
}

fn normalize_path(path: &str) -> String {
    // Accept both `/tickets` and a full URL pasted from the docs.
    if let Some(rest) = path.strip_prefix(edesk_client::DEFAULT_BASE_URL) {
        rest.to_string()
    } else {
        path.to_string()
    }
}
