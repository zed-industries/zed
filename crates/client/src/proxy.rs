//! client proxy

mod http_proxy;
mod socks_proxy;

use anyhow::{Context, Result, anyhow};
use http_client::Url;
use http_proxy::{HttpProxyType, connect_with_http_proxy, parse_http_proxy};
use socks_proxy::{SocksVersion, connect_with_socks_proxy, parse_socks_proxy};

pub(crate) async fn connect_proxy_stream(
    proxy: &Url,
    rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    let Some(((proxy_domain, proxy_port), proxy_type)) = parse_proxy_type(proxy) else {
        // If parsing the proxy URL fails, we must avoid falling back to an insecure connection.
        // SOCKS proxies are often used in contexts where security and privacy are critical,
        // so any fallback could expose users to significant risks.
        return Err(anyhow!("Parsing proxy url failed"));
    };

    // Connect to proxy and wrap protocol later
    let stream = tokio::net::TcpStream::connect((proxy_domain.as_str(), proxy_port))
        .await
        .context("Failed to connect to proxy")?;

    let proxy_stream = match proxy_type {
        ProxyType::SocksProxy(proxy) => connect_with_socks_proxy(stream, proxy, rpc_host).await?,
        ProxyType::HttpProxy(proxy) => {
            connect_with_http_proxy(stream, proxy, rpc_host, &proxy_domain).await?
        }
    };

    Ok(proxy_stream)
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
