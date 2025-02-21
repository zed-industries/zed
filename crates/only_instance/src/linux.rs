use release_channel::RELEASE_CHANNEL_NAME;

use std::io::Result;
use std::os::unix::net::UnixDatagram;
use std::path::Path;

pub fn ensure_only_instance() -> Result<UnixDatagram> {
    let sock_path = paths::support_dir().join(format!("zed-{}.sock", *RELEASE_CHANNEL_NAME));
    // remove the socket if the process listening on it has died
    if let Err(e) = UnixDatagram::unbound()?.connect(&sock_path) {
        if e.kind() == std::io::ErrorKind::ConnectionRefused {
            std::fs::remove_file(&sock_path)?;
        }
    }
    UnixDatagram::bind(&sock_path)
}

pub fn other_instance_running() -> Result<UnixDatagram> {
    let sock_path = paths::support_dir().join(format!("zed-{}.sock", *RELEASE_CHANNEL_NAME));
    let sock = UnixDatagram::unbound()?;
    sock.connect(&sock_path).and(Ok(sock))
}
