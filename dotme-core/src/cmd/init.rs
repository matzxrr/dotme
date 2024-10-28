use anyhow::Result;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

use dialoguer::{theme::ColorfulTheme, Input};
use std::io::Write;

use crate::path_utils::{base_dirs, parse_into_absolute};
use crate::repo::Repo;
use crate::shell::ShellConfig;

pub fn init() -> Result<()> {
    println!("\n\nSetting up a new dotfile repo\n");

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

    let repo_path = PathBuf::from(repo_location);
    let absolute_repo_path = parse_into_absolute(&repo_path);

    print!(
        "\n1. Creating bare git repository at '{}'... ",
        repo_path.display()
    );
    Repo::create_bare_repo(&absolute_repo_path)?;
    println!("Done");

    print!("2. Writing 'config' command to your shell file... ");
    add_config_cmd_to_shell_file(&repo_path)?;
    println!("Done\n");

    Ok(())
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
        .append(true)
        .open(shell_config_file)
        .unwrap();

    file.write_all(alias.as_bytes()).unwrap();

    Ok(())
}
