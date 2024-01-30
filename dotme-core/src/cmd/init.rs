use std::path::PathBuf;
use thiserror::Error;

use dialoguer::{theme::ColorfulTheme, Error as DialoguerError, Input};

use crate::path_utils::{
    add_config_cmd_to_shell_file, base_dirs, PathUtilsError, RepoPathLocation,
};
use crate::repo::{Repo, RepoError};

#[derive(Debug, Error)]
pub enum InitError {
    #[error("Default path error")]
    DefaultPathError,
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Dialoguer Error: {0}")]
    DialoguerError(#[from] DialoguerError),
    #[error("Repo Error: {0}")]
    RepoError(#[from] RepoError),
    #[error("Path Utils: {0}")]
    PathUtilsError(#[from] PathUtilsError),
}

type Result<T> = std::result::Result<T, InitError>;

pub fn init() -> Result<()> {
    println!("\n\nSetting up a new dotfile repo\n");

    let mut repo_location = RepoPathLocation::default();

    let repo_location: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Repo location")
        .default(default_path)
        .validate_with(|input: &String| {
            let path = PathBuf::from(input);
            if path.is_dir() {
                Err("Directory already exists")
            } else {
                Ok(())
            }
        })
        .interact_text()?;
    let repo_location_path = PathBuf::from(repo_location);

    // print!(
    //     "\n1. Creating bare git repository at '{}'... ",
    //     repo_location_path.display()
    // );
    // let _repo = Repo::create_bare_repo(&repo_location_path)?;
    // println!("Done");

    // print!("2. Writing 'config' command to your shell file... ");
    // add_config_cmd_to_shell_file(repo.repo.path())?;
    // println!("Done\n");

    Ok(())
}

// pub fn add_config_cmd_to_shell_file(path: &Path) -> Result<()> {
//     let shell_config = ShellConfig::load()?;
//     let alias = format!(
//         "alias config='/usr/bin/git --git-dir={} --work-tree=$HOME'\n",
//         path.display()
//     );

//     let base_dirs = base_dirs()?;
//     let home = base_dirs.home_dir();
//     let shell_config_file = home.join(shell_config.file);

//     let mut file = OpenOptions::new()
//         .write(true)
//         .append(true)
//         .open(shell_config_file)
//         .unwrap();

//     file.write_all(alias.as_bytes()).unwrap();

//     Ok(())
// }

fn get_default_path() -> Result<String> {
    let base_dirs = base_dirs().map_err(|_| InitError::DefaultPathError)?;
    let path = base_dirs.home_dir();
    let path = path.join(".cfg");
    path.to_str()
        .ok_or(InitError::DefaultPathError)
        .map(ToString::to_string)
}
