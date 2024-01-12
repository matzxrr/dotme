use anyhow::Result;
use std::{fs, path::PathBuf, process::exit};

use directories::{BaseDirs, ProjectDirs};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigToml {
    pub dotmerepo: DotmeRepo,
}

#[derive(Debug, Deserialize)]
pub struct DotmeRepo {
    pub name: String,
    pub location: PathBuf,
}

impl ConfigToml {
    pub fn load(config_str: &str) -> Result<Self> {
        let config: Self = toml::from_str(config_str)?;
        Ok(config)
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
}

impl From<&ConfigToml> for Config {
    fn from(value: &ConfigToml) -> Self {
        let base_dirs = base_dirs();
        let location = if value
            .dotmerepo
            .location
            .to_str()
            .is_some_and(|s| s.to_ascii_lowercase() == "home")
        {
            base_dirs.home_dir().to_path_buf()
        } else {
            value.dotmerepo.location.to_owned()
        };
        let repo = location.join(&value.dotmerepo.name);
        Config::new(repo, base_dirs.home_dir().to_path_buf())
    }
}

fn project_dirs() -> ProjectDirs {
    match ProjectDirs::from("", "", "dotme") {
        Some(p) => p,
        None => {
            eprintln!("Could not find Project Directory base");
            exit(1);
        }
    }
}

fn base_dirs() -> BaseDirs {
    match BaseDirs::new() {
        Some(p) => p,
        None => {
            eprintln!("Could not find Base Directories");
            exit(1);
        }
    }
}

pub fn read_dotme_config() -> ConfigToml {
    let dirs = project_dirs();
    let config_base_dir = dirs.config_dir();
    let config_toml_path = config_base_dir.join("config.toml");
    let config = fs::read_to_string(config_toml_path).unwrap();
    ConfigToml::load(&config).unwrap()
}
