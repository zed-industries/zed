// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg(feature = "system")]

use bstr::ByteSlice;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};

macro_rules! start_proc {
    ($time:literal, $name:literal) => {
        if cfg!(target_os = "windows") {
            std::process::Command::new("waitfor")
                .arg("/t")
                .arg($time)
                .arg($name)
                .stdout(std::process::Stdio::null())
                .spawn()
                .unwrap()
        } else {
            std::process::Command::new("sleep")
                .arg($time)
                .stdout(std::process::Stdio::null())
                .spawn()
                .unwrap()
        }
    };
}

#[test]
fn test_cwd() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let mut p = start_proc!("3", "CwdSignal");

    let pid = Pid::from_u32(p.id() as _);
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut s = System::new();
    s.refresh_processes_specifics(
        ProcessesToUpdate::All,
        false,
        ProcessRefreshKind::nothing().with_cwd(UpdateKind::Always),
    );
    p.kill().expect("Unable to kill process.");

    let processes = s.processes();
    let p = processes.get(&pid);

    if let Some(p) = p {
        assert_eq!(p.pid(), pid);
        assert_eq!(p.cwd().unwrap(), &std::env::current_dir().unwrap());
    } else {
        panic!("Process not found!");
    }
}

#[test]
fn test_cmd() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let mut p = start_proc!("3", "CmdSignal");
    std::thread::sleep(std::time::Duration::from_millis(500));
    let mut s = System::new();
    assert!(s.processes().is_empty());
    s.refresh_processes_specifics(
        ProcessesToUpdate::All,
        false,
        ProcessRefreshKind::nothing().with_cmd(UpdateKind::Always),
    );
    p.kill().expect("Unable to kill process");
    assert!(!s.processes().is_empty());
    if let Some(process) = s.process(Pid::from_u32(p.id() as _)) {
        if cfg!(target_os = "windows") {
            // Sometimes, we get the full path instead for some reasons... So just in case,
            // we check for the command independently that from the arguments.
            assert!(process.cmd()[0].as_encoded_bytes().contains_str("waitfor"));
            assert_eq!(&process.cmd()[1..], &["/t", "3", "CmdSignal"]);
        } else {
            assert_eq!(process.cmd(), &["sleep", "3"]);
        }
    } else {
        panic!("Process not found!");
    }
}

fn build_test_binary(file_name: &str) {
    std::process::Command::new("rustc")
        .arg("test_bin/main.rs")
        .arg("-o")
        .arg(file_name)
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[test]
#[allow(clippy::zombie_processes)]
fn test_environ() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let file_name = "target/test_binary";
    build_test_binary(file_name);
    let mut p = std::process::Command::new(format!("./{file_name}"))
        .env("FOO", "BAR")
        .env("OTHER", "VALUE")
        .spawn()
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));
    let pid = Pid::from_u32(p.id() as _);
    let mut s = System::new();

    s.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        false,
        ProcessRefreshKind::everything(),
    );
    p.kill().expect("Unable to kill process.");

    let processes = s.processes();
    let proc_ = processes.get(&pid);

    if let Some(proc_) = proc_ {
        assert_eq!(proc_.pid(), pid);
        assert!(proc_.environ().iter().any(|e| e == "FOO=BAR"));
        assert!(proc_.environ().iter().any(|e| e == "OTHER=VALUE"));
    } else {
        panic!("Process not found!");
    }

    // Test to ensure that a process with a lot of environment variables doesn't get truncated.
    // More information in <https://github.com/GuillaumeGomez/sysinfo/issues/886>.
    const SIZE: usize = 30_000;
    let mut big_env = String::with_capacity(SIZE);
    for _ in 0..SIZE {
        big_env.push('a');
    }
    let mut p = std::process::Command::new("./target/test_binary")
        .env("FOO", &big_env)
        .spawn()
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));
    let pid = Pid::from_u32(p.id() as _);
    let mut s = System::new();

    s.refresh_processes_specifics(
        ProcessesToUpdate::All,
        false,
        ProcessRefreshKind::nothing().with_environ(UpdateKind::Always),
    );

    let processes = s.processes();
    let proc_ = processes.get(&pid);

    if let Some(proc_) = proc_ {
        p.kill().expect("Unable to kill process.");
        assert_eq!(proc_.pid(), pid);
        let env = format!("FOO={big_env}");
        assert!(proc_.environ().iter().any(|e| *e == *env));
    } else {
        panic!("Process not found!");
    }
}

