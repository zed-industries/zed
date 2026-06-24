//! Per-connection logic: parse the first request, decide allowed/denied,
//! either pump bytes through to the upstream or close (with an explanatory
//! 511 for policy denials).
//!
//! Both CONNECT and HTTP forward go through here. After the policy decision,
//! the TCP connection is pinned to the approved destination — directly, or
//! via a CONNECT tunnel through the upstream proxy — and everything becomes
//! opaque byte pumping. We don't parse anything else (chunked encoding,
//! keep-alive HTTP requests after the first, etc.); because the whole
//! connection can only reach the one approved host, later requests the
//! client sends on it cannot escape the policy decision. Per-TCP-connection
//! event granularity, by design.

use crate::proxy::{
    DenyReason, ProxyEvent, RequestMethod, RequestOutcome, RuntimeState, UpstreamProxy,
};
use anyhow::{Context, Result, anyhow, bail};
use base64::Engine as _;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream, ToSocketAddrs as _};
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use url::Url;

/// Buffer size for each direction of bidir copy. 64 KiB is the sweet spot
/// for most networks — large enough to keep the pipe full, small enough
/// not to balloon memory under many concurrent connections.
const PUMP_BUFFER_SIZE: usize = 64 * 1024;

/// Cap on request/response header bytes. The proxy runs inside the editor
/// process and its sole client is model-driven code — exactly the party the
/// sandbox distrusts — so an unbounded header read would let a malicious
/// command balloon the editor's memory. 64 KiB is far beyond what real HTTP
/// clients send.
const MAX_HEADER_BYTES: usize = 64 * 1024;

/// How long to wait for the client's request headers. Pooled connections
/// that never send a request get closed; well-behaved clients retry on a
/// fresh connection. Cleared before the pump phase so long-lived idle
/// tunnels (long polls, slow downloads) are unaffected.
const HEADER_READ_TIMEOUT: Duration = Duration::from_secs(60);

/// Timeout for outbound TCP connects (direct or to the upstream proxy),
/// so a black-holed destination doesn't pin a connection thread for the
/// OS default (~75s or more).
const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

/// How long to wait for the upstream proxy's CONNECT response.
const UPSTREAM_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(30);

/// Top-level entry from the listener thread. Owns the client connection
/// and the runtime state; emits events; never returns errors that escape
/// the connection (those are logged at the listener level).
pub(crate) enum ClientStream {
    Tcp(TcpStream),
    #[cfg(unix)]
    Unix(UnixStream),
}

impl ClientStream {
    fn try_clone(&self) -> std::io::Result<Self> {
        match self {
            Self::Tcp(stream) => stream.try_clone().map(Self::Tcp),
            #[cfg(unix)]
            Self::Unix(stream) => stream.try_clone().map(Self::Unix),
        }
    }

    fn set_read_timeout(&self, duration: Option<Duration>) -> std::io::Result<()> {
        match self {
            Self::Tcp(stream) => stream.set_read_timeout(duration),
            #[cfg(unix)]
            Self::Unix(stream) => stream.set_read_timeout(duration),
        }
    }
}

impl Read for ClientStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Tcp(stream) => stream.read(buf),
            #[cfg(unix)]
            Self::Unix(stream) => stream.read(buf),
        }
    }
}

impl Write for ClientStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Tcp(stream) => stream.write(buf),
            #[cfg(unix)]
            Self::Unix(stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Tcp(stream) => stream.flush(),
            #[cfg(unix)]
            Self::Unix(stream) => stream.flush(),
        }
    }
}

trait StreamIo: Read + Write + Send + 'static {
    fn shutdown(&self, how: Shutdown) -> std::io::Result<()>;
}

impl StreamIo for ClientStream {
    fn shutdown(&self, how: Shutdown) -> std::io::Result<()> {
        match self {
            Self::Tcp(stream) => stream.shutdown(how),
            #[cfg(unix)]
            Self::Unix(stream) => stream.shutdown(how),
        }
    }
}

impl StreamIo for TcpStream {
    fn shutdown(&self, how: Shutdown) -> std::io::Result<()> {
        TcpStream::shutdown(self, how)
    }
}

pub(crate) fn handle(client: ClientStream, state: Arc<RuntimeState>) -> Result<()> {
    if let ClientStream::Tcp(stream) = &client
        && let Err(error) = stream.set_nodelay(true)
    {
        log::debug!("[http_proxy] failed to set TCP_NODELAY on client socket: {error}");
    }

    let mut client = client;
    if let Err(error) = client.set_read_timeout(Some(HEADER_READ_TIMEOUT)) {
        log::debug!("[http_proxy] failed to set client header read timeout: {error}");
    }

    // Read until end-of-headers (`\r\n\r\n`). The first request determines
    // the destination for this entire TCP connection.
    let (header_buf, header_end) = read_request_headers(&mut client)?;

    // The header timeout must not apply to the pump phase. (Socket options
    // are shared with the `try_clone` handles used by the pump.)
    if let Err(error) = client.set_read_timeout(None) {
        log::debug!("[http_proxy] failed to clear client read timeout: {error}");
    }

    let request = ParsedRequest::parse(&header_buf[..header_end])?;

    // Bytes after the request headers — the start of an HTTP request body
    // for forwarding cases, or (for unusually eager clients) bytes pipelined
    // ahead of the CONNECT response. Replayed to the upstream either way.
    let leftover_body = header_buf[header_end..].to_vec();

    match request {
        ParsedRequest::Connect { host, port } => {
            handle_connect(client, host, port, leftover_body, state)
        }
        ParsedRequest::Http {
            method,
            host,
            port,
            request_bytes,
        } => handle_http_forward(
            client,
            method,
            host,
            port,
            request_bytes,
            leftover_body,
            state,
        ),
    }
}

