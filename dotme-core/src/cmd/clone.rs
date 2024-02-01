use std::path::{Path, PathBuf};

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input, Password};
use git2::Cred;

use crate::{
    path_utils::{is_ssh_path, parse_into_absolute},
    repo::Repo,
};

pub fn clone(remote: &Path) -> Result<()> {
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

    let repo_path = PathBuf::from(repo_location);
    let absolute_repo_path = parse_into_absolute(&repo_path);
    let is_remote_ssh = is_ssh_path(remote);

    let cred;
    if is_remote_ssh {
        let default_ssh_key = String::from("~/.ssh/id_rsa");
        let ssh_key = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("SSH Key")
            .default(default_ssh_key)
            .interact_text()?;
        let ssh_password = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("SSH Password")
            .interact();
        cred = Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )?;
    }

    // let _result = Repo::clone(remote, &absolute_repo_path)?;
    // println!("Found {}", remote);

    Ok(())
}
