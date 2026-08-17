// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "component")]
#[test]
fn test_components() {
    use std::env::var;

    let mut c = sysinfo::Components::new();
    assert!(c.is_empty());

    // Unfortunately, we can't get components in the CI...
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(windows) || var("CI").is_ok() {
        return;
    }

    c.refresh_list();
    assert!(!c.is_empty());
}