/// Reads request headers (until `\r\n\r\n`) from the client. Returns the
/// full buffer (which may include some bytes after the headers) and the
/// offset where the headers ended. Capped at [`MAX_HEADER_BYTES`].
fn read_request_headers(client: &mut ClientStream) -> Result<(Vec<u8>, usize)> {
    let mut buf = Vec::with_capacity(4096);
    let mut tmp = [0u8; 4096];
    let mut searched = 0usize;
    loop {
        let n = client.read(&mut tmp)?;
        if n == 0 {
            bail!("client closed before sending complete request headers");
        }
        buf.extend_from_slice(&tmp[..n]);
        // Only scan the new bytes (plus 3 bytes of overlap for a delimiter
        // straddling the read boundary), keeping the search linear overall.
        let scan_start = searched.saturating_sub(3);
        if let Some(end) = find_double_crlf(&buf[scan_start..]) {
            return Ok((buf, scan_start + end));
        }
        searched = buf.len();
        if buf.len() > MAX_HEADER_BYTES {
            bail!("request headers exceed {MAX_HEADER_BYTES} bytes");
        }
    }
}

fn find_double_crlf(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n").map(|i| i + 4)
}

/// Parsed first request from the client.
enum ParsedRequest {
    Connect {
        host: String,
        port: u16,
    },
    Http {
        method: String,
        host: String,
        port: u16,
        /// Request bytes (line + headers) to forward to the origin.
        /// Absolute-form requests are rewritten to origin-form (see
        /// [`build_origin_form_request`]); origin-form requests pass
        /// through verbatim.
        request_bytes: Vec<u8>,
    },
}

impl ParsedRequest {
    fn parse(headers: &[u8]) -> Result<Self> {
        let mut header_storage = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut header_storage);
        let status = req.parse(headers).context("malformed HTTP request")?;
        if !status.is_complete() {
            bail!("incomplete HTTP request after \\r\\n\\r\\n boundary");
        }

        let method = req
            .method
            .ok_or_else(|| anyhow!("missing HTTP method"))?
            .to_string();
        let target = req.path.ok_or_else(|| anyhow!("missing request target"))?;

        if method.eq_ignore_ascii_case("CONNECT") {
            let (host, port) = parse_authority_form(target)?;
            Ok(ParsedRequest::Connect { host, port })
        } else if target.starts_with("http://") || target.starts_with("https://") {
            let (host, port, url) = parse_absolute_form_target(target)?;
            let request_bytes = build_origin_form_request(&method, &url, req.version, req.headers);
            Ok(ParsedRequest::Http {
                method,
                host,
                port,
                request_bytes,
            })
        } else {
            // Origin-form request: destination comes from the Host: header,
            // and the bytes are already in the form an origin server expects,
            // so forward them verbatim.
            let host_hdr = req
                .headers
                .iter()
                .find(|h| h.name.eq_ignore_ascii_case("host"))
                .ok_or_else(|| anyhow!("origin-form request missing Host: header"))?;
            let value =
                std::str::from_utf8(host_hdr.value).context("Host: header is not valid UTF-8")?;
            let (host, port) = parse_host_header(value)?;
            Ok(ParsedRequest::Http {
                method,
                host,
                port,
                request_bytes: headers.to_vec(),
            })
        }
    }
}

/// Parse `host:port` (CONNECT authority-form). Port is required, per RFC 9112
/// §3.2.3.
///
/// The port is split off manually rather than via `Url::parse`, because `Url`
/// elides scheme-default ports — `host:80` parsed with an `http://` prefix
/// becomes indistinguishable from a missing port, and `CONNECT host:80` is
/// legitimate (e.g. `ws://` through a proxy). `Url` still canonicalizes the
/// host part (bracketed IPv6, IDN-to-punycode, lowercasing).
fn parse_authority_form(input: &str) -> Result<(String, u16)> {
    let (host_part, port_part) = input
        .rsplit_once(':')
        .ok_or_else(|| anyhow!("CONNECT target '{input}' must include a port"))?;
    let port: u16 = port_part
        .parse()
        .with_context(|| format!("CONNECT target '{input}' has an invalid port"))?;
    let parsed = Url::parse(&format!("http://{host_part}"))
        .with_context(|| format!("parsing CONNECT target '{input}'"))?;
    let host = parsed
        .host_str()
        .ok_or_else(|| anyhow!("CONNECT target '{input}' has no host"))?
        .to_string();
    Ok((host, port))
}

