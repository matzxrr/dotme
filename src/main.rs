use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use which::which;

fn git_exists() -> Result<bool> {
    let result = which("git");
    if result.is_err() {
        return Err(anyhow!(
            "Cound not find 'git' on your PATH. Make sure it is installed."
        ));
    }
    Ok(true)
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
}

fn main() -> Result<()> {
    git_exists()?;

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            println!("Calling init");
        }
        None => {}
    }

    Ok(())
}
