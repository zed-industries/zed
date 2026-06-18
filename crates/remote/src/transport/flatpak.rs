use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::{borrow::Cow, sync::Arc};
use util::command::Stdio;
use util::flatpak;
use util::paths::{PathStyle, RemotePathBuf};

use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext as _, AsyncApp, Task};
use rpc::proto::Envelope;

use crate::{
    RemoteClientDelegate, RemoteConnection, RemoteConnectionOptions, RemotePlatform,
    remote_client::{CommandTemplate, Interactive},
    transport::{os_version_command, parse_os_version, parse_platform},
};

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod document_portal;
mod host_spawn;

use host_spawn::HostCommand;

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct FlatpakConnectionOptions {
    pub remote_env: BTreeMap<String, String>,
}

impl FlatpakConnectionOptions {
    /// Translate a Path as seen from inside a Flatpak sandbox to the corresponding host path.
    ///
    /// When Zed is running inside a Flatpak sandbox, the paths returned by the standard
    /// file chooser are part of the XDG Document Portal FUSE filesystem, not the actual
    /// path on the host that the user chose. These paths need translation to make sense
    /// to any tooling outside the Flatpak sandbox.
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub async fn as_host_path<'a>(&self, path: &'a Path) -> Result<Cow<'a, Path>> {
        match document_portal::lookup_host_path(path).await {
            Ok(None) => Ok(Cow::from(path)),
            Ok(Some(host_path)) => Ok(Cow::from(host_path)),
            Err(err) => {
                // Ignore errors from the connection to the document portal service
                if document_portal::is_likely_document_portal_path(path) {
                    log::error!("while resolving likely document portal path {path:?}: {err}");
                }
                Ok(Cow::from(path))
            }
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    pub async fn as_host_path<'a>(&self, path: &'a Path) -> Result<Cow<'a, Path>> {
        Ok(Cow::from(path))
    }
}

pub(crate) struct FlatpakHostConnection {
    proxy_process: Mutex<Option<u32>>,
    remote_binary_host_path: String,
    connection_options: FlatpakConnectionOptions,
    shell: String,
    platform: RemotePlatform,
    os_version: Option<String>,
}

impl FlatpakHostConnection {
    pub async fn new(
        connection_options: FlatpakConnectionOptions,
        _delegate: Arc<dyn RemoteClientDelegate>,
        _cx: &mut AsyncApp,
    ) -> Result<Self> {
        let meta = flatpak::CURRENT_SANDBOX_METADATA.as_ref().ok_or_else(|| {
            anyhow!("Flatpak host transport is only available inside a Flatpak sandbox")
        })?;

        // The Flatpak bundles a matching zed-remote-server, which we find at this path:
        let remote_binary_host_path = format!(
            "{}/libexec/zed-remote-server",
            meta.app_host_path()?.trim_end_matches("/")
        );
        HostCommand::new(&remote_binary_host_path, None)
            .arg("version")
            .output()
            .await
            .context("bundled zed-remote-server failed to execute")?;

        // Detect the host platform
        let platform = parse_platform(
            &HostCommand::new("uname", None)
                .arg("-sm")
                .output()
                .await
                .context("failed to run uname -sm on target system")?,
        )?;

        // Detect the host OS version
        let (program, args) = os_version_command(platform.os);
        let os_version = match HostCommand::new(program, None).args(args).output().await {
            Ok(output) => parse_os_version(platform.os, &output),
            Err(err) => {
                log::warn!("failed to detect host OS version: {err}");
                None
            }
        };

        // Detect the user's preferred shell
        let shell = match HostCommand::new("/bin/sh", None)
            .arg("-c")
            .arg("echo $SHELL")
            .output()
            .await
        {
            Ok(shell) if !shell.trim().is_empty() => shell.trim().to_owned(),
            _ => "/bin/sh".to_string(),
        };
        log::info!("Flatpak host shell discovered: {shell}");

        Ok(Self {
            proxy_process: Mutex::new(None),
            remote_binary_host_path,
            connection_options,
            shell,
            platform,
            os_version,
        })
    }
}

#[async_trait(?Send)]
impl RemoteConnection for FlatpakHostConnection {
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
        delegate.set_status(Some("Starting proxy"), cx);

