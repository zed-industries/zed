#![cfg_attr(any(target_os = "linux", target_os = "windows"), allow(dead_code))]

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cli::{CliRequest, CliResponse};
use serde::Deserialize;
use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};
use util::paths::PathLikeWithPosition;

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
    /// Custom Zed.app path
    #[arg(short, long)]
    bundle_path: Option<PathBuf>,
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

#[derive(Debug, Deserialize)]
struct InfoPlist {
    #[serde(rename = "CFBundleShortVersionString")]
    bundle_short_version_string: String,
}

fn main() -> Result<()> {
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

    let bundle = Bundle::detect(args.bundle_path.as_deref()).context("Bundle detection")?;

    if let Some(dev_server_token) = args.dev_server_token {
        return bundle.spawn(vec!["--dev-server-token".into(), dev_server_token]);
    }

    if args.version {
        println!("{}", bundle.zed_version_string());
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

    let (tx, rx) = bundle.launch()?;
    let open_new_workspace = if args.new {
        Some(true)
    } else if args.add {
        Some(false)
    } else {
        None
    };

    tx.send(CliRequest::Open {
        paths,
        wait: args.wait,
        open_new_workspace,
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

#[cfg(target_os = "linux")]
mod linux {
    use std::path::Path;

    use cli::{CliRequest, CliResponse};
    use ipc_channel::ipc::{IpcReceiver, IpcSender};

    use crate::{Bundle, InfoPlist};

    impl Bundle {
        pub fn detect(_args_bundle_path: Option<&Path>) -> anyhow::Result<Self> {
            unimplemented!()
        }

        pub fn plist(&self) -> &InfoPlist {
            unimplemented!()
        }

        pub fn path(&self) -> &Path {
            unimplemented!()
        }

        pub fn launch(&self) -> anyhow::Result<(IpcSender<CliRequest>, IpcReceiver<CliResponse>)> {
            unimplemented!()
        }

        pub fn spawn(&self, _args: Vec<String>) -> anyhow::Result<()> {
            unimplemented!()
        }

        pub fn zed_version_string(&self) -> String {
            unimplemented!()
        }
    }
}

// todo("windows")
#[cfg(target_os = "windows")]
mod windows {
    use std::path::Path;

    use cli::{CliRequest, CliResponse};
    use ipc_channel::ipc::{IpcReceiver, IpcSender};

    use crate::{Bundle, InfoPlist};

    impl Bundle {
        pub fn detect(_args_bundle_path: Option<&Path>) -> anyhow::Result<Self> {
            unimplemented!()
        }

        pub fn plist(&self) -> &InfoPlist {
            unimplemented!()
        }

        pub fn path(&self) -> &Path {
            unimplemented!()
        }

        pub fn launch(&self) -> anyhow::Result<(IpcSender<CliRequest>, IpcReceiver<CliResponse>)> {
            unimplemented!()
        }

        pub fn spawn(&self, _args: Vec<String>) -> anyhow::Result<()> {
            unimplemented!()
        }

        pub fn zed_version_string(&self) -> String {
            unimplemented!()
        }
    }
}

#[cfg(target_os = "macos")]
mod mac_os {
    use anyhow::{Context, Result};
    use core_foundation::{
        array::{CFArray, CFIndex},
        string::kCFStringEncodingUTF8,
        url::{CFURLCreateWithBytes, CFURL},
    };
    use core_services::{kLSLaunchDefaults, LSLaunchURLSpec, LSOpenFromURLSpec, TCFType};
    use std::{fs, path::Path, process::Command, ptr};

    use cli::{CliRequest, CliResponse, IpcHandshake, FORCE_CLI_MODE_ENV_VAR_NAME};
    use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};

    use crate::{locate_bundle, Bundle, InfoPlist};

    impl Bundle {
        pub fn detect(args_bundle_path: Option<&Path>) -> anyhow::Result<Self> {
            let bundle_path = if let Some(bundle_path) = args_bundle_path {
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
                    Ok(Self::App {
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
                    Ok(Self::LocalPath {
                        executable: bundle_path,
                        plist,
                    })
                }
            }
        }

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

        pub fn spawn(&self, args: Vec<String>) -> Result<()> {
            let path = match self {
                Self::App { app_bundle, .. } => app_bundle.join("Contents/MacOS/zed"),
                Self::LocalPath { executable, .. } => executable.clone(),
            };
            Command::new(path).args(args).status()?;
            Ok(())
        }

        pub fn launch(&self) -> anyhow::Result<(IpcSender<CliRequest>, IpcReceiver<CliResponse>)> {
            let (server, server_name) =
                IpcOneShotServer::<IpcHandshake>::new().context("Handshake before Zed spawn")?;
            let url = format!("zed-cli://{server_name}");

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

            let (_, handshake) = server.accept().context("Handshake after Zed spawn")?;
            Ok((handshake.requests, handshake.responses))
        }

        pub fn zed_version_string(&self) -> String {
            let is_dev = matches!(self, Self::LocalPath { .. });
            format!(
                "Zed {}{} – {}",
                self.plist().bundle_short_version_string,
                if is_dev { " (dev)" } else { "" },
                self.path().display(),
            )
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
