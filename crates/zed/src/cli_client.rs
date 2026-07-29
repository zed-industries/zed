//! The terminal client mode of the `zed` binary, previously shipped as the
//! separate `cli` binary.
//!
//! In this mode the process stays resident in the terminal: it hands an IPC
//! url over to a (possibly freshly launched, detached) Zed instance, then
//! relays stdout/stderr/exit-status back until the request is finished.

#![allow(
    clippy::disallowed_methods,
    reason = "We are not in an async environment, so std::process::Command is fine"
)]
#![cfg_attr(
    any(target_os = "linux", target_os = "freebsd", target_os = "windows"),
    allow(dead_code)
)]

use anyhow::{Context as _, Result};
use cli::{CliRequest, CliResponse, IpcHandshake, ipc::IpcOneShotServer};
use parking_lot::Mutex;
use release_channel::ReleaseChannel;
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs, io,
    io::IsTerminal,
    path::{Path, PathBuf},
    process::ExitStatus,
    sync::Arc,
    thread::{self, JoinHandle},
};
use tempfile::{NamedTempFile, TempDir};
use util::paths::PathWithPosition;
use walkdir::WalkDir;

use crate::Args;

const URL_PREFIX: [&'static str; 5] = ["zed://", "http://", "https://", "file://", "ssh://"];

struct Detect;

trait InstalledApp {
    fn zed_version_string(&self) -> String;
    fn launch(&self, ipc_url: String, user_data_dir: Option<&str>) -> anyhow::Result<()>;
    fn run_foreground(
        &self,
        ipc_url: String,
        user_data_dir: Option<&str>,
    ) -> io::Result<ExitStatus>;
}

/// Decides whether this invocation of the `zed` binary should act as the
/// terminal client instead of becoming the GUI editor.
///
/// Any explicit CLI argument selects client mode. For bare invocations the
/// decision is platform-specific.
pub(crate) fn should_run_as_cli_client(args: &Args) -> bool {
    if args
        .paths_or_urls
        .iter()
        .any(|path| path.starts_with("zed-cli://"))
    {
        return false;
    }
    #[cfg(target_os = "windows")]
    if args.dock_action.is_some() {
        return false;
    }

    #[allow(unused_mut)]
    let mut has_cli_flags = args.wait
        || args.add
        || args.new
        || args.reuse
        || args.existing
        || args.classic
        || args.foreground
        || args.dev_container
        || args.version
        || args.zed.is_some()
        || args.dev_server_token.is_some()
        || !args.diff.is_empty();
    #[cfg(target_os = "windows")]
    {
        has_cli_flags |= args.wsl.is_some();
    }
    if has_cli_flags {
        return true;
    }

    // Development builds keep the `cargo run [paths]` workflow: the GUI runs
    // attached to the terminal unless client behavior is explicitly requested
    // via the flags above.
    if *zed_env_vars::ZED_STATELESS || *release_channel::RELEASE_CHANNEL == ReleaseChannel::Dev {
        return false;
    }

    if !args.paths_or_urls.is_empty() {
        return true;
    }

    #[cfg(target_os = "macos")]
    {
        // LaunchServices-launched apps are children of launchd (pid 1), while
        // terminal invocations are children of the shell. A terminal-spawned
        // GUI would die with the terminal and get its TCC permissions
        // attributed to the terminal app, so relaunch detached instead.
        std::os::unix::process::parent_id() != 1
    }
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        true
    }
    #[cfg(target_os = "windows")]
    {
        // zed.exe is a windows-subsystem binary: it detaches from the console
        // automatically and interactive shells don't wait for it, so a bare
        // launch can become the GUI directly.
        false
    }
}

