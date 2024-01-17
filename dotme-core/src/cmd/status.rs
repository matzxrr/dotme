use std::process::exit;

use git2::StatusOptions;

use crate::repo::Repo;

pub fn cmd_status() {
    let mut repo = match Repo::load() {
        Ok(repo) => repo,
        Err(err) => {
            eprintln!("ecountered error: {}", err);
            exit(1);
        }
    };

    if let Err(err) = repo.set_worktree() {
        eprintln!("encountered error: {}", err);
        exit(1);
    }

    match repo.get_branch_name() {
        Ok(branch_name) => println!("# branch: {}", branch_name),
        Err(err) => {
            eprintln!("encountered error: {}", err);
            exit(1);
        }
    }

    // https://github.com/rust-lang/git2-rs/blob/master/examples/status.rs
    // Example ^^
    println!("# repo: {:?}", repo.repo.path());

    let mut opts = StatusOptions::new();
    opts.include_untracked(false);

    let statuses = match repo.repo.statuses(Some(&mut opts)) {
        Ok(statuses) => statuses,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    let mut header = false;
    // Print Staged changes
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
    header = false;

    // Print unstaged changes
    for entry in statuses.iter() {
        // IDK why this needs to be here, check the example :)
        if entry.status() == git2::Status::CURRENT || entry.index_to_workdir().is_none() {
            continue;
        }
        let istatus = match entry.status() {
            s if s.contains(git2::Status::WT_MODIFIED) => "modified: ",
            s if s.contains(git2::Status::WT_DELETED) => "deleted: ",
            s if s.contains(git2::Status::WT_RENAMED) => "renamed: ",
            s if s.contains(git2::Status::WT_TYPECHANGE) => "typechange:",
            _ => continue,
        };
        if !header {
            println!("#");
            println!("# Changes not staged for commit:");
            println!("#");
            header = true;
        }
        let old_path = entry.index_to_workdir().unwrap().old_file().path();
        let new_path = entry.index_to_workdir().unwrap().new_file().path();
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
