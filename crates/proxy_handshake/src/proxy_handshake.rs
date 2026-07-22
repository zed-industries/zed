//! Sans-IO client handshakes for HTTP CONNECT and SOCKS proxies.
//!
//! Speaks the client side of the protocols needed to tunnel a TCP connection
//! through a proxy — HTTP CONNECT (RFC 9110 §9.3.6), SOCKS4/4a, and SOCKS5
//! (RFC 1928, with RFC 1929 username/password authentication) — as a pure
//! state machine that never touches a socket. Callers own all I/O: resolving
//! and connecting to the proxy, wrapping the connection in TLS when the proxy
//! requires it, and pumping bytes between the [`Handshake`] and their
//! transport. The `futures-io` and `tokio` features add ready-made drivers
//! for the corresponding I/O traits.
//!
//! ```
//! use proxy_handshake::{Handshake, ProxySpec, Step, Target, Url};
//!
//! let url: Url = "http://localhost:8888".parse().unwrap();
//! let spec = ProxySpec::parse(&url).unwrap();
//! let target = Target::Domain("example.com".into(), 443);
//! let mut handshake = Handshake::new(&spec, &target).unwrap();
//!
//! // The sans-IO loop: write `Send` bytes to the proxy, feed bytes the proxy
//! // sent back through `advance`, and start using the stream for the
//! // tunneled protocol once `Done` is returned.
//! let Ok(Step::Send(connect_request)) = handshake.advance(&[]) else {
//!     unreachable!()
//! };
//! assert!(connect_request.starts_with(b"CONNECT example.com:443 HTTP/1.1\r\n"));
//! ```

mod handshake;

#[cfg(feature = "futures-io")]
pub mod futures_io;
#[cfg(feature = "tokio")]
pub mod tokio;

use std::net::SocketAddr;

pub use handshake::{Handshake, Step};
pub use url::Url;

/// How to reach and authenticate with a proxy: the validated interpretation
/// of a proxy [`Url`].
///
/// A raw [`Url`] can hold any scheme, leaves default ports implicit, and
/// carries credentials percent-encoded. Parsing into a `ProxySpec` settles
/// all of that once — unsupported schemes are rejected, scheme-specific
/// default ports are applied, and credentials are decoded — so the handshake
/// and its callers never re-derive them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxySpec {
    pub scheme: ProxyScheme,
    /// The proxy's host: a domain name or an IP address literal (without
    /// brackets for IPv6).
    pub host: String,
    pub port: u16,
    pub credentials: Option<Credentials>,
}

/// The protocol the proxy speaks, taken from the proxy URL's scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyScheme {
    /// An HTTP CONNECT proxy (`http://` or `https://`). `tls` is whether the
    /// connection *to the proxy itself* is wrapped in TLS (an `https://`
    /// proxy URL); it is independent of any TLS the tunneled protocol layers
    /// inside the tunnel.
    Http { tls: bool },
    /// A SOCKS4 proxy (`socks4://` or `socks4a://`).
    Socks4 { remote_dns: bool },
    /// A SOCKS5 proxy (`socks5://` or `socks5h://`).
    Socks5 { remote_dns: bool },
}

/// Proxy credentials, percent-decoded from the URL's userinfo component.
#[derive(Clone, PartialEq, Eq)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// Redacts both fields so credentials can't leak through debug formatting of
/// any containing type. Implemented manually (rather than leaving `Debug`
/// underived) so a future `#[derive(Debug)]` on this struct is a conscious
/// choice instead of an accident.
impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("username", &"<redacted>")
            .field("password", &"<redacted>")
            .finish()
    }
}

/// The destination to tunnel to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// An unresolved host name and port: DNS resolution is delegated to the
    /// proxy. With SOCKS4 proxies this is encoded using the SOCKS4a
    /// extension, which not every SOCKS4 proxy supports.
    Domain(String, u16),
    /// An already-resolved socket address. Use this when
    /// [`ProxySpec::remote_dns`] is false, resolving the host locally first.
    Address(SocketAddr),
}

/// The size cap on an HTTP CONNECT response head; a proxy that sends more
/// than this before the blank line is misbehaving.
pub const MAX_HTTP_RESPONSE_LENGTH: usize = 8192;