pub(crate) fn run(args: Args) -> Result<()> {
    #[cfg(target_os = "linux")]
    let args = flatpak::set_bin_if_no_escape(args);

    let app = Detect::detect(args.zed.as_deref()).context("Bundle detection")?;

    if args.version {
        println!("{}", app.zed_version_string());
        return Ok(());
    }

    let user_data_dir = args.user_data_dir.clone();

    let (server, server_name) =
        IpcOneShotServer::<IpcHandshake>::new().context("Handshake before Zed spawn")?;
    let url = format!("zed-cli://{server_name}");

    let open_behavior = if args.new {
        cli::OpenBehavior::AlwaysNew
    } else if args.add {
        cli::OpenBehavior::Add
    } else if args.existing {
        cli::OpenBehavior::ExistingWindow
    } else if args.classic {
        cli::OpenBehavior::Classic
    } else if args.reuse {
        cli::OpenBehavior::Reuse
    } else {
        cli::OpenBehavior::Default
    };

    let env = {
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            use collections::HashMap;

            // On Linux, the desktop entry uses `cli` to spawn `zed`.
            // We need to handle env vars correctly since std::env::vars() may not contain
            // project-specific vars (e.g. those set by direnv).
            // By setting env to None here, the LSP will use worktree env vars instead,
            // which is what we want.
            if !std::io::stdout().is_terminal() {
                None
            } else {
                Some(std::env::vars().collect::<HashMap<_, _>>())
            }
        }

        #[cfg(target_os = "windows")]
        {
            // On Windows, by default, a child process inherits a copy of the environment block of the parent process.
            // So we don't need to pass env vars explicitly.
            None
        }

        #[cfg(not(any(target_os = "linux", target_os = "freebsd", target_os = "windows")))]
        {
            use collections::HashMap;

            Some(std::env::vars().collect::<HashMap<_, _>>())
        }
    };

    let exit_status = Arc::new(Mutex::new(None));
    let mut paths = vec![];
    let mut urls = vec![];
    let mut diff_paths = vec![];
    let mut stdin_tmp_file: Option<fs::File> = None;
    let mut anonymous_fd_tmp_files = vec![];

    // Check if any diff paths are directories to determine diff_all mode
    let diff_all_mode = args
        .diff
        .chunks(2)
        .any(|pair| Path::new(&pair[0]).is_dir() || Path::new(&pair[1]).is_dir());

    for path in args.diff.chunks(2) {
        let left = parse_path_with_position(&path[0])?;
        let right = parse_path_with_position(&path[1])?;
        for diff_path in [&left, &right] {
            anyhow::ensure!(
                diff_path_exists(diff_path),
                "--diff path does not exist: {diff_path}"
            );
        }
        diff_paths.push([left, right]);
    }

    let (expanded_diff_paths, temp_dirs) = expand_directory_diff_pairs(diff_paths)?;
    diff_paths = expanded_diff_paths;
    // Prevent automatic cleanup of temp directories containing empty stub files
    // for directory diffs. The CLI process may exit before Zed has read these
    // files (e.g., when RPC-ing into an already-running instance). The files
    // live in the OS temp directory and will be cleaned up on reboot.
    for temp_dir in temp_dirs {
        let _ = temp_dir.keep();
    }

    #[cfg(target_os = "windows")]
    let wsl = args.wsl.as_ref();
    #[cfg(not(target_os = "windows"))]
    let wsl = None::<&String>;

    for path in args.paths_or_urls.iter() {
        if URL_PREFIX.iter().any(|&prefix| path.starts_with(prefix)) {
            urls.push(path.to_string());
        } else if path == "-" && args.paths_or_urls.len() == 1 {
            let file = NamedTempFile::new()?;
            paths.push(file.path().to_string_lossy().into_owned());
            let (file, _) = file.keep()?;
            stdin_tmp_file = Some(file);
        } else if let Some(file) = anonymous_fd(path) {
            let tmp_file = NamedTempFile::new()?;
            paths.push(tmp_file.path().to_string_lossy().into_owned());
            let (tmp_file, _) = tmp_file.keep()?;
            anonymous_fd_tmp_files.push((file, tmp_file));
        } else if let Some(wsl) = wsl {
            urls.push(format!("file://{}", parse_path_in_wsl(path, wsl)?));
        } else {
            paths.push(parse_path_with_position(path)?);
        }
    }

    anyhow::ensure!(
        args.dev_server_token.is_none(),
        "Dev servers were removed in v0.157.x please upgrade to SSH remoting: https://zed.dev/docs/remote-development"
    );

    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .stack_size(10 * 1024 * 1024)
        .thread_name(|ix| format!("RayonWorker{}", ix))
        .build_global()
        .unwrap();

    let sender: JoinHandle<anyhow::Result<()>> = thread::Builder::new()
        .name("CliReceiver".to_string())
        .spawn({
            let exit_status = exit_status.clone();
            let user_data_dir_for_thread = user_data_dir.clone();
            move || {
                let (_, handshake) = server.accept().context("Handshake after Zed spawn")?;
                let (tx, rx) = (handshake.requests, handshake.responses);

                #[cfg(target_os = "windows")]
                let wsl = args.wsl;
                #[cfg(not(target_os = "windows"))]
                let wsl = None;

                let open_request = CliRequest::Open {
                    paths,
                    urls,
                    diff_paths,
                    diff_all: diff_all_mode,
                    wsl,
                    wait: args.wait,
                    open_behavior,
                    env,
                    user_data_dir: user_data_dir_for_thread,
                    dev_container: args.dev_container,
                    cwd: env::current_dir().ok(),
                };

                tx.send(open_request)?;

                while let Ok(response) = rx.recv() {
                    match response {
                        CliResponse::Ping => {}
                        CliResponse::Stdout { message } => println!("{message}"),
                        CliResponse::Stderr { message } => eprintln!("{message}"),
                        CliResponse::Exit { status } => {
                            exit_status.lock().replace(status);
                            return Ok(());
                        }
                        CliResponse::PromptOpenBehavior => {
                            let behavior = prompt_open_behavior()
                                .unwrap_or(cli::CliBehaviorSetting::ExistingWindow);
                            tx.send(CliRequest::SetOpenBehavior { behavior })?;
                        }
                    }
                }

                Ok(())
            }
        })
        .unwrap();

    let stdin_pipe_handle: Option<JoinHandle<anyhow::Result<()>>> =
        stdin_tmp_file.map(|mut tmp_file| {
            thread::Builder::new()
                .name("CliStdin".to_string())
                .spawn(move || {
                    let mut stdin = std::io::stdin().lock();
                    if !io::IsTerminal::is_terminal(&stdin) {
                        io::copy(&mut stdin, &mut tmp_file)?;
                    }
                    Ok(())
                })
                .unwrap()
        });

    let anonymous_fd_pipe_handles: Vec<_> = anonymous_fd_tmp_files
        .into_iter()
        .map(|(mut file, mut tmp_file)| {
            thread::Builder::new()
                .name("CliAnonymousFd".to_string())
                .spawn(move || io::copy(&mut file, &mut tmp_file))
                .unwrap()
        })
        .collect();

    if args.foreground {
        app.run_foreground(url, user_data_dir.as_deref())?;
    } else {
        app.launch(url, user_data_dir.as_deref())?;
        sender.join().unwrap()?;
        if let Some(handle) = stdin_pipe_handle {
            handle.join().unwrap()?;
        }
        for handle in anonymous_fd_pipe_handles {
            handle.join().unwrap()?;
        }
    }

    if let Some(exit_status) = exit_status.lock().take() {
        std::process::exit(exit_status);
    }
    Ok(())
}

