use crate::{
    RemoteClientDelegate, RemotePlatform,
    remote_client::{CommandTemplate, RemoteConnection, RemoteConnectionOptions},
};
use anyhow::{Result, anyhow, bail};
use async_trait::async_trait;
use collections::HashMap;
use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext as _, AsyncApp, SemanticVersion, Task};
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use rpc::proto::Envelope;
use smol::{fs, process};
use std::{
    ffi::OsStr,
    fmt::Write as _,
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::Instant,
};
use util::{
    paths::{PathStyle, RemotePathBuf},
    rel_path::RelPath,
    shell::ShellKind,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WslConnectionOptions {
    pub distro_name: String,
    pub user: Option<String>,
}

impl From<settings::WslConnection> for WslConnectionOptions {
    fn from(val: settings::WslConnection) -> Self {
        WslConnectionOptions {
            distro_name: val.distro_name.into(),
            user: val.user,
        }
    }
}

#[derive(Debug)]
pub(crate) struct WslRemoteConnection {
    remote_binary_path: Option<Arc<RelPath>>,
    platform: RemotePlatform,
    shell: String,
    default_system_shell: String,
    connection_options: WslConnectionOptions,
    can_exec: bool,
}

impl WslRemoteConnection {
    pub(crate) async fn new(
        connection_options: WslConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        log::info!(
            "Connecting to WSL distro {} with user {:?}",
            connection_options.distro_name,
            connection_options.user
        );
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
            default_system_shell: String::from("/bin/sh"),
            can_exec: true,
        };
        delegate.set_status(Some("Detecting WSL environment"), cx);
        this.shell = this.detect_shell().await?;
        let shell = ShellKind::new(&this.shell, false);
        this.can_exec = this.detect_can_exec(shell).await?;
        this.platform = this.detect_platform(shell).await?;
        this.remote_binary_path = Some(
            this.ensure_server_binary(&delegate, release_channel, version, commit, shell, cx)
                .await?,
        );
        log::debug!("Detected WSL environment: {this:#?}");

        Ok(this)
    }

    async fn detect_can_exec(&self, shell: ShellKind) -> Result<bool> {
        let options = &self.connection_options;
        let program = if shell == ShellKind::Nushell {
            "^uname"
        } else {
            "uname"
        };
        let args = &["-m"];
        let output = wsl_command_impl(options, program, args, true)
            .output()
            .await?;

        if !output.status.success() {
            let output = wsl_command_impl(options, program, args, false)
                .output()
                .await?;

            if !output.status.success() {
                return Err(anyhow!(
                    "Command '{}' failed: {}",
                    program,
                    String::from_utf8_lossy(&output.stderr).trim()
                ));
            }

            Ok(false)
        } else {
            Ok(true)
        }
    }
    async fn detect_platform(&self, shell: ShellKind) -> Result<RemotePlatform> {
        let arch_str = if shell == ShellKind::Nushell {
            // https://github.com/nushell/nushell/issues/12570
            self.run_wsl_command("sh", &["-c", "uname -m"])
        } else {
            self.run_wsl_command("uname", &["-m"])
        }
        .await?;
        let arch_str = arch_str.trim().to_string();
        let arch = match arch_str.as_str() {
            "x86_64" => "x86_64",
            "aarch64" | "arm64" => "aarch64",
            _ => "x86_64",
        };
        Ok(RemotePlatform { os: "linux", arch })
    }

    async fn detect_shell(&self) -> Result<String> {
        Ok(self
            .run_wsl_command("sh", &["-c", "echo $SHELL"])
            .await
            .ok()
            .unwrap_or_else(|| "/bin/sh".to_string()))
    }

    async fn windows_path_to_wsl_path(&self, source: &Path) -> Result<String> {
        windows_path_to_wsl_path_impl(&self.connection_options, source, self.can_exec).await
    }

    fn wsl_command(&self, program: &str, args: &[impl AsRef<OsStr>]) -> process::Command {
        wsl_command_impl(&self.connection_options, program, args, self.can_exec)
    }

    async fn run_wsl_command(&self, program: &str, args: &[&str]) -> Result<String> {
        run_wsl_command_impl(&self.connection_options, program, args, self.can_exec).await
    }

    async fn ensure_server_binary(
        &self,
        delegate: &Arc<dyn RemoteClientDelegate>,
        release_channel: ReleaseChannel,
        version: SemanticVersion,
        commit: Option<AppCommitSha>,
        shell: ShellKind,
        cx: &mut AsyncApp,
    ) -> Result<Arc<RelPath>> {
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

        let dst_path =
            paths::remote_wsl_server_dir_relative().join(RelPath::unix(&binary_name).unwrap());

        if let Some(parent) = dst_path.parent() {
            let parent = parent.display(PathStyle::Posix);
            if shell == ShellKind::Nushell {
                self.run_wsl_command("mkdir", &[&parent]).await
            } else {
                self.run_wsl_command("mkdir", &["-p", &parent]).await
            }
            .map_err(|e| anyhow!("Failed to create directory: {}", e))?;
        }

        #[cfg(debug_assertions)]
        if let Some(remote_server_path) =
            super::build_remote_server_from_source(&self.platform, delegate.as_ref(), cx).await?
        {
            let tmp_path = paths::remote_wsl_server_dir_relative().join(
                &RelPath::unix(&format!(
                    "download-{}-{}",
                    std::process::id(),
                    remote_server_path.file_name().unwrap().to_string_lossy()
                ))
                .unwrap(),
            );
            self.upload_file(&remote_server_path, &tmp_path, delegate, &shell, cx)
                .await?;
            self.extract_and_install(&tmp_path, &dst_path, delegate, cx)
                .await?;
            return Ok(dst_path);
        }

        if self
            .run_wsl_command(&dst_path.display(PathStyle::Posix), &["version"])
            .await
            .is_ok()
        {
            return Ok(dst_path);
        }

        delegate.set_status(Some("Installing remote server"), cx);

        let wanted_version = match release_channel {
            ReleaseChannel::Nightly | ReleaseChannel::Dev => None,
            _ => Some(cx.update(|cx| AppVersion::global(cx))?),
        };

        let src_path = delegate
            .download_server_binary_locally(self.platform, release_channel, wanted_version, cx)
            .await?;

        let tmp_path = format!(
            "{}.{}.gz",
            dst_path.display(PathStyle::Posix),
            std::process::id()
        );
        let tmp_path = RelPath::unix(&tmp_path).unwrap();

        self.upload_file(&src_path, &tmp_path, delegate, &shell, cx)
            .await?;
        self.extract_and_install(&tmp_path, &dst_path, delegate, cx)
            .await?;

        Ok(dst_path)
    }

    async fn upload_file(
        &self,
        src_path: &Path,
        dst_path: &RelPath,
        delegate: &Arc<dyn RemoteClientDelegate>,
        shell: &ShellKind,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Uploading remote server to WSL"), cx);

        if let Some(parent) = dst_path.parent() {
            let parent = parent.display(PathStyle::Posix);
            if *shell == ShellKind::Nushell {
                self.run_wsl_command("mkdir", &[&parent]).await
            } else {
                self.run_wsl_command("mkdir", &["-p", &parent]).await
            }
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

        let src_path_in_wsl = self.windows_path_to_wsl_path(src_path).await?;
        self.run_wsl_command(
            "cp",
            &["-f", &src_path_in_wsl, &dst_path.display(PathStyle::Posix)],
        )
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
        tmp_path: &RelPath,
        dst_path: &RelPath,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Extracting remote server"), cx);

        let tmp_path_str = tmp_path.display(PathStyle::Posix);
        let dst_path_str = dst_path.display(PathStyle::Posix);

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

        let Some(remote_binary_path) = &self.remote_binary_path else {
            return Task::ready(Err(anyhow!("Remote binary path not set")));
        };

        let mut proxy_args = vec![];
        for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
            if let Some(value) = std::env::var(env_var).ok() {
                // We don't quote the value here as it seems excessive and may result in invalid envs for the
                // proxy server. For example, `RUST_LOG='debug'` will result in a warning "invalid logging spec 'debug'', ignoring it"
                // in the proxy server. Therefore, we pass the env vars as is.
                proxy_args.push(format!("{}={}", env_var, value));
            }
        }
        proxy_args.push(remote_binary_path.display(PathStyle::Posix).into_owned());
        proxy_args.push("proxy".to_owned());
        proxy_args.push("--identifier".to_owned());
        proxy_args.push(unique_identifier);

        if reconnect {
            proxy_args.push("--reconnect".to_owned());
        }
        let proxy_process = match self
            .wsl_command("env", &proxy_args)
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
            let can_exec = self.can_exec;
            async move {
                let wsl_src = windows_path_to_wsl_path_impl(&options, &src_path, can_exec).await?;

                run_wsl_command_impl(
                    &options,
                    "cp",
                    &["-r", &wsl_src, &dest_path.to_string()],
                    can_exec,
                )
                .await
                .map_err(|e| {
                    anyhow!(
                        "failed to upload directory {} -> {}: {}",
                        src_path.display(),
                        dest_path,
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

    fn shares_network_interface(&self) -> bool {
        true
    }

    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        if port_forward.is_some() {
            bail!("WSL shares the network interface with the host system");
        }

        let working_dir = working_dir
            .map(|working_dir| RemotePathBuf::new(working_dir, PathStyle::Posix).to_string())
            .unwrap_or("~".to_string());

        let mut exec = String::from("exec env ");

        for (k, v) in env.iter() {
            if let Some((k, v)) = shlex::try_quote(k).ok().zip(shlex::try_quote(v).ok()) {
                write!(exec, "{}={} ", k, v).unwrap();
            }
        }

        if let Some(program) = program {
            write!(exec, "{}", shlex::try_quote(&program)?).unwrap();
            for arg in args {
                let arg = shlex::try_quote(&arg)?;
                write!(exec, " {}", &arg).unwrap();
            }
        } else {
            write!(&mut exec, "{} -l", self.shell).unwrap();
        }

        let wsl_args = if let Some(user) = &self.connection_options.user {
            vec![
                "--distribution".to_string(),
                self.connection_options.distro_name.clone(),
                "--user".to_string(),
                user.clone(),
                "--cd".to_string(),
                working_dir,
                "--".to_string(),
                self.shell.clone(),
                "-c".to_string(),
                exec,
            ]
        } else {
            vec![
                "--distribution".to_string(),
                self.connection_options.distro_name.clone(),
                "--cd".to_string(),
                working_dir,
                "--".to_string(),
                self.shell.clone(),
                "-c".to_string(),
                exec,
            ]
        };

        Ok(CommandTemplate {
            program: "wsl.exe".to_string(),
            args: wsl_args,
            env: HashMap::default(),
        })
    }

    fn build_forward_ports_command(
        &self,
        _: Vec<(u16, String, u16)>,
    ) -> anyhow::Result<CommandTemplate> {
        Err(anyhow!("WSL shares a network interface with the host"))
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

    fn default_system_shell(&self) -> String {
        self.default_system_shell.clone()
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

async fn windows_path_to_wsl_path_impl(
    options: &WslConnectionOptions,
    source: &Path,
    exec: bool,
) -> Result<String> {
    let source = sanitize_path(source).await?;
    run_wsl_command_impl(options, "wslpath", &["-u", &source], exec).await
}

async fn run_wsl_command_impl(
    options: &WslConnectionOptions,
    program: &str,
    args: &[&str],
    exec: bool,
) -> Result<String> {
    let output = wsl_command_impl(options, program, args, exec)
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!(
            "Command '{}' failed: {}",
            program,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Creates a new `wsl.exe` command that runs the given program with the given arguments.
///
/// If `exec` is true, the command will be executed in the WSL environment without spawning a new shell.
fn wsl_command_impl(
    options: &WslConnectionOptions,
    program: &str,
    args: &[impl AsRef<OsStr>],
    exec: bool,
) -> process::Command {
    let mut command = util::command::new_smol_command("wsl.exe");

    if let Some(user) = &options.user {
        command.arg("--user").arg(user);
    }

    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--distribution")
        .arg(&options.distro_name)
        .arg("--cd")
        .arg("~");

    if exec {
        command.arg("--exec");
    }

    command.arg(program).args(args);

    log::debug!("wsl {:?}", command);
    command
}
