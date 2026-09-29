//! Proxied connections for the collaboration WebSocket.
//!
//! Proxy URLs come from settings or the environment (see
//! `ProxySettings::proxy_url`); this module dials the proxy, wraps the
//! connection in TLS when an `https://` proxy asks for it, and tunnels
//! through it with `proxy_handshake`.

use anyhow::{Context as _, Result};
use http_client::Url;
use proxy_handshake::{ProxyScheme, ProxySpec, Target};
use tokio::net::TcpStream;

pub(crate) trait AsyncReadWrite:
    tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static
{
}
impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static> AsyncReadWrite
    for T
{
}

/// Whether `NO_PROXY` in the environment excludes `host` from proxying,
/// matching the exclusions the HTTP client already applies to its own
/// requests.
pub(crate) fn excluded_from_proxy(host: &str) -> bool {
    http_client::read_no_proxy_from_env()
        .is_some_and(|no_proxy| proxy_handshake::no_proxy_matches(&no_proxy, host))
}

pub(crate) async fn connect_proxy_stream(
    proxy: &Url,
    rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    // If parsing the proxy URL fails, we must avoid falling back to an
    // insecure connection. Proxies are often used in contexts where security
    // and privacy are critical, so any fallback could expose users to
    // significant risks.
    let spec = ProxySpec::parse(proxy).context("parsing proxy URL")?;

    let target = if spec.remote_dns() {
        Target::Domain(rpc_host.0.to_string(), rpc_host.1)
    } else {
        // SOCKS4 requests carry a raw IPv4 address, so the target must
        // resolve to one.
        let requires_ipv4 = matches!(spec.scheme, ProxyScheme::Socks4 { .. });
        let address = tokio::net::lookup_host(rpc_host)
            .await
            .with_context(|| format!("failed to lookup domain {}", rpc_host.0))?
            .find(|address| !requires_ipv4 || address.is_ipv4())
            .with_context(|| format!("failed to lookup domain {}", rpc_host.0))?;
        Target::Address(address)
    };

    let stream = TcpStream::connect((spec.host.as_str(), spec.port))
        .await
        .context("Failed to connect to proxy")?;

    let stream: Box<dyn AsyncReadWrite> = if spec.tls() {
        Box::new(connect_tls_to_proxy(stream, &spec.host).await?)
    } else {
        Box::new(stream)
    };

    let stream = proxy_handshake::tokio::establish(stream, &spec, &target)
        .await
        .context("error connecting through proxy")?;
    Ok(Box::new(stream))
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
async fn connect_tls_to_proxy(
    stream: TcpStream,
    proxy_domain: &str,
) -> Result<tokio_native_tls::TlsStream<TcpStream>> {
    use tokio_native_tls::{TlsConnector, native_tls};

    let tls_connector = TlsConnector::from(native_tls::TlsConnector::new()?);
    Ok(tls_connector.connect(proxy_domain, stream).await?)
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
async fn connect_tls_to_proxy(
    stream: TcpStream,
    proxy_domain: &str,
) -> Result<tokio_rustls::client::TlsStream<TcpStream>> {
    let proxy_domain = rustls_pki_types::ServerName::try_from(proxy_domain)
        .context("invalid DNS name for proxy TLS")?
        .to_owned();
    let tls_connector =
        tokio_rustls::TlsConnector::from(std::sync::Arc::new(http_client_tls::tls_config()));
    Ok(tls_connector.connect(proxy_domain, stream).await?)
}