/// Parses a path containing a position (e.g. `path:line:column`)
/// and returns its canonicalized string representation.
///
/// If a part of path doesn't exist, it will canonicalize the
/// existing part and append the non-existing part.
///
/// This method must return an absolute path, as many zed
/// crates assume absolute paths.
fn parse_path_with_position(argument_str: &str) -> anyhow::Result<String> {
    match Path::new(argument_str).canonicalize() {
        Ok(existing_path) => Ok(PathWithPosition::from_path(existing_path)),
        Err(_) => PathWithPosition::parse_str(argument_str).map_path(|mut path| {
            let curdir = env::current_dir().context("retrieving current directory")?;
            let mut children = Vec::new();
            let root;
            loop {
                // canonicalize handles './', and '/'.
                if let Ok(canonicalized) = fs::canonicalize(&path) {
                    root = canonicalized;
                    break;
                }
                // The comparison to `curdir` is just a shortcut
                // since we know it is canonical. The other one
                // is if `argument_str` is a string that starts
                // with a name (e.g. "foo/bar").
                if path == curdir || path == Path::new("") {
                    root = curdir;
                    break;
                }
                children.push(
                    path.file_name()
                        .with_context(|| format!("parsing as path with position {argument_str}"))?
                        .to_owned(),
                );
                if !path.pop() {
                    unreachable!("parsing as path with position {argument_str}");
                }
            }
            Ok(children.iter().rev().fold(root, |mut path, child| {
                path.push(child);
                path
            }))
        }),
    }
    .map(|path_with_pos| path_with_pos.to_string(&|path| path.to_string_lossy().into_owned()))
}

/// Returns whether a `--diff` argument refers to an existing path, allowing a
/// trailing `:line:column` suffix (parsed later by the Zed side, matching how
/// regular `zed path:line:column` arguments are handled).
fn diff_path_exists(diff_path: &str) -> bool {
    Path::new(diff_path).exists() || PathWithPosition::parse_str(diff_path).path.exists()
}

fn expand_directory_diff_pairs(
    diff_pairs: Vec<[String; 2]>,
) -> anyhow::Result<(Vec<[String; 2]>, Vec<TempDir>)> {
    let mut expanded = Vec::new();
    let mut temp_dirs = Vec::new();

    for pair in diff_pairs {
        let left = PathBuf::from(&pair[0]);
        let right = PathBuf::from(&pair[1]);

        if left.is_dir() && right.is_dir() {
            let (mut pairs, temp_dir) = expand_directory_pair(&left, &right)?;
            expanded.append(&mut pairs);
            if let Some(temp_dir) = temp_dir {
                temp_dirs.push(temp_dir);
            }
        } else {
            expanded.push(pair);
        }
    }

    Ok((expanded, temp_dirs))
}

fn expand_directory_pair(
    left: &Path,
    right: &Path,
) -> anyhow::Result<(Vec<[String; 2]>, Option<TempDir>)> {
    let left_files = collect_files(left)?;
    let right_files = collect_files(right)?;

    let mut rel_paths = BTreeSet::new();
    rel_paths.extend(left_files.keys().cloned());
    rel_paths.extend(right_files.keys().cloned());

    let mut temp_dir = TempDir::new()?;
    let mut temp_dir_used = false;
    let mut pairs = Vec::new();

    for rel in rel_paths {
        match (left_files.get(&rel), right_files.get(&rel)) {
            (Some(left_path), Some(right_path)) => {
                pairs.push([
                    left_path.to_string_lossy().into_owned(),
                    right_path.to_string_lossy().into_owned(),
                ]);
            }
            (Some(left_path), None) => {
                let stub = create_empty_stub(&mut temp_dir, &rel)?;
                temp_dir_used = true;
                pairs.push([
                    left_path.to_string_lossy().into_owned(),
                    stub.to_string_lossy().into_owned(),
                ]);
            }
            (None, Some(right_path)) => {
                let stub = create_empty_stub(&mut temp_dir, &rel)?;
                temp_dir_used = true;
                pairs.push([
                    stub.to_string_lossy().into_owned(),
                    right_path.to_string_lossy().into_owned(),
                ]);
            }
            (None, None) => {}
        }
    }

    let temp_dir = if temp_dir_used { Some(temp_dir) } else { None };
    Ok((pairs, temp_dir))
}

fn collect_files(root: &Path) -> anyhow::Result<BTreeMap<PathBuf, PathBuf>> {
    let mut files = BTreeMap::new();

    for entry in WalkDir::new(root) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let rel = entry
                .path()
                .strip_prefix(root)
                .context("stripping directory prefix")?
                .to_path_buf();
            files.insert(rel, entry.into_path());
        }
    }

    Ok(files)
}

fn create_empty_stub(temp_dir: &mut TempDir, rel: &Path) -> anyhow::Result<PathBuf> {
    let stub_path = temp_dir.path().join(rel);
    if let Some(parent) = stub_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::File::create(&stub_path)?;
    Ok(stub_path)
}

