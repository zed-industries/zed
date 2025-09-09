use crate::{
    RemoteClientDelegate, RemotePlatform,
    remote_client::{CommandTemplate, RemoteConnection, RemoteConnectionOptions},
};
use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{
    AsyncReadExt as _, FutureExt as _,
    channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender},
    select_biased,
};
use gpui::{App, AppContext as _, AsyncApp, SemanticVersion, Task};
use itertools::Itertools;
use parking_lot::Mutex;
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use rpc::proto::Envelope;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smol::{
    fs,
    process::{self, Child, Stdio},
};
use std::{
    iter,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};
use tempfile::TempDir;
use util::paths::{PathStyle, RemotePathBuf};

pub(crate) struct SshRemoteConnection {
    socket: SshSocket,
    master_process: Mutex<Option<Child>>,
    remote_binary_path: Option<RemotePathBuf>,
    ssh_platform: RemotePlatform,
    ssh_path_style: PathStyle,
    ssh_shell: String,
    _temp_dir: TempDir,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SshConnectionOptions {
    pub host: String,
    pub username: Option<String>,
    pub port: Option<u16>,
    pub password: Option<String>,
    pub args: Option<Vec<String>>,
    pub port_forwards: Option<Vec<SshPortForwardOption>>,

    pub nickname: Option<String>,
    pub upload_binary_over_ssh: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema)]
pub struct SshPortForwardOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_host: Option<String>,
    pub local_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_host: Option<String>,
    pub remote_port: u16,
}

#[derive(Clone)]
struct SshSocket {
    connection_options: SshConnectionOptions,
    #[cfg(not(target_os = "windows"))]
    socket_path: PathBuf,
    envs: HashMap<String, String>,
}

macro_rules! shell_script {
    ($fmt:expr, $($name:ident = $arg:expr),+ $(,)?) => {{
        format!(
            $fmt,
            $(
                $name = shlex::try_quote($arg).unwrap()
            ),+
        )
    }};
}