#[derive(Debug, thiserror::Error)]
pub enum ProxyError {
    #[error(
        "unsupported proxy scheme `{0}` (expected http, https, socks4, socks4a, socks5, or socks5h)"
    )]
    UnsupportedScheme(String),
    #[error("proxy URL is missing a host")]
    MissingHost,
    #[error("proxy credentials are not valid UTF-8 after percent-decoding")]
    InvalidCredentials,
    #[error("SOCKS5 usernames and passwords are limited to 255 bytes")]
    CredentialsTooLong,
    #[error("SOCKS target domains are limited to 255 bytes")]
    DomainTooLong,
    #[error("SOCKS4 proxies only support IPv4 targets")]
    Socks4Ipv4Only,
    #[error("proxy sent a malformed HTTP response: {0}")]
    MalformedHttpResponse(String),
    #[error("proxy refused CONNECT with HTTP status {0}")]
    HttpConnectRefused(u16),
    #[error("HTTP response from proxy exceeded {MAX_HTTP_RESPONSE_LENGTH} bytes")]
    HttpResponseTooLarge,
    #[error("proxy replied with SOCKS version {0:#04x}")]
    UnexpectedSocksVersion(u8),
    #[error("SOCKS5 proxy rejected the offered authentication method (selected {selected:#04x})")]
    AuthMethodRejected { selected: u8 },
    #[error("SOCKS5 proxy rejected the username/password")]
    AuthenticationFailed,
    #[error("SOCKS5 proxy refused the connection: {}", socks5_refusal_message(*.0))]
    Socks5ConnectRefused(u8),
    #[error("SOCKS5 proxy sent unknown address type {0:#04x}")]
    UnknownAddressType(u8),
    #[error("SOCKS4 proxy rejected the connection (code {0:#04x})")]
    Socks4ConnectRefused(u8),
}

