// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(all(feature = "system", feature = "disk"))]
fn should_skip() -> bool {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return true;
    }

    // If we don't have any physical core present, it's very likely that we're inside a VM...
    sysinfo::System::physical_core_count().unwrap_or_default() == 0
}

#[test]
#[cfg(all(feature = "system", feature = "disk"))]
fn test_disks() {
    if should_skip() {
        return;
    }

    let mut disks = sysinfo::Disks::new();
    assert!(disks.list().is_empty());
    disks.refresh(false);
    assert!(!disks.list().is_empty());
}

#[test]
#[cfg(all(feature = "system", feature = "disk"))]
fn test_disk_refresh_kind() {
    use itertools::Itertools;

    use sysinfo::{DiskKind, DiskRefreshKind, Disks};

    if should_skip() {
        return;
    }

    for fs in [
        DiskRefreshKind::with_kind,
        DiskRefreshKind::without_kind,
        DiskRefreshKind::with_storage,
        DiskRefreshKind::without_storage,
        DiskRefreshKind::with_io_usage,
        DiskRefreshKind::without_io_usage,
    ]
    .iter()
    .powerset()
    {
        let mut refreshes = DiskRefreshKind::nothing();
        for f in fs {
            refreshes = f(refreshes);
        }

        let assertions = |name: &'static str, disks: &Disks| {
            if refreshes.kind() {
                // This would ideally assert that *all* are refreshed, but we settle for a weaker
                // assertion because failures can't be distinguished from "not refreshed" values.
                #[cfg(not(any(target_os = "freebsd", target_os = "windows")))]
                assert!(
                    disks
                        .iter()
                        .any(|disk| disk.kind() != DiskKind::Unknown(-1)),
                    "{name}: disk.kind should be refreshed"
                );
            } else {
                assert!(
                    disks
                        .iter()
                        .all(|disk| disk.kind() == DiskKind::Unknown(-1)),
                    "{name}: disk.kind should not be refreshed"
                );
            }

            if refreshes.storage() {
                // These would ideally assert that *all* are refreshed, but we settle for a weaker
                // assertion because failures can't be distinguished from "not refreshed" values.
                assert!(
                    disks
                        .iter()
                        .any(|disk| disk.available_space() != Default::default()),
                    "{name}: disk.available_space should be refreshed"
                );
                assert!(
                    disks
                        .iter()
                        .any(|disk| disk.total_space() != Default::default()),
                    "{name}: disk.total_space should be refreshed"
                );
                // We can't assert anything about booleans, since false is indistinguishable from
                // not-refreshed
            } else {
                assert!(
                    disks
                        .iter()
                        .all(|disk| disk.available_space() == Default::default()),
                    "{name}: disk.available_space should not be refreshed"
                );
                assert!(
                    disks
                        .iter()
                        .all(|disk| disk.total_space() == Default::default()),
                    "{name}: disk.total_space should not be refreshed"
                );
            }

            if refreshes.io_usage() {
                // This would ideally assert that *all* are refreshed, but we settle for a weaker
                // assertion because failures can't be distinguished from "not refreshed" values.
                assert!(
                    disks.iter().any(|disk| disk.usage() != Default::default()),
                    "{name}: disk.usage should be refreshed"
                );
            } else {
                assert!(
                    disks.iter().all(|disk| disk.usage() == Default::default()),
                    "{name}: disk.usage should not be refreshed"
                );
            }
        };

        // load and refresh with the desired details should work
        let disks = Disks::new_with_refreshed_list_specifics(refreshes);
        assertions("full", &disks);

        // load with minimal `DiskRefreshKind`, then refresh for added detail should also work!
        let mut disks = Disks::new_with_refreshed_list_specifics(DiskRefreshKind::nothing());
        disks.refresh_specifics(false, refreshes);
        assertions("incremental", &disks);
    }
}

#[test]
#[cfg(all(feature = "system", feature = "disk"))]
fn test_disks_usage() {
    use std::fs::{File, remove_file};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::thread::sleep;

    use sysinfo::Disks;

    if should_skip() {
        return;
    }

    // The test always fails in CI on Linux. For some unknown reason, /proc/diskstats just doesn't
    // update, regardless of how long we wait. Until the root cause is discovered, skip the test
    // in CI.
    if cfg!(target_os = "linux") && std::env::var("CI").is_ok() {
        return;
    }

    let mut disks = Disks::new_with_refreshed_list();

    let path = match std::env::var("CARGO_TARGET_DIR") {
        Ok(p) => Path::new(&p).join("data.tmp"),
        _ => PathBuf::from("target/data.tmp"),
    };
    let mut file = File::create(&path).expect("failed to create temporary file");

    // Write 10mb worth of data to the temp file.
    let data = vec![1u8; 10 * 1024 * 1024];
    file.write_all(&data).unwrap();
    // The sync_all call is important to ensure all the data is persisted to disk. Without
    // the call, this test is flaky.
    file.sync_all().unwrap();

    // Wait a bit just in case
    sleep(std::time::Duration::from_millis(500));
    disks.refresh(false);

    // Depending on the OS and how disks are configured, the disk usage may be the exact same
    // across multiple disks. To account for this, collect the disk usages and dedup.
    let mut disk_usages = disks.list().iter().map(|d| d.usage()).collect::<Vec<_>>();
    disk_usages.dedup();

    let mut written_bytes = 0;
    for disk_usage in disk_usages {
        written_bytes += disk_usage.written_bytes;
    }

    let _ = remove_file(path);

    // written_bytes should have increased by about 10mb, but this is not fully reliable in CI Linux. For now,
    // just verify the number is non-zero.
    assert!(written_bytes > 0);
}
