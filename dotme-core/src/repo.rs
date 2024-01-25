use std::path::Path;

use crate::config::{load_dotme_config, Config};
use crate::path_utils::{self, PathUtilsError};
use git2::Error as Git2Error;
use git2::{ErrorCode, Repository};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("repoistory config is invalid: {0}")]
    InvalidRepoConfig(String),
    #[error("loading config.toml file: {0}")]
    ConfigLoadError(#[from] crate::config::ConfigLoadError),
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
    pub config: Config,
}

impl Repo {
    pub fn load() -> Result<Self> {
        let config = load_dotme_config()?;
        let repo = Repository::open(&config.repo)?;
        let repo = Repo { repo, config };
        repo.validate_config()?;
        Ok(repo)
    }

    pub fn validate_config(&self) -> Result<()> {
        if !self.repo.is_bare() {
            Err(RepoError::InvalidRepoConfig(String::from(
                "not a bare repo",
            )))
        } else {
            Ok(())
        }
    }

    pub fn set_worktree(&mut self) -> Result<()> {
        self.repo
            .set_workdir(&self.config.work_tree, false)
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
        let base_dirs = path_utils::base_dirs()?;
        let workspace_path = base_dirs.home_dir();
        let config = Config::new(path.to_path_buf(), workspace_path.to_path_buf());
        let repo = Repository::init_bare(path)?;
        Ok(Repo { repo, config })
    }
}
