//! An SSH transport that runs entirely in-process via `russh`.
//!
//! iOS forbids spawning child processes, so the OpenSSH-subprocess transport in
//! [`super::ssh`] cannot work there. This transport keeps the same
//! [`RemoteConnection`] contract but speaks the SSH protocol itself over a
//! socket owned by the app.
//!
//! Limitations compared to the OpenSSH transport:
//! - password authentication only (no keys, agents, or jump hosts)
//! - the host key is accepted on first use without verification
//! - no local terminals or port forwards (`build_command` would need a local
//!   `ssh` binary to hand the command to)

use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering::SeqCst},
};

use anyhow::{Context as _, Result, anyhow};
use askpass::IKnowWhatIAmDoingAndIHaveReadTheDocs;
use async_trait::async_trait;
use collections::HashMap;
use futures::{
    StreamExt as _,
    channel::{
        mpsc::{Sender, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use gpui::{App, AsyncApp, Task};
use gpui_tokio::Tokio;
use prost::Message as _;
use release_channel::ReleaseChannel;
use rpc::proto::Envelope;
use russh::{ChannelMsg, client::AuthResult};
use util::paths::{PathStyle, RemotePathBuf};
use util::rel_path::RelPath;

use crate::protocol::{MESSAGE_LEN_SIZE, message_len_from_buffer};
use crate::remote_client::{
    CommandTemplate, Interactive, RemoteClientDelegate, RemoteConnection, RemoteConnectionOptions,
    RemoteOs, RemotePlatform,
};

use super::ssh::SshConnectionOptions;

struct ClientHandler;

impl russh::client::Handler for ClientHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &russh::keys::PublicKeyOrCertificate,
    ) -> Result<bool, Self::Error> {
        // Trust-on-first-use without persistence. Fine for a development
        // preview; a production build must verify against known hosts.
        log::info!("accepting SSH host key: {server_public_key:?}");
        Ok(true)
    }
}

type SshHandle = russh::client::Handle<ClientHandler>;

pub(crate) struct InProcessSshConnection {
    handle: Arc<SshHandle>,
    connection_options: SshConnectionOptions,
    platform: RemotePlatform,
    os_version: Option<String>,
    remote_binary_path: Arc<RelPath>,
    killed: AtomicBool,
}

impl InProcessSshConnection {
    pub(crate) async fn new(
        options: SshConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        delegate.set_status(Some("Connecting"), cx);

        let host = options.host.to_string();
        let port = options.port.unwrap_or(22);
        let username = options
            .username
            .clone()
            .context("An explicit user is required (use user@host)")?;

        let config = Arc::new(russh::client::Config {
            inactivity_timeout: None,
            keepalive_interval: Some(std::time::Duration::from_secs(30)),
            ..Default::default()
        });

        let handle = {
            let host = host.clone();
            Tokio::spawn_result(cx, async move {
                russh::client::connect(config, (host.as_str(), port), ClientHandler)
                    .await
                    .map_err(|error| anyhow!("failed to connect to {host}:{port}: {error}"))
            })
            .await?
        };

        // Try the default client keys first, then fall back to a password
        // prompt.
        let mut private_keys = Vec::new();
        if let Some(home) = std::env::var_os("HOME") {
            for name in ["id_ed25519", "id_rsa"] {
                let path = std::path::Path::new(&home).join(".ssh").join(name);
                if let Ok(key) = russh::keys::load_secret_key(&path, None) {
                    private_keys.push(key);
                }
            }
        }

        let (mut success, mut handle) = {
            let username = username.clone();
            Tokio::spawn_result(cx, async move {
                let mut handle = handle;
                let mut success = false;
                for key in private_keys {
                    let key = russh::keys::PrivateKeyWithHashAlg::new(
                        Arc::new(key),
                        handle.best_supported_rsa_hash().await?.flatten(),
                    );
                    let result = handle
                        .authenticate_publickey(username.clone(), key)
                        .await
                        .context("SSH public key authentication failed")?;
                    if matches!(result, AuthResult::Success) {
                        success = true;
                        break;
                    }
                }
                anyhow::Ok((success, handle))
            })
            .await?
        };

        if !success {
            let password = match options.password.clone() {
                Some(password) => password,
                None => {
                    let (tx, rx) = oneshot::channel();
                    let (_cancel_tx, cancel_rx) = oneshot::channel();
                    delegate.ask_password(
                        format!("{username}@{host}'s password:"),
                        tx,
                        cancel_rx,
                        cx,
                    );
                    rx.await
                        .map_err(|_| anyhow!("password prompt was cancelled"))?
                        .decrypt(IKnowWhatIAmDoingAndIHaveReadTheDocs)?
                }
            };
            (success, handle) = {
                let username = username.clone();
                Tokio::spawn_result(cx, async move {
                    let mut handle = handle;
                    let result = handle
                        .authenticate_password(username, password)
                        .await
                        .context("SSH password authentication failed")?;
                    anyhow::Ok((matches!(result, AuthResult::Success), handle))
                })
                .await?
            };
        }
        anyhow::ensure!(success, "the SSH server rejected the credentials");
        let handle = Arc::new(handle);

        delegate.set_status(Some("Detecting remote platform"), cx);
        let uname = run_command(&handle, "uname -sm", cx).await?;
        let platform = super::parse_platform(&uname)?;
        anyhow::ensure!(
            platform.os != RemoteOs::Windows,
            "in-process SSH does not support Windows hosts yet"
        );

        let (os_version_program, os_version_args) = super::os_version_command(platform.os);
        let os_version_command = std::iter::once(os_version_program)
            .chain(os_version_args.iter().copied())
            .collect::<Vec<_>>()
            .join(" ");
        let os_version = match run_command(&handle, &os_version_command, cx).await {
            Ok(output) => super::parse_os_version(platform.os, &output),
            Err(_) => None,
        };

        let remote_binary_path = ensure_server_binary(&handle, platform, &delegate, cx).await?;

        Ok(Self {
            handle,
            connection_options: options,
            platform,
            os_version,
            remote_binary_path,
            killed: AtomicBool::new(false),
        })
    }
}

#[async_trait(?Send)]
impl RemoteConnection for InProcessSshConnection {
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

        let mut command = format!(
            "{} proxy --identifier {}",
            shell_quote(&self.remote_binary_path.display(self.path_style())),
            shell_quote(&unique_identifier),
        );
        if reconnect {
            command.push_str(" --reconnect");
        }

        let handle = self.handle.clone();
        Tokio::spawn_result(cx, async move {
            run_proxy(
                handle,
                command,
                incoming_tx,
                outgoing_rx,
                connection_activity_tx,
            )
            .await
        })
    }

    fn upload_directory(
        &self,
        _src_path: PathBuf,
        _dest_path: RemotePathBuf,
        _cx: &App,
    ) -> Task<Result<()>> {
        Task::ready(Err(anyhow!(
            "uploading directories is not supported by the in-process SSH transport yet"
        )))
    }

    async fn kill(&self) -> Result<()> {
        self.killed.store(true, SeqCst);
        self.handle
            .disconnect(russh::Disconnect::ByApplication, "", "en")
            .await
            .ok();
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        self.killed.load(SeqCst) || self.handle.is_closed()
    }

    fn build_command(
        &self,
        _program: Option<String>,
        _args: &[String],
        _env: &HashMap<String, String>,
        _working_dir: Option<String>,
        _port_forward: Option<(u16, String, u16)>,
        _interactive: Interactive,
    ) -> Result<CommandTemplate> {
        anyhow::bail!(
            "local terminals and port forwards are not supported by the in-process SSH transport"
        )
    }

    fn build_forward_ports_command(
        &self,
        _forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        anyhow::bail!("port forwards are not supported by the in-process SSH transport")
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Ssh(self.connection_options.clone())
    }

    fn path_style(&self) -> PathStyle {
        PathStyle::Unix
    }

    fn remote_platform(&self) -> RemotePlatform {
        self.platform
    }

    fn remote_os_version(&self) -> Option<String> {
        self.os_version.clone()
    }

    fn shell(&self) -> String {
        "sh".to_string()
    }

    fn default_system_shell(&self) -> String {
        "sh".to_string()
    }

    fn has_wsl_interop(&self) -> bool {
        false
    }
}

