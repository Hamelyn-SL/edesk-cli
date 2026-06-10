use anyhow::Result;
use clap::{Args, Subcommand};
use edesk_client::api::ListSalesOrdersParams;

use super::util::{self, ListOpts};
use crate::context::Context;
use crate::output::{self, col, Column};

const COLUMNS: &[Column] = &[
    col("id", "id"),
    col("seller order", "seller_order_id"),
    col("status", "status"),
    col("channel", "channel_id"),
    col("contact", "contact_id"),
    col("total", "total_amount"),
    col("created", "created_at"),
];

#[derive(Debug, Subcommand)]
pub enum OrderCmd {
    /// List sales orders
    List(Box<ListArgs>),
    /// Show one sales order
    View {
        /// Sales order ID (internal eDesk ID, not the marketplace order ID)
        id: i64,
    },
    /// Create a sales order from a JSON body
    ///
    /// The schema is wide (order items, addresses, tracking codes); see
    /// https://developers.edesk.com/reference/createsalesorder for fields.
    Create {
        /// Inline JSON body
        #[arg(long, conflicts_with = "body_file")]
        body: Option<String>,
        /// Read the JSON body from a file (use - for stdin)
        #[arg(long, value_name = "PATH")]
        body_file: Option<std::path::PathBuf>,
    },
    /// Update a sales order from a JSON body
    ///
    /// Note: each order item in the body must include its existing item `id`.
    Update {
        /// Sales order ID
        id: i64,
        /// Inline JSON body
        #[arg(long, conflicts_with = "body_file")]
        body: Option<String>,
        /// Read the JSON body from a file (use - for stdin)
        #[arg(long, value_name = "PATH")]
        body_file: Option<std::path::PathBuf>,
    },
    /// Delete a sales order
    Delete {
        /// Sales order ID
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
    /// Filter by status (OrderReceived, OrderShipped, Delivered, Canceled, ...)
    #[arg(long)]
    pub status: Option<String>,
    /// Filter by channel ID
    #[arg(long)]
    pub channel: Option<i64>,
    /// Filter by contact ID
    #[arg(long)]
    pub contact: Option<i64>,
    /// Filter by marketplace order ID (exact match)
    #[arg(long = "seller-order")]
    pub seller_order: Option<String>,
    /// Only IDs greater than or equal to this
    #[arg(long, value_name = "ID")]
    pub id_min: Option<i64>,
    /// Only IDs less than or equal to this
    #[arg(long, value_name = "ID")]
    pub id_max: Option<i64>,
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

pub async fn run(ctx: &Context, cmd: OrderCmd) -> Result<()> {
    let client = ctx.client()?;
    match cmd {
        OrderCmd::List(args) => {
            let params = ListSalesOrdersParams {
                page: Default::default(),
                order_by: args.sort,
                order_direction: args.direction,
                contact_id: args.contact,
                channel_id: args.channel,
                seller_order_id: args.seller_order,
                status: args.status,
                id_gte: args.id_min,
                id_lte: args.id_max,
                created_at_gte: args.created_after,
                created_at_lte: args.created_before,
                last_updated_at_gte: args.updated_after,
                last_updated_at_lte: args.updated_before,
            };
            let (items, paginator) = util::paginate(&args.list, |page| {
                let mut params = params.clone();
                params.page = page;
                let client = &client;
                async move { client.list_sales_orders(&params).await }
            })
            .await?;
            output::print_list(&ctx.global, items, paginator, COLUMNS)
        }
        OrderCmd::View { id } => {
            let resp = client.get_sales_order(id).await?;
            output::print_single(&ctx.global, resp.data)
        }
        OrderCmd::Create { body, body_file } => {
            let body = util::read_body(body.as_deref(), body_file.as_deref())?;
            let resp = client.create_sales_order(&body).await?;
            output::print_single(&ctx.global, resp.data)
        }
        OrderCmd::Update {
            id,
            body,
            body_file,
        } => {
            let body = util::read_body(body.as_deref(), body_file.as_deref())?;
            let resp = client.update_sales_order(id, &body).await?;
            output::print_single(&ctx.global, resp.data)
        }
        OrderCmd::Delete { id, yes } => {
            util::confirm(&format!("delete sales order {id}"), yes)?;
            let resp = client.delete_sales_order(id).await?;
            output::print_confirmation(&ctx.global, resp.merged())
        }
    }
}
