use super::*;

#[test]
fn it_should_parse_basic_env() {
    let input = Path::new("someting/test/$CARGO_PKG_NAME");
    let result = interpoate_env(input).expect("Should parse env variable");
    let expected = PathBuf::from("something/test/dotme-path");
    assert_eq!(expected, result);
}
