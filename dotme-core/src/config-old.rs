use serde_derive::Deserialize;
use std::io::Error as IoError;
use std::{fmt::Display, fs, path::PathBuf};
use toml::de::Error as TomlError;

use crate::path_utils::{base_dirs, project_dirs, PathUtilsError};

#[derive(Debug)]
pub enum ConfigLoadError {
    DirectoryError(&'static str),
    IoError(IoError),
    DeserializeError(TomlError),
    PathUtilsError(PathUtilsError),
}

impl From<IoError> for ConfigLoadError {
    fn from(value: IoError) -> Self {
        ConfigLoadError::IoError(value)
    }
}

impl From<TomlError> for ConfigLoadError {
    fn from(value: TomlError) -> Self {
        ConfigLoadError::DeserializeError(value)
    }
}

impl From<PathUtilsError> for ConfigLoadError {
    fn from(value: PathUtilsError) -> Self {
        ConfigLoadError::PathUtilsError(value)
    }
}

impl Display for ConfigLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigLoadError::DirectoryError(e) => write!(f, "Directory error: {}", e),
            ConfigLoadError::IoError(e) => write!(f, "IO Error: {}", e),
            ConfigLoadError::DeserializeError(e) => write!(f, "Deserialize Error: {}", e),
            ConfigLoadError::PathUtilsError(e) => write!(f, "Path Util Error: {}", e),
        }
    }
}

impl std::error::Error for ConfigLoadError {}

type Result<T> = std::result::Result<T, ConfigLoadError>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct RawConfig {
    dotme_repo: DotmeRepo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct DotmeRepo {
    name: String,
    location: String,
}

impl RawConfig {
    pub fn load(config_string: &str) -> Result<RawConfig> {
        let raw_config: RawConfig = toml::from_str(config_string)?;
        Ok(raw_config)
    }
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
    fn load(config_string: &str) -> Result<Config> {
        let raw_config = RawConfig::load(config_string)?;
        let base_dirs = base_dirs()?;
        let location = if raw_config.dotme_repo.location.to_ascii_lowercase() == "home" {
            base_dirs.home_dir().to_path_buf()
        } else {
            PathBuf::from(raw_config.dotme_repo.location)
        };
        let repo = location.join(&raw_config.dotme_repo.name);
        let work_tree = base_dirs.home_dir().to_path_buf();
        Ok(Config::new(repo, work_tree))
    }
}

/// Loads the dotme config
/// Linux Path: ~/.config/dotme/config.toml
pub fn load_dotme_config() -> Result<Config> {
    let project_dirs = project_dirs()?;
    let config_base_dir = project_dirs.config_dir();
    let config_toml_path = config_base_dir.join("config.toml");
    let config_string = fs::read_to_string(config_toml_path)?;
    Config::load(&config_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_config() {
        let test_config = include_str!("../config.toml");
        Config::load(test_config).unwrap();
    }
}
