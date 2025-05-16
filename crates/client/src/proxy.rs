//! client proxy

mod http_proxy;
mod socks_proxy;

use anyhow::{Context, Result, anyhow};
use http_client::Url;
use http_proxy::{HttpProxyType, connect_with_http_proxy, parse_http_proxy};
use socks_proxy::{SocksVersion, connect_with_socks_proxy, parse_socks_proxy};

pub(crate) async fn connect_with_proxy_stream(
    proxy: &Url,
    rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    let Some((proxy_url, proxy_type)) = parse_proxy_type(proxy) else {
        // If parsing the proxy URL fails, we must avoid falling back to an insecure connection.
        // SOCKS proxies are often used in contexts where security and privacy are critical,
        // so any fallback could expose users to significant risks.
        return Err(anyhow!("Parsing proxy url failed"));
    };

    // Connect to proxy and wrap protocol later
    let stream = tokio::net::TcpStream::connect(proxy_url)
        .await
        .context("Failed to connect to socks proxy")?;

    let socks: Box<dyn AsyncReadWrite> = match proxy_type {
        ProxyType::SocksProxy(socks_version) => {
            connect_with_socks_proxy(stream, socks_version, rpc_host).await?
        }
        ProxyType::HttpProxy(http_proxy) => {
            connect_with_http_proxy(stream, http_proxy, rpc_host).await?
        }
    };

    Ok(socks)
}

enum ProxyType<'t> {
    SocksProxy(SocksVersion<'t>),
    HttpProxy(HttpProxyType<'t>),
}

fn parse_proxy_type<'t>(proxy: &'t Url) -> Option<((String, u16), ProxyType<'t>)> {
    let scheme = proxy.scheme();
    let host = proxy.host()?.to_string();
    let port = proxy.port_or_known_default()?;
    let proxy_type = match scheme {
        scheme if scheme.starts_with("socks") => {
            Some(ProxyType::SocksProxy(parse_socks_proxy(scheme, proxy)))
        }
        scheme if scheme.starts_with("http") => {
            Some(ProxyType::HttpProxy(parse_http_proxy(scheme, proxy)))
        }
        _ => None,
    }?;

    Some(((host, port), proxy_type))
}

pub(crate) trait AsyncReadWrite:
    tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static
{
}
impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static> AsyncReadWrite
    for T
{
}