/// Errors from a driver establishing a tunnel: either the proxy conversation
/// failed or the transport did.
#[cfg(any(feature = "futures-io", feature = "tokio"))]
#[derive(Debug, thiserror::Error)]
pub enum EstablishError {
    #[error(transparent)]
    Proxy(#[from] ProxyError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl ProxySpec {
    /// Parses a proxy URL with an `http`, `https`, `socks4`, `socks4a`,
    /// `socks5`, or `socks5h` scheme. Missing ports default by scheme (80,
    /// 443, or 1080). Credentials in the userinfo component are
    /// percent-decoded, per RFC 3986 §3.2.1.
    pub fn parse(url: &Url) -> Result<Self, ProxyError> {
        let scheme = match url.scheme() {
            "http" => ProxyScheme::Http { tls: false },
            "https" => ProxyScheme::Http { tls: true },
            "socks4" => ProxyScheme::Socks4 { remote_dns: false },
            "socks4a" => ProxyScheme::Socks4 { remote_dns: true },
            "socks5" => ProxyScheme::Socks5 { remote_dns: false },
            "socks5h" => ProxyScheme::Socks5 { remote_dns: true },
            other => return Err(ProxyError::UnsupportedScheme(other.to_string())),
        };

        let host = match url.host().ok_or(ProxyError::MissingHost)? {
            url::Host::Domain(domain) => domain.to_string(),
            url::Host::Ipv4(ip) => ip.to_string(),
            url::Host::Ipv6(ip) => ip.to_string(),
        };
        let default_port = match scheme {
            ProxyScheme::Http { tls: false } => 80,
            ProxyScheme::Http { tls: true } => 443,
            ProxyScheme::Socks4 { .. } | ProxyScheme::Socks5 { .. } => 1080,
        };
        let port = url.port().unwrap_or(default_port);

        let credentials = if url.username().is_empty() && url.password().is_none() {
            None
        } else {
            Some(Credentials {
                username: decode_userinfo(url.username())?,
                password: decode_userinfo(url.password().unwrap_or_default())?,
            })
        };

        Ok(Self {
            scheme,
            host,
            port,
            credentials,
        })
    }

    /// Whether the target should be handed to the proxy as a host name
    /// ([`Target::Domain`]) instead of being resolved locally. True for HTTP
    /// CONNECT and the `socks4a`/`socks5h` schemes, matching curl's scheme
    /// semantics.
    pub fn remote_dns(&self) -> bool {
        match self.scheme {
            ProxyScheme::Http { .. } => true,
            ProxyScheme::Socks4 { remote_dns } | ProxyScheme::Socks5 { remote_dns } => remote_dns,
        }
    }

    /// Whether the connection to the proxy itself must be wrapped in TLS
    /// before the handshake starts (an `https://` proxy URL).
    pub fn tls(&self) -> bool {
        matches!(self.scheme, ProxyScheme::Http { tls: true })
    }
}

/// Decodes one component (username or password) of a URL's userinfo, which
/// RFC 3986 §3.2.1 defines as percent-encoded: the proxy must be sent the
/// decoded form.
fn decode_userinfo(component: &str) -> Result<String, ProxyError> {
    percent_encoding::percent_decode_str(component)
        .decode_utf8()
        .map(|decoded| decoded.into_owned())
        .map_err(|_| ProxyError::InvalidCredentials)
}

/// Matches `host` against a `NO_PROXY`-style exclusion list: comma-separated
/// entries that are `*`, an exact host, or a domain suffix (bare, with a
/// leading dot, or with a leading `*.`). Entries may carry a `:port`, which
/// is ignored — exclusion is by host. CIDR ranges are not supported.
pub fn no_proxy_matches(no_proxy: &str, host: &str) -> bool {
    no_proxy
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .any(|entry| {
            if entry == "*" {
                return true;
            }
            // Bare IPv6 literals also contain colons, so only strip a port
            // when there is exactly one colon and the suffix parses as one.
            let entry = match entry.rsplit_once(':') {
                Some((prefix, suffix))
                    if !prefix.contains(':') && suffix.parse::<u16>().is_ok() =>
                {
                    prefix
                }
                _ => entry,
            };
            let entry = entry.strip_prefix("*.").unwrap_or(entry);
            let entry = entry.strip_prefix('.').unwrap_or(entry);
            host.eq_ignore_ascii_case(entry)
                || (host.len() > entry.len()
                    && host.as_bytes()[host.len() - entry.len() - 1] == b'.'
                    && host[host.len() - entry.len()..].eq_ignore_ascii_case(entry))
        })
}

/// Human-readable descriptions of the SOCKS5 reply field's error codes,
/// verbatim from RFC 1928 §6.
fn socks5_refusal_message(code: u8) -> String {
    match code {
        0x01 => "general SOCKS server failure".to_string(),
        0x02 => "connection not allowed by ruleset".to_string(),
        0x03 => "network unreachable".to_string(),
        0x04 => "host unreachable".to_string(),
        0x05 => "connection refused".to_string(),
        0x06 => "TTL expired".to_string(),
        0x07 => "command not supported".to_string(),
        0x08 => "address type not supported".to_string(),
        other => format!("unknown error code {other:#04x}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_applies_default_ports_by_scheme() {
        assert_eq!(parse("http://proxy.example.com").port, 80);
        assert_eq!(parse("https://proxy.example.com").port, 443);
        assert_eq!(parse("socks4://proxy.example.com").port, 1080);
        assert_eq!(parse("socks5://proxy.example.com").port, 1080);
        assert_eq!(parse("http://proxy.example.com:3128").port, 3128);
    }

    #[test]
    fn parse_maps_schemes() {
        assert_eq!(parse("http://p").scheme, ProxyScheme::Http { tls: false });
        assert_eq!(parse("https://p").scheme, ProxyScheme::Http { tls: true });
        assert_eq!(
            parse("socks4://p").scheme,
            ProxyScheme::Socks4 { remote_dns: false }
        );
        assert_eq!(
            parse("socks4a://p").scheme,
            ProxyScheme::Socks4 { remote_dns: true }
        );
        assert_eq!(
            parse("socks5://p").scheme,
            ProxyScheme::Socks5 { remote_dns: false }
        );
        assert_eq!(
            parse("socks5h://p").scheme,
            ProxyScheme::Socks5 { remote_dns: true }
        );
    }

    #[test]
    fn parse_rejects_unknown_scheme() {
        let url: Url = "ftp://proxy.example.com".parse().unwrap();
        let Err(error) = ProxySpec::parse(&url) else {
            panic!("expected parsing to fail");
        };
        assert!(matches!(error, ProxyError::UnsupportedScheme(scheme) if scheme == "ftp"));
    }

    #[test]
    fn parse_percent_decodes_credentials() {
        let spec = parse("http://user:p%40ss@proxy.example.com:8080");
        let credentials = spec.credentials.unwrap();
        assert_eq!(credentials.username, "user");
        assert_eq!(credentials.password, "p@ss");
    }

    #[test]
    fn parse_supports_username_only_credentials() {
        let spec = parse("socks4://userid@proxy.example.com");
        let credentials = spec.credentials.unwrap();
        assert_eq!(credentials.username, "userid");
        assert_eq!(credentials.password, "");
    }

    #[test]
    fn parse_unbrackets_ipv6_hosts() {
        assert_eq!(parse("http://[::1]:8080").host, "::1");
    }

    #[test]
    fn remote_dns_follows_scheme() {
        assert!(parse("http://p").remote_dns());
        assert!(parse("https://p").remote_dns());
        assert!(!parse("socks4://p").remote_dns());
        assert!(parse("socks4a://p").remote_dns());
        assert!(!parse("socks5://p").remote_dns());
        assert!(parse("socks5h://p").remote_dns());
    }

    #[test]
    fn credentials_debug_output_is_redacted() {
        let credentials = Credentials {
            username: "user".to_string(),
            password: "hunter2".to_string(),
        };
        let debug_output = format!("{credentials:?}");
        assert!(!debug_output.contains("hunter2"));
        assert!(debug_output.contains("<redacted>"));
    }

    #[test]
    fn no_proxy_matching() {
        assert!(no_proxy_matches("example.com", "example.com"));
        assert!(no_proxy_matches("example.com", "sub.example.com"));
        assert!(no_proxy_matches(".example.com", "sub.example.com"));
        assert!(no_proxy_matches("*.example.com", "sub.example.com"));
        assert!(no_proxy_matches("other.org, example.com", "example.com"));
        assert!(no_proxy_matches("example.com:443", "example.com"));
        assert!(no_proxy_matches("*", "anything.at.all"));
        assert!(no_proxy_matches("EXAMPLE.com", "example.COM"));
        assert!(no_proxy_matches("::1", "::1"));

        assert!(!no_proxy_matches("example.com", "notexample.com"));
        assert!(!no_proxy_matches("example.com", "example.org"));
        assert!(!no_proxy_matches("", "example.com"));
        assert!(!no_proxy_matches("sub.example.com", "example.com"));
    }

    fn parse(url: &str) -> ProxySpec {
        ProxySpec::parse(&url.parse().unwrap()).unwrap()
    }
}
