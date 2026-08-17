// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg(feature = "system")]

// This test is used to ensure that the CPUs are not loaded by default.
#[test]
fn test_cpu() {
    let mut s = sysinfo::System::new();
    assert!(s.cpus().is_empty());

    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    s.refresh_cpu_all();
    assert!(!s.cpus().is_empty());

    let s = sysinfo::System::new_all();
    assert!(!s.cpus().is_empty());

    assert!(!s.cpus()[0].brand().chars().any(|c| c == '\0'));

    if !cfg!(target_os = "freebsd") {
        // This information is currently not retrieved on freebsd...
        assert!(s.cpus().iter().any(|c| !c.brand().is_empty()));
    }
    assert!(s.cpus().iter().any(|c| !c.vendor_id().is_empty()));
}

#[test]
fn test_physical_core_numbers() {
    if sysinfo::IS_SUPPORTED_SYSTEM {
        let s = sysinfo::System::new();
        let count = s.physical_core_count();
        assert_ne!(count, None);
        assert!(count.unwrap() > 0);
    }
}

#[test]
fn test_too_rapid_cpu_refresh() {
    let mut s = sysinfo::System::new();
    assert!(s.cpus().is_empty());

    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    s.refresh_cpu_all();
    s.refresh_cpu_all();

    assert!(s.cpus().iter().any(|c| !c.cpu_usage().is_nan()));
}
