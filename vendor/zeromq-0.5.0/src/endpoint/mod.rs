mod error;
mod host;
mod transport;

use once_cell::sync::Lazy;
use regex::Regex;

use std::fmt;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

pub use error::EndpointError;
pub use host::Host;
pub use transport::Transport;

pub type Port = u16;

static TRANSPORT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([[:lower:]]+)://(.+)$").unwrap());
static HOST_PORT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.+):(\d+)$").unwrap());

/// Represents a ZMQ Endpoint.
///
/// # Examples
/// ```
/// # use zeromq::{Endpoint, Host};
/// # fn main() -> Result<(), zeromq::ZmqError> {
/// assert_eq!(
///     "tcp://example.com:4567".parse::<Endpoint>()?,
///     Endpoint::Tcp(Host::Domain("example.com".to_string()), 4567)
/// );
/// # Ok(()) }
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Endpoint {
    // TODO: Add endpoints for the other transport variants
    Tcp(Host, Port),
    Ipc(Option<PathBuf>),
}

impl Endpoint {
    pub fn transport(&self) -> Transport {
        match self {
            Self::Tcp(_, _) => Transport::Tcp,
            Self::Ipc(_) => Transport::Ipc,
        }
    }

    /// Creates an `Endpoint::Tcp` from a [`SocketAddr`]
    pub fn from_tcp_addr(addr: SocketAddr) -> Self {
        Endpoint::Tcp(addr.ip().into(), addr.port())
    }

    pub fn from_tcp_domain(addr: String, port: u16) -> Self {
        Endpoint::Tcp(Host::Domain(addr), port)
    }
}

impl FromStr for Endpoint {
    type Err = EndpointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = TRANSPORT_REGEX
            .captures(s)
            .ok_or(EndpointError::Syntax("Could not parse transport"))?;
        let transport: &str = caps.get(1).unwrap().into();
        let transport: Transport = transport.parse()?;
        let address = caps.get(2).unwrap().as_str();

        fn extract_host_port(address: &str) -> Result<(Host, Port), EndpointError> {
            let caps = HOST_PORT_REGEX
                .captures(address)
                .ok_or(EndpointError::Syntax("Could not parse host and port"))?;
            let host = caps.get(1).unwrap().as_str();
            let port = caps.get(2).unwrap().as_str();
            let port: Port = port
                .parse()
                .map_err(|_e| EndpointError::Syntax("Port must be a u16 but was out of range"))?;

            let host: Host = host.parse()?;
            Ok((host, port))
        }

        let endpoint = match transport {
            Transport::Tcp => {
                let (host, port) = extract_host_port(address)?;
                Endpoint::Tcp(host, port)
            }
            Transport::Ipc => {
                let path: PathBuf = address.to_string().into();
                Endpoint::Ipc(Some(path))
            }
        };

        Ok(endpoint)
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Endpoint::Tcp(host, port) => {
                if let Host::Ipv6(_) = host {
                    write!(f, "tcp://[{}]:{}", host, port)
                } else {
                    write!(f, "tcp://{}:{}", host, port)
                }
            }
            Endpoint::Ipc(Some(path)) => write!(f, "ipc://{}", path.display()),
            Endpoint::Ipc(None) => write!(f, "ipc://????"),
        }
    }
}

/// Represents a type that can be converted into an [`Endpoint`].
///
/// This trait is intentionally sealed to prevent implementation on third-party
/// types.
// TODO: Is sealing this trait actually necessary?
pub trait TryIntoEndpoint: Send + private::Sealed {
    /// Convert into an `Endpoint` via an owned `Self`.
    ///
    /// Enables efficient `Endpoint` -> `Endpoint` conversion, while permitting
    /// the creation of a new `Endpoint` when given types like `&str`.
    fn try_into(self) -> Result<Endpoint, EndpointError>;
}
impl TryIntoEndpoint for &str {
    fn try_into(self) -> Result<Endpoint, EndpointError> {
        self.parse()
    }
}
impl TryIntoEndpoint for Endpoint {
    fn try_into(self) -> Result<Endpoint, EndpointError> {
        Ok(self)
    }
}

