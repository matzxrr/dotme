use std::path::Path;

use crate::path_utils::{self};
use anyhow::{anyhow, Result};
use git2::{ErrorCode, Repository};

/// A struct that wraps `crate::config::Config` and `git2::Repository`
/// making it easy to load the dotme repo.
///
/// Does some basic validation of the repo's config settings on `load()`.
///
/// Has some helper functions
pub struct Repo {
    pub repo: Repository,
}

impl Repo {
    pub fn validate_config(&self) -> Result<()> {
        if !self.repo.is_bare() {
            Err(anyhow!("repo needs to be a bare repository"))
        } else {
            Ok(())
        }
    }

    pub fn set_worktree(&mut self, input: Option<&Path>) -> Result<()> {
        let path = match input {
            Some(path) => path.to_path_buf(),
            None => {
                let base_dirs = path_utils::base_dirs()?;
                base_dirs.home_dir().to_path_buf()
            }
        };
        self.repo.set_workdir(&path, false).map_err(Into::into)
    }

    pub fn get_branch_name(&self) -> Result<String> {
        let head = match self.repo.head() {
            Ok(head) => head,
            Err(ref e)
                if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound =>
            {
                return Err(anyhow!("Unknown branch"));
            }
            Err(e) => return Err(Into::into(e)),
        };
        head.shorthand()
            .map(|x| x.to_owned())
            .ok_or_else(|| anyhow!("Branch not utf8"))
    }

    pub fn create_bare_repo(path: &Path) -> Result<Repo> {
        let mut repo = Repo {
            repo: Repository::init_bare(path)?,
        };
        repo.set_worktree(None)?;
        repo.set_default_config()?;
        Ok(repo)
    }

    pub fn set_default_config(&self) -> Result<()> {
        let mut config = self.repo.config()?;
        config.set_str("status.showuntrackedfiles", "no")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use dotme_test::create_temp_bare_repo;

    use super::*;

    #[test]
    fn test_set_default_config() {
        let temp = create_temp_bare_repo();
        let repo = Repo {
            repo: temp.clone_repo().unwrap(),
        };
        repo.set_default_config().unwrap();
        let config = repo.repo.config().unwrap();
        // set_str config command doesn't work
        let show_untracked = config.get_entry("status.showuntrackedfiles").unwrap();
        assert_eq!(show_untracked.value(), Some("no"));
    }
}
