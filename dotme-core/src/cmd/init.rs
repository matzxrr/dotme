use std::thread;
use std::time::Duration;
use std::{error::Error, path::PathBuf};

use dialoguer::{theme::ColorfulTheme, Input};

use console::Term;

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

fn validate_repo_location(input: &String) -> Result<(), &str> {
    Ok(())
}