/// Parse an absolute-form HTTP request target like `http://foo.com/path`.
/// Returns the parsed URL too, for the origin-form rewrite.
fn parse_absolute_form_target(target: &str) -> Result<(String, u16, Url)> {
    let parsed =
        Url::parse(target).with_context(|| format!("parsing absolute-form target '{target}'"))?;
    let host = parsed
        .host_str()
        .ok_or_else(|| anyhow!("absolute-form target '{target}' has no host"))?
        .to_string();
    // `port_or_known_default` covers `http://foo.com` (→ 80) without forcing
    // the agent to spell it.
    let port = parsed
        .port_or_known_default()
        .ok_or_else(|| anyhow!("absolute-form target '{target}' has no port"))?;
    Ok((host, port, parsed))
}

/// Rewrite an absolute-form proxy request into origin-form for the origin
/// server.
///
/// RFC 9112 §3.2.2 requires origin servers to accept absolute-form, but many
/// real servers and frameworks don't handle it; every production proxy
/// rewrites, and so do we. The `Host` header is regenerated from the URL
/// (per the same section, a proxy must use the URI host and ignore a
/// mismatched `Host`), and `Proxy-*` headers — which are addressed to us and
/// may carry credentials (`Proxy-Authorization`) — are stripped rather than
/// leaked to the origin.
fn build_origin_form_request(
    method: &str,
    url: &Url,
    version: Option<u8>,
    headers: &[httparse::Header],
) -> Vec<u8> {
    let mut target = url.path().to_string();
    if let Some(query) = url.query() {
        target.push('?');
        target.push_str(query);
    }
    // `host_str` keeps brackets on IPv6 literals, which is what the Host
    // header wants. `url.port()` is None for scheme-default ports, matching
    // the convention of omitting them.
    let host_value = match (url.host_str(), url.port()) {
        (Some(host), Some(port)) => format!("{host}:{port}"),
        (Some(host), None) => host.to_string(),
        // Unreachable in practice: callers parsed the host already.
        (None, _) => String::new(),
    };
    let minor_version = version.unwrap_or(1);

    let mut out = Vec::with_capacity(256);
    out.extend_from_slice(format!("{method} {target} HTTP/1.{minor_version}\r\n").as_bytes());
    out.extend_from_slice(format!("Host: {host_value}\r\n").as_bytes());
    for header in headers {
        let name = header.name;
        if name.eq_ignore_ascii_case("host")
            || name
                .get(.."proxy-".len())
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("proxy-"))
        {
            continue;
        }
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(b": ");
        out.extend_from_slice(header.value);
        out.extend_from_slice(b"\r\n");
    }
    out.extend_from_slice(b"\r\n");
    out
}

/// Parse a `Host:` header value into `(host, port)`. Default port is 80
/// since this is only called for HTTP forward, never CONNECT.
fn parse_host_header(value: &str) -> Result<(String, u16)> {
    let value = value.trim();
    if value.is_empty() {
        bail!("empty Host header");
    }
    let parsed = Url::parse(&format!("http://{value}"))
        .with_context(|| format!("parsing Host header '{value}'"))?;
    let host = parsed
        .host_str()
        .ok_or_else(|| anyhow!("Host header '{value}' has no host"))?
        .to_string();
    let port = parsed.port().unwrap_or(80);
    Ok((host, port))
}

/// Normalize a hostname for allowlist matching. Strips brackets from IPv6
/// literals and a single trailing dot. Lowercasing happens inside the
/// allowlist matcher.
fn normalize_host(host: &str) -> String {
    let stripped = host
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .unwrap_or(host);
    stripped.trim_end_matches('.').to_string()
}

/// Whether a hostname is an IP literal, per our policy.
fn is_ip_literal(host: &str) -> bool {
    let stripped = host
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .unwrap_or(host);
    stripped.parse::<IpAddr>().is_ok()
}

/// The policy check shared by CONNECT and HTTP forward: IP-literal targets
/// and hosts outside the allowlist are denied. Both checks are skipped when
/// the allowlist allows any host — that grant means unrestricted egress,
/// including IP literals (matching the pre-allowlist `allow_network`
/// behavior).
fn policy_denial(host: &str, port: u16, state: &RuntimeState) -> Option<DenyReason> {
    if state.allowlist.allows_any() {
        return None;
    }
    if is_ip_literal(host) {
        return Some(DenyReason::IpLiteralRejected {
            target: format!("{host}:{port}"),
        });
    }
    if !state.allowlist.allows(host) {
        return Some(DenyReason::HostNotInAllowlist {
            host: host.to_string(),
        });
    }
    None
}

/// How an approved request will reach its destination.
enum Route {
    /// Connect directly to one of these resolved-and-vetted addresses.
    Direct(Vec<SocketAddr>),
    /// Tunnel through the upstream proxy with a CONNECT handshake.
    ViaUpstream(UpstreamProxy),
}

enum RouteFailure {
    /// Policy denial — respond with 511.
    Denied(DenyReason),
    /// Network-level failure — close silently, per "no proxy here".
    Error(anyhow::Error),
}

