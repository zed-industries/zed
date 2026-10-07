use anyhow::{Context as _, Result};
use std::process::Stdio;

/// A wrapper around `smol::process::Child` that ensures all subprocesses
/// are killed when the process is terminated: on Unix by using process
/// groups, and on Windows by using job objects.
///
/// On Linux and macOS, `kill` also terminates descendants that escaped the
/// original process group via `setsid`.
///
/// On Windows, dropping this struct closes the job object handle, which
/// terminates all processes in the job. This also applies when the Zed
/// process exits for any reason (including crashes), since the OS closes
/// its handles, so spawned process trees can never outlive Zed.
pub struct Child {
    process: smol::process::Child,
    #[cfg(windows)]
    job: Option<windows_job::JobObject>,
}

impl std::ops::Deref for Child {
    type Target = smol::process::Child;

    fn deref(&self) -> &Self::Target {
        &self.process
    }
}

impl std::ops::DerefMut for Child {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.process
    }
}

impl Child {
    #[cfg(not(windows))]
    pub fn spawn(
        mut command: std::process::Command,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> Result<Self> {
        crate::set_pre_exec_to_start_new_session(&mut command);
        let mut command = smol::process::Command::from(command);
        let process = command
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .with_context(|| {
                format!(
                    "failed to spawn command {}",
                    crate::redact::redact_command(&format!("{command:?}"))
                )
            })?;
        Ok(Self { process })
    }

    #[cfg(windows)]
    pub fn spawn(
        command: std::process::Command,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> Result<Self> {
        let mut command = smol::process::Command::from(command);
        let process = command
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .with_context(|| {
                format!(
                    "failed to spawn command {}",
                    crate::redact::redact_command(&format!("{command:?}"))
                )
            })?;

        // Assign the child to a job object configured to kill the entire
        // process tree when the last job handle is closed, so descendants
        // (e.g. node workers and MCP servers spawned by agent servers) are
        // reaped even if the direct child doesn't clean them up. Any process
        // the child spawns after this assignment is automatically part of the
        // job.
        //
        // There is a small race: descendants the child spawns between the
        // `spawn()` call returning and the assignment below escape the job.
        // Closing it fully would require creating the process suspended
        // (`CREATE_SUSPENDED`), assigning it, then resuming it, which the
        // std/smol process APIs don't support without reimplementing process
        // creation. The window is microseconds, and the children we care
        // about (`npx`, `node`, etc.) take far longer to load their runtime
        // and spawn anything, so in practice nothing escapes.
        let job = windows_job::JobObject::new()
            .and_then(|job| {
                job.assign_process(process.id())?;
                Ok(job)
            })
            .map_err(|error| {
                log::error!("failed to assign spawned process to a job object: {error:#}");
            })
            .ok();

        Ok(Self { process, job })
    }

    /// Consumes the child, draining its stdout/stderr and waiting for it to
    /// exit, then returns the collected output.
    pub async fn output(self) -> Result<std::process::Output> {
        // NOTE: Keep `self` alive across this await, do not destructure it to
        // pull `process` out first. On Windows that drops the job object early,
        // which triggers `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` and kills the
        // child before `output()` finishes collecting its stdout/stderr.
        Ok(self.process.output().await?)
    }

    #[cfg(not(windows))]
    pub fn kill(&mut self) -> Result<()> {
        let pid = self.process.id();
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        kill_descendant_tree(pid);
        // SAFETY: ESRCH (empty group) is the only failure mode and we ignore it.
        unsafe {
            libc::killpg(pid as i32, libc::SIGKILL);
        }
        Ok(())
    }

    #[cfg(windows)]
    pub fn kill(&mut self) -> Result<()> {
        if let Some(job) = &self.job {
            job.terminate()
        } else {
            self.process.kill()?;
            Ok(())
        }
    }
}

#[cfg(windows)]
mod windows_job {
    use crate::ResultExt as _;
    use anyhow::{Context as _, Result};
    use windows::Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::{
            JobObjects::{
                AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
                JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
                SetInformationJobObject, TerminateJobObject,
            },
            Threading::{OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE},
        },
    };

    /// A Win32 job object configured with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`:
    /// all processes assigned to the job (and their descendants) are terminated
    /// when the last handle to the job is closed, which happens when this struct
    /// is dropped, or when the OS closes the owning process's handles after it
    /// exits for any reason.
    pub(crate) struct JobObject(HANDLE);

    // SAFETY: Job object handles can be used from any thread.
    unsafe impl Send for JobObject {}
    unsafe impl Sync for JobObject {}

    impl JobObject {
        pub(crate) fn new() -> Result<Self> {
            unsafe {
                let job =
                    Self(CreateJobObjectW(None, None).context("failed to create job object")?);
                let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
                info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
                SetInformationJobObject(
                    job.0,
                    JobObjectExtendedLimitInformation,
                    &info as *const _ as *const _,
                    size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
                )
                .context("failed to set job object limits")?;
                Ok(job)
            }
        }

        pub(crate) fn assign_process(&self, pid: u32) -> Result<()> {
            unsafe {
                let process = OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, false, pid)
                    .context("failed to open process")?;
                let result = AssignProcessToJobObject(self.0, process)
                    .context("failed to assign process to job object");
                CloseHandle(process).log_err();
                result
            }
        }

        pub(crate) fn terminate(&self) -> Result<()> {
            unsafe { TerminateJobObject(self.0, 1).context("failed to terminate job object") }
        }
    }

