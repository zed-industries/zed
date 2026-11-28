use std::ffi::OsStr;
#[cfg(not(target_os = "macos"))]
use std::path::Path;

#[cfg(target_os = "macos")]
mod darwin;

#[cfg(target_os = "macos")]
pub use darwin::{Child, Command, Stdio};

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000_u32;

pub use gpui_util::new_std_command;

pub fn new_command(program: impl AsRef<OsStr>) -> Command {
    Command::new(program)
}

#[cfg(not(target_os = "macos"))]
pub type Child = smol::process::Child;

#[cfg(not(target_os = "macos"))]
pub use std::process::Stdio;

#[cfg(not(target_os = "macos"))]
#[derive(Debug)]
pub struct Command(smol::process::Command);

#[cfg(not(target_os = "macos"))]
impl Command {
    #[inline]
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        #[cfg(target_os = "windows")]
        {
            use smol::process::windows::CommandExt;
            let mut cmd = smol::process::Command::new(program);
            cmd.creation_flags(CREATE_NO_WINDOW);
            Self(cmd)
        }
        #[cfg(not(target_os = "windows"))]
        Self(smol::process::Command::new(program))
    }

    pub fn arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
        self.0.arg(arg);
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.0.args(args);
        self
    }

    pub fn get_args(&self) -> impl Iterator<Item = &OsStr> {
        self.0.get_args()
    }

    pub fn env(&mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> &mut Self {
        self.0.env(key, val);
        self
    }

    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.0.envs(vars);
        self
    }

    pub fn env_remove(&mut self, key: impl AsRef<OsStr>) -> &mut Self {
        self.0.env_remove(key);
        self
    }

    pub fn env_clear(&mut self) -> &mut Self {
        self.0.env_clear();
        self
    }

    pub fn current_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self {
        self.0.current_dir(dir);
        self
    }

    pub fn stdin(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        self.0.stdin(cfg.into());
        self
    }

    pub fn stdout(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        self.0.stdout(cfg.into());
        self
    }

    pub fn stderr(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        self.0.stderr(cfg.into());
        self
    }

    pub fn kill_on_drop(&mut self, kill_on_drop: bool) -> &mut Self {
        self.0.kill_on_drop(kill_on_drop);
        self
    }

    pub fn spawn(&mut self) -> std::io::Result<Child> {
        self.0.spawn()
    }

    pub async fn output(&mut self) -> std::io::Result<std::process::Output> {
        self.0.output().await
    }

    pub async fn status(&mut self) -> std::io::Result<std::process::ExitStatus> {
        self.0.status().await
    }

    pub fn get_program(&self) -> &OsStr {
        self.0.get_program()
    }
}

/// Runs `program` until it exits and returns its exit code, blocking the
/// calling thread. `raw_arguments` is appended to the command line verbatim,
/// so quoting is up to the caller.
///
/// Unlike [`Command`], this does not go through `std::process::Command`, which
/// holds a process-wide lock across `CreateProcessW` so that inheritable stdio
/// handles cannot leak into unrelated children. Antivirus software can stall
/// the creation of a never-before-seen executable for many seconds, and every
/// other process spawn, including ones on the main thread, would wait for that
/// lock meanwhile. The child inherits no handles here, so the lock is not
/// needed.
#[cfg(target_os = "windows")]
pub fn run_without_inheriting_handles(program: &Path, raw_arguments: &str) -> anyhow::Result<u32> {
    use anyhow::Context as _;
    use std::os::windows::{
        ffi::OsStrExt as _,
        io::{AsRawHandle as _, FromRawHandle as _, OwnedHandle},
    };
    use windows::{
        Win32::{
            Foundation::{HANDLE, WAIT_OBJECT_0},
            System::Threading::{
                CREATE_NO_WINDOW, CreateProcessW, GetExitCodeProcess, INFINITE,
                PROCESS_INFORMATION, STARTF_FORCEOFFFEEDBACK, STARTUPINFOW, WaitForSingleObject,
            },
        },
        core::{HSTRING, PCWSTR, PWSTR},
    };

    let application_name = HSTRING::from(program.as_os_str());
    let mut command_line: Vec<u16> = "\""
        .encode_utf16()
        .chain(program.as_os_str().encode_wide())
        .chain("\" ".encode_utf16())
        .chain(raw_arguments.encode_utf16())
        .chain(std::iter::once(0))
        .collect();
    let startup_info = STARTUPINFOW {
        cb: size_of::<STARTUPINFOW>() as u32,
        // A GUI program that never shows a window would otherwise leave the
        // busy cursor on for several seconds.
        dwFlags: STARTF_FORCEOFFFEEDBACK,
        ..Default::default()
    };
    let mut process_information = PROCESS_INFORMATION::default();
    // SAFETY: every pointer refers to a NUL-terminated buffer or an
    // initialized struct that outlives the call.
    unsafe {
        CreateProcessW(
            &application_name,
            Some(PWSTR(command_line.as_mut_ptr())),
            None,
            None,
            false,
            CREATE_NO_WINDOW,
            None,
            PCWSTR::null(),
            &startup_info,
            &mut process_information,
        )
    }
    .with_context(|| format!("failed to start {}", program.display()))?;
    // SAFETY: `CreateProcessW` succeeded, so both handles are valid and ours
    // to close.
    let (process, _thread) = unsafe {
        (
            OwnedHandle::from_raw_handle(process_information.hProcess.0),
            OwnedHandle::from_raw_handle(process_information.hThread.0),
        )
    };

    let process_handle = HANDLE(process.as_raw_handle());
    // SAFETY: `process` keeps the handle open until this function returns.
    let wait_result = unsafe { WaitForSingleObject(process_handle, INFINITE) };
    if wait_result != WAIT_OBJECT_0 {
        return Err(std::io::Error::last_os_error())
            .with_context(|| format!("failed to wait for {} to exit", program.display()));
    }
    let mut exit_code = 0;
    // SAFETY: as above, and `exit_code` is a valid out pointer.
    unsafe { GetExitCodeProcess(process_handle, &mut exit_code) }
        .with_context(|| format!("failed to read the exit code of {}", program.display()))?;
    Ok(exit_code)
}

#[cfg(all(test, target_os = "windows"))]
mod windows_tests {
    use super::*;
    use std::path::PathBuf;

    fn command_interpreter() -> PathBuf {
        PathBuf::from(std::env::var_os("SystemRoot").expect("SystemRoot is always set on Windows"))
            .join("System32")
            .join("cmd.exe")
    }

    #[test]
    fn test_run_without_inheriting_handles_returns_exit_code() {
        let exit_code = run_without_inheriting_handles(&command_interpreter(), "/c exit 0");
        assert_eq!(exit_code.unwrap(), 0);

        let exit_code = run_without_inheriting_handles(&command_interpreter(), "/c exit 7");
        assert_eq!(exit_code.unwrap(), 7);
    }

    #[test]
    fn test_run_without_inheriting_handles_fails_for_missing_program() {
        let directory = tempfile::tempdir().unwrap();
        let missing_program = directory.path().join("missing.exe");

        let error = run_without_inheriting_handles(&missing_program, "").unwrap_err();
        let operating_system_error = error.downcast_ref::<windows::core::Error>();
        assert_eq!(
            operating_system_error.map(|error| error.code()),
            Some(windows::Win32::Foundation::ERROR_FILE_NOT_FOUND.to_hresult()),
            "unexpected error: {error:#}"
        );
    }
}
