use anyhow::{Context as _, Result};
use clap::{Args, Subcommand};
use edesk_client::api::{CreateMessageRequest, MessageAttachment, UpdateMessageRequest};
use serde_json::Value;

use super::util;
use crate::context::Context;
use crate::output::{self, col, Column};

const COLUMNS: &[Column] = &[
    col("id", "id"),
    col("type", "type"),
    col("direction", "direction"),
    col("subject", "subject"),
    col("created", "created_at"),
    col("ticket", "ticket_id"),
];

#[derive(Debug, Subcommand)]
pub enum MessageCmd {
    /// List the messages of a ticket
    ///
    /// The API has no list-messages endpoint; this reads the ticket's
    /// messages_ids and fetches each message.
    List {
        /// Ticket ID
        #[arg(long)]
        ticket: i64,
    },
    /// Show one message
    View {
        /// Message ID
        id: i64,
    },
    /// Add a message or internal note to a ticket
    Create(CreateArgs),
    /// Update a message (the API only allows re-typing notes)
    Update(UpdateArgs),
    /// Delete a message
    Delete {
        /// Message ID
        id: i64,
        /// Skip the confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Ticket ID to add the message to
    #[arg(long)]
    pub ticket: i64,
    /// Message body (HTML or plain text)
    #[arg(long, conflicts_with = "body_file")]
    pub body: Option<String>,
    /// Read the message body from a file (use - for stdin)
    #[arg(long, value_name = "PATH")]
    pub body_file: Option<std::path::PathBuf>,
    /// Message or Note (Note = internal, not visible to the customer)
    #[arg(long = "type", default_value = "Message")]
    pub message_type: String,
    /// Message subject
    #[arg(long)]
    pub subject: Option<String>,
    /// Direction: Incoming or Outgoing
    #[arg(long)]
    pub direction: Option<String>,
    /// Actually send the message to the customer by email
    #[arg(long)]
    pub send: bool,
    /// Attachment as NAME=URL (repeatable; the API downloads the URL)
    #[arg(long = "attach", value_name = "NAME=URL")]
    pub attachments: Vec<String>,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Message ID
    pub id: i64,
    /// New subject
    #[arg(long)]
    pub subject: Option<String>,
    /// New body
    #[arg(long)]
    pub body: Option<String>,
    /// Convert to this type (the API only accepts Note)
    #[arg(long = "type")]
    pub message_type: Option<String>,
}

pub async fn run(ctx: &Context, cmd: MessageCmd) -> Result<()> {
    let client = ctx.client()?;
    match cmd {
        MessageCmd::List { ticket } => {
            let resp = client.get_ticket(ticket).await?;
            let ids: Vec<i64> = resp
                .data
                .get("messages_ids")
                .and_then(Value::as_array)
                .map(|ids| ids.iter().filter_map(Value::as_i64).collect())
                .unwrap_or_default();

            let mut join_set = tokio::task::JoinSet::new();
            for (index, id) in ids.iter().copied().enumerate() {
                let client = client.clone();
                join_set.spawn(async move { (index, client.get_message(id).await) });
            }
            let mut items: Vec<(usize, Value)> = Vec::with_capacity(ids.len());
            while let Some(joined) = join_set.join_next().await {
                let (index, result) = joined.context("message fetch task failed")?;
                items.push((index, result?.data));
            }
            items.sort_by_key(|(index, _)| *index);
            let items: Vec<Value> = items.into_iter().map(|(_, value)| value).collect();
            output::print_list(&ctx.global, items, None, COLUMNS)
        }
        MessageCmd::View { id } => {
            let resp = client.get_message(id).await?;
            output::print_single(&ctx.global, resp.data)
        }
        MessageCmd::Create(args) => {
            let body = match (&args.body, &args.body_file) {
                (Some(inline), None) => inline.clone(),
                (None, Some(path)) if path.as_os_str() == "-" => {
                    use std::io::Read;
                    let mut buf = String::new();
                    std::io::stdin().read_to_string(&mut buf)?;
                    buf
                }
                (None, Some(path)) => std::fs::read_to_string(path)
                    .with_context(|| format!("could not read {}", path.display()))?,
                _ => anyhow::bail!("provide the message body with --body or --body-file"),
            };
            let attachments = parse_attachments(&args.attachments)?;
            let req = CreateMessageRequest {
                ticket_id: args.ticket,
                body,
                message_type: args.message_type,
                subject: args.subject,
                attachments,
                created_at: None,
                full_response: None,
                direction: args.direction,
                send: args.send.then_some(true),
            };
            let resp = client.create_message(&req).await?;
            output::print_single(&ctx.global, resp.data)
        }
        MessageCmd::Update(args) => {
            let req = UpdateMessageRequest {
                subject: args.subject,
                body: args.body,
                message_type: args.message_type,
            };
            let resp = client.update_message(args.id, &req).await?;
            output::print_single(&ctx.global, resp.data)
        }
        MessageCmd::Delete { id, yes } => {
            util::confirm(&format!("delete message {id}"), yes)?;
            let resp = client.delete_message(id).await?;
            output::print_confirmation(&ctx.global, resp.merged())
        }
    }
}

fn parse_attachments(raw: &[String]) -> Result<Option<Vec<MessageAttachment>>> {
    if raw.is_empty() {
        return Ok(None);
    }
    let attachments = raw
        .iter()
        .map(|pair| {
            let (name, url) = util::parse_key_value(pair)?;
            Ok(MessageAttachment { name, url })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Some(attachments))
}
