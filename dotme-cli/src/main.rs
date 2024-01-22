// Copyright matzxrr

#![deny(warnings, clippy::all)]
#![forbid(unsafe_code)]

mod dotme_panic;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dotme")]
#[command(about = "Dotfile management through a bare repo.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Setup new dotfile repo")]
    Init,
    #[command(about = "Install your dotfiles on a system")]
    Clone,
    #[command(about = "Add 'config' command to your PATH")]
    Config,
}

fn main() {
    dotme_panic::setup();
    let args = Cli::parse();
    match args.command {
        Commands::Init => {
            println!("Setup a new dotfile repo");
        }
        Commands::Clone => {
            println!("Install your dotfiles on a system")
        }
        Commands::Config => {
            println!("Add 'config' command to your PATH")
        }
    };
}
