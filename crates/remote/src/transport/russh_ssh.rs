use crate::{
    RemoteClientDelegate, RemotePlatform,
    json_log::LogRecord,
    protocol::{MESSAGE_LEN_SIZE, message_len_from_buffer},
    remote_client::{CommandTemplate, Interactive, RemoteConnection, RemoteConnectionOptions},
    transport::{parse_platform, parse_shell},
    transport::ssh::SshConnectionOptions,
};
use askpass::IKnowWhatIAmDoingAndIHaveReadTheDocs;
use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AsyncApp, Task};
use gpui_tokio::Tokio;
use prost::Message as _;
use rpc::proto::Envelope;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use util::paths::{PathStyle, RemotePathBuf};

const DEFAULT_SHELL: &str = "/bin/bash";

struct ClientHandler;

impl russh::client::Handler for ClientHandler {
    type Error = anyhow::Error;

    // check_server_key default rejects all keys; override to accept for now
    #[allow(refining_impl_trait)]
    async fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

type SessionHandle = russh::client::Handle<ClientHandler>;

pub struct RusshRemoteConnection {
    session: Arc<tokio::sync::Mutex<SessionHandle>>,
    connection_options: SshConnectionOptions,
    remote_platform: RemotePlatform,
    remote_shell: String,
    default_system_shell: String,
    killed: AtomicBool,
}

async fn exec_remote_command(
    session: &Arc<tokio::sync::Mutex<SessionHandle>>,
    command: &str,
) -> Result<String> {
    let handle = session.lock().await;
    let channel = handle.channel_open_session().await?;
    channel.exec(true, command.as_bytes().to_vec()).await?;
    drop(handle);

    let (mut read_half, _write_half) = channel.split();
    let mut output = Vec::new();
    while let Some(msg) = read_half.wait().await {
        match msg {
            russh::ChannelMsg::Data { data } => output.extend_from_slice(&data),
            russh::ChannelMsg::Eof | russh::ChannelMsg::Close => break,
            _ => {}
        }
    }
    Ok(String::from_utf8_lossy(&output).to_string())
}

impl RusshRemoteConnection {
    pub async fn new(
        connection_options: SshConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let host = connection_options.host.to_string();
        let port = connection_options.port.unwrap_or(22);
        let username = connection_options
            .username
            .clone()
            .unwrap_or_else(|| "root".to_string());

        delegate.set_status(Some("Connecting"), cx);

        let config = Arc::new(russh::client::Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(30)),
            keepalive_interval: Some(std::time::Duration::from_secs(15)),
            keepalive_max: 3,
            ..Default::default()
        });

        let session: SessionHandle = Tokio::spawn_result(cx, {
            let config = config.clone();
            let addr = format!("{}:{}", host, port);
            async move {
                russh::client::connect(config, addr, ClientHandler)
                    .await
                    .map_err(anyhow::Error::from)
            }
        })
        .await?;

        let session = Arc::new(tokio::sync::Mutex::new(session));

        delegate.set_status(Some("Authenticating"), cx);

        // Try password if one was provided with the connection options
        let mut authenticated = false;
        if let Some(password) = &connection_options.password {
            let result = Tokio::spawn_result(cx, {
                let session = session.clone();
                let username = username.clone();
                let password = password.clone();
                async move {
                    let mut handle = session.lock().await;
                    handle
                        .authenticate_password(&username, &password)
                        .await
                        .map_err(anyhow::Error::from)
                }
            })
            .await?;

            authenticated = matches!(result, russh::client::AuthResult::Success);
        }

        if !authenticated {
            // Prompt for password via delegate
            let (password_tx, password_rx) = futures::channel::oneshot::channel();
            delegate.ask_password(
                format!("Password for {}@{}:", username, host),
                password_tx,
                cx,
            );

            let encrypted_password = password_rx
                .await
                .context("password prompt was cancelled")?;
            let password =
                encrypted_password.decrypt(IKnowWhatIAmDoingAndIHaveReadTheDocs)?;

            let result = Tokio::spawn_result(cx, {
                let session = session.clone();
                let username = username.clone();
                async move {
                    let mut handle = session.lock().await;
                    handle
                        .authenticate_password(&username, &password)
                        .await
                        .map_err(anyhow::Error::from)
                }
            })
            .await?;

            if !matches!(result, russh::client::AuthResult::Success) {
                anyhow::bail!("authentication failed: incorrect password");
            }
        }

        delegate.set_status(Some("Probing remote"), cx);

        let uname_output = Tokio::spawn_result(cx, {
            let session = session.clone();
            async move { exec_remote_command(&session, "uname -sm").await }
        })
        .await?;
        let remote_platform = parse_platform(&uname_output)?;

        let shell_output = Tokio::spawn_result(cx, {
            let session = session.clone();
            async move { exec_remote_command(&session, "echo $SHELL").await }
        })
        .await?;
        let remote_shell = parse_shell(&shell_output, DEFAULT_SHELL);

        Ok(Self {
            session,
            connection_options,
            remote_platform,
            remote_shell: remote_shell.clone(),
            default_system_shell: remote_shell,
            killed: AtomicBool::new(false),
        })
    }
}

