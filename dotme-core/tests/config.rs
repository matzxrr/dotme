use dotme_core::config::Config;

#[test]
fn it_should_read_config_yaml() {
    let test_config = include_str!("config.toml");
    Config::load(test_config).unwrap();
}
