// Copyright matzxrr

#![deny(warnings, clippy::all)]
#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use dotme_core::cmd;

#[derive(Debug, Parser)]
#[command(name = "dotme")]
#[command(about = "A git based dotfile manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Show the uncommited changes")]
    Status,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Status => {
            cmd::status::cmd_status();
        }
    };
}
