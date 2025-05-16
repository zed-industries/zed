//! client proxy

mod http_proxy;
mod socks_proxy;

use anyhow::{Context, Result, anyhow};
use http_client::Url;
use socks_proxy::{SocksVersion, connect_with_socks_proxy, parse_socks_proxy};

pub(crate) async fn connect_with_proxy_stream(
    proxy: &Url,
    rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    println!(
        "Connecting to socks proxy: {:?}, with ({:?})",
        proxy, rpc_host
    );
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
        ProxyType::HttpProxy => connect_with_http_proxy()?,
    };

    Ok(socks)
}

enum ProxyType<'t> {
    SocksProxy(SocksVersion<'t>),
    HttpProxy,
}

fn parse_proxy_type<'t>(proxy: &'t Url) -> Option<((String, u16), ProxyType<'t>)> {
    let scheme = proxy.scheme();
    let host = proxy.host()?.to_string();
    let port = proxy.port_or_known_default()?;
    let proxy_type = match scheme {
        scheme if scheme.starts_with("socks") => parse_socks_proxy(scheme, proxy),
        scheme if scheme.starts_with("http") => parse_http_proxy(proxy),
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

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::connect_with_proxy_stream;

    /// If parsing the proxy URL fails, we must avoid falling back to an insecure connection.
    /// SOCKS proxies are often used in contexts where security and privacy are critical,
    /// so any fallback could expose users to significant risks.
    #[tokio::test]
    async fn fails_on_bad_proxy() {
        // Should fail connecting because http is not a valid Socks proxy scheme
        let proxy = Url::parse("http://localhost:2313").unwrap();

        let result = connect_with_proxy_stream(&proxy, ("test", 1080)).await;
        match result {
            Err(e) => assert_eq!(e.to_string(), "Parsing proxy url failed"),
            Ok(_) => panic!("Connecting on bad proxy should fail"),
        };
    }
}
