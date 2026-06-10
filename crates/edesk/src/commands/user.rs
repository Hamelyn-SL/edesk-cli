use anyhow::Result;
use clap::Subcommand;

use super::util::{self, ListOpts};
use crate::context::Context;
use crate::output::{self, col, Column};

const COLUMNS: &[Column] = &[
    col("id", "id"),
    col("name", "name"),
    col("email", "email"),
    col("username", "username"),
    col("role", "role"),
    col("active", "active"),
];

#[derive(Debug, Subcommand)]
pub enum UserCmd {
    /// List the agent users on the account
    List {
        #[command(flatten)]
        list: ListOpts,
    },
}

pub async fn run(ctx: &Context, cmd: UserCmd) -> Result<()> {
    let client = ctx.client()?;
    match cmd {
        UserCmd::List { list } => {
            let (items, paginator) = util::paginate(&list, |page| {
                let client = &client;
                async move { client.list_users(page).await }
            })
            .await?;
            output::print_list(&ctx.global, items, paginator, COLUMNS)
        }
    }
}
