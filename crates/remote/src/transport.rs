use anyhow::Result;
use async_trait::async_trait;
use futures::channel::{
    mpsc::{Sender, UnboundedReceiver, UnboundedSender},
    oneshot,
};
use gpui::{App, AsyncApp, SemanticVersion, Task};
use release_channel::ReleaseChannel;
use rpc::proto::Envelope;
use std::path::PathBuf;
use std::sync::Arc;

use crate::ssh_session::{SshConnectionOptions, SshPlatform};

/// Configuration for different transport types
#[derive(Debug, Clone)]
pub enum TransportConfig {
    Ssh(SshConnectionOptions),
    // Future: Mosh(MoshConnectionOptions),
    // Future: Docker(DockerConnectionOptions),
    // Future: Kubernetes(K8sConnectionOptions),
}

/// Platform information for remote systems
#[derive(Copy, Clone, Debug)]
pub struct RemotePlatform {
    pub os: &'static str,
    pub arch: &'static str,
}

impl From<SshPlatform> for RemotePlatform {
    fn from(ssh_platform: SshPlatform) -> Self {
        Self {
            os: ssh_platform.os,
            arch: ssh_platform.arch,
        }
    }
}

/// Delegate trait for handling transport events
#[async_trait(?Send)]
pub trait TransportDelegate: Send + Sync {
    fn ask_password(&self, prompt: String, tx: oneshot::Sender<String>, cx: &mut AsyncApp);
    fn get_download_params(
        &self,
        platform: RemotePlatform,
        release_channel: ReleaseChannel,
        version: Option<SemanticVersion>,
        cx: &mut AsyncApp,
    ) -> Task<Result<Option<(String, String)>>>;
    fn download_server_binary_locally(
        &self,
        platform: RemotePlatform,
        release_channel: ReleaseChannel,
        version: Option<SemanticVersion>,
        cx: &mut AsyncApp,
    ) -> Task<Result<PathBuf>>;
    fn set_status(&self, status: Option<&str>, cx: &mut AsyncApp);
}

/// Main transport trait for establishing connections
#[async_trait(?Send)]
pub trait Transport: Send + Sync {
    /// Establish connection and return a transport connection
    async fn connect(
        &self,
        config: &TransportConfig,
        delegate: Arc<dyn TransportDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Box<dyn TransportConnection>>;

    /// Get human-readable name for UI
    fn name(&self) -> &'static str;

    /// Check if this transport can handle the given config
    fn supports_config(&self, config: &TransportConfig) -> bool;
}

/// Connection trait for active transport connections
#[async_trait(?Send)]
pub trait TransportConnection: Send + Sync {
    /// Start the proxy process that bridges RPC messages
    fn start_proxy(
        &self,
        unique_identifier: String,
        reconnect: bool,
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        cx: &mut AsyncApp,
    ) -> Task<Result<i32>>;

    /// Upload files/directories to remote
    fn upload_directory(&self, src: PathBuf, dest: PathBuf, cx: &App) -> Task<Result<()>>;

    /// Terminate connection
    async fn kill(&self) -> Result<()>;

    /// Check if connection is terminated
    fn has_been_killed(&self) -> bool;

    /// Get connection-specific arguments (for backwards compat)
    fn connection_args(&self) -> Vec<String>;

    /// Get original connection config
    fn connection_config(&self) -> TransportConfig;
}