#[test]
fn test_process_refresh() {
    let mut s = System::new();
    assert_eq!(s.processes().len(), 0);

    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    s.refresh_processes(
        ProcessesToUpdate::Some(&[sysinfo::get_current_pid().expect("failed to get current pid")]),
        false,
    );
    assert!(
        s.process(sysinfo::get_current_pid().expect("failed to get current pid"))
            .is_some()
    );

    assert!(
        s.processes()
            .iter()
            .all(|(_, p)| p.environ().is_empty() && p.cwd().is_none() && p.cmd().is_empty())
    );
    assert!(
        s.processes()
            .iter()
            .any(|(_, p)| !p.name().is_empty() && p.memory() != 0)
    );
}

#[test]
fn test_process_disk_usage() {
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use sysinfo::get_current_pid;

    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    if std::env::var("FREEBSD_CI").is_ok() {
        // For an unknown reason, when running this test on Cirrus CI, it fails. It works perfectly
        // locally though... Dark magic...
        return;
    }

    fn inner() -> System {
        {
            let mut file = File::create("test.txt").expect("failed to create file");
            file.write_all(b"This is a test file\nwith test data.\n")
                .expect("failed to write to file");
        }
        fs::remove_file("test.txt").expect("failed to remove file");
        // Waiting a bit just in case...
        std::thread::sleep(std::time::Duration::from_millis(250));
        let mut system = System::new();
        assert!(system.processes().is_empty());
        system.refresh_processes(ProcessesToUpdate::All, false);
        assert!(!system.processes().is_empty());
        system
    }

    let mut system = inner();
    let mut p = system
        .process(get_current_pid().expect("Failed retrieving current pid."))
        .expect("failed to get process");

    if cfg!(any(target_os = "macos", target_os = "ios")) && p.disk_usage().total_written_bytes == 0
    {
        // For whatever reason, sometimes, mac doesn't work on the first time when running
        // `cargo test`. Two solutions, either run with "cargo test -- --test-threads 1", or
        // check twice...
        system = inner();
        p = system
            .process(get_current_pid().expect("Failed retrieving current pid."))
            .expect("failed to get process");
    }

    assert!(
        p.disk_usage().total_written_bytes > 0,
        "found {} total written bytes...",
        p.disk_usage().total_written_bytes
    );
    assert!(
        p.disk_usage().written_bytes > 0,
        "found {} written bytes...",
        p.disk_usage().written_bytes
    );
}

#[test]
fn cpu_usage_is_not_nan() {
    let mut system = System::new();
    system.refresh_processes(ProcessesToUpdate::All, false);

    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }

    // We need `collect` otherwise we can't have mutable access to `system`.
    #[allow(clippy::needless_collect)]
    let first_pids = system
        .processes()
        .iter()
        .take(10)
        .map(|(&pid, _)| pid)
        .collect::<Vec<_>>();
    let mut checked = 0;

    first_pids.into_iter().for_each(|pid| {
        system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
        if let Some(p) = system.process(pid) {
            assert!(!p.cpu_usage().is_nan());
            checked += 1;
        }
    });
    assert!(checked > 0);
}

#[test]
fn test_process_times() {
    use std::time::{SystemTime, UNIX_EPOCH};

    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let boot_time = System::boot_time();
    assert!(boot_time > 0);
    let mut p = start_proc!("3", "ProcessTimes");

    let pid = Pid::from_u32(p.id() as _);
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut s = System::new();
    s.refresh_processes(ProcessesToUpdate::All, false);
    p.kill().expect("Unable to kill process.");

    if let Some(p) = s.process(pid) {
        assert_eq!(p.pid(), pid);
        assert!(p.run_time() >= 1);
        assert!(p.run_time() <= 2);
        assert!(p.start_time() > p.run_time());
        // On linux, for whatever reason, the uptime seems to be older than the boot time, leading
        // to this weird `+ 5` to ensure the test is passing as it should...
        assert!(
            p.start_time() + 5
                > SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
        );
        assert!(p.start_time() >= boot_time);
    } else {
        panic!("Process not found!");
    }
}

