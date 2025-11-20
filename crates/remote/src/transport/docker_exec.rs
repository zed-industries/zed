use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use collections::HashMap;
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use std::time::Instant;
use std::{
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
};
use util::shell::ShellKind;
use util::{
    paths::{PathStyle, RemotePathBuf},
    rel_path::RelPath,
};

use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext, AsyncApp, SemanticVersion, Task};
use rpc::proto::Envelope;

use crate::{
    RemoteClientDelegate, RemoteConnection, RemoteConnectionOptions, RemotePlatform,
    remote_client::CommandTemplate,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct DockerExecConnectionOptions {
    pub name: String,
    pub container_id: String,
    pub upload_binary_over_docker_exec: bool,
    pub working_directory: String,
}

pub(crate) struct DockerExecConnection {
    remote_binary_path: Option<Arc<RelPath>>,
    connection_options: DockerExecConnectionOptions,
    remote_platform: Option<RemotePlatform>,
    path_style: Option<PathStyle>,
    shell: Option<String>,
}

impl DockerExecConnection {
    pub async fn new(
        connection_options: DockerExecConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let mut this = Self {
            remote_binary_path: None,
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
        })?;
        let remote_platform = this.check_remote_platform().await?;

        this.path_style = match remote_platform.os {
            "windows" => Some(PathStyle::Windows),
            _ => Some(PathStyle::Posix),
        };

        this.remote_platform = Some(remote_platform);

        this.shell = Some(this.discover_shell().await);

        this.remote_binary_path = Some(
            this.ensure_server_binary(&delegate, release_channel, version, commit, cx)
                .await?,
        );

        Ok(this)
    }

    async fn discover_shell(&self) -> String {
        let default_shell = "sh";
        match self.run_command("sh", &["-c", "echo $SHELL"]).await {
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
        let uname = self.run_command("uname", &["-sm"]).await?;
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

    async fn ensure_server_binary(
        &self,
        delegate: &Arc<dyn RemoteClientDelegate>,
        release_channel: ReleaseChannel,
        version: SemanticVersion,
        commit: Option<AppCommitSha>,
        cx: &mut AsyncApp,
    ) -> Result<Arc<RelPath>> {
        let remote_platform = if self.remote_platform.is_some() {
            self.remote_platform.clone().unwrap()
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
            self.upload_local_server_binary(&remote_server_path, &tmp_path, delegate, cx)
                .await?;
            self.extract_server_binary(&dst_path, &tmp_path, delegate, cx)
                .await?;
            return Ok(dst_path);
        }

        if self
            .run_command(&dst_path.display(self.path_style()), &["version"])
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
                .get_download_url(remote_platform.clone(), release_channel, wanted_version, cx)
                .await?
        {
            match self
                .download_binary_on_server(&url, &tmp_path_gz, delegate, cx)
                .await
            {
                Ok(_) => {
                    self.extract_server_binary(&dst_path, &tmp_path_gz, delegate, cx)
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
                remote_platform.clone(),
                release_channel,
                wanted_version,
                cx,
            )
            .await
            .context("downloading server binary locally")?;
        self.upload_local_server_binary(&src_path, &tmp_path_gz, delegate, cx)
            .await
            .context("uploading server binary")?;
        self.extract_server_binary(&dst_path, &tmp_path_gz, delegate, cx)
            .await
            .context("extracting server binary")?;
        Ok(dst_path)
    }

    async fn extract_server_binary(
        &self,
        dst_path: &RelPath,
        tmp_path: &RelPath,
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
        self.run_command("sh", &args).await?;
        Ok(())
    }

    async fn upload_local_server_binary(
        &self,
        src_path: &Path,
        tmp_path_gz: &RelPath,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        if let Some(parent) = tmp_path_gz.parent() {
            self.run_command("mkdir", &["-p", parent.display(self.path_style()).as_ref()])
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
        self.upload_file(src_path, tmp_path_gz)
            .await
            .context("failed to upload server binary")?;
        log::info!("uploaded remote development server in {:?}", t0.elapsed());
        Ok(())
    }

    async fn upload_file(&self, src_path: &Path, dest_path: &RelPath) -> Result<()> {
        log::debug!("uploading file {:?} to {:?}", src_path, dest_path);

        let src_path_display = src_path.display().to_string();
        let dest_path_str = dest_path.display(self.path_style());

        let mut command = util::command::new_smol_command("docker");
        command.arg("cp");
        command.arg("-a");
        command.arg(&src_path_display);
        command.arg(format!(
            "{}:{}/{}",
            &self.connection_options.container_id,
            self.connection_options.working_directory,
            dest_path_str
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

    async fn run_command(&self, program: &str, args: &[impl AsRef<str>]) -> Result<String> {
        let mut command = util::command::new_smol_command("docker"); // TODO docker needs to be a field
        command.arg("exec");

        command.arg("-w");
        command.arg(&self.connection_options.working_directory);

        command.arg(&self.connection_options.container_id);

        command.arg(program);

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

    async fn download_binary_on_server(
        &self,
        url: &str,
        tmp_path_gz: &RelPath,
        delegate: &Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        if let Some(parent) = tmp_path_gz.parent() {
            self.run_command("mkdir", &["-p", parent.display(self.path_style()).as_ref()])
                .await?;
        }

        delegate.set_status(Some("Downloading remote development server on host"), cx);

        match self
            .run_command(
                "curl",
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
                if self.run_command("which", &["curl"]).await.is_ok() {
                    return Err(e);
                }

                log::info!("curl is not available, trying wget");
                match self
                    .run_command(
                        "wget",
                        &[url, "-O", &tmp_path_gz.display(self.path_style())],
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        if self.run_command("which", &["wget"]).await.is_ok() {
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
}

#[async_trait(?Send)]
impl RemoteConnection for DockerExecConnection {
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
        // let path = RelPath::new(Path::new(".zed_server/zed-remote-server"), PathStyle::Posix)?;
        // let remote_binary_path = Arc::new(RelPath::new(
        //     Path::new(".zed_server/zed-remote-server"),
        //     PathStyle::Posix,
        // ));

        let mut docker_args = vec![
            "exec",
            "-w",
            &self.connection_options.working_directory,
            "-i",
            &self.connection_options.container_id,
        ];
        // TODO not sure how you do this best in an exec context
        // for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
        //     if let Some(value) = std::env::var(env_var).ok() {
        //         docker_args.push(format!("{}='{}'", env_var, value));
        //     }
        // }
        let val = remote_binary_path.display(self.path_style()).into_owned();
        docker_args.push(&val);
        docker_args.push("proxy");
        docker_args.push("--identifier");
        docker_args.push(&unique_identifier);

        // if reconnect {
        //     proxy_args.push("--reconnect".to_owned());
        // }

        // let ssh_proxy_process = match self
        //     .socket
        //     .ssh_command(self.ssh_shell_kind, "env", &proxy_args, false)
        //     // IMPORTANT: we kill this process when we drop the task that uses it.
        //     .kill_on_drop(true)
        //     .spawn()
        // {
        //     Ok(process) => process,
        //     Err(error) => {
        //         return Task::ready(Err(anyhow!("failed to spawn remote server: {}", error)));
        //     }
        // };
        let mut command = util::command::new_smol_command("docker");
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(docker_args); // TODO
        let child = command.spawn().unwrap(); // TODO

        // So the question is really, why doesn't this have a stdout /in handle?
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

        println!("Upload path: {}", dest_path_str);
        println!("Source path: {}", src_path_display);

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

            dbg!("output: {}", &output);

            // TODO stderr mapping and such
            if output.status.success() {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Failed to upload directory"))
            }
        })
    }

    async fn kill(&self) -> Result<()> {
        // Docker exec is not stateful
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        // Docker exec is not stateful
        true
    }

    // This provides a TTY, but normall we can't count on one.
    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        use std::fmt::Write as _;

        let mut parsed_working_dir = None;

        let path_style = self.path_style();
        // let shell_kind = self.shell_kind();

        let mut exec = String::new();
        if let Some(working_dir) = working_dir {
            let working_dir = RemotePathBuf::new(working_dir, path_style).to_string();

            // shlex will wrap the command in single quotes (''), disabling ~ expansion,
            // replace with with something that works
            const TILDE_PREFIX: &'static str = "~/";
            if working_dir.starts_with(TILDE_PREFIX) {
                let working_dir = working_dir.trim_start_matches("~").trim_start_matches("/");
                parsed_working_dir = Some(format!("$HOME/{working_dir}"));
                // write!(
                //     exec,
                //     "cd \"$HOME/{working_dir}\" {} ",
                //     ssh_shell_kind.sequential_and_commands_separator()
                // )?;
            } else {
                parsed_working_dir = Some(working_dir);
                // write!(
                //     exec,
                //     "cd \"{working_dir}\" {} ",
                //     ssh_shell_kind.sequential_and_commands_separator()
                // )?;
            }
        }

        let mut env_str = String::new();

        for (k, v) in env.iter() {
            write!(env_str, "{}={}", k, v)?; // TODO shell escaping
        }
        write!(exec, "exec env ")?;

        let mut inner_program = Vec::new();

        if let Some(program) = program {
            inner_program.push(program);
            for arg in args {
                // let arg = ssh_shell_kind.try_quote(&arg).context("shell quoting")?;
                inner_program.push(arg.clone());
            }
        } else {
            inner_program.push("/bin/bash".to_string()); // TODO shell extraction
            inner_program.push("-l".to_string());
        };

        let mut the_args = vec!["exec".to_string()];

        if parsed_working_dir.is_some() {
            the_args.push("-w".to_string());
            the_args.push(parsed_working_dir.unwrap());
        }

        if env_str != "" {
            the_args.push("-e".to_string());
            the_args.push(env_str);
        }

        the_args.push("-it".to_string());
        the_args.push(self.connection_options.container_id.to_string());

        the_args.append(&mut inner_program);

        print!("The Args: {:?}", the_args);
        Ok(CommandTemplate {
            program: "docker".to_string(),
            args: the_args,
            // Docker-exec pipes in environment via the "-e" argument
            env: Default::default(),
        })
    }

    fn build_forward_ports_command(
        &self,
        forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        todo!()
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::DockerExec(self.connection_options.clone())
    }

    fn path_style(&self) -> PathStyle {
        self.path_style.unwrap_or(PathStyle::Posix)
    }

    fn shell(&self) -> String {
        match &self.shell {
            Some(shell) => shell.clone(),
            None => self.default_system_shell().clone(),
        }
    }

    fn default_system_shell(&self) -> String {
        String::from("/bin/sh")
    }
}
