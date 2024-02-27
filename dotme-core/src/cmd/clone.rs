use std::path::{Path, PathBuf};

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};

pub fn clone(_remote: &Path) -> Result<()> {
    println!("\n\nCloning a repository");

    let default_path = String::from("$HOME/.cfg");

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

    dbg!(repo_location);

    Ok(())
}