// Checks that `session_id` is working.
#[test]
fn test_process_session_id() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let mut s = System::new();
    s.refresh_processes(ProcessesToUpdate::All, false);
    assert!(s.processes().values().any(|p| p.session_id().is_some()));
}

// Checks that `refresh_processes` is removing dead processes.
#[test]
fn test_refresh_processes() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let mut p = start_proc!("300", "RefreshProcesses");

    let pid = Pid::from_u32(p.id() as _);
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Checks that the process is listed as it should.
    let mut s = System::new();
    s.refresh_processes(ProcessesToUpdate::All, false);
    assert!(s.process(pid).is_some());
    // We will use this `System` instance for another check.
    let mut old_system = System::new();
    old_system.refresh_processes(ProcessesToUpdate::All, false);
    assert!(old_system.process(pid).is_some());

    // Check that the process name is not empty.
    assert!(!s.process(pid).unwrap().name().is_empty());

    p.kill().expect("Unable to kill process.");
    // We need this, otherwise the process will still be around as a zombie on linux.
    let _ = p.wait();
    // Let's give some time to the system to clean up...
    std::thread::sleep(std::time::Duration::from_secs(1));

    let mut new_system = sysinfo::System::new_with_specifics(RefreshKind::nothing());
    new_system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        true,
        ProcessRefreshKind::nothing(),
    );

    // `new_system` should not have this removed process.
    assert!(new_system.process(pid).is_none());

    s.refresh_processes(ProcessesToUpdate::All, true);
    // Checks that the process isn't listed anymore.
    assert!(s.process(pid).is_none());

    // And we ensure that refreshing it this way will work too (ie, not listed anymore).
    old_system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        true,
        ProcessRefreshKind::nothing(),
    );

    assert!(old_system.process(pid).is_none());
}

// This test ensures that if we refresh only one process, then only this process is removed.
#[test]
fn test_refresh_process_doesnt_remove() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let mut p1 = start_proc!("300", "RefreshProcessRemove1");
    let mut p2 = start_proc!("300", "RefreshProcessRemove2");

    let pid1 = Pid::from_u32(p1.id() as _);
    let pid2 = Pid::from_u32(p2.id() as _);
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Checks that the process is listed as it should.
    let mut s = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing()),
    );
    s.refresh_processes(ProcessesToUpdate::All, false);

    assert!(s.process(pid1).is_some());
    assert!(s.process(pid2).is_some());

    p1.kill().expect("Unable to kill process.");
    p2.kill().expect("Unable to kill process.");
    // We need this, otherwise the process will still be around as a zombie on linux.
    let _ = p1.wait();
    let _ = p2.wait();

    // Let's give some time to the system to clean up...
    std::thread::sleep(std::time::Duration::from_secs(1));

    assert_eq!(
        s.refresh_processes(ProcessesToUpdate::Some(&[pid1]), false),
        0
    );

    // We check that none of the two processes were removed.
    assert!(s.process(pid1).is_some());
    assert!(s.process(pid2).is_some());

    assert_eq!(
        s.refresh_processes(ProcessesToUpdate::Some(&[pid1]), true),
        0
    );

    // We check that only `pid1` was removed.
    assert!(s.process(pid1).is_none());
    assert!(s.process(pid2).is_some());
}

