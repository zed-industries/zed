use anyhow::Result;
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
    pub fn spawn(
        mut command: std::process::Command,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> Result<Self> {
        #[cfg(not(windows))]
        {
            crate::set_pre_exec_to_start_new_session(&mut command);
        }
        #[cfg(windows)]
        {
            // TODO(windows): create a job object and add the child process handle to it,
            // see https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects
        }
        let mut command = smol::process::Command::from(command);
        let process = command.stdin(stdin).stdout(stdout).stderr(stderr).spawn()?;
        Ok(Self { process })
    }

    pub fn into_inner(self) -> smol::process::Child {
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
        // TODO(windows): terminate the job object in kill
        self.process.kill()?;
        Ok(())
    }
}
