#![cfg_attr(any(target_os = "linux", target_os = "windows"), allow(dead_code))]

use anyhow::{Context, Result};
use clap::Parser;
use cli::{ipc::IpcOneShotServer, CliRequest, CliResponse, IpcHandshake};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::ExitStatus,
    thread::{self, JoinHandle},
};
use util::paths::PathLikeWithPosition;

struct Detect;

trait InstalledApp {
    fn zed_version_string(&self) -> String;
    fn launch(&self, ipc_url: String) -> anyhow::Result<()>;
    fn run_foreground(&self, ipc_url: String) -> io::Result<ExitStatus>;
}

#[derive(Parser, Debug)]
#[command(name = "zed", disable_version_flag = true)]
struct Args {
    /// Wait for all of the given paths to be opened/closed before exiting.
    #[arg(short, long)]
    wait: bool,
    /// Add files to the currently open workspace
    #[arg(short, long, overrides_with = "new")]
    add: bool,
    /// Create a new workspace
    #[arg(short, long, overrides_with = "add")]
    new: bool,
    /// A sequence of space-separated paths that you want to open.
    ///
    /// Use `path:line:row` syntax to open a file at a specific location.
    /// Non-existing paths and directories will ignore `:line:row` suffix.
    #[arg(value_parser = parse_path_with_position)]
    paths_with_position: Vec<PathLikeWithPosition<PathBuf>>,
    /// Print Zed's version and the app path.
    #[arg(short, long)]
    version: bool,
    /// Run zed in the foreground (useful for debugging)
    #[arg(long)]
    foreground: bool,
    /// Custom path to Zed.app or the zed binary
    #[arg(long)]
    zed: Option<PathBuf>,
    /// Run zed in dev-server mode
    #[arg(long)]
    dev_server_token: Option<String>,
}

fn parse_path_with_position(
    argument_str: &str,
) -> Result<PathLikeWithPosition<PathBuf>, std::convert::Infallible> {
    PathLikeWithPosition::parse_str(argument_str, |path_str| {
        Ok(Path::new(path_str).to_path_buf())
    })
}

fn main() -> Result<()> {
    // Exit flatpak sandbox if needed
    #[cfg(target_os = "linux")]
    {
        flatpak::try_restart_to_host();
        flatpak::ld_extra_libs();
    }

    // Intercept version designators
    #[cfg(target_os = "macos")]
    if let Some(channel) = std::env::args().nth(1).filter(|arg| arg.starts_with("--")) {
        // When the first argument is a name of a release channel, we're gonna spawn off a cli of that version, with trailing args passed along.
        use std::str::FromStr as _;

        if let Ok(channel) = release_channel::ReleaseChannel::from_str(&channel[2..]) {
            return mac_os::spawn_channel_cli(channel, std::env::args().skip(2).collect());
        }
    }
    let args = Args::parse();

    #[cfg(target_os = "linux")]
    let args = flatpak::set_bin_if_no_escape(args);

    let app = Detect::detect(args.zed.as_deref()).context("Bundle detection")?;

    if args.version {
        println!("{}", app.zed_version_string());
        return Ok(());
    }

    let curdir = env::current_dir()?;
    let mut paths = vec![];
    for path in args.paths_with_position {
        let canonicalized = path.map_path_like(|path| match fs::canonicalize(&path) {
            Ok(path) => Ok(path),
            Err(e) => {
                if let Some(mut parent) = path.parent() {
                    if parent == Path::new("") {
                        parent = &curdir;
                    }
                    match fs::canonicalize(parent) {
                        Ok(parent) => Ok(parent.join(path.file_name().unwrap())),
                        Err(_) => Err(e),
                    }
                } else {
                    Err(e)
                }
            }
        })?;
        paths.push(canonicalized.to_string(|path| path.display().to_string()))
    }

    let (server, server_name) =
        IpcOneShotServer::<IpcHandshake>::new().context("Handshake before Zed spawn")?;
    let url = format!("zed-cli://{server_name}");

    let open_new_workspace = if args.new {
        Some(true)
    } else if args.add {
        Some(false)
    } else {
        None
    };

    let sender: JoinHandle<anyhow::Result<()>> = thread::spawn(move || {
        let (_, handshake) = server.accept().context("Handshake after Zed spawn")?;
        let (tx, rx) = (handshake.requests, handshake.responses);
        tx.send(CliRequest::Open {
            paths,
            wait: args.wait,
            open_new_workspace,
            dev_server_token: args.dev_server_token,
        })?;

        while let Ok(response) = rx.recv() {
            match response {
                CliResponse::Ping => {}
                CliResponse::Stdout { message } => println!("{message}"),
                CliResponse::Stderr { message } => eprintln!("{message}"),
                CliResponse::Exit { status } => std::process::exit(status),
            }
        }

        Ok(())
    });

    if args.foreground {
        app.run_foreground(url)?;
    } else {
        app.launch(url)?;
        sender.join().unwrap()?;
    }

    Ok(())
}