#[cfg(target_os = "windows")]
fn parse_path_in_wsl(source: &str, wsl: &str) -> Result<String> {
    use std::ffi::OsStr;

    let mut source = PathWithPosition::parse_str(source);

    let (user, distro_name) = if let Some((user, distro)) = wsl.split_once('@') {
        if user.is_empty() {
            anyhow::bail!("user is empty in wsl argument");
        }
        (Some(user), distro)
    } else {
        (None, wsl)
    };

    let mut args = vec!["--distribution", distro_name];
    if let Some(user) = user {
        args.push("--user");
        args.push(user);
    }

    let command = [
        OsStr::new("realpath"),
        OsStr::new("-s"),
        source.path.as_ref(),
    ];

    let output = util::command::new_std_command("wsl.exe")
        .args(&args)
        .arg("--exec")
        .args(&command)
        .output()?;
    let result = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        let fallback = util::command::new_std_command("wsl.exe")
            .args(&args)
            .arg("--")
            .args(&command)
            .output()?;
        String::from_utf8_lossy(&fallback.stdout).to_string()
    };

    source.path = Path::new(result.trim()).to_owned();

    Ok(source.to_string(&|path| path.to_string_lossy().into_owned()))
}

#[cfg(not(target_os = "windows"))]
fn parse_path_in_wsl(_source: &str, _wsl: &str) -> Result<String> {
    anyhow::bail!("--wsl is only supported on Windows")
}

