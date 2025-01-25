mod cppnix;
mod init;
mod nixos;

use clap::{Parser, Subcommand};

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
    Init,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init(path),
    }
}
