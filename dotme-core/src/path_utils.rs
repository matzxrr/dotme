use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use directories::{BaseDirs, ProjectDirs};

pub fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("", "", "dotme").ok_or(anyhow!("cant find root config for dotme"))
}

pub fn base_dirs() -> Result<BaseDirs> {
    BaseDirs::new().ok_or(anyhow!("Cannot get base directories",))
}

/// This function does a thing
pub fn parse_into_absolute(path: &Path) -> PathBuf {
    path.strip_prefix("$HOME")
        .ok()
        .and_then(|rest| {
            let home = BaseDirs::new().map(|x| x.home_dir().to_path_buf());
            home.map(|x| x.join(rest))
        })
        .unwrap_or(path.to_path_buf())
}
