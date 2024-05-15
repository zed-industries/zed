use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Stdio;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// A process builder, providing fine-grained control
/// over how a new process should be spawned.
///
/// A default configuration can be
/// generated using `Command::new(program)`, where `program` gives a path to the
/// program to be executed. Additional builder methods allow the configuration
/// to be changed (for example, by adding arguments) prior to spawning
/// This wraps both std::process::Command and smol::process::Command as well as
/// providing support for spawning threads using [https://man7.org/linux/man-pages/man1/flatpak-spawn.1.html](flatpak-spawn) when in flatpak
/// environments.
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

    #[cfg(windows)]
    windows_creation_flags: u32,
}

impl Process {
    /// Constructs a new `Command` for launching the program at
    /// path `program`, with the following default configuration:
    ///
    /// * No arguments to the program
    /// * Inherit the current process's environment
    /// * Inherit the current process's working directory
    /// * Inherit stdin/stdout/stderr for [`spawn`] or [`status`], but create pipes for [`output`]
    ///
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    ///
    /// Builder methods are provided to change these defaults and
    /// otherwise configure the process.
    ///
    /// If `program` is not an absolute path, the `PATH` will be searched in
    /// an OS-defined way.
    ///
    /// The search path to be used may be controlled by setting the
    /// `PATH` environment variable on the Command,
    /// but this has some implementation limitations on Windows
    /// (see issue #37519).
    ///
    /// # Platform-specific behavior
    ///
    /// Note on Windows: For executable files with the .exe extension,
    /// it can be omitted when specifying the program for this Command.
    /// However, if the file has a different extension,
    /// a filename including the extension needs to be provided,
    /// otherwise the file won't be found.
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