/// Decide how to reach `host:port`, resolving and vetting addresses for
/// direct connections.
///
/// Resolution happens here, in the unsandboxed editor process, so this is
/// also where we keep allowlisted hostnames from smuggling the sandboxed
/// command onto the local machine or local network: a hostname whose DNS
/// points into loopback / private / link-local space (DNS rebinding) is
/// denied, and the connection later uses the vetted addresses rather than
/// re-resolving. The filter is skipped when the allowlist allows any host,
/// since that grant means unrestricted egress. Upstream-proxied destinations
/// aren't resolved locally at all — the upstream does its own resolution
/// inside the user's trusted network.
fn plan_route(host: &str, port: u16, state: &RuntimeState) -> Result<Route, RouteFailure> {
    if let Some(upstream) = &state.upstream
        && !upstream.bypasses(host, port)
    {
        return Ok(Route::ViaUpstream(upstream.clone()));
    }

    let resolved: Vec<SocketAddr> = (host, port)
        .to_socket_addrs()
        .map_err(|error| RouteFailure::Error(anyhow!("resolving {host}:{port}: {error}")))?
        .collect();
    if resolved.is_empty() {
        return Err(RouteFailure::Error(anyhow!(
            "{host}:{port} did not resolve to any address"
        )));
    }

    let vetted: Vec<SocketAddr> = if state.allowlist.allows_any() {
        resolved
    } else {
        resolved
            .into_iter()
            .filter(|addr| !is_forbidden_ip(addr.ip()))
            .collect()
    };
    if vetted.is_empty() {
        return Err(RouteFailure::Denied(DenyReason::ResolvedToForbiddenIp {
            host: host.to_string(),
        }));
    }
    Ok(Route::Direct(vetted))
}

/// Whether a resolved address is in loopback / private / link-local space —
/// destinations a hostname allowlist must never reach. The Seatbelt rule
/// already blocks them for direct connections from the sandbox; the proxy
/// (which runs outside the sandbox) must not reopen them.
fn is_forbidden_ip(ip: IpAddr) -> bool {
    // Escape hatch for the NixOS sandbox integration tests only: their echo
    // servers live on the VM's private network, which this filter would
    // otherwise reject. It is compiled in ONLY under the
    // `nixos-integration-tests` feature (enabled via `sandbox/nixos-test` when
    // building `bwrap_test_helper`), so in a real Zed build the env var has no
    // effect and cannot disable DNS-rebinding/SSRF protection.
    #[cfg(feature = "nixos-integration-tests")]
    if std::env::var_os("ZED_SANDBOX_PROXY_ALLOW_LOCAL_IPS").is_some() {
        return false;
    }
    match ip {
        IpAddr::V4(v4) => is_forbidden_ipv4(v4),
        IpAddr::V6(v6) => {
            if let Some(v4) = v6.to_ipv4_mapped() {
                return is_forbidden_ipv4(v4);
            }
            v6.is_loopback()
                || v6.is_unspecified()
                // Link-local (fe80::/10) and unique-local (fc00::/7); the
                // dedicated `is_unicast_link_local` / `is_unique_local`
                // methods are not yet stable.
                || (v6.segments()[0] & 0xffc0) == 0xfe80
                || (v6.segments()[0] & 0xfe00) == 0xfc00
        }
    }
}

fn is_forbidden_ipv4(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local() // includes 169.254.169.254 cloud metadata
        || ip.is_unspecified()
        || ip.is_broadcast()
        // Shared address space (RFC 6598, 100.64.0.0/10): CGNAT, and notably
        // Tailscale-style overlay networks.
        || (octets[0] == 100 && (octets[1] & 0xc0) == 64)
}

fn handle_connect(
    mut client: ClientStream,
    host: String,
    port: u16,
    leftover_body: Vec<u8>,
    state: Arc<RuntimeState>,
) -> Result<()> {
    let normalized = normalize_host(&host);

    if let Some(reason) = policy_denial(&normalized, port, &state) {
        return deny_request(
            &mut client,
            &state,
            normalized,
            port,
            RequestMethod::Connect,
            reason,
        );
    }

    let route = match plan_route(&normalized, port, &state) {
        Ok(route) => route,
        Err(RouteFailure::Denied(reason)) => {
            return deny_request(
                &mut client,
                &state,
                normalized,
                port,
                RequestMethod::Connect,
                reason,
            );
        }
        Err(RouteFailure::Error(error)) => {
            log::debug!("[http_proxy] routing failed for CONNECT {normalized}:{port}: {error:#}");
            // Per "no proxy here" — close abruptly. Client sees a connection drop.
            return Ok(());
        }
    };

    emit(
        &state,
        ProxyEvent::RequestAttempt {
            host: normalized.clone(),
            port,
            method: RequestMethod::Connect,
            outcome: RequestOutcome::Allowed,
        },
    );

    let (mut upstream, upstream_leftover) = match open_route(&route, &normalized, port) {
        Ok(opened) => opened,
        Err(error) => {
            log::debug!(
                "[http_proxy] upstream open failed for CONNECT {normalized}:{port}: {error:#}"
            );
            return Ok(());
        }
    };

    if let Err(error) = upstream.set_nodelay(true) {
        log::debug!("[http_proxy] failed to set TCP_NODELAY on upstream socket: {error}");
    }

    // Tell the client the tunnel is up, then replay anything the upstream
    // sent past its CONNECT response and anything the client pipelined ahead
    // of ours.
    client.write_all(b"HTTP/1.1 200 Connection established\r\n\r\n")?;
    if !upstream_leftover.is_empty() {
        client.write_all(&upstream_leftover)?;
    }
    if !leftover_body.is_empty() {
        upstream.write_all(&leftover_body)?;
    }

    let started = Instant::now();
    let (pumped_to_remote, pumped_from_remote) = pump_bidir(client, upstream);

    emit(
        &state,
        ProxyEvent::RequestCompleted {
            host: normalized,
            port,
            method: RequestMethod::Connect,
            bytes_to_remote: pumped_to_remote + leftover_body.len() as u64,
            bytes_from_remote: pumped_from_remote + upstream_leftover.len() as u64,
            duration_ms: started.elapsed().as_millis() as u64,
        },
    );

    Ok(())
}

