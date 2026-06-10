use anyhow::{Context as _, Result};
use base64::Engine;
use clap::{Args, Subcommand};
use edesk_client::api::{CreateOrderNoteRequest, NoteFile, UpdateOrderNoteRequest};

use super::util::{self, ListOpts};
use crate::context::Context;
use crate::output::{self, col, Column};

const COLUMNS: &[Column] = &[
    col("id", "id"),
    col("order", "sales_order_id"),
    col("text", "text"),
    col("author", "user.name"),
    col("created", "created_at"),
];

#[derive(Debug, Subcommand)]
pub enum NoteCmd {
    /// List order notes
    List {
        #[command(flatten)]
        list: ListOpts,
    },
    /// Show one order note
    View {
        /// Order note ID
        id: i64,
    },
    /// Create an order note
    Create {
        /// Sales order ID the note belongs to
        #[arg(long)]
        order: i64,
        /// Note text
        #[arg(long)]
        text: String,
    },
    /// Update an order note
    Update {
        /// Order note ID
        id: i64,
        /// New note text
        #[arg(long)]
        text: String,
        /// Move the note to another sales order
        #[arg(long)]
        order: Option<i64>,
    },
    /// Attach local files to an order note (uploaded as base64)
    Attach(AttachArgs),
    /// Delete an order note
    Delete {
        /// Order note ID
        id: i64,
        /// Skip the confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Debug, Args)]
pub struct AttachArgs {
    /// Order note ID
    pub id: i64,
    /// File to attach (repeatable)
    #[arg(long, required = true, value_name = "PATH")]
    pub file: Vec<std::path::PathBuf>,
    /// Attachment kind: Other or Invoice
    #[arg(long, default_value = "Other", value_name = "KIND")]
    pub kind: String,
    /// Override the MIME type (default: guessed from the extension)
    #[arg(long, value_name = "MIME")]
    pub mime: Option<String>,
}

pub async fn run(ctx: &Context, cmd: NoteCmd) -> Result<()> {
    let client = ctx.client()?;
    match cmd {
        NoteCmd::List { list } => {
            let (items, paginator) = util::paginate(&list, |page| {
                let client = &client;
                async move { client.list_order_notes(page).await }
            })
            .await?;
            output::print_list(&ctx.global, items, paginator, COLUMNS)
        }
        NoteCmd::View { id } => {
            let resp = client.get_order_note(id).await?;
            output::print_single(&ctx.global, resp.data)
        }
        NoteCmd::Create { order, text } => {
            let req = CreateOrderNoteRequest {
                sales_order_id: order,
                text,
            };
            let resp = client.create_order_note(&req).await?;
            output::print_single(&ctx.global, resp.data)
        }
        NoteCmd::Update { id, text, order } => {
            let req = UpdateOrderNoteRequest {
                sales_order_id: order,
                text,
            };
            let resp = client.update_order_note(id, &req).await?;
            output::print_single(&ctx.global, resp.data)
        }
        NoteCmd::Attach(args) => {
            let files = args
                .file
                .iter()
                .map(|path| {
                    let bytes = std::fs::read(path)
                        .with_context(|| format!("could not read {}", path.display()))?;
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "attachment".to_string());
                    Ok(NoteFile {
                        mime_type: args
                            .mime
                            .clone()
                            .unwrap_or_else(|| guess_mime(&name).to_string()),
                        name,
                        base64: base64::engine::general_purpose::STANDARD.encode(&bytes),
                        attachment_type: args.kind.clone(),
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let resp = client.create_order_note_attachment(args.id, &files).await?;
            output::print_single(&ctx.global, resp.data)
        }
        NoteCmd::Delete { id, yes } => {
            util::confirm(&format!("delete order note {id}"), yes)?;
            let resp = client.delete_order_note(id).await?;
            output::print_confirmation(&ctx.global, resp.data)
        }
    }
}

fn guess_mime(name: &str) -> &'static str {
    let ext = name.rsplit('.').next().unwrap_or_default().to_lowercase();
    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "pdf" => "application/pdf",
        "txt" | "log" => "text/plain",
        "csv" => "text/csv",
        "json" => "application/json",
        "zip" => "application/zip",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        _ => "application/octet-stream",
    }
}
