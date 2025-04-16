//! socks proxy
use anyhow::{Result, anyhow};
use http_client::Uri;
use tokio_socks::tcp::{Socks4Stream, Socks5Stream};

/// Data used for Socks5 Authorization
// NOTE: Do not implement Display or Debug
#[derive(Clone)]
struct Socks5Auth {
    username: String,
    password: String,
}

/// Socks Proxy types including data needed for Authorization
#[derive(Clone)]
enum SocksProxyType {
    // TODO: Socks4 identification using userid
    Socks4,
    Socks5(Option<Socks5Auth>),
}

/// Connect to a rpc host using an optional Socks4/Socks5 Proxy
pub(crate) async fn connect_socks_proxy_stream(
    proxy: Option<&Uri>,
    rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    // Connect directly incase there is no proxy
    let Some((address, proxy_type)) = parse_socks_proxy(proxy) else {
        return Ok(Box::new(tokio::net::TcpStream::connect(rpc_host).await?));
    };

    let socket = tokio::net::TcpStream::connect(address).await?;

    let proxy_stream: Box<dyn AsyncReadWrite> = match proxy_type {
        SocksProxyType::Socks4 => Box::new(
            Socks4Stream::connect_with_socket(socket, rpc_host)
                .await
                .map_err(|e| anyhow!("SOCKS4 connection failed: {e}"))?,
        ),
        SocksProxyType::Socks5(auth) => match auth {
            Some(auth) => Box::new(
                Socks5Stream::connect_with_password_and_socket(
                    socket,
                    rpc_host,
                    &auth.username,
                    &auth.password,
                )
                .await
                .map_err(|e| anyhow!("SOCKS5 (auth) connection failed: {e}"))?,
            ),
            None => Box::new(
                Socks5Stream::connect_with_socket(socket, rpc_host)
                    .await
                    .map_err(|e| anyhow!("SOCKS5 connection failed: {e}"))?,
            ),
        },
    };

    Ok(proxy_stream)
}

fn parse_socks_proxy(proxy: Option<&Uri>) -> Option<((String, u16), SocksProxyType)> {
    let proxy_uri = proxy?;
    let scheme = proxy_uri.scheme_str()?;

    let host = proxy_uri.host()?;
    let port = proxy_uri.port_u16()?;

    let address = (host.to_string(), port);

    if scheme.starts_with("socks4") {
        Some((address, SocksProxyType::Socks4))
    } else if scheme.starts_with("socks") {
        let auth = parse_auth(proxy_uri);
        Some((address, SocksProxyType::Socks5(auth)))
    } else {
        None
    }
}

/// Attempt to parse a username and password from a Uri
fn parse_auth(uri: &Uri) -> Option<Socks5Auth> {
    let url = url::Url::parse(uri.to_string().as_str()).ok()?;
    let password = url.password()?;
    let auth = Socks5Auth {
        username: url.username().to_string(),
        password: password.to_string(),
    };
    Some(auth)
}

pub(crate) trait AsyncReadWrite:
    tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static
{
}
impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static> AsyncReadWrite
    for T
{
}
