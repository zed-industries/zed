//! Defines structures and functions for Jupyter kernel connection information.
//!
//! This module provides types and utilities for working with Jupyter kernel
//! connection information, including the `ConnectionInfo` struct and related
//! functionality for formatting URLs and serializing/deserializing connection data.
//!
//! The main struct, `ConnectionInfo`, encapsulates all necessary information for
//! establishing a connection with a Jupyter kernel, including IP address, ports,
//! transport protocol, and authentication details.
//! Defines structures and functions for Jupyter kernel connection information.
//!
//! This module provides types and utilities for working with Jupyter kernel
//! connection information, including the `ConnectionInfo` struct and related
//! functionality for formatting URLs and serializing/deserializing connection data.
//!
//! The main struct, `ConnectionInfo`, encapsulates all necessary information for
//! establishing a connection with a Jupyter kernel, including IP address, ports,
//! transport protocol, and authentication details.
//!
//! # Examples
//!
//! ```rust
//! use jupyter_protocol::connection_info::{ConnectionInfo, Transport};
//!
//! let info = ConnectionInfo {
//!     ip: "127.0.0.1".to_string(),
//!     transport: Transport::TCP,
//!     shell_port: 6767,
//!     iopub_port: 6768,
//!     stdin_port: 6769,
//!     control_port: 6770,
//!     hb_port: 6771,
//!     key: "secret_key".to_string(),
//!     signature_scheme: "hmac-sha256".to_string(),
//!     kernel_name: Some("python3".to_string()),
//! };
//!
//! assert_eq!(info.shell_url(), "tcp://127.0.0.1:6767");
//! ```
use serde::{Deserialize, Serialize};

/// Represents the transport protocol used for Jupyter kernel communication.
///
/// This enum is used to specify whether the kernel should use IPC (Inter-Process Communication)
/// or TCP (Transmission Control Protocol) for its network communications.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Transport {
    IPC,
    TCP,
}

impl std::fmt::Display for Transport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transport::IPC => write!(f, "ipc"),
            Transport::TCP => write!(f, "tcp"),
        }
    }
}

/// Represents the runtime connection information for a Jupyter kernel.
///
/// This struct contains all the necessary information for a Jupyter client
/// to connect to a kernel, including ports, transport protocol, and authentication details.
///
/// # Fields
///
/// * `ip` - The IP address of the kernel.
/// * `transport` - The transport protocol (TCP or IPC).
/// * `shell_port` - The port number for the shell channel.
/// * `iopub_port` - The port number for the IOPub channel.
/// * `stdin_port` - The port number for the stdin channel.
/// * `control_port` - The port number for the control channel.
/// * `hb_port` - The port number for the heartbeat channel.
/// * `key` - The authentication key.
/// * `signature_scheme` - The signature scheme used for message authentication.
/// * `kernel_name` - An optional name for the kernel.
///
/// # Example
///
/// ```
/// use jupyter_protocol::connection_info::{ConnectionInfo, Transport};
///
/// let info = ConnectionInfo {
///     ip: "127.0.0.1".to_string(),
///     transport: Transport::TCP,
///     shell_port: 6767,
///     iopub_port: 6768,
///     stdin_port: 6790,
///     control_port: 6791,
///     hb_port: 6792,
///     key: "secret_key".to_string(),
///     signature_scheme: "hmac-sha256".to_string(),
///     kernel_name: Some("python3".to_string()),
/// };
///
/// assert_eq!(info.shell_url(), "tcp://127.0.0.1:6767");
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConnectionInfo {
    pub ip: String,
    pub transport: Transport,
    pub shell_port: u16,
    pub iopub_port: u16,
    pub stdin_port: u16,
    pub control_port: u16,
    pub hb_port: u16,
    pub key: String,
    pub signature_scheme: String,
    // Ignore if not present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_name: Option<String>,
}

/// Constructs a URL string from the given transport, IP address, and port.
///
/// This is a helper function used internally to create formatted URL strings
/// for various Jupyter communication channels.
///
/// # Arguments
///
/// * `transport` - The transport protocol (`Transport::TCP` or `Transport::IPC`).
/// * `ip` - The IP address as a string.
/// * `port` - The port number.
///
/// # Returns
///
/// A `String` containing the formatted URL.
fn form_url(transport: &Transport, ip: &str, port: u16) -> String {
    format!("{}://{}:{}", transport, ip, port)
}