/// Runs `command` on the remote host, returning its stdout on exit status 0.
async fn run_command(handle: &Arc<SshHandle>, command: &str, cx: &AsyncApp) -> Result<String> {
    let handle = handle.clone();
    let command = command.to_string();
    let task = Tokio::spawn_result(cx, async move {
        let mut channel = handle.channel_open_session().await?;
        channel.exec(true, command.as_bytes()).await?;

        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut exit_status = None;
        while let Some(message) = channel.wait().await {
            match message {
                ChannelMsg::Data { data } => stdout.extend_from_slice(&data),
                ChannelMsg::ExtendedData { data, .. } => stderr.extend_from_slice(&data),
                ChannelMsg::ExitStatus { exit_status: code } => exit_status = Some(code),
                _ => {}
            }
        }
        let stdout = String::from_utf8_lossy(&stdout).into_owned();
        match exit_status {
            Some(0) => anyhow::Ok(stdout),
            status => Err(anyhow!(
                "remote command exited with {status:?}: {}",
                String::from_utf8_lossy(&stderr)
            )),
        }
    });
    task.await
}

async fn run_proxy(
    handle: Arc<SshHandle>,
    command: String,
    incoming_tx: UnboundedSender<Envelope>,
    mut outgoing_rx: UnboundedReceiver<Envelope>,
    connection_activity_tx: Sender<()>,
) -> Result<i32> {
    let mut channel = handle.channel_open_session().await?;
    channel.exec(true, command.as_bytes()).await?;

    let mut read_buffer: Vec<u8> = Vec::new();
    let mut stderr_buffer: Vec<u8> = Vec::new();
    let mut write_buffer: Vec<u8> = Vec::new();
    let mut exit_status: i32 = 1;

    loop {
        tokio::select! {
            message = channel.wait() => {
                let Some(message) = message else { break };
                match message {
                    ChannelMsg::Data { data } => {
                        connection_activity_tx.clone().try_send(()).ok();
                        read_buffer.extend_from_slice(&data);
                        while read_buffer.len() >= MESSAGE_LEN_SIZE {
                            let len =
                                message_len_from_buffer(&read_buffer[..MESSAGE_LEN_SIZE]) as usize;
                            if read_buffer.len() < MESSAGE_LEN_SIZE + len {
                                break;
                            }
                            let envelope = Envelope::decode(
                                &read_buffer[MESSAGE_LEN_SIZE..MESSAGE_LEN_SIZE + len],
                            )?;
                            read_buffer.drain(..MESSAGE_LEN_SIZE + len);
                            if incoming_tx.unbounded_send(envelope).is_err() {
                                break;
                            }
                        }
                    }
                    ChannelMsg::ExtendedData { data, .. } => {
                        stderr_buffer.extend_from_slice(&data);
                        while let Some(newline) = stderr_buffer.iter().position(|&b| b == b'\n') {
                            let line: Vec<u8> = stderr_buffer.drain(..=newline).collect();
                            log::warn!(
                                "remote server: {}",
                                String::from_utf8_lossy(&line).trim_end()
                            );
                        }
                    }
                    ChannelMsg::ExitStatus { exit_status: code } => {
                        exit_status = code as i32;
                    }
                    _ => {}
                }
            }
            envelope = outgoing_rx.next() => {
                match envelope {
                    Some(envelope) => {
                        let len = envelope.encoded_len() as u32;
                        write_buffer.clear();
                        write_buffer.extend_from_slice(&len.to_le_bytes());
                        envelope.encode(&mut write_buffer)?;
                        channel.data(&write_buffer[..]).await?;
                    }
                    None => {
                        channel.eof().await.ok();
                    }
                }
            }
        }
    }

    Ok(exit_status)
}