    impl Drop for JobObject {
        fn drop(&mut self) {
            unsafe {
                CloseHandle(self.0).log_err();
            }
        }
    }
}

#[cfg(all(test, windows))]
mod windows_tests {
    use super::*;
    use std::time::{Duration, Instant};

    /// Spawns a process tree `powershell -> ping` via `Child::spawn` and
    /// returns the `Child` along with the pid of the grandchild (`ping`).
    fn spawn_process_tree(temp_dir: &std::path::Path) -> (Child, u32) {
        let pid_file = temp_dir.join("grandchild_pid");
        let mut command = std::process::Command::new("powershell.exe");
        command.args(["-NoProfile", "-Command"]).arg(format!(
            "$p = Start-Process -FilePath ping.exe -ArgumentList @('-n','60','127.0.0.1') -PassThru -WindowStyle Hidden; \
             Set-Content -LiteralPath '{}' -Value $p.Id; \
             Wait-Process -Id $p.Id",
            pid_file.display()
        ));
        let child = Child::spawn(command, Stdio::null(), Stdio::null(), Stdio::null())
            .expect("failed to spawn powershell");

        let deadline = Instant::now() + Duration::from_secs(5);
        let grandchild_pid = loop {
            if let Ok(contents) = std::fs::read_to_string(&pid_file)
                && let Ok(pid) = contents.trim().parse::<u32>()
            {
                break pid;
            }
            assert!(
                Instant::now() < deadline,
                "timed out waiting for grandchild pid file"
            );
            std::thread::sleep(Duration::from_millis(50));
        };
        assert!(
            process_is_alive(grandchild_pid),
            "grandchild should be alive after spawning"
        );
        (child, grandchild_pid)
    }

    fn process_is_alive(pid: u32) -> bool {
        use windows::Win32::{
            Foundation::{CloseHandle, STILL_ACTIVE},
            System::Threading::{
                GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
            },
        };

        unsafe {
            let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else {
                return false;
            };
            let mut exit_code = 0u32;
            let alive = GetExitCodeProcess(handle, &mut exit_code).is_ok()
                && exit_code == STILL_ACTIVE.0 as u32;
            CloseHandle(handle).expect("failed to close process handle");
            alive
        }
    }

