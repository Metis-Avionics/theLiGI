//! theligi-cli: CLI entry point.

use clap::{Parser, Subcommand};

/// theligi CLI.
#[derive(Parser, Debug)]
#[command(name = "theligi")]
#[command(about = "TheLiGI CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the server.
    Serve,
    /// Run the desktop.
    Desktop,
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Serve => {
            println!("Starting server...");
            Ok(())
        }
        Commands::Desktop => {
            println!("Starting desktop...");
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}