fn handle_http_forward(
    mut client: ClientStream,
    method: String,
    host: String,
    port: u16,
    request_bytes: Vec<u8>,
    leftover_body: Vec<u8>,
    state: Arc<RuntimeState>,
) -> Result<()> {
    let normalized = normalize_host(&host);

    if let Some(reason) = policy_denial(&normalized, port, &state) {
        return deny_request(
            &mut client,
            &state,
            normalized,
            port,
            RequestMethod::Http(method),
            reason,
        );
    }

    let route = match plan_route(&normalized, port, &state) {
        Ok(route) => route,
        Err(RouteFailure::Denied(reason)) => {
            return deny_request(
                &mut client,
                &state,
                normalized,
                port,
                RequestMethod::Http(method),
                reason,
            );
        }
        Err(RouteFailure::Error(error)) => {
            log::debug!("[http_proxy] routing failed for {method} {normalized}:{port}: {error:#}");
            return Ok(());
        }
    };

    emit(
        &state,
        ProxyEvent::RequestAttempt {
            host: normalized.clone(),
            port,
            method: RequestMethod::Http(method.clone()),
            outcome: RequestOutcome::Allowed,
        },
    );

    let (mut upstream, upstream_leftover) = match open_route(&route, &normalized, port) {
        Ok(opened) => opened,
        Err(error) => {
            log::debug!(
                "[http_proxy] upstream open failed for {method} {normalized}:{port}: {error:#}"
            );
            return Ok(());
        }
    };

    if let Err(error) = upstream.set_nodelay(true) {
        log::debug!("[http_proxy] failed to set TCP_NODELAY on upstream socket: {error}");
    }

    if !upstream_leftover.is_empty() {
        client.write_all(&upstream_leftover)?;
    }
    upstream.write_all(&request_bytes)?;
    if !leftover_body.is_empty() {
        upstream.write_all(&leftover_body)?;
    }

    let started = Instant::now();
    let (pumped_to_remote, pumped_from_remote) = pump_bidir(client, upstream);
    let to_remote = pumped_to_remote + request_bytes.len() as u64 + leftover_body.len() as u64;

    emit(
        &state,
        ProxyEvent::RequestCompleted {
            host: normalized,
            port,
            method: RequestMethod::Http(method),
            bytes_to_remote: to_remote,
            bytes_from_remote: pumped_from_remote + upstream_leftover.len() as u64,
            duration_ms: started.elapsed().as_millis() as u64,
        },
    );

    Ok(())
}

/// Open the connection that will carry this request's bytes to the origin —
/// a direct TCP connection to a vetted address, or a CONNECT tunnel through
/// the upstream proxy. Returns the stream plus any bytes the upstream sent
/// past its CONNECT response (rare; replayed to the client by the caller).
///
/// HTTP forward also goes through a CONNECT tunnel when chaining: handing
/// the upstream a routable absolute-form byte stream would let later
/// keep-alive requests on this connection name a different (unapproved)
/// host and have the upstream route them there. A tunnel pins the whole
/// connection to the approved `host:port` — and shares the upstream auth
/// handshake with the CONNECT path.
fn open_route(route: &Route, host: &str, port: u16) -> Result<(TcpStream, Vec<u8>)> {
    match route {
        Route::Direct(addrs) => Ok((connect_to_any(addrs, host, port)?, Vec::new())),
        Route::ViaUpstream(upstream) => connect_via_upstream(host, port, upstream),
    }
}

/// Connect to the first address that accepts, with a per-attempt timeout.
fn connect_to_any(addrs: &[SocketAddr], host: &str, port: u16) -> Result<TcpStream> {
    let mut last_error = None;
    for addr in addrs {
        match TcpStream::connect_timeout(addr, CONNECT_TIMEOUT) {
            Ok(stream) => return Ok(stream),
            Err(error) => last_error = Some(error),
        }
    }
    match last_error {
        Some(error) => Err(anyhow!("connect to {host}:{port}: {error}")),
        None => Err(anyhow!("no addresses to connect to for {host}:{port}")),
    }
}

