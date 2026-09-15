//! Capability-gated TCP streams for local development tools.

pub use crate::wit::zed::extension::tcp::TcpStream;

/// Opens a TCP stream to a loopback endpoint declared by the extension and
/// granted by the user through `network:tcp-local`.
pub fn connect(host: &str, port: u16) -> Result<TcpStream, String> {
    crate::wit::zed::extension::tcp::connect(host, port)
}
