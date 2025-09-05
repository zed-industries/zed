pub mod json_log;
pub mod protocol;
pub mod proxy;
pub mod remote_client;
mod transport;

pub use remote_client::{
    ConnectionIdentifier, ConnectionState, RemoteClient, RemoteClientDelegate, RemoteClientEvent,
    RemoteConnectionOptions, RemotePlatform,
};
pub use transport::iroh::{
    IrohConnectionOptions, IrohZedRemote, MAX_MESSAGE_SIZE, Message, ZED_ALPN, ZedIrohTicket,
};
pub use transport::ssh::{SshConnectionOptions, SshPortForwardOption};
pub use transport::wsl::WslConnectionOptions;
