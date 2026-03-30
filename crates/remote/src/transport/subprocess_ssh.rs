use crate::{
    RemoteArch, RemoteClientDelegate, RemoteOs, RemotePlatform,
    remote_client::{CommandTemplate, Interactive, RemoteConnection, RemoteConnectionOptions},
    transport::{parse_platform, parse_shell},
};
use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{
    AsyncReadExt as _, FutureExt as _,
    channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender},
    select_biased,
};
use gpui::{App, AppContext as _, AsyncApp, Task};
use parking_lot::Mutex;
use paths::remote_server_dir_relative;
use release_channel::{AppVersion, ReleaseChannel};
use rpc::proto::Envelope;
use semver::Version;
use smol::fs;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};
use tempfile::TempDir;
use util::command::{Child, Stdio};
use util::{
    paths::{PathStyle, RemotePathBuf},
    rel_path::RelPath,
    shell::ShellKind,
};

use super::ssh::{SshConnectionOptions, bracket_ipv6};

fn scp_destination(options: &SshConnectionOptions) -> String {
    if let Some(username) = &options.username {
        format!("{}@{}", username, options.host.to_bracketed_string())
    } else {
        options.host.to_string()
    }
}

pub(crate) struct SshRemoteConnection {
    socket: SshSocket,
    master_process: Mutex<Option<MasterProcess>>,
    remote_binary_path: Option<Arc<RelPath>>,
    ssh_platform: RemotePlatform,
    ssh_path_style: PathStyle,
    ssh_shell: String,
    ssh_shell_kind: ShellKind,
    ssh_default_system_shell: String,
    _temp_dir: TempDir,
}

struct SshSocket {
    connection_options: SshConnectionOptions,
    #[cfg(not(windows))]
    socket_path: std::path::PathBuf,
    /// Extra environment variables needed for the ssh process
    envs: HashMap<String, String>,
    #[cfg(windows)]
    _proxy: askpass::PasswordProxy,
}

struct MasterProcess {
    process: Child,
}

#[cfg(not(windows))]
impl MasterProcess {
    pub fn new(
        askpass_script_path: &std::ffi::OsStr,
        additional_args: Vec<String>,
        socket_path: &std::path::Path,
        destination: &str,
    ) -> Result<Self> {
        let args = [
            "-N",
            "-o",
            "ControlPersist=no",
            "-o",
            "ControlMaster=yes",
            "-o",
        ];

        let mut master_process = util::command::new_command("ssh");
        master_process
            .kill_on_drop(true)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("SSH_ASKPASS_REQUIRE", "force")
            .env("SSH_ASKPASS", askpass_script_path)
            .args(additional_args)
            .args(args);

        master_process.arg(format!("ControlPath={}", socket_path.display()));

        let process = master_process.arg(&destination).spawn()?;

        Ok(MasterProcess { process })
    }

    pub async fn wait_connected(&mut self) -> Result<()> {
        let Some(mut stdout) = self.process.stdout.take() else {
            anyhow::bail!("ssh process stdout capture failed");
        };

        let mut output = Vec::new();
        stdout.read_to_end(&mut output).await?;
        Ok(())
    }
}

#[cfg(windows)]
impl MasterProcess {
    const CONNECTION_ESTABLISHED_MAGIC: &str = "ZED_SSH_CONNECTION_ESTABLISHED";

    pub fn new(
        askpass_script_path: &std::ffi::OsStr,
        additional_args: Vec<String>,
        destination: &str,
    ) -> Result<Self> {
        // On Windows, `ControlMaster` and `ControlPath` are not supported:
        // https://github.com/PowerShell/Win32-OpenSSH/issues/405
        // https://github.com/PowerShell/Win32-OpenSSH/wiki/Project-Scope
        //
        // Using an ugly workaround to detect connection establishment
        // -N doesn't work with JumpHosts as windows openssh never closes stdin in that case
        let args = [
            "-t",
            &format!("echo '{}'; exec $0", Self::CONNECTION_ESTABLISHED_MAGIC),
        ];

        let mut master_process = util::command::new_command("ssh");
        master_process
            .kill_on_drop(true)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("SSH_ASKPASS_REQUIRE", "force")
            .env("SSH_ASKPASS", askpass_script_path)
            .args(additional_args)
            .arg(destination)
            .args(args);

        let process = master_process.spawn()?;

        Ok(MasterProcess { process })
    }

    pub async fn wait_connected(&mut self) -> Result<()> {
        use smol::io::AsyncBufReadExt;

        let Some(stdout) = self.process.stdout.take() else {
            anyhow::bail!("ssh process stdout capture failed");
        };

        let mut reader = smol::io::BufReader::new(stdout);

        let mut line = String::new();

        loop {
            let n = reader.read_line(&mut line).await?;
            if n == 0 {
                anyhow::bail!("ssh process exited before connection established");
            }

            if line.contains(Self::CONNECTION_ESTABLISHED_MAGIC) {
                return Ok(());
            }
        }
    }
}

impl AsRef<Child> for MasterProcess {
    fn as_ref(&self) -> &Child {
        &self.process
    }
}

impl AsMut<Child> for MasterProcess {
    fn as_mut(&mut self) -> &mut Child {
        &mut self.process
    }
}

#[async_trait(?Send)]
impl RemoteConnection for SshRemoteConnection {
    async fn kill(&self) -> Result<()> {
        let Some(mut process) = self.master_process.lock().take() else {
            return Ok(());
        };
        process.as_mut().kill().ok();
        process.as_mut().status().await?;
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        self.master_process.lock().is_none()
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Ssh(self.socket.connection_options.clone())
    }

    fn shell(&self) -> String {
        self.ssh_shell.clone()
    }

    fn default_system_shell(&self) -> String {
        self.ssh_default_system_shell.clone()
    }