/// Open a TCP connection to the upstream proxy and complete a CONNECT
/// handshake to (host, port). Returns once the upstream has confirmed `200`,
/// along with any bytes it sent past its response headers.
fn connect_via_upstream(
    host: &str,
    port: u16,
    upstream: &UpstreamProxy,
) -> Result<(TcpStream, Vec<u8>)> {
    let addrs: Vec<SocketAddr> = (upstream.host.as_str(), upstream.port)
        .to_socket_addrs()
        .with_context(|| format!("resolving upstream proxy {upstream}"))?
        .collect();
    let mut stream = connect_to_any(&addrs, &upstream.host, upstream.port)
        .with_context(|| format!("connect to upstream proxy {upstream}"))?;

    if let Err(error) = stream.set_read_timeout(Some(UPSTREAM_HANDSHAKE_TIMEOUT)) {
        log::debug!("[http_proxy] failed to set upstream handshake timeout: {error}");
    }

    let auth_header = upstream.auth.as_ref().map(|auth| {
        let creds = format!("{}:{}", auth.user, auth.password);
        format!(
            "Proxy-Authorization: Basic {}\r\n",
            base64::engine::general_purpose::STANDARD.encode(creds.as_bytes())
        )
    });

    let request = format!(
        "CONNECT {host}:{port} HTTP/1.1\r\n\
         Host: {host}:{port}\r\n\
         {}\
         \r\n",
        auth_header.unwrap_or_default()
    );
    stream.write_all(request.as_bytes())?;

    // Read upstream's response status line + headers (up to \r\n\r\n).
    let mut buf = Vec::with_capacity(512);
    let mut tmp = [0u8; 512];
    let header_end = loop {
        let n = stream.read(&mut tmp)?;
        if n == 0 {
            bail!("upstream closed before responding to CONNECT");
        }
        buf.extend_from_slice(&tmp[..n]);
        if let Some(end) = find_double_crlf(&buf) {
            break end;
        }
        if buf.len() > MAX_HEADER_BYTES {
            bail!("upstream CONNECT response headers exceed {MAX_HEADER_BYTES} bytes");
        }
    };

    let mut header_storage = [httparse::EMPTY_HEADER; 16];
    let mut response = httparse::Response::new(&mut header_storage);
    response
        .parse(&buf[..header_end])
        .context("malformed upstream CONNECT response")?;
    let status = response
        .code
        .ok_or_else(|| anyhow!("upstream CONNECT response missing status code"))?;
    if status != 200 {
        let reason = response.reason.unwrap_or("");
        bail!("upstream CONNECT refused: HTTP {status} {reason}");
    }

    if let Err(error) = stream.set_read_timeout(None) {
        log::debug!("[http_proxy] failed to clear upstream handshake timeout: {error}");
    }

    // Bytes past the response headers already belong to the tunnel (an
    // origin could in principle speak first); hand them back for replay.
    Ok((stream, buf[header_end..].to_vec()))
}

/// Send a 511 response with `Via` and `Proxy-Status` headers and an
/// explanatory body. Closes the connection afterwards.
fn deny_request(
    client: &mut ClientStream,
    state: &RuntimeState,
    host: String,
    port: u16,
    method: RequestMethod,
    reason: DenyReason,
) -> Result<()> {
    emit(
        state,
        ProxyEvent::RequestAttempt {
            host,
            port,
            method,
            outcome: RequestOutcome::Denied {
                reason: reason.clone(),
            },
        },
    );

    let body = format!(
        "Request blocked by the Zed sandbox network policy.\n\n  \
         Reason: {}\n\n  \
         This is not a network or server failure — it's a policy decision.\n  \
         To proceed, ask the user to approve the host on the next terminal call.\n",
        reason.human_explanation()
    );
    let response = format!(
        "HTTP/1.1 511 Network Authentication Required\r\n\
         Via: 1.1 zed-sandbox-proxy\r\n\
         Proxy-Status: zed-sandbox-proxy; error={}; details=\"{}\"\r\n\
         Content-Type: text/plain; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\r\n{body}",
        reason.proxy_status_error(),
        proxy_status_details(&reason),
        body.len(),
    );
    client.write_all(response.as_bytes())?;
    Ok(())
}

fn proxy_status_details(reason: &DenyReason) -> String {
    reason
        .human_explanation()
        .replace(['\r', '\n'], " ")
        .replace('"', "'")
}

fn emit(state: &RuntimeState, event: ProxyEvent) {
    // Unbounded send is sync and never blocks. Only fails if the receiver
    // has been dropped, which we silently ignore — events are diagnostic,
    // not load-bearing.
    let _ = state.events.unbounded_send(event);
}

