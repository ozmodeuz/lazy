mod config;
mod cppnix;
mod errors;
mod init;
mod nixos;

use clap::{Parser, Subcommand};
use init::init;

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
            init();
        }
    }
}
