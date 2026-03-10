//! cgroups v2 session management for Linux process tracking.
//!
//! Each terminal session gets its own cgroup, providing an inescapable
//! mechanism for tracking and killing all descendant processes.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// A cgroup v2 session for tracking processes spawned by a terminal.
///
/// All processes in the cgroup can be killed atomically, regardless of
/// `setsid()`, `setpgid()`, or reparenting.
pub struct CgroupSession {
    cgroup_path: PathBuf,
    owns_cgroup: bool,
}

impl CgroupSession {
    /// Create a new cgroup under the user's systemd slice.
    ///
    /// The cgroup is created at:
    /// `/sys/fs/cgroup/user.slice/user-<uid>.slice/user@<uid>.service/zed-terminal-<uuid>.scope`
    pub fn new() -> io::Result<Self> {
        let uid = unsafe { libc::getuid() };
        let uuid = uuid::Uuid::new_v4();
        let scope_name = format!("zed-terminal-{uuid}.scope");

        // Try the systemd user slice first
        let user_slice = format!("/sys/fs/cgroup/user.slice/user-{uid}.slice/user@{uid}.service");

        let cgroup_path = if Path::new(&user_slice).exists() {
            let path = PathBuf::from(&user_slice).join(&scope_name);
            fs::create_dir(&path)?;
            path
        } else {
            // Fallback: try directly under the current process's cgroup
            let self_cgroup = read_self_cgroup()?;
            let parent = PathBuf::from("/sys/fs/cgroup").join(self_cgroup.trim_start_matches('/'));
            if !parent.exists() {
                return Err(io::Error::other(
                    "cgroups v2 not available: cannot find parent cgroup",
                ));
            }
            let path = parent.join(&scope_name);
            fs::create_dir(&path)?;
            path
        };

        Ok(Self {
            cgroup_path,
            owns_cgroup: true,
        })
    }

    /// Reconstruct a CgroupSession from a path string (used by the child process).
    /// Does NOT create the cgroup — assumes the parent already created it.
    pub fn from_path(path: &str) -> Self {
        Self {
            cgroup_path: PathBuf::from(path),
            owns_cgroup: false,
        }
    }

    /// Returns the cgroup filesystem path.
    pub fn path(&self) -> &Path {
        &self.cgroup_path
    }

    /// Returns the cgroup path as a string for serialization.
    pub fn path_string(&self) -> String {
        self.cgroup_path.to_string_lossy().into_owned()
    }

    /// Move a process into this cgroup by writing its PID to cgroup.procs.
    pub fn add_process(&self, pid: libc::pid_t) -> io::Result<()> {
        let procs_path = self.cgroup_path.join("cgroup.procs");
        fs::write(&procs_path, pid.to_string().as_bytes())
    }

    /// Kill all processes in the cgroup.
    ///
    /// Tries the atomic `cgroup.kill` interface first (kernel 5.14+),
    /// falling back to reading cgroup.procs and killing each PID.
    pub fn kill_all(&self) -> io::Result<()> {
        // Step 1: Freeze the cgroup to prevent new forks
        let freeze_path = self.cgroup_path.join("cgroup.freeze");
        if freeze_path.exists() {
            if let Err(err) = fs::write(&freeze_path, b"1") {
                log::debug!("Failed to freeze cgroup: {err}");
            }
        }

        // Step 2: Try atomic kill via cgroup.kill (kernel 5.14+)
        let kill_path = self.cgroup_path.join("cgroup.kill");
        if kill_path.exists() {
            if fs::write(&kill_path, b"1").is_ok() {
                return Ok(());
            }
        }

        // Step 3: Fallback — read cgroup.procs and kill each PID
        let procs_path = self.cgroup_path.join("cgroup.procs");
        loop {
            let content = fs::read_to_string(&procs_path)?;
            let pids: Vec<libc::pid_t> = content
                .lines()
                .filter_map(|line| line.trim().parse().ok())
                .collect();

            if pids.is_empty() {
                break;
            }

            for pid in &pids {
                unsafe {
                    libc::kill(*pid, libc::SIGKILL);
                }
            }

            // Brief sleep to let processes die before re-scanning
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        Ok(())
    }

    /// Remove the cgroup directory. Must be called after all processes are dead.
    pub fn cleanup(&self) {
        if let Err(err) = fs::remove_dir(&self.cgroup_path) {
            log::warn!(
                "Failed to remove cgroup directory {:?}: {err}",
                self.cgroup_path
            );
        }
    }

    /// Kill all processes and clean up the cgroup.
    pub fn kill_all_and_cleanup(&self) {
        if let Err(err) = self.kill_all() {
            log::warn!("Failed to kill cgroup processes: {err}");
        }
        self.cleanup();
    }
}

impl Drop for CgroupSession {
    fn drop(&mut self) {
        if self.owns_cgroup {
            self.kill_all_and_cleanup();
        }
    }
}

/// Read the current process's cgroup path from /proc/self/cgroup.
/// For cgroups v2, the format is "0::/path".
fn read_self_cgroup() -> io::Result<String> {
    let content = fs::read_to_string("/proc/self/cgroup")?;
    for line in content.lines() {
        if let Some(path) = line.strip_prefix("0::") {
            return Ok(path.to_string());
        }
    }
    Err(io::Error::other(
        "cgroups v2 not found in /proc/self/cgroup",
    ))
}
