// Take a look at the license at the top of the repository in the LICENSE file.

#[test]
#[cfg(all(feature = "system", feature = "disk"))]
fn test_disks() {
    if sysinfo::IS_SUPPORTED_SYSTEM {
        let s = sysinfo::System::new_all();
        // If we don't have any physical core present, it's very likely that we're inside a VM...
        if s.physical_core_count().unwrap_or_default() > 0 {
            let mut disks = sysinfo::Disks::new();
            assert!(disks.list().is_empty());
            disks.refresh_list();
            assert!(!disks.list().is_empty());
        }
    }
}
