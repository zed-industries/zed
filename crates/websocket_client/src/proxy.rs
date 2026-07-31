//! Proxy tunneling for WebSocket connections.
//!
//! The WebSocket dials a raw TCP stream instead of going through the app's
//! HTTP client, so proxy configuration must be applied here explicitly — an
//! explicitly configured proxy or the proxy environment variables
//! (`HTTPS_PROXY` and friends); otherwise networks that force traffic
//! through a proxy (or block direct egress) break only this connection.
//! The proxy protocols themselves (HTTP/HTTPS CONNECT and SOCKS4/4a/5/5h)
//! are spoken by the `proxy_handshake` crate; this module supplies the I/O
//! that crate deliberately doesn't own: DNS resolution, the TCP connection
//! to the proxy, and TLS when an `https://` proxy asks for it.

use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::Arc;

use anyhow::Context as _;
use futures::io::{AsyncRead, AsyncWrite};
use proxy_handshake::{ProxyScheme, ProxySpec, Target};
use smol::Async;
use url::Url;

/// A byte stream that can back a WebSocket connection, whether it reaches the
/// server directly or through a proxy tunnel.
pub(crate) trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> AsyncReadWrite for T {}

/// Picks the proxy for connecting to `host`: an explicitly `configured`
/// proxy beats the proxy environment variables. A `NO_PROXY` match for the
/// host disables both.
pub(crate) fn proxy_for_host(configured: Option<Url>, host: &str) -> Option<Url> {
    let proxy = configured.or_else(http_client::read_proxy_from_env)?;
    if let Some(no_proxy) = http_client::read_no_proxy_from_env()
        && proxy_handshake::no_proxy_matches(&no_proxy, host)
    {
        return None;
    }
    Some(proxy)
}

/// The proxy URL with any credentials omitted, safe to include in logs.
pub(crate) fn proxy_for_logging(proxy: &Url) -> String {
    let host = proxy.host_str().unwrap_or("<invalid>");
    match proxy.port() {
        Some(port) => format!("{}://{}:{}", proxy.scheme(), host, port),
        None => format!("{}://{}", proxy.scheme(), host),
    }
}

/// Connects to `proxy` and tunnels a stream through it to the target.
///
/// Every failure is returned as an error rather than falling back to a direct
/// connection: users configure proxies in contexts where security and privacy
/// matter, and silently bypassing the proxy would leak traffic.
pub(crate) async fn connect_proxy_stream(
    proxy: &Url,
    target_host: &str,
    target_port: u16,
) -> anyhow::Result<Box<dyn AsyncReadWrite>> {
    let spec = ProxySpec::parse(proxy).context("parsing proxy URL")?;

    let target = if spec.remote_dns() {
        Target::Domain(target_host.to_string(), target_port)
    } else {
        // SOCKS4 requests carry a raw IPv4 address, so the target must
        // resolve to one.
        let requires_ipv4 = matches!(spec.scheme, ProxyScheme::Socks4 { .. });
        let address = resolve(target_host, target_port)
            .await?
            .into_iter()
            .find(|address| !requires_ipv4 || address.is_ipv4())
            .with_context(|| format!("failed to resolve target host {target_host}"))?;
        Target::Address(address)
    };

    let proxy_address = resolve(&spec.host, spec.port)
        .await?
        .into_iter()
        .next()
        .with_context(|| format!("failed to resolve proxy host {}", spec.host))?;
    let stream = Async::<TcpStream>::connect(proxy_address)
        .await
        .with_context(|| format!("failed to connect to proxy at {}:{}", spec.host, spec.port))?;

    let stream: Box<dyn AsyncReadWrite> = if spec.tls() {
        let connector = futures_rustls::TlsConnector::from(Arc::new(http_client_tls::tls_config()));
        let server_name = rustls::pki_types::ServerName::try_from(spec.host.clone())
            .context("invalid DNS name for proxy TLS")?;
        Box::new(
            connector
                .connect(server_name, stream)
                .await
                .context("TLS handshake with proxy failed")?,
        )
    } else {
        Box::new(stream)
    };

    let stream = proxy_handshake::futures_io::establish(stream, &spec, &target)
        .await
        .context("error connecting through proxy")?;
    Ok(Box::new(stream))
}

pub(crate) async fn resolve(host: &str, port: u16) -> anyhow::Result<Vec<SocketAddr>> {
    let host = host.to_string();
    smol::unblock(move || {
        Ok((host.as_str(), port)
            .to_socket_addrs()
            .with_context(|| format!("failed to resolve {host}"))?
            .collect())
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proxy_for_logging_omits_credentials() {
        let proxy = Url::parse("http://user:hunter2@proxy.example.com:8080").unwrap();
        assert_eq!(proxy_for_logging(&proxy), "http://proxy.example.com:8080");

        let proxy = Url::parse("socks5://proxy.example.com").unwrap();
        assert_eq!(proxy_for_logging(&proxy), "socks5://proxy.example.com");
    }
}