fn anonymous_fd(path: &str) -> Option<fs::File> {
    #[cfg(target_os = "linux")]
    {
        use std::os::fd::{self, FromRawFd};

        let fd_str = path.strip_prefix("/proc/self/fd/")?;

        let link = fs::read_link(path).ok()?;
        if !link.starts_with("memfd:") {
            return None;
        }

        let fd: fd::RawFd = fd_str.parse().ok()?;
        let file = unsafe { fs::File::from_raw_fd(fd) };
        Some(file)
    }
    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
    {
        use std::os::{
            fd::{self, FromRawFd},
            unix::fs::FileTypeExt,
        };

        let fd_str = path.strip_prefix("/dev/fd/")?;

        let metadata = fs::metadata(path).ok()?;
        let file_type = metadata.file_type();
        if !file_type.is_fifo() && !file_type.is_socket() {
            return None;
        }
        let fd: fd::RawFd = fd_str.parse().ok()?;
        let file = unsafe { fs::File::from_raw_fd(fd) };
        Some(file)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
    {
        _ = path;
        // not implemented for bsd, windows. Could be, but isn't yet
        None
    }
}

/// Shows an interactive prompt asking the user to choose the default open
/// behavior for `zed <path>`. Returns `None` if the prompt cannot be shown
/// (e.g. stdin is not a terminal) or the user cancels.
fn prompt_open_behavior() -> Option<cli::CliBehaviorSetting> {
    if !std::io::stdin().is_terminal() {
        return None;
    }

    let blue = console::Style::new().blue();
    let items = [
        format!(
            "Add to existing Zed window ({})",
            blue.apply_to("zed --existing")
        ),
        format!("Open a new window ({})", blue.apply_to("zed --classic")),
    ];

    let prompt = format!(
        "Configure default behavior for {}\n{}",
        blue.apply_to("zed <path>"),
        console::style("You can change this later in Zed settings"),
    );

    let selection = dialoguer::Select::new()
        .with_prompt(&prompt)
        .items(&items)
        .default(0)
        .interact()
        .ok()?;

    Some(if selection == 0 {
        cli::CliBehaviorSetting::ExistingWindow
    } else {
        cli::CliBehaviorSetting::NewWindow
    })
}

pub(crate) mod completions {
    mod shells {
        pub use clap_complete::aot::{Bash, Elvish, Fish, PowerShell, Zsh};
        pub use clap_complete_nushell::Nushell;
    }

    use clap_complete::Generator;

    #[derive(Clone, Debug, clap::ValueEnum)]
    #[non_exhaustive]
    #[value(rename_all = "lower")]
    pub(crate) enum Shell {
        Bash,
        Elvish,
        Fish,
        Nushell,
        PowerShell,
        Zsh,
    }

    impl Generator for Shell {
        fn file_name(&self, name: &str) -> String {
            match self {
                Shell::Bash => self::shells::Bash.file_name(name),
                Shell::Elvish => self::shells::Elvish.file_name(name),
                Shell::Fish => self::shells::Fish.file_name(name),
                Shell::Nushell => self::shells::Nushell.file_name(name),
                Shell::PowerShell => self::shells::PowerShell.file_name(name),
                Shell::Zsh => self::shells::Zsh.file_name(name),
            }
        }

        fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::Write) {
            match self {
                Shell::Bash => self::shells::Bash.generate(cmd, buf),
                Shell::Elvish => self::shells::Elvish.generate(cmd, buf),
                Shell::Fish => self::shells::Fish.generate(cmd, buf),
                Shell::Nushell => self::shells::Nushell.generate(cmd, buf),
                Shell::PowerShell => self::shells::PowerShell.generate(cmd, buf),
                Shell::Zsh => self::shells::Zsh.generate(cmd, buf),
            }
        }
    }

    pub(crate) fn generate(cmd: &clap::Command, shell: &Shell) {
        let buf = &mut std::io::stdout();
        shell.generate(cmd, buf);
    }
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux {
    use std::{
        env,
        ffi::OsString,
        io,
        path::{Path, PathBuf},
        process::{self, ExitStatus},
    };

    use anyhow::anyhow;
    use cli::FORCE_CLI_MODE_ENV_VAR_NAME;
    use fork::Fork;

    use super::{Detect, InstalledApp};

    struct App(PathBuf);

    impl Detect {
        pub fn detect(path: Option<&Path>) -> anyhow::Result<impl InstalledApp> {
            let path = if let Some(path) = path {
                path.to_path_buf().canonicalize()?
            } else {
                env::current_exe()?.canonicalize()?
            };

            Ok(App(path))
        }
    }

    impl InstalledApp for App {
        fn zed_version_string(&self) -> String {
            format!(
                "Zed {}{}{} – {}",
                if *release_channel::RELEASE_CHANNEL_NAME == "stable" {
                    "".to_string()
                } else {
                    format!("{} ", *release_channel::RELEASE_CHANNEL_NAME)
                },
                option_env!("RELEASE_VERSION").unwrap_or_default(),
                match option_env!("ZED_COMMIT_SHA") {
                    Some(commit_sha) => format!(" {commit_sha} "),
                    None => "".to_string(),
                },
                self.0.display(),
            )
        }

        fn launch(&self, ipc_url: String, user_data_dir: Option<&str>) -> anyhow::Result<()> {
            let data_dir = user_data_dir
                .map(PathBuf::from)
                .unwrap_or_else(|| paths::data_dir().clone());

            let sock_path = data_dir.join(format!(
                "zed-{}.sock",
                *release_channel::RELEASE_CHANNEL_NAME
            ));
            let sock = std::os::unix::net::UnixDatagram::unbound()?;
            if sock.connect(&sock_path).is_err() {
                self.boot_background(ipc_url, user_data_dir)?;
            } else {
                sock.send(ipc_url.as_bytes())?;
            }
            Ok(())
        }

        fn run_foreground(
            &self,
            ipc_url: String,
            user_data_dir: Option<&str>,
        ) -> io::Result<ExitStatus> {
            let mut cmd = std::process::Command::new(self.0.clone());
            cmd.arg(ipc_url);
            if let Some(dir) = user_data_dir {
                cmd.arg("--user-data-dir").arg(dir);
            }
            cmd.status()
        }
    }

    impl App {
        fn boot_background(
            &self,
            ipc_url: String,
            user_data_dir: Option<&str>,
        ) -> anyhow::Result<()> {
            let path = &self.0;

            match fork::fork() {
                Ok(Fork::Parent(_)) => Ok(()),
                Ok(Fork::Child) => {
                    unsafe { std::env::set_var(FORCE_CLI_MODE_ENV_VAR_NAME, "") };
                    if fork::setsid().is_err() {
                        eprintln!("failed to setsid: {}", std::io::Error::last_os_error());
                        process::exit(1);
                    }
                    if fork::close_fd().is_err() {
                        eprintln!("failed to close_fd: {}", std::io::Error::last_os_error());
                    }
                    let mut args: Vec<OsString> =
                        vec![path.as_os_str().to_owned(), OsString::from(ipc_url)];
                    if let Some(dir) = user_data_dir {
                        args.push(OsString::from("--user-data-dir"));
                        args.push(OsString::from(dir));
                    }
                    let error = exec::execvp(path.clone(), &args);
                    // if exec succeeded, we never get here.
                    eprintln!("failed to exec {:?}: {}", path, error);
                    process::exit(1)
                }
                Err(_) => Err(anyhow!(io::Error::last_os_error())),
            }
        }
    }
}

#[cfg(target_os = "linux")]
pub(crate) mod flatpak {
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::process::Command;
    use std::{env, process};

    const EXTRA_LIB_ENV_NAME: &str = "ZED_FLATPAK_LIB_PATH";
    const NO_ESCAPE_ENV_NAME: &str = "ZED_FLATPAK_NO_ESCAPE";

    /// Adds bundled libraries to LD_LIBRARY_PATH if running under flatpak
    pub fn ld_extra_libs() {
        let mut paths = if let Ok(paths) = env::var("LD_LIBRARY_PATH") {
            env::split_paths(&paths).collect()
        } else {
            Vec::new()
        };

        if let Ok(extra_path) = env::var(EXTRA_LIB_ENV_NAME) {
            paths.push(extra_path.into());
        }

        unsafe { env::set_var("LD_LIBRARY_PATH", env::join_paths(paths).unwrap()) };
    }

    /// Restarts outside of the sandbox if currently running within it
    pub fn try_restart_to_host() {
        if let Some(flatpak_dir) = get_flatpak_dir() {
            let mut args = vec!["/usr/bin/flatpak-spawn".into(), "--host".into()];
            args.append(&mut get_xdg_env_args());
            args.push("--env=ZED_UPDATE_EXPLANATION=Please use flatpak to update zed".into());
            args.push(
                format!(
                    "--env={EXTRA_LIB_ENV_NAME}={}",
                    flatpak_dir.join("lib").to_str().unwrap()
                )
                .into(),
            );
            args.push(flatpak_dir.join("libexec").join("zed-editor").into());

            for arg in &env::args_os().collect::<Vec<_>>()[1..] {
                args.push(arg.clone());
            }

            let error = exec::execvp("/usr/bin/flatpak-spawn", args);
            eprintln!("failed restart zed on host: {:?}", error);
            process::exit(1);
        }
    }

    pub fn set_bin_if_no_escape(mut args: crate::Args) -> crate::Args {
        if env::var(NO_ESCAPE_ENV_NAME).is_ok()
            && env::var("FLATPAK_ID").is_ok_and(|id| id.starts_with("dev.zed.Zed"))
            && args.zed.is_none()
        {
            args.zed = Some("/app/libexec/zed-editor".into());
            unsafe { env::set_var("ZED_UPDATE_EXPLANATION", "Please use flatpak to update zed") };
        }
        args
    }

    fn get_flatpak_dir() -> Option<PathBuf> {
        if env::var(NO_ESCAPE_ENV_NAME).is_ok() {
            return None;
        }

        if let Ok(flatpak_id) = env::var("FLATPAK_ID") {
            if !flatpak_id.starts_with("dev.zed.Zed") {
                return None;
            }

            let install_dir = Command::new("/usr/bin/flatpak-spawn")
                .arg("--host")
                .arg("flatpak")
                .arg("info")
                .arg("--show-location")
                .arg(flatpak_id)
                .output()
                .unwrap();
            let install_dir = PathBuf::from(String::from_utf8(install_dir.stdout).unwrap().trim());
            Some(install_dir.join("files"))
        } else {
            None
        }
    }

    fn get_xdg_env_args() -> Vec<OsString> {
        let xdg_keys = [
            "XDG_DATA_HOME",
            "XDG_CONFIG_HOME",
            "XDG_CACHE_HOME",
            "XDG_STATE_HOME",
        ];
        env::vars()
            .filter(|(key, _)| xdg_keys.contains(&key.as_str()))
            .map(|(key, val)| format!("--env=FLATPAK_{}={}", key, val).into())
            .collect()
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use anyhow::Context;
    use release_channel::app_identifier;
    use windows::{
        Win32::{
            Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GENERIC_WRITE, GetLastError},
            Storage::FileSystem::{
                CreateFileW, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE, OPEN_EXISTING, WriteFile,
            },
            System::Threading::CreateMutexW,
        },
        core::HSTRING,
    };

    use super::{Detect, InstalledApp};
    use std::io;
    use std::path::{Path, PathBuf};
    use std::process::{ExitStatus, Stdio};

    fn check_single_instance() -> bool {
        let mutex = unsafe {
            CreateMutexW(
                None,
                false,
                &HSTRING::from(format!("{}-Instance-Mutex", app_identifier())),
            )
            .expect("Unable to create instance sync event")
        };
        let last_err = unsafe { GetLastError() };
        let _ = unsafe { CloseHandle(mutex) };
        last_err != ERROR_ALREADY_EXISTS
    }

    struct App(PathBuf);

    impl InstalledApp for App {
        fn zed_version_string(&self) -> String {
            format!(
                "Zed {}{}{} – {}",
                if *release_channel::RELEASE_CHANNEL_NAME == "stable" {
                    "".to_string()
                } else {
                    format!("{} ", *release_channel::RELEASE_CHANNEL_NAME)
                },
                option_env!("RELEASE_VERSION").unwrap_or_default(),
                match option_env!("ZED_COMMIT_SHA") {
                    Some(commit_sha) => format!(" {commit_sha} "),
                    None => "".to_string(),
                },
                self.0.display(),
            )
        }

        fn launch(&self, ipc_url: String, user_data_dir: Option<&str>) -> anyhow::Result<()> {
            if check_single_instance() {
                let mut cmd = std::process::Command::new(self.0.clone());
                cmd.arg(ipc_url);
                if let Some(dir) = user_data_dir {
                    cmd.arg("--user-data-dir").arg(dir);
                }
                cmd.stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null());
                cmd.spawn()?;
            } else {
                unsafe {
                    let pipe = CreateFileW(
                        &HSTRING::from(format!("\\\\.\\pipe\\{}-Named-Pipe", app_identifier())),
                        GENERIC_WRITE.0,
                        FILE_SHARE_MODE::default(),
                        None,
                        OPEN_EXISTING,
                        FILE_FLAGS_AND_ATTRIBUTES::default(),
                        None,
                    )?;
                    let message = ipc_url.as_bytes();
                    let mut bytes_written = 0;
                    WriteFile(pipe, Some(message), Some(&mut bytes_written), None)?;
                    CloseHandle(pipe)?;
                }
            }
            Ok(())
        }

        fn run_foreground(
            &self,
            ipc_url: String,
            user_data_dir: Option<&str>,
        ) -> io::Result<ExitStatus> {
            let mut cmd = std::process::Command::new(self.0.clone());
            cmd.arg(ipc_url).arg("--foreground");
            if let Some(dir) = user_data_dir {
                cmd.arg("--user-data-dir").arg(dir);
            }
            cmd.spawn()?.wait()
        }
    }

    impl Detect {
        pub fn detect(path: Option<&Path>) -> anyhow::Result<impl InstalledApp> {
            let path = if let Some(path) = path {
                path.to_path_buf().canonicalize()?
            } else {
                std::env::current_exe()?
                    .canonicalize()
                    .context("canonicalizing the zed binary path")?
            };

            Ok(App(path))
        }
    }
}

