use anyhow::{Context as _, Result};
use std::process::Stdio;

/// A wrapper around `smol::process::Child` that ensures all subprocesses
/// are killed when the process is terminated by using process groups.
pub struct Child {
    process: smol::process::Child,
    /// A Windows Job Object configured with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`.
    /// When this handle is closed (on drop), all processes assigned to the job
    /// — including any grandchildren — are terminated automatically.
    #[cfg(windows)]
    _job: Option<Win32JobObject>,
}

/// RAII wrapper around a Windows Job Object handle.
///
/// The job is created with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, so closing
/// the last handle to the job will terminate all processes assigned to it.
#[cfg(windows)]
struct Win32JobObject {
    handle: windows::Win32::Foundation::HANDLE,
}

#[cfg(windows)]
// SAFETY: The Win32 Job Object handle can be safely sent between threads.
// Job object handles in Windows are kernel objects and the operations we
// perform (close/terminate) are inherently thread-safe.
unsafe impl Send for Win32JobObject {}

#[cfg(windows)]
// SAFETY: The Win32 Job Object handle can be safely shared between threads.
// We only perform atomic kernel operations (close/terminate) on the handle.
unsafe impl Sync for Win32JobObject {}

#[cfg(windows)]
impl Win32JobObject {
    /// Creates a new anonymous Job Object configured to kill all assigned
    /// processes when the job handle is closed.
    fn new() -> Result<Self> {
        use windows::Win32::System::JobObjects::{
            CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
            JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
            SetInformationJobObject,
        };

        unsafe {
            let handle = CreateJobObjectW(None, None).context("failed to create job object")?;

            let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

            SetInformationJobObject(
                handle,
                JobObjectExtendedLimitInformation,
                &info as *const _ as *const std::ffi::c_void,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
            .context("failed to set job object information")?;

            Ok(Self { handle })
        }
    }

    /// Assigns a process to this job object by its raw handle.
    fn assign_process(&self, process_handle: windows::Win32::Foundation::HANDLE) -> Result<()> {
        use windows::Win32::System::JobObjects::AssignProcessToJobObject;

        unsafe {
            AssignProcessToJobObject(self.handle, process_handle)
                .context("failed to assign process to job object")?;
        }
        Ok(())
    }

    /// Terminates all processes in the job object.
    fn terminate(&self) -> Result<()> {
        use windows::Win32::System::JobObjects::TerminateJobObject;

        unsafe {
            TerminateJobObject(self.handle, 1).context("failed to terminate job object")?;
        }
        Ok(())
    }
}

#[cfg(windows)]
impl Drop for Win32JobObject {
    fn drop(&mut self) {
        unsafe {
            let _ = windows::Win32::Foundation::CloseHandle(self.handle);
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
            .with_context(|| format!("failed to spawn command {command:?}"))?;
        Ok(Self { process })
    }

    #[cfg(windows)]
    pub fn spawn(
        command: std::process::Command,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> Result<Self> {
        use std::os::windows::io::AsRawHandle;
        use windows::Win32::Foundation::HANDLE;

        let mut command = smol::process::Command::from(command);
        let process = command
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .with_context(|| format!("failed to spawn command {command:?}"))?;

        // Create a job object and assign the child process to it.
        // The job is configured with JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE so that
        // all processes in the job (including any children spawned by the process)
        // are terminated when the job handle is closed (i.e. when Child is dropped).
        let job = match Win32JobObject::new() {
            Ok(job) => {
                let raw_handle = process.as_raw_handle();
                let process_handle = HANDLE(raw_handle);
                match job.assign_process(process_handle) {
                    Ok(()) => Some(job),
                    Err(e) => {
                        log::warn!("failed to assign child process to job object: {e:#}");
                        None
                    }
                }
            }
            Err(e) => {
                log::warn!("failed to create job object for child process: {e:#}");
                None
            }
        };

        Ok(Self { process, _job: job })
    }

    /// Consumes this wrapper and returns the inner `smol::process::Child`.
    ///
    /// On Windows, this leaks the job object handle so the process is not
    /// killed. The caller assumes full responsibility for process lifetime.
    pub fn into_inner(self) -> smol::process::Child {
        #[cfg(windows)]
        {
            // Leak the job object handle so that dropping it doesn't kill the
            // process tree. The caller now owns the process lifetime.
            std::mem::forget(self._job);
        }
        self.process
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
        if let Some(ref job) = self._job {
            job.terminate()?;
        } else {
            // Fallback: kill only the immediate process if no job object
            self.process.kill()?;
        }
        Ok(())
    }
}
