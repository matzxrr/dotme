use std::{fmt::Display, path::PathBuf};

use directories::{BaseDirs, ProjectDirs};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfigRaw {
    pub dotme_repo: DotmeRepo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DotmeRepo {
    pub name: String,
    pub location: String,
}

#[derive(Debug)]
pub struct Config {
    pub repo: PathBuf,
    pub work_tree: PathBuf,
}

impl Config {
    pub fn new(repo: PathBuf, work_tree: PathBuf) -> Self {
        Self { repo, work_tree }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            repo: PathBuf::from(".cfg"),
            work_tree: PathBuf::new(),
        }
    }
}

#[derive(Debug)]
pub enum ConfigLoadError {
    DirectoryError(&'static str),
}

impl Display for ConfigLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigLoadError::DirectoryError(e) => write!(f, "Directory error: {}", e),
        }
    }
}

impl std::error::Error for ConfigLoadError {}

type Result<T> = std::result::Result<T, ConfigLoadError>;

// impl From<&ConfigToml> for Config {
//     fn from(value: &ConfigToml) -> Self {
//         let base_dirs = base_dirs();
//         let location = if value
//             .dotmerepo
//             .location
//             .to_str()
//             .is_some_and(|s| s.to_ascii_lowercase() == "home")
//         {
//             base_dirs.home_dir().to_path_buf()
//         } else {
//             value.dotmerepo.location.to_owned()
//         };
//         let repo = location.join(&value.dotmerepo.name);
//         Config::new(repo, base_dirs.home_dir().to_path_buf())
//     }
// }

fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("", "", "dotme").ok_or(ConfigLoadError::DirectoryError(
        "Cannot get project directories",
    ))
}

fn base_dirs() -> Result<BaseDirs> {
    BaseDirs::new().ok_or(ConfigLoadError::DirectoryError(
        "Cannot get base directories",
    ))
}

// pub fn read_dotme_config() -> ConfigToml {
//     let dirs = project_dirs();
//     let config_base_dir = dirs.config_dir();
//     let config_toml_path = config_base_dir.join("config.toml");
//     let config = fs::read_to_string(config_toml_path).unwrap();
//     ConfigToml::load(&config).unwrap()
// }

impl Config {
    /// Loads a config based on the input string
    pub fn load() -> Config {
        Config::default()
    }
    /// Loades the default config
    pub fn load_default() -> Config {
        Config::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erors() {
        let dirs = project_dirs();
        if let Err(e) = dirs {
            println!("{:?}", e);
        }
    }
}
