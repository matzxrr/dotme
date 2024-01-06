use std::process::exit;

use git2::Repository;

use crate::config::{read_config_toml, Config};

pub fn cmd_status() {
    let config_toml = read_config_toml();
    let config = Config::from(&config_toml);
    let repo = match Repository::open(&config.repo) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Unable to load repo '{}'", config.repo.display());
            eprintln!("{}", e);
            exit(1);
        }
    };
    // https://github.com/rust-lang/git2-rs/blob/master/examples/status.rs
    // Example ^^
    println!("Found repo from config file {:?}", repo.path());
}
