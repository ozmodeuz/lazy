mod config;
mod init;
mod nix;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "lazy")]
#[command(about = "Lazy configuration and package management for NixOS")]
#[command(version = "0.1.1")]
#[command(long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init { path: Option<PathBuf> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            init::import_test(path)?;
            Ok(())
        }
        _ => Ok(()),
    }
}
