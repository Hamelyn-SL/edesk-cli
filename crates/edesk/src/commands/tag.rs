use anyhow::{Context as _, Result};
use clap::{Args, Subcommand};
use edesk_client::api::TagRequest;
use serde_json::Value;

use super::util::{self, ListOpts};
use crate::context::Context;
use crate::output::{self, col, Column};

const COLUMNS: &[Column] = &[
    col("id", "id"),
    col("name", "name"),
    col("group", "tag_group_id"),
    col("color", "color"),
    col("icon", "icon"),
    col("active", "active"),
];

#[derive(Debug, Subcommand)]
pub enum TagCmd {
    /// List tags
    List {
        #[command(flatten)]
        list: ListOpts,
    },
    /// Show one tag
    View {
        /// Tag ID
        id: i64,
    },
    /// Create a tag
    Create(CreateArgs),
    /// Update a tag
    ///
    /// The API replaces the whole tag on update, so unspecified fields are
    /// preserved by reading the current tag first.
    Update(UpdateArgs),
    /// Delete a tag
    Delete {
        /// Tag ID
        id: i64,
        /// Skip the confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Tag name
    #[arg(long)]
    pub name: String,
    /// Tag group ID (see `edesk tag-group list`)
    #[arg(long)]
    pub group: i64,
    /// Hex color from the eDesk palette, without '#' (e.g. F44336)
    #[arg(long)]
    pub color: Option<String>,
    /// Icon name from the eDesk set (e.g. flag, star, truck)
    #[arg(long)]
    pub icon: Option<String>,
    /// Create the tag as inactive
    #[arg(long)]
    pub inactive: bool,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Tag ID
    pub id: i64,
    /// New name
    #[arg(long)]
    pub name: Option<String>,
    /// Move to tag group ID
    #[arg(long)]
    pub group: Option<i64>,
    /// New hex color, without '#'
    #[arg(long)]
    pub color: Option<String>,
    /// New icon name
    #[arg(long)]
    pub icon: Option<String>,
    /// Set active state
    #[arg(long, value_name = "BOOL")]
    pub active: Option<bool>,
}

pub async fn run(ctx: &Context, cmd: TagCmd) -> Result<()> {
    let client = ctx.client()?;
    match cmd {
        TagCmd::List { list } => {
            let (items, paginator) = util::paginate(&list, |page| {
                let client = &client;
                async move { client.list_tags(page).await }
            })
            .await?;
            output::print_list(&ctx.global, items, paginator, COLUMNS)
        }
        TagCmd::View { id } => {
            let resp = client.get_tag(id).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TagCmd::Create(args) => {
            let req = TagRequest {
                name: args.name,
                tag_group_id: args.group,
                color: args.color,
                icon: args.icon,
                active: args.inactive.then_some(false),
            };
            let resp = client.create_tag(&req).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TagCmd::Update(args) => {
            // Read-modify-write: the PUT is full-replace and requires
            // name + tag_group_id even when only changing e.g. the color.
            let current = client.get_tag(args.id).await?.data;
            let req = TagRequest {
                name: args
                    .name
                    .or_else(|| str_field(&current, "name"))
                    .context("current tag has no name; pass --name")?,
                tag_group_id: args
                    .group
                    .or_else(|| current.get("tag_group_id").and_then(Value::as_i64))
                    .context("current tag has no group; pass --group")?,
                color: args.color.or_else(|| str_field(&current, "color")),
                icon: args.icon.or_else(|| str_field(&current, "icon")),
                active: args
                    .active
                    .or_else(|| current.get("active").and_then(flexible_bool)),
            };
            let resp = client.update_tag(args.id, &req).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TagCmd::Delete { id, yes } => {
            util::confirm(&format!("delete tag {id}"), yes)?;
            let resp = client.delete_tag(id).await?;
            output::print_confirmation(&ctx.global, resp.data)
        }
    }
}

fn str_field(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

/// The API sometimes returns booleans as 0/1.
fn flexible_bool(value: &Value) -> Option<bool> {
    match value {
        Value::Bool(b) => Some(*b),
        Value::Number(n) => n.as_i64().map(|n| n != 0),
        _ => None,
    }
}
