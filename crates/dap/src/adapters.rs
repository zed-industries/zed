use crate::client::TransportParams;
use ::fs::Fs;
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::AsyncReadExt;
use gpui::AsyncAppContext;
use http_client::HttpClient;
use node_runtime::NodeRuntime;
use serde_json::Value;
use smol::{
    self,
    io::BufReader,
    net::{TcpListener, TcpStream},
    process,
};
use std::{
    collections::HashMap,
    ffi::OsString,
    fmt::Debug,
    net::{Ipv4Addr, SocketAddrV4},
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::Duration,
};

use task::TCPHost;

/// Get an open port to use with the tcp client when not supplied by debug config
async fn get_port(host: Ipv4Addr) -> Option<u16> {
    Some(
        TcpListener::bind(SocketAddrV4::new(host, 0))
            .await
            .ok()?
            .local_addr()
            .ok()?
            .port(),
    )
}

pub trait DapDelegate {
    fn http_client(&self) -> Option<Arc<dyn HttpClient>>;
    fn node_runtime(&self) -> Option<NodeRuntime>;
    fn fs(&self) -> Arc<dyn Fs>;
}

/// TCP clients don't have an error communication stream with an adapter
/// # Parameters
/// - `host`: The ip/port that that the client will connect too
/// - `adapter_binary`: The debug adapter binary to start
/// - `cx`: The context that the new client belongs too
pub async fn create_tcp_client(
    host: TCPHost,
    adapter_binary: DebugAdapterBinary,
    cx: &mut AsyncAppContext,
) -> Result<TransportParams> {
    let host_address = host.host.unwrap_or_else(|| Ipv4Addr::new(127, 0, 0, 1));

    let mut port = host.port;
    if port.is_none() {
        port = get_port(host_address).await;
    }
    let mut command = if let Some(start_command) = &adapter_binary.start_command {
        let mut command = process::Command::new(start_command);
        command.arg(adapter_binary.path);
        command
    } else {
        process::Command::new(adapter_binary.path)
    };

    command
        .args(adapter_binary.arguments)
        .envs(adapter_binary.env.clone().unwrap_or_default())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true);

    let process = command
        .spawn()
        .with_context(|| "failed to start debug adapter.")?;

    if let Some(delay) = host.delay {
        // some debug adapters need some time to start the TCP server
        // so we have to wait few milliseconds before we can connect to it
        cx.background_executor()
            .timer(Duration::from_millis(delay))
            .await;
    }

    let address = SocketAddrV4::new(
        host_address,
        port.ok_or(anyhow!("Port is required to connect to TCP server"))?,
    );

    let (rx, tx) = TcpStream::connect(address).await?.split();
    log::info!("Debug adapter has connected to tcp server");

    Ok(TransportParams::new(
        Box::new(BufReader::new(rx)),
        Box::new(tx),
        None,
        Some(process),
    ))
}

/// Creates a debug client that connects to an adapter through std input/output
///
/// # Parameters
/// - `adapter_binary`: The debug adapter binary to start
pub fn create_stdio_client(adapter_binary: DebugAdapterBinary) -> Result<TransportParams> {
    let mut command = if let Some(start_command) = &adapter_binary.start_command {
        let mut command = process::Command::new(start_command);
        command.arg(adapter_binary.path);
        command
    } else {
        let command = process::Command::new(adapter_binary.path);
        command
    };

    command
        .args(adapter_binary.arguments)
        .envs(adapter_binary.env.clone().unwrap_or_default())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);

    let mut process = command
        .spawn()
        .with_context(|| "failed to spawn command.")?;

    let stdin = process
        .stdin
        .take()
        .ok_or_else(|| anyhow!("Failed to open stdin"))?;
    let stdout = process
        .stdout
        .take()
        .ok_or_else(|| anyhow!("Failed to open stdout"))?;
    let stderr = process
        .stderr
        .take()
        .ok_or_else(|| anyhow!("Failed to open stderr"))?;

    log::info!("Debug adapter has connected to stdio adapter");

    Ok(TransportParams::new(
        Box::new(BufReader::new(stdout)),
        Box::new(stdin),
        Some(Box::new(BufReader::new(stderr))),
        Some(process),
    ))
}

pub struct DebugAdapterName(pub Arc<str>);

impl AsRef<Path> for DebugAdapterName {
    fn as_ref(&self) -> &Path {
        Path::new(&*self.0)
    }
}

impl std::fmt::Display for DebugAdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone)]
pub struct DebugAdapterBinary {
    pub start_command: Option<String>,
    pub path: PathBuf,
    pub arguments: Vec<OsString>,
    pub env: Option<HashMap<String, String>>,
}

#[async_trait(?Send)]
pub trait DebugAdapter: Debug + Send + Sync + 'static {
    fn id(&self) -> String {
        "".to_string()
    }

    fn name(&self) -> DebugAdapterName;

    async fn connect(
        &self,
        adapter_binary: DebugAdapterBinary,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<TransportParams>;

    async fn install_or_fetch_binary(
        &self,
        delegate: Box<dyn DapDelegate>,
    ) -> Result<DebugAdapterBinary>;

    fn request_args(&self) -> Value;
}
