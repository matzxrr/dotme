use std::{fmt::Display, path::PathBuf};

use directories::{BaseDirs, ProjectDirs};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathUtilsError {
    #[error("Dirs Error: {0}")]
    DirectoriesError(&'static str),
}

type Result<T> = std::result::Result<T, PathUtilsError>;

pub fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("", "", "dotme").ok_or(PathUtilsError::DirectoriesError(
        "Cannot get project directories",
    ))
}

pub fn base_dirs() -> Result<BaseDirs> {
    BaseDirs::new().ok_or(PathUtilsError::DirectoriesError(
        "Cannot get base directories",
    ))
}

pub struct RepoPathLocation {
    is_absolute: bool,
    path: PathBuf,
}

impl Display for RepoPathLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.is_absolute {
            true => write!(f, "{}", self.path.display()),
            false => write!(f, "$HOME/{}", self.path.display()),
        }
    }
}

impl Default for RepoPathLocation {
    fn default() -> Self {
        Self {
            is_absolute: false,
            path: PathBuf::from(".cfg"),
        }
    }
}
