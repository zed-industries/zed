use anyhow::{Context as _, Result};
use std::process::Stdio;

/// A wrapper around `smol::process::Child` that ensures all subprocesses
/// (including descendants that escape the original process group via
/// `setsid` on Unix or `CREATE_BREAKAWAY_FROM_JOB` on Windows) are killed
/// when the wrapper's `kill` is called.
#[cfg(not(windows))]
pub struct Child {
    process: smol::process::Child,
}

#[cfg(windows)]
pub struct Child {
    process: smol::process::Child,
    /// Owns the job object the spawned process is assigned to.
    /// `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` means the kernel terminates every
    /// process in the job when this handle is closed, which is the safety net
    /// for app-quit cleanup.
    _job: std::os::windows::io::OwnedHandle,
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
        use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
        use windows::Win32::Foundation::HANDLE;
        use windows::Win32::System::JobObjects::{
            AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
            JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
            SetInformationJobObject,
        };
        use windows::core::PCWSTR;

        // SAFETY: CreateJobObjectW with null attrs/name creates an unnamed job
        // with default security; returns Err on failure.
        let job_handle = unsafe { CreateJobObjectW(None, PCWSTR::null()) }
            .context("CreateJobObjectW failed")?;
        // SAFETY: CreateJobObjectW transfers ownership of the handle. Wrapping
        // it in OwnedHandle means the kernel cleans up the job (and, with
        // KILL_ON_JOB_CLOSE, every process in it) when this struct is dropped.
        let job: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(job_handle.0 as _) };

        let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
        info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        // SAFETY: pointer to local info, valid for the duration of the call.
        unsafe {
            SetInformationJobObject(
                HANDLE(job.as_raw_handle() as _),
                JobObjectExtendedLimitInformation,
                &info as *const _ as *const _,
                std::mem::size_of_val(&info) as u32,
            )
        }
        .context("SetInformationJobObject failed")?;

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

        // Race window: if the spawned process forks descendants between
        // `spawn` returning and `AssignProcessToJobObject` running, those
        // descendants escape the job. Closing it race-free would require
        // CREATE_SUSPENDED, which std::process::Command doesn't expose.
        // Debug adapters don't fork during the first instructions of startup,
        // so this is acceptable in practice.
        // SAFETY: process and job handles are both owned and valid here.
        unsafe {
            AssignProcessToJobObject(
                HANDLE(job.as_raw_handle() as _),
                HANDLE(process.as_raw_handle() as _),
            )
        }
        .context("AssignProcessToJobObject failed")?;

        Ok(Self {
            process,
            _job: job,
        })
    }

    pub fn into_inner(self) -> smol::process::Child {
        self.process
    }

    #[cfg(not(windows))]
    pub fn kill(&mut self) -> Result<()> {
        let pid = self.process.id();
        kill_descendant_tree(pid);
        // SAFETY: killpg with SIGKILL on the original process group.
        // Returns ESRCH if the group is empty, which we ignore.
        unsafe {
            libc::killpg(pid as i32, libc::SIGKILL);
        }
        Ok(())
    }

    #[cfg(windows)]
    pub fn kill(&mut self) -> Result<()> {
        use std::os::windows::io::AsRawHandle;
        use windows::Win32::Foundation::HANDLE;
        use windows::Win32::System::JobObjects::TerminateJobObject;

        // SAFETY: job handle is owned by self and valid for this call.
        // TerminateJobObject atomically kills every process in the job.
        unsafe { TerminateJobObject(HANDLE(self._job.as_raw_handle() as _), 1) }
            .context("TerminateJobObject failed")?;
        Ok(())
    }
}

#[cfg(not(windows))]
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
        // SAFETY: kill(2) with SIGKILL on a nonexistent PID returns ESRCH,
        // which we ignore.
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
        let Some(pid_str) = name.to_str() else { continue };
        let Ok(pid) = pid_str.parse::<u32>() else { continue };
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
        let Some(pid_str) = parts.next() else { continue };
        let Some(ppid_str) = parts.next() else { continue };
        let Ok(pid) = pid_str.parse::<u32>() else { continue };
        let Ok(ppid) = ppid_str.parse::<u32>() else { continue };
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
        let pid_file = std::env::temp_dir().join(format!(
            "zed-kill-descendants-test-{}",
            std::process::id()
        ));
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
