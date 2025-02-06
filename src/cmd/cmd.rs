use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "veil")]
#[command(about = "Secure Group Messaging Protocol implementation in Rust", long_about = None)]
pub struct Cmd {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Run as server (admin): listen for join requests and relay messages.
    Server {
        /// Address to bind (default: 127.0.0.1:8080)
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        addr: String,
    },
    /// Run as client: connect to a server and join the group.
    Client {
        /// Server address (default: 127.0.0.1:8080)
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        addr: String,
        /// Username for this client
        #[arg(short, long, default_value = "Anonymous")]
        username: String,
    },
}