pub mod api;
pub mod auth;
pub mod channel;
pub mod config_cmd;
pub mod contact;
pub mod message;
pub mod note;
pub mod order;
pub mod tag;
pub mod tag_group;
pub mod template;
pub mod ticket;
pub mod tracking;
pub mod upgrade;
pub mod user;
pub mod util;
pub mod whoami;

use clap::CommandFactory;

use crate::cli::{Cli, Command};
use crate::context::Context;

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    let ctx = Context::new(cli.global);
    match cli.command {
        Command::Whoami => whoami::run(&ctx).await,
        Command::Ticket(cmd) => ticket::run(&ctx, cmd).await,
        Command::Message(cmd) => message::run(&ctx, cmd).await,
        Command::Order(cmd) => order::run(&ctx, cmd).await,
        Command::Tracking(cmd) => tracking::run(&ctx, cmd).await,
        Command::Note(cmd) => note::run(&ctx, cmd).await,
        Command::Tag(cmd) => tag::run(&ctx, cmd).await,
        Command::TagGroup(cmd) => tag_group::run(&ctx, cmd).await,
        Command::Template(cmd) => template::run(&ctx, cmd).await,
        Command::Contact(cmd) => contact::run(&ctx, cmd).await,
        Command::Channel(cmd) => channel::run(&ctx, cmd).await,
        Command::User(cmd) => user::run(&ctx, cmd).await,
        Command::Auth(cmd) => auth::run(&ctx, cmd).await,
        Command::Config(cmd) => config_cmd::run(&ctx, cmd),
        Command::Api(args) => api::run(&ctx, args).await,
        Command::Upgrade(args) => upgrade::run(args).await,
        Command::Completion { shell } => {
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "edesk", &mut std::io::stdout());
            Ok(())
        }
    }
}