#[cfg(target_os = "macos")]
pub(crate) mod mac_os {
    use anyhow::{Context as _, Result};
    use core_foundation::{
        array::{CFArray, CFIndex},
        base::TCFType as _,
        string::kCFStringEncodingUTF8,
        url::{CFURL, CFURLCreateWithBytes},
    };
    use core_services::{
        LSLaunchURLSpec, LSOpenFromURLSpec, kLSLaunchDefaults, kLSLaunchDontSwitch,
    };
    use serde::Deserialize;
    use std::{
        ffi::OsStr,
        fs, io,
        path::{Path, PathBuf},
        process::{Command, ExitStatus},
        ptr,
    };

    use cli::FORCE_CLI_MODE_ENV_VAR_NAME;

    use super::{Detect, InstalledApp};

    #[derive(Debug, Deserialize)]
    struct InfoPlist {
        #[serde(rename = "CFBundleShortVersionString")]
        bundle_short_version_string: String,
    }

    enum Bundle {
        App {
            app_bundle: PathBuf,
            plist: InfoPlist,
        },
        LocalPath {
            executable: PathBuf,
        },
    }

    fn locate_bundle() -> Result<PathBuf> {
        let zed_path = std::env::current_exe()?.canonicalize()?;
        let mut app_path = zed_path.clone();
        while app_path.extension() != Some(OsStr::new("app")) {
            anyhow::ensure!(
                app_path.pop(),
                "cannot find app bundle containing {zed_path:?}"
            );
        }
        Ok(app_path)
    }

