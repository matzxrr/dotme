// Copyright matzxrr

#![deny(warnings, clippy::all)]
#![forbid(unsafe_code)]

mod dotme_panic;

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
    dotme_panic::setup();
    let args = Cli::parse();
    match args.command {
        Commands::Status => {
            cmd::status::cmd_status();
            if true {
                panic!("what??");
            }
        }
    };
}
