use std::path::Path;

use crate::path_utils::{self, PathUtilsError};
use git2::Error as Git2Error;
use git2::{ErrorCode, Repository};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("repoistory config is invalid: {0}")]
    InvalidRepoConfig(String),
    #[error("git2: {0}")]
    Git2Error(#[from] Git2Error),
    #[error("unknown branch")]
    UnknownBranch,
    #[error("branch name is not valid utf-8")]
    BranchNotUtf8,
    #[error("path error: {0}")]
    PathUtilError(#[from] PathUtilsError),
}

type Result<T> = std::result::Result<T, RepoError>;

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
            Err(RepoError::InvalidRepoConfig(String::from(
                "not a bare repo",
            )))
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
        self.repo
            .set_workdir(&path, false)
            .map_err(RepoError::Git2Error)
    }

    pub fn get_branch_name(&self) -> Result<String> {
        let head = match self.repo.head() {
            Ok(head) => head,
            Err(ref e)
                if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound =>
            {
                return Err(RepoError::UnknownBranch);
            }
            Err(e) => return Err(RepoError::Git2Error(e)),
        };
        head.shorthand()
            .map(|x| x.to_owned())
            .ok_or_else(|| RepoError::BranchNotUtf8)
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