async fn ensure_server_binary(
    handle: &Arc<SshHandle>,
    platform: RemotePlatform,
    delegate: &Arc<dyn RemoteClientDelegate>,
    cx: &mut AsyncApp,
) -> Result<Arc<RelPath>> {
    let (release_channel, version) = cx.update(|cx| {
        (
            ReleaseChannel::try_global(cx).unwrap_or(ReleaseChannel::Dev),
            release_channel::AppVersion::global(cx),
        )
    });
    let version_str = match release_channel {
        ReleaseChannel::Dev => "build".to_string(),
        _ => version.to_string(),
    };
    let binary_name = format!(
        "zed-remote-server-{}-{}",
        release_channel.dev_name(),
        version_str
    );
    let dst_path = paths::remote_server_dir_relative()
        .join(RelPath::from_unix_str(&binary_name).context("invalid binary name")?);
    let dst_display = dst_path.display(PathStyle::Unix).into_owned();

    delegate.set_status(Some("Checking for remote development server"), cx);
    if run_command(
        handle,
        &format!("{} version", shell_quote(&dst_display)),
        cx,
    )
    .await
    .is_ok()
    {
        return Ok(dst_path.into());
    }

    // Try to download a matching release directly on the host, like the
    // OpenSSH transport does.
    if let Some(wanted_version) = match release_channel {
        ReleaseChannel::Dev => None,
        ReleaseChannel::Nightly => Some(None),
        _ => Some(Some(version)),
    } && let Some(url) = delegate
        .get_download_url(platform, release_channel, wanted_version, cx)
        .await?
    {
        delegate.set_status(Some("Downloading remote development server on host"), cx);
        let tmp_path = format!("{dst_display}.download.gz");
        run_command(
            handle,
            &format!(
                "mkdir -p {} && (curl -fsSL {} -o {} || wget -qO {} {})",
                shell_quote(&paths::remote_server_dir_relative().display(PathStyle::Unix)),
                shell_quote(&url),
                shell_quote(&tmp_path),
                shell_quote(&tmp_path),
                shell_quote(&url),
            ),
            cx,
        )
        .await
        .context("downloading the remote server on the host")?;
        run_command(
            handle,
            &format!(
                "gunzip -f {} && mv {} {} && chmod +x {}",
                shell_quote(&tmp_path),
                shell_quote(tmp_path.trim_end_matches(".gz")),
                shell_quote(&dst_display),
                shell_quote(&dst_display),
            ),
            cx,
        )
        .await
        .context("extracting the remote server on the host")?;
        return Ok(dst_path.into());
    }

    anyhow::bail!(
        "No remote development server found on the host. \
         Install one at ~/{dst_display} (for dev builds: \
         `cargo build -p remote_server` on the host, then copy \
         target/debug/remote_server there)."
    )
}

fn shell_quote(text: &str) -> String {
    if !text.is_empty()
        && text
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_./=:@%+,~".contains(c))
    {
        text.to_string()
    } else {
        format!("'{}'", text.replace('\'', r"'\''"))
    }
}
