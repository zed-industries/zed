use anyhow::{Context, Result};
use base64::Engine;
use httparse::{EMPTY_HEADER, Response};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufStream},
    net::TcpStream,
};
use tokio_native_tls::{TlsConnector, native_tls};
use url::Url;

use super::AsyncReadWrite;

pub(super) enum HttpProxyType<'t> {
    HTTP(Option<HttpProxyAuthorization<'t>>),
    HTTPS(Option<HttpProxyAuthorization<'t>>),
}

pub(super) struct HttpProxyAuthorization<'t> {
    username: &'t str,
    password: &'t str,
}

pub(super) fn parse_http_proxy<'t>(scheme: &str, proxy: &'t Url) -> HttpProxyType<'t> {
    let auth = proxy.password().map(|password| HttpProxyAuthorization {
        username: proxy.username(),
        password,
    });
    if scheme.starts_with("https") {
        HttpProxyType::HTTPS(auth)
    } else {
        HttpProxyType::HTTP(auth)
    }
}

pub(crate) async fn connect_http_proxy_stream(
    stream: TcpStream,
    http_proxy: HttpProxyType<'_>,
    rpc_host: (&str, u16),
    proxy_domain: &str,
) -> Result<Box<dyn AsyncReadWrite>> {
    match http_proxy {
        HttpProxyType::HTTP(auth) => http_connect(stream, rpc_host, auth).await,
        HttpProxyType::HTTPS(auth) => https_connect(stream, rpc_host, auth, proxy_domain).await,
    }
    .context("error connecting to http/https proxy")
}

async fn http_connect<T>(
    stream: T,
    target: (&str, u16),
    auth: Option<HttpProxyAuthorization<'_>>,
) -> Result<Box<dyn AsyncReadWrite>>
where
    T: AsyncReadWrite,
{
    let mut stream = BufStream::new(stream);
    let request = make_request(target, auth);
    stream.write_all(request.as_bytes()).await?;
    stream.flush().await?;
    check_response(&mut stream).await?;
    Ok(Box::new(stream))
}

async fn https_connect<T>(
    stream: T,
    target: (&str, u16),
    auth: Option<HttpProxyAuthorization<'_>>,
    proxy_domain: &str,
) -> Result<Box<dyn AsyncReadWrite>>
where
    T: AsyncReadWrite,
{
    let tls_connector = TlsConnector::from(native_tls::TlsConnector::new()?);
    let stream = tls_connector.connect(proxy_domain, stream).await?;
    http_connect(stream, target, auth).await
}

fn make_request(target: (&str, u16), auth: Option<HttpProxyAuthorization<'_>>) -> String {
    let (host, port) = target;
    let mut request = format!(
        "CONNECT {host}:{port} HTTP/1.1\r\nHost: {host}:{port}\r\nProxy-Connection: Keep-Alive\r\n"
    );
    if let Some(HttpProxyAuthorization { username, password }) = auth {
        let auth =
            base64::prelude::BASE64_STANDARD.encode(format!("{username}:{password}").as_bytes());
        let auth = format!("Proxy-Authorization: Basic {auth}\r\n");
        request.push_str(&auth);
    }
    request.push_str("\r\n");
    request
}

async fn check_response<T>(stream: &mut BufStream<T>) -> Result<()>
where
    T: AsyncReadWrite,
{
    let response = recv_response(stream).await?;
    let mut dummy_headers = [EMPTY_HEADER; MAX_RESPONSE_HEADERS];
    let mut parser = Response::new(&mut dummy_headers);
    parser.parse(response.as_bytes())?;

    if parser.code.is_some_and(|code| code != 200) {
        return Err(anyhow::anyhow!("Proxy response error"));
    }

    Ok(())
}

const MAX_RESPONSE_HEADER_LENGTH: usize = 4096;
const MAX_RESPONSE_HEADERS: usize = 16;

async fn recv_response<T>(stream: &mut BufStream<T>) -> Result<String>
where
    T: AsyncReadWrite,
{
    let mut response = String::new();
    loop {
        if stream.read_line(&mut response).await? == 0 {
            return Err(anyhow::anyhow!("End of stream"));
        }

        if MAX_RESPONSE_HEADER_LENGTH < response.len() {
            return Err(anyhow::anyhow!("Maximum response header length exceeded"));
        }

        if response.ends_with("\r\n\r\n") {
            return Ok(response);
        }
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::{HttpProxyAuthorization, HttpProxyType, parse_http_proxy};

    #[test]
    fn test_parse_http_proxy() {
        let proxy = Url::parse("http://proxy.example.com:1080").unwrap();
        let scheme = proxy.scheme();

        let version = parse_http_proxy(scheme, &proxy);
        assert!(matches!(version, HttpProxyType::HTTP(None)))
    }

    #[test]
    fn test_parse_http_proxy_with_auth() {
        let proxy = Url::parse("http://username:password@proxy.example.com:1080").unwrap();
        let scheme = proxy.scheme();

        let version = parse_http_proxy(scheme, &proxy);
        assert!(matches!(
            version,
            HttpProxyType::HTTP(Some(HttpProxyAuthorization {
                username: "username",
                password: "password"
            }))
        ))
    }
}