#[async_trait(?Send)]
impl RemoteConnection for SshRemoteConnection {
    async fn kill(&self) -> Result<()> {
        let Some(mut process) = self.master_process.lock().take() else {
            return Ok(());
        };
        process.kill().ok();
        process.status().await?;
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

    fn build_command(
        &self,
        input_program: Option<String>,
        input_args: &[String],
        input_env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        use std::fmt::Write as _;

        let mut script = String::new();
        if let Some(working_dir) = working_dir {
            let working_dir =
                RemotePathBuf::new(working_dir.into(), self.ssh_path_style).to_string();

            // shlex will wrap the command in single quotes (''), disabling ~ expansion,
            // replace ith with something that works
            const TILDE_PREFIX: &'static str = "~/";
            let working_dir = if working_dir.starts_with(TILDE_PREFIX) {
                let working_dir = working_dir.trim_start_matches("~").trim_start_matches("/");
                format!("$HOME/{working_dir}")
            } else {
                working_dir
            };
            write!(&mut script, "cd \"{working_dir}\"; ",).unwrap();
        } else {
            write!(&mut script, "cd; ").unwrap();
        };

        for (k, v) in input_env.iter() {
            if let Some((k, v)) = shlex::try_quote(k).ok().zip(shlex::try_quote(v).ok()) {
                write!(&mut script, "{}={} ", k, v).unwrap();
            }
        }

        let shell = &self.ssh_shell;

        if let Some(input_program) = input_program {
            let command = shlex::try_quote(&input_program)?;
            script.push_str(&command);
            for arg in input_args {
                let arg = shlex::try_quote(&arg)?;
                script.push_str(" ");
                script.push_str(&arg);
            }
        } else {
            write!(&mut script, "exec {shell} -l").unwrap();
        };

        let shell_invocation = format!("{shell} -c {}", shlex::try_quote(&script).unwrap());

        let mut args = Vec::new();
        args.extend(self.socket.ssh_args());

        if let Some((local_port, host, remote_port)) = port_forward {
            args.push("-L".into());
            args.push(format!("{local_port}:{host}:{remote_port}"));
        }

        args.push("-t".into());
        args.push(shell_invocation);
        Ok(CommandTemplate {
            program: "ssh".into(),
            args,
            env: self.socket.envs.clone(),
        })
    }

    fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: RemotePathBuf,
        cx: &App,
    ) -> Task<Result<()>> {
        let mut command = util::command::new_smol_command("scp");
        let output = self
            .socket
            .ssh_options(&mut command)
            .args(
                self.socket
                    .connection_options
                    .port
                    .map(|port| vec!["-P".to_string(), port.to_string()])
                    .unwrap_or_default(),
            )
            .arg("-C")
            .arg("-r")
            .arg(&src_path)
            .arg(format!(
                "{}:{}",
                self.socket.connection_options.scp_url(),
                dest_path
            ))
            .output();

        cx.background_spawn(async move {
            let output = output.await?;

            anyhow::ensure!(
                output.status.success(),
                "failed to upload directory {} -> {}: {}",
                src_path.display(),
                dest_path.to_string(),
                String::from_utf8_lossy(&output.stderr)
            );

            Ok(())
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
        delegate.set_status(Some("Starting proxy"), cx);

        let Some(remote_binary_path) = self.remote_binary_path.clone() else {
            return Task::ready(Err(anyhow!("Remote binary path not set")));
        };

        let mut start_proxy_command = shell_script!(
            "exec {binary_path} proxy --identifier {identifier}",
            binary_path = &remote_binary_path.to_string(),
            identifier = &unique_identifier,
        );

        for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
            if let Some(value) = std::env::var(env_var).ok() {
                start_proxy_command = format!(
                    "{}={} {} ",
                    env_var,
                    shlex::try_quote(&value).unwrap(),
                    start_proxy_command,
                );
            }
        }

        if reconnect {
            start_proxy_command.push_str(" --reconnect");
        }

        let ssh_proxy_process = match self
            .socket
            .ssh_command("sh", &["-c", &start_proxy_command])
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
}

impl SshRemoteConnection {
    pub(crate) async fn new(
        connection_options: SshConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        use askpass::AskPassResult;

        delegate.set_status(Some("Connecting"), cx);

        let url = connection_options.ssh_url();

        let temp_dir = tempfile::Builder::new()
            .prefix("zed-ssh-session")
            .tempdir()?;
        let askpass_delegate = askpass::AskPassDelegate::new(cx, {
            let delegate = delegate.clone();
            move |prompt, tx, cx| delegate.ask_password(prompt, tx, cx)
        });

        let mut askpass =
            askpass::AskPassSession::new(cx.background_executor(), askpass_delegate).await?;

        // Start the master SSH process, which does not do anything except for establish
        // the connection and keep it open, allowing other ssh commands to reuse it
        // via a control socket.
        #[cfg(not(target_os = "windows"))]
        let socket_path = temp_dir.path().join("ssh.sock");

        let mut master_process = {
            #[cfg(not(target_os = "windows"))]
            let args = [
                "-N",
                "-o",
                "ControlPersist=no",
                "-o",
                "ControlMaster=yes",
                "-o",
            ];
            // On Windows, `ControlMaster` and `ControlPath` are not supported:
            // https://github.com/PowerShell/Win32-OpenSSH/issues/405
            // https://github.com/PowerShell/Win32-OpenSSH/wiki/Project-Scope
            #[cfg(target_os = "windows")]
            let args = ["-N"];
            let mut master_process = util::command::new_smol_command("ssh");
            master_process
                .kill_on_drop(true)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .env("SSH_ASKPASS_REQUIRE", "force")
                .env("SSH_ASKPASS", askpass.script_path())
                .args(connection_options.additional_args())
                .args(args);
            #[cfg(not(target_os = "windows"))]
            master_process.arg(format!("ControlPath={}", socket_path.display()));
            master_process.arg(&url).spawn()?
        };
        // Wait for this ssh process to close its stdout, indicating that authentication
        // has completed.
        let mut stdout = master_process.stdout.take().unwrap();
        let mut output = Vec::new();

        let result = select_biased! {
            result = askpass.run().fuse() => {
                match result {
                    AskPassResult::CancelledByUser => {
                        master_process.kill().ok();
                        anyhow::bail!("SSH connection canceled")
                    }
                    AskPassResult::Timedout => {
                        anyhow::bail!("connecting to host timed out")
                    }
                }
            }
            _ = stdout.read_to_end(&mut output).fuse() => {
                anyhow::Ok(())
            }
        };

        if let Err(e) = result {
            return Err(e.context("Failed to connect to host"));
        }

        if master_process.try_status()?.is_some() {
            output.clear();
            let mut stderr = master_process.stderr.take().unwrap();
            stderr.read_to_end(&mut output).await?;

            let error_message = format!(
                "failed to connect: {}",
                String::from_utf8_lossy(&output).trim()
            );
            anyhow::bail!(error_message);
        }

        #[cfg(not(target_os = "windows"))]
        let socket = SshSocket::new(connection_options, socket_path)?;
        #[cfg(target_os = "windows")]
        let socket = SshSocket::new(connection_options, &temp_dir, askpass.get_password())?;
        drop(askpass);

        let ssh_platform = socket.platform().await?;
        let ssh_path_style = match ssh_platform.os {
            "windows" => PathStyle::Windows,
            _ => PathStyle::Posix,
        };
        let ssh_shell = socket.shell().await;

        let mut this = Self {
            socket,
            master_process: Mutex::new(Some(master_process)),
            _temp_dir: temp_dir,
            remote_binary_path: None,
            ssh_path_style,
            ssh_platform,
            ssh_shell,
        };

        let (release_channel, version, commit) = cx.update(|cx| {
            (
                ReleaseChannel::global(cx),
                AppVersion::global(cx),
                AppCommitSha::try_global(cx),
            )
        })?;
        this.remote_binary_path = Some(
            this.ensure_server_binary(&delegate, release_channel, version, commit, cx)
                .await?,
        );

        Ok(this)
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
            paths::remote_server_dir_relative().join(binary_name),
            self.ssh_path_style,
        );

        #[cfg(debug_assertions)]
        if let Some(remote_server_path) =
            super::build_remote_server_from_source(&self.ssh_platform, delegate.as_ref(), cx)
                .await?
        {
            let tmp_path = RemotePathBuf::new(
                paths::remote_server_dir_relative().join(format!(
                    "download-{}-{}",
                    std::process::id(),
                    remote_server_path.file_name().unwrap().to_string_lossy()
                )),
                self.ssh_path_style,
            );
            self.upload_local_server_binary(&remote_server_path, &tmp_path, delegate, cx)
                .await?;
            self.extract_server_binary(&dst_path, &tmp_path, delegate, cx)
                .await?;
            return Ok(dst_path);
        }

        if self
            .socket
            .run_command(&dst_path.to_string(), &["version"])
            .await
            .is_ok()
        {
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
        })??;

        let tmp_path_gz = RemotePathBuf::new(
            PathBuf::from(format!("{}-download-{}.gz", dst_path, std::process::id())),
            self.ssh_path_style,
        );
        if !self.socket.connection_options.upload_binary_over_ssh
            && let Some((url, body)) = delegate
                .get_download_params(self.ssh_platform, release_channel, wanted_version, cx)
                .await?
        {
            match self
                .download_binary_on_server(&url, &body, &tmp_path_gz, delegate, cx)
                .await
            {
                Ok(_) => {
                    self.extract_server_binary(&dst_path, &tmp_path_gz, delegate, cx)
                        .await?;
                    return Ok(dst_path);
                }
                Err(e) => {
                    log::error!(
                        "Failed to download binary on server, attempting to upload server: {}",
                        e
                    )
                }
            }
        }

        let src_path = delegate
            .download_server_binary_locally(self.ssh_platform, release_channel, wanted_version, cx)
            .await?;
        self.upload_local_server_binary(&src_path, &tmp_path_gz, delegate, cx)
            .await?;
        self.extract_server_binary(&dst_path, &tmp_path_gz, delegate, cx)
            .await?;
        Ok(dst_path)
    }

