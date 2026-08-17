use crate::p2::bindings::sockets::network::ErrorCode;
use crate::sockets::SocketAddrCheck;
use crate::{SocketAddrUse, TrappableError};
use core::net::SocketAddr;

pub type SocketResult<T> = Result<T, SocketError>;

pub type SocketError = TrappableError<ErrorCode>;

impl From<wasmtime::component::ResourceTableError> for SocketError {
    fn from(error: wasmtime::component::ResourceTableError) -> Self {
        Self::trap(error)
    }
}

impl From<std::io::Error> for SocketError {
    fn from(error: std::io::Error) -> Self {
        ErrorCode::from(error).into()
    }
}

impl From<rustix::io::Errno> for SocketError {
    fn from(error: rustix::io::Errno) -> Self {
        ErrorCode::from(error).into()
    }
}

impl From<crate::sockets::util::ErrorCode> for SocketError {
    fn from(error: crate::sockets::util::ErrorCode) -> Self {
        ErrorCode::from(error).into()
    }
}

impl From<crate::sockets::util::ErrorCode> for ErrorCode {
    fn from(error: crate::sockets::util::ErrorCode) -> Self {
        match error {
            crate::sockets::util::ErrorCode::Unknown => Self::Unknown,
            crate::sockets::util::ErrorCode::AccessDenied => Self::AccessDenied,
            crate::sockets::util::ErrorCode::NotSupported => Self::NotSupported,
            crate::sockets::util::ErrorCode::InvalidArgument => Self::InvalidArgument,
            crate::sockets::util::ErrorCode::OutOfMemory => Self::OutOfMemory,
            crate::sockets::util::ErrorCode::Timeout => Self::Timeout,
            crate::sockets::util::ErrorCode::InvalidState => Self::InvalidState,
            crate::sockets::util::ErrorCode::AddressNotBindable => Self::AddressNotBindable,
            crate::sockets::util::ErrorCode::AddressInUse => Self::AddressInUse,
            crate::sockets::util::ErrorCode::RemoteUnreachable => Self::RemoteUnreachable,
            crate::sockets::util::ErrorCode::ConnectionRefused => Self::ConnectionRefused,
            crate::sockets::util::ErrorCode::ConnectionReset => Self::ConnectionReset,
            crate::sockets::util::ErrorCode::ConnectionAborted => Self::ConnectionAborted,
            crate::sockets::util::ErrorCode::DatagramTooLarge => Self::DatagramTooLarge,
        }
    }
}

pub struct Network {
    pub socket_addr_check: SocketAddrCheck,
    pub allow_ip_name_lookup: bool,
}

impl Network {
    pub async fn check_socket_addr(
        &self,
        addr: SocketAddr,
        reason: SocketAddrUse,
    ) -> std::io::Result<()> {
        self.socket_addr_check.check(addr, reason).await
    }
}