// Checks that `refresh_processes` is adding and removing task.
#[test]
#[cfg(all(
    any(target_os = "linux", target_os = "android"),
    not(feature = "unknown-ci")
))]
fn test_refresh_tasks() {
    // Skip if unsupported.
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }

    // 1) Spawn a thread that waits on a channel, so we control when it exits.
    let task_name = "controlled_test_thread";
    let (tx, rx) = std::sync::mpsc::channel::<()>();

    std::thread::Builder::new()
        .name(task_name.to_string())
        .spawn(move || {
            // Wait until the main thread signals we can exit.
            let _ = rx.recv();
        })
        .unwrap();

    let pid = Pid::from_u32(std::process::id() as _);
    let mut sys = System::new();

    // Wait until the new thread shows up in the process/tasks list.
    // We do a short loop and check each time by refreshing processes.
    const MAX_POLLS: usize = 20;
    const POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(100);

    for _ in 0..MAX_POLLS {
        sys.refresh_processes(ProcessesToUpdate::All, /*refresh_users=*/ false);

        // Check if our thread is present in two ways:
        //   (a) via parent's tasks
        //   (b) by exact name
        let parent_proc = sys.process(pid);
        let tasks_contain_thread = parent_proc
            .and_then(|p| p.tasks())
            .map(|tids| {
                tids.iter().any(|tid| {
                    sys.process(*tid)
                        .map(|t| t.name() == task_name)
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false);

        let by_exact_name_exists = sys
            .processes_by_exact_name(task_name.as_ref())
            .next()
            .is_some();

        if tasks_contain_thread && by_exact_name_exists {
            // We confirmed the thread is now visible
            break;
        }
        std::thread::sleep(POLL_INTERVAL);
    }

    // At this point we know the task is visible in the system's process/tasks list.
    // Let's validate a few more things:
    // * ProcessRefreshKind::nothing() should have task information.
    // * ProcessRefreshKind::nothing().with_tasks() should have task information.
    // * ProcessRefreshKind::nothing().without_tasks() shouldn't have task information.
    // * ProcessRefreshKind::everything() should have task information.
    // * ProcessRefreshKind::everything() should have task information.
    // * ProcessRefreshKind::everything().without_tasks() should not have task information.

    let expectations = [
        (ProcessRefreshKind::nothing(), true),
        (ProcessRefreshKind::nothing().with_tasks(), true),
        (ProcessRefreshKind::nothing().without_tasks(), false),
        (ProcessRefreshKind::everything(), true),
        (ProcessRefreshKind::everything().with_tasks(), true),
        (ProcessRefreshKind::everything().without_tasks(), false),
    ];
    for (kind, expect_tasks) in expectations.iter() {
        let mut sys_new = System::new();
        sys_new.refresh_processes_specifics(ProcessesToUpdate::All, true, *kind);
        let proc = sys_new.process(pid).unwrap();
        assert_eq!(proc.tasks().is_some(), *expect_tasks);
    }

    // 3) Signal the thread to exit.
    drop(tx);

    // 4) Wait until the thread is gone from the system’s process/tasks list.
    for _ in 0..MAX_POLLS {
        sys.refresh_processes(ProcessesToUpdate::All, /*refresh_users=*/ true);

        let parent_proc = sys.process(pid as sysinfo::Pid);
        let tasks_contain_thread = parent_proc
            .and_then(|p| p.tasks())
            .map(|tids| {
                tids.iter().any(|tid| {
                    sys.process(*tid)
                        .map(|t| t.name() == task_name)
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false);

        let by_exact_name_exists = sys
            .processes_by_exact_name(task_name.as_ref())
            .next()
            .is_some();

        // If it's gone from both checks, we're good.
        if !tasks_contain_thread && !by_exact_name_exists {
            break;
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}

// Checks that `refresh_process` is removing dead processes when asked.
#[test]
fn test_refresh_process() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let mut p = start_proc!("300", "RefreshProcess");

    let pid = Pid::from_u32(p.id() as _);
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Checks that the process is listed as it should.
    let mut s = System::new();
    s.refresh_processes(ProcessesToUpdate::Some(&[pid]), false);
    assert!(s.process(pid).is_some());

    // Check that the process name is not empty.
    assert!(!s.process(pid).unwrap().name().is_empty());

    p.kill().expect("Unable to kill process.");
    // We need this, otherwise the process will still be around as a zombie on linux.
    let _ = p.wait();
    // Let's give some time to the system to clean up...
    std::thread::sleep(std::time::Duration::from_secs(1));

    assert_eq!(
        s.refresh_processes(ProcessesToUpdate::Some(&[pid]), false),
        0
    );
    // Checks that the process is still listed.
    assert!(s.process(pid).is_some());

    assert_eq!(
        s.refresh_processes(ProcessesToUpdate::Some(&[pid]), true),
        0
    );
    // Checks that the process is not listed anymore.
    assert!(s.process(pid).is_none());
}

#[test]
fn test_wait_child() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let p = start_proc!("300", "WaitChild");

    let before = std::time::Instant::now();
    let pid = Pid::from_u32(p.id() as _);

    let mut s = System::new();
    s.refresh_processes(ProcessesToUpdate::Some(&[pid]), false);
    let process = s.process(pid).unwrap();

    // Kill the child process.
    process.kill();
    // Wait for child process should work.
    process.wait();

    // Child process should not be present.
    assert_eq!(
        s.refresh_processes(ProcessesToUpdate::Some(&[pid]), true),
        0
    );
    assert!(before.elapsed() < std::time::Duration::from_millis(1000));
}

#[test]
fn test_wait_non_child() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }

    let before = std::time::Instant::now();

    // spawn non child process.
    let p = if !cfg!(target_os = "linux") {
        return;
    } else {
        std::process::Command::new("setsid")
            .arg("-w")
            .arg("sleep")
            .arg("2")
            .stdout(std::process::Stdio::null())
            .spawn()
            .unwrap()
    };
    let pid = Pid::from_u32(p.id());

    let mut s = System::new();
    s.refresh_processes(ProcessesToUpdate::Some(&[pid]), false);
    let process = s.process(pid).expect("Process not found!");

    // Wait for a non child process.
    process.wait();

    // Child process should not be present.
    assert_eq!(
        s.refresh_processes(ProcessesToUpdate::Some(&[pid]), true),
        0
    );

    // should wait for 2s.
    assert!(
        before.elapsed() > std::time::Duration::from_millis(1900),
        "Elapsed time {:?} is not greater than 1900ms",
        before.elapsed()
    );
    assert!(
        before.elapsed() < std::time::Duration::from_millis(3000),
        "Elapsed time {:?} is not less than 3000ms",
        before.elapsed()
    );
}

#[test]
fn test_process_iterator_lifetimes() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }

    let s = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing()),
    );

    let process: Option<&sysinfo::Process>;
    {
        let name = String::from("");
        // errors before PR #904: name does not live long enough
        process = s.processes_by_name(name.as_ref()).next();
    }
    process.unwrap();

    let process: Option<&sysinfo::Process>;
    {
        // worked fine before and after: &'static str lives longer than System, error couldn't appear
        process = s.processes_by_name("".as_ref()).next();
    }
    process.unwrap();
}

