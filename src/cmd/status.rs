use anyhow::Result;
use std::process::exit;

use git2::{ErrorCode, Repository, StatusOptions};

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

    if repo.set_workdir(&config.work_tree, false).is_err() {
        eprintln!("Nope, cant setup work tree");
        exit(1);
    }

    if show_branch(&repo).is_err() {
        eprintln!("Unable to read current branch");
        exit(1);
    }

    // https://github.com/rust-lang/git2-rs/blob/master/examples/status.rs
    // Example ^^
    println!("# repo: {:?}", repo.path());

    let mut opts = StatusOptions::new();
    opts.include_untracked(false);

    let statuses = match repo.statuses(Some(&mut opts)) {
        Ok(statuses) => statuses,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    let mut header = false;
    // Print index changes
    for entry in statuses
        .iter()
        .filter(|e| e.status() != git2::Status::CURRENT)
    {
        let istatus = match entry.status() {
            s if s.contains(git2::Status::INDEX_NEW) => "new file: ",
            s if s.contains(git2::Status::INDEX_MODIFIED) => "modified: ",
            s if s.contains(git2::Status::INDEX_DELETED) => "deleted: ",
            s if s.contains(git2::Status::INDEX_RENAMED) => "renamed: ",
            s if s.contains(git2::Status::INDEX_TYPECHANGE) => "typechange: ",
            _ => continue,
        };
        if !header {
            println!("#");
            println!("# Changes to be commited:");
            println!("#");
            header = true;
        }

        let old_path = entry.head_to_index().unwrap().old_file().path();
        let new_path = entry.head_to_index().unwrap().new_file().path();
        match (old_path, new_path) {
            (Some(old), Some(new)) if old != new => {
                println!("#\t{}\t{} -> {}", istatus, old.display(), new.display());
            }
            (old, new) => {
                println!("#\t{}\t{}", istatus, old.or(new).unwrap().display());
            }
        }
    }
}

fn show_branch(repo: &Repository) -> Result<(), git2::Error> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return Err(e),
    };
    let head = head.as_ref().and_then(|h| h.shorthand());
    println!(
        "# branch: {}",
        head.unwrap_or("Not currently on any branch")
    );
    Ok(())
}