#[cfg(target_os = "linux")]
mod linux {
    use std::{
        env,
        ffi::OsString,
        io,
        os::{
            linux::net::SocketAddrExt,
            unix::net::{SocketAddr, UnixDatagram},
        },
        path::{Path, PathBuf},
        process::{self, ExitStatus},
        thread,
        time::Duration,
    };

    use anyhow::anyhow;
    use cli::FORCE_CLI_MODE_ENV_VAR_NAME;
    use fork::Fork;
    use once_cell::sync::Lazy;

    use crate::{Detect, InstalledApp};

    static RELEASE_CHANNEL: Lazy<String> =
        Lazy::new(|| include_str!("../../zed/RELEASE_CHANNEL").trim().to_string());

    struct App(PathBuf);

    impl Detect {
        pub fn detect(path: Option<&Path>) -> anyhow::Result<impl InstalledApp> {
            let path = if let Some(path) = path {
                path.to_path_buf().canonicalize()
            } else {
                let cli = env::current_exe()?;
                let dir = cli
                    .parent()
                    .ok_or_else(|| anyhow!("no parent path for cli"))?;

                match dir.join("zed").canonicalize() {
                    Ok(path) => Ok(path),
                    // development builds have Zed capitalized
                    Err(e) => match dir.join("Zed").canonicalize() {
                        Ok(path) => Ok(path),
                        Err(_) => Err(e),
                    },
                }
            }?;

            Ok(App(path))
        }
    }

    impl InstalledApp for App {
        fn zed_version_string(&self) -> String {
            format!(
                "Zed {}{} – {}",
                if *RELEASE_CHANNEL == "stable" {
                    "".to_string()
                } else {
                    format!(" {} ", *RELEASE_CHANNEL)
                },
                option_env!("RELEASE_VERSION").unwrap_or_default(),
                self.0.display(),
            )
        }

        fn launch(&self, ipc_url: String) -> anyhow::Result<()> {
            let uid: u32 = unsafe { libc::getuid() };
            let sock_addr =
                SocketAddr::from_abstract_name(format!("zed-{}-{}", *RELEASE_CHANNEL, uid))?;

            let sock = UnixDatagram::unbound()?;
            if sock.connect_addr(&sock_addr).is_err() {
                self.boot_background(ipc_url)?;
            } else {
                sock.send(ipc_url.as_bytes())?;
            }
            Ok(())
        }

        fn run_foreground(&self, ipc_url: String) -> io::Result<ExitStatus> {
            std::process::Command::new(self.0.clone())
                .arg(ipc_url)
                .status()
        }
    }

