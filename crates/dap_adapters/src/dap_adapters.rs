mod gdb;
mod go;
mod javascript;
mod lldb;
mod php;
mod python;

use std::{net::Ipv4Addr, sync::Arc};

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use dap::{
    DapRegistry,
    adapters::{
        self, AdapterVersion, DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName,
        GithubRepo,
    },
};
use gdb::GdbDebugAdapter;
use go::GoDebugAdapter;
use javascript::JsDebugAdapter;
use lldb::LldbDebugAdapter;
use php::PhpDebugAdapter;
use python::PythonDebugAdapter;
use serde_json::{Value, json};
use task::{DebugAdapterConfig, TCPHost};

pub fn init(registry: Arc<DapRegistry>) {
    registry.add_adapter(Arc::from(PythonDebugAdapter));
    registry.add_adapter(Arc::from(PhpDebugAdapter));
    registry.add_adapter(Arc::from(JsDebugAdapter::default()));
    registry.add_adapter(Arc::from(LldbDebugAdapter));
    registry.add_adapter(Arc::from(GoDebugAdapter));
    registry.add_adapter(Arc::from(GdbDebugAdapter));
}

pub(crate) async fn configure_tcp_connection(
    tcp_connection: TCPHost,
) -> Result<(Ipv4Addr, u16, Option<u64>)> {
    let host = tcp_connection.host();
    let timeout = tcp_connection.timeout;

    let port = if let Some(port) = tcp_connection.port {
        port
    } else {
        dap::transport::TcpTransport::port(&tcp_connection).await?
    };

    Ok((host, port, timeout))
}
