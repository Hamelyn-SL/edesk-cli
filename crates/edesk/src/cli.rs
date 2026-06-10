use clap::{Args, Parser, Subcommand};

use crate::commands;

#[derive(Debug, Parser)]
#[command(
    name = "edesk",
    version,
    about = "Work with the eDesk API from the command line",
    long_about = "Work with eDesk tickets, messages, sales orders, tags, templates and more \
                  from the command line.\n\nAuthenticate once with `edesk auth login`, then try \
                  `edesk ticket list` or `edesk whoami`.",
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[command(flatten)]
    pub global: GlobalArgs,
}

#[derive(Clone, Args)]
pub struct GlobalArgs {
    /// API token (overrides stored credentials)
    #[arg(long, global = true, env = "EDESK_TOKEN", hide_env_values = true)]
    pub token: Option<String>,

    /// API base URL
    #[arg(long, global = true, env = "EDESK_BASE_URL", hide = true)]
    pub base_url: Option<String>,

    /// Output raw JSON instead of a table
    #[arg(long, global = true)]
    pub json: bool,

    /// Filter the JSON output with a jq expression (implies --json)
    #[arg(long, global = true, value_name = "EXPR")]
    pub jq: Option<String>,

    /// Comma-separated list of fields to output
    #[arg(long, global = true, value_delimiter = ',', value_name = "FIELD,...")]
    pub fields: Option<Vec<String>>,

    /// Suppress informational messages on stderr
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

// Hand-written so the token can never leak through `{:?}` formatting.
impl std::fmt::Debug for GlobalArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalArgs")
            .field("token", &self.token.as_deref().map(|_| "[REDACTED]"))
            .field("base_url", &self.base_url)
            .field("json", &self.json)
            .field("jq", &self.jq)
            .field("fields", &self.fields)
            .field("quiet", &self.quiet)
            .finish()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Show the identity behind the current API token
    Whoami,

    /// Manage tickets
    #[command(subcommand)]
    Ticket(commands::ticket::TicketCmd),

    /// Manage messages within tickets
    #[command(subcommand)]
    Message(commands::message::MessageCmd),

    /// Manage sales orders
    #[command(subcommand)]
    Order(commands::order::OrderCmd),

    /// Manage tracking links of a sales order
    #[command(subcommand)]
    Tracking(commands::tracking::TrackingCmd),

    /// Manage order notes
    #[command(subcommand)]
    Note(commands::note::NoteCmd),

    /// Manage tags
    #[command(subcommand)]
    Tag(commands::tag::TagCmd),

    /// List tag groups
    #[command(subcommand, name = "tag-group")]
    TagGroup(commands::tag_group::TagGroupCmd),

    /// Manage templates
    #[command(subcommand)]
    Template(commands::template::TemplateCmd),

    /// Search contacts
    #[command(subcommand)]
    Contact(commands::contact::ContactCmd),

    /// List channels
    #[command(subcommand)]
    Channel(commands::channel::ChannelCmd),

    /// List account users (agents)
    #[command(subcommand)]
    User(commands::user::UserCmd),

    /// Authenticate edesk with the eDesk API
    #[command(subcommand)]
    Auth(commands::auth::AuthCmd),

    /// Manage edesk configuration
    #[command(subcommand)]
    Config(commands::config_cmd::ConfigCmd),

    /// Make a raw, authenticated eDesk API request
    Api(commands::api::ApiArgs),

    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}