    impl App {
        fn boot_background(&self, ipc_url: String) -> anyhow::Result<()> {
            let path = &self.0;

            match fork::fork() {
                Ok(Fork::Parent(_)) => Ok(()),
                Ok(Fork::Child) => {
                    std::env::set_var(FORCE_CLI_MODE_ENV_VAR_NAME, "");
                    if let Err(_) = fork::setsid() {
                        eprintln!("failed to setsid: {}", std::io::Error::last_os_error());
                        process::exit(1);
                    }
                    if std::env::var("ZED_KEEP_FD").is_err() {
                        if let Err(_) = fork::close_fd() {
                            eprintln!("failed to close_fd: {}", std::io::Error::last_os_error());
                        }
                    }
                    let error =
                        exec::execvp(path.clone(), &[path.as_os_str(), &OsString::from(ipc_url)]);
                    // if exec succeeded, we never get here.
                    eprintln!("failed to exec {:?}: {}", path, error);
                    process::exit(1)
                }
                Err(_) => Err(anyhow!(io::Error::last_os_error())),
            }
        }

        fn wait_for_socket(
            &self,
            sock_addr: &SocketAddr,
            sock: &mut UnixDatagram,
        ) -> Result<(), std::io::Error> {
            for _ in 0..100 {
                thread::sleep(Duration::from_millis(10));
                if sock.connect_addr(&sock_addr).is_ok() {
                    return Ok(());
                }
            }
            sock.connect_addr(&sock_addr)
        }
    }
}

#[cfg(target_os = "linux")]
mod flatpak {
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::process::Command;
    use std::{env, process};

    const EXTRA_LIB_ENV_NAME: &'static str = "ZED_FLATPAK_LIB_PATH";
    const NO_ESCAPE_ENV_NAME: &'static str = "ZED_FLATPAK_NO_ESCAPE";

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

