// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg(feature = "system")]
#![allow(clippy::assertions_on_constants)]

use sysinfo::{ProcessesToUpdate, System};

#[test]
fn test_refresh_system() {
    let mut sys = System::new();
    sys.refresh_memory();
    sys.refresh_cpu_usage();
    // We don't want to test on unsupported systems.
    if sysinfo::IS_SUPPORTED_SYSTEM {
        assert!(sys.total_memory() != 0);
        assert!(sys.free_memory() != 0);
    }
    assert!(sys.total_memory() >= sys.free_memory());
    assert!(sys.total_swap() >= sys.free_swap());
}

#[test]
fn test_refresh_process() {
    let mut sys = System::new();
    assert!(sys.processes().is_empty(), "no process should be listed!");
    // We don't want to test on unsupported systems.

    #[cfg(not(feature = "apple-sandbox"))]
    if sysinfo::IS_SUPPORTED_SYSTEM {
        assert_eq!(
            sys.refresh_processes(ProcessesToUpdate::Some(&[
                sysinfo::get_current_pid().expect("failed to get current pid")
            ])),
            1,
            "process not listed",
        );
        // Ensure that the process was really added to the list!
        assert!(sys
            .process(sysinfo::get_current_pid().expect("failed to get current pid"))
            .is_some());
    }
}

#[test]
fn test_get_process() {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All);
    let current_pid = match sysinfo::get_current_pid() {
        Ok(pid) => pid,
        _ => {
            if !sysinfo::IS_SUPPORTED_SYSTEM {
                return;
            }
            panic!("get_current_pid should work!");
        }
    };
    if let Some(p) = sys.process(current_pid) {
        assert!(p.memory() > 0);
    } else {
        #[cfg(not(feature = "apple-sandbox"))]
        assert!(!sysinfo::IS_SUPPORTED_SYSTEM);
    }
}

#[test]
fn check_if_send_and_sync() {
    trait Foo {
        fn foo(&self) {}
    }
    impl<T> Foo for T where T: Send {}

    trait Bar {
        fn bar(&self) {}
    }

    impl<T> Bar for T where T: Sync {}

    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All);
    let current_pid = match sysinfo::get_current_pid() {
        Ok(pid) => pid,
        _ => {
            if !sysinfo::IS_SUPPORTED_SYSTEM {
                return;
            }
            panic!("get_current_pid should work!");
        }
    };
    if let Some(p) = sys.process(current_pid) {
        p.foo(); // If this doesn't compile, it'll simply mean that the Process type
                 // doesn't implement the Send trait.
        p.bar(); // If this doesn't compile, it'll simply mean that the Process type
                 // doesn't implement the Sync trait.
    } else {
        #[cfg(not(feature = "apple-sandbox"))]
        assert!(!sysinfo::IS_SUPPORTED_SYSTEM);
    }
}

#[test]
fn check_hostname_has_no_nuls() {
    if let Some(hostname) = System::host_name() {
        assert!(!hostname.contains('\u{0}'))
    }
}

#[test]
fn check_uptime() {
    let uptime = System::uptime();
    if sysinfo::IS_SUPPORTED_SYSTEM {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let new_uptime = System::uptime();
        assert!(uptime < new_uptime);
    }
}

#[test]
fn check_boot_time() {
    if sysinfo::IS_SUPPORTED_SYSTEM {
        assert_ne!(System::boot_time(), 0);
    }
}

// This test is used to ensure that the CPU usage computation isn't completely going off
// when refreshing it too frequently (ie, multiple times in a row in a very small interval).
#[test]
#[ignore] // This test MUST be run on its own to prevent wrong CPU usage measurements.
fn test_consecutive_cpu_usage_update() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use sysinfo::{Pid, ProcessRefreshKind, System};

    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    let mut sys = System::new_all();
    assert!(!sys.cpus().is_empty());
    sys.refresh_processes_specifics(ProcessesToUpdate::All, ProcessRefreshKind::new().with_cpu());

    let stop = Arc::new(AtomicBool::new(false));
    // Spawning a few threads to ensure that it will actually have an impact on the CPU usage.
    for it in 0..sys.cpus().len() / 2 + 1 {
        let stop_c = Arc::clone(&stop);
        std::thread::spawn(move || {
            while !stop_c.load(Ordering::Relaxed) {
                if it != 0 {
                    // The first thread runs at 100% to be sure it'll be noticeable.
                    std::thread::sleep(Duration::from_millis(1));
                }
            }
        });
    }

    let mut pids = sys
        .processes()
        .iter()
        .map(|(pid, _)| *pid)
        .take(2)
        .collect::<Vec<_>>();
    let pid = std::process::id();
    pids.push(Pid::from_u32(pid));
    assert_eq!(pids.len(), 3);

    for it in 0..3 {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL + Duration::from_millis(1));
        for pid in &pids {
            sys.refresh_processes_specifics(
                ProcessesToUpdate::Some(&[*pid]),
                ProcessRefreshKind::new().with_cpu(),
            );
        }
        // To ensure that Linux doesn't give too high numbers.
        assert!(
            sys.process(pids[2]).unwrap().cpu_usage() < sys.cpus().len() as f32 * 100.,
            "using ALL CPU: failed at iteration {}",
            it
        );
        // To ensure it's not 0 either.
        assert!(
            sys.process(pids[2]).unwrap().cpu_usage() > 0.,
            "using NO CPU: failed at iteration {}",
            it
        );
    }
    stop.store(false, Ordering::Relaxed);
}

#[test]
fn test_refresh_memory() {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }
    // On linux, since it's the same file, memory information are always retrieved.
    let is_linux = cfg!(any(target_os = "linux", target_os = "android"));
    let mut s = System::new();
    assert_eq!(s.total_memory(), 0);
    assert_eq!(s.free_memory(), 0);

    s.refresh_memory_specifics(sysinfo::MemoryRefreshKind::new().with_ram());
    assert_ne!(s.total_memory(), 0);
    assert_ne!(s.free_memory(), 0);

    if is_linux {
        assert_ne!(s.total_swap(), 0);
        assert_ne!(s.free_swap(), 0);
    } else {
        assert_eq!(s.total_swap(), 0);
        assert_eq!(s.free_swap(), 0);
    }

    let mut s = System::new();
    assert_eq!(s.total_swap(), 0);
    assert_eq!(s.free_swap(), 0);

    if std::env::var("APPLE_CI").is_ok() {
        // Apparently there is no swap for macOS in CIs so can't run futher than this point.
        return;
    }

    s.refresh_memory_specifics(sysinfo::MemoryRefreshKind::new().with_swap());
    // SWAP can be 0 on macOS so this test is disabled
    #[cfg(not(target_os = "macos"))]
    {
        assert_ne!(s.total_swap(), 0);
        assert_ne!(s.free_swap(), 0);
    }

    if is_linux {
        assert_ne!(s.total_memory(), 0);
        assert_ne!(s.free_memory(), 0);
    } else {
        assert_eq!(s.total_memory(), 0);
        assert_eq!(s.free_memory(), 0);
    }

    let mut s = System::new();
    s.refresh_memory();
    // SWAP can be 0 on macOS so this test is disabled
    #[cfg(not(target_os = "macos"))]
    {
        assert_ne!(s.total_swap(), 0);
        assert_ne!(s.free_swap(), 0);
    }
    assert_ne!(s.total_memory(), 0);
    assert_ne!(s.free_memory(), 0);
}
