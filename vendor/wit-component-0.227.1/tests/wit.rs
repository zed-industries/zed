#![cfg(feature = "wat")]

use anyhow::Result;
use wit_parser::Resolve;

/// Ensure that parse_wit_from_path works with directories
#[test]
fn parse_wit_dir() -> Result<()> {
    drop(env_logger::try_init());

    let mut resolver = Resolve::default();
    let (package_id, _) = resolver.push_path("tests/wit/parse-dir/wit")?;
    assert!(resolver
        .select_world(package_id, "foo-world".into())
        .is_ok());

    Ok(())
}

/// Ensure that parse_wit_from_path works for a single file
#[test]
fn parse_wit_file() -> Result<()> {
    drop(env_logger::try_init());

    let mut resolver = Resolve::default();
    let (package_id, _) = resolver.push_path("tests/wit/parse-dir/wit/deps/bar/bar.wit")?;
    resolver.select_world(package_id, "bar-world".into())?;
    assert!(resolver
        .interfaces
        .iter()
        .any(|(_, iface)| iface.name == Some("bar".into())));

    Ok(())
}

/// Ensure that parse_with_from_path fails for missing paths
#[test]
fn parse_wit_missing_path() -> Result<()> {
    drop(env_logger::try_init());

    let mut resolver = Resolve::default();
    assert!(resolver.push_path("tests/nonexistent/path").is_err());

    Ok(())
}
