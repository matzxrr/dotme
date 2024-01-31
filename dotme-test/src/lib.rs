use std::{fs, path::Path};

use anyhow::Result;
use git2::{Repository, RepositoryInitOptions};

pub struct TestRepo(pub Repository);

impl TestRepo {
    pub fn clone_repo(&self) -> Result<Repository> {
        let path = self.0.path();
        Repository::open_bare(path).map_err(Into::into)
    }
}

impl Drop for TestRepo {
    fn drop(&mut self) {
        let repo_path = self.0.path();
        println!("Destorying {}", repo_path.display());
        if let Err(err) = std::fs::remove_dir_all(repo_path) {
            eprintln!("Unable to destory {} - {}", repo_path.display(), err);
        };
    }
}

/// Creates a test repository that gets destroyed on `Drop`
pub fn create_temp_bare_repo() -> TestRepo {
    let temp_dir = Path::new(".tmp");
    println!("creating dir {}", temp_dir.display());
    if let Err(err) = fs::create_dir_all(temp_dir) {
        panic!(
            "Unable to create temp directory {} - {}",
            temp_dir.display(),
            err
        );
    }
    let name = match std::thread::current().name() {
        Some(name) => name.to_owned(),
        None => {
            panic!("Can't find thread name");
        }
    };
    let temp_git_dir = temp_dir.join(name);
    println!("creating temp repo {}", temp_git_dir.display());

    let mut opts = RepositoryInitOptions::new();
    opts.bare(true);
    opts.no_dotgit_dir(true);
    opts.no_reinit(true);

    match Repository::init_opts(temp_git_dir, &opts) {
        Ok(repo) => TestRepo(repo),
        Err(e) => panic!("failed to create temp repo: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_temp_bare_repo() {
        create_temp_bare_repo();
    }

    #[test]
    fn temp_repo_should_be_destroyed_after_it_goes_out_of_scope() {
        let name;
        {
            let repo = create_temp_bare_repo();
            name = repo.0.path().to_owned();
        } // repo goes out of scope here
        assert!(!name.is_dir());
    }
}
