use crate::{
    RemoteClientDelegate, RemotePlatform,
    remote_client::{CommandTemplate, RemoteConnection, RemoteConnectionOptions},
};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext as _, AsyncApp, SemanticVersion, Task};
use itertools::Itertools;
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use rpc::proto::Envelope;
use smol::{fs, process};
use std::{
    fmt::Write as _,
    iter,
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::Instant,
};
use util::paths::{PathStyle, RemotePathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WslConnectionOptions {
    pub distro_name: String,
}

pub(crate) struct WslRemoteConnection {
    remote_binary_path: Option<RemotePathBuf>,
    platform: RemotePlatform,
    shell: String,
    connection_options: WslConnectionOptions,
}

#[allow(dead_code)]
enum WslPathConverterKind {
    WslToWindows,
    WindowsToWsl,
}

impl WslRemoteConnection {
    pub(crate) async fn new(
        connection_options: WslConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let (release_channel, version, commit) = cx.update(|cx| {
            (
                ReleaseChannel::global(cx),
                AppVersion::global(cx),
                AppCommitSha::try_global(cx),
            )
        })?;

        let mut this = Self {
            connection_options,
            remote_binary_path: None,
            platform: RemotePlatform { os: "", arch: "" },
            shell: String::new(),
        };
        delegate.set_status(Some("Detecting WSL environment"), cx);
        this.platform = this.detect_platform().await?;
        this.shell = this.detect_shell().await?;
        this.remote_binary_path = Some(
            this.ensure_server_binary(&delegate, release_channel, version, commit, cx)
                .await?,
        );

        Ok(this)
    }

    async fn detect_platform(&self) -> Result<RemotePlatform> {
        let arch_str = self.run_wsl_command("uname", &["-m"]).await?;
        let arch_str = arch_str.trim().to_string();
        let arch = match arch_str.as_str() {
            "x86_64" => "x86_64",
            "aarch64" | "arm64" => "aarch64",
            _ => "x86_64",
        };
        Ok(RemotePlatform { os: "linux", arch })
    }

    async fn detect_shell(&self) -> Result<String> {
        let shell = self
            .run_wsl_command("sh", &["-c", "echo $SHELL"])
            .await
            .unwrap_or_else(|_| "bash".to_string());
        Ok(shell.trim().split('/').last().unwrap_or("bash").to_string())
    }

    async fn path_converter(&self, source: &Path, kind: WslPathConverterKind) -> Result<String> {
        let source = sanitize_path(source).await?;
        let arg = match kind {
            WslPathConverterKind::WslToWindows => "-w",
            WslPathConverterKind::WindowsToWsl => "-u",
        };
        self.run_wsl_command("wslpath", &[arg, &source]).await
    }

    fn wsl_command(&self, program: &str, args: &[&str]) -> process::Command {
        wsl_command_impl(&self.connection_options, program, args)
    }

    async fn run_wsl_command(&self, program: &str, args: &[&str]) -> Result<String> {
        run_wsl_command_impl(&self.connection_options, program, args).await
    }

    #[cfg(debug_assertions)]
    #[cfg(not(target_os = "windows"))]
    async fn build_local(
        &self,
        _build_remote_server: String,
        _delegate: &Arc<dyn RemoteClientDelegate>,
        _cx: &mut AsyncApp,
    ) -> Result<PathBuf> {
        Err(anyhow!(
            "Local build of remote server is only supported on Windows"
        ))
    }

    #[cfg(debug_assertions)]
    #[cfg(target_os = "windows")]
    async fn build_local(
        &self,
        build_remote_server: String,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<PathBuf> {
        use smol::process::{Command, Stdio};
        use std::env::VarError;

        async fn run_cmd(command: &mut Command) -> Result<()> {
            let output = command
                .kill_on_drop(true)
                .stderr(Stdio::inherit())
                .output()
                .await?;
            anyhow::ensure!(
                output.status.success(),
                "Failed to run command: {command:?}"
            );
            Ok(())
        }

        let use_musl = !build_remote_server.contains("nomusl");
        let triple = format!(
            "{}-{}",
            self.platform.arch,
            match self.platform.os {
                "linux" =>
                    if use_musl {
                        "unknown-linux-musl"
                    } else {
                        "unknown-linux-gnu"
                    },
                _ => anyhow::bail!("can't cross compile for: {:?}", self.platform),
            }
        );
        let mut rust_flags = match std::env::var("RUSTFLAGS") {
            Ok(val) => val,
            Err(VarError::NotPresent) => String::new(),
            Err(e) => {
                log::error!("Failed to get env var `RUSTFLAGS` value: {e}");
                String::new()
            }
        };
        if self.platform.os == "linux" && use_musl {
            rust_flags.push_str(" -C target-feature=+crt-static");
        }
        if build_remote_server.contains("mold") {
            rust_flags.push_str(" -C link-arg=-fuse-ld=mold");
        }

        if build_remote_server.contains("cross") {
            use util::paths::SanitizedPath;

            delegate.set_status(Some("Installing cross.rs for cross-compilation"), cx);
            log::info!("installing cross");
            run_cmd(Command::new("cargo").args([
                "install",
                "cross",
                "--git",
                "https://github.com/cross-rs/cross",
            ]))
            .await?;

            delegate.set_status(
                Some(&format!(
                    "Building remote server binary from source for {} with Docker",
                    &triple
                )),
                cx,
            );
            log::info!("building remote server binary from source for {}", &triple);

            // On Windows, the binding needs to be set to the canonical path
            let src =
                SanitizedPath::from(smol::fs::canonicalize("./target").await?).to_glob_string();
            run_cmd(
                Command::new("cross")
                    .args([
                        "build",
                        "--package",
                        "remote_server",
                        "--features",
                        "debug-embed",
                        "--target-dir",
                        "target/remote_server",
                        "--target",
                        &triple,
                    ])
                    .env(
                        "CROSS_CONTAINER_OPTS",
                        format!("--mount type=bind,src={src},dst=/app/target"),
                    )
                    .env("RUSTFLAGS", &rust_flags),
            )
            .await?;
        } else {
            let which = cx
                .background_spawn(async move { which::which("zig") })
                .await;

            if which.is_err() {
                anyhow::bail!(
                    "zig not found on $PATH, install zig (use `winget install -e --id zig.zig` or see https://ziglang.org/learn/getting-started or use zigup) or pass ZED_BUILD_REMOTE_SERVER=cross to use cross"
                )
            }

            delegate.set_status(Some("Adding rustup target for cross-compilation"), cx);
            log::info!("adding rustup target");
            run_cmd(Command::new("rustup").args(["target", "add"]).arg(&triple)).await?;

            delegate.set_status(Some("Installing cargo-zigbuild for cross-compilation"), cx);
            log::info!("installing cargo-zigbuild");
            run_cmd(Command::new("cargo").args(["install", "--locked", "cargo-zigbuild"])).await?;

            delegate.set_status(
                Some(&format!(
                    "Building remote binary from source for {triple} with Zig"
                )),
                cx,
            );
            log::info!("building remote binary from source for {triple} with Zig");
            run_cmd(
                Command::new("cargo")
                    .args([
                        "zigbuild",
                        "--package",
                        "remote_server",
                        "--features",
                        "debug-embed",
                        "--target-dir",
                        "target/remote_server",
                        "--target",
                        &triple,
                    ])
                    .env("RUSTFLAGS", &rust_flags),
            )
            .await?;
        };
        let bin_path = Path::new("target")
            .join("remote_server")
            .join(&triple)
            .join("debug")
            .join("remote_server");

        let path = if !build_remote_server.contains("nocompress") {
            use anyhow::Context;

            delegate.set_status(Some("Compressing binary"), cx);

            // On Windows, we use 7z to compress the binary
            let seven_zip = which::which("7z.exe").context("7z.exe not found on $PATH, install it (e.g. with `winget install -e --id 7zip.7zip`) or, if you don't want this behaviour, set $env:ZED_BUILD_REMOTE_SERVER=\"nocompress\"")?;
            let gz_path = format!("target/remote_server/{}/debug/remote_server.gz", triple);
            if smol::fs::metadata(&gz_path).await.is_ok() {
                smol::fs::remove_file(&gz_path).await?;
            }
            run_cmd(Command::new(seven_zip).args([
                "a",
                "-tgzip",
                &gz_path,
                &bin_path.to_string_lossy(),
            ]))
            .await?;

            let mut archive_path = bin_path;
            archive_path.set_extension("gz");
            std::env::current_dir()?.join(archive_path)
        } else {
            bin_path
        };

        Ok(path.canonicalize()?)
    }

    async fn ensure_server_binary(
        &self,
        delegate: &Arc<dyn RemoteClientDelegate>,
        release_channel: ReleaseChannel,
        version: SemanticVersion,
        commit: Option<AppCommitSha>,
        cx: &mut AsyncApp,
    ) -> Result<RemotePathBuf> {
        let version_str = match release_channel {
            ReleaseChannel::Nightly => {
                let commit = commit.map(|s| s.full()).unwrap_or_default();
                format!("{}-{}", version, commit)
            }
            ReleaseChannel::Dev => "build".to_string(),
            _ => version.to_string(),
        };

        let binary_name = format!(
            "zed-remote-server-{}-{}",
            release_channel.dev_name(),
            version_str
        );

        let dst_path = RemotePathBuf::new(
            paths::remote_wsl_server_dir_relative().join(binary_name),
            PathStyle::Posix,
        );

        if let Some(parent) = dst_path.parent() {
            self.run_wsl_command("mkdir", &["-p", &parent.to_string()])
                .await
                .map_err(|e| anyhow!("Failed to create directory: {}", e))?;
        }

        let build_remote_server = std::env::var("ZED_BUILD_REMOTE_SERVER").ok();
        #[cfg(debug_assertions)]
        if let Some(build_remote_server) = build_remote_server {
            let src_path = self.build_local(build_remote_server, delegate, cx).await?;
            let tmp_path = RemotePathBuf::new(
                paths::remote_wsl_server_dir_relative().join(format!(
                    "download-{}-{}",
                    std::process::id(),
                    src_path.file_name().unwrap().to_string_lossy()
                )),
                PathStyle::Posix,
            );
            self.upload_file(&src_path, &tmp_path, delegate, cx).await?;
            self.extract_and_install(&tmp_path, &dst_path, delegate, cx)
                .await?;
            return Ok(dst_path);
        }

        if self
            .run_wsl_command(&dst_path.to_string(), &["version"])
            .await
            .is_ok()
        {
            return Ok(dst_path);
        }

        delegate.set_status(Some("Installing remote server"), cx);

        let wanted_version = match release_channel {
            ReleaseChannel::Nightly => None,
            ReleaseChannel::Dev => {
                return Err(anyhow!("Dev builds require manual installation"));
            }
            _ => Some(cx.update(|cx| AppVersion::global(cx))?),
        };

        let src_path = delegate
            .download_server_binary_locally(self.platform, release_channel, wanted_version, cx)
            .await?;

        let tmp_path = RemotePathBuf::new(
            PathBuf::from(format!("{}.{}.tmp", dst_path, std::process::id())),
            PathStyle::Posix,
        );

        self.upload_file(&src_path, &tmp_path, delegate, cx).await?;
        self.extract_and_install(&tmp_path, &dst_path, delegate, cx)
            .await?;

        Ok(dst_path)
    }

    async fn upload_file(
        &self,
        src_path: &Path,
        dst_path: &RemotePathBuf,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Uploading remote server to WSL"), cx);

        if let Some(parent) = dst_path.parent() {
            self.run_wsl_command("mkdir", &["-p", &parent.to_string()])
                .await
                .map_err(|e| anyhow!("Failed to create directory when uploading file: {}", e))?;
        }

        let t0 = Instant::now();
        let src_stat = fs::metadata(&src_path).await?;
        let size = src_stat.len();
        log::info!(
            "uploading remote server to WSL {:?} ({}kb)",
            dst_path,
            size / 1024
        );

        let src_path_in_wsl = self
            .path_converter(src_path, WslPathConverterKind::WindowsToWsl)
            .await?;
        self.run_wsl_command("cp", &["-f", &src_path_in_wsl, &dst_path.to_string()])
            .await
            .map_err(|e| {
                anyhow!(
                    "Failed to copy file {}({}) to WSL {:?}: {}",
                    src_path.display(),
                    src_path_in_wsl,
                    dst_path,
                    e
                )
            })?;

        log::info!("uploaded remote server in {:?}", t0.elapsed());
        Ok(())
    }

    async fn extract_and_install(
        &self,
        tmp_path: &RemotePathBuf,
        dst_path: &RemotePathBuf,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Extracting remote server"), cx);

        let tmp_path_str = tmp_path.to_string();
        let dst_path_str = dst_path.to_string();

        // Build extraction script with proper error handling
        let script = if tmp_path_str.ends_with(".gz") {
            let uncompressed = tmp_path_str.trim_end_matches(".gz");
            format!(
                "set -e; gunzip -f '{}' && chmod 755 '{}' && mv -f '{}' '{}'",
                tmp_path_str, uncompressed, uncompressed, dst_path_str
            )
        } else {
            format!(
                "set -e; chmod 755 '{}' && mv -f '{}' '{}'",
                tmp_path_str, tmp_path_str, dst_path_str
            )
        };

        self.run_wsl_command("sh", &["-c", &script])
            .await
            .map_err(|e| anyhow!("Failed to extract server binary: {}", e))?;
        Ok(())
    }
}

#[async_trait(?Send)]
impl RemoteConnection for WslRemoteConnection {
    fn start_proxy(
        &self,
        unique_identifier: String,
        reconnect: bool,
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Task<Result<i32>> {
        delegate.set_status(Some("Starting proxy"), cx);

        let Some(remote_binary_path) = self.remote_binary_path.clone() else {
            return Task::ready(Err(anyhow!("Remote binary path not set")));
        };

        let mut proxy_command = format!(
            "exec {} proxy --identifier {}",
            remote_binary_path.to_string(),
            unique_identifier
        );

        if reconnect {
            proxy_command.push_str(" --reconnect");
        }

        for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
            if let Some(value) = std::env::var(env_var).ok() {
                proxy_command = format!("{}='{}' {}", env_var, value, proxy_command);
            }
        }
        let proxy_process = match self
            .wsl_command("sh", &["-lc", &proxy_command])
            .kill_on_drop(true)
            .spawn()
        {
            Ok(process) => process,
            Err(error) => {
                return Task::ready(Err(anyhow!("failed to spawn remote server: {}", error)));
            }
        };

        super::handle_rpc_messages_over_child_process_stdio(
            proxy_process,
            incoming_tx,
            outgoing_rx,
            connection_activity_tx,
            cx,
        )
    }

    fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: RemotePathBuf,
        cx: &App,
    ) -> Task<Result<()>> {
        cx.background_spawn({
            let options = self.connection_options.clone();
            async move {
                let wsl_src =
                    path_converter_impl(&options, &src_path, WslPathConverterKind::WindowsToWsl)
                        .await?;

                run_wsl_command_impl(&options, "cp", &["-r", &wsl_src, &dest_path.to_string()])
                    .await
                    .map_err(|e| {
                        anyhow!(
                            "failed to upload directory {} -> {}: {}",
                            src_path.display(),
                            dest_path.to_string(),
                            e
                        )
                    })?;

                Ok(())
            }
        })
    }

    async fn kill(&self) -> Result<()> {
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        false
    }

    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        let mut script = String::new();
        if let Some(working_dir) = working_dir {
            let working_dir = RemotePathBuf::new(working_dir.into(), PathStyle::Posix).to_string();
            if working_dir.starts_with("~/") {
                let working_dir = working_dir.trim_start_matches("~").trim_start_matches("/");
                write!(&mut script, "cd \"$HOME/{}\"; ", working_dir).unwrap();
            } else {
                write!(&mut script, "cd \"{}\"; ", working_dir).unwrap();
            }
        } else {
            write!(&mut script, "cd; ").unwrap();
        }

        for (k, v) in env.iter() {
            write!(&mut script, "{}='{}' ", k, v).unwrap();
        }

        if let Some(program) = program {
            script.push_str(&program);
            for arg in args {
                script.push(' ');
                script.push_str(arg);
            }
        } else {
            write!(&mut script, "exec {} -l", self.shell).unwrap();
        }

        let wsl_args = vec![
            "--distribution".to_string(),
            self.connection_options.distro_name.clone(),
            "--".to_string(),
            self.shell.clone(),
            "-c".to_string(),
            script,
        ];

        // todo(max): port forwarding
        if port_forward.is_some() {
            log::warn!("Port forwarding is not directly supported in WSL transport");
        }

        Ok(CommandTemplate {
            program: "wsl.exe".to_string(),
            args: wsl_args,
            env: HashMap::default(),
        })
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Wsl(self.connection_options.clone())
    }

    fn path_style(&self) -> PathStyle {
        PathStyle::Posix
    }

    fn shell(&self) -> String {
        self.shell.clone()
    }
}

