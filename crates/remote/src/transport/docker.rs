use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use collections::HashMap;
use parking_lot::Mutex;
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use semver::Version as SemanticVersion;
use std::time::Instant;
use std::{
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
};
use util::ResultExt;
use util::shell::ShellKind;
use util::{
    paths::{PathStyle, RemotePathBuf},
    rel_path::RelPath,
};

use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext, AsyncApp, Task};
use rpc::proto::Envelope;

use crate::{
    RemoteClientDelegate, RemoteConnection, RemoteConnectionOptions, RemoteOs, RemotePlatform,
    remote_client::CommandTemplate, transport::parse_platform,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct DockerConnectionOptions {
    pub name: String,
    pub container_id: String,
    pub upload_binary_over_docker_exec: bool,
}

pub(crate) struct DockerExecConnection {
    proxy_process: Mutex<Option<u32>>,
    remote_dir_for_server: String,
    remote_binary_relpath: Option<Arc<RelPath>>,
    connection_options: DockerConnectionOptions,
    remote_platform: Option<RemotePlatform>,
    path_style: Option<PathStyle>,
    shell: Option<String>,
}

impl DockerExecConnection {
    pub async fn new(
        connection_options: DockerConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let mut this = Self {
            proxy_process: Mutex::new(None),
            remote_dir_for_server: "/".to_string(),
            remote_binary_relpath: None,
            connection_options,
            remote_platform: None,
            path_style: None,
            shell: None,
        };
        let (release_channel, version, commit) = cx.update(|cx| {
            (
                ReleaseChannel::global(cx),
                AppVersion::global(cx),
                AppCommitSha::try_global(cx),
            )
        });
        let remote_platform = this.check_remote_platform().await?;

        this.path_style = match remote_platform.os {
            RemoteOs::Windows => Some(PathStyle::Windows),
            _ => Some(PathStyle::Posix),
        };

        this.remote_platform = Some(remote_platform);

        this.shell = Some(this.discover_shell().await);

        this.remote_dir_for_server = this.docker_user_home_dir().await?.trim().to_string();

        this.remote_binary_relpath = Some(
            this.ensure_server_binary(
                &delegate,
                release_channel,
                version,
                &this.remote_dir_for_server,
                commit,
                cx,
            )
            .await?,
        );

        Ok(this)
    }

    async fn discover_shell(&self) -> String {
        let default_shell = "sh";
        match self
            .run_docker_exec("sh", None, &Default::default(), &["-c", "echo $SHELL"])
            .await
        {
            Ok(shell) => match shell.trim() {
                "" => {
                    log::error!("$SHELL is not set, falling back to {default_shell}");
                    default_shell.to_owned()
                }
                shell => shell.to_owned(),
            },
            Err(e) => {
                log::error!("Failed to get shell: {e}");
                default_shell.to_owned()
            }
        }
    }

    async fn check_remote_platform(&self) -> Result<RemotePlatform> {
        let uname = self
            .run_docker_exec("uname", None, &Default::default(), &["-sm"])
            .await?;
        parse_platform(&uname)
    }

    async fn ensure_server_binary(
        &self,
        delegate: &Arc<dyn RemoteClientDelegate>,
        release_channel: ReleaseChannel,
        version: SemanticVersion,
        remote_dir_for_server: &str,
        commit: Option<AppCommitSha>,
        cx: &mut AsyncApp,
    ) -> Result<Arc<RelPath>> {
        let remote_platform = if self.remote_platform.is_some() {
            self.remote_platform.unwrap()
        } else {
            anyhow::bail!("No remote platform defined; cannot proceed.")
        };

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
            paths::remote_server_dir_relative().join(RelPath::unix(&binary_name).unwrap());

        #[cfg(debug_assertions)]
        if let Some(remote_server_path) =
            super::build_remote_server_from_source(&remote_platform, delegate.as_ref(), cx).await?
        {
            let tmp_path = paths::remote_server_dir_relative().join(
                RelPath::unix(&format!(
                    "download-{}-{}",
                    std::process::id(),
                    remote_server_path.file_name().unwrap().to_string_lossy()
                ))
                .unwrap(),
            );
            self.upload_local_server_binary(
                &remote_server_path,
                &tmp_path,
                &remote_dir_for_server,
                delegate,
                cx,
            )
            .await?;
            self.extract_server_binary(&dst_path, &tmp_path, &remote_dir_for_server, delegate, cx)
                .await?;
            return Ok(dst_path);
        }

        if self
            .run_docker_exec(
                &dst_path.display(self.path_style()),
                Some(&remote_dir_for_server),
                &Default::default(),
                &["version"],
            )
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
        })?;

        let tmp_path_gz = paths::remote_server_dir_relative().join(
            RelPath::unix(&format!(
                "{}-download-{}.gz",
                binary_name,
                std::process::id()
            ))
            .unwrap(),
        );
        if !self.connection_options.upload_binary_over_docker_exec
            && let Some(url) = delegate
                .get_download_url(remote_platform, release_channel, wanted_version.clone(), cx)
                .await?
        {
            match self
                .download_binary_on_server(&url, &tmp_path_gz, &remote_dir_for_server, delegate, cx)
                .await
            {
                Ok(_) => {
                    self.extract_server_binary(
                        &dst_path,
                        &tmp_path_gz,
                        &remote_dir_for_server,
                        delegate,
                        cx,
                    )
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
            .download_server_binary_locally(remote_platform, release_channel, wanted_version, cx)
            .await
            .context("downloading server binary locally")?;
        self.upload_local_server_binary(
            &src_path,
            &tmp_path_gz,
            &remote_dir_for_server,
            delegate,
            cx,
        )
        .await
        .context("uploading server binary")?;
        self.extract_server_binary(
            &dst_path,
            &tmp_path_gz,
            &remote_dir_for_server,
            delegate,
            cx,
        )
        .await
        .context("extracting server binary")?;
        Ok(dst_path)
    }

    async fn docker_user_home_dir(&self) -> Result<String> {
        let inner_program = self.shell();
        self.run_docker_exec(
            &inner_program,
            None,
            &Default::default(),
            &["-c", "echo $HOME"],
        )
        .await
    }

    async fn extract_server_binary(
        &self,
        dst_path: &RelPath,
        tmp_path: &RelPath,
        remote_dir_for_server: &str,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Extracting remote development server"), cx);
        let server_mode = 0o755;

        let shell_kind = ShellKind::Posix;
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
        self.run_docker_exec(
            "sh",
            Some(&remote_dir_for_server),
            &Default::default(),
            &args,
        )
        .await
        .log_err();
        Ok(())
    }

    async fn upload_local_server_binary(
        &self,
        src_path: &Path,
        tmp_path_gz: &RelPath,
        remote_dir_for_server: &str,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        if let Some(parent) = tmp_path_gz.parent() {
            self.run_docker_exec(
                "mkdir",
                Some(remote_dir_for_server),
                &Default::default(),
                &["-p", parent.display(self.path_style()).as_ref()],
            )
            .await?;
        }

        let src_stat = smol::fs::metadata(&src_path).await?;
        let size = src_stat.len();

        let t0 = Instant::now();
        delegate.set_status(Some("Uploading remote development server"), cx);
        log::info!(
            "uploading remote development server to {:?} ({}kb)",
            tmp_path_gz,
            size / 1024
        );
        self.upload_file(src_path, tmp_path_gz, remote_dir_for_server)
            .await
            .context("failed to upload server binary")?;
        log::info!("uploaded remote development server in {:?}", t0.elapsed());
        Ok(())
    }

    async fn upload_file(
        &self,
        src_path: &Path,
        dest_path: &RelPath,
        remote_dir_for_server: &str,
    ) -> Result<()> {
        log::debug!("uploading file {:?} to {:?}", src_path, dest_path);

        let src_path_display = src_path.display().to_string();
        let dest_path_str = dest_path.display(self.path_style());

        let mut command = util::command::new_smol_command("docker");
        command.arg("cp");
        command.arg("-a");
        command.arg(&src_path_display);
        command.arg(format!(
            "{}:{}/{}",
            &self.connection_options.container_id, remote_dir_for_server, dest_path_str
        ));

        let output = command.output().await?;

        if output.status.success() {
            return Ok(());
        }

        let stderr = String::from_utf8_lossy(&output.stderr);
        log::debug!(
            "failed to upload file via docker cp {src_path_display} -> {dest_path_str}: {stderr}",
        );
        anyhow::bail!(
            "failed to upload file via docker cp {} -> {}: {}",
            src_path_display,
            dest_path_str,
            stderr,
        );
    }

    async fn run_docker_command(
        &self,
        subcommand: &str,
        args: &[impl AsRef<str>],
    ) -> Result<String> {
        let mut command = util::command::new_smol_command("docker");
        command.arg(subcommand);
        for arg in args {
            command.arg(arg.as_ref());
        }
        let output = command.output().await?;
        anyhow::ensure!(
            output.status.success(),
            "failed to run command {command:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    async fn run_docker_exec(
        &self,
        inner_program: &str,
        working_directory: Option<&str>,
        env: &HashMap<String, String>,
        program_args: &[impl AsRef<str>],
    ) -> Result<String> {
        let mut args = match working_directory {
            Some(dir) => vec!["-w".to_string(), dir.to_string()],
            None => vec![],
        };

        for (k, v) in env.iter() {
            args.push("-e".to_string());
            let env_declaration = format!("{}={}", k, v);
            args.push(env_declaration);
        }

        args.push(self.connection_options.container_id.clone());
        args.push(inner_program.to_string());

        for arg in program_args {
            args.push(arg.as_ref().to_owned());
        }
        self.run_docker_command("exec", args.as_ref()).await
    }

    async fn download_binary_on_server(
        &self,
        url: &str,
        tmp_path_gz: &RelPath,
        remote_dir_for_server: &str,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        if let Some(parent) = tmp_path_gz.parent() {
            self.run_docker_exec(
                "mkdir",
                Some(remote_dir_for_server),
                &Default::default(),
                &["-p", parent.display(self.path_style()).as_ref()],
            )
            .await?;
        }

        delegate.set_status(Some("Downloading remote development server on host"), cx);

        match self
            .run_docker_exec(
                "curl",
                Some(remote_dir_for_server),
                &Default::default(),
                &[
                    "-f",
                    "-L",
                    url,
                    "-o",
                    &tmp_path_gz.display(self.path_style()),
                ],
            )
            .await
        {
            Ok(_) => {}
            Err(e) => {
                if self
                    .run_docker_exec("which", None, &Default::default(), &["curl"])
                    .await
                    .is_ok()
                {
                    return Err(e);
                }

                log::info!("curl is not available, trying wget");
                match self
                    .run_docker_exec(
                        "wget",
                        Some(remote_dir_for_server),
                        &Default::default(),
                        &[url, "-O", &tmp_path_gz.display(self.path_style())],
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        if self
                            .run_docker_exec("which", None, &Default::default(), &["wget"])
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

    fn kill_inner(&self) -> Result<()> {
        if let Some(pid) = self.proxy_process.lock().take() {
            if let Ok(_) = util::command::new_smol_command("kill")
                .arg(pid.to_string())
                .spawn()
            {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Failed to kill process"))
            }
        } else {
            Ok(())
        }
    }
}

#[async_trait(?Send)]
impl RemoteConnection for DockerExecConnection {
    fn has_wsl_interop(&self) -> bool {
        false
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
        // We'll try connecting anew every time we open a devcontainer, so proactively try to kill any old connections.
        if !self.has_been_killed() {
            if let Err(e) = self.kill_inner() {
                return Task::ready(Err(e));
            };
        }

        delegate.set_status(Some("Starting proxy"), cx);

        let Some(remote_binary_relpath) = self.remote_binary_relpath.clone() else {
            return Task::ready(Err(anyhow!("Remote binary path not set")));
        };

        let mut docker_args = vec!["exec".to_string()];
        for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
            if let Some(value) = std::env::var(env_var).ok() {
                docker_args.push("-e".to_string());
                docker_args.push(format!("{}='{}'", env_var, value));
            }
        }

        docker_args.extend([
            "-w".to_string(),
            self.remote_dir_for_server.clone(),
            "-i".to_string(),
            self.connection_options.container_id.to_string(),
        ]);

        let val = remote_binary_relpath
            .display(self.path_style())
            .into_owned();
        docker_args.push(val);
        docker_args.push("proxy".to_string());
        docker_args.push("--identifier".to_string());
        docker_args.push(unique_identifier);
        if reconnect {
            docker_args.push("--reconnect".to_string());
        }
        let mut command = util::command::new_smol_command("docker");
        command
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(docker_args);

        let Ok(child) = command.spawn() else {
            return Task::ready(Err(anyhow::anyhow!(
                "Failed to start remote server process"
            )));
        };

        let mut proxy_process = self.proxy_process.lock();
        *proxy_process = Some(child.id());

        super::handle_rpc_messages_over_child_process_stdio(
            child,
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
        let dest_path_str = dest_path.to_string();
        let src_path_display = src_path.display().to_string();

        let mut command = util::command::new_smol_command("docker");
        command.arg("cp");
        command.arg("-a"); // Archive mode is required to assign the file ownership to the default docker exec user
        command.arg(src_path_display);
        command.arg(format!(
            "{}:{}",
            self.connection_options.container_id, dest_path_str
        ));

        cx.background_spawn(async move {
            let output = command.output().await?;

            if output.status.success() {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Failed to upload directory"))
            }
        })
    }

    async fn kill(&self) -> Result<()> {
        self.kill_inner()
    }

    fn has_been_killed(&self) -> bool {
        self.proxy_process.lock().is_none()
    }

    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<String>,
        _port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        let mut parsed_working_dir = None;

        let path_style = self.path_style();

        if let Some(working_dir) = working_dir {
            let working_dir = RemotePathBuf::new(working_dir, path_style).to_string();

            const TILDE_PREFIX: &'static str = "~/";
            if working_dir.starts_with(TILDE_PREFIX) {
                let working_dir = working_dir.trim_start_matches("~").trim_start_matches("/");
                parsed_working_dir = Some(format!("$HOME/{working_dir}"));
            } else {
                parsed_working_dir = Some(working_dir);
            }
        }

        let mut inner_program = Vec::new();

        if let Some(program) = program {
            inner_program.push(program);
            for arg in args {
                inner_program.push(arg.clone());
            }
        } else {
            inner_program.push(self.shell());
            inner_program.push("-l".to_string());
        };

        let mut docker_args = vec!["exec".to_string()];

        if let Some(parsed_working_dir) = parsed_working_dir {
            docker_args.push("-w".to_string());
            docker_args.push(parsed_working_dir);
        }

        for (k, v) in env.iter() {
            docker_args.push("-e".to_string());
            docker_args.push(format!("{}={}", k, v));
        }

        docker_args.push("-it".to_string());
        docker_args.push(self.connection_options.container_id.to_string());

        docker_args.append(&mut inner_program);

        Ok(CommandTemplate {
            program: "docker".to_string(),
            args: docker_args,
            // Docker-exec pipes in environment via the "-e" argument
            env: Default::default(),
        })
    }

    fn build_forward_ports_command(
        &self,
        _forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        Err(anyhow::anyhow!("Not currently supported for docker_exec"))
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Docker(self.connection_options.clone())
    }

    fn path_style(&self) -> PathStyle {
        self.path_style.unwrap_or(PathStyle::Posix)
    }

    fn shell(&self) -> String {
        match &self.shell {
            Some(shell) => shell.clone(),
            None => self.default_system_shell(),
        }
    }

    fn default_system_shell(&self) -> String {
        String::from("/bin/sh")
    }
}
