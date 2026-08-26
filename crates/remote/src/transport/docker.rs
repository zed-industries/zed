use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use collections::HashMap;
use parking_lot::Mutex;
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use semver::Version as SemanticVersion;
use std::collections::BTreeMap;
use std::time::Instant;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::ResultExt;
use util::command::Stdio;
use util::shell::ShellKind;
use util::{
    paths::{PathStyle, RemotePathBuf},
    rel_path::RelPath,
};

use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext, AsyncApp, Task};
use rpc::proto::Envelope;

use crate::{
    RemoteArch, RemoteClientDelegate, RemoteConnection, RemoteConnectionOptions, RemoteOs,
    RemotePlatform,
    remote_client::{CommandTemplate, Interactive},
    transport::parse_platform,
    transport::ssh::SshConnectionOptions,
    transport::wsl::WslConnectionOptions,
};

/// Where the docker daemon that owns the container lives. `Local` is the
/// client machine; `Ssh` and `Wsl` are hosts Zed already knows how to connect
/// to, so the docker invocations can be routed through that connection.
///
/// This is deliberately flat: a host is always one hop from the client, which
/// is what lets the connection pool rebuild it without recursing.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DockerHost {
    #[default]
    Local,
    Ssh(SshConnectionOptions),
    Wsl(WslConnectionOptions),
    #[cfg(any(test, feature = "test-support"))]
    Mock(crate::transport::mock::MockConnectionOptions),
}