    async fn download_binary_on_server(
        &self,
        url: &str,
        body: &str,
        tmp_path_gz: &RemotePathBuf,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        if let Some(parent) = tmp_path_gz.parent() {
            self.socket
                .run_command(
                    "sh",
                    &[
                        "-c",
                        &shell_script!("mkdir -p {parent}", parent = parent.to_string().as_ref()),
                    ],
                )
                .await?;
        }

        delegate.set_status(Some("Downloading remote development server on host"), cx);

        match self
            .socket
            .run_command(
                "curl",
                &[
                    "-f",
                    "-L",
                    "-X",
                    "GET",
                    "-H",
                    "Content-Type: application/json",
                    "-d",
                    body,
                    url,
                    "-o",
                    &tmp_path_gz.to_string(),
                ],
            )
            .await
        {
            Ok(_) => {}
            Err(e) => {
                if self.socket.run_command("which", &["curl"]).await.is_ok() {
                    return Err(e);
                }

                match self
                    .socket
                    .run_command(
                        "wget",
                        &[
                            "--method=GET",
                            "--header=Content-Type: application/json",
                            "--body-data",
                            body,
                            url,
                            "-O",
                            &tmp_path_gz.to_string(),
                        ],
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        if self.socket.run_command("which", &["wget"]).await.is_ok() {
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
        tmp_path_gz: &RemotePathBuf,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        if let Some(parent) = tmp_path_gz.parent() {
            self.socket
                .run_command(
                    "sh",
                    &[
                        "-c",
                        &shell_script!("mkdir -p {parent}", parent = parent.to_string().as_ref()),
                    ],
                )
                .await?;
        }

        let src_stat = fs::metadata(&src_path).await?;
        let size = src_stat.len();

        let t0 = Instant::now();
        delegate.set_status(Some("Uploading remote development server"), cx);
        log::info!(
            "uploading remote development server to {:?} ({}kb)",
            tmp_path_gz,
            size / 1024
        );
        self.upload_file(src_path, tmp_path_gz)
            .await
            .context("failed to upload server binary")?;
        log::info!("uploaded remote development server in {:?}", t0.elapsed());
        Ok(())
    }

    async fn extract_server_binary(
        &self,
        dst_path: &RemotePathBuf,
        tmp_path: &RemotePathBuf,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Extracting remote development server"), cx);
        let server_mode = 0o755;

        let orig_tmp_path = tmp_path.to_string();
        let script = if let Some(tmp_path) = orig_tmp_path.strip_suffix(".gz") {
            shell_script!(
                "gunzip -f {orig_tmp_path} && chmod {server_mode} {tmp_path} && mv {tmp_path} {dst_path}",
                server_mode = &format!("{:o}", server_mode),
                dst_path = &dst_path.to_string(),
            )
        } else {
            shell_script!(
                "chmod {server_mode} {orig_tmp_path} && mv {orig_tmp_path} {dst_path}",
                server_mode = &format!("{:o}", server_mode),
                dst_path = &dst_path.to_string()
            )
        };
        self.socket.run_command("sh", &["-c", &script]).await?;
        Ok(())
    }

    async fn upload_file(&self, src_path: &Path, dest_path: &RemotePathBuf) -> Result<()> {
        log::debug!("uploading file {:?} to {:?}", src_path, dest_path);
        let mut command = util::command::new_smol_command("scp");
        let output = self
            .socket
            .ssh_options(&mut command)
            .args(
                self.socket
                    .connection_options
                    .port
                    .map(|port| vec!["-P".to_string(), port.to_string()])
                    .unwrap_or_default(),
            )
            .arg(src_path)
            .arg(format!(
                "{}:{}",
                self.socket.connection_options.scp_url(),
                dest_path
            ))
            .output()
            .await?;

        anyhow::ensure!(
            output.status.success(),
            "failed to upload file {} -> {}: {}",
            src_path.display(),
            dest_path.to_string(),
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(())
    }
}

impl SshSocket {
    #[cfg(not(target_os = "windows"))]
    fn new(options: SshConnectionOptions, socket_path: PathBuf) -> Result<Self> {
        Ok(Self {
            connection_options: options,
            envs: HashMap::default(),
            socket_path,
        })
    }

    #[cfg(target_os = "windows")]
    fn new(options: SshConnectionOptions, temp_dir: &TempDir, secret: String) -> Result<Self> {
        let askpass_script = temp_dir.path().join("askpass.bat");
        std::fs::write(&askpass_script, "@ECHO OFF\necho %ZED_SSH_ASKPASS%")?;
        let mut envs = HashMap::default();
        envs.insert("SSH_ASKPASS_REQUIRE".into(), "force".into());
        envs.insert("SSH_ASKPASS".into(), askpass_script.display().to_string());
        envs.insert("ZED_SSH_ASKPASS".into(), secret);
        Ok(Self {
            connection_options: options,
            envs,
        })
    }

    // :WARNING: ssh unquotes arguments when executing on the remote :WARNING:
    // e.g. $ ssh host sh -c 'ls -l' is equivalent to $ ssh host sh -c ls -l
    // and passes -l as an argument to sh, not to ls.
    // Furthermore, some setups (e.g. Coder) will change directory when SSH'ing
    // into a machine. You must use `cd` to get back to $HOME.
    // You need to do it like this: $ ssh host "cd; sh -c 'ls -l /tmp'"
    fn ssh_command(&self, program: &str, args: &[&str]) -> process::Command {
        let mut command = util::command::new_smol_command("ssh");
        let to_run = iter::once(&program)
            .chain(args.iter())
            .map(|token| {
                // We're trying to work with: sh, bash, zsh, fish, tcsh, ...?
                debug_assert!(
                    !token.contains('\n'),
                    "multiline arguments do not work in all shells"
                );
                shlex::try_quote(token).unwrap()
            })
            .join(" ");
        let to_run = format!("cd; {to_run}");
        log::debug!("ssh {} {:?}", self.connection_options.ssh_url(), to_run);
        self.ssh_options(&mut command)
            .arg(self.connection_options.ssh_url())
            .arg(to_run);
        command
    }

    async fn run_command(&self, program: &str, args: &[&str]) -> Result<String> {
        let output = self.ssh_command(program, args).output().await?;
        anyhow::ensure!(
            output.status.success(),
            "failed to run command: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    #[cfg(not(target_os = "windows"))]
    fn ssh_options<'a>(&self, command: &'a mut process::Command) -> &'a mut process::Command {
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(self.connection_options.additional_args())
            .args(["-o", "ControlMaster=no", "-o"])
            .arg(format!("ControlPath={}", self.socket_path.display()))
    }

    #[cfg(target_os = "windows")]
    fn ssh_options<'a>(&self, command: &'a mut process::Command) -> &'a mut process::Command {
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(self.connection_options.additional_args())
            .envs(self.envs.clone())
    }

    // On Windows, we need to use `SSH_ASKPASS` to provide the password to ssh.
    // On Linux, we use the `ControlPath` option to create a socket file that ssh can use to
    #[cfg(not(target_os = "windows"))]
    fn ssh_args(&self) -> Vec<String> {
        let mut arguments = self.connection_options.additional_args();
        arguments.extend(vec![
            "-o".to_string(),
            "ControlMaster=no".to_string(),
            "-o".to_string(),
            format!("ControlPath={}", self.socket_path.display()),
            self.connection_options.ssh_url(),
        ]);
        arguments
    }

    #[cfg(target_os = "windows")]
    fn ssh_args(&self) -> Vec<String> {
        let mut arguments = self.connection_options.additional_args();
        arguments.push(self.connection_options.ssh_url());
        arguments
    }

    async fn platform(&self) -> Result<RemotePlatform> {
        let uname = self.run_command("sh", &["-c", "uname -sm"]).await?;
        let Some((os, arch)) = uname.split_once(" ") else {
            anyhow::bail!("unknown uname: {uname:?}")
        };

        let os = match os.trim() {
            "Darwin" => "macos",
            "Linux" => "linux",
            _ => anyhow::bail!(
                "Prebuilt remote servers are not yet available for {os:?}. See https://zed.dev/docs/remote-development"
            ),
        };
        // exclude armv5,6,7 as they are 32-bit.
        let arch = if arch.starts_with("armv8")
            || arch.starts_with("armv9")
            || arch.starts_with("arm64")
            || arch.starts_with("aarch64")
        {
            "aarch64"
        } else if arch.starts_with("x86") {
            "x86_64"
        } else {
            anyhow::bail!(
                "Prebuilt remote servers are not yet available for {arch:?}. See https://zed.dev/docs/remote-development"
            )
        };

        Ok(RemotePlatform { os, arch })
    }

    async fn shell(&self) -> String {
        match self.run_command("sh", &["-c", "echo $SHELL"]).await {
            Ok(shell) => shell.trim().to_owned(),
            Err(e) => {
                log::error!("Failed to get shell: {e}");
                "sh".to_owned()
            }
        }
    }
}

fn parse_port_number(port_str: &str) -> Result<u16> {
    port_str
        .parse()
        .with_context(|| format!("parsing port number: {port_str}"))
}

fn parse_port_forward_spec(spec: &str) -> Result<SshPortForwardOption> {
    let parts: Vec<&str> = spec.split(':').collect();

    match parts.len() {
        4 => {
            let local_port = parse_port_number(parts[1])?;
            let remote_port = parse_port_number(parts[3])?;

            Ok(SshPortForwardOption {
                local_host: Some(parts[0].to_string()),
                local_port,
                remote_host: Some(parts[2].to_string()),
                remote_port,
            })
        }
        3 => {
            let local_port = parse_port_number(parts[0])?;
            let remote_port = parse_port_number(parts[2])?;

            Ok(SshPortForwardOption {
                local_host: None,
                local_port,
                remote_host: Some(parts[1].to_string()),
                remote_port,
            })
        }
        _ => anyhow::bail!("Invalid port forward format"),
    }
}

impl SshConnectionOptions {
    pub fn parse_command_line(input: &str) -> Result<Self> {
        let input = input.trim_start_matches("ssh ");
        let mut hostname: Option<String> = None;
        let mut username: Option<String> = None;
        let mut port: Option<u16> = None;
        let mut args = Vec::new();
        let mut port_forwards: Vec<SshPortForwardOption> = Vec::new();

        // disallowed: -E, -e, -F, -f, -G, -g, -M, -N, -n, -O, -q, -S, -s, -T, -t, -V, -v, -W
        const ALLOWED_OPTS: &[&str] = &[
            "-4", "-6", "-A", "-a", "-C", "-K", "-k", "-X", "-x", "-Y", "-y",
        ];
        const ALLOWED_ARGS: &[&str] = &[
            "-B", "-b", "-c", "-D", "-F", "-I", "-i", "-J", "-l", "-m", "-o", "-P", "-p", "-R",
            "-w",
        ];

        let mut tokens = shlex::split(input).context("invalid input")?.into_iter();

        'outer: while let Some(arg) = tokens.next() {
            if ALLOWED_OPTS.contains(&(&arg as &str)) {
                args.push(arg.to_string());
                continue;
            }
            if arg == "-p" {
                port = tokens.next().and_then(|arg| arg.parse().ok());
                continue;
            } else if let Some(p) = arg.strip_prefix("-p") {
                port = p.parse().ok();
                continue;
            }
            if arg == "-l" {
                username = tokens.next();
                continue;
            } else if let Some(l) = arg.strip_prefix("-l") {
                username = Some(l.to_string());
                continue;
            }
            if arg == "-L" || arg.starts_with("-L") {
                let forward_spec = if arg == "-L" {
                    tokens.next()
                } else {
                    Some(arg.strip_prefix("-L").unwrap().to_string())
                };

                if let Some(spec) = forward_spec {
                    port_forwards.push(parse_port_forward_spec(&spec)?);
                } else {
                    anyhow::bail!("Missing port forward format");
                }
            }

            for a in ALLOWED_ARGS {
                if arg == *a {
                    args.push(arg);
                    if let Some(next) = tokens.next() {
                        args.push(next);
                    }
                    continue 'outer;
                } else if arg.starts_with(a) {
                    args.push(arg);
                    continue 'outer;
                }
            }
            if arg.starts_with("-") || hostname.is_some() {
                anyhow::bail!("unsupported argument: {:?}", arg);
            }
            let mut input = &arg as &str;
            // Destination might be: username1@username2@ip2@ip1
            if let Some((u, rest)) = input.rsplit_once('@') {
                input = rest;
                username = Some(u.to_string());
            }
            if let Some((rest, p)) = input.split_once(':') {
                input = rest;
                port = p.parse().ok()
            }
            hostname = Some(input.to_string())
        }

        let Some(hostname) = hostname else {
            anyhow::bail!("missing hostname");
        };

        let port_forwards = match port_forwards.len() {
            0 => None,
            _ => Some(port_forwards),
        };

        Ok(Self {
            host: hostname,
            username,
            port,
            port_forwards,
            args: Some(args),
            password: None,
            nickname: None,
            upload_binary_over_ssh: false,
        })
    }

    pub fn ssh_url(&self) -> String {
        let mut result = String::from("ssh://");
        if let Some(username) = &self.username {
            // Username might be: username1@username2@ip2
            let username = urlencoding::encode(username);
            result.push_str(&username);
            result.push('@');
        }
        result.push_str(&self.host);
        if let Some(port) = self.port {
            result.push(':');
            result.push_str(&port.to_string());
        }
        result
    }

    pub fn additional_args(&self) -> Vec<String> {
        let mut args = self.args.iter().flatten().cloned().collect::<Vec<String>>();

        if let Some(forwards) = &self.port_forwards {
            args.extend(forwards.iter().map(|pf| {
                let local_host = match &pf.local_host {
                    Some(host) => host,
                    None => "localhost",
                };
                let remote_host = match &pf.remote_host {
                    Some(host) => host,
                    None => "localhost",
                };

                format!(
                    "-L{}:{}:{}:{}",
                    local_host, pf.local_port, remote_host, pf.remote_port
                )
            }));
        }

        args
    }

    fn scp_url(&self) -> String {
        if let Some(username) = &self.username {
            format!("{}@{}", username, self.host)
        } else {
            self.host.clone()
        }
    }

    pub fn connection_string(&self) -> String {
        let host = if let Some(username) = &self.username {
            format!("{}@{}", username, self.host)
        } else {
            self.host.clone()
        };
        if let Some(port) = &self.port {
            format!("{}:{}", host, port)
        } else {
            host
        }
    }
}
