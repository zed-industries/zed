use crate::client::TransportParams;
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::AsyncReadExt;
use gpui::AsyncAppContext;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use smol::{
    self,
    io::BufReader,
    net::{TcpListener, TcpStream},
    process,
};
use std::{
    fmt::Debug,
    net::{Ipv4Addr, SocketAddrV4},
    path::PathBuf,
    process::Stdio,
    sync::Arc,
    time::Duration,
};
use task::{DebugAdapterConfig, DebugAdapterKind, TCPHost};

pub fn build_adapter(adapter_config: &DebugAdapterConfig) -> Result<Box<dyn DebugAdapter>> {
    match adapter_config.kind {
        DebugAdapterKind::Custom => Err(anyhow!("Custom is not implemented")),
        DebugAdapterKind::Python => Ok(Box::new(PythonDebugAdapter::new(adapter_config))),
        DebugAdapterKind::PHP => Ok(Box::new(PhpDebugAdapter::new(adapter_config))),
        DebugAdapterKind::Lldb => Ok(Box::new(LldbDebugAdapter::new(adapter_config))),
    }
}

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

/// Creates a debug client that connects to an adapter through tcp
///
/// TCP clients don't have an error communication stream with an adapter
///
/// # Parameters
/// - `command`: The command that starts the debugger
/// - `args`: Arguments of the command that starts the debugger
/// - `cwd`: The absolute path of the project that is being debugged
/// - `cx`: The context that the new client belongs too
async fn create_tcp_client(
    host: TCPHost,
    command: &String,
    args: &Vec<String>,
    cx: &mut AsyncAppContext,
) -> Result<TransportParams> {
    let host_address = host.host.unwrap_or_else(|| Ipv4Addr::new(127, 0, 0, 1));

    let mut port = host.port;
    if port.is_none() {
        port = get_port(host_address).await;
    }

    let mut command = process::Command::new(command);
    command
        .args(args)
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
/// - `command`: The command that starts the debugger
/// - `args`: Arguments of the command that starts the debugger
fn create_stdio_client(command: &String, args: &Vec<String>) -> Result<TransportParams> {
    let mut command = process::Command::new(command);
    command
        .args(args)
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

    Ok(TransportParams::new(
        Box::new(BufReader::new(stdout)),
        Box::new(stdin),
        Some(Box::new(BufReader::new(stderr))),
        Some(process),
    ))
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct DebugAdapterName(pub Arc<str>);

pub struct DebugAdapterBinary {
    pub path: PathBuf,
}

#[async_trait(?Send)]
pub trait DebugAdapter: Debug + Send + Sync + 'static {
    fn id(&self) -> String {
        "".to_string()
    }

    fn name(&self) -> DebugAdapterName;

    async fn connect(&self, cx: &mut AsyncAppContext) -> anyhow::Result<TransportParams>;

    fn is_installed(&self) -> Option<DebugAdapterBinary>;

    fn download_adapter(&self) -> anyhow::Result<DebugAdapterBinary>;

    fn request_args(&self) -> Value;
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PythonDebugAdapter {
    program: String,
    adapter_path: Option<String>,
}

impl PythonDebugAdapter {
    const _ADAPTER_NAME: &'static str = "debugpy";

    fn new(adapter_config: &DebugAdapterConfig) -> Self {
        PythonDebugAdapter {
            program: adapter_config.program.clone(),
            adapter_path: adapter_config.adapter_path.clone(),
        }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PythonDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::_ADAPTER_NAME.into())
    }

    async fn connect(&self, _cx: &mut AsyncAppContext) -> Result<TransportParams> {
        let command = "python3".to_string();

        let args = if let Some(path) = self.adapter_path.clone() {
            vec![path]
        } else {
            Vec::new()
        };

        create_stdio_client(&command, &args)
    }

    fn is_installed(&self) -> Option<DebugAdapterBinary> {
        None
    }

    fn download_adapter(&self) -> anyhow::Result<DebugAdapterBinary> {
        Err(anyhow::format_err!("Not implemented"))
    }

    fn request_args(&self) -> Value {
        json!({"program": format!("{}", &self.program)})
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PhpDebugAdapter {
    program: String,
    adapter_path: Option<String>,
}

impl PhpDebugAdapter {
    const _ADAPTER_NAME: &'static str = "vscode-php-debug";

    fn new(adapter_config: &DebugAdapterConfig) -> Self {
        PhpDebugAdapter {
            program: adapter_config.program.clone(),
            adapter_path: adapter_config.adapter_path.clone(),
        }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PhpDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::_ADAPTER_NAME.into())
    }

    async fn connect(&self, cx: &mut AsyncAppContext) -> Result<TransportParams> {
        let command = "bun".to_string();

        let args = if let Some(path) = self.adapter_path.clone() {
            vec![path, "--server=8132".into()]
        } else {
            Vec::new()
        };

        let host = TCPHost {
            port: Some(8132),
            host: None,
            delay: Some(1000),
        };

        create_tcp_client(host, &command, &args, cx).await
    }

    fn is_installed(&self) -> Option<DebugAdapterBinary> {
        None
    }

    fn download_adapter(&self) -> anyhow::Result<DebugAdapterBinary> {
        Err(anyhow::format_err!("Not implemented"))
    }

    fn request_args(&self) -> Value {
        json!({"program": format!("{}", &self.program)})
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct LldbDebugAdapter {
    program: String,
    adapter_path: Option<String>,
}

impl LldbDebugAdapter {
    const _ADAPTER_NAME: &'static str = "lldb";

    fn new(adapter_config: &DebugAdapterConfig) -> Self {
        LldbDebugAdapter {
            program: adapter_config.program.clone(),
            adapter_path: adapter_config.adapter_path.clone(),
        }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for LldbDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::_ADAPTER_NAME.into())
    }

    async fn connect(&self, _: &mut AsyncAppContext) -> Result<TransportParams> {
        let command = "/opt/homebrew/opt/llvm/bin/lldb-dap".to_string();

        create_stdio_client(&command, &vec![])
    }

    fn is_installed(&self) -> Option<DebugAdapterBinary> {
        None
    }

    fn download_adapter(&self) -> anyhow::Result<DebugAdapterBinary> {
        Err(anyhow::format_err!("Not implemented"))
    }

    fn request_args(&self) -> Value {
        json!({"program": format!("{}", &self.program)})
    }
}
