pub(crate) mod cmd;
pub(crate) mod server;
pub(crate) mod client;
pub(crate) mod protocol;

use cmd::cmd::{Cmd, SubCommand};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cmd::parse();

    match cli.command {
        SubCommand::Server { addr } => {
            server::server::run_server(&addr).await?;
        },
        SubCommand::Client { addr, username } => {
            client::client::run_client(&addr, &username).await?;
        },
    }

    Ok(())
}