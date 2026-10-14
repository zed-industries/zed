pub mod command;
pub mod json_log;
pub mod protocol;
pub mod proxy;
pub mod remote_client;
pub mod remote_identity;
mod transport;

#[cfg(target_os = "windows")]
pub use remote_client::OpenWslPath;
pub use remote_client::{
    CommandTemplate, ConnectionIdentifier, ConnectionState, Interactive, RemoteArch, RemoteClient,
    RemoteClientDelegate, RemoteClientEvent, RemoteConnection, RemoteConnectionOptions, RemoteOs,
    RemotePlatform, active_connections_matching, active_ssh_connections, connect,
    has_active_connection,
};
pub use remote_identity::{
    RemoteConnectionIdentity, remote_connection_identity, same_remote_connection_identity,
};
pub use transport::docker::DockerConnectionOptions;
pub use transport::ssh::{
    SshConnectionOptions, SshPortForwardOption, format_host_port, parse_host_port,
    port_forward_spec,
};
pub use transport::wsl::WslConnectionOptions;
#[cfg(target_os = "windows")]
pub use transport::wsl::wsl_path_to_windows_path;

#[cfg(any(test, feature = "test-support"))]
pub use transport::mock::{
    MOCK_STDIO_FRAME_HEX, MockConnection, MockConnectionOptions, MockConnectionRegistry,
    MockDelegate,
};
