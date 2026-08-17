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
        ProcessRefreshKind::new().with_cwd(UpdateKind::Always),
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
        ProcessRefreshKind::new().with_cmd(UpdateKind::Always),
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
        sysinfo::ProcessRefreshKind::everything(),
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
        ProcessRefreshKind::new().with_environ(UpdateKind::Always),
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
    s.refresh_processes(ProcessesToUpdate::Some(&[
        sysinfo::get_current_pid().expect("failed to get current pid")
    ]));
    assert!(s
        .process(sysinfo::get_current_pid().expect("failed to get current pid"))
        .is_some());

    assert!(s
        .processes()
        .iter()
        .all(|(_, p)| p.environ().is_empty() && p.cwd().is_none() && p.cmd().is_empty()));
    assert!(s
        .processes()
        .iter()
        .any(|(_, p)| !p.name().is_empty() && p.memory() != 0));
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
        system.refresh_processes(ProcessesToUpdate::All);
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
    system.refresh_processes(ProcessesToUpdate::All);

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
        system.refresh_processes(ProcessesToUpdate::Some(&[pid]));
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
    s.refresh_processes(ProcessesToUpdate::All);
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
    s.refresh_processes(ProcessesToUpdate::All);
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
    s.refresh_processes(ProcessesToUpdate::All);
    assert!(s.process(pid).is_some());

    // Check that the process name is not empty.
    assert!(!s.process(pid).unwrap().name().is_empty());

    p.kill().expect("Unable to kill process.");
    // We need this, otherwise the process will still be around as a zombie on linux.
    let _ = p.wait();
    // Let's give some time to the system to clean up...
    std::thread::sleep(std::time::Duration::from_secs(1));

    s.refresh_processes(ProcessesToUpdate::All);
    // Checks that the process isn't listed anymore.
    assert!(s.process(pid).is_none());
}

// This test ensures that if we refresh only one process, then no process is removed.
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
        RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()),
    );
    s.refresh_processes(ProcessesToUpdate::All);

    assert!(s.process(pid1).is_some());
    assert!(s.process(pid2).is_some());

    p1.kill().expect("Unable to kill process.");
    p2.kill().expect("Unable to kill process.");
    // We need this, otherwise the process will still be around as a zombie on linux.
    let _ = p1.wait();
    let _ = p2.wait();

    // Let's give some time to the system to clean up...
    std::thread::sleep(std::time::Duration::from_secs(1));

    assert_eq!(s.refresh_processes(ProcessesToUpdate::Some(&[pid1])), 0);

    // We check that none of the two processes were removed.
    assert!(s.process(pid1).is_some());
    assert!(s.process(pid2).is_some());
}

// Checks that `refresh_processes` is adding and removing task.
#[test]
#[cfg(all(
    any(target_os = "linux", target_os = "android"),
    not(feature = "unknown-ci")
))]
fn test_refresh_tasks() {
    if !sysinfo::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
        return;
    }
    let task_name = "task_1_second";
    std::thread::Builder::new()
        .name(task_name.into())
        .spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(1));
        })
        .unwrap();

    let pid = Pid::from_u32(std::process::id() as _);

    // Checks that the task is listed as it should.
    let mut s = System::new();
    s.refresh_processes(ProcessesToUpdate::All);

    assert!(s
        .process(pid)
        .unwrap()
        .tasks()
        .map(|tasks| tasks.iter().any(|task_pid| s
            .process(*task_pid)
            .map(|task| task.name() == task_name)
            .unwrap_or(false)))
        .unwrap_or(false));
    assert!(s
        .processes_by_exact_name(task_name.as_ref())
        .next()
        .is_some());

    // Let's give some time to the system to clean up...
    std::thread::sleep(std::time::Duration::from_secs(2));

    s.refresh_processes(ProcessesToUpdate::All);

    assert!(!s
        .process(pid)
        .unwrap()
        .tasks()
        .map(|tasks| tasks.iter().any(|task_pid| s
            .process(*task_pid)
            .map(|task| task.name() == task_name)
            .unwrap_or(false)))
        .unwrap_or(false));
    assert!(s
        .processes_by_exact_name(task_name.as_ref())
        .next()
        .is_none());
}

