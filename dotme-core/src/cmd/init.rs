use std::path::PathBuf;
use thiserror::Error;

use dialoguer::{theme::ColorfulTheme, Error as DialoguerError, Input};

use console::Term;

use crate::path_utils::{add_to_bashrc, base_dirs};
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
}

type Result<T> = std::result::Result<T, InitError>;

pub fn init() -> Result<()> {
    let term = Term::stdout();
    term.write_line("\n\nSetting up a new dotfile repo\n")?;
    let default_path = get_default_path()?;
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

    term.write_line(&format!(
        "\nCreating bare git repository at '{}'",
        repo_location_path.display()
    ))?;
    let repo = Repo::create_bare_repo(repo_location_path.as_path())?;
    term.write_line(&format!(
        "Bare repostiory created at {}",
        repo.repo.path().display()
    ))?;

    add_to_bashrc(repo.repo.path()).unwrap();

    let repo_config = repo.repo.config().unwrap();
    repo_config.get_bool("showUntrackedFiles").expect("exists");

    Ok(())
}

fn get_default_path() -> Result<String> {
    let base_dirs = base_dirs().map_err(|_| InitError::DefaultPathError)?;
    let path = base_dirs.home_dir().to_path_buf();
    let path = path.join(".cfg");
    path.to_str()
        .ok_or(InitError::DefaultPathError)
        .map(ToString::to_string)
}

#[cfg(test)]
mod test_init {
    use git2::Repository;

    use super::*;

    #[test]
    fn test_init_repo() {
        let repo = Repository::open("/home/magreenberg/.test").unwrap();
        let config = repo.config().unwrap();
        let _val = config.get_string("GIT_SUBMODULE_IGNORE_UNTRACKED").unwrap();
    }
}