    fn build_command(
        &self,
        input_program: Option<String>,
        input_args: &[String],
        input_env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
        interactive: Interactive,
    ) -> Result<CommandTemplate> {
        let Self {
            ssh_path_style,
            socket,
            ssh_shell_kind,
            ssh_shell,
            ..
        } = self;
        let env = socket.envs.clone();

        if self.ssh_platform.os.is_windows() {
            build_command_windows(
                input_program,
                input_args,
                input_env,
                working_dir,
                port_forward,
                env,
                *ssh_path_style,
                ssh_shell,
                *ssh_shell_kind,
                socket.ssh_command_options(),
                &socket.connection_options.ssh_destination(),
                interactive,
            )
        } else {
            build_command_posix(
                input_program,
                input_args,
                input_env,
                working_dir,
                port_forward,
                env,
                *ssh_path_style,
                ssh_shell,
                *ssh_shell_kind,
                socket.ssh_command_options(),
                &socket.connection_options.ssh_destination(),
                interactive,
            )
        }
    }

    fn build_forward_ports_command(
        &self,
        forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        let Self { socket, .. } = self;
        let mut args = socket.ssh_command_options();
        args.push("-N".into());
        for (local_port, host, remote_port) in forwards {
            args.push("-L".into());
            args.push(format!(
                "{}:{}:{}",
                local_port,
                bracket_ipv6(&host),
                remote_port
            ));
        }
        args.push(socket.connection_options.ssh_destination());
        Ok(CommandTemplate {
            program: "ssh".into(),
            args,
            env: Default::default(),
        })
    }

    fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: RemotePathBuf,
        cx: &App,
    ) -> Task<Result<()>> {
        let dest_path_str = dest_path.to_string();
        let src_path_display = src_path.display().to_string();

        let mut sftp_command = self.build_sftp_command();
        let mut scp_command =
            self.build_scp_command(&src_path, &dest_path_str, Some(&["-C", "-r"]));

        cx.background_spawn(async move {
            // We will try SFTP first, and if that fails, we will fall back to SCP.
            // If SCP fails also, we give up and return an error.
            // The reason we allow a fallback from SFTP to SCP is that if the user has to specify a password,
            // depending on the implementation of SSH stack, SFTP may disable interactive password prompts in batch mode.
            // This is for example the case on Windows as evidenced by this implementation snippet:
            // https://github.com/PowerShell/openssh-portable/blob/b8c08ef9da9450a94a9c5ef717d96a7bd83f3332/sshconnect2.c#L417
            if Self::is_sftp_available().await {
                log::debug!("using SFTP for directory upload");
                let mut child = sftp_command.spawn()?;
                if let Some(mut stdin) = child.stdin.take() {
                    use futures::AsyncWriteExt;
                    let sftp_batch = format!("put -r \"{src_path_display}\" \"{dest_path_str}\"\n");
                    stdin.write_all(sftp_batch.as_bytes()).await?;
                    stdin.flush().await?;
                }

                let output = child.output().await?;
                if output.status.success() {
                    return Ok(());
                }

                let stderr = String::from_utf8_lossy(&output.stderr);
                log::debug!("failed to upload directory via SFTP {src_path_display} -> {dest_path_str}: {stderr}");
            }

            log::debug!("using SCP for directory upload");
            let output = scp_command.output().await?;

            if output.status.success() {
                return Ok(());
            }

            let stderr = String::from_utf8_lossy(&output.stderr);
            log::debug!("failed to upload directory via SCP {src_path_display} -> {dest_path_str}: {stderr}");

            anyhow::bail!(
                "failed to upload directory via SFTP/SCP {} -> {}: {}",
                src_path_display,
                dest_path_str,
                stderr,
            );
        })
    }

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
        const VARS: [&str; 3] = ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"];
        delegate.set_status(Some("Starting proxy"), cx);

        let Some(remote_binary_path) = self.remote_binary_path.clone() else {
            return Task::ready(Err(anyhow!("Remote binary path not set")));
        };

        let mut ssh_command = if self.ssh_platform.os.is_windows() {
            // TODO: Set the `VARS` environment variables, we do not have `env` on windows
            // so this needs a different approach
            let mut proxy_args = vec![];
            proxy_args.push("proxy".to_owned());
            proxy_args.push("--identifier".to_owned());
            proxy_args.push(unique_identifier);

            if reconnect {
                proxy_args.push("--reconnect".to_owned());
            }
            self.socket.ssh_command(
                self.ssh_shell_kind,
                &remote_binary_path.display(self.path_style()),
                &proxy_args,
                false,
            )
        } else {
            let mut proxy_args = vec![];
            for env_var in VARS {
                if let Some(value) = std::env::var(env_var).ok() {
                    proxy_args.push(format!("{env_var}={value}"));
                }
            }
            proxy_args.push(remote_binary_path.display(self.path_style()).into_owned());
            proxy_args.push("proxy".to_owned());
            proxy_args.push("--identifier".to_owned());
            proxy_args.push(unique_identifier);

            if reconnect {
                proxy_args.push("--reconnect".to_owned());
            }
            self.socket
                .ssh_command(self.ssh_shell_kind, "env", &proxy_args, false)
        };

        let ssh_proxy_process = match ssh_command
            // IMPORTANT: we kill this process when we drop the task that uses it.
            .kill_on_drop(true)
            .spawn()
        {
            Ok(process) => process,
            Err(error) => {
                return Task::ready(Err(anyhow!("failed to spawn remote server: {}", error)));
            }
        };

        super::handle_rpc_messages_over_child_process_stdio(
            ssh_proxy_process,
            incoming_tx,
            outgoing_rx,
            connection_activity_tx,
            cx,
        )
    }

    fn path_style(&self) -> PathStyle {
        self.ssh_path_style
    }

    fn has_wsl_interop(&self) -> bool {
        false
    }
}