/// `wslpath` is a executable available in WSL, it's a linux binary.
/// So it doesn't support Windows style paths.
async fn sanitize_path(path: &Path) -> Result<String> {
    let path = smol::fs::canonicalize(path).await?;
    let path_str = path.to_string_lossy();

    let sanitized = path_str.strip_prefix(r"\\?\").unwrap_or(&path_str);
    Ok(sanitized.replace('\\', "/"))
}

async fn path_converter_impl(
    options: &WslConnectionOptions,
    source: &Path,
    kind: WslPathConverterKind,
) -> Result<String> {
    let source = sanitize_path(source).await?;
    let arg = match kind {
        WslPathConverterKind::WslToWindows => "-w",
        WslPathConverterKind::WindowsToWsl => "-u",
    };
    run_wsl_command_impl(options, "wslpath", &[arg, &source]).await
}

fn wsl_command_impl(
    options: &WslConnectionOptions,
    program: &str,
    args: &[&str],
) -> process::Command {
    let to_run = iter::once(&program)
        .chain(args.iter())
        .map(|token| shlex::try_quote(token).unwrap())
        .join(" ");

    let mut command = util::command::new_smol_command("wsl.exe");
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--distribution")
        .arg(&options.distro_name)
        .arg("sh")
        .arg("-c")
        .arg(format!("cd; {to_run}"));

    command
}

async fn run_wsl_command_impl(
    options: &WslConnectionOptions,
    program: &str,
    args: &[&str],
) -> Result<String> {
    let output = wsl_command_impl(options, program, args).output().await?;

    if !output.status.success() {
        return Err(anyhow!(
            "Command '{}' failed: {}",
            program,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
