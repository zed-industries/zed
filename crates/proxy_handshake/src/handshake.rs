//! The sans-IO proxy handshake state machine.

use std::collections::VecDeque;
use std::net::{IpAddr, SocketAddr};

use base64::Engine as _;

use crate::{Credentials, MAX_HTTP_RESPONSE_LENGTH, ProxyError, ProxyScheme, ProxySpec, Target};

const MAX_HTTP_RESPONSE_HEADERS: usize = 64;

const SOCKS4_VERSION: u8 = 0x04;
const SOCKS5_VERSION: u8 = 0x05;
const SOCKS_COMMAND_CONNECT: u8 = 0x01;
const SOCKS4_REPLY_GRANTED: u8 = 0x5A;
const SOCKS5_METHOD_NO_AUTHENTICATION: u8 = 0x00;
const SOCKS5_METHOD_USERNAME_PASSWORD: u8 = 0x02;
const SOCKS5_ADDRESS_IPV4: u8 = 0x01;
const SOCKS5_ADDRESS_DOMAIN: u8 = 0x03;
const SOCKS5_ADDRESS_IPV6: u8 = 0x04;

/// A proxy handshake in progress.
///
/// Sans-IO: the handshake never performs I/O itself. Drive it by calling
/// [`advance`](Self::advance) with bytes received from the proxy and acting
/// on the returned [`Step`]s until it reports [`Step::Done`].
pub struct Handshake {
    exchanges: VecDeque<Exchange>,
    input: Vec<u8>,
}

/// What the caller must do next to make progress on the handshake.
#[derive(Debug, PartialEq, Eq)]
pub enum Step {
    /// Write these bytes to the proxy.
    Send(Vec<u8>),
    /// Read more bytes from the proxy and pass them to the next `advance`.
    NeedMoreInput,
    /// The tunnel is established. `leftover` holds any bytes the proxy sent
    /// past the end of the handshake; they belong to the tunneled stream and
    /// must be consumed before reading from the transport again.
    Done { leftover: Vec<u8> },
}

enum Exchange {
    Send(Vec<u8>),
    Expect(Expectation),
}

enum Expectation {
    HttpResponseHead,
    Socks5MethodSelection { offered: u8 },
    Socks5AuthReply,
    Socks5ConnectReply,
    Socks4ConnectReply,
}

impl Handshake {
    /// Plans the handshake for tunneling to `target` through the proxy
    /// described by `spec`.
    ///
    /// Passing a [`Target::Domain`] delegates DNS resolution to the proxy
    /// regardless of scheme (SOCKS4 proxies are sent the 4a extension);
    /// resolve the host locally and pass a [`Target::Address`] when
    /// [`ProxySpec::remote_dns`] is false, to follow the proxy URL's intent.
    pub fn new(spec: &ProxySpec, target: &Target) -> Result<Self, ProxyError> {
        let mut exchanges = VecDeque::new();
        match spec.scheme {
            ProxyScheme::Http { .. } => {
                exchanges.push_back(Exchange::Send(http_connect_request(
                    target,
                    spec.credentials.as_ref(),
                )));
                exchanges.push_back(Exchange::Expect(Expectation::HttpResponseHead));
            }
            ProxyScheme::Socks4 { .. } => {
                exchanges.push_back(Exchange::Send(socks4_connect_request(
                    target,
                    spec.credentials.as_ref(),
                )?));
                exchanges.push_back(Exchange::Expect(Expectation::Socks4ConnectReply));
            }
            ProxyScheme::Socks5 { .. } => {
                let method = if spec.credentials.is_some() {
                    SOCKS5_METHOD_USERNAME_PASSWORD
                } else {
                    SOCKS5_METHOD_NO_AUTHENTICATION
                };
                exchanges.push_back(Exchange::Send(vec![SOCKS5_VERSION, 1, method]));
                exchanges.push_back(Exchange::Expect(Expectation::Socks5MethodSelection {
                    offered: method,
                }));
                if let Some(credentials) = &spec.credentials {
                    exchanges.push_back(Exchange::Send(socks5_auth_request(credentials)?));
                    exchanges.push_back(Exchange::Expect(Expectation::Socks5AuthReply));
                }
                exchanges.push_back(Exchange::Send(socks5_connect_request(target)?));
                exchanges.push_back(Exchange::Expect(Expectation::Socks5ConnectReply));
            }
        }
        Ok(Self {
            exchanges,
            input: Vec::new(),
        })
    }

