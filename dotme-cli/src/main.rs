// Copyright matzxrr

#![deny(warnings, clippy::all)]
#![forbid(unsafe_code)]

mod dotme_panic;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use dotme_core::cmd;

#[derive(Debug, Parser)]
#[command(name = "dotme")]
#[command(about = "Configure a git dotfile repo", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Setup a new dotfile repo")]
    Init,
    #[command(
        about = "Clone and install your dotfiles",
        arg_required_else_help = true
    )]
    Clone { remote: PathBuf },
    #[command(about = "Configure your existing repo")]
    Config,
    #[command(about = "Uninstall your dotfiles and restore previous files")]
    Restore,
}

fn main() {
    dotme_panic::setup();
    let args = Cli::parse();
    match args.command {
        Commands::Init => {
            match cmd::init() {
                Ok(()) => println!("Init Complete!"),
                Err(err) => println!("{}", err),
            };
        }
        Commands::Clone { remote } => match cmd::clone(&remote) {
            Ok(()) => println!("Clone Complete!"),
            Err(err) => println!("{}", err),
        },
        Commands::Config => {
            todo!()
        }
        Commands::Restore => {
            todo!()
        }
    };
}
