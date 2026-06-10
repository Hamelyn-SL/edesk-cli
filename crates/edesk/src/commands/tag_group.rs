use anyhow::Result;
use clap::Subcommand;

use super::util::{self, ListOpts};
use crate::context::Context;
use crate::output::{self, col, Column};

const COLUMNS: &[Column] = &[
    col("id", "id"),
    col("name", "name"),
    col("entity", "entity_type"),
    col("type", "type"),
    col("owner", "user_id"),
    col("hidden", "hide"),
];

#[derive(Debug, Subcommand)]
pub enum TagGroupCmd {
    /// List tag groups (tags are created inside a group)
    List {
        #[command(flatten)]
        list: ListOpts,
    },
}

pub async fn run(ctx: &Context, cmd: TagGroupCmd) -> Result<()> {
    let client = ctx.client()?;
    match cmd {
        TagGroupCmd::List { list } => {
            let (items, paginator) = util::paginate(&list, |page| {
                let client = &client;
                async move { client.list_tag_groups(page).await }
            })
            .await?;
            output::print_list(&ctx.global, items, paginator, COLUMNS)
        }
    }
}
