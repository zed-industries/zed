//! Process spawning with consistent asynchronous command semantics.
//!
//! This crate provides the command abstraction used by Zed. On macOS it
//! invokes `posix_spawn` directly so callers can request child setup that
//! `std::process::Command` would otherwise implement with `fork`.

use std::ffi::OsStr;
#[cfg(not(target_os = "macos"))]
use std::path::Path;

#[cfg(target_os = "macos")]
mod darwin;

#[cfg(target_os = "macos")]
pub use darwin::{Child, Command, Stdio};

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000_u32;

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
            let mut command = smol::process::Command::new(program);
            command.creation_flags(CREATE_NO_WINDOW);
            Self(command)
        }
        #[cfg(not(target_os = "windows"))]
        Self(smol::process::Command::new(program))
    }

    pub fn arg(&mut self, argument: impl AsRef<OsStr>) -> &mut Self {
        self.0.arg(argument);
        self
    }

    pub fn args<I, S>(&mut self, arguments: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.0.args(arguments);
        self
    }

    pub fn get_args(&self) -> impl Iterator<Item = &OsStr> {
        self.0.get_args()
    }

    pub fn env(&mut self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> &mut Self {
        self.0.env(key, value);
        self
    }

    pub fn envs<I, K, V>(&mut self, variables: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.0.envs(variables);
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

    pub fn current_dir(&mut self, directory: impl AsRef<Path>) -> &mut Self {
        self.0.current_dir(directory);
        self
    }

    pub fn stdin(&mut self, configuration: impl Into<Stdio>) -> &mut Self {
        self.0.stdin(configuration.into());
        self
    }

    pub fn stdout(&mut self, configuration: impl Into<Stdio>) -> &mut Self {
        self.0.stdout(configuration.into());
        self
    }

    pub fn stderr(&mut self, configuration: impl Into<Stdio>) -> &mut Self {
        self.0.stderr(configuration.into());
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
