use serde::{Deserialize, Serialize};

/// Permission to open a TCP connection to an explicitly named local endpoint.
///
/// This deliberately accepts only loopback endpoint names. Resolving the name and
/// checking the resolved address is the responsibility of the host that opens the
/// socket; this type prevents extensions from declaring remote endpoints.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TcpConnectCapability {
    /// A loopback hostname or address: `localhost`, `127.0.0.1`, or `::1`.
    pub host: String,
    /// An optional exact TCP port. When omitted, any port on the named loopback
    /// host is allowed. This supports services such as nREPL that choose a port
    /// dynamically and publish it in a file.
    pub port: Option<u16>,
}

impl TcpConnectCapability {
    pub fn allows(&self, desired_host: &str, desired_port: u16) -> bool {
        self.port.is_none_or(|port| port == desired_port)
            && Self::is_loopback_host(&self.host)
            && Self::is_loopback_host(desired_host)
            && self.host.eq_ignore_ascii_case(desired_host)
    }

    pub fn is_loopback_host(host: &str) -> bool {
        matches!(
            host.to_ascii_lowercase().as_str(),
            "localhost" | "127.0.0.1" | "::1"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_allows_the_declared_loopback_endpoint() {
        let capability = TcpConnectCapability {
            host: "localhost".to_string(),
            port: Some(7888),
        };

        assert!(capability.allows("localhost", 7888));
        assert!(!capability.allows("localhost", 7889));
        assert!(!capability.allows("127.0.0.1", 7888));
        assert!(!capability.allows("example.com", 7888));
    }

    #[test]
    fn a_loopback_host_may_allow_dynamic_ports() {
        let capability = TcpConnectCapability {
            host: "localhost".to_string(),
            port: None,
        };

        assert!(capability.allows("localhost", 7888));
        assert!(capability.allows("localhost", 0));
    }

    #[test]
    fn recognizes_only_supported_loopback_names() {
        assert!(TcpConnectCapability::is_loopback_host("localhost"));
        assert!(TcpConnectCapability::is_loopback_host("127.0.0.1"));
        assert!(TcpConnectCapability::is_loopback_host("::1"));
        assert!(!TcpConnectCapability::is_loopback_host("0.0.0.0"));
        assert!(!TcpConnectCapability::is_loopback_host("example.com"));
    }
}