/// Provides methods to generate formatted URLs for various Jupyter communication channels.
impl ConnectionInfo {
    /// Formats the URL for the IOPub channel.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted URL for the IOPub channel.
    /// format the iopub url for a ZeroMQ connection
    pub fn iopub_url(&self) -> String {
        form_url(&self.transport, &self.ip, self.iopub_port)
    }

    /// format the shell url for a ZeroMQ connection
    /// Formats the URL for the shell channel.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted URL for the shell channel.
    pub fn shell_url(&self) -> String {
        form_url(&self.transport, &self.ip, self.shell_port)
    }

    /// format the stdin url for a ZeroMQ connection
    /// Formats the URL for the stdin channel.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted URL for the stdin channel.
    pub fn stdin_url(&self) -> String {
        form_url(&self.transport, &self.ip, self.stdin_port)
    }

    /// format the control url for a ZeroMQ connection
    /// Formats the URL for the control channel.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted URL for the control channel.
    pub fn control_url(&self) -> String {
        form_url(&self.transport, &self.ip, self.control_port)
    }

    /// format the heartbeat url for a ZeroMQ connection
    /// Formats the URL for the heartbeat channel.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted URL for the heartbeat channel.
    pub fn hb_url(&self) -> String {
        form_url(&self.transport, &self.ip, self.hb_port)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_connection_info_urls() {
        let connection_info = ConnectionInfo {
            ip: "127.0.0.1".to_string(),
            transport: Transport::TCP,
            shell_port: 6767,
            iopub_port: 6768,
            stdin_port: 6769,
            control_port: 6770,
            hb_port: 6771,
            key: "test_key".to_string(),
            signature_scheme: "hmac-sha256".to_string(),
            kernel_name: Some("test_kernel".to_string()),
        };

        assert_eq!(connection_info.shell_url(), "tcp://127.0.0.1:6767");
        assert_eq!(connection_info.iopub_url(), "tcp://127.0.0.1:6768");
        assert_eq!(connection_info.stdin_url(), "tcp://127.0.0.1:6769");
        assert_eq!(connection_info.control_url(), "tcp://127.0.0.1:6770");
        assert_eq!(connection_info.hb_url(), "tcp://127.0.0.1:6771");

        let ipc_connection_info = ConnectionInfo {
            transport: Transport::IPC,
            ..connection_info
        };

        assert_eq!(ipc_connection_info.shell_url(), "ipc://127.0.0.1:6767");
        assert_eq!(ipc_connection_info.iopub_url(), "ipc://127.0.0.1:6768");
        assert_eq!(ipc_connection_info.stdin_url(), "ipc://127.0.0.1:6769");
        assert_eq!(ipc_connection_info.control_url(), "ipc://127.0.0.1:6770");
        assert_eq!(ipc_connection_info.hb_url(), "ipc://127.0.0.1:6771");
    }

    #[test]
    fn test_parse_connection_info() {
        let json_str = r#"
        {
            "shell_port": 53380,
            "iopub_port": 53381,
            "stdin_port": 53382,
            "control_port": 53383,
            "hb_port": 53384,
            "ip": "127.0.0.1",
            "key": "e733b584-1d43845bc7d8d11a60df6363",
            "transport": "tcp",
            "signature_scheme": "hmac-sha256",
            "kernel_name": "anaconda",
            "jupyter_session": "/Users/kylekelley/Untitled3.ipynb"
        }"#;

        let connection_info: ConnectionInfo = serde_json::from_str(json_str).unwrap();

        assert_eq!(connection_info.shell_port, 53380);
        assert_eq!(connection_info.iopub_port, 53381);
        assert_eq!(connection_info.stdin_port, 53382);
        assert_eq!(connection_info.control_port, 53383);
        assert_eq!(connection_info.hb_port, 53384);
        assert_eq!(connection_info.ip, "127.0.0.1");
        assert_eq!(connection_info.key, "e733b584-1d43845bc7d8d11a60df6363");
        assert_eq!(connection_info.transport, Transport::TCP);
        assert_eq!(connection_info.signature_scheme, "hmac-sha256");
        assert_eq!(connection_info.kernel_name, Some("anaconda".to_string()));
    }
}