#[async_trait(?Send)]
impl RemoteConnection for RusshRemoteConnection {
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

        let session = self.session.clone();

        let mut proxy_command = String::from("zed --headless proxy");
        proxy_command.push_str(&format!(" --identifier {}", unique_identifier));
        if reconnect {
            proxy_command.push_str(" --reconnect");
        }

        Tokio::spawn_result(cx, async move {
            let handle = session.lock().await;
            let channel = handle.channel_open_session().await?;
            channel.exec(true, proxy_command.into_bytes()).await?;
            drop(handle);

            let (mut read_half, write_half) = channel.split();

            // Writer task: outgoing Envelopes → length-prefixed protobuf → SSH channel
            let stdin_task = tokio::spawn({
                async move {
                    use futures::StreamExt;
                    let mut outgoing_rx = outgoing_rx;
                    while let Some(outgoing) = outgoing_rx.next().await {
                        let message_len = outgoing.encoded_len() as u32;
                        let mut buffer = Vec::with_capacity(4 + message_len as usize);
                        buffer.extend_from_slice(&message_len.to_le_bytes());
                        outgoing.encode(&mut buffer)?;

                        write_half.data(&buffer[..]).await
                            .map_err(|e| anyhow::anyhow!("{}", e))?;
                    }
                    anyhow::Ok(())
                }
            });

            // Reader task: SSH channel → reassemble length-prefixed protobuf → Envelopes
            let stdout_task = tokio::spawn({
                let mut connection_activity_tx = connection_activity_tx.clone();
                async move {
                    let mut buffer = Vec::new();

                    loop {
                        // Accumulate data until we have a complete message
                        while buffer.len() < MESSAGE_LEN_SIZE
                            || buffer.len()
                                < MESSAGE_LEN_SIZE
                                    + message_len_from_buffer(&buffer[..MESSAGE_LEN_SIZE]) as usize
                        {
                            match read_half.wait().await {
                                Some(russh::ChannelMsg::Data { data }) => {
                                    buffer.extend_from_slice(&data);
                                }
                                Some(russh::ChannelMsg::ExtendedData { data, ext })
                                    if ext == 1 =>
                                {
                                    handle_stderr_data(&data);
                                    connection_activity_tx.try_send(()).ok();
                                    continue;
                                }
                                Some(russh::ChannelMsg::ExitStatus { exit_status }) => {
                                    return anyhow::Ok(Some(exit_status));
                                }
                                Some(russh::ChannelMsg::Eof | russh::ChannelMsg::Close) => {
                                    return Ok(None);
                                }
                                Some(_) => continue,
                                None => return Ok(None),
                            }
                        }

                        let message_len =
                            message_len_from_buffer(&buffer[..MESSAGE_LEN_SIZE]) as usize;
                        let total_len = MESSAGE_LEN_SIZE + message_len;

                        let envelope = Envelope::decode(&buffer[MESSAGE_LEN_SIZE..total_len])?;
                        buffer.drain(..total_len);

                        connection_activity_tx.try_send(()).ok();
                        incoming_tx.unbounded_send(envelope).ok();
                    }
                }
            });

            tokio::select! {
                result = stdin_task => {
                    result?.context("stdin task")?;
                    Ok(0)
                }
                result = stdout_task => {
                    let status = result?.context("stdout task")?;
                    Ok(status.unwrap_or(0) as i32)
                }
            }
        })
    }

    fn upload_directory(
        &self,
        _src_path: std::path::PathBuf,
        _dest_path: RemotePathBuf,
        _cx: &App,
    ) -> Task<Result<()>> {
        Task::ready(Err(anyhow!(
            "upload_directory is not yet supported over russh"
        )))
    }

    async fn kill(&self) -> Result<()> {
        self.killed.store(true, Ordering::SeqCst);
        // Try to disconnect gracefully. The tokio mutex may not be
        // immediately available if the message pump holds it, but
        // try_lock avoids blocking the GPUI foreground thread.
        if let Ok(handle) = self.session.try_lock() {
            let _ = handle
                .disconnect(russh::Disconnect::ByApplication, "", "en")
                .await;
        }
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        self.killed.load(Ordering::SeqCst)
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
        anyhow::bail!("build_command is not supported on iOS (no process spawning)")
    }

    fn build_forward_ports_command(
        &self,
        _forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        anyhow::bail!("port forwarding commands are not yet supported over russh")
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Ssh(self.connection_options.clone())
    }

    fn path_style(&self) -> PathStyle {
        if self.remote_platform.os.is_windows() {
            PathStyle::Windows
        } else {
            PathStyle::Posix
        }
    }

    fn shell(&self) -> String {
        self.remote_shell.clone()
    }

    fn default_system_shell(&self) -> String {
        self.default_system_shell.clone()
    }

    fn has_wsl_interop(&self) -> bool {
        false
    }
}

fn handle_stderr_data(data: &[u8]) {
    for line in data.split(|b| *b == b'\n') {
        if line.is_empty() {
            continue;
        }
        if let Ok(record) = serde_json::from_slice::<LogRecord>(line) {
            record.log(log::logger());
        } else {
            log::info!("(remote) {}", String::from_utf8_lossy(line));
        }
    }
}