impl DockerHost {
    /// How Zed would connect to this host on its own, or `None` when the
    /// daemon runs on the client machine.
    pub fn connection_options(&self) -> Option<RemoteConnectionOptions> {
        match self {
            DockerHost::Local => None,
            DockerHost::Ssh(options) => Some(RemoteConnectionOptions::Ssh(options.clone())),
            DockerHost::Wsl(options) => Some(RemoteConnectionOptions::Wsl(options.clone())),
            #[cfg(any(test, feature = "test-support"))]
            DockerHost::Mock(options) => Some(RemoteConnectionOptions::Mock(options.clone())),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DockerConnectionOptions {
    pub name: String,
    pub container_id: String,
    pub remote_user: String,
    pub upload_binary_over_docker_exec: bool,
    pub use_podman: bool,
    pub remote_env: BTreeMap<String, String>,
    #[serde(default)]
    pub host: DockerHost,
}

pub(crate) struct DockerExecConnection {
    proxy_process: Mutex<Option<u32>>,
    remote_dir_for_server: String,
    remote_binary_relpath: Option<Arc<RelPath>>,
    connection_options: DockerConnectionOptions,
    remote_platform: Option<RemotePlatform>,
    os_version: Option<String>,
    path_style: Option<PathStyle>,
    shell: String,
    /// Connection to the machine running the docker daemon, when it is not
    /// this one. Every docker invocation is routed through it.
    host: Option<Arc<dyn RemoteConnection>>,
    /// How the container is addressed from the machine running the daemon.
    /// `None` when it could not be determined, which makes port forwarding
    /// unavailable rather than wrong.
    container_address: Option<String>,
}

impl DockerExecConnection {
    pub async fn new(
        connection_options: DockerConnectionOptions,
        host: Option<Arc<dyn RemoteConnection>>,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let mut this = Self {
            proxy_process: Mutex::new(None),
            remote_dir_for_server: "/".to_string(),
            remote_binary_relpath: None,
            connection_options,
            remote_platform: None,
            os_version: None,
            path_style: None,
            shell: "sh".to_owned(),
            host,
            container_address: None,
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
            _ => Some(PathStyle::Unix),
        };

        this.remote_platform = Some(remote_platform);
        log::info!("Remote platform discovered: {:?}", this.remote_platform);

        this.os_version = this.discover_os_version(remote_platform.os).await;
        log::info!("Remote OS version discovered: {:?}", this.os_version);

        this.shell = this.discover_shell().await;
        log::info!("Remote shell discovered: {}", this.shell);

        this.container_address = this.discover_container_address().await;
        log::info!(
            "Container address on its daemon's machine: {:?}",
            this.container_address
        );

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

    fn docker_cli(&self) -> &str {
        if self.connection_options.use_podman {
            "podman"
        } else {
            "docker"
        }
    }

    /// The single place docker invocations are turned into a runnable command.
    /// Without a host the template is the docker CLI itself; with one it is
    /// whatever that host's transport uses to run the docker CLI over there.
    ///
    /// `interactive` has to match what the docker invocation itself asks for.
    /// A `docker exec -it` reached over a host that was not also asked for a
    /// TTY fails with "the input device is not a TTY", and a TTY on the
    /// non-interactive paths would corrupt the framed stdio the proxy carries.
    fn docker_command(
        &self,
        args: Vec<String>,
        interactive: Interactive,
    ) -> Result<CommandTemplate> {
        match &self.host {
            None => Ok(CommandTemplate {
                program: self.docker_cli().to_string(),
                args,
                env: Default::default(),
            }),
            Some(host) => host.build_command(
                Some(self.docker_cli().to_string()),
                &args,
                &Default::default(),
                None,
                None,
                interactive,
            ),
        }
    }

    /// Run a shell command inside the container and reliably extract its output
    /// using unique delimiters, so that shell initialization noise (e.g. from
    /// BASH_ENV or .bashrc) does not corrupt the result.
    async fn run_docker_exec_delimited(&self, script: &str) -> Result<String> {
        const MARKER: &str = "=====ZED_DELIM_7f3a9c=====";
        let wrapped =
            format!("printf '{MARKER}'; {script}; __exit=$?; printf '{MARKER}'; exit $__exit");
        let output = self
            .run_docker_exec("sh", None, &Default::default(), &["-c", &wrapped])
            .await?;
        let start = output.find(MARKER).map(|i| i + MARKER.len()).unwrap_or(0);
        let end = output[start..]
            .find(MARKER)
            .map(|i| start + i)
            .unwrap_or(output.len());
        Ok(output[start..end].to_string())
    }

    /// Asks the daemon how the container is reached from its own machine.
    ///
    /// A container's ports are only published on that machine if the
    /// configuration asked for it, but on a bridge network every port is
    /// reachable at the container's own address without publishing. A
    /// container sharing the host's network namespace has no address of its
    /// own and is reached on loopback instead.
    async fn discover_container_address(&self) -> Option<String> {
        let output = self
            .run_docker_command(
                "inspect",
                &[
                    "-f",
                    "{{.HostConfig.NetworkMode}} {{range .NetworkSettings.Networks}}{{.IPAddress}} {{end}}",
                    &self.connection_options.container_id,
                ],
            )
            .await
            .map_err(|error| log::warn!("Could not inspect the container's network: {error}"))
            .ok()?;

        let mut fields = output.split_whitespace();
        if fields.next()? == "host" {
            return Some("localhost".to_string());
        }
        fields.next().map(str::to_string)
    }

    async fn discover_shell(&self) -> String {
        let default_shell = "sh";
        match self.run_docker_exec_delimited("echo $SHELL").await {
            Ok(shell) => match shell.trim() {
                "" => {
                    log::info!("$SHELL is not set, checking passwd for user");
                }
                shell => {
                    return shell.to_owned();
                }
            },
            Err(e) => {
                log::error!("Failed to get $SHELL: {e}. Checking passwd for user");
            }
        }

        match self
            .run_docker_exec_delimited("getent passwd \"$(id -un)\" | cut -d: -f7")
            .await
        {
            Ok(shell) => match shell.trim() {
                "" => {
                    log::info!("No shell found in passwd, falling back to {default_shell}");
                }
                shell => {
                    return shell.to_owned();
                }
            },
            Err(e) => {
                log::info!("Error getting shell from passwd: {e}. Falling back to {default_shell}");
            }
        }
        default_shell.to_owned()
    }

    async fn check_remote_platform(&self) -> Result<RemotePlatform> {
        let uname = self.run_docker_exec_delimited("uname -sm").await?;
        parse_platform(&uname)
    }

    /// Best-effort detection of the container's OS version for telemetry.
    async fn discover_os_version(&self, os: RemoteOs) -> Option<String> {
        let (program, args) = super::os_version_command(os);
        match self
            .run_docker_exec(program, None, &Default::default(), args)
            .await
        {
            Ok(output) => super::parse_os_version(os, &output),
            Err(error) => {
                log::warn!("Failed to determine remote OS version: {error:#}");
                None
            }
        }
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
        let remote_platform = self
            .remote_platform
            .context("No remote platform defined; cannot proceed.")?;

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
            paths::remote_server_dir_relative().join(RelPath::from_unix_str(&binary_name).unwrap());

        let binary_exists_on_server = self
            .run_docker_exec(
                &dst_path.display(self.path_style()),
                Some(&remote_dir_for_server),
                &Default::default(),
                &["version"],
            )
            .await
            .is_ok();
        #[cfg(any(debug_assertions, feature = "build-remote-server-binary"))]
        if let Some(remote_server_path) = super::build_remote_server_from_source(
            &remote_platform,
            delegate.as_ref(),
            binary_exists_on_server,
            cx,
        )
        .await?
        {
            let tmp_path = paths::remote_server_dir_relative().join(
                RelPath::from_unix_str(&format!(
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
            return Ok(dst_path.into());
        }

        if binary_exists_on_server {
            return Ok(dst_path.into());
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
            RelPath::from_unix_str(&format!(
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
                    return Ok(dst_path.into());
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
        Ok(dst_path.into())
    }

    async fn docker_user_home_dir(&self) -> Result<String> {
        self.run_docker_exec_delimited("echo $HOME").await
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

    fn upload_commands(
        &self,
        src_path: &str,
        dst_path: &str,
    ) -> Result<(CommandTemplate, CommandTemplate)> {
        let container_id = &self.connection_options.container_id;
        let remote_user = &self.connection_options.remote_user;

        let copy_command = self.docker_command(
            vec![
                "cp".to_string(),
                "-a".to_string(),
                src_path.to_string(),
                format!("{container_id}:{dst_path}"),
            ],
            Interactive::No,
        )?;
        let chown_command = self.docker_command(
            vec![
                "exec".to_string(),
                container_id.to_string(),
                "chown".to_string(),
                format!("{remote_user}:{remote_user}"),
                dst_path.to_string(),
            ],
            Interactive::No,
        )?;

        Ok((copy_command, chown_command))
    }

    /// Takes prebuilt templates rather than building them itself, because
    /// `upload_directory` needs a `'static` future and cannot borrow `self`.
    async fn upload_and_chown(
        copy_command: CommandTemplate,
        chown_command: CommandTemplate,
        src_path: String,
        dst_path: String,
    ) -> Result<()> {
        let output = copy_command.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::debug!("failed to upload via docker cp {src_path} -> {dst_path}: {stderr}",);
            anyhow::bail!(
                "failed to upload via docker cp {} -> {}: {}",
                src_path,
                dst_path,
                stderr,
            );
        }

        let output = chown_command.output().await?;

        if output.status.success() {
            return Ok(());
        }

        let stderr = String::from_utf8_lossy(&output.stderr);
        log::debug!("failed to change ownership for via chown: {stderr}",);
        anyhow::bail!(
            "failed to change ownership for zed_remote_server via chown: {}",
            stderr,
        );
    }

    /// The argv that unpacks a tar stream from this connection's stdin into
    /// `dst_path` inside the container, as the connection's remote user.
    ///
    /// `tar` has to exist in the container. Every other way of moving a
    /// directory needs either a path the daemon's machine can see (which
    /// `docker cp` requires and a client-side directory is not) or one round
    /// trip per file.
    fn extract_archive_command(&self, dst_path: &str) -> Result<CommandTemplate> {
        self.docker_command(
            vec![
                "exec".to_string(),
                "-i".to_string(),
                "-u".to_string(),
                self.connection_options.remote_user.clone(),
                self.connection_options.container_id.clone(),
                "sh".to_string(),
                "-c".to_string(),
                "mkdir -p \"$1\" && tar -xf - -C \"$1\"".to_string(),
                "zed-upload".to_string(),
                dst_path.to_string(),
            ],
            Interactive::No,
        )
    }

    /// Packs `src_path`'s contents into a tar file next to it, ready to be
    /// streamed into a container.
    async fn archive_directory(src_path: &Path) -> Result<tempfile::NamedTempFile> {
        let archive = tempfile::NamedTempFile::new().context("creating an upload archive")?;
        let file = smol::fs::File::create(archive.path())
            .await
            .context("opening the upload archive")?;

        let mut builder = async_tar::Builder::new(file);
        builder
            .append_dir_all(".", src_path)
            .await
            .with_context(|| format!("packing {} for upload", src_path.display()))?;
        builder
            .finish()
            .await
            .context("finishing the upload archive")?;

        Ok(archive)
    }

    /// The argv that writes this connection's stdin into `dst_path` inside the
    /// container, as the connection's remote user.
    fn stream_into_container_command(&self, dst_path: &str) -> Result<CommandTemplate> {
        self.docker_command(
            vec![
                "exec".to_string(),
                "-i".to_string(),
                "-u".to_string(),
                self.connection_options.remote_user.clone(),
                self.connection_options.container_id.clone(),
                "sh".to_string(),
                "-c".to_string(),
                "cat > \"$1\"".to_string(),
                "zed-upload".to_string(),
                dst_path.to_string(),
            ],
            Interactive::No,
        )
    }

    /// Sends a file into the container over the connection's own stdin, which
    /// works no matter which machine the daemon is on. `docker cp` cannot do
    /// that when the daemon is remote, because the source path it is given is
    /// resolved on the daemon's machine, not this one.
    async fn stream_file_into_container(&self, src_path: &Path, dst_path: &str) -> Result<()> {
        Self::stream_file_into(src_path, self.stream_into_container_command(dst_path)?).await
    }

    async fn stream_file_into(src_path: &Path, command: CommandTemplate) -> Result<()> {
        let mut file = smol::fs::File::open(src_path).await.with_context(|| {
            format!(
                "opening {} to stream into the container",
                src_path.display()
            )
        })?;

        let mut child = util::command::new_command(&command.program)
            .args(&command.args)
            .envs(&command.env)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| {
                format!(
                    "spawning `{}` to stream into the container",
                    command.program
                )
            })?;

        let mut stdin = child
            .stdin
            .take()
            .context("upload child did not expose stdin")?;

        let copy_result = smol::io::copy(&mut file, &mut stdin).await;
        let flush_result = smol::io::AsyncWriteExt::flush(&mut stdin).await;
        drop(stdin);

        let output = child
            .output()
            .await
            .context("awaiting the container upload child")?;

        copy_result
            .with_context(|| format!("writing {} into the container", src_path.display()))?;
        flush_result.context("flushing the container upload")?;

        anyhow::ensure!(
            output.status.success(),
            "failed to stream {} into the container: {}",
            src_path.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
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
        let full_server_path = format!("{}/{}", remote_dir_for_server, dest_path_str);

        if self.host.is_some() {
            return self
                .stream_file_into_container(src_path, &full_server_path)
                .await;
        }

        let (copy_command, chown_command) =
            self.upload_commands(&src_path_display, &full_server_path)?;

        Self::upload_and_chown(
            copy_command,
            chown_command,
            src_path_display,
            full_server_path,
        )
        .await
    }

    async fn run_docker_command(
        &self,
        subcommand: &str,
        args: &[impl AsRef<str>],
    ) -> Result<String> {
        let mut command_args = vec![subcommand.to_string()];
        command_args.extend(args.iter().map(|arg| arg.as_ref().to_string()));

        let command = self.docker_command(command_args, Interactive::No)?;
        let output = command.output().await?;
        log::debug!("{:?}: {:?}", command, output);
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

        args.push("-u".to_string());
        args.push(self.connection_options.remote_user.clone());

        for (k, v) in self.connection_options.remote_env.iter() {
            args.push("-e".to_string());
            args.push(format!("{k}={v}"));
        }

        for (k, v) in env.iter() {
            args.push("-e".to_string());
            args.push(format!("{k}={v}"));
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

    /// The command whose stdio carries the RPC framing. Exactly one local child
    /// is spawned from it; with a host, that child is the host transport rather
    /// than the docker CLI.
    fn proxy_command(&self, unique_identifier: String, reconnect: bool) -> Result<CommandTemplate> {
        let remote_binary_relpath = self
            .remote_binary_relpath
            .clone()
            .context("Remote binary path not set")?;

        let mut docker_args = vec!["exec".to_string()];

        for (k, v) in self.connection_options.remote_env.iter() {
            docker_args.push("-e".to_string());
            docker_args.push(format!("{k}={v}"));
        }
        for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
            if let Some(value) = std::env::var(env_var).ok() {
                docker_args.push("-e".to_string());
                docker_args.push(format!("{env_var}={value}"));
            }
        }

        docker_args.extend([
            "-u".to_string(),
            self.connection_options.remote_user.to_string(),
            "-w".to_string(),
            self.remote_dir_for_server.clone(),
            "-i".to_string(),
            self.connection_options.container_id.to_string(),
        ]);

        docker_args.push(
            remote_binary_relpath
                .display(self.path_style())
                .into_owned(),
        );
        docker_args.push("proxy".to_string());
        docker_args.push("--identifier".to_string());
        docker_args.push(unique_identifier);
        if reconnect {
            docker_args.push("--reconnect".to_string());
        }

        // The proxy's stdio carries framed protobuf, so it must never be given
        // a TTY no matter what the host connection is.
        self.docker_command(docker_args, Interactive::No)
    }

    /// Kills the proxy child and nothing else. The host connection is shared —
    /// the same SSH master may be carrying an ordinary remote project — so it
    /// is left alone and released only when the last owner drops it.
    ///
    /// Killing the proxy child closes the stdin the remote `docker exec` is
    /// reading from, which is what the in-container proxy exits on, so the
    /// remote side is not orphaned by this.
    fn kill_inner(&self) -> Result<()> {
        if let Some(pid) = self.proxy_process.lock().take() {
            if let Ok(_) = util::command::new_command("kill")
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

        let proxy_command = match self.proxy_command(unique_identifier, reconnect) {
            Ok(proxy_command) => proxy_command,
            Err(error) => return Task::ready(Err(error)),
        };

        let mut command = util::command::new_command(&proxy_command.program);
        command
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(&proxy_command.args)
            .envs(&proxy_command.env);

        let Ok(child) = command.spawn() else {
            return Task::ready(Err(anyhow::anyhow!(
                "Failed to start remote server process"
            )));
        };

        let mut proxy_process = self.proxy_process.lock();
        *proxy_process = Some(child.id());

        cx.spawn(async move |cx| {
            super::handle_rpc_messages_over_child_process_stdio(
                child,
                incoming_tx,
                outgoing_rx,
                connection_activity_tx,
                cx,
            )
            .await
            .and_then(|status| {
                if status != 0 {
                    anyhow::bail!("Remote server exited with status {status}");
                }
                Ok(0)
            })
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

        if self.host.is_some() {
            // `docker cp` would resolve the source on the host, where this
            // directory does not exist, so the directory travels as a tar
            // stream on the exec channel instead — the same route the server
            // binary takes, which does not care where the daemon runs.
            let extract_command = match self.extract_archive_command(&dest_path_str) {
                Ok(command) => command,
                Err(error) => return Task::ready(Err(error)),
            };
            return cx.background_spawn(async move {
                let archive = Self::archive_directory(&src_path).await?;
                Self::stream_file_into(archive.path(), extract_command)
                    .await
                    .with_context(|| format!("uploading {src_path_display} into the container"))
            });
        }

        let (copy_command, chown_command) =
            match self.upload_commands(&src_path_display, &dest_path_str) {
                Ok(commands) => commands,
                Err(error) => return Task::ready(Err(error)),
            };

        cx.background_spawn(Self::upload_and_chown(
            copy_command,
            chown_command,
            src_path_display,
            dest_path_str,
        ))
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
        interactive: Interactive,
    ) -> Result<CommandTemplate> {
        let mut parsed_working_dir = None;

        let path_style = self.path_style();

        if let Some(working_dir) = working_dir {
            let working_dir = RemotePathBuf::new(working_dir, path_style).to_string();

            const TILDE_PREFIX: &'static str = "~/";
            if working_dir.starts_with(TILDE_PREFIX) {
                let working_dir = working_dir.trim_start_matches("~").trim_start_matches("/");
                parsed_working_dir =
                    Some(format!("{}/{}", self.remote_dir_for_server, working_dir));
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

        let mut docker_args = vec![
            "exec".to_string(),
            "-u".to_string(),
            self.connection_options.remote_user.clone(),
        ];

        if let Some(parsed_working_dir) = parsed_working_dir {
            docker_args.push("-w".to_string());
            docker_args.push(parsed_working_dir);
        }

        for (k, v) in self.connection_options.remote_env.iter() {
            docker_args.push("-e".to_string());
            docker_args.push(format!("{k}={v}"));
        }

        for (k, v) in env.iter() {
            docker_args.push("-e".to_string());
            docker_args.push(format!("{k}={v}"));
        }

        match interactive {
            Interactive::Yes => docker_args.push("-it".to_string()),
            Interactive::No => docker_args.push("-i".to_string()),
        }
        docker_args.push(self.connection_options.container_id.to_string());

        docker_args.append(&mut inner_program);

        // Docker-exec pipes in environment via the "-e" argument, so the
        // template's own env stays empty.
        self.docker_command(docker_args, interactive)
    }

    /// Forwards through the machine running the daemon, which can reach the
    /// container directly. Without a host there is no such hop to compose
    /// with: the daemon is here, and reaching an unpublished container port
    /// from here would need a relay process inside the container.
    fn build_forward_ports_command(
        &self,
        forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        let Some(host) = &self.host else {
            anyhow::bail!("port forwarding is not supported for a container on this machine");
        };
        let container_address = self.container_address.as_ref().context(
            "the container's address on its host is unknown, so ports cannot be forwarded",
        )?;

        let forwards = forwards
            .into_iter()
            .map(|(local_port, destination, remote_port)| {
                // The destination is named as the container sees it. Only
                // loopback survives being re-pointed at the container from one
                // hop away; anything else would silently forward to a host of
                // the same name on the daemon's machine.
                anyhow::ensure!(
                    matches!(destination.as_str(), "localhost" | "127.0.0.1" | "::1"),
                    "only loopback ports inside a container can be forwarded, not {destination}"
                );
                Ok((local_port, container_address.clone(), remote_port))
            })
            .collect::<Result<Vec<_>>>()?;

        host.build_forward_ports_command(forwards)
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Docker(self.connection_options.clone())
    }

    fn path_style(&self) -> PathStyle {
        self.path_style.unwrap_or(PathStyle::Unix)
    }

    fn remote_platform(&self) -> RemotePlatform {
        // Docker containers are always Linux; the platform is populated during
        // setup, so this fallback is only for the brief pre-detection window.
        self.remote_platform.unwrap_or(RemotePlatform {
            os: RemoteOs::Linux,
            arch: RemoteArch::X86_64,
        })
    }

    fn remote_os_version(&self) -> Option<String> {
        self.os_version.clone()
    }

    fn shell(&self) -> String {
        self.shell.clone()
    }

    fn default_system_shell(&self) -> String {
        String::from("/bin/sh")
    }
}

#[cfg(test)]
mod tests {
    use super::{DockerConnectionOptions, DockerExecConnection, DockerHost};
    use crate::RemoteConnection;
    use crate::remote_client::CommandTemplate;
    use crate::remote_client::Interactive;
    use crate::transport::mock::{MockConnection, MockConnectionRegistry, MockRemoteConnection};
    use crate::transport::ssh::SshConnectionOptions;
    use gpui::TestAppContext;
    use parking_lot::Mutex;
    use std::sync::Arc;
    use util::paths::PathStyle;
    use util::rel_path::RelPath;

    fn local_connection(options: DockerConnectionOptions) -> DockerExecConnection {
        DockerExecConnection {
            proxy_process: Mutex::new(None),
            remote_dir_for_server: "/home/anth".to_string(),
            remote_binary_relpath: None,
            connection_options: options,
            remote_platform: None,
            os_version: None,
            path_style: Some(PathStyle::Unix),
            shell: "/bin/bash".to_string(),
            host: None,
            container_address: Some("172.17.0.2".to_string()),
        }
    }

    async fn mock_host(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) -> Arc<MockRemoteConnection> {
        let (options, _server_client, connect_guard) = MockConnection::new(cx, server_cx);
        connect_guard.send(()).ok();
        cx.update(|cx| cx.default_global::<MockConnectionRegistry>().take(&options))
            .expect("the mock connection should be registered")
            .await
    }

    /// The argv the proxy child is spawned from, minus the `-e` pairs, which
    /// depend on which of `RUST_LOG` and friends the test process happens to
    /// have set.
    fn without_env_args(args: &[String]) -> Vec<String> {
        let mut kept = Vec::new();
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            if arg == "-e" {
                args.next();
                continue;
            }
            kept.push(arg.clone());
        }
        kept
    }

    fn docker_options() -> DockerConnectionOptions {
        DockerConnectionOptions {
            name: "zed-dev".to_string(),
            container_id: "container-123".to_string(),
            remote_user: "anth".to_string(),
            upload_binary_over_docker_exec: false,
            use_podman: false,
            remote_env: Default::default(),
            host: DockerHost::Local,
        }
    }

    #[test]
    fn local_host_commands_are_unwrapped_docker_invocations() {
        let connection = local_connection(docker_options());
        let command = connection
            .docker_command(vec!["ps".to_string(), "-a".to_string()], Interactive::No)
            .expect("building a local docker command should succeed");

        assert_eq!(command.program, "docker");
        assert_eq!(command.args, vec!["ps".to_string(), "-a".to_string()]);
        assert!(command.env.is_empty());

        let podman = local_connection(DockerConnectionOptions {
            use_podman: true,
            ..docker_options()
        });
        assert_eq!(
            podman
                .docker_command(vec!["ps".to_string()], Interactive::No)
                .expect("building a local podman command should succeed")
                .program,
            "podman"
        );
    }

    #[test]
    fn local_host_build_command_argv_is_unchanged() {
        let connection = local_connection(DockerConnectionOptions {
            remote_env: [("FOO".to_string(), "BAR".to_string())]
                .into_iter()
                .collect(),
            ..docker_options()
        });

        let command = connection
            .build_command(
                Some("ls".to_string()),
                &["-la".to_string()],
                &Default::default(),
                Some("~/project".to_string()),
                None,
                Interactive::No,
            )
            .expect("building a command for a local container should succeed");

        assert_eq!(command.program, "docker");
        assert_eq!(
            command.args,
            vec![
                "exec",
                "-u",
                "anth",
                "-w",
                "/home/anth/project",
                "-e",
                "FOO=BAR",
                "-i",
                "container-123",
                "ls",
                "-la",
            ]
        );
        assert!(command.env.is_empty());
    }

    #[test]
    fn local_host_proxy_command_argv_is_unchanged() {
        let mut connection = local_connection(docker_options());
        connection.remote_binary_relpath = Some(
            RelPath::from_unix_str("zed-remote-server")
                .expect("a relative unix path should parse")
                .into(),
        );

        let command = connection
            .proxy_command("some-identifier".to_string(), false)
            .expect("building the proxy command should succeed");

        assert_eq!(command.program, "docker");
        assert_eq!(
            without_env_args(&command.args),
            vec![
                "exec",
                "-u",
                "anth",
                "-w",
                "/home/anth",
                "-i",
                "container-123",
                "zed-remote-server",
                "proxy",
                "--identifier",
                "some-identifier",
            ]
        );
    }

    /// A terminal is `docker exec -it`, and the hop that carries it has to ask
    /// for a TTY as well — the docker CLI refuses with "the input device is
    /// not a TTY" when its stdin is a pipe.
    #[gpui::test]
    async fn an_interactive_command_asks_the_host_for_a_tty(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let mut connection = local_connection(docker_options());
        connection.host = Some(mock_host(cx, server_cx).await);

        let interactive = connection
            .build_command(None, &[], &Default::default(), None, None, Interactive::Yes)
            .expect("building an interactive command should succeed");
        assert_eq!(interactive.args.first().map(String::as_str), Some("-t"));
        assert!(interactive.args.contains(&"-it".to_string()));

        let batch = connection
            .build_command(
                Some("ls".to_string()),
                &[],
                &Default::default(),
                None,
                None,
                Interactive::No,
            )
            .expect("building a non-interactive command should succeed");
        assert_eq!(batch.args.first().map(String::as_str), Some("-T"));
        assert!(batch.args.contains(&"-i".to_string()));
    }

    /// The container's ports are not published on the daemon's machine, but
    /// they are reachable there at the container's own address, so the
    /// forward is the host's with its destination re-pointed.
    #[gpui::test]
    async fn ports_are_forwarded_through_the_containers_host(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let mut connection = local_connection(docker_options());

        assert!(
            connection
                .build_forward_ports_command(vec![(9000, "localhost".to_string(), 3000)])
                .is_err(),
            "a container on this machine has no hop to forward through"
        );

        connection.host = Some(mock_host(cx, server_cx).await);
        let command = connection
            .build_forward_ports_command(vec![(9000, "localhost".to_string(), 3000)])
            .expect("forwarding through the host should succeed");
        assert_eq!(command.program, "mock");
        assert_eq!(command.args, vec!["-N", "9000:172.17.0.2:3000"]);

        assert!(
            connection
                .build_forward_ports_command(vec![(9000, "elsewhere.internal".to_string(), 3000)])
                .is_err(),
            "a destination that is not the container itself must not be re-pointed at it"
        );

        connection.container_address = None;
        assert!(
            connection
                .build_forward_ports_command(vec![(9000, "localhost".to_string(), 3000)])
                .is_err(),
            "forwarding to an unknown address would silently reach the daemon's machine"
        );
    }

    #[gpui::test]
    async fn proxy_command_runs_docker_on_the_host(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let mut connection = local_connection(docker_options());
        connection.host = Some(mock_host(cx, server_cx).await);
        connection.remote_binary_relpath = Some(
            RelPath::from_unix_str("zed-remote-server")
                .expect("a relative unix path should parse")
                .into(),
        );

        let command = connection
            .proxy_command("some-identifier".to_string(), true)
            .expect("building the proxy command should succeed");

        // The spawned child is the host transport; the docker CLI is the
        // program it is asked to run over there.
        assert_eq!(command.program, "mock");
        assert_eq!(
            without_env_args(&command.args),
            vec![
                // The proxy's stdio is framed protobuf, so the hop must not
                // allocate a TTY.
                "-T",
                "docker",
                "exec",
                "-u",
                "anth",
                "-w",
                "/home/anth",
                "-i",
                "container-123",
                "zed-remote-server",
                "proxy",
                "--identifier",
                "some-identifier",
                "--reconnect",
            ]
        );
    }

    /// The host may be carrying an ordinary remote project as well, so closing
    /// a dev container must leave it running. Regression test for the shared
    /// SSH master being torn down with the container.
    #[cfg(unix)]
    #[gpui::test]
    async fn killing_a_dev_container_leaves_its_host_alive(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let host = mock_host(cx, server_cx).await;
        let mut connection = local_connection(docker_options());
        connection.host = Some(host.clone());

        // Stand in for a running proxy child so that `kill` takes the branch
        // that kills something. `u32::MAX` is not a live pid, so the signal
        // lands nowhere.
        *connection.proxy_process.lock() = Some(u32::MAX);
        connection
            .kill()
            .await
            .expect("killing the proxy should succeed");

        assert!(
            !host.was_killed(),
            "killing the dev container must not kill the host connection"
        );
        assert!(
            connection.proxy_process.lock().is_none(),
            "the proxy must be forgotten so a later kill does not signal a reused pid"
        );

        drop(connection);
        assert!(
            !host.was_killed(),
            "dropping the dev container must not kill the host connection"
        );
    }

    #[gpui::test]
    async fn server_binary_is_streamed_into_a_container_on_a_remote_host(
        cx: &mut TestAppContext,
        server_cx: &mut TestAppContext,
    ) {
        let mut connection = local_connection(docker_options());
        connection.host = Some(mock_host(cx, server_cx).await);

        let command = connection
            .stream_into_container_command("/home/anth/.zed_server/zed-remote-server")
            .expect("building the streaming command should succeed");

        assert_eq!(command.program, "mock");
        assert_eq!(
            command.args,
            vec![
                "-T",
                "docker",
                "exec",
                "-i",
                "-u",
                "anth",
                "container-123",
                "sh",
                "-c",
                "cat > \"$1\"",
                "zed-upload",
                "/home/anth/.zed_server/zed-remote-server",
            ]
        );
    }

    /// The container is stood in for by the local shell, so that the piping
    /// itself is exercised: the bytes have to reach the child's stdin and land
    /// at the destination path.
    ///
    /// Deliberately not a `gpui::test`: spawning a real process parks the
    /// thread, which the test scheduler forbids.
    #[cfg(unix)]
    #[test]
    fn streaming_writes_the_source_file_to_the_destination_path() {
        let source = tempfile::NamedTempFile::new().expect("creating a source file should succeed");
        std::fs::write(source.path(), b"zed-remote-server bytes")
            .expect("writing the source file should succeed");
        let destination =
            tempfile::NamedTempFile::new().expect("creating a destination file should succeed");

        let command = CommandTemplate {
            program: "sh".to_string(),
            args: vec![
                "-c".to_string(),
                "cat > \"$1\"".to_string(),
                "zed-upload".to_string(),
                destination.path().to_string_lossy().to_string(),
            ],
            env: Default::default(),
        };

        smol::block_on(DockerExecConnection::stream_file_into(
            source.path(),
            command,
        ))
        .expect("streaming into the destination should succeed");

        assert_eq!(
            std::fs::read(destination.path()).expect("reading the destination should succeed"),
            b"zed-remote-server bytes"
        );
    }

    /// A dev extension is a directory, and `docker cp` cannot carry one to a
    /// daemon on another machine. The local shell stands in for the container
    /// so the archive is really packed, piped, and unpacked.
    ///
    /// Deliberately not a `gpui::test`: spawning a real process parks the
    /// thread, which the test scheduler forbids.
    #[cfg(unix)]
    #[test]
    fn a_directory_is_streamed_into_the_container_as_an_archive() {
        let source = tempfile::tempdir().expect("creating a source directory should succeed");
        std::fs::create_dir(source.path().join("languages"))
            .expect("creating a nested directory should succeed");
        std::fs::write(source.path().join("extension.toml"), b"id = \"nextflow\"")
            .expect("writing the manifest should succeed");
        std::fs::write(
            source.path().join("languages/config.toml"),
            b"name = \"nf\"",
        )
        .expect("writing a nested file should succeed");

        let destination =
            tempfile::tempdir().expect("creating a destination directory should succeed");
        let destination = destination.path().join("nested/nextflow");

        let command = CommandTemplate {
            program: "sh".to_string(),
            args: vec![
                "-c".to_string(),
                "mkdir -p \"$1\" && tar -xf - -C \"$1\"".to_string(),
                "zed-upload".to_string(),
                destination.to_string_lossy().to_string(),
            ],
            env: Default::default(),
        };

        smol::block_on(async {
            let archive = DockerExecConnection::archive_directory(source.path())
                .await
                .expect("packing the directory should succeed");
            DockerExecConnection::stream_file_into(archive.path(), command).await
        })
        .expect("streaming the archive should succeed");

        assert_eq!(
            std::fs::read(destination.join("extension.toml"))
                .expect("the manifest should have arrived"),
            b"id = \"nextflow\""
        );
        assert_eq!(
            std::fs::read(destination.join("languages/config.toml"))
                .expect("nested files should have arrived"),
            b"name = \"nf\""
        );
    }

    /// A non-zero exit has to fail the upload rather than leave a truncated
    /// server binary in place.
    #[cfg(unix)]
    #[test]
    fn streaming_reports_a_failing_destination_command() {
        let source = tempfile::NamedTempFile::new().expect("creating a source file should succeed");

        let command = CommandTemplate {
            program: "sh".to_string(),
            args: vec![
                "-c".to_string(),
                "cat > /dev/null; echo 'no such container' >&2; exit 1".to_string(),
            ],
            env: Default::default(),
        };

        let error = smol::block_on(DockerExecConnection::stream_file_into(
            source.path(),
            command,
        ))
        .expect_err("a failing destination command should fail the upload");
        assert!(error.to_string().contains("no such container"));
    }

    #[test]
    fn local_host_upload_commands_argv_is_unchanged() {
        let connection = local_connection(docker_options());
        let (copy_command, chown_command) = connection
            .upload_commands("/tmp/src", "/home/anth/dst")
            .expect("building upload commands should succeed");

        assert_eq!(copy_command.program, "docker");
        assert_eq!(
            copy_command.args,
            vec!["cp", "-a", "/tmp/src", "container-123:/home/anth/dst"]
        );
        assert_eq!(
            chown_command.args,
            vec![
                "exec",
                "container-123",
                "chown",
                "anth:anth",
                "/home/anth/dst",
            ]
        );
    }

    #[test]
    fn options_without_host_deserialize_as_local() {
        let legacy = r#"{
            "name": "zed-dev",
            "container_id": "container-123",
            "remote_user": "anth",
            "upload_binary_over_docker_exec": false,
            "use_podman": false,
            "remote_env": {}
        }"#;

        let options: DockerConnectionOptions =
            serde_json::from_str(legacy).expect("legacy payload should deserialize");
        assert_eq!(options.host, DockerHost::Local);
    }

    #[test]
    fn options_with_ssh_host_round_trip() {
        let options = DockerConnectionOptions {
            name: "zed-dev".to_string(),
            container_id: "container-123".to_string(),
            remote_user: "anth".to_string(),
            upload_binary_over_docker_exec: false,
            use_podman: false,
            remote_env: Default::default(),
            host: DockerHost::Ssh(SshConnectionOptions {
                host: "example.com".into(),
                username: Some("anth".to_string()),
                port: Some(2222),
                ..Default::default()
            }),
        };

        let encoded = serde_json::to_string(&options).expect("options should serialize");
        let decoded: DockerConnectionOptions =
            serde_json::from_str(&encoded).expect("options should deserialize");
        assert_eq!(decoded, options);
    }
}
