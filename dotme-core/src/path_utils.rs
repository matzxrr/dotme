use std::{
    fs::{self, OpenOptions},
    io::{self, BufReader, Write},
    path::Path,
    process::Command,
};

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

/*
Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process")

Reading bash file
basename $(readlink /proc/$$/exe)
ps -o comm= -p $$
## My Aliases
alias config='/usr/bin/git --git-dir=$HOME/.cfg/ --work-tree=$HOME'
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("my-file")
        .unwrap();

    if let Err(e) = writeln!(file, "A new line!") {
        eprintln!("Couldn't write to file: {}", e);
    }
*/
pub fn add_to_bashrc(path: &Path) -> Result<()> {
    let base_dirs = base_dirs()?;
    let home_dirs = base_dirs.home_dir();
    let bashrc = home_dirs.join(".bashrc");
    let _ = home_dirs.join(".");
    let alias = format!(
        r"alias config='/usr/bin/git --git-dir={} --work-tree=$HOME'",
        path.display()
    );

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(bashrc)
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
        add_to_bashrc(path.as_path()).unwrap();
    }
}
