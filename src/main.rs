use clap::Parser;
use std::process;

mod cli;
mod generators;
mod templates;
mod config;
mod error;
mod filesystem;

use cli::Commands;
use error::CliError;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let commands = Commands::parse();

    if let Err(e) = run(commands).await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn run(commands: Commands) -> Result<(), CliError> {
    match commands {
        Commands::Init { framework, name } => {
            println!("Initializing project '{}' with framework {:?}", name, framework);
            // TODO: Implement init command
            Ok(())
        }
        Commands::Generate { component } => {
            println!("Generating component: {:?}", component);
            // TODO: Implement generate command
            Ok(())
        }
        Commands::Config { action } => {
            println!("Config action: {:?}", action);
            // TODO: Implement config command
            Ok(())
        }
    }
}