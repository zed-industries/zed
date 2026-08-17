// Take a look at the license at the top of the repository in the LICENSE file.

// This test is used to ensure that the networks are not loaded by default.

#[cfg(feature = "network")]
#[test]
fn test_networks() {
    use sysinfo::Networks;

    if sysinfo::IS_SUPPORTED_SYSTEM {
        let mut n = Networks::new();
        assert_eq!(n.iter().count(), 0);
        n.refresh();
        assert_eq!(n.iter().count(), 0);
        n.refresh_list();
        assert!(n.iter().count() > 0);
    }
}
