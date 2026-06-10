use anyhow::Result;
use clap::Subcommand;

use super::util::{self, ListOpts};
use crate::context::Context;
use crate::output::{self, col, Column};

const COLUMNS: &[Column] = &[
    col("id", "id"),
    col("name", "name"),
    col("type", "type"),
    col("country", "country"),
    col("currency", "currency"),
];

#[derive(Debug, Subcommand)]
pub enum ChannelCmd {
    /// List the channels connected to the account
    List {
        #[command(flatten)]
        list: ListOpts,
    },
}

pub async fn run(ctx: &Context, cmd: ChannelCmd) -> Result<()> {
    let client = ctx.client()?;
    match cmd {
        ChannelCmd::List { list } => {
            let (items, paginator) = util::paginate(&list, |page| {
                let client = &client;
                async move { client.list_channels(page).await }
            })
            .await?;
            output::print_list(&ctx.global, items, paginator, COLUMNS)
        }
    }
}
