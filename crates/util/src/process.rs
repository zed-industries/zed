use anyhow::{Context as _, Result};
use std::process::Stdio;

/// A wrapper around `smol::process::Child` that attempts to clean up subprocess
/// trees: on Unix by using process groups, and on Windows by using job objects.
///
/// On Unix, dropping this struct kills the child's process group. Descendants
/// that leave the group are not tracked.
///
/// On Windows, if job setup and assignment succeed, dropping this struct
/// closes the job handle and terminates processes in the job. If either step
/// fails, the direct child remains usable but descendants are not tracked.
/// A descendant spawned before job assignment can also escape the job.
pub struct Child {
    /// Held for its `Drop`, never read.
    ///
    /// Declared before `process` deliberately: fields drop in declaration
    /// order, and dropping `process` synchronously reaps the child if it has
    /// already exited, which releases its pid. Signalling the group first
    /// means the pid we pass to `killpg` is still held by `process`.
    #[cfg(not(windows))]
    _process_group: ProcessGroup,
    process: smol::process::Child,
    #[cfg(windows)]
    job: Option<windows_job::JobObject>,
}

/// Kills a child's process group when dropped, so descendants the child
/// spawned do not outlive it.
///
/// This mirrors what the job object does on Windows. It is a separate guard
/// rather than a `Drop` impl on [`Child`] so that [`Child::output`] can still
/// move `process` out of `self`, which a `Drop` impl would forbid.
#[cfg(not(windows))]
struct ProcessGroup(u32);