    /// Advances the state machine with bytes newly received from the proxy
    /// (empty on the first call, and whenever there is nothing new to feed).
    ///
    /// Callers must not feed the same bytes twice: partial input is buffered
    /// internally until it can be parsed.
    pub fn advance(&mut self, received: &[u8]) -> Result<Step, ProxyError> {
        self.input.extend_from_slice(received);
        loop {
            match self.exchanges.pop_front() {
                None => {
                    return Ok(Step::Done {
                        leftover: std::mem::take(&mut self.input),
                    });
                }
                Some(Exchange::Send(bytes)) => return Ok(Step::Send(bytes)),
                Some(Exchange::Expect(expectation)) => {
                    match try_parse(&expectation, &self.input)? {
                        Some(consumed) => {
                            self.input.drain(..consumed);
                        }
                        None => {
                            self.exchanges.push_front(Exchange::Expect(expectation));
                            return Ok(Step::NeedMoreInput);
                        }
                    }
                }
            }
        }
    }
}

/// Attempts to parse one expected proxy message from the buffered input.
/// Returns the number of bytes consumed, or `None` when more input is needed.
fn try_parse(expectation: &Expectation, input: &[u8]) -> Result<Option<usize>, ProxyError> {
    match expectation {
        Expectation::HttpResponseHead => parse_http_response_head(input),
        Expectation::Socks5MethodSelection { offered } => {
            let Some(reply) = input.get(..2) else {
                return Ok(None);
            };
            if reply[0] != SOCKS5_VERSION {
                return Err(ProxyError::UnexpectedSocksVersion(reply[0]));
            }
            if reply[1] != *offered {
                return Err(ProxyError::AuthMethodRejected { selected: reply[1] });
            }
            Ok(Some(2))
        }
        Expectation::Socks5AuthReply => {
            let Some(reply) = input.get(..2) else {
                return Ok(None);
            };
            // RFC 1929: any non-zero status is failure.
            if reply[1] != 0x00 {
                return Err(ProxyError::AuthenticationFailed);
            }
            Ok(Some(2))
        }
        Expectation::Socks5ConnectReply => {
            let Some(header) = input.get(..4) else {
                return Ok(None);
            };
            if header[0] != SOCKS5_VERSION {
                return Err(ProxyError::UnexpectedSocksVersion(header[0]));
            }
            if header[1] != 0x00 {
                return Err(ProxyError::Socks5ConnectRefused(header[1]));
            }
            let address_length = match header[3] {
                SOCKS5_ADDRESS_IPV4 => 4,
                SOCKS5_ADDRESS_IPV6 => 16,
                SOCKS5_ADDRESS_DOMAIN => match input.get(4) {
                    Some(length) => 1 + *length as usize,
                    None => return Ok(None),
                },
                other => return Err(ProxyError::UnknownAddressType(other)),
            };
            // The reply's bound address and port only matter for BIND, so
            // they are consumed and discarded.
            let total = 4 + address_length + 2;
            if input.len() < total {
                return Ok(None);
            }
            Ok(Some(total))
        }
        Expectation::Socks4ConnectReply => {
            if input.len() < 8 {
                return Ok(None);
            }
            if input[1] != SOCKS4_REPLY_GRANTED {
                return Err(ProxyError::Socks4ConnectRefused(input[1]));
            }
            Ok(Some(8))
        }
    }
}

