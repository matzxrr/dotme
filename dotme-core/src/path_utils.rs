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
