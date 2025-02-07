mod client;
mod server;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the Veil server
    Server,
    /// Run the Veil client
    Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Server => {
            println!("Starting Veil Server...");
            #[cfg(feature = "server")]
            server::run_server().await; // Call run_server from server module
            #[cfg(not(feature = "server"))]
            println!("Server feature not enabled. Compile with `--features server`");
        }
        Commands::Client => {
            println!("Starting Veil Client...");
            #[cfg(feature = "client")]
            client::run_client()?; // Call run_client from client module
            #[cfg(not(feature = "client"))]
            println!("Client feature not enabled. Compile with `--features client`");
        }
    }
    Ok(())
}
