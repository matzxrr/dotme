use std::{fs::OpenOptions, io::Write, path::Path};

use directories::{BaseDirs, ProjectDirs};
use thiserror::Error;

use crate::shell::{ShellConfig, ShellError};

#[derive(Debug, Error)]
pub enum PathUtilsError {
    #[error("Dirs Error: {0}")]
    DirectoriesError(&'static str),
    #[error("Shell Error: {0}")]
    ShellError(#[from] ShellError),
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

pub fn add_config_cmd_to_shell_file(path: &Path) -> Result<()> {
    let shell_config = ShellConfig::load()?;
    let alias = format!(
        "alias config='/usr/bin/git --git-dir={} --work-tree=$HOME'\n",
        path.display()
    );

    let base_dirs = base_dirs()?;
    let home = base_dirs.home_dir();
    let shell_config_file = home.join(shell_config.file);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(shell_config_file)
        .unwrap();

    file.write_all(alias.as_bytes()).unwrap();

    Ok(())
}

#[cfg(test)]
mod test_path_utils {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_add() {
        let path = PathBuf::from("/home/magreenberg/.test");
        add_config_cmd_to_shell_file(path.as_path()).unwrap();
    }
}