// Regression test for <https://github.com/GuillaumeGomez/sysinfo/issues/918>.
#[test]
fn test_process_cpu_usage() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }

    let mut sys = System::new_all();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_all();

    let max_usage = sys.cpus().len() as f32 * 100.;

    for process in sys.processes().values() {
        assert!(process.cpu_usage() <= max_usage);
    }
}

#[test]
fn test_process_creds() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    // Just ensure there is at least one process on the system whose credentials can be retrieved.
    assert!(sys.processes().values().any(|process| {
        if process.user_id().is_none() {
            return false;
        }

        #[cfg(not(windows))]
        {
            if process.group_id().is_none()
                || process.effective_user_id().is_none()
                || process.effective_group_id().is_none()
            {
                return false;
            }
        }

        true
    }));

    // On Windows, make sure no process has real group ID and no effective IDs.
    #[cfg(windows)]
    assert!(sys.processes().values().all(|process| {
        if process.group_id().is_some()
            || process.effective_user_id().is_some()
            || process.effective_group_id().is_some()
        {
            return false;
        }

        true
    }));
}

// This test ensures that only the requested information is retrieved.
#[test]
fn test_process_specific_refresh() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }

    fn check_empty(s: &System, pid: Pid) {
        let p = s.process(pid).unwrap();

        // Name should never be empty.
        assert!(!p.name().is_empty());
        if cfg!(target_os = "windows") {
            assert_eq!(p.user_id(), None);
        }
        assert_eq!(p.environ().len(), 0);
        assert_eq!(p.cmd().len(), 0);
        assert_eq!(p.exe(), None);
        assert_eq!(p.cwd(), None);
        assert_eq!(p.root(), None);
        assert_eq!(p.memory(), 0);
        assert_eq!(p.virtual_memory(), 0);
        // These two won't be checked, too much laziness in testing them...
        assert_eq!(p.disk_usage(), sysinfo::DiskUsage::default());
        assert_eq!(p.cpu_usage(), 0.);
    }

    let mut s = System::new();
    let pid = Pid::from_u32(std::process::id());

    macro_rules! update_specific_and_check {
        (memory) => {
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), false, ProcessRefreshKind::nothing());
            {
                let p = s.process(pid).unwrap();
                assert_eq!(p.memory(), 0, "failed 0 check for memory");
                assert_eq!(p.virtual_memory(), 0, "failed 0 check for virtual memory");
            }
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), false, ProcessRefreshKind::nothing().with_memory());
            {
                let p = s.process(pid).unwrap();
                assert_ne!(p.memory(), 0, "failed non-0 check for memory");
                assert_ne!(p.virtual_memory(), 0, "failed non-0 check for virtual memory");
            }
            // And now we check that re-refreshing nothing won't remove the
            // information.
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), false, ProcessRefreshKind::nothing());
            {
                let p = s.process(pid).unwrap();
                assert_ne!(p.memory(), 0, "failed non-0 check (number 2) for memory");
                assert_ne!(p.virtual_memory(), 0, "failed non-0 check(number 2) for virtual memory");
            }
        };
        ($name:ident, $method:ident, $($extra:tt)+) => {
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), false, ProcessRefreshKind::nothing());
            {
                let p = s.process(pid).unwrap();
                assert_eq!(
                    p.$name()$($extra)+,
                    concat!("failed 0 check check for ", stringify!($name)),
                );
            }
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), false, ProcessRefreshKind::nothing().$method(UpdateKind::Always));
            {
                let p = s.process(pid).unwrap();
                assert_ne!(
                    p.$name()$($extra)+,
                    concat!("failed non-0 check check for ", stringify!($name)),);
            }
            // And now we check that re-refreshing nothing won't remove the
            // information.
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), false, ProcessRefreshKind::nothing());
            {
                let p = s.process(pid).unwrap();
                assert_ne!(
                    p.$name()$($extra)+,
                    concat!("failed non-0 check (number 2) check for ", stringify!($name)),);
            }
        }
    }

    s.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        false,
        ProcessRefreshKind::nothing(),
    );
    check_empty(&s, pid);

    s.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        false,
        ProcessRefreshKind::nothing(),
    );
    check_empty(&s, pid);

    update_specific_and_check!(memory);
    update_specific_and_check!(environ, with_environ, .len(), 0);
    update_specific_and_check!(cmd, with_cmd, .len(), 0);
    if !cfg!(any(
        target_os = "macos",
        target_os = "ios",
        feature = "apple-sandbox",
    )) {
        update_specific_and_check!(root, with_root, , None);
    }
    update_specific_and_check!(exe, with_exe, , None);
    update_specific_and_check!(cwd, with_cwd, , None);
}

