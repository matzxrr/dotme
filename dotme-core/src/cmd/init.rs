use std::thread;
use std::time::Duration;
use std::{error::Error, path::PathBuf};

use dialoguer::{theme::ColorfulTheme, Input};

use console::Term;

use crate::path_utils::{base_dirs, verify_bash_path};

pub fn init() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    term.write_line("\n\nSetting up a new dotfile repo\n")?;

    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Repo location")
        .default("~/.cfg".to_string())
        .validate_with(validate_repo_location)
        .interact_text()?;

    term.write_line(&format!("found {}", input))?;

    thread::sleep(Duration::from_millis(2000));
    Ok(())
}

fn validate_repo_location<'a>(input: &String) -> Result<(), &'a str> {
    if input.starts_with("~") {
        verify_bash_path(input);
    }

    Ok(())
}

fn get_default_path() -> Result<String, Box<dyn Error>> {
    let base_dirs = base_dirs()?;
    let mut path = base_dirs.home_dir().to_path_buf();
    path.join(".cfg");
    path.to_str()
}