    impl Detect {
        pub fn detect(path: Option<&Path>) -> anyhow::Result<impl InstalledApp> {
            let bundle_path = if let Some(bundle_path) = path {
                bundle_path
                    .canonicalize()
                    .with_context(|| format!("Args bundle path {bundle_path:?} canonicalization"))?
            } else {
                match locate_bundle() {
                    Ok(bundle_path) => bundle_path,
                    // Development builds are not packaged into an *.app bundle.
                    Err(_) => std::env::current_exe()?.canonicalize()?,
                }
            };

            match bundle_path.extension().and_then(|ext| ext.to_str()) {
                Some("app") => {
                    let plist_path = bundle_path.join("Contents/Info.plist");
                    let plist =
                        plist::from_file::<_, InfoPlist>(&plist_path).with_context(|| {
                            format!("Reading *.app bundle plist file at {plist_path:?}")
                        })?;
                    Ok(Bundle::App {
                        app_bundle: bundle_path,
                        plist,
                    })
                }
                _ => Ok(Bundle::LocalPath {
                    executable: bundle_path,
                }),
            }
        }
    }

    impl InstalledApp for Bundle {
        fn zed_version_string(&self) -> String {
            format!("Zed {} – {}", self.version(), self.path().display(),)
        }

        fn launch(&self, url: String, user_data_dir: Option<&str>) -> anyhow::Result<()> {
            match self {
                Self::App { app_bundle, .. } => {
                    let app_path = app_bundle;

                    let status = unsafe {
                        let app_url = CFURL::from_path(app_path, true)
                            .with_context(|| format!("invalid app path {app_path:?}"))?;
                        let url_to_open = CFURL::wrap_under_create_rule(CFURLCreateWithBytes(
                            ptr::null(),
                            url.as_ptr(),
                            url.len() as CFIndex,
                            kCFStringEncodingUTF8,
                            ptr::null(),
                        ));
                        // equivalent to: open zed-cli:... -a /Applications/Zed\ Preview.app
                        let urls_to_open =
                            CFArray::from_copyable(&[url_to_open.as_concrete_TypeRef()]);
                        LSOpenFromURLSpec(
                            &LSLaunchURLSpec {
                                appURL: app_url.as_concrete_TypeRef(),
                                itemURLs: urls_to_open.as_concrete_TypeRef(),
                                passThruParams: ptr::null(),
                                launchFlags: kLSLaunchDefaults | kLSLaunchDontSwitch,
                                asyncRefCon: ptr::null_mut(),
                            },
                            ptr::null_mut(),
                        )
                    };

                    anyhow::ensure!(
                        status == 0,
                        "cannot start app bundle {}",
                        self.zed_version_string()
                    );
                }

                Self::LocalPath { executable, .. } => {
                    let executable_parent = executable
                        .parent()
                        .with_context(|| format!("Executable {executable:?} path has no parent"))?;
                    let subprocess_stdout_file = fs::File::create(
                        executable_parent.join("zed_dev.log"),
                    )
                    .with_context(|| format!("Log file creation in {executable_parent:?}"))?;
                    let subprocess_stdin_file =
                        subprocess_stdout_file.try_clone().with_context(|| {
                            format!("Cloning descriptor for file {subprocess_stdout_file:?}")
                        })?;
                    let mut command = std::process::Command::new(executable);
                    command.env(FORCE_CLI_MODE_ENV_VAR_NAME, "");
                    if let Some(dir) = user_data_dir {
                        command.arg("--user-data-dir").arg(dir);
                    }
                    command
                        .stderr(subprocess_stdout_file)
                        .stdout(subprocess_stdin_file)
                        .arg(url);

                    command
                        .spawn()
                        .with_context(|| format!("Spawning {command:?}"))?;
                }
            }

            Ok(())
        }