#[test]
fn test_refresh_pids() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let self_pid = sysinfo::get_current_pid().expect("failed to get current pid");
    let mut s = System::new();

    let mut p = start_proc!("3", "RefreshPids");

    let child_pid = Pid::from_u32(p.id() as _);
    let pids = &[child_pid, self_pid];
    std::thread::sleep(std::time::Duration::from_millis(500));
    s.refresh_processes(ProcessesToUpdate::Some(pids), false);
    p.kill().expect("Unable to kill process.");

    assert_eq!(s.processes().len(), 2);
    for pid in s.processes().keys() {
        assert!(pids.contains(pid));
    }
}

#[test]
fn test_process_run_time() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let mut s = System::new();
    let current_pid = sysinfo::get_current_pid().expect("failed to get current pid");
    s.refresh_processes(ProcessesToUpdate::Some(&[current_pid]), false);
    let run_time = s.process(current_pid).expect("no process found").run_time();
    std::thread::sleep(std::time::Duration::from_secs(2));
    s.refresh_processes(ProcessesToUpdate::Some(&[current_pid]), true);
    let new_run_time = s.process(current_pid).expect("no process found").run_time();
    assert!(
        new_run_time > run_time,
        "{new_run_time} not superior to {run_time}",
    );
}

// Test that if the parent of a process is removed, then the child PID will be
// updated as well.
#[test]
fn test_parent_change() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") || cfg!(windows) {
        // Windows never updates its parent PID so no need to check anything.
        return;
    }

    let file_name = "target/test_binary2";
    build_test_binary(file_name);
    let mut p = std::process::Command::new(format!("./{file_name}"))
        .arg("1")
        .spawn()
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    let pid = Pid::from_u32(p.id() as _);
    let mut s = System::new();
    s.refresh_processes(ProcessesToUpdate::All, false);

    assert_eq!(
        s.process(pid).expect("process was not created").parent(),
        sysinfo::get_current_pid().ok(),
    );

    let child_pid = s
        .processes()
        .iter()
        .find(|(_, proc_)| proc_.parent() == Some(pid))
        .map(|(pid, _)| *pid)
        .expect("failed to get child process");

    // Waiting for the parent process to stop.
    p.wait().expect("wait failed");

    s.refresh_processes(ProcessesToUpdate::All, true);
    // Parent should not be around anymore.
    assert!(s.process(pid).is_none());

    let child = s.process(child_pid).expect("child is dead");
    // Child should have a different parent now.
    assert_ne!(child.parent(), Some(pid));

    // We kill the child to clean up.
    child.kill();
}

