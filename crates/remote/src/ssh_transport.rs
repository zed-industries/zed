use anyhow::{Context as _, Result};
use async_trait::async_trait;
use futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use gpui::{App, AppContext as _, AsyncApp, Task};
use parking_lot::Mutex;
use rpc::proto::Envelope;
use smol::process::{Child, Stdio};
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

use crate::ssh_session::{SshConnectionOptions, SshSocket};
use crate::transport::{Transport, TransportConfig, TransportConnection, TransportDelegate};

/// SSH implementation of the Transport trait
pub struct SshTransport;

impl SshTransport {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait(?Send)]
impl Transport for SshTransport {
    async fn connect(
        &self,
        config: &TransportConfig,
        _delegate: Arc<dyn TransportDelegate>,
        _cx: &mut AsyncApp,
    ) -> Result<Box<dyn TransportConnection>> {
        let TransportConfig::Ssh(ssh_config) = config;

        // Create temporary directory for control socket
        let temp_dir = TempDir::new()
            .context("failed to create temporary directory for SSH connection")?;
        
        let socket_path = temp_dir.path().join("ssh.sock");
        
        let socket = SshSocket {
            connection_options: ssh_config.clone(),
            socket_path,
        };

        // Set up SSH master connection
        let mut master_process = util::command::new_smol_command("ssh");
        master_process
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .args(["-N", "-o", "ControlMaster=yes", "-o"])
            .arg(format!("ControlPath={}", socket.socket_path.display()))
            .args(&ssh_config.additional_args())
            .arg(ssh_config.ssh_url());

        let master_process = master_process
            .spawn()
            .context("failed to spawn SSH master process")?;

        // Wait for SSH connection to be established
        // In a real implementation, we'd check the control socket exists
        // For now, we'll just wait a bit
        smol::Timer::after(std::time::Duration::from_secs(1)).await;

        Ok(Box::new(SshRemoteConnectionAdapter {
            _socket: socket,
            master_process: Mutex::new(Some(master_process)),
            _remote_binary_path: None,
            _temp_dir: temp_dir,
            connection_options: ssh_config.clone(),
        }))
    }

    fn name(&self) -> &'static str {
        "ssh"
    }

    fn supports_config(&self, config: &TransportConfig) -> bool {
        matches!(config, TransportConfig::Ssh(_))
    }
}

/// Adapter that implements TransportConnection for SSH
struct SshRemoteConnectionAdapter {
    _socket: SshSocket,
    master_process: Mutex<Option<Child>>,
    _remote_binary_path: Option<PathBuf>,
    _temp_dir: TempDir,
    connection_options: SshConnectionOptions,
}

#[async_trait(?Send)]
impl TransportConnection for SshRemoteConnectionAdapter {
    fn start_proxy(
        &self,
        _unique_identifier: String,
        _reconnect: bool,
        _incoming_tx: UnboundedSender<Envelope>,
        _outgoing_rx: UnboundedReceiver<Envelope>,
        _connection_activity_tx: Sender<()>,
        cx: &mut AsyncApp,
    ) -> Task<Result<i32>> {
        // This would be implemented by moving logic from SshRemoteConnection::start_proxy
        // For now, return a placeholder
        cx.background_spawn(async move {
            Ok(0)
        })
    }

    fn upload_directory(&self, _src_path: PathBuf, _dest_path: PathBuf, cx: &App) -> Task<Result<()>> {
        // TODO: This would need to be properly implemented by moving logic from SshRemoteConnection
        // For now, return a placeholder
        cx.background_spawn(async move {
            Err(anyhow::anyhow!("Upload directory not yet implemented for transport adapter"))
        })
    }

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

    fn connection_args(&self) -> Vec<String> {
        // TODO: Access ssh_args through the socket when properly exposed
        vec![]
    }

    fn connection_config(&self) -> TransportConfig {
        TransportConfig::Ssh(self.connection_options.clone())
    }
}

