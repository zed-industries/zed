use crate::p3::bindings::sockets::types::{ErrorCode, Host};
use crate::p3::sockets::{SocketError, WasiSockets};
use crate::sockets::{SocketAddrCheck, SocketAddrUse, WasiSocketsCtxView};
use core::net::SocketAddr;
use wasmtime::component::Accessor;

mod tcp;
mod udp;

impl Host for WasiSocketsCtxView<'_> {
    fn convert_error_code(&mut self, error: SocketError) -> anyhow::Result<ErrorCode> {
        error.downcast()
    }
}

fn get_socket_addr_check<T>(store: &Accessor<T, WasiSockets>) -> SocketAddrCheck {
    store.with(|mut view| view.get().ctx.socket_addr_check.clone())
}

async fn is_addr_allowed<T>(
    store: &Accessor<T, WasiSockets>,
    addr: SocketAddr,
    reason: SocketAddrUse,
) -> bool {
    get_socket_addr_check(store)(addr, reason).await
}
