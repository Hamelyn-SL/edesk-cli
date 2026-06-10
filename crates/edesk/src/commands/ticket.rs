use anyhow::Result;
use clap::{Args, Subcommand};
use edesk_client::api::{
    ContactRequest, CreateTicketRequest, ListTicketsParams, UpdateTicketRequest,
};

use super::util::{self, ListOpts};
use crate::context::Context;
use crate::output::{self, col, Column};

const COLUMNS: &[Column] = &[
    col("id", "id"),
    col("subject", "subject"),
    col("status", "status"),
    col("type", "type"),
    col("channel", "channel_id"),
    col("contact", "contact_id"),
    col("updated", "last_updated_at"),
];

#[derive(Debug, Subcommand)]
pub enum TicketCmd {
    /// List tickets
    List(ListArgs),
    /// Show one ticket
    View {
        /// Ticket ID
        id: i64,
    },
    /// Create a ticket
    Create(CreateArgs),
    /// Update a ticket
    Update(UpdateArgs),
    /// Update only the custom-field data of a ticket
    UpdateData {
        /// Ticket ID
        id: i64,
        /// Custom field as NAME=VALUE (repeatable)
        #[arg(
            short = 'f',
            long = "field",
            value_name = "NAME=VALUE",
            required = true
        )]
        custom_fields: Vec<String>,
    },
    /// Delete a ticket
    Delete {
        /// Ticket ID
        id: i64,
        /// Skip the confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[command(flatten)]
    pub list: ListOpts,
    /// Filter by status (Open, Pending, Closed, Spam, Unread, Read, Priority, ...)
    #[arg(long)]
    pub status: Option<String>,
    /// Filter by ticket type (OrderQuery, Refund, ReturnRequest, ...)
    #[arg(long = "type")]
    pub ticket_type: Option<String>,
    /// Filter by channel ID
    #[arg(long)]
    pub channel: Option<i64>,
    /// Filter by contact ID
    #[arg(long)]
    pub contact: Option<i64>,
    /// Filter by sales order ID
    #[arg(long = "sales-order")]
    pub sales_order: Option<i64>,
    /// Filter by marketplace order ID
    #[arg(long = "seller-order")]
    pub seller_order: Option<String>,
    /// Filter by owner (agent) user ID
    #[arg(long)]
    pub owner: Option<i64>,
    /// Created on or after this date (YYYY-MM-DD)
    #[arg(long, value_name = "DATE")]
    pub created_after: Option<String>,
    /// Created on or before this date (YYYY-MM-DD)
    #[arg(long, value_name = "DATE")]
    pub created_before: Option<String>,
    /// Updated on or after this date (YYYY-MM-DD)
    #[arg(long, value_name = "DATE")]
    pub updated_after: Option<String>,
    /// Updated on or before this date (YYYY-MM-DD)
    #[arg(long, value_name = "DATE")]
    pub updated_before: Option<String>,
    /// Sort key: id, created_at or last_updated_at
    #[arg(long, value_name = "KEY")]
    pub sort: Option<String>,
    /// Sort direction: asc or desc
    #[arg(long, value_name = "DIR")]
    pub direction: Option<String>,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Ticket subject
    #[arg(long)]
    pub subject: String,
    /// Channel ID the ticket belongs to
    #[arg(long)]
    pub channel: i64,
    /// Initial status (Open, Pending, Closed, Archived, Scheduled, Spam)
    #[arg(long, default_value = "Open")]
    pub status: String,
    /// Existing contact ID
    #[arg(long, visible_alias = "contact", conflicts_with = "contact_email")]
    pub contact_id: Option<i64>,
    /// Inline contact email (creates/links the contact)
    #[arg(long)]
    pub contact_email: Option<String>,
    /// Inline contact full name
    #[arg(long, requires = "contact_email")]
    pub contact_name: Option<String>,
    /// Link to a sales order ID
    #[arg(long = "sales-order")]
    pub sales_order: Option<i64>,
    /// Backdate creation (YYYY-MM-DD HH:MM:SS)
    #[arg(long, value_name = "DATETIME")]
    pub created_at: Option<String>,
    /// Custom field as NAME=VALUE (repeatable)
    #[arg(short = 'f', long = "field", value_name = "NAME=VALUE")]
    pub custom_fields: Vec<String>,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Ticket ID
    pub id: i64,
    /// New subject
    #[arg(long)]
    pub subject: Option<String>,
    /// New status (wide set: Open, Pending, Closed, Unread, Read, Priority, ...)
    #[arg(long)]
    pub status: Option<String>,
    /// Move to channel ID
    #[arg(long)]
    pub channel: Option<i64>,
    /// Re-link to contact ID
    #[arg(long, visible_alias = "contact")]
    pub contact_id: Option<i64>,
    /// Link to a sales order ID
    #[arg(long = "sales-order")]
    pub sales_order: Option<i64>,
    /// Assign to owner (agent) user ID
    #[arg(long)]
    pub owner: Option<i64>,
    /// Replace tags with these tag IDs (repeatable)
    #[arg(long = "tag", value_name = "TAG_ID")]
    pub tags: Vec<i64>,
    /// Custom field as NAME=VALUE (repeatable)
    #[arg(short = 'f', long = "field", value_name = "NAME=VALUE")]
    pub custom_fields: Vec<String>,
}

