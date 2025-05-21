pub mod adapters;
pub mod client;
pub mod debugger_settings;
pub mod inline_value;
pub mod proto_conversions;
mod registry;
pub mod transport;

use std::net::Ipv4Addr;

pub use dap_types::*;
pub use registry::{DapLocator, DapRegistry};
pub use task::DebugRequest;

pub type ScopeId = u64;
pub type VariableReference = u64;
pub type StackFrameId = u64;

#[cfg(any(test, feature = "test-support"))]
pub use adapters::FakeAdapter;
use task::TcpArgumentsTemplate;

pub async fn configure_tcp_connection(
    tcp_connection: TcpArgumentsTemplate,
) -> anyhow::Result<(Ipv4Addr, u16, Option<u64>)> {
    let host = tcp_connection.host();
    let timeout = tcp_connection.timeout;

    let port = if let Some(port) = tcp_connection.port {
        port
    } else {
        transport::TcpTransport::port(&tcp_connection).await?
    };

    Ok((host, port, timeout))
}
