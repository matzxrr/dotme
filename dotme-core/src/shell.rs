use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("Can't find the users shell, might not be supported")]
    CannotFindShell,
}

type Result<T> = std::result::Result<T, ShellError>;

#[derive(Default)]
pub struct ShellConfig {
    pub shell: String,
    pub path: String,
    pub file: String,
}

impl ShellConfig {
    /// This should load the users shell information
    /// Currently only works if the SHELL env variable is set
    ///
    /// TODO: Need to find fallback ways of checking a users shell
    pub fn load() -> Result<ShellConfig> {
        let shell_result = env::var("SHELL");
        if let Ok(shell_path) = shell_result {
            if let Some(shell_name) = get_shell_name_from_path(&shell_path) {
                if let Some(shell_file) = match_shell_to_config_file(&shell_name) {
                    return Ok(ShellConfig {
                        shell: shell_name,
                        path: shell_path,
                        file: shell_file,
                    });
                }
            }
        }
        Err(ShellError::CannotFindShell)
    }
}

fn match_shell_to_config_file(shell: &str) -> Option<String> {
    match shell {
        "bash" => Some(String::from(".bashrc")),
        "zsh" => Some(String::from(".zshrc")),
        _ => None,
    }
}

fn get_shell_name_from_path(shell: &str) -> Option<String> {
    let shell_name = shell.split('/').last().expect("should have last value");
    match shell_name {
        "bash" => Some(String::from("bash")),
        "zsh" => Some(String::from("zsh")),
        // "fish" => Some(String::from("fish")),
        // "sh" => Some(String::from("sh")),
        _ => None,
    }
}

#[cfg(test)]
mod shell_tests {
    use super::*;

    #[test]
    fn it_should_load_users_shell() {
        let shell = ShellConfig::load().expect("load shell");
        assert_eq!(&shell.shell, "bash");
        assert_eq!(&shell.file, ".bashrc");
        assert_eq!(&shell.path, "/bin/bash");
    }

    #[test]
    fn test_something() {
        if let Ok(exe_path) = env::current_exe() {
            if let Some(parent) = exe_path.parent() {
                println!("{}", parent.display());
            }
        }
    }
}