    fn assert_process_exits(pid: u32, message: &str) {
        let deadline = Instant::now() + Duration::from_secs(2);
        while process_is_alive(pid) {
            assert!(Instant::now() < deadline, "{message} (pid {pid})");
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    #[test]
    fn test_kill_terminates_grandchildren() {
        let temp_dir = tempfile::tempdir().unwrap();
        let (mut child, grandchild_pid) = spawn_process_tree(temp_dir.path());

        child.kill().expect("failed to kill child");

        assert_process_exits(
            grandchild_pid,
            "grandchild should be terminated after killing the child",
        );
    }

    #[test]
    fn test_drop_terminates_grandchildren() {
        let temp_dir = tempfile::tempdir().unwrap();
        let (child, grandchild_pid) = spawn_process_tree(temp_dir.path());

        drop(child);

        assert_process_exits(
            grandchild_pid,
            "grandchild should be terminated after dropping the child",
        );
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn kill_descendant_tree(root_pid: u32) {
    use std::collections::HashMap;

    let parent_of = list_parent_pids();

    let mut children_of: HashMap<u32, Vec<u32>> = HashMap::new();
    for (&pid, &ppid) in &parent_of {
        children_of.entry(ppid).or_default().push(pid);
    }

    let mut queue = vec![root_pid];
    let mut descendants = Vec::new();
    while let Some(pid) = queue.pop() {
        if let Some(children) = children_of.get(&pid) {
            for &child in children {
                descendants.push(child);
                queue.push(child);
            }
        }
    }

    for pid in descendants {
        // SAFETY: ESRCH (PID gone) is acceptable; we ignore the result.
        unsafe {
            libc::kill(pid as i32, libc::SIGKILL);
        }
    }
}

#[cfg(target_os = "linux")]
fn list_parent_pids() -> std::collections::HashMap<u32, u32> {
    use std::collections::HashMap;
    let mut parent_of: HashMap<u32, u32> = HashMap::new();
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return parent_of;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(pid_str) = name.to_str() else {
            continue;
        };
        let Ok(pid) = pid_str.parse::<u32>() else {
            continue;
        };
        let Ok(status) = std::fs::read_to_string(format!("/proc/{pid}/status")) else {
            continue;
        };
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("PPid:")
                && let Ok(ppid) = rest.trim().parse::<u32>()
            {
                parent_of.insert(pid, ppid);
                break;
            }
        }
    }
    parent_of
}

// libc on macOS doesn't expose `kinfo_proc`, and a hand-rolled struct layout is
// fragile. `ps` is universally present on macOS, runs once per kill, and the
// extra fork is irrelevant at app-quit. If ps becomes a problem, replace this
// with a `sysctl(KERN_PROC_ALL)` binding.
#[cfg(target_os = "macos")]
fn list_parent_pids() -> std::collections::HashMap<u32, u32> {
    use std::collections::HashMap;
    let mut parent_of: HashMap<u32, u32> = HashMap::new();
    // Synchronous fork is intentional: this only runs during app-quit cleanup,
    // not on an async runtime thread.
    #[allow(clippy::disallowed_methods)]
    let Ok(output) = std::process::Command::new("/bin/ps")
        .args(["-A", "-o", "pid=,ppid="])
        .output()
    else {
        return parent_of;
    };
    if !output.status.success() {
        return parent_of;
    }
    let Ok(text) = std::str::from_utf8(&output.stdout) else {
        return parent_of;
    };
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let Some(pid_str) = parts.next() else {
            continue;
        };
        let Some(ppid_str) = parts.next() else {
            continue;
        };
        let Ok(pid) = pid_str.parse::<u32>() else {
            continue;
        };
        let Ok(ppid) = ppid_str.parse::<u32>() else {
            continue;
        };
        parent_of.insert(pid, ppid);
    }
    parent_of
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn kill_reaches_descendants_across_setsid_boundaries() {
        let pid_file =
            std::env::temp_dir().join(format!("zed-kill-descendants-test-{}", std::process::id()));
        std::fs::remove_file(&pid_file).ok();

        let script = format!(
            "setsid sleep 300 & echo $! > {} && wait",
            pid_file.display()
        );

        let mut command = std::process::Command::new("sh");
        command.arg("-c").arg(&script);
        let mut child = Child::spawn(command, Stdio::null(), Stdio::null(), Stdio::null())
            .expect("failed to spawn sh");

        let grandchild_pid = {
            let deadline = Instant::now() + Duration::from_secs(2);
            loop {
                if let Ok(contents) = std::fs::read_to_string(&pid_file)
                    && let Ok(pid) = contents.trim().parse::<u32>()
                    && pid != 0
                {
                    break pid;
                }
                if Instant::now() >= deadline {
                    child.kill().ok();
                    std::fs::remove_file(&pid_file).ok();
                    panic!("grandchild did not start within 2s");
                }
                std::thread::sleep(Duration::from_millis(20));
            }
        };

        let sh_pid = child.id();
        assert_ne!(
            sid_of(grandchild_pid),
            Some(sh_pid),
            "test setup wrong: grandchild {grandchild_pid} should be in its own session, not sh's ({sh_pid})"
        );

        child.kill().expect("Child::kill");

        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            if !pid_exists(grandchild_pid) {
                std::fs::remove_file(&pid_file).ok();
                return;
            }
            if Instant::now() >= deadline {
                // SAFETY: cleanup of a test-spawned PID; ESRCH is fine.
                unsafe {
                    libc::kill(grandchild_pid as i32, libc::SIGKILL);
                }
                std::fs::remove_file(&pid_file).ok();
                panic!(
                    "grandchild pid {grandchild_pid} should have been killed by Child::kill's \
                     /proc walk, but it's still alive"
                );
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn pid_exists(pid: u32) -> bool {
        // SAFETY: kill(2) with signal 0 only probes for existence.
        unsafe { libc::kill(pid as i32, 0) == 0 }
    }

    fn sid_of(pid: u32) -> Option<u32> {
        // SAFETY: getsid(pid) reads session id; returns -1 on error.
        let sid = unsafe { libc::getsid(pid as i32) };
        if sid < 0 { None } else { Some(sid as u32) }
    }
}