#[cfg(not(windows))]
impl Drop for ProcessGroup {
    fn drop(&mut self) {
        // SAFETY: `killpg` takes no pointers and cannot fail in a way we can
        // act on. `Child::spawn` puts the child in its own session, and
        // therefore its own process group, so the group id is the child's pid
        // and this can never signal Zed's own process group.
        unsafe {
            libc::killpg(self.0 as i32, libc::SIGKILL);
        }
    }
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
        let _process_group = ProcessGroup(process.id());
        Ok(Self {
            process,
            _process_group,
        })
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
        // creation. The window is normally short, but descendants created in
        // it are not tracked by the job.
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
        // pull `process` out first. That drops the teardown guard early and
        // kills the child before `output()` finishes collecting its
        // stdout/stderr: on Windows by triggering
        // `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, and on Unix by signalling the
        // child's process group.
        Ok(self.process.output().await?)
    }

    #[cfg(not(windows))]
    pub fn kill(&mut self) -> Result<()> {
        let pid = self.process.id();
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

    /// Returns job activity, or `None` when descendant cleanup is unavailable.
    ///
    /// The count includes descendants after the direct child exits. `None`
    /// means job setup failed, not that the process tree is empty.
    ///
    /// # Errors
    ///
    /// Returns an error if Windows cannot query the owned job's activity.
    #[cfg(windows)]
    pub fn has_running_processes(&self) -> Result<Option<bool>> {
        self.job
            .as_ref()
            .map(windows_job::JobObject::has_running_processes)
            .transpose()
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
                JOBOBJECT_BASIC_ACCOUNTING_INFORMATION, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
                JobObjectBasicAccountingInformation, JobObjectExtendedLimitInformation,
                QueryInformationJobObject, SetInformationJobObject, TerminateJobObject,
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

        pub(crate) fn has_running_processes(&self) -> Result<bool> {
            let mut accounting = JOBOBJECT_BASIC_ACCOUNTING_INFORMATION::default();
            // SAFETY: self owns a valid job handle, and accounting is writable
            // for the exact buffer size passed to this synchronous query.
            unsafe {
                QueryInformationJobObject(
                    Some(self.0),
                    JobObjectBasicAccountingInformation,
                    &mut accounting as *mut _ as *mut _,
                    size_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>() as u32,
                    None,
                )
                .context("failed to query active job processes")?;
            }
            Ok(accounting.ActiveProcesses > 0)
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

#[cfg(all(test, unix))]
mod unix_tests {
    use super::*;
    use std::time::{Duration, Instant};

    /// Spawns a process tree `sh -> sleep` via `Child::spawn` and returns the
    /// `Child` along with the pid of the grandchild (`sleep`).
    fn spawn_process_tree(temp_dir: &std::path::Path) -> (Child, u32) {
        let pid_file = temp_dir.join("grandchild_pid");
        let mut command = std::process::Command::new("/bin/sh");
        command.arg("-c").arg(format!(
            "sleep 60 & echo $! > '{}'; wait",
            pid_file.display()
        ));
        let child = Child::spawn(command, Stdio::null(), Stdio::null(), Stdio::null())
            .expect("failed to spawn sh");

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

    /// Signal 0 performs error checking without sending a signal. Only
    /// `ESRCH` means the process is gone: `EPERM` means it is still there and
    /// merely not ours to signal, so treating any error as "exited" would let
    /// these tests pass without the process group being torn down.
    fn process_is_alive(pid: u32) -> bool {
        // SAFETY: `kill` with signal 0 takes no pointers and only probes
        // whether the pid exists.
        let result = unsafe { libc::kill(pid as libc::pid_t, 0) };
        result != -1 || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
    }

    fn assert_process_exits(pid: u32, message: &str) {
        // The grandchild is reparented to init once its parent dies, so this
        // waits on init reaping it. Matches `wait_until_gone` in
        // `crate::command::darwin`.
        let deadline = Instant::now() + Duration::from_secs(10);
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

#[cfg(all(test, windows))]
mod windows_tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test_unmanaged_child_reports_unavailable_job_activity() {
        let mut command = gpui_util::new_std_command("ping.exe");
        command.args(["-n", "60", "127.0.0.1"]);
        let process = smol::process::Command::from(command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .expect("failed to spawn unmanaged process");
        let mut child = Child { process, job: None };

        let pid = child.id();
        assert!(
            process_is_alive(pid),
            "direct child should still be running"
        );
        assert_eq!(
            child.has_running_processes().expect("failed to query job"),
            None
        );
        child
            .kill()
            .expect("failed to kill unassigned direct child");
        smol::block_on(child.status()).expect("failed to reap direct child");
        assert_process_exits(pid, "direct child should be terminated");
    }

    #[test]
    fn test_job_tracks_grandchild_after_direct_child_exits() {
        let temp_dir = tempfile::tempdir().unwrap();
        let pid_file = temp_dir.path().join("grandchild_pid");
        let mut command = gpui_util::new_std_command("powershell.exe");
        command.args(["-NoProfile", "-Command"]).arg(format!(
            "$p = Start-Process -FilePath ping.exe -ArgumentList @('-n','60','127.0.0.1') -PassThru -WindowStyle Hidden; \
             Set-Content -LiteralPath '{}' -Value $p.Id",
            pid_file.display()
        ));
        let mut child = Child::spawn(command, Stdio::null(), Stdio::null(), Stdio::null())
            .expect("failed to spawn powershell");
        let status = smol::block_on(child.status()).expect("failed to await powershell");
        assert!(status.success(), "powershell failed: {status}");

        let grandchild_pid = std::fs::read_to_string(&pid_file)
            .expect("powershell did not record grandchild pid")
            .trim()
            .parse::<u32>()
            .expect("invalid grandchild pid");
        assert!(process_is_alive(grandchild_pid));
        assert_eq!(
            child.has_running_processes().expect("failed to query job"),
            Some(true),
            "job should remain active after its direct child exits"
        );

        child.kill().expect("failed to terminate job");
        let deadline = Instant::now() + Duration::from_secs(2);
        while child.has_running_processes().expect("failed to query job") != Some(false) {
            assert!(Instant::now() < deadline, "job did not become inactive");
            std::thread::sleep(Duration::from_millis(50));
        }
        assert_process_exits(grandchild_pid, "job grandchild should be terminated");
    }

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
