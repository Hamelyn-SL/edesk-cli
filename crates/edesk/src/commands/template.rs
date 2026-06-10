use anyhow::Result;
use clap::{Args, Subcommand};
use serde_json::{json, Value};

use super::util::{self, ListOpts};
use crate::context::Context;
use crate::output::{self, col, Column};

const COLUMNS: &[Column] = &[
    col("id", "id"),
    col("name", "name"),
    col("usage", "template_usage"),
    col("type", "template_type"),
    col("active", "active"),
    col("created", "created_at"),
];

/// Fields the TemplateRequest schema accepts; used to filter the current
/// template state during read-modify-write updates.
const REQUEST_FIELDS: &[&str] = &[
    "name",
    "subject",
    "body_text",
    "channels",
    "template_usage",
    "template_type",
    "query_type",
    "order_status",
    "delivery_date",
    "active",
    "order_fulfilment",
    "message_subject",
    "invoice_attached",
    "only_use_if_no_replies_yet",
];

#[derive(Debug, Subcommand)]
pub enum TemplateCmd {
    /// List templates
    List {
        #[command(flatten)]
        list: ListOpts,
    },
    /// Show one template
    View {
        /// Template ID
        id: i64,
    },
    /// Create a template
    Create(CreateArgs),
    /// Update a template
    ///
    /// The API replaces the whole template on update; unspecified fields are
    /// preserved by reading the current template first. Use --body for full
    /// control.
    Update(UpdateArgs),
    /// Delete a template
    Delete {
        /// Template ID
        id: i64,
        /// Skip the confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Template name
    #[arg(long)]
    pub name: Option<String>,
    /// Template body text
    #[arg(long)]
    pub body_text: Option<String>,
    /// Message subject line
    #[arg(long)]
    pub subject: Option<String>,
    /// Usage: Manual or ManualAuto
    #[arg(long, default_value = "Manual")]
    pub usage: String,
    /// Audience: Consumer, Internal or External (repeatable)
    #[arg(long = "type", default_value = "Consumer")]
    pub template_type: Vec<String>,
    /// Create as inactive
    #[arg(long)]
    pub inactive: bool,
    /// Full JSON body instead of flags
    #[arg(long, conflicts_with_all = ["name", "body_text", "subject"])]
    pub body: Option<String>,
    /// Read the full JSON body from a file (use - for stdin)
    #[arg(long, value_name = "PATH", conflicts_with = "body")]
    pub body_file: Option<std::path::PathBuf>,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Template ID
    pub id: i64,
    /// New name
    #[arg(long)]
    pub name: Option<String>,
    /// New body text
    #[arg(long)]
    pub body_text: Option<String>,
    /// New subject line
    #[arg(long)]
    pub subject: Option<String>,
    /// New usage: Manual or ManualAuto
    #[arg(long)]
    pub usage: Option<String>,
    /// Set active state
    #[arg(long, value_name = "BOOL")]
    pub active: Option<bool>,
    /// Full JSON body instead of flags (skips read-modify-write)
    #[arg(long)]
    pub body: Option<String>,
    /// Read the full JSON body from a file (use - for stdin)
    #[arg(long, value_name = "PATH", conflicts_with = "body")]
    pub body_file: Option<std::path::PathBuf>,
}

pub async fn run(ctx: &Context, cmd: TemplateCmd) -> Result<()> {
    let client = ctx.client()?;
    match cmd {
        TemplateCmd::List { list } => {
            let (items, paginator) = util::paginate(&list, |page| {
                let client = &client;
                async move { client.list_templates(page).await }
            })
            .await?;
            output::print_list(&ctx.global, items, paginator, COLUMNS)
        }
        TemplateCmd::View { id } => {
            let resp = client.get_template(id).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TemplateCmd::Create(args) => {
            let body = if args.body.is_some() || args.body_file.is_some() {
                util::read_body(args.body.as_deref(), args.body_file.as_deref())?
            } else {
                let (Some(name), Some(body_text)) = (args.name, args.body_text) else {
                    anyhow::bail!(
                        "--name and --body-text are required (or pass --body/--body-file)"
                    );
                };
                let mut body = json!({
                    "name": name,
                    "body_text": body_text,
                    "template_usage": args.usage,
                    "template_type": args.template_type,
                    "active": !args.inactive,
                });
                if let Some(subject) = args.subject {
                    body["subject"] = Value::String(subject);
                }
                body
            };
            let resp = client.create_template(&body).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TemplateCmd::Update(args) => {
            let body = if args.body.is_some() || args.body_file.is_some() {
                util::read_body(args.body.as_deref(), args.body_file.as_deref())?
            } else {
                // Read-modify-write: fetch the template, keep only fields the
                // request schema accepts, then apply the flag overrides.
                let current = client.get_template(args.id).await?.data;
                let mut body = serde_json::Map::new();
                if let Value::Object(map) = current {
                    for (key, value) in map {
                        if REQUEST_FIELDS.contains(&key.as_str()) && !value.is_null() {
                            body.insert(key, value);
                        }
                    }
                }
                let mut body = Value::Object(body);
                if let Some(name) = args.name {
                    body["name"] = Value::String(name);
                }
                if let Some(text) = args.body_text {
                    body["body_text"] = Value::String(text);
                }
                if let Some(subject) = args.subject {
                    body["subject"] = Value::String(subject);
                }
                if let Some(usage) = args.usage {
                    body["template_usage"] = Value::String(usage);
                }
                if let Some(active) = args.active {
                    body["active"] = Value::Bool(active);
                }
                body
            };
            let resp = client.update_template(args.id, &body).await?;
            output::print_single(&ctx.global, resp.data)
        }
        TemplateCmd::Delete { id, yes } => {
            util::confirm(&format!("delete template {id}"), yes)?;
            let resp = client.delete_template(id).await?;
            output::print_confirmation(&ctx.global, resp.data)
        }
    }
}