        let mut command = HostCommand::new(&self.remote_binary_host_path, None)
            .current_dir(&paths::home_dir())
            .envs(&self.connection_options.remote_env)
            .arg("proxy")
            .arg("--identifier")
            .arg(unique_identifier);
        if reconnect {
            command = command.arg("--reconnect");
        }
        for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
            if let Ok(value) = std::env::var(env_var) {
                command = command.env(env_var, value);
            }
        }

        let mut command = command.build_command();
        command
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = match command.spawn() {
            Ok(child) => child,
            Err(err) => {
                return Task::ready(Err(anyhow!("Failed to start remote server process: {err}")));
            }
        };

        *self.proxy_process.lock() = Some(child.id());

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
        // The sandbox and host do not share a filesystem, so we use `tar` to bundle the
        // entire directory tree. The tarball itself is fed across the sandbox boundary
        // through the stdin/stdout of the commands, which is forwarded by HostCommand.
        let dest_path = dest_path.to_string();
        cx.background_spawn(async move {
            // Make sure the destination directory exists on the host before transfer
            {
                let mut mkdir_command = HostCommand::new("mkdir", None)
                    .arg("-p")
                    .arg(&dest_path)
                    .build_command();
                mkdir_command
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::inherit())
                    .output()
                    .await
                    .with_context(|| {
                        format!("creating destination directory {dest_path} on the host")
                    })?;
            }

            // Start the subprocesses to do the transfer. Consumer starts first to avoid
            // blocking on filled buffers.
            let mut extract_command = HostCommand::new("tar", None)
                .arg("x")
                .arg("-C")
                .arg(&dest_path)
                .build_command();
            let mut extract_child = extract_command
                .kill_on_drop(true)
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .spawn()
                .context("spawning tar on host for extraction")?;
            let pipe = extract_child
                .stdin
                .take()
                .context("missing tar stdin pipe")?
                .into_stdio()
                .await?;

            let mut archive_command = util::command::new_command("tar");
            archive_command.arg("c").arg("-C").arg(&src_path).arg(".");
            let archive_child = archive_command
                .kill_on_drop(true)
                .stdin(Stdio::null())
                .stdout(pipe)
                .stderr(Stdio::piped())
                .spawn()
                .context("spawning tar in sandbox for archival")?;

            // Wait for the process to complete
            let extract_out = extract_child.output().await?;
            let archive_out = archive_child.output().await?;
            anyhow::ensure!(
                archive_out.status.success() && extract_out.status.success(),
                "failed while transferring directory {} => {}: {} {}",
                src_path.display(),
                dest_path,
                String::from_utf8_lossy(&archive_out.stderr),
                String::from_utf8_lossy(&extract_out.stderr),
            );

            Ok(())
        })
    }

    async fn kill(&self) -> Result<()> {
        if let Some(pid) = self.proxy_process.lock().take() {
            util::command::new_command("kill")
                .arg(pid.to_string())
                .spawn()
                .map(|_| ())
                .map_err(|_| anyhow!("Failed to kill process"))
        } else {
            Ok(())
        }
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
        let mut command = match program {
            Some(program) => {
                HostCommand::new(&program, Some(&self.remote_binary_host_path)).args(args)
            }
            None => HostCommand::new(&self.shell, Some(&self.remote_binary_host_path)),
        };
        command = match working_dir {
            Some(working_dir) => {
                if let Some(relative) = working_dir.strip_prefix("~/") {
                    command.current_dir(paths::home_dir().join(relative))
                } else {
                    command.current_dir(working_dir)
                }
            }
            None => command.current_dir(paths::home_dir()),
        };
        Ok(command
            .envs(env)
            .pty(match interactive {
                Interactive::Yes => true,
                Interactive::No => false,
            })
            .build_template())
    }

    fn build_forward_ports_command(
        &self,
        _forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        Err(anyhow!(
            "port forwarding is not supported for the Flatpak host transport"
        ))
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::FlatpakHost(self.connection_options.clone())
    }

    fn path_style(&self) -> PathStyle {
        // Flatpak only works on Linux, so always POSIX path styles
        PathStyle::Posix
    }

    fn remote_platform(&self) -> RemotePlatform {
        self.platform
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
