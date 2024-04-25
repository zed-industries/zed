use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Stdio;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

pub struct Process {
    program: PathBuf,
    working_dir: Option<PathBuf>,
    args: Vec<(OsString, bool)>,

    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
    stderr: Option<Stdio>,

    envs: HashMap<OsString, OsString>,
    cleared_env: bool,
    removed_envs: Vec<OsString>,

    flatpak_use_pty: bool,
    #[cfg(windows)]
    windows_creation_flags: u32,
}

impl Process {
    pub fn new<P: AsRef<Path>>(program: P) -> Self {
        Self {
            program: program.as_ref().to_path_buf(),
            working_dir: None,
            args: Vec::new(),

            stdin: None,
            stdout: None,
            stderr: None,

            envs: HashMap::new(),
            cleared_env: false,
            removed_envs: Vec::new(),

            flatpak_use_pty: false,
            #[cfg(windows)]
            windows_creation_flags: 0,
        }
    }

    pub fn flatpak_use_pty(&mut self) -> &mut Self {
        self.flatpak_use_pty = true;
        self
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.args.push((arg.as_ref().to_os_string(), false));
        self
    }

    pub fn args<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(&mut self, args: I) -> &mut Self {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.working_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    pub fn env<K: AsRef<OsStr>, V: AsRef<OsStr>>(&mut self, key: K, val: V) -> &mut Self {
        self.envs
            .insert(key.as_ref().to_os_string(), val.as_ref().to_os_string());
        self
    }

    pub fn envs<I: IntoIterator<Item = (K, V)>, K: AsRef<OsStr>, V: AsRef<OsStr>>(
        &mut self,
        vars: I,
    ) -> &mut Self {
        for (k, v) in vars {
            self.env(k, v);
        }
        self
    }

    pub fn env_remove<S: AsRef<OsStr>>(&mut self, key: S) -> &mut Self {
        self.envs.remove(key.as_ref());
        self.removed_envs.push(key.as_ref().to_os_string());
        self
    }

    pub fn env_clear(&mut self) -> &mut Self {
        self.cleared_env = true;
        self.envs.clear();
        self
    }

    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.stdin = Some(cfg.into());
        self
    }

    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.stdout = Some(cfg.into());
        self
    }

    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.stderr = Some(cfg.into());
        self
    }

    pub fn spawn(&mut self) -> Result<std::process::Child> {
        self.make_command().spawn()
    }

    pub fn output(&mut self) -> Result<std::process::Output> {
        self.make_command().output()
    }

    pub fn status(&mut self) -> Result<std::process::ExitStatus> {
        self.make_command().status()
    }

    pub fn spawn_async(&mut self, kill_on_drop: bool) -> Result<smol::process::Child> {
        self.make_async_command(kill_on_drop).spawn()
    }

    pub fn output_async(
        &mut self,
        kill_on_drop: bool,
    ) -> impl std::future::Future<Output = Result<std::process::Output>> {
        self.make_async_command(kill_on_drop).output()
    }

    pub fn status_async(
        &mut self,
        kill_on_drop: bool,
    ) -> impl std::future::Future<Output = Result<std::process::ExitStatus>> {
        self.make_async_command(kill_on_drop).status()
    }

    pub fn get_program(&self) -> &PathBuf {
        &self.program
    }

    pub fn get_args(&self) -> Vec<OsString> {
        self.args
            .iter()
            .map(|(arg, _)| arg.clone())
            .collect::<Vec<_>>()
    }

    pub fn get_envs(&self) -> &HashMap<OsString, OsString> {
        &self.envs
    }

    #[cfg(windows)]
    pub fn windows_creation_flags(&mut self, flags: u32) -> &mut Self {
        self.windows_creation_flags |= flags;
        self
    }

    pub fn windows_raw_arg<S: AsRef<OsStr>>(&mut self, raw_text: S) -> &mut Self {
        self.args.push((raw_text.as_ref().to_os_string(), true));
        self
    }

    pub fn get_actual_program(&self) -> PathBuf {
        #[cfg(feature = "flatpak")]
        return <PathBuf as std::str::FromStr>::from_str("/app/bin/host-spawn").unwrap();
        #[cfg(not(feature = "flatpak"))]
        return self.program.clone();
    }

    pub fn get_actual_args(&mut self) -> Vec<String> {
        let command = self.make_command_common(true);
        command
            .get_args()
            .map(|arg| arg.to_str().unwrap().to_string())
            .collect()
    }

    fn make_command_common(&mut self, only_args: bool) -> std::process::Command {
        let env = self
            .envs
            .clone()
            .into_iter()
            .chain(
                std::env::vars()
                    .map(|(key, val)| (OsString::from(key), OsString::from(val)))
                    .filter(|(key, _)| !self.removed_envs.contains(key) && !self.cleared_env),
            )
            .collect::<Vec<_>>();

        #[cfg(not(feature = "flatpak"))]
        let mut command = std::process::Command::new(&self.program);
        #[cfg(feature = "flatpak")]
        let mut command = {
            let mut command = std::process::Command::new("/app/bin/host-spawn");
            if let Some(working_directory) = &self.working_dir {
                command.arg("-directory").arg(working_directory);
            }
            if !env.is_empty() {
                command.arg("-env").arg(
                    env.iter()
                        .map(|(key, _)| key.clone())
                        .collect::<Vec<_>>()
                        .join(&OsString::from(",")),
                );
            }
            command.arg(if self.flatpak_use_pty {
                "-pty"
            } else {
                "-no-pty"
            });
            command.arg(self.program.as_os_str());
            command
        };

        if !only_args {
            command.env_clear().envs(env);
            if let Some(working_directory) = &self.working_dir {
                command.current_dir(working_directory);
            }
        }

        for (arg, is_raw) in &self.args {
            if *is_raw {
                #[cfg(windows)]
                command.raw_arg(arg);
            } else {
                command.arg(arg);
            }
        }
        command
    }

    fn make_command(&mut self) -> std::process::Command {
        let mut command = self.make_command_common(false);
        if self.stdin.is_some() {
            command.stdin(self.stdin.take().unwrap()); // smol::process::Command::from doesn't work with stdin et al
        }
        if self.stdout.is_some() {
            command.stdout(self.stdout.take().unwrap());
        }
        if self.stderr.is_some() {
            command.stderr(self.stderr.take().unwrap());
        }
        command
    }

    fn make_async_command(&mut self, kill_on_drop: bool) -> smol::process::Command {
        let mut command = smol::process::Command::from(self.make_command_common(false));
        if self.stdin.is_some() {
            command.stdin(self.stdin.take().unwrap()); // smol::process::Command::from doesn't work with stdin et al
        }
        if self.stdout.is_some() {
            command.stdout(self.stdout.take().unwrap());
        }
        if self.stderr.is_some() {
            command.stderr(self.stderr.take().unwrap());
        }
        command.kill_on_drop(kill_on_drop);
        command
    }
}
