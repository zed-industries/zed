//! socks proxy
use anyhow::{Result, anyhow};
use http_client::Url;
use tokio_socks::tcp::{Socks4Stream, Socks5Stream};

pub(crate) async fn connect_socks_proxy_stream(
    proxy: Option<&Url>,
    rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    let stream = match parse_socks_proxy(proxy) {
        Some((socks_proxy, SocksVersion::V4)) => {
            let stream = Socks4Stream::connect_with_socket(
                tokio::net::TcpStream::connect(socks_proxy).await?,
                rpc_host,
            )
            .await
            .map_err(|err| anyhow!("error connecting to socks {}", err))?;
            Box::new(stream) as Box<dyn AsyncReadWrite>
        }
        Some((socks_proxy, SocksVersion::V5)) => Box::new(
            Socks5Stream::connect_with_socket(
                tokio::net::TcpStream::connect(socks_proxy).await?,
                rpc_host,
            )
            .await
            .map_err(|err| anyhow!("error connecting to socks {}", err))?,
        ) as Box<dyn AsyncReadWrite>,
        None => {
            Box::new(tokio::net::TcpStream::connect(rpc_host).await?) as Box<dyn AsyncReadWrite>
        }
    };
    Ok(stream)
}

fn parse_socks_proxy(proxy: Option<&Url>) -> Option<((String, u16), SocksVersion)> {
    let proxy_url = proxy?;
    let scheme = proxy_url.scheme();
    let socks_version = if scheme.starts_with("socks4") {
        // socks4
        SocksVersion::V4
    } else if scheme.starts_with("socks") {
        // socks, socks5
        SocksVersion::V5
    } else {
        return None;
    };
    if let Some((host, port)) = proxy_url.host().zip(proxy_url.port_or_known_default()) {
        Some(((host.to_string(), port), socks_version))
    } else {
        None
    }
}

// private helper structs and traits

enum SocksVersion {
    V4,
    V5,
}

pub(crate) trait AsyncReadWrite:
    tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static
{
}
impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static> AsyncReadWrite
    for T
{
}
