// Take a look at the license at the top of the repository in the LICENSE file.

#[test]
#[allow(clippy::extra_unused_type_parameters)]
#[cfg(feature = "system")]
fn test_send_sync() {
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    is_send::<sysinfo::System>();
    is_sync::<sysinfo::System>();
}
