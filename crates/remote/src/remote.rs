pub mod json_log;
pub mod protocol;
pub mod proxy;
pub mod remote_client;
mod transport;

#[cfg(target_os = "windows")]
pub use remote_client::OpenWslPath;
pub use remote_client::{
    ConnectionIdentifier, ConnectionState, RemoteArch, RemoteClient, RemoteClientDelegate,
    RemoteClientEvent, RemoteConnection, RemoteConnectionOptions, RemoteOs, RemotePlatform,
    connect,
};
pub use transport::docker::DockerConnectionOptions;
pub use transport::ssh::{SshConnectionOptions, SshPortForwardOption};
pub use transport::wsl::WslConnectionOptions;