// We want to ensure that if `System::refresh_process*` methods are called
// one after the other, it won't badly impact the CPU usage computation.
#[test]
fn test_multiple_single_process_refresh() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") || cfg!(windows) {
        // Windows never updates its parent PID so no need to check anything.
        return;
    }

    let file_name = "target/test_binary3";
    build_test_binary(file_name);
    let mut p_a = std::process::Command::new(format!("./{file_name}"))
        .arg("1")
        .spawn()
        .unwrap();
    let mut p_b = std::process::Command::new(format!("./{file_name}"))
        .arg("1")
        .spawn()
        .unwrap();

    let pid_a = Pid::from_u32(p_a.id() as _);
    let pid_b = Pid::from_u32(p_b.id() as _);

    let mut s = System::new();
    let process_refresh_kind = ProcessRefreshKind::nothing().with_cpu();
    s.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid_a]),
        false,
        process_refresh_kind,
    );
    s.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid_b]),
        false,
        process_refresh_kind,
    );

    std::thread::sleep(std::time::Duration::from_secs(1));
    s.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid_a]),
        true,
        process_refresh_kind,
    );
    s.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid_b]),
        true,
        process_refresh_kind,
    );

    let cpu_a = s.process(pid_a).unwrap().cpu_usage();
    let cpu_b = s.process(pid_b).unwrap().cpu_usage();

    p_a.kill().expect("failed to kill process a");
    p_b.kill().expect("failed to kill process b");

    let _ = p_a.wait();
    let _ = p_b.wait();

    assert!(cpu_b - 5. < cpu_a && cpu_b + 5. > cpu_a);
}

#[test]
fn accumulated_cpu_time() {
    fn generate_cpu_usage() {
        use std::sync::atomic::{AtomicBool, Ordering};

        let atomic = std::sync::Arc::new(AtomicBool::new(false));
        let thread_atomic = atomic.clone();
        std::thread::spawn(move || {
            while !thread_atomic.load(Ordering::Relaxed) {
                System::new_all();
            }
        });
        std::thread::sleep(std::time::Duration::from_millis(250));
        atomic.store(true, Ordering::Relaxed);
    }

    if !sysinfo::IS_SUPPORTED_SYSTEM
        || cfg!(feature = "apple-sandbox")
        || cfg!(target_os = "freebsd")
    {
        return;
    }

    let mut s = System::new();
    let current_pid = sysinfo::get_current_pid().expect("failed to get current pid");
    let refresh_kind = ProcessRefreshKind::nothing().with_cpu();
    generate_cpu_usage();
    s.refresh_processes_specifics(ProcessesToUpdate::Some(&[current_pid]), false, refresh_kind);
    let acc_time = s
        .process(current_pid)
        .expect("no process found")
        .accumulated_cpu_time();
    assert_ne!(acc_time, 0);

    generate_cpu_usage();
    s.refresh_processes_specifics(ProcessesToUpdate::Some(&[current_pid]), true, refresh_kind);
    let new_acc_time = s
        .process(current_pid)
        .expect("no process found")
        .accumulated_cpu_time();
    assert!(
        new_acc_time > acc_time,
        "{new_acc_time} not superior to {acc_time}",
    );
}