/// Bidirectional byte pump.
///
/// The client→remote direction runs on a spawned thread; remote→client runs
/// on the current (connection) thread, halving the thread count per
/// connection. Each direction reads from one socket and writes to the other
/// in a tight loop with a fixed-size buffer. When one direction reaches EOF
/// (peer closed write-half), the receiving side shuts down the other side's
/// write-half so the partner eventually sees EOF too. This mirrors what
/// `tokio::io::copy_bidirectional` does, with explicit thread join.
///
/// Returns `(client→remote bytes, remote→client bytes)`. Errors are
/// swallowed — partial transfer is fine, the caller just emits whatever
/// totals we got.
fn pump_bidir(client: ClientStream, upstream: TcpStream) -> (u64, u64) {
    // Two clones per direction so each side owns the half it touches.
    // `try_clone` dups the underlying fd, so reads/writes on the two
    // halves don't contend for the same kernel state.
    let client_read = match client.try_clone() {
        Ok(s) => s,
        Err(e) => {
            log::debug!("[http_proxy] failed to clone client socket for bidir pump: {e}");
            return (0, 0);
        }
    };
    let upstream_read = match upstream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            log::debug!("[http_proxy] failed to clone upstream socket for bidir pump: {e}");
            return (0, 0);
        }
    };
    let client_write = client;
    let upstream_write = upstream;

    let to_remote_handle = match thread::Builder::new()
        .name("http-proxy-pump-out".to_string())
        .stack_size(128 * 1024)
        .spawn(move || copy_one_way(client_read, upstream_write))
    {
        Ok(handle) => handle,
        Err(error) => {
            // Returning drops all stream handles, closing both sockets.
            log::warn!("[http_proxy] failed to spawn pump thread: {error}");
            return (0, 0);
        }
    };
    let from_remote = copy_one_way(upstream_read, client_write);
    let to_remote = to_remote_handle.join().unwrap_or_else(|_| {
        log::warn!("[http_proxy] pump thread panicked");
        0
    });
    (to_remote, from_remote)
}

