use std::time::Duration;
use std::{path::PathBuf, thread};
use thiserror::Error;

use dialoguer::{theme::ColorfulTheme, Error as DialoguerError, Input};

use console::Term;

use crate::path_utils::base_dirs;

#[derive(Debug, Error)]
pub enum InitError {
    #[error("Default path error")]
    DefaultPathError,
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Dialoguer Error: {0}")]
    DialoguerError(#[from] DialoguerError),
}

type Result<T> = std::result::Result<T, InitError>;

pub fn init() -> Result<()> {
    let term = Term::stdout();
    term.write_line("\n\nSetting up a new dotfile repo\n")?;
    let default_path = get_default_path()?;
    let input: String = Input::with_theme(&ColorfulTheme::default())
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

    term.write_line(&format!("\nCreating bare git repository at '{}'", input))?;
    thread::sleep(Duration::from_millis(2000));
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