fn parse_http_response_head(input: &[u8]) -> Result<Option<usize>, ProxyError> {
    let mut headers = [httparse::EMPTY_HEADER; MAX_HTTP_RESPONSE_HEADERS];
    let mut response = httparse::Response::new(&mut headers);
    match response.parse(input) {
        Ok(httparse::Status::Complete(consumed)) => {
            let code = response.code.ok_or_else(|| {
                ProxyError::MalformedHttpResponse("missing status code".to_string())
            })?;
            if !(200..300).contains(&code) {
                return Err(ProxyError::HttpConnectRefused(code));
            }
            Ok(Some(consumed))
        }
        Ok(httparse::Status::Partial) => {
            if input.len() > MAX_HTTP_RESPONSE_LENGTH {
                Err(ProxyError::HttpResponseTooLarge)
            } else {
                Ok(None)
            }
        }
        Err(error) => Err(ProxyError::MalformedHttpResponse(error.to_string())),
    }
}

fn http_connect_request(target: &Target, credentials: Option<&Credentials>) -> Vec<u8> {
    let host = match target {
        Target::Domain(domain, port) => format!("{domain}:{port}"),
        // `SocketAddr`'s `Display` brackets IPv6 addresses, matching the
        // `host:port` form CONNECT requires.
        Target::Address(address) => address.to_string(),
    };
    let mut request =
        format!("CONNECT {host} HTTP/1.1\r\nHost: {host}\r\nProxy-Connection: Keep-Alive\r\n");
    if let Some(Credentials { username, password }) = credentials {
        let encoded = base64::prelude::BASE64_STANDARD.encode(format!("{username}:{password}"));
        request.push_str(&format!("Proxy-Authorization: Basic {encoded}\r\n"));
    }
    request.push_str("\r\n");
    request.into_bytes()
}

fn socks5_auth_request(credentials: &Credentials) -> Result<Vec<u8>, ProxyError> {
    let username = credentials.username.as_bytes();
    let password = credentials.password.as_bytes();
    if username.len() > 255 || password.len() > 255 {
        return Err(ProxyError::CredentialsTooLong);
    }
    let mut request = Vec::with_capacity(3 + username.len() + password.len());
    // RFC 1929 subnegotiation version.
    request.push(0x01);
    request.push(username.len() as u8);
    request.extend_from_slice(username);
    request.push(password.len() as u8);
    request.extend_from_slice(password);
    Ok(request)
}

fn socks5_connect_request(target: &Target) -> Result<Vec<u8>, ProxyError> {
    let mut request = vec![SOCKS5_VERSION, SOCKS_COMMAND_CONNECT, 0x00];
    let port = match target {
        Target::Domain(domain, port) => {
            let domain = domain.as_bytes();
            if domain.len() > 255 {
                return Err(ProxyError::DomainTooLong);
            }
            request.push(SOCKS5_ADDRESS_DOMAIN);
            request.push(domain.len() as u8);
            request.extend_from_slice(domain);
            *port
        }
        Target::Address(address) => {
            match address.ip() {
                IpAddr::V4(ip) => {
                    request.push(SOCKS5_ADDRESS_IPV4);
                    request.extend_from_slice(&ip.octets());
                }
                IpAddr::V6(ip) => {
                    request.push(SOCKS5_ADDRESS_IPV6);
                    request.extend_from_slice(&ip.octets());
                }
            }
            address.port()
        }
    };
    request.extend_from_slice(&port.to_be_bytes());
    Ok(request)
}

