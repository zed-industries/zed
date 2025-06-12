//! client proxy

mod http_proxy;
mod socks_proxy;

use anyhow::{Context as _, Result};
use hickory_resolver::{TokioAsyncResolver, config::LookupIpStrategy, system_conf};
use http_client::Url;
use http_proxy::{HttpProxyType, connect_http_proxy_stream, parse_http_proxy};
use socks_proxy::{SocksVersion, connect_socks_proxy_stream, parse_socks_proxy};
use util::ResultExt;

pub(crate) async fn connect_proxy_stream(
    proxy: &Url,
    rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    let Some(((proxy_domain, proxy_port), proxy_type)) = parse_proxy_type(proxy).await else {
        // If parsing the proxy URL fails, we must avoid falling back to an insecure connection.
        // SOCKS proxies are often used in contexts where security and privacy are critical,
        // so any fallback could expose users to significant risks.
        anyhow::bail!("Parsing proxy url type failed");
    };

    // Connect to proxy and wrap protocol later
    let stream = tokio::net::TcpStream::connect((proxy_domain.as_str(), proxy_port))
        .await
        .context("Failed to connect to proxy")?;

    let proxy_stream = match proxy_type {
        ProxyType::SocksProxy(proxy) => connect_socks_proxy_stream(stream, proxy, rpc_host).await?,
        ProxyType::HttpProxy(proxy) => {
            connect_http_proxy_stream(stream, proxy, rpc_host, &proxy_domain).await?
        }
    };

    Ok(proxy_stream)
}

enum ProxyType<'t> {
    SocksProxy(SocksVersion<'t>),
    HttpProxy(HttpProxyType<'t>),
}

async fn parse_proxy_type(proxy: &Url) -> Option<((String, u16), ProxyType<'_>)> {
    let scheme = proxy.scheme();
    let host = resolve_proxy_url_if_needed(proxy.host()?.to_string())
        .await
        .log_err()?;
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

async fn resolve_proxy_url_if_needed(proxy_domain: String) -> Result<String> {
    let (config, mut opts) = system_conf::read_system_conf().unwrap();
    opts.ip_strategy = LookupIpStrategy::Ipv4AndIpv6;
    let resolver = TokioAsyncResolver::tokio(config, opts);
    let ip = resolver
        .lookup_ip(proxy_domain.as_str())
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No IP found for proxy domain {proxy_domain}"))?;
    Ok(ip.to_string())
}

pub(crate) trait AsyncReadWrite:
    tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static
{
}
impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static> AsyncReadWrite
    for T
{
}