#[test]
fn test_exists() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }

    let file_name = "target/test_binary4";
    build_test_binary(file_name);
    let mut p = std::process::Command::new(format!("./{file_name}"))
        .arg("1")
        .spawn()
        .unwrap();
    let pid = Pid::from_u32(p.id() as _);

    let mut s = System::new();
    let process_refresh_kind = ProcessRefreshKind::nothing().with_memory();
    s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), false, process_refresh_kind);
    assert!(s.process(pid).unwrap().exists());

    p.kill().expect("Unable to kill process.");
    // We need this, otherwise the process will still be around as a zombie on linux.
    let _ = p.wait();
    // Let's give some time to the system to clean up...
    std::thread::sleep(std::time::Duration::from_secs(1));

    s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), false, process_refresh_kind);
    assert!(!s.process(pid).unwrap().exists());
}

#[cfg(target_os = "linux")]
#[test]
fn test_tasks() {
    use std::collections::HashSet;

    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }

    fn get_tasks(system: &System, pid: Pid) -> HashSet<Pid> {
        let mut task_pids: HashSet<Pid> = HashSet::new();
        if let Some(process) = system.process(pid)
            && let Some(tasks) = process.tasks()
        {
            task_pids.extend(tasks);
        }
        task_pids
    }

    let mut system = System::new_with_specifics(RefreshKind::nothing());
    system.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing());
    let pid = sysinfo::get_current_pid().expect("failed to get current pid");
    let old_tasks = get_tasks(&system, pid);

    // Spawn a thread to increase the task count
    let scheduler = std::thread::spawn(move || {
        system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing(),
        );

        let mut system_new = System::new_with_specifics(RefreshKind::nothing());
        system_new.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing(),
        );

        let new_tasks = get_tasks(&system, pid);
        assert_ne!(old_tasks, new_tasks);
        assert_eq!(new_tasks, get_tasks(&system_new, pid));
    });
    scheduler.join().expect("Scheduler panicked");
}

#[test]
fn open_files() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let pid = sysinfo::get_current_pid().expect("failed to get current pid");
    let _file =
        std::fs::File::create(std::env::temp_dir().join("sysinfo-open-files.test")).unwrap();
    let mut s = System::new();
    s.refresh_processes(ProcessesToUpdate::Some(&[pid]), false);
    let cur_process = s.process(pid).unwrap();
    assert!(
        cur_process
            .open_files()
            .is_some_and(|open_files| open_files > 0)
    );
    assert!(
        cur_process
            .open_files_limit()
            .is_some_and(|open_files| open_files > 0)
    );
}

#[test]
fn test_wait() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let p = start_proc!("2", "TestWait");
    let pid = Pid::from_u32(p.id() as _);
    let mut s = System::new();
    s.refresh_processes_specifics(ProcessesToUpdate::All, false, ProcessRefreshKind::nothing());
    // We check the result of the exiting process.
    let exit_status = s.process(pid).unwrap().wait();
    assert!(exit_status.is_some());

    // Now we check that it doesn't exist anymore.
    let mut s2 = System::new();
    s2.refresh_processes_specifics(ProcessesToUpdate::All, false, ProcessRefreshKind::nothing());
    assert!(s2.process(pid).is_none());
    // And we check that waiting for it will return `None`.
    if cfg!(target_os = "linux") {
        assert_eq!(s.process(pid).unwrap().wait(), None);
    } else {
        // On windows we can get the exit status as long as we have a handle.
        // On mac and freebsd, no clue why.
        let exit_status = s.process(pid).unwrap().wait();
        assert!(exit_status.is_some());
    }
}

// Regression test for <https://github.com/GuillaumeGomez/sysinfo/issues/1528>.
//
// On macOS, we didn't set the `old_stime` and `old_utime` when we create processes, meaning
// that we can get CPU usage only on the third try.
#[test]
fn test_cpu_processes_usage() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    if std::env::var("FREEBSD_CI").is_ok() {
        // FIXME: once I'm able to run a virtual freebsd machine, need to check if this test
        // is working.
        return;
    }

    let mut sys = System::new_all();

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    assert!(sys.processes().iter().any(|(_, p)| p.cpu_usage() > 0.));
}
