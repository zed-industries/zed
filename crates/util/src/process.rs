use anyhow::{Context as _, Result};
use std::process::Stdio;

/// A wrapper around `smol::process::Child` that ensures all subprocesses
/// are killed when the process is terminated by using process groups.
pub struct Child {
    process: smol::process::Child,
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
        // TODO(windows): create a job object and add the child process handle to it,
        // see https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects
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

    pub fn into_inner(self) -> smol::process::Child {
        self.process
    }

    #[cfg(not(windows))]
    pub fn kill(&mut self) -> Result<()> {
        let pid = self.process.id();
        #[cfg(target_os = "linux")]
        kill_descendant_tree(pid);
        unsafe {
            libc::killpg(pid as i32, libc::SIGKILL);
        }
        Ok(())
    }

    #[cfg(windows)]
    pub fn kill(&mut self) -> Result<()> {
        // TODO(windows): terminate the job object in kill
        self.process.kill()?;
        Ok(())
    }
}

#[cfg(target_os = "linux")]
fn kill_descendant_tree(root_pid: u32) {
    use std::collections::HashMap;

    let mut parent_of: HashMap<u32, u32> = HashMap::new();
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return;
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
