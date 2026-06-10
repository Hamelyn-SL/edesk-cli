use anyhow::Result;
use clap::{Args, Subcommand};
use edesk_client::api::ListContactsParams;

use super::util::{self, ListOpts};
use crate::context::Context;
use crate::output::{self, col, Column};

const COLUMNS: &[Column] = &[
    col("id", "id"),
    col("name", "full_name"),
    col("email", "email"),
    col("phone", "phone_number"),
    col("channel", "channel_id"),
];

#[derive(Debug, Subcommand)]
pub enum ContactCmd {
    /// List or search contacts
    List(ListArgs),
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[command(flatten)]
    pub list: ListOpts,
    /// Full-text search across contact fields
    #[arg(long, short = 'Q')]
    pub query: Option<String>,
    /// Filter by exact email
    #[arg(long)]
    pub email: Option<String>,
    /// Filter by full name
    #[arg(long)]
    pub name: Option<String>,
    /// Filter by phone number
    #[arg(long)]
    pub phone: Option<String>,
    /// Filter by consumer (contact) ID (the upstream API parameter name)
    #[arg(long, visible_alias = "contact-id", value_name = "ID")]
    pub consumer_id: Option<i64>,
}

pub async fn run(ctx: &Context, cmd: ContactCmd) -> Result<()> {
    let client = ctx.client()?;
    match cmd {
        ContactCmd::List(args) => {
            let params = ListContactsParams {
                page: Default::default(),
                query: args.query,
                consumer_id: args.consumer_id,
                email: args.email,
                name: args.name,
                phone_number: args.phone,
            };
            let (items, paginator) = util::paginate(&args.list, |page| {
                let mut params = params.clone();
                params.page = page;
                let client = &client;
                async move { client.list_contacts(&params).await }
            })
            .await?;
            output::print_list(&ctx.global, items, paginator, COLUMNS)
        }
    }
}
