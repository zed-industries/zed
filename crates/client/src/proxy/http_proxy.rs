use anyhow::Result;
use base64::Engine;
use httparse::{EMPTY_HEADER, Response};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufStream},
    net::TcpStream,
};
use url::Url;

use super::AsyncReadWrite;

pub(super) enum HttpProxyType<'t> {
    HTTP(Option<HttpProxyAuthorization<'t>>),
    HTTPS(Option<HttpProxyAuthorization<'t>>),
}

struct HttpProxyAuthorization<'t> {
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

pub(crate) async fn connect_with_http_proxy(
    stream: TcpStream,
    http_proxy: HttpProxyType<'_>,
    rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    match http_proxy {
        HttpProxyType::HTTP(auth) => http_connect(stream, rpc_host, auth).await,
        HttpProxyType::HTTPS(auth) => https_connect(stream, rpc_host, auth).await,
    }
}

async fn http_connect(
    stream: TcpStream,
    target: (&str, u16),
    auth: Option<HttpProxyAuthorization<'_>>,
) -> Result<Box<dyn AsyncReadWrite>> {
    let mut stream = BufStream::new(stream);
    let request = make_request(target, auth);
    stream.write_all(request.as_bytes()).await?;
    stream.flush().await?;
    check_response(&mut stream).await?;
    Ok(Box::new(stream))
}

async fn https_connect(
    _stream: TcpStream,
    _target: (&str, u16),
    _auth: Option<HttpProxyAuthorization<'_>>,
) -> Result<Box<dyn AsyncReadWrite>> {
    Err(anyhow::anyhow!("HTTPS proxy not implemented"))
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

async fn check_response(stream: &mut BufStream<TcpStream>) -> Result<()> {
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

async fn recv_response(stream: &mut BufStream<TcpStream>) -> Result<String> {
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