impl SshRemoteConnection {
    pub(crate) async fn new(
        connection_options: SshConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        use askpass::AskPassResult;

        let destination = connection_options.ssh_destination();

        let temp_dir = tempfile::Builder::new()
            .prefix("zed-ssh-session")
            .tempdir()?;
        let askpass_delegate = askpass::AskPassDelegate::new(cx, {
            let delegate = delegate.clone();
            move |prompt, tx, cx| delegate.ask_password(prompt, tx, cx)
        });

        let mut askpass =
            askpass::AskPassSession::new(cx.background_executor().clone(), askpass_delegate)
                .await?;

        delegate.set_status(Some("Connecting"), cx);

        // Start the master SSH process, which does not do anything except for establish
        // the connection and keep it open, allowing other ssh commands to reuse it
        // via a control socket.
        #[cfg(not(windows))]
        let socket_path = temp_dir.path().join("ssh.sock");

        #[cfg(windows)]
        let mut master_process = MasterProcess::new(
            askpass.script_path().as_ref(),
            connection_options.additional_args(),
            &destination,
        )?;
        #[cfg(not(windows))]
        let mut master_process = MasterProcess::new(
            askpass.script_path().as_ref(),
            connection_options.additional_args(),
            &socket_path,
            &destination,
        )?;

        let result = select_biased! {
            result = askpass.run().fuse() => {
                match result {
                    AskPassResult::CancelledByUser => {
                        master_process.as_mut().kill().ok();
                        anyhow::bail!("SSH connection canceled")
                    }
                    AskPassResult::Timedout => {
                        anyhow::bail!("connecting to host timed out")
                    }
                }
            }
            _ = master_process.wait_connected().fuse() => {
                anyhow::Ok(())
            }
        };

        if let Err(e) = result {
            return Err(e.context("Failed to connect to host"));
        }

        if master_process.as_mut().try_status()?.is_some() {
            let mut output = Vec::new();
            output.clear();
            let mut stderr = master_process.as_mut().stderr.take().unwrap();
            stderr.read_to_end(&mut output).await?;

            let error_message = format!(
                "failed to connect: {}",
                String::from_utf8_lossy(&output).trim()
            );
            anyhow::bail!(error_message);
        }

        #[cfg(not(windows))]
        let socket = SshSocket::new(connection_options, socket_path).await?;
        #[cfg(windows)]
        let socket = SshSocket::new(
            connection_options,
            askpass
                .get_password()
                .or_else(|| askpass::EncryptedPassword::try_from("").ok())
                .context("Failed to fetch askpass password")?,
            cx.background_executor().clone(),
        )
        .await?;
        drop(askpass);

        let is_windows = socket.probe_is_windows().await;
        log::info!("Remote is windows: {}", is_windows);

        let ssh_shell = socket.shell(is_windows).await;
        log::info!("Remote shell discovered: {}", ssh_shell);

        let ssh_shell_kind = ShellKind::new(&ssh_shell, is_windows);
        let ssh_platform = socket.platform(ssh_shell_kind, is_windows).await?;
        log::info!("Remote platform discovered: {:?}", ssh_platform);

        let (ssh_path_style, ssh_default_system_shell) = match ssh_platform.os {
            RemoteOs::Windows => (PathStyle::Windows, ssh_shell.clone()),
            _ => (PathStyle::Posix, String::from("/bin/sh")),
        };

        let mut this = Self {
            socket,
            master_process: Mutex::new(Some(master_process)),
            _temp_dir: temp_dir,
            remote_binary_path: None,
            ssh_path_style,
            ssh_platform,
            ssh_shell,
            ssh_shell_kind,
            ssh_default_system_shell,
        };

        let (release_channel, version) =
            cx.update(|cx| (ReleaseChannel::global(cx), AppVersion::global(cx)));
        this.remote_binary_path = Some(
            this.ensure_server_binary(&delegate, release_channel, version, cx)
                .await?,
        );

        Ok(this)
    }

    async fn ensure_server_binary(
        &self,
        delegate: &Arc<dyn RemoteClientDelegate>,
        release_channel: ReleaseChannel,
        version: Version,
        cx: &mut AsyncApp,
    ) -> Result<Arc<RelPath>> {
        let version_str = match release_channel {
            ReleaseChannel::Dev => "build".to_string(),
            _ => version.to_string(),
        };
        let binary_name = format!(
            "zed-remote-server-{}-{}{}",
            release_channel.dev_name(),
            version_str,
            if self.ssh_platform.os.is_windows() {
                ".exe"
            } else {
                ""
            }
        );
        let dst_path =
            paths::remote_server_dir_relative().join(RelPath::unix(&binary_name).unwrap());

        let binary_exists_on_server = self
            .socket
            .run_command(
                self.ssh_shell_kind,
                &dst_path.display(self.path_style()),
                &["version"],
                true,
            )
            .await
            .is_ok();

        #[cfg(any(debug_assertions, feature = "build-remote-server-binary"))]
        if let Some(remote_server_path) = super::build_remote_server_from_source(
            &self.ssh_platform,
            delegate.as_ref(),
            binary_exists_on_server,
            cx,
        )
        .await?
        {
            let tmp_path = paths::remote_server_dir_relative().join(
                RelPath::unix(&format!(
                    "download-{}-{}",
                    std::process::id(),
                    remote_server_path.file_name().unwrap().to_string_lossy()
                ))
                .unwrap(),
            );
            self.upload_local_server_binary(&remote_server_path, &tmp_path, delegate, cx)
                .await?;
            self.extract_server_binary(&dst_path, &tmp_path, delegate, cx)
                .await?;
            return Ok(dst_path);
        }

        if binary_exists_on_server {
            return Ok(dst_path);
        }

        let wanted_version = cx.update(|cx| match release_channel {
            ReleaseChannel::Nightly => Ok(None),
            ReleaseChannel::Dev => {
                anyhow::bail!(
                    "ZED_BUILD_REMOTE_SERVER is not set and no remote server exists at ({:?})",
                    dst_path
                )
            }
            _ => Ok(Some(AppVersion::global(cx))),
        })?;

        let tmp_path_compressed = remote_server_dir_relative().join(
            RelPath::unix(&format!(
                "{}-download-{}.{}",
                binary_name,
                std::process::id(),
                if self.ssh_platform.os.is_windows() {
                    "zip"
                } else {
                    "gz"
                }
            ))
            .unwrap(),
        );
        if !self.socket.connection_options.upload_binary_over_ssh
            && let Some(url) = delegate
                .get_download_url(
                    self.ssh_platform,
                    release_channel,
                    wanted_version.clone(),
                    cx,
                )
                .await?
        {
            match self
                .download_binary_on_server(&url, &tmp_path_compressed, delegate, cx)
                .await
            {
                Ok(_) => {
                    self.extract_server_binary(&dst_path, &tmp_path_compressed, delegate, cx)
                        .await
                        .context("extracting server binary")?;
                    return Ok(dst_path);
                }
                Err(e) => {
                    log::error!(
                        "Failed to download binary on server, attempting to download locally and then upload it the server: {e:#}",
                    )
                }
            }
        }

        let src_path = delegate
            .download_server_binary_locally(
                self.ssh_platform,
                release_channel,
                wanted_version.clone(),
                cx,
            )
            .await
            .context("downloading server binary locally")?;
        self.upload_local_server_binary(&src_path, &tmp_path_compressed, delegate, cx)
            .await
            .context("uploading server binary")?;
        self.extract_server_binary(&dst_path, &tmp_path_compressed, delegate, cx)
            .await
            .context("extracting server binary")?;
        Ok(dst_path)
    }

