use anyhow::{bail, Result};
use clap::Subcommand;
use edesk_client::api::TrackingLink;

use super::util;
use crate::context::Context;
use crate::output::{self, col, Column};

const COLUMNS: &[Column] = &[
    col("link", "tracking_link"),
    col("carrier", "tracking_carrier_name"),
];

#[derive(Debug, Subcommand)]
pub enum TrackingCmd {
    /// Show the tracking links of a sales order
    View {
        /// Sales order ID
        order_id: i64,
    },
    /// Add tracking links to a sales order
    Add {
        /// Sales order ID
        order_id: i64,
        /// Full tracking URL (repeatable)
        #[arg(long, required = true)]
        link: Vec<String>,
        /// Carrier name, paired with --link by position (max 64 chars)
        #[arg(long)]
        carrier: Vec<String>,
    },
    /// Replace all tracking links of a sales order
    Set {
        /// Sales order ID
        order_id: i64,
        /// Full tracking URL (repeatable)
        #[arg(long, required = true)]
        link: Vec<String>,
        /// Carrier name, paired with --link by position (max 64 chars)
        #[arg(long)]
        carrier: Vec<String>,
    },
    /// Delete ALL tracking links of a sales order
    Clear {
        /// Sales order ID
        order_id: i64,
        /// Skip the confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}

pub async fn run(ctx: &Context, cmd: TrackingCmd) -> Result<()> {
    let client = ctx.client()?;
    match cmd {
        TrackingCmd::View { order_id } => {
            let resp = client.get_tracking_links(order_id).await?;
            let links = resp
                .data
                .get("tracking_links")
                .and_then(serde_json::Value::as_array)
                .cloned()
                .unwrap_or_default();
            output::print_list(&ctx.global, links, None, COLUMNS)
        }
        TrackingCmd::Add {
            order_id,
            link,
            carrier,
        } => {
            let links = build_links(link, carrier)?;
            let resp = client.create_tracking_links(order_id, &links).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TrackingCmd::Set {
            order_id,
            link,
            carrier,
        } => {
            let links = build_links(link, carrier)?;
            let resp = client.update_tracking_links(order_id, &links).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TrackingCmd::Clear { order_id, yes } => {
            util::confirm(
                &format!("delete ALL tracking links of sales order {order_id}"),
                yes,
            )?;
            let resp = client.delete_tracking_links(order_id).await?;
            output::print_confirmation(&ctx.global, resp.data)
        }
    }
}

fn build_links(urls: Vec<String>, carriers: Vec<String>) -> Result<Vec<TrackingLink>> {
    if carriers.len() > urls.len() {
        bail!("more --carrier values than --link values");
    }
    Ok(urls
        .into_iter()
        .enumerate()
        .map(|(index, tracking_link)| TrackingLink {
            tracking_link,
            tracking_carrier_name: carriers.get(index).cloned(),
        })
        .collect())
}