fn socks4_connect_request(
    target: &Target,
    credentials: Option<&Credentials>,
) -> Result<Vec<u8>, ProxyError> {
    let user_id = credentials
        .map(|credentials| credentials.username.as_str())
        .unwrap_or("");
    let mut request = vec![SOCKS4_VERSION, SOCKS_COMMAND_CONNECT];
    match target {
        Target::Address(SocketAddr::V4(address)) => {
            request.extend_from_slice(&address.port().to_be_bytes());
            request.extend_from_slice(&address.ip().octets());
            request.extend_from_slice(user_id.as_bytes());
            request.push(0x00);
        }
        Target::Address(SocketAddr::V6(_)) => return Err(ProxyError::Socks4Ipv4Only),
        Target::Domain(domain, port) => {
            // SOCKS4a: the invalid destination address 0.0.0.1 signals that
            // a host name follows the user id.
            request.extend_from_slice(&port.to_be_bytes());
            request.extend_from_slice(&[0, 0, 0, 1]);
            request.extend_from_slice(user_id.as_bytes());
            request.push(0x00);
            request.extend_from_slice(domain.as_bytes());
            request.push(0x00);
        }
    }
    Ok(request)
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn http_connect_completes_on_200_and_preserves_leftover() {
        let server_bytes = b"HTTP/1.1 200 Connection Established\r\n\r\ntunnel-bytes";
        let (sends, leftover) = run(
            "http://proxy:8080",
            &domain_target(),
            server_bytes,
            &[server_bytes.len()],
        )
        .unwrap();

        assert_eq!(sends.len(), 1);
        let request = String::from_utf8(sends[0].clone()).unwrap();
        assert!(
            request.starts_with("CONNECT cloud.example.com:443 HTTP/1.1\r\n"),
            "unexpected request: {request:?}"
        );
        assert!(request.contains("Host: cloud.example.com:443\r\n"));
        assert!(request.ends_with("\r\n\r\n"));
        assert_eq!(leftover, b"tunnel-bytes");
    }

    #[test]
    fn http_connect_sends_basic_authorization() {
        let (sends, _) = run(
            "http://user:pass@proxy:8080",
            &domain_target(),
            b"HTTP/1.1 200 OK\r\n\r\n",
            &[64],
        )
        .unwrap();

        let request = String::from_utf8(sends[0].clone()).unwrap();
        // "dXNlcjpwYXNz" is base64("user:pass").
        assert!(
            request.contains("Proxy-Authorization: Basic dXNlcjpwYXNz\r\n"),
            "unexpected request: {request:?}"
        );
    }

    #[test]
    fn http_connect_fails_on_error_status() {
        let error = run(
            "http://proxy:8080",
            &domain_target(),
            b"HTTP/1.1 407 Proxy Authentication Required\r\nProxy-Authenticate: Basic\r\n\r\n",
            &[64],
        )
        .unwrap_err();
        assert!(matches!(error, ProxyError::HttpConnectRefused(407)));
    }

    #[test]
    fn http_connect_rejects_oversized_response() {
        // A single never-ending header keeps the response incomplete (and
        // under httparse's header-count limit) until the size cap trips.
        let mut server_bytes = b"HTTP/1.1 200 OK\r\nX-Padding: ".to_vec();
        server_bytes.resize(MAX_HTTP_RESPONSE_LENGTH + 1024, b'a');
        let error = run("http://proxy:8080", &domain_target(), &server_bytes, &[64]).unwrap_err();
        assert!(matches!(error, ProxyError::HttpResponseTooLarge));
    }

    #[test]
    fn socks5_connect_with_domain_target() {
        let mut server_bytes = vec![0x05, 0x00];
        server_bytes.extend_from_slice(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0]);
        server_bytes.extend_from_slice(b"tunnel");
        let (sends, leftover) = run(
            "socks5h://proxy:1080",
            &domain_target(),
            &server_bytes,
            &[server_bytes.len()],
        )
        .unwrap();

        assert_eq!(sends[0], vec![0x05, 0x01, 0x00]);
        let mut expected_connect = vec![0x05, 0x01, 0x00, 0x03, 17];
        expected_connect.extend_from_slice(b"cloud.example.com");
        expected_connect.extend_from_slice(&443u16.to_be_bytes());
        assert_eq!(sends[1], expected_connect);
        assert_eq!(leftover, b"tunnel");
    }

    #[test]
    fn socks5_connect_with_username_password() {
        let mut server_bytes = vec![0x05, 0x02];
        server_bytes.extend_from_slice(&[0x01, 0x00]);
        server_bytes.extend_from_slice(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0]);
        let (sends, _) = run(
            "socks5h://user:pass@proxy:1080",
            &domain_target(),
            &server_bytes,
            &[server_bytes.len()],
        )
        .unwrap();

        assert_eq!(sends[0], vec![0x05, 0x01, 0x02]);
        assert_eq!(sends[1], b"\x01\x04user\x04pass".to_vec());
        assert_eq!(sends.len(), 3);
    }

    #[test]
    fn socks5_reports_connection_refusal() {
        let mut server_bytes = vec![0x05, 0x00];
        server_bytes.extend_from_slice(&[0x05, 0x02, 0x00, 0x01, 0, 0, 0, 0, 0, 0]);
        let error = run(
            "socks5h://proxy:1080",
            &domain_target(),
            &server_bytes,
            &[server_bytes.len()],
        )
        .unwrap_err();
        assert!(matches!(error, ProxyError::Socks5ConnectRefused(0x02)));
    }

    #[test]
    fn socks5_reports_rejected_authentication_method() {
        let error = run(
            "socks5h://proxy:1080",
            &domain_target(),
            &[0x05, 0xFF],
            &[2],
        )
        .unwrap_err();
        assert!(matches!(
            error,
            ProxyError::AuthMethodRejected { selected: 0xFF }
        ));
    }

    #[test]
    fn socks5_encodes_ip_targets() {
        let mut server_bytes = vec![0x05, 0x00];
        server_bytes.extend_from_slice(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0]);

        let target = Target::Address("192.0.2.10:443".parse().unwrap());
        let (sends, _) = run("socks5://proxy:1080", &target, &server_bytes, &[64]).unwrap();
        assert_eq!(
            sends[1],
            vec![0x05, 0x01, 0x00, 0x01, 192, 0, 2, 10, 0x01, 0xBB]
        );

        let target = Target::Address("[2001:db8::1]:443".parse().unwrap());
        let (sends, _) = run("socks5://proxy:1080", &target, &server_bytes, &[64]).unwrap();
        assert_eq!(sends[1][3], SOCKS5_ADDRESS_IPV6);
        assert_eq!(sends[1].len(), 4 + 16 + 2);
    }

    #[test]
    fn socks5_reply_with_domain_bound_address_is_consumed() {
        let mut server_bytes = vec![0x05, 0x00];
        server_bytes.extend_from_slice(&[0x05, 0x00, 0x00, 0x03, 4]);
        server_bytes.extend_from_slice(b"host");
        server_bytes.extend_from_slice(&[0, 80]);
        server_bytes.extend_from_slice(b"rest");
        let (_, leftover) = run(
            "socks5h://proxy:1080",
            &domain_target(),
            &server_bytes,
            &[server_bytes.len()],
        )
        .unwrap();
        assert_eq!(leftover, b"rest");
    }

    #[test]
    fn socks4_connect_with_ipv4_target() {
        let target = Target::Address("192.0.2.10:443".parse().unwrap());
        let (sends, leftover) = run(
            "socks4://proxy:1080",
            &target,
            &[0x00, 0x5A, 0, 0, 0, 0, 0, 0],
            &[8],
        )
        .unwrap();
        assert_eq!(sends[0], vec![0x04, 0x01, 0x01, 0xBB, 192, 0, 2, 10, 0x00]);
        assert_eq!(leftover, b"");
    }

    #[test]
    fn socks4a_connect_with_domain_target_and_user_id() {
        let (sends, _) = run(
            "socks4a://userid@proxy:1080",
            &domain_target(),
            &[0x00, 0x5A, 0, 0, 0, 0, 0, 0],
            &[8],
        )
        .unwrap();
        let mut expected = vec![0x04, 0x01, 0x01, 0xBB, 0, 0, 0, 1];
        expected.extend_from_slice(b"userid\x00");
        expected.extend_from_slice(b"cloud.example.com\x00");
        assert_eq!(sends[0], expected);
    }

    #[test]
    fn socks4_rejects_ipv6_targets() {
        let spec = spec("socks4://proxy:1080");
        let target = Target::Address("[2001:db8::1]:443".parse().unwrap());
        let Err(error) = Handshake::new(&spec, &target) else {
            panic!("expected handshake construction to fail");
        };
        assert!(matches!(error, ProxyError::Socks4Ipv4Only));
    }

    #[test]
    fn socks4_reports_rejection() {
        let target = Target::Address("192.0.2.10:443".parse().unwrap());
        let error = run(
            "socks4://proxy:1080",
            &target,
            &[0x00, 0x5B, 0, 0, 0, 0, 0, 0],
            &[8],
        )
        .unwrap_err();
        assert!(matches!(error, ProxyError::Socks4ConnectRefused(0x5B)));
    }

    proptest! {
        /// The outcome of a handshake must not depend on how the proxy's
        /// bytes are chunked across reads.
        #[test]
        fn chunking_does_not_change_the_outcome(
            chunk_sizes in proptest::collection::vec(1usize..16, 1..32),
        ) {
            for (url, server_bytes) in transcripts() {
                let baseline = run(url, &domain_target(), &server_bytes, &[server_bytes.len()]);
                let chunked = run(url, &domain_target(), &server_bytes, &chunk_sizes);
                match (baseline, chunked) {
                    (Ok(expected), Ok(actual)) => prop_assert_eq!(expected, actual),
                    (Err(_), Err(_)) => {}
                    (baseline, chunked) => {
                        return Err(TestCaseError::fail(format!(
                            "outcomes diverged: baseline {baseline:?} vs chunked {chunked:?}"
                        )));
                    }
                }
            }
        }
    }

    /// Scripted proxy conversations used by the chunking property test, each
    /// with trailing bytes to pin leftover handling.
    fn transcripts() -> Vec<(&'static str, Vec<u8>)> {
        let mut socks5 = vec![0x05, 0x02];
        socks5.extend_from_slice(&[0x01, 0x00]);
        socks5.extend_from_slice(&[0x05, 0x00, 0x00, 0x03, 4]);
        socks5.extend_from_slice(b"host");
        socks5.extend_from_slice(&[0, 80]);
        socks5.extend_from_slice(b"leftover-bytes");

        vec![
            (
                "http://user:pass@proxy:8080",
                b"HTTP/1.1 200 Connection Established\r\nVia: test\r\n\r\nleftover-bytes".to_vec(),
            ),
            ("socks5h://user:pass@proxy:1080", socks5),
            (
                "socks4a://userid@proxy:1080",
                b"\x00\x5A\x00\x00\x00\x00\x00\x00leftover-bytes".to_vec(),
            ),
        ]
    }

    fn spec(url: &str) -> ProxySpec {
        ProxySpec::parse(&url.parse().unwrap()).unwrap()
    }

    fn domain_target() -> Target {
        Target::Domain("cloud.example.com".to_string(), 443)
    }

    /// Drives a handshake to completion against scripted proxy bytes,
    /// feeding input in the given chunk sizes (cycled). Returns everything
    /// the client sent and the leftover bytes reported at completion.
    fn run(
        url: &str,
        target: &Target,
        server_bytes: &[u8],
        chunk_sizes: &[usize],
    ) -> Result<(Vec<Vec<u8>>, Vec<u8>), ProxyError> {
        let mut handshake = Handshake::new(&spec(url), target)?;
        let mut sends = Vec::new();
        let mut remaining = server_bytes;
        let mut sizes = chunk_sizes.iter().copied().cycle();
        let mut pending: &[u8] = &[];
        loop {
            let step = handshake.advance(pending)?;
            pending = &[];
            match step {
                Step::Send(bytes) => sends.push(bytes),
                Step::NeedMoreInput => {
                    assert!(
                        !remaining.is_empty(),
                        "handshake wants more input than the transcript provides"
                    );
                    let size = sizes
                        .next()
                        .expect("chunk_sizes must not be empty")
                        .clamp(1, remaining.len());
                    let (chunk, rest) = remaining.split_at(size);
                    pending = chunk;
                    remaining = rest;
                }
                Step::Done { leftover } => {
                    let mut leftover = leftover;
                    // Any transcript bytes not yet fed to the machine are
                    // also tunnel bytes; account for them so results don't
                    // depend on chunking.
                    leftover.extend_from_slice(remaining);
                    return Ok((sends, leftover));
                }
            }
        }
    }
}