    async fn download_binary_on_server(
        &self,
        url: &str,
        tmp_path: &RelPath,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        if let Some(parent) = tmp_path.parent() {
            let res = self
                .socket
                .run_command(
                    self.ssh_shell_kind,
                    "mkdir",
                    &["-p", parent.display(self.path_style()).as_ref()],
                    true,
                )
                .await;
            if !self.ssh_platform.os.is_windows() {
                // mkdir fails on windows if the path already exists ...
                res?;
            }
        }

        delegate.set_status(Some("Downloading remote development server on host"), cx);

        let connection_timeout = self
            .socket
            .connection_options
            .connection_timeout
            .unwrap_or(10)
            .to_string();

        match self
            .socket
            .run_command(
                self.ssh_shell_kind,
                "curl",
                &[
                    "-f",
                    "-L",
                    "--connect-timeout",
                    &connection_timeout,
                    url,
                    "-o",
                    &tmp_path.display(self.path_style()),
                ],
                true,
            )
            .await
        {
            Ok(_) => {}
            Err(e) => {
                if self
                    .socket
                    .run_command(self.ssh_shell_kind, "which", &["curl"], true)
                    .await
                    .is_ok()
                {
                    return Err(e);
                }

                log::info!("curl is not available, trying wget");
                match self
                    .socket
                    .run_command(
                        self.ssh_shell_kind,
                        "wget",
                        &[
                            "--connect-timeout",
                            &connection_timeout,
                            "--tries",
                            "1",
                            url,
                            "-O",
                            &tmp_path.display(self.path_style()),
                        ],
                        true,
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        if self
                            .socket
                            .run_command(self.ssh_shell_kind, "which", &["wget"], true)
                            .await
                            .is_ok()
                        {
                            return Err(e);
                        } else {
                            anyhow::bail!("Neither curl nor wget is available");
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn upload_local_server_binary(
        &self,
        src_path: &Path,
        tmp_path: &RelPath,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        if let Some(parent) = tmp_path.parent() {
            let res = self
                .socket
                .run_command(
                    self.ssh_shell_kind,
                    "mkdir",
                    &["-p", parent.display(self.path_style()).as_ref()],
                    true,
                )
                .await;
            if !self.ssh_platform.os.is_windows() {
                // mkdir fails on windows if the path already exists ...
                res?;
            }
        }

        let src_stat = fs::metadata(&src_path)
            .await
            .with_context(|| format!("failed to get metadata for {:?}", src_path))?;
        let size = src_stat.len();

        let t0 = Instant::now();
        delegate.set_status(Some("Uploading remote development server"), cx);
        log::info!(
            "uploading remote development server to {:?} ({}kb)",
            tmp_path,
            size / 1024
        );
        self.upload_file(src_path, tmp_path)
            .await
            .context("failed to upload server binary")?;
        log::info!("uploaded remote development server in {:?}", t0.elapsed());
        Ok(())
    }

    async fn extract_server_binary(
        &self,
        dst_path: &RelPath,
        tmp_path: &RelPath,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Extracting remote development server"), cx);

        if self.ssh_platform.os.is_windows() {
            self.extract_server_binary_windows(dst_path, tmp_path).await
        } else {
            self.extract_server_binary_posix(dst_path, tmp_path).await
        }
    }

    async fn extract_server_binary_posix(
        &self,
        dst_path: &RelPath,
        tmp_path: &RelPath,
    ) -> Result<()> {
        let shell_kind = ShellKind::Posix;
        let server_mode = 0o755;
        let orig_tmp_path = tmp_path.display(self.path_style());
        let server_mode = format!("{:o}", server_mode);
        let server_mode = shell_kind
            .try_quote(&server_mode)
            .context("shell quoting")?;
        let dst_path = dst_path.display(self.path_style());
        let dst_path = shell_kind.try_quote(&dst_path).context("shell quoting")?;
        let script = if let Some(tmp_path) = orig_tmp_path.strip_suffix(".gz") {
            let orig_tmp_path = shell_kind
                .try_quote(&orig_tmp_path)
                .context("shell quoting")?;
            let tmp_path = shell_kind.try_quote(&tmp_path).context("shell quoting")?;
            format!(
                "gunzip -f {orig_tmp_path} && chmod {server_mode} {tmp_path} && mv {tmp_path} {dst_path}",
            )
        } else {
            let orig_tmp_path = shell_kind
                .try_quote(&orig_tmp_path)
                .context("shell quoting")?;
            format!("chmod {server_mode} {orig_tmp_path} && mv {orig_tmp_path} {dst_path}",)
        };
        let args = shell_kind.args_for_shell(false, script.to_string());
        self.socket
            .run_command(self.ssh_shell_kind, "sh", &args, true)
            .await?;
        Ok(())
    }

    async fn extract_server_binary_windows(
        &self,
        dst_path: &RelPath,
        tmp_path: &RelPath,
    ) -> Result<()> {
        let shell_kind = ShellKind::Pwsh;
        let orig_tmp_path = tmp_path.display(self.path_style());
        let dst_path = dst_path.display(self.path_style());
        let dst_path = shell_kind.try_quote(&dst_path).context("shell quoting")?;

        let script = if let Some(tmp_path) = orig_tmp_path.strip_suffix(".zip") {
            let orig_tmp_path = shell_kind
                .try_quote(&orig_tmp_path)
                .context("shell quoting")?;
            let tmp_path = shell_kind.try_quote(tmp_path).context("shell quoting")?;
            let tmp_exe_path = format!("{tmp_path}\\remote_server.exe");
            let tmp_exe_path = shell_kind
                .try_quote(&tmp_exe_path)
                .context("shell quoting")?;
            format!(
                "Expand-Archive -Force -Path {orig_tmp_path} -DestinationPath {tmp_path} -ErrorAction Stop; Move-Item -Force {tmp_exe_path} {dst_path}; Remove-Item -Force {tmp_path} -Recurse; Remove-Item -Force {orig_tmp_path}",
            )
        } else {
            let orig_tmp_path = shell_kind
                .try_quote(&orig_tmp_path)
                .context("shell quoting")?;
            format!("Move-Item -Force {orig_tmp_path} {dst_path}")
        };

        let args = shell_kind.args_for_shell(false, script);
        self.socket
            .run_command(self.ssh_shell_kind, "powershell", &args, true)
            .await?;
        Ok(())
    }

    fn build_scp_command(
        &self,
        src_path: &Path,
        dest_path_str: &str,
        args: Option<&[&str]>,
    ) -> util::command::Command {
        let mut command = util::command::new_command("scp");
        self.socket.ssh_options(&mut command, false).args(
            self.socket
                .connection_options
                .port
                .map(|port| vec!["-P".to_string(), port.to_string()])
                .unwrap_or_default(),
        );
        if let Some(args) = args {
            command.args(args);
        }
        command.arg(src_path).arg(format!(
            "{}:{}",
            scp_destination(&self.socket.connection_options),
            dest_path_str
        ));
        command
    }

    fn build_sftp_command(&self) -> util::command::Command {
        let mut command = util::command::new_command("sftp");
        self.socket.ssh_options(&mut command, false).args(
            self.socket
                .connection_options
                .port
                .map(|port| vec!["-P".to_string(), port.to_string()])
                .unwrap_or_default(),
        );
        command.arg("-b").arg("-");
        command.arg(scp_destination(&self.socket.connection_options));
        command.stdin(Stdio::piped());
        command
    }

    async fn upload_file(&self, src_path: &Path, dest_path: &RelPath) -> Result<()> {
        log::debug!("uploading file {:?} to {:?}", src_path, dest_path);

        let src_path_display = src_path.display().to_string();
        let dest_path_str = dest_path.display(self.path_style());

        // We will try SFTP first, and if that fails, we will fall back to SCP.
        // If SCP fails also, we give up and return an error.
        // The reason we allow a fallback from SFTP to SCP is that if the user has to specify a password,
        // depending on the implementation of SSH stack, SFTP may disable interactive password prompts in batch mode.
        // This is for example the case on Windows as evidenced by this implementation snippet:
        // https://github.com/PowerShell/openssh-portable/blob/b8c08ef9da9450a94a9c5ef717d96a7bd83f3332/sshconnect2.c#L417
        if Self::is_sftp_available().await {
            log::debug!("using SFTP for file upload");
            let mut command = self.build_sftp_command();
            let sftp_batch = format!("put {src_path_display} {dest_path_str}\n");

            let mut child = command.spawn()?;
            if let Some(mut stdin) = child.stdin.take() {
                use futures::AsyncWriteExt;
                stdin.write_all(sftp_batch.as_bytes()).await?;
                stdin.flush().await?;
            }

            let output = child.output().await?;
            if output.status.success() {
                return Ok(());
            }

            let stderr = String::from_utf8_lossy(&output.stderr);
            log::debug!(
                "failed to upload file via SFTP {src_path_display} -> {dest_path_str}: {stderr}"
            );
        }

        log::debug!("using SCP for file upload");
        let mut command = self.build_scp_command(src_path, &dest_path_str, None);
        let output = command.output().await?;

        if output.status.success() {
            return Ok(());
        }

        let stderr = String::from_utf8_lossy(&output.stderr);
        log::debug!(
            "failed to upload file via SCP {src_path_display} -> {dest_path_str}: {stderr}",
        );
        anyhow::bail!(
            "failed to upload file via STFP/SCP {} -> {}: {}",
            src_path_display,
            dest_path_str,
            stderr,
        );
    }

    async fn is_sftp_available() -> bool {
        which::which("sftp").is_ok()
    }
}

impl SshSocket {
    #[cfg(not(windows))]
    async fn new(options: SshConnectionOptions, socket_path: PathBuf) -> Result<Self> {
        Ok(Self {
            connection_options: options,
            envs: HashMap::default(),
            socket_path,
        })
    }

    #[cfg(windows)]
    async fn new(
        options: SshConnectionOptions,
        password: askpass::EncryptedPassword,
        executor: gpui::BackgroundExecutor,
    ) -> Result<Self> {
        let mut envs = HashMap::default();
        let get_password =
            move |_| Task::ready(std::ops::ControlFlow::Continue(Ok(password.clone())));

        let _proxy = askpass::PasswordProxy::new(Box::new(get_password), executor).await?;
        envs.insert("SSH_ASKPASS_REQUIRE".into(), "force".into());
        envs.insert(
            "SSH_ASKPASS".into(),
            _proxy.script_path().as_ref().display().to_string(),
        );

        Ok(Self {
            connection_options: options,
            envs,
            _proxy,
        })
    }

    // :WARNING: ssh unquotes arguments when executing on the remote :WARNING:
    // e.g. $ ssh host sh -c 'ls -l' is equivalent to $ ssh host sh -c ls -l
    // and passes -l as an argument to sh, not to ls.
    // Furthermore, some setups (e.g. Coder) will change directory when SSH'ing
    // into a machine. You must use `cd` to get back to $HOME.
    // You need to do it like this: $ ssh host "cd; sh -c 'ls -l /tmp'"
    fn ssh_command(
        &self,
        shell_kind: ShellKind,
        program: &str,
        args: &[impl AsRef<str>],
        allow_pseudo_tty: bool,
    ) -> util::command::Command {
        let mut command = util::command::new_command("ssh");
        let program = shell_kind.prepend_command_prefix(program);
        let mut to_run = shell_kind
            .try_quote_prefix_aware(&program)
            .expect("shell quoting")
            .into_owned();
        for arg in args {
            // We're trying to work with: sh, bash, zsh, fish, tcsh, ...?
            debug_assert!(
                !arg.as_ref().contains('\n'),
                "multiline arguments do not work in all shells"
            );
            to_run.push(' ');
            to_run.push_str(&shell_kind.try_quote(arg.as_ref()).expect("shell quoting"));
        }
        let to_run = if shell_kind == ShellKind::Cmd {
            to_run // 'cd' prints the current directory in CMD
        } else {
            let separator = shell_kind.sequential_commands_separator();
            format!("cd{separator} {to_run}")
        };
        self.ssh_options(&mut command, true)
            .arg(self.connection_options.ssh_destination());
        if !allow_pseudo_tty {
            command.arg("-T");
        }
        command.arg(to_run);
        log::debug!("ssh {:?}", command);
        command
    }

    async fn run_command(
        &self,
        shell_kind: ShellKind,
        program: &str,
        args: &[impl AsRef<str>],
        allow_pseudo_tty: bool,
    ) -> Result<String> {
        let mut command = self.ssh_command(shell_kind, program, args, allow_pseudo_tty);
        let output = command.output().await?;
        log::debug!("{:?}: {:?}", command, output);
        anyhow::ensure!(
            output.status.success(),
            "failed to run command {command:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn ssh_options<'a>(
        &self,
        command: &'a mut util::command::Command,
        include_port_forwards: bool,
    ) -> &'a mut util::command::Command {
        let args = if include_port_forwards {
            self.connection_options.additional_args()
        } else {
            self.connection_options.additional_args_for_scp()
        };

        let cmd = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(args);

        if cfg!(windows) {
            cmd.envs(self.envs.clone());
        }
        #[cfg(not(windows))]
        {
            cmd.args(["-o", "ControlMaster=no", "-o"])
                .arg(format!("ControlPath={}", self.socket_path.display()));
        }
        cmd
    }

    // Returns the SSH command-line options (without the destination) for building commands.
    // On Linux, this includes the ControlPath option to reuse the existing connection.
    // Note: The destination must be added separately after all options to ensure proper
    // SSH command structure: ssh [options] destination [command]
    fn ssh_command_options(&self) -> Vec<String> {
        let arguments = self.connection_options.additional_args();
        #[cfg(not(windows))]
        let arguments = {
            let mut args = arguments;
            args.extend(vec![
                "-o".to_string(),
                "ControlMaster=no".to_string(),
                "-o".to_string(),
                format!("ControlPath={}", self.socket_path.display()),
            ]);
            args
        };
        arguments
    }

    async fn platform(&self, shell: ShellKind, is_windows: bool) -> Result<RemotePlatform> {
        if is_windows {
            self.platform_windows(shell).await
        } else {
            self.platform_posix(shell).await
        }
    }

    async fn platform_posix(&self, shell: ShellKind) -> Result<RemotePlatform> {
        let output = self
            .run_command(shell, "uname", &["-sm"], false)
            .await
            .context("Failed to run 'uname -sm' to determine platform")?;
        parse_platform(&output)
    }

    async fn platform_windows(&self, shell: ShellKind) -> Result<RemotePlatform> {
        let output = self
            .run_command(
                shell,
                "cmd.exe",
                &["/c", "echo", "%PROCESSOR_ARCHITECTURE%"],
                false,
            )
            .await
            .context(
                "Failed to run 'echo %PROCESSOR_ARCHITECTURE%' to determine Windows architecture",
            )?;

        Ok(RemotePlatform {
            os: RemoteOs::Windows,
            arch: match output.trim() {
                "AMD64" => RemoteArch::X86_64,
                "ARM64" => RemoteArch::Aarch64,
                arch => anyhow::bail!(
                    "Prebuilt remote servers are not yet available for windows-{arch}. See https://zed.dev/docs/remote-development"
                ),
            },
        })
    }

    /// Probes whether the remote host is running Windows.
    ///
    /// This is done by attempting to run a simple Windows-specific command.
    /// If it succeeds and returns Windows-like output, we assume it's Windows.
    async fn probe_is_windows(&self) -> bool {
        match self
            .run_command(ShellKind::Cmd, "cmd.exe", &["/c", "ver"], false)
            .await
        {
            // Windows 'ver' command outputs something like "Microsoft Windows [Version 10.0.19045.5011]"
            Ok(output) => output.trim().contains("indows"),
            Err(_) => false,
        }
    }

    async fn shell(&self, is_windows: bool) -> String {
        if is_windows {
            self.shell_windows().await
        } else {
            self.shell_posix().await
        }
    }

    async fn shell_posix(&self) -> String {
        const DEFAULT_SHELL: &str = "sh";
        match self
            .run_command(ShellKind::Posix, "sh", &["-c", "echo $SHELL"], false)
            .await
        {
            Ok(output) => parse_shell(&output, DEFAULT_SHELL),
            Err(e) => {
                log::error!("Failed to detect remote shell: {e}");
                DEFAULT_SHELL.to_owned()
            }
        }
    }

    async fn shell_windows(&self) -> String {
        const DEFAULT_SHELL: &str = "cmd.exe";

        // We detect the shell used by the SSH session by running the following command in PowerShell:
        // (Get-CimInstance Win32_Process -Filter "ProcessId = $((Get-CimInstance Win32_Process -Filter ProcessId=$PID).ParentProcessId)").Name
        // This prints the name of PowerShell's parent process (which will be the shell that SSH launched).
        // We pass it as a Base64 encoded string since we don't yet know how to correctly quote that command.
        // (We'd need to know what the shell is to do that...)
        match self
            .run_command(
                ShellKind::Cmd,
                "powershell",
                &[
                    "-E",
                    "KABHAGUAdAAtAEMAaQBtAEkAbgBzAHQAYQBuAGMAZQAgAFcAaQBuADMAMgBfAFAAcgBvAGMAZQBzAHMAIAAtAEYAaQBsAHQAZQByACAAIgBQAHIAbwBjAGUAcwBzAEkAZAAgAD0AIAAkACgAKABHAGUAdAAtAEMAaQBtAEkAbgBzAHQAYQBuAGMAZQAgAFcAaQBuADMAMgBfAFAAcgBvAGMAZQBzAHMAIAAtAEYAaQBsAHQAZQByACAAUAByAG8AYwBlAHMAcwBJAGQAPQAkAFAASQBEACkALgBQAGEAcgBlAG4AdABQAHIAbwBjAGUAcwBzAEkAZAApACIAKQAuAE4AYQBtAGUA",
                ],
                false,
            )
            .await
        {
            Ok(output) => parse_shell(&output, DEFAULT_SHELL),
            Err(e) => {
                log::error!("Failed to detect remote shell: {e}");
                DEFAULT_SHELL.to_owned()
            }
        }
    }
}

fn build_command_posix(
    input_program: Option<String>,
    input_args: &[String],
    input_env: &HashMap<String, String>,
    working_dir: Option<String>,
    port_forward: Option<(u16, String, u16)>,
    ssh_env: HashMap<String, String>,
    ssh_path_style: PathStyle,
    ssh_shell: &str,
    ssh_shell_kind: ShellKind,
    ssh_options: Vec<String>,
    ssh_destination: &str,
    interactive: Interactive,
) -> Result<CommandTemplate> {
    use std::fmt::Write as _;

    let mut exec = String::new();
    if let Some(working_dir) = working_dir {
        let working_dir = RemotePathBuf::new(working_dir, ssh_path_style).to_string();

        // shlex will wrap the command in single quotes (''), disabling ~ expansion,
        // replace with something that works
        const TILDE_PREFIX: &'static str = "~/";
        if working_dir.starts_with(TILDE_PREFIX) {
            let working_dir = working_dir.trim_start_matches("~").trim_start_matches("/");
            write!(
                exec,
                "cd \"$HOME/{working_dir}\" {} ",
                ssh_shell_kind.sequential_and_commands_separator()
            )?;
        } else {
            write!(
                exec,
                "cd \"{working_dir}\" {} ",
                ssh_shell_kind.sequential_and_commands_separator()
            )?;
        }
    } else {
        write!(
            exec,
            "cd {} ",
            ssh_shell_kind.sequential_and_commands_separator()
        )?;
    };
    write!(exec, "exec env ")?;

    for (k, v) in input_env.iter() {
        let assignment = format!("{k}={v}");
        let assignment = ssh_shell_kind
            .try_quote(&assignment)
            .context("shell quoting")?;
        write!(exec, "{assignment} ")?;
    }

    if let Some(input_program) = input_program {
        write!(
            exec,
            "{}",
            ssh_shell_kind
                .try_quote_prefix_aware(&input_program)
                .context("shell quoting")?
        )?;
        for arg in input_args {
            let arg = ssh_shell_kind.try_quote(&arg).context("shell quoting")?;
            write!(exec, " {}", &arg)?;
        }
    } else {
        write!(exec, "{ssh_shell} -l")?;
    };

    let mut args = Vec::new();
    args.extend(ssh_options);

    if let Some((local_port, host, remote_port)) = port_forward {
        args.push("-L".into());
        args.push(format!(
            "{}:{}:{}",
            local_port,
            bracket_ipv6(&host),
            remote_port
        ));
    }

    // -q suppresses the "Connection to ... closed." message that SSH prints when
    // the connection terminates with -t (pseudo-terminal allocation)
    args.push("-q".into());
    match interactive {
        // -t forces pseudo-TTY allocation (for interactive use)
        Interactive::Yes => args.push("-t".into()),
        // -T disables pseudo-TTY allocation (for non-interactive piped stdio)
        Interactive::No => args.push("-T".into()),
    }
    // The destination must come after all options but before the command
    args.push(ssh_destination.into());
    args.push(exec);

    Ok(CommandTemplate {
        program: "ssh".into(),
        args,
        env: ssh_env,
    })
}

fn build_command_windows(
    input_program: Option<String>,
    input_args: &[String],
    _input_env: &HashMap<String, String>,
    working_dir: Option<String>,
    port_forward: Option<(u16, String, u16)>,
    ssh_env: HashMap<String, String>,
    ssh_path_style: PathStyle,
    ssh_shell: &str,
    _ssh_shell_kind: ShellKind,
    ssh_options: Vec<String>,
    ssh_destination: &str,
    interactive: Interactive,
) -> Result<CommandTemplate> {
    use base64::Engine as _;
    use std::fmt::Write as _;

    let mut exec = String::new();
    let shell_kind = ShellKind::PowerShell;

    if let Some(working_dir) = working_dir {
        let working_dir = RemotePathBuf::new(working_dir, ssh_path_style).to_string();

        write!(
            exec,
            "Set-Location -Path {} {} ",
            shell_kind
                .try_quote(&working_dir)
                .context("shell quoting")?,
            shell_kind.sequential_and_commands_separator()
        )?;
    }

    // Windows OpenSSH has an 8K character limit for command lines. Sending a lot of environment variables easily puts us over the limit.
    // Until we have a better solution for this, we just won't set environment variables for now.
    // for (k, v) in input_env.iter() {
    //     write!(
    //         exec,
    //         "$env:{}={} {} ",
    //         k,
    //         shell_kind.try_quote(v).context("shell quoting")?,
    //         shell_kind.sequential_and_commands_separator()
    //     )?;
    // }

    if let Some(input_program) = input_program {
        write!(
            exec,
            "{}",
            shell_kind
                .try_quote_prefix_aware(&shell_kind.prepend_command_prefix(&input_program))
                .context("shell quoting")?
        )?;
        for arg in input_args {
            let arg = shell_kind.try_quote(arg).context("shell quoting")?;
            write!(exec, " {}", &arg)?;
        }
    } else {
        // Launch an interactive shell session
        write!(exec, "{ssh_shell}")?;
    };

    let mut args = Vec::new();
    args.extend(ssh_options);

    if let Some((local_port, host, remote_port)) = port_forward {
        args.push("-L".into());
        args.push(format!(
            "{}:{}:{}",
            local_port,
            bracket_ipv6(&host),
            remote_port
        ));
    }

    // -q suppresses the "Connection to ... closed." message that SSH prints when
    // the connection terminates with -t (pseudo-terminal allocation)
    args.push("-q".into());
    match interactive {
        // -t forces pseudo-TTY allocation (for interactive use)
        Interactive::Yes => args.push("-t".into()),
        // -T disables pseudo-TTY allocation (for non-interactive piped stdio)
        Interactive::No => args.push("-T".into()),
    }

    // The destination must come after all options but before the command
    args.push(ssh_destination.into());

    // Windows OpenSSH server incorrectly escapes the command string when the PTY is used.
    // The simplest way to work around this is to use a base64 encoded command, which doesn't require escaping.
    let utf16_bytes: Vec<u16> = exec.encode_utf16().collect();
    let byte_slice: Vec<u8> = utf16_bytes.iter().flat_map(|&u| u.to_le_bytes()).collect();
    let base64_encoded = base64::engine::general_purpose::STANDARD.encode(&byte_slice);

    args.push(format!("powershell.exe -E {}", base64_encoded));

    Ok(CommandTemplate {
        program: "ssh".into(),
        args,
        env: ssh_env,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_command() -> Result<()> {
        let mut input_env = HashMap::default();
        input_env.insert("INPUT_VA".to_string(), "val".to_string());
        let mut env = HashMap::default();
        env.insert("SSH_VAR".to_string(), "ssh-val".to_string());

        // Test non-interactive command (interactive=false should use -T)
        let command = build_command_posix(
            Some("remote_program".to_string()),
            &["arg1".to_string(), "arg2".to_string()],
            &input_env,
            Some("~/work".to_string()),
            None,
            env.clone(),
            PathStyle::Posix,
            "/bin/bash",
            ShellKind::Posix,
            vec!["-o".to_string(), "ControlMaster=auto".to_string()],
            "user@host",
            Interactive::No,
        )?;
        assert_eq!(command.program, "ssh");
        // Should contain -T for non-interactive
        assert!(command.args.iter().any(|arg| arg == "-T"));
        assert!(!command.args.iter().any(|arg| arg == "-t"));

        // Test interactive command (interactive=true should use -t)
        let command = build_command_posix(
            Some("remote_program".to_string()),
            &["arg1".to_string(), "arg2".to_string()],
            &input_env,
            Some("~/work".to_string()),
            None,
            env.clone(),
            PathStyle::Posix,
            "/bin/fish",
            ShellKind::Fish,
            vec!["-p".to_string(), "2222".to_string()],
            "user@host",
            Interactive::Yes,
        )?;

        assert_eq!(command.program, "ssh");
        assert_eq!(
            command.args.iter().map(String::as_str).collect::<Vec<_>>(),
            [
                "-p",
                "2222",
                "-q",
                "-t",
                "user@host",
                "cd \"$HOME/work\" && exec env 'INPUT_VA=val' remote_program arg1 arg2"
            ]
        );
        assert_eq!(command.env, env);

        let mut input_env = HashMap::default();
        input_env.insert("INPUT_VA".to_string(), "val".to_string());
        let mut env = HashMap::default();
        env.insert("SSH_VAR".to_string(), "ssh-val".to_string());

        let command = build_command_posix(
            None,
            &[],
            &input_env,
            None,
            Some((1, "foo".to_owned(), 2)),
            env.clone(),
            PathStyle::Posix,
            "/bin/fish",
            ShellKind::Fish,
            vec!["-p".to_string(), "2222".to_string()],
            "user@host",
            Interactive::Yes,
        )?;

        assert_eq!(command.program, "ssh");
        assert_eq!(
            command.args.iter().map(String::as_str).collect::<Vec<_>>(),
            [
                "-p",
                "2222",
                "-L",
                "1:foo:2",
                "-q",
                "-t",
                "user@host",
                "cd && exec env 'INPUT_VA=val' /bin/fish -l"
            ]
        );
        assert_eq!(command.env, env);

        Ok(())
    }

    #[test]
    fn test_build_command_quotes_env_assignment() -> Result<()> {
        let mut input_env = HashMap::default();
        input_env.insert("ZED$(echo foo)".to_string(), "value".to_string());

        let command = build_command_posix(
            Some("remote_program".to_string()),
            &[],
            &input_env,
            None,
            None,
            HashMap::default(),
            PathStyle::Posix,
            "/bin/bash",
            ShellKind::Posix,
            vec![],
            "user@host",
            Interactive::No,
        )?;

        let remote_command = command
            .args
            .last()
            .context("missing remote command argument")?;
        assert!(
            remote_command.contains("exec env 'ZED$(echo foo)=value' remote_program"),
            "expected env assignment to be quoted, got: {remote_command}"
        );

        Ok(())
    }

    #[test]
    fn test_build_command_with_ipv6_port_forward() -> Result<()> {
        let command = build_command_posix(
            None,
            &[],
            &HashMap::default(),
            None,
            Some((8080, "::1".to_owned(), 80)),
            HashMap::default(),
            PathStyle::Posix,
            "/bin/bash",
            ShellKind::Posix,
            vec![],
            "user@host",
            Interactive::No,
        )?;

        assert!(
            command.args.iter().any(|arg| arg == "8080:[::1]:80"),
            "expected bracketed IPv6 in port forward arg: {:?}",
            command.args
        );

        Ok(())
    }
}
