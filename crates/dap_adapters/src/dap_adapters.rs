mod codelldb;
mod gdb;
mod go;
mod javascript;
mod php;
mod python;

use std::{net::Ipv4Addr, sync::Arc};

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use codelldb::CodeLldbDebugAdapter;
use dap::{
    DapRegistry, DebugRequestType,
    adapters::{
        self, AdapterVersion, DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName,
        GithubRepo,
    },
};
use gdb::GdbDebugAdapter;
use go::GoDebugAdapter;
use javascript::JsDebugAdapter;
use php::PhpDebugAdapter;
use python::PythonDebugAdapter;
use serde_json::{Value, json};
use task::TCPHost;

pub fn init(registry: Arc<DapRegistry>) {
    registry.add_adapter(Arc::from(CodeLldbDebugAdapter::default()));
    registry.add_adapter(Arc::from(PythonDebugAdapter));
    registry.add_adapter(Arc::from(PhpDebugAdapter));
    registry.add_adapter(Arc::from(JsDebugAdapter));
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

trait ToDap {
    fn to_dap(&self) -> dap::StartDebuggingRequestArgumentsRequest;
}

impl ToDap for DebugRequestType {
    fn to_dap(&self) -> dap::StartDebuggingRequestArgumentsRequest {
        match self {
            Self::Launch(_) => dap::StartDebuggingRequestArgumentsRequest::Launch,
            Self::Attach(_) => dap::StartDebuggingRequestArgumentsRequest::Attach,
        }
    }
}