            #[cfg(windows)]
            windows_creation_flags: 0,
        }
    }

    /// Adds an argument to pass to the program.
    ///
    /// Only one argument can be passed per use. To pass multiple arguments see [`args`].
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.args.push((arg.as_ref().to_os_string(), false));
        self
    }

    // Adds multiple arguments to pass to the program.
    ///
    /// To pass a single argument see [`arg`].
    pub fn args<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(&mut self, args: I) -> &mut Self {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    /// Inserts or updates an explicit environment variable mapping.
    ///
    /// This method allows you to add an environment variable mapping to the spawned process or
    /// overwrite a previously set value. You can use [`Command::envs`] to set multiple environment
    /// variables simultaneously.
    ///
    /// Child processes will inherit environment variables from their parent process by default.
    /// Environment variables explicitly set using [`Command::env`] take precedence over inherited
    /// variables. You can disable environment variable inheritance entirely using
    /// [`Command::env_clear`] or for a single key using [`Command::env_remove`].
    pub fn env<K: AsRef<OsStr>, V: AsRef<OsStr>>(&mut self, key: K, val: V) -> &mut Self {
        self.envs
            .insert(key.as_ref().to_os_string(), val.as_ref().to_os_string());
        self
    }

    /// Inserts or updates multiple explicit environment variable mappings.
    ///
    /// This method allows you to add multiple environment variable mappings to the spawned process
    /// or overwrite previously set values. You can use [`Command::env`] to set a single environment
    /// variable.
    ///
    /// Child processes will inherit environment variables from their parent process by default.
    /// Environment variables explicitly set using [`Command::envs`] take precedence over inherited
    /// variables. You can disable environment variable inheritance entirely using
    /// [`Command::env_clear`] or for a single key using [`Command::env_remove`].
    pub fn envs<I: IntoIterator<Item = (K, V)>, K: AsRef<OsStr>, V: AsRef<OsStr>>(
        &mut self,
        vars: I,
    ) -> &mut Self {
        for (k, v) in vars {
            self.env(k, v);
        }
        self
    }

    /// Removes an explicitly set environment variable and prevents inheriting it from a parent
    /// process.
    ///
    /// This method will remove the explicit value of an environment variable set via
    /// [`Command::env`] or [`Command::envs`]. In addition, it will prevent the spawned child
    /// process from inheriting that environment variable from its parent process.
    ///
    /// After calling [`Command::env_remove`], the value associated with its key from
    /// [`Command::get_envs`] will be [`None`].
    ///
    /// To clear all explicitly set environment variables and disable all environment variable
    /// inheritance, you can use [`Command::env_clear`].
    pub fn env_remove<S: AsRef<OsStr>>(&mut self, key: S) -> &mut Self {
        self.envs.remove(key.as_ref());
        self.removed_envs.push(key.as_ref().to_os_string());
        self
    }

    /// Clears all explicitly set environment variables and prevents inheriting any parent process
    /// environment variables.
    ///
    /// This method will remove all explicitly added environment variables set via [`Command::env`]
    /// or [`Command::envs`]. In addition, it will prevent the spawned child process from inheriting
    /// any environment variable from its parent process.
    ///
    /// After calling [`Command::env_clear`], the iterator from [`Command::get_envs`] will be
    /// empty.
    ///
    /// You can use [`Command::env_remove`] to clear a single mapping.
    pub fn env_clear(&mut self) -> &mut Self {
        self.cleared_env = true;
        self.envs.clear();
        self
    }

    /// Sets the working directory for the child process.
    ///
    /// # Platform-specific behavior
    ///
    /// If the program path is relative (e.g., `"./script.sh"`), it's ambiguous
    /// whether it should be interpreted relative to the parent's working
    /// directory or relative to `current_dir`. The behavior in this case is
    /// platform specific and unstable, and it's recommended to use
    /// [`canonicalize`] to get an absolute program path instead.
    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.working_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Configuration for the child process's standard input (stdin) handle.
    ///
    /// Defaults to [`inherit`] when used with [`spawn`] or [`status`], and
    /// defaults to [`piped`] when used with [`output`].
    ///
    /// [`inherit`]: Stdio::inherit
    /// [`piped`]: Stdio::piped
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.stdin = Some(cfg.into());
        self
    }

    /// Configuration for the child process's standard output (stdout) handle.
    ///
    /// Defaults to [`inherit`] when used with [`spawn`] or [`status`], and
    /// defaults to [`piped`] when used with [`output`].
    ///
    /// [`inherit`]: Stdio::inherit
    /// [`piped`]: Stdio::piped
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.stdout = Some(cfg.into());
        self
    }

    /// Configuration for the child process's standard error (stderr) handle.
    ///
    /// Defaults to [`inherit`] when used with [`spawn`] or [`status`], and
    /// defaults to [`piped`] when used with [`output`].
    ///
    /// [`inherit`]: Stdio::inherit
    /// [`piped`]: Stdio::piped
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.stderr = Some(cfg.into());
        self
    }

    /// Executes the command as a child process using std::process::Command,
    /// returning a handle to it.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    pub fn spawn(&mut self) -> Result<std::process::Child> {
        self.make_command().spawn()
    }

    /// Executes the command as a child process using std::process::Command,
    /// waiting for it to finish and collecting all of its output.
    ///
    /// By default, stdout and stderr are captured (and used to provide the
    /// resulting output). Stdin is not inherited from the parent and any
    /// attempt by the child process to read from the stdin stream will result
    /// in the stream immediately closing.
    pub fn output(&mut self) -> Result<std::process::Output> {
        self.make_command().output()
    }

    /// Executes a command as a child process using std::process::Command,
    /// waiting for it to finish and collecting its status.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    pub fn status(&mut self) -> Result<std::process::ExitStatus> {
        self.make_command().status()
    }

    /// Executes the command as a child process using smol::process::Command,
    /// returning a handle to it.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    pub fn spawn_async(&mut self, kill_on_drop: bool) -> Result<smol::process::Child> {
        self.make_async_command(kill_on_drop).spawn()
    }

    /// Executes the command as a child process using smol::process::Command,
    /// waiting for it to finish and collecting all of its output.
    ///
    /// By default, stdout and stderr are captured (and used to provide the
    /// resulting output). Stdin is not inherited from the parent and any
    /// attempt by the child process to read from the stdin stream will result
    /// in the stream immediately closing.
    pub fn output_async(
        &mut self,
        kill_on_drop: bool,
    ) -> impl std::future::Future<Output = Result<std::process::Output>> {
        self.make_async_command(kill_on_drop).output()
    }

    /// Executes a command as a child process using smol::process::Command,
    /// waiting for it to finish and collecting its status.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    pub fn status_async(
        &mut self,
        kill_on_drop: bool,
    ) -> impl std::future::Future<Output = Result<std::process::ExitStatus>> {
        self.make_async_command(kill_on_drop).status()
    }

    /// Returns the path to the program that was given to [`Command::new`].
    pub fn get_program(&self) -> &PathBuf {
        &self.program
    }

    /// Returns an iterator of the arguments that will be passed to the program.
    ///
    /// This does not include the path to the program as the first argument;
    /// it only includes the arguments specified with [`Command::arg`] and
    /// [`Command::args`].
    pub fn get_args(&self) -> Vec<OsString> {
        self.args
            .iter()
            .map(|(arg, _)| arg.clone())
            .collect::<Vec<_>>()
    }

    /// Returns a hashmap of the environment variables explicitly set for the
    /// child process.
    ///
    /// Environment variables explicitly set using [`Command::env`] and
    /// [`Command::envs`]can be retrieved with this method.
    ///
    /// Note that this output does not include environment variables inherited from the parent
    /// process.
    pub fn get_envs(&self) -> &HashMap<OsString, OsString> {
        &self.envs
    }

    /// ## Windows-Only
    /// Sets the [process creation flags][1] to be passed to `CreateProcess`.
    ///
    /// These will always be ORed with `CREATE_UNICODE_ENVIRONMENT`.
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
    #[cfg(windows)]
    pub fn windows_creation_flags(&mut self, flags: u32) -> &mut Self {
        self.windows_creation_flags |= flags;
        self
    }

    #[cfg(windows)]
    /// ## Windows-Only
    /// Append literal text to the command line without any quoting or escaping.
    ///
    /// This is useful for passing arguments to applications which doesn't follow
    /// the standard C run-time escaping rules, such as `cmd.exe /c`.
    pub fn windows_raw_arg<S: AsRef<OsStr>>(&mut self, raw_text: S) -> &mut Self {
        self.args.push((raw_text.as_ref().to_os_string(), true));
        self
    }

    /// Gets the actual program passed to std::process::Command or
    /// smol::process::Command. When compiled for flatpak, this is
    /// `flatpak-spawn`, and is `self.get_program()` otherwise.
    pub fn get_actual_program(&self) -> PathBuf {
        #[cfg(feature = "flatpak")]
        return <PathBuf as std::str::FromStr>::from_str("flatpak-spawn").unwrap();
        #[cfg(not(feature = "flatpak"))]
        return self.program.clone();
    }

    /// Gets the actual arguments passed to std::process::Command or
    /// smol::process::Command. When compiled for flatpak, this includes
    /// arguments meant for flatpak-spawn as well as the target program.
    /// Outside of flatpak this is the same as `self.get_args()`.
    pub fn get_passed_args(&mut self) -> Vec<String> {
        let command = self.make_command_common();
        command
            .get_args()
            .map(|arg| arg.to_str().unwrap().to_string())
            .collect()
    }

    fn make_command_common(&mut self) -> std::process::Command {
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
            let mut command = std::process::Command::new("flatpak-spawn");
            command.arg("--host");
            if let Some(working_directory) = &self.working_dir {
                command.arg(format!(
                    "--directory={}",
                    working_directory.to_str().unwrap()
                ));
            }
            for (k, v) in &env {
                command.arg(format!(
                    "--env={}={}",
                    k.as_os_str().to_str().unwrap(),
                    v.as_os_str().to_str().unwrap()
                ));
            }
            command.arg(self.program.as_os_str());
            command
        };

        command.env_clear().envs(env);
        if let Some(working_directory) = &self.working_dir {
            command.current_dir(working_directory);
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
        let mut command = self.make_command_common();
        if self.stdin.is_some() {
            command.stdin(self.stdin.take().unwrap());
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
        let mut command = smol::process::Command::from(self.make_command_common());
        if self.stdin.is_some() {
            command.stdin(self.stdin.take().unwrap());
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
