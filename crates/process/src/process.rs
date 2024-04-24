use std::ffi::{OsStr, OsString};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, CommandEnvs, ExitStatus, Output, Stdio};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
pub trait WindowsCommandExt {
    fn creation_flags(&mut self, flags: u32) -> &mut Process;
    fn raw_arg<S: AsRef<OsStr>>(&mut self, text_to_append_as_is: S) -> &mut Process;
}

pub struct Process {
    process: std::process::Command,

    program: OsString,
    working_dir: Option<PathBuf>,
    args: Vec<OsString>,
    use_pty: bool,
}

impl Process {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        Self {
            #[cfg(feature = "flatpak")]
            process: Command::new("/app/bin/host-spawn"),
            #[cfg(not(feature = "flatpak"))]
            process: Command::new(&program),

            program: program.as_ref().into(),
            working_dir: None,
            args: Vec::new(),
            use_pty: false,
        }
    }

    pub fn flatpak_use_pty(&mut self) -> &mut Self {
        self.use_pty = true;
        self
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.args.push(arg.as_ref().into());
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
        self.process.env(key, val);
        self
    }

    pub fn envs<I: IntoIterator<Item = (K, V)>, K: AsRef<OsStr>, V: AsRef<OsStr>>(
        &mut self,
        vars: I,
    ) -> &mut Self {
        self.process.envs(vars);
        self
    }

    pub fn env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Self {
        self.process.env_remove(key);
        self
    }

    pub fn env_clear(&mut self) -> &mut Self {
        self.process.env_clear();
        self
    }

    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.process.stdin(cfg);
        self
    }

    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.process.stdout(cfg);
        self
    }

    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.process.stderr(cfg);
        self
    }

    pub fn spawn(&mut self) -> Result<Child> {
        self.process.args(self.get_actual_args());
        if let Some(working_dir) = &self.working_dir {
            self.process.current_dir(working_dir);
        }

        self.process.spawn()
    }

    pub fn output(&mut self) -> Result<Output> {
        self.process.args(self.get_actual_args());
        if let Some(working_dir) = &self.working_dir {
            self.process.current_dir(working_dir);
        }

        self.process.output()
    }

    pub fn status(&mut self) -> Result<ExitStatus> {
        self.process.args(self.get_actual_args());
        if let Some(working_dir) = &self.working_dir {
            self.process.current_dir(working_dir);
        }

        self.process.status()
    }

    pub fn get_program(&self) -> &OsStr {
        &self.program
    }

    pub fn get_args(&self) -> &[OsString] {
        &self.args
    }

    pub fn get_envs(&self) -> CommandEnvs<'_> {
        self.process.get_envs()
    }

    pub fn get_actual_program(&self) -> &OsStr {
        self.process.get_program()
    }

    pub fn get_actual_args(&self) -> Vec<OsString> {
        #[cfg(feature = "flatpak")]
        let mut args = self.flatpak_args();

        #[cfg(not(feature = "flatpak"))]
        let mut args = Vec::new();

        for arg in &self.args {
            args.push(arg.clone());
        }
        args
    }

    #[cfg(feature = "flatpak")]
    fn flatpak_args(&self) -> Vec<OsString> {
        let env_keys = self
            .process
            .get_envs()
            .map(|(k, _)| k.to_str().unwrap().to_string())
            .chain(std::env::vars().map(|(k, _)| k))
            .collect::<Vec<_>>()
            .join(",");

        let mut flatpak_args = Vec::new();

        flatpak_args.push(if self.use_pty {
            "-pty".into()
        } else {
            "-no-pty".into()
        });
        if !env_keys.is_empty() {
            flatpak_args.push("-env".into());
            flatpak_args.push(env_keys.into());
        }
        if let Some(working_dir) = &self.working_dir {
            flatpak_args.push("-directory".into());
            flatpak_args.push(working_dir.into());
        }
        flatpak_args.push(self.program.clone());
        flatpak_args
    }
}

#[cfg(windows)]
impl WindowsCommandExt for Process {
    fn creation_flags(&mut self, flags: u32) -> &mut Self {
        self.process.creation_flags(flags);
        self
    }

    fn raw_arg<S: AsRef<OsStr>>(&mut self, raw_text: S) -> &mut Self {
        self.process.raw_arg(raw_text);
        self
    }
}