/// Copy bytes from `from` to `to` until EOF on the read side or write
/// failure. Half-closes `to` for writes when done so the partner sees EOF.
fn copy_one_way(mut from: impl Read, mut to: impl StreamIo) -> u64 {
    let mut total = 0u64;
    let mut buf = vec![0u8; PUMP_BUFFER_SIZE];
    loop {
        let n = match from.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };
        if to.write_all(&buf[..n]).is_err() {
            break;
        }
        total += n as u64;
    }
    // Half-close the write side. Mirrors what tokio's copy_bidirectional
    // does on EOF: the partner's read returns EOF eventually, completing
    // the other direction.
    let _ = to.shutdown(Shutdown::Write);
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allowlist::{Allowlist, HostPattern};
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn plan_route_denies_host_resolving_to_loopback() {
        // `localhost` resolves to loopback without real DNS, standing in for
        // any allowlisted hostname whose DNS points at the local machine
        // (DNS rebinding).
        let state = runtime_state(Allowlist::from_patterns([
            HostPattern::parse("github.com").unwrap()
        ]));
        match plan_route("localhost", 80, &state) {
            Err(RouteFailure::Denied(DenyReason::ResolvedToForbiddenIp { host })) => {
                assert_eq!(host, "localhost");
            }
            Ok(_) => panic!("expected denial, got a route"),
            Err(RouteFailure::Denied(reason)) => panic!("unexpected deny reason: {reason:?}"),
            Err(RouteFailure::Error(error)) => panic!("expected denial, got error: {error}"),
        }
    }

    #[test]
    fn plan_route_allows_loopback_when_allowlist_allows_any() {
        let state = runtime_state(Allowlist::any());
        match plan_route("localhost", 80, &state) {
            Ok(Route::Direct(addrs)) => {
                assert!(!addrs.is_empty());
                assert!(addrs.iter().all(|addr| addr.ip().is_loopback()));
            }
            Ok(Route::ViaUpstream(_)) => panic!("expected direct route"),
            Err(RouteFailure::Denied(reason)) => panic!("unexpected denial: {reason:?}"),
            Err(RouteFailure::Error(error)) => panic!("unexpected error: {error}"),
        }
    }

    fn runtime_state(allowlist: Allowlist) -> RuntimeState {
        let (events, _receiver) = futures::channel::mpsc::unbounded();
        RuntimeState {
            allowlist,
            upstream: None,
            events,
            active_connections: AtomicUsize::new(0),
        }
    }

    #[test]
    fn parse_authority_form_basic() {
        let (h, p) = parse_authority_form("github.com:443").unwrap();
        assert_eq!(h, "github.com");
        assert_eq!(p, 443);
    }

    #[test]
    fn parse_authority_form_ipv6() {
        let (h, p) = parse_authority_form("[::1]:443").unwrap();
        assert_eq!(h, "[::1]");
        assert_eq!(p, 443);
    }

    #[test]
    fn parse_authority_form_accepts_scheme_default_ports() {
        // Regression test: `Url::parse` elides scheme-default ports, so a
        // naive URL round-trip would reject `host:80` as "missing a port".
        let (h, p) = parse_authority_form("example.com:80").unwrap();
        assert_eq!(h, "example.com");
        assert_eq!(p, 80);
    }

    #[test]
    fn parse_authority_form_requires_port() {
        assert!(parse_authority_form("github.com").is_err());
        assert!(parse_authority_form("[::1]").is_err());
    }

    #[test]
    fn parse_absolute_form_basic() {
        let (h, p, _) = parse_absolute_form_target("http://example.com/path").unwrap();
        assert_eq!(h, "example.com");
        assert_eq!(p, 80);
    }

    #[test]
    fn parse_absolute_form_with_port() {
        let (h, p, _) = parse_absolute_form_target("http://example.com:8080/").unwrap();
        assert_eq!(h, "example.com");
        assert_eq!(p, 8080);
    }

    #[test]
    fn host_header_default_port() {
        let (h, p) = parse_host_header("example.com").unwrap();
        assert_eq!(h, "example.com");
        assert_eq!(p, 80);
    }

    #[test]
    fn host_header_explicit_port() {
        let (h, p) = parse_host_header("example.com:8080").unwrap();
        assert_eq!(h, "example.com");
        assert_eq!(p, 8080);
    }

    #[test]
    fn host_header_ipv6() {
        let (h, p) = parse_host_header("[::1]:443").unwrap();
        assert_eq!(h, "[::1]");
        assert_eq!(p, 443);
    }

    #[test]
    fn detects_ip_literals() {
        assert!(is_ip_literal("1.2.3.4"));
        assert!(is_ip_literal("[::1]"));
        assert!(is_ip_literal("::1"));
        assert!(!is_ip_literal("github.com"));
        assert!(!is_ip_literal("localhost"));
    }

    #[test]
    fn forbidden_ips_cover_local_space() {
        for forbidden in [
            "127.0.0.1",
            "10.1.2.3",
            "172.16.0.1",
            "192.168.1.1",
            "169.254.169.254",
            "100.100.1.1",
            "0.0.0.0",
            "::1",
            "::",
            "fe80::1",
            "fd00::1",
            "::ffff:127.0.0.1",
            "::ffff:10.0.0.1",
        ] {
            assert!(
                is_forbidden_ip(forbidden.parse().unwrap()),
                "{forbidden} should be forbidden"
            );
        }
        for public in ["140.82.112.3", "8.8.8.8", "2606:4700::6810:84e5"] {
            assert!(
                !is_forbidden_ip(public.parse().unwrap()),
                "{public} should be allowed"
            );
        }
    }

    #[test]
    fn parsed_request_recognizes_connect() {
        let req = b"CONNECT example.com:443 HTTP/1.1\r\nHost: example.com:443\r\n\r\n";
        match ParsedRequest::parse(req).unwrap() {
            ParsedRequest::Connect { host, port } => {
                assert_eq!(host, "example.com");
                assert_eq!(port, 443);
            }
            ParsedRequest::Http { .. } => panic!("expected Connect"),
        }
    }

    #[test]
    fn parsed_request_recognizes_http_absolute_form() {
        let req = b"GET http://example.com/foo HTTP/1.1\r\nHost: example.com\r\n\r\n";
        match ParsedRequest::parse(req).unwrap() {
            ParsedRequest::Http {
                method, host, port, ..
            } => {
                assert_eq!(method, "GET");
                assert_eq!(host, "example.com");
                assert_eq!(port, 80);
            }
            ParsedRequest::Connect { .. } => panic!("expected Http"),
        }
    }

    #[test]
    fn absolute_form_is_rewritten_to_origin_form() {
        let req = b"GET http://example.com/foo?q=1 HTTP/1.1\r\n\
            Host: wrong.example\r\n\
            Proxy-Connection: keep-alive\r\n\
            Proxy-Authorization: Basic c2VjcmV0\r\n\
            User-Agent: test\r\n\r\n";
        match ParsedRequest::parse(req).unwrap() {
            ParsedRequest::Http { request_bytes, .. } => {
                let text = String::from_utf8(request_bytes).unwrap();
                assert!(text.starts_with("GET /foo?q=1 HTTP/1.1\r\n"), "{text}");
                // Host is regenerated from the URI, not trusted from the
                // (mismatched) header.
                assert!(text.contains("Host: example.com\r\n"), "{text}");
                assert!(!text.contains("wrong.example"), "{text}");
                // Proxy-* headers are addressed to us (and may carry
                // credentials); they must not leak to the origin.
                assert!(!text.to_ascii_lowercase().contains("proxy-"), "{text}");
                assert!(text.contains("User-Agent: test\r\n"), "{text}");
                assert!(text.ends_with("\r\n\r\n"), "{text}");
            }
            ParsedRequest::Connect { .. } => panic!("expected Http"),
        }
    }

    #[test]
    fn absolute_form_rewrite_keeps_non_default_port_in_host_header() {
        let req = b"GET http://example.com:8080/ HTTP/1.1\r\nHost: example.com:8080\r\n\r\n";
        match ParsedRequest::parse(req).unwrap() {
            ParsedRequest::Http { request_bytes, .. } => {
                let text = String::from_utf8(request_bytes).unwrap();
                assert!(text.starts_with("GET / HTTP/1.1\r\n"), "{text}");
                assert!(text.contains("Host: example.com:8080\r\n"), "{text}");
            }
            ParsedRequest::Connect { .. } => panic!("expected Http"),
        }
    }

    #[test]
    fn parsed_request_recognizes_http_origin_form_via_host_header() {
        let req = b"GET /foo HTTP/1.1\r\nHost: example.com:8080\r\n\r\n";
        match ParsedRequest::parse(req).unwrap() {
            ParsedRequest::Http {
                host,
                port,
                request_bytes,
                ..
            } => {
                assert_eq!(host, "example.com");
                assert_eq!(port, 8080);
                // Origin-form bytes pass through verbatim.
                assert_eq!(request_bytes, req.to_vec());
            }
            ParsedRequest::Connect { .. } => panic!("expected Http"),
        }
    }
}