        fn run_foreground(
            &self,
            ipc_url: String,
            user_data_dir: Option<&str>,
        ) -> io::Result<ExitStatus> {
            let path = match self {
                Bundle::App { app_bundle, .. } => app_bundle.join("Contents/MacOS/zed"),
                Bundle::LocalPath { executable, .. } => executable.clone(),
            };

            let mut cmd = std::process::Command::new(path);
            cmd.arg(ipc_url);
            if let Some(dir) = user_data_dir {
                cmd.arg("--user-data-dir").arg(dir);
            }
            cmd.status()
        }
    }

    impl Bundle {
        fn version(&self) -> String {
            match self {
                Self::App { plist, .. } => plist.bundle_short_version_string.clone(),
                Self::LocalPath { .. } => "<development>".to_string(),
            }
        }

        fn path(&self) -> &Path {
            match self {
                Self::App { app_bundle, .. } => app_bundle,
                Self::LocalPath { executable, .. } => executable,
            }
        }
    }

    pub(crate) fn spawn_channel_cli(
        channel: release_channel::ReleaseChannel,
        leftover_args: Vec<String>,
    ) -> Result<()> {
        use anyhow::bail;

        let app_path_prompt = format!(
            "POSIX path of (path to application \"{}\")",
            channel.display_name()
        );
        let app_path_output = Command::new("osascript")
            .arg("-e")
            .arg(&app_path_prompt)
            .output()?;
        if !app_path_output.status.success() {
            bail!(
                "Could not determine app path for {}",
                channel.display_name()
            );
        }
        let app_path = String::from_utf8(app_path_output.stdout)?.trim().to_owned();
        let cli_path = format!("{app_path}/Contents/MacOS/cli");
        Command::new(cli_path).args(leftover_args).spawn()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use util::path;
    use util::paths::SanitizedPath;
    use util::test::TempTree;

    macro_rules! assert_path_eq {
        ($left:expr, $right:expr) => {
            assert_eq!(
                SanitizedPath::new(Path::new(&$left)),
                SanitizedPath::new(Path::new(&$right))
            )
        };
    }

    fn cwd() -> PathBuf {
        env::current_dir().unwrap()
    }

    static CWD_LOCK: Mutex<()> = Mutex::new(());

    fn with_cwd<T>(path: &Path, f: impl FnOnce() -> anyhow::Result<T>) -> anyhow::Result<T> {
        let _lock = CWD_LOCK.lock();
        let old_cwd = cwd();
        env::set_current_dir(path)?;
        let result = f();
        env::set_current_dir(old_cwd)?;
        result
    }

    #[test]
    fn test_parse_non_existing_path() {
        // Absolute path
        let result = parse_path_with_position(path!("/non/existing/path.txt")).unwrap();
        assert_path_eq!(result, path!("/non/existing/path.txt"));

        // Absolute path in cwd
        let path = cwd().join(path!("non/existing/path.txt"));
        let expected = path.to_string_lossy().to_string();
        let result = parse_path_with_position(&expected).unwrap();
        assert_path_eq!(result, expected);

        // Relative path
        let result = parse_path_with_position(path!("non/existing/path.txt")).unwrap();
        assert_path_eq!(result, expected)
    }

    #[test]
    fn test_parse_existing_path() {
        let temp_tree = TempTree::new(json!({
            "file.txt": "",
        }));
        let file_path = temp_tree.path().join("file.txt");
        let expected = file_path.to_string_lossy().to_string();

        // Absolute path
        let result = parse_path_with_position(file_path.to_str().unwrap()).unwrap();
        assert_path_eq!(result, expected);

        // Relative path
        let result = with_cwd(temp_tree.path(), || parse_path_with_position("file.txt")).unwrap();
        assert_path_eq!(result, expected);
    }

    // NOTE:
    // While POSIX symbolic links are somewhat supported on Windows, they are an opt in by the user, and thus
    // we assume that they are not supported out of the box.
    #[cfg(not(windows))]
    #[test]
    fn test_parse_symlink_file() {
        let temp_tree = TempTree::new(json!({
            "target.txt": "",
        }));
        let target_path = temp_tree.path().join("target.txt");
        let symlink_path = temp_tree.path().join("symlink.txt");
        std::os::unix::fs::symlink(&target_path, &symlink_path).unwrap();

        // Absolute path
        let result = parse_path_with_position(symlink_path.to_str().unwrap()).unwrap();
        assert_eq!(result, target_path.to_string_lossy());

        // Relative path
        let result =
            with_cwd(temp_tree.path(), || parse_path_with_position("symlink.txt")).unwrap();
        assert_eq!(result, target_path.to_string_lossy());
    }

    #[cfg(not(windows))]
    #[test]
    fn test_parse_symlink_dir() {
        let temp_tree = TempTree::new(json!({
            "some": {
                "dir": { // symlink target
                    "ec": {
                        "tory": {
                            "file.txt": "",
        }}}}}));

        let target_file_path = temp_tree.path().join("some/dir/ec/tory/file.txt");
        let expected = target_file_path.to_string_lossy();

        let dir_path = temp_tree.path().join("some/dir");
        let symlink_path = temp_tree.path().join("symlink");
        std::os::unix::fs::symlink(&dir_path, &symlink_path).unwrap();

        // Absolute path
        let result =
            parse_path_with_position(symlink_path.join("ec/tory/file.txt").to_str().unwrap())
                .unwrap();
        assert_eq!(result, expected);

        // Relative path
        let result = with_cwd(temp_tree.path(), || {
            parse_path_with_position("symlink/ec/tory/file.txt")
        })
        .unwrap();
        assert_eq!(result, expected);
    }
}
