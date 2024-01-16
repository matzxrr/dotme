use crate::config::{load_dotme_config, Config};
use git2::Repository;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("repoistory config is invalid: {0}")]
    InvalidRepoConfig(String),
    #[error("loading config.toml file: {0}")]
    ConfigLoadError(#[from] crate::config::ConfigLoadError),
    #[error("git2: {0}")]
    Git2Error(#[from] git2::Error),
}

type Result<T> = std::result::Result<T, RepoError>;

/// A struct that wraps `crate::config::Config` and `git2::Repository`
/// making it easy to load the dotme repo.
///
/// Does some basic validation of the repo's config settings on `load()`.
///
/// ```
/// let repo = Repo::load().unwrap();
/// let r = repo.repo;   // <- git2::Repository
/// let c = repo.config; // <- crate::config::Config
/// ```
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
}
