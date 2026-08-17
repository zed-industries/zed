#[test]
fn version_matches_cargo_pkg_version() {
    assert_eq!(dugong::VERSION, env!("CARGO_PKG_VERSION"));
    assert!(!dugong::VERSION.is_empty());
}