pub async fn run(ctx: &Context, cmd: TicketCmd) -> Result<()> {
    let client = ctx.client()?;
    match cmd {
        TicketCmd::List(args) => {
            let params = ListTicketsParams {
                page: Default::default(),
                order_by: args.sort,
                order_direction: args.direction,
                contact_id: args.contact,
                channel_id: args.channel,
                status: args.status,
                ticket_type: args.ticket_type,
                sales_order_id: args.sales_order,
                created_at_gte: args.created_after,
                created_at_lte: args.created_before,
                last_updated_at_gte: args.updated_after,
                last_updated_at_lte: args.updated_before,
                owner_user_id: args.owner,
                seller_order_id: args.seller_order,
            };
            let (items, paginator) = util::paginate(&args.list, |page| {
                let mut params = params.clone();
                params.page = page;
                let client = &client;
                async move { client.list_tickets(&params).await }
            })
            .await?;
            output::print_list(&ctx.global, items, paginator, COLUMNS)
        }
        TicketCmd::View { id } => {
            let resp = client.get_ticket(id).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TicketCmd::Create(args) => {
            let contact = args.contact_email.map(|email| ContactRequest {
                email,
                full_name: args.contact_name,
                phone_number: None,
            });
            let req = CreateTicketRequest {
                subject: args.subject,
                channel_id: args.channel,
                status: args.status,
                sales_order_id: args.sales_order,
                created_at: args.created_at,
                full_response: None,
                custom_fields: optional_fields(&args.custom_fields)?,
                contact_id: args.contact_id,
                contact,
            };
            let resp = client.create_ticket(&req).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TicketCmd::Update(args) => {
            let req = UpdateTicketRequest {
                subject: args.subject,
                channel_id: args.channel,
                status: args.status,
                sales_order_id: args.sales_order,
                custom_fields: optional_fields(&args.custom_fields)?,
                contact_id: args.contact_id,
                tag_ids: if args.tags.is_empty() {
                    None
                } else {
                    Some(args.tags)
                },
                owner_user_id: args.owner,
            };
            let resp = client.update_ticket(args.id, &req).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TicketCmd::UpdateData { id, custom_fields } => {
            let custom_fields = util::parse_custom_fields(&custom_fields)?;
            let resp = client.update_ticket_data(id, &custom_fields).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TicketCmd::Delete { id, yes } => {
            util::confirm(&format!("delete ticket {id}"), yes)?;
            let resp = client.delete_ticket(id).await?;
            output::print_confirmation(&ctx.global, resp.merged())
        }
    }
}

fn optional_fields(raw: &[String]) -> Result<Option<Vec<edesk_client::api::CustomField>>> {
    if raw.is_empty() {
        Ok(None)
    } else {
        Ok(Some(util::parse_custom_fields(raw)?))
    }
}
