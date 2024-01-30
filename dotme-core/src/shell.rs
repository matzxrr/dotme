use std::env;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ShellError {
    #[error("Can't find the users shell, might not be supported")]
    CannotFindShell,
}

type Result<T> = std::result::Result<T, ShellError>;

#[derive(Default, Debug, PartialEq)]
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
        if let Ok(shell_path) = env::var("SHELL") {
            if let Ok(shell_config) = try_parse_shell(&shell_path) {
                return Ok(shell_config);
            }
        }
        Err(ShellError::CannotFindShell)
    }
}

fn try_parse_shell(shell_path: &str) -> Result<ShellConfig> {
    if let Some(shell_name) = get_shell_name_from_path(shell_path) {
        if let Some(shell_file) = match_shell_to_config_file(&shell_name) {
            return Ok(ShellConfig {
                shell: shell_name,
                path: shell_path.to_owned(),
                file: shell_file,
            });
        }
    }
    Err(ShellError::CannotFindShell)
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
mod try_parse_shell_tests {
    use super::*;

    #[test]
    fn it_should_load_bash_shell() {
        let shell = try_parse_shell("/bin/bash").expect("'/bin/bash' should parse");
        assert_eq!(&shell.shell, "bash");
        assert_eq!(&shell.file, ".bashrc");
        assert_eq!(&shell.path, "/bin/bash");
    }

    #[test]
    fn it_should_load_zsh_shell() {
        let shell = try_parse_shell("/usr/bin/zsh").expect("'/usr/bin/zsh' should parse");
        assert_eq!(&shell.shell, "zsh");
        assert_eq!(&shell.file, ".zshrc");
        assert_eq!(&shell.path, "/usr/bin/zsh");
    }
    #[test]
    fn it_shouldnt_load_unknown_shell() {
        let shell = try_parse_shell("/usr/bin/unknown");
        assert_eq!(shell, Err(ShellError::CannotFindShell));
    }
}
