mod config;
mod cppnix;
mod nixos;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "sysconf")]
#[command(about = "Simplified NixOS configuration and package manager")]
#[command(version = "0.0.1")]
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
        Commands::Init => {
            println!("{:#?}", config::initialize());
        }
    }
}