// Checks that `refresh_process` is NOT removing dead processes.
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
    s.refresh_processes(ProcessesToUpdate::Some(&[pid]));
    assert!(s.process(pid).is_some());

    // Check that the process name is not empty.
    assert!(!s.process(pid).unwrap().name().is_empty());

    p.kill().expect("Unable to kill process.");
    // We need this, otherwise the process will still be around as a zombie on linux.
    let _ = p.wait();
    // Let's give some time to the system to clean up...
    std::thread::sleep(std::time::Duration::from_secs(1));

    assert_eq!(s.refresh_processes(ProcessesToUpdate::Some(&[pid])), 0);
    // Checks that the process is still listed.
    assert!(s.process(pid).is_some());
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
    s.refresh_processes(ProcessesToUpdate::Some(&[pid]));
    let process = s.process(pid).unwrap();

    // Kill the child process.
    process.kill();
    // Wait for child process should work.
    process.wait();

    // Child process should not be present.
    assert_eq!(s.refresh_processes(ProcessesToUpdate::Some(&[pid])), 0);
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
    s.refresh_processes(ProcessesToUpdate::Some(&[pid]));
    let process = s.process(pid).expect("Process not found!");

    // Wait for a non child process.
    process.wait();

    // Child process should not be present.
    assert_eq!(s.refresh_processes(ProcessesToUpdate::Some(&[pid])), 0);

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
        sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()),
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
    use sysinfo::{DiskUsage, ProcessRefreshKind};

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
        // These two won't be checked, too much lazyness in testing them...
        assert_eq!(p.disk_usage(), DiskUsage::default());
        assert_eq!(p.cpu_usage(), 0.);
    }

    let mut s = System::new();
    let pid = Pid::from_u32(std::process::id());

    macro_rules! update_specific_and_check {
        (memory) => {
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), ProcessRefreshKind::new());
            {
                let p = s.process(pid).unwrap();
                assert_eq!(p.memory(), 0, "failed 0 check for memory");
                assert_eq!(p.virtual_memory(), 0, "failed 0 check for virtual memory");
            }
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), ProcessRefreshKind::new().with_memory());
            {
                let p = s.process(pid).unwrap();
                assert_ne!(p.memory(), 0, "failed non-0 check for memory");
                assert_ne!(p.virtual_memory(), 0, "failed non-0 check for virtual memory");
            }
            // And now we check that re-refreshing nothing won't remove the
            // information.
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), ProcessRefreshKind::new());
            {
                let p = s.process(pid).unwrap();
                assert_ne!(p.memory(), 0, "failed non-0 check (number 2) for memory");
                assert_ne!(p.virtual_memory(), 0, "failed non-0 check(number 2) for virtual memory");
            }
        };
        ($name:ident, $method:ident, $($extra:tt)+) => {
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), ProcessRefreshKind::new());
            {
                let p = s.process(pid).unwrap();
                assert_eq!(
                    p.$name()$($extra)+,
                    concat!("failed 0 check check for ", stringify!($name)),
                );
            }
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), ProcessRefreshKind::new().$method(UpdateKind::Always));
            {
                let p = s.process(pid).unwrap();
                assert_ne!(
                    p.$name()$($extra)+,
                    concat!("failed non-0 check check for ", stringify!($name)),);
            }
            // And now we check that re-refreshing nothing won't remove the
            // information.
            s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), ProcessRefreshKind::new());
            {
                let p = s.process(pid).unwrap();
                assert_ne!(
                    p.$name()$($extra)+,
                    concat!("failed non-0 check (number 2) check for ", stringify!($name)),);
            }
        }
    }

    s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), ProcessRefreshKind::new());
    check_empty(&s, pid);

    s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), ProcessRefreshKind::new());
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
    s.refresh_processes(ProcessesToUpdate::Some(pids));
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
    s.refresh_processes(ProcessesToUpdate::Some(&[current_pid]));
    let run_time = s.process(current_pid).expect("no process found").run_time();
    std::thread::sleep(std::time::Duration::from_secs(2));
    s.refresh_processes(ProcessesToUpdate::Some(&[current_pid]));
    let new_run_time = s.process(current_pid).expect("no process found").run_time();
    assert!(
        new_run_time > run_time,
        "{} not superior to {}",
        new_run_time,
        run_time
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
    s.refresh_processes(ProcessesToUpdate::All);

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

    s.refresh_processes(ProcessesToUpdate::All);
    // Parent should not be around anymore.
    assert!(s.process(pid).is_none());

    let child = s.process(child_pid).expect("child is dead");
    // Child should have a different parent now.
    assert_ne!(child.parent(), Some(pid));

    // We kill the child to clean up.
    child.kill();
}

// We want to ensure that if `System::refresh_process*` methods are called
// one after the other, it won't impact the CPU usage computation badly.
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
    let process_refresh_kind = ProcessRefreshKind::new().with_cpu();
    s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid_a]), process_refresh_kind);
    s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid_b]), process_refresh_kind);

    std::thread::sleep(std::time::Duration::from_secs(1));
    s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid_a]), process_refresh_kind);
    s.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid_b]), process_refresh_kind);

    let cpu_a = s.process(pid_a).unwrap().cpu_usage();
    let cpu_b = s.process(pid_b).unwrap().cpu_usage();

    p_a.kill().expect("failed to kill process a");
    p_b.kill().expect("failed to kill process b");

    let _ = p_a.wait();
    let _ = p_b.wait();

    assert!(cpu_b - 5. < cpu_a && cpu_b + 5. > cpu_a);
}