impl private::Sealed for str {}
impl private::Sealed for &str {}
impl private::Sealed for Endpoint {}

mod private {
    pub trait Sealed {}
}

#[cfg(test)]
mod tests {
    use super::*;

    static PAIRS: Lazy<Vec<(Endpoint, &'static str)>> = Lazy::new(|| {
        vec![
            (
                Endpoint::Ipc(Some(PathBuf::from("/tmp/asdf"))),
                "ipc:///tmp/asdf",
            ),
            (
                Endpoint::Ipc(Some(PathBuf::from("my/dir_1/dir-2"))),
                "ipc://my/dir_1/dir-2",
            ),
            (
                Endpoint::Ipc(Some(PathBuf::from("@abstract/namespace"))),
                "ipc://@abstract/namespace",
            ),
            (
                Endpoint::Tcp(Host::Domain("www.example.com".to_string()), 1234),
                "tcp://www.example.com:1234",
            ),
            (
                Endpoint::Tcp(Host::Domain("*".to_string()), 2345),
                "tcp://*:2345",
            ),
            (
                Endpoint::Tcp(Host::Ipv4("127.0.0.1".parse().unwrap()), 8080),
                "tcp://127.0.0.1:8080",
            ),
            (
                Endpoint::Tcp(Host::Ipv6("::1".parse().unwrap()), 34567),
                "tcp://[::1]:34567",
            ),
            (
                Endpoint::Tcp(Host::Domain("i❤.ws".to_string()), 80),
                "tcp://i❤.ws:80",
            ),
            (
                Endpoint::Tcp(Host::Domain("xn--i-7iq.ws".to_string()), 80),
                "tcp://xn--i-7iq.ws:80",
            ),
            (
                Endpoint::Tcp(Host::Ipv4("127.0.0.1".parse().unwrap()), 65535),
                "tcp://127.0.0.1:65535",
            ),
            (
                Endpoint::Tcp(Host::Ipv4("127.0.0.1".parse().unwrap()), 0),
                "tcp://127.0.0.1:0",
            ),
        ]
    });

    #[test]
    fn test_endpoint_display() {
        for (e, s) in PAIRS.iter() {
            assert_eq!(&format!("{}", e), s);
        }
        assert_eq!(&format!("{}", Endpoint::Ipc(None)), "ipc://????");
    }

    #[test]
    fn test_endpoint_parse() {
        use std::mem::discriminant as disc;

        // Test example 1:1 pairs
        for (e, s) in PAIRS.iter() {
            assert_eq!(&s.parse::<Endpoint>().unwrap(), e);
        }

        let exact_counter_examples = vec![(
            "abc://127.0.0.1:1234",
            EndpointError::UnknownTransport("abc".to_string()),
        )];

        for (s, target_err) in exact_counter_examples {
            let parse_err = s.parse::<Endpoint>().unwrap_err();
            assert_eq!(parse_err.to_string(), target_err.to_string());
            assert_eq!(disc(&parse_err), disc(&target_err));
        }

        // Examples where we only care about matching variants, rather than contents of
        // those variants
        let inexact_counter_examples = vec![
            ("://127.0.0.1:1234", EndpointError::Syntax("")),
            ("tcp://127.0.0.1:", EndpointError::Syntax("")),
            ("tcp://:1234", EndpointError::Syntax("")),
            ("tcp://127.0.0.1", EndpointError::Syntax("")),
            ("tcp://127.0.0.1:65536", EndpointError::Syntax("")),
            ("TCP://127.0.0.1:1234", EndpointError::Syntax("")),
        ];

        for (s, target_variant) in inexact_counter_examples {
            let parse_err = s.parse::<Endpoint>().unwrap_err();
            assert_eq!(disc(&parse_err), disc(&target_variant));
        }
    }
}
