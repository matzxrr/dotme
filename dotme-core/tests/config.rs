use std::path::PathBuf;

use dotme_core::config::ConfigToml;

#[test]
fn it_should_read_config_yaml() {
    let test_config = include_str!("config.toml");
    let config_toml = ConfigToml::load(test_config).unwrap();

    assert_eq!(config_toml.dotmerepo.name, String::from(".cfg"));
    assert_eq!(config_toml.dotmerepo.location, PathBuf::from("HOME"));
}