        env::set_var("LD_LIBRARY_PATH", env::join_paths(paths).unwrap());
    }

    /// Restarts outside of the sandbox if currently running within it
    pub fn try_restart_to_host() {
        if let Some(flatpak_dir) = get_flatpak_dir() {
            let mut args = vec!["/usr/bin/flatpak-spawn".into(), "--host".into()];
            args.append(&mut get_xdg_env_args());
            args.push("--env=ZED_IS_FLATPAK_INSTALL=1".into());
            args.push(
                format!(
                    "--env={EXTRA_LIB_ENV_NAME}={}",
                    flatpak_dir.join("lib").to_str().unwrap()
                )
                .into(),
            );
            args.push(flatpak_dir.join("bin").join("zed").into());

            let mut is_app_location_set = false;
            for arg in &env::args_os().collect::<Vec<_>>()[1..] {
                args.push(arg.clone());
                is_app_location_set |= arg == "--zed";
            }

            if !is_app_location_set {
                args.push("--zed".into());
                args.push(flatpak_dir.join("bin").join("zed-app").into());
            }

            let error = exec::execvp("/usr/bin/flatpak-spawn", args);
            eprintln!("failed restart cli on host: {:?}", error);
            process::exit(1);
        }
    }

    pub fn set_bin_if_no_escape(mut args: super::Args) -> super::Args {
        if env::var(NO_ESCAPE_ENV_NAME).is_ok()
            && env::var("FLATPAK_ID").map_or(false, |id| id.starts_with("dev.zed.Zed"))
        {
            if args.zed.is_none() {
                args.zed = Some("/app/bin/zed-app".into());
                env::set_var("ZED_IS_FLATPAK_INSTALL", "1");
            }
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

// todo("windows")
#[cfg(target_os = "windows")]
mod windows {
    use crate::{Detect, InstalledApp};
    use std::io;
    use std::path::Path;
    use std::process::ExitStatus;

    struct App;
    impl InstalledApp for App {
        fn zed_version_string(&self) -> String {
            unimplemented!()
        }
        fn launch(&self, _ipc_url: String) -> anyhow::Result<()> {
            unimplemented!()
        }
        fn run_foreground(&self, _ipc_url: String) -> io::Result<ExitStatus> {
            unimplemented!()
        }
    }

    impl Detect {
        pub fn detect(_path: Option<&Path>) -> anyhow::Result<impl InstalledApp> {
            Ok(App)
        }
    }
}

#[cfg(target_os = "macos")]
mod mac_os {
    use anyhow::{anyhow, Context, Result};
    use core_foundation::{
        array::{CFArray, CFIndex},
        string::kCFStringEncodingUTF8,
        url::{CFURLCreateWithBytes, CFURL},
    };
    use core_services::{kLSLaunchDefaults, LSLaunchURLSpec, LSOpenFromURLSpec, TCFType};
    use serde::Deserialize;
    use std::{
        ffi::OsStr,
        fs, io,
        path::{Path, PathBuf},
        process::{Command, ExitStatus},
        ptr,
    };

    use cli::FORCE_CLI_MODE_ENV_VAR_NAME;

    use crate::{Detect, InstalledApp};

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
            plist: InfoPlist,
        },
    }

    fn locate_bundle() -> Result<PathBuf> {
        let cli_path = std::env::current_exe()?.canonicalize()?;
        let mut app_path = cli_path.clone();
        while app_path.extension() != Some(OsStr::new("app")) {
            if !app_path.pop() {
                return Err(anyhow!("cannot find app bundle containing {:?}", cli_path));
            }
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
                locate_bundle().context("bundle autodiscovery")?
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
                _ => {
                    println!("Bundle path {bundle_path:?} has no *.app extension, attempting to locate a dev build");
                    let plist_path = bundle_path
                        .parent()
                        .with_context(|| format!("Bundle path {bundle_path:?} has no parent"))?
                        .join("WebRTC.framework/Resources/Info.plist");
                    let plist =
                        plist::from_file::<_, InfoPlist>(&plist_path).with_context(|| {
                            format!("Reading dev bundle plist file at {plist_path:?}")
                        })?;
                    Ok(Bundle::LocalPath {
                        executable: bundle_path,
                        plist,
                    })
                }
            }
        }
    }

    impl InstalledApp for Bundle {
        fn zed_version_string(&self) -> String {
            let is_dev = matches!(self, Self::LocalPath { .. });
            format!(
                "Zed {}{} – {}",
                self.plist().bundle_short_version_string,
                if is_dev { " (dev)" } else { "" },
                self.path().display(),
            )
        }

        fn launch(&self, url: String) -> anyhow::Result<()> {
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
                                launchFlags: kLSLaunchDefaults,
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
                    let command = command
                        .env(FORCE_CLI_MODE_ENV_VAR_NAME, "")
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

        fn run_foreground(&self, ipc_url: String) -> io::Result<ExitStatus> {
            let path = match self {
                Bundle::App { app_bundle, .. } => app_bundle.join("Contents/MacOS/zed"),
                Bundle::LocalPath { executable, .. } => executable.clone(),
            };

            std::process::Command::new(path).arg(ipc_url).status()
        }
    }

    impl Bundle {
        fn plist(&self) -> &InfoPlist {
            match self {
                Self::App { plist, .. } => plist,
                Self::LocalPath { plist, .. } => plist,
            }
        }

        fn path(&self) -> &Path {
            match self {
                Self::App { app_bundle, .. } => app_bundle,
                Self::LocalPath { executable, .. } => executable,
            }
        }
    }

    pub(super) fn spawn_channel_cli(
        channel: release_channel::ReleaseChannel,
        leftover_args: Vec<String>,
    ) -> Result<()> {
        use anyhow::bail;

        let app_id_prompt = format!("id of app \"{}\"", channel.display_name());
        let app_id_output = Command::new("osascript")
            .arg("-e")
            .arg(&app_id_prompt)
            .output()?;
        if !app_id_output.status.success() {
            bail!("Could not determine app id for {}", channel.display_name());
        }
        let app_name = String::from_utf8(app_id_output.stdout)?.trim().to_owned();
        let app_path_prompt = format!("kMDItemCFBundleIdentifier == '{app_name}'");
        let app_path_output = Command::new("mdfind").arg(app_path_prompt).output()?;
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
