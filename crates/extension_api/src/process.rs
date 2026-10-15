//! A module for working with processes.

use crate::wit::zed::extension::process;
pub use crate::wit::zed::extension::process::{Command, Output};

impl Command {
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            command: program.into(),
            args: Vec::new(),
            env: Vec::new(),
        }
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.push((key.into(), value.into()));
        self
    }

    pub fn envs(
        mut self,
        envs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.env.extend(
            envs.into_iter()
                .map(|(key, value)| (key.into(), value.into())),
        );
        self
    }

    pub fn output(&mut self) -> Result<Output, String> {
        process::run_command(self)
    }
}

/// Spawns a background child process.
/// Output will be delivered via `Extension::process_output` and exit via `Extension::process_exit`.
pub fn spawn(
    command: &str,
    args: &[&str],
    env: &[(&str, &str)],
    cwd: Option<&str>,
) -> Result<u64, String> {
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let env: Vec<(String, String)> = env
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    process::spawn(command, &args, &env, cwd)
}

/// Terminates a spawned background process by handle.
pub fn kill(handle: u64) -> Result<(), String> {
    process::kill(handle)
}
