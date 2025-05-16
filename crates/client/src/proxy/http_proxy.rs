use anyhow::Result;
use base64::Engine;
use httparse::{EMPTY_HEADER, Response};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufStream},
    net::TcpStream,
};
use url::Url;

use super::AsyncReadWrite;

pub(super) struct HttpProxyContent<'t> {
    auth: Option<HttpProxyAuthorization<'t>>,
}

struct HttpProxyAuthorization<'t> {
    username: &'t str,
    password: &'t str,
}

pub(super) fn parse_http_proxy<'t>(proxy: &'t Url) -> Option<HttpProxyContent<'t>> {
    let auth = proxy.password().map(|password| HttpProxyAuthorization {
        username: proxy.username(),
        password,
    });
    Some(HttpProxyContent { auth })
}

pub(crate) async fn connect_with_http_proxy(
    stream: TcpStream,
    http_proxy: HttpProxyContent<'_>,
    rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    match http_proxy.auth {
        Some(HttpProxyAuthorization { username, password }) => {
            connect_with_auth(stream, rpc_host, username, password).await
        }
        None => connect(stream, rpc_host).await,
    }
}

async fn connect(stream: TcpStream, target: (&str, u16)) -> Result<Box<dyn AsyncReadWrite>> {
    let mut stream = BufStream::new(stream);
    let mut request = make_request(target);
    request.push_str("\r\n");
    stream.write_all(request.as_bytes()).await?;
    stream.flush().await?;

    check_response(&mut stream).await?;
    Ok(Box::new(stream))
}

async fn connect_with_auth(
    mut stream: TcpStream,
    target: (&str, u16),
    username: &str,
    password: &str,
) -> Result<Box<dyn AsyncReadWrite>> {
    let mut stream = BufStream::new(stream);
    let request = make_request_with_auth(target, username, password);
    stream.write_all(request.as_bytes()).await?;
    stream.flush().await?;

    check_response(&mut stream).await?;
    Ok(Box::new(stream))
}

fn make_request_with_auth(target: (&str, u16), username: &str, password: &str) -> String {
    let mut request = make_request(target);
    let auth =
        base64::prelude::BASE64_STANDARD.encode(format!("{}:{}", username, password).as_bytes());
    let proxy_authorization = format!("Proxy-Authorization: Basic {}\r\n", auth);
    request.push_str(&proxy_authorization);
    request.push_str("\r\n");
    request
}

fn make_request(target: (&str, u16)) -> String {
    let (host, port) = target;
    format!(
        "CONNECT {host}:{port} HTTP/1.1\r\n\
         Host: {host}:{port}\r\n\
         Proxy-Connection: Keep-Alive\r\n"
    )
}

async fn check_response(stream: &mut BufStream<TcpStream>) -> Result<()> {
    let response_string = get_response(stream).await?;
    parse_and_check(&response_string)
}

const MAXIMUM_RESPONSE_HEADER_LENGTH: usize = 4096;
const MAXIMUM_RESPONSE_HEADERS: usize = 16;

async fn get_response(stream: &mut BufStream<TcpStream>) -> Result<String> {
    let mut response = String::new();
    loop {
        if stream.read_line(&mut response).await? == 0 {
            return Err(anyhow::anyhow!("End of stream"));
        }

        if MAXIMUM_RESPONSE_HEADER_LENGTH < response.len() {
            return Err(anyhow::anyhow!("Maximum response header length exceeded"));
        }

        if response.ends_with("\r\n\r\n") {
            return Ok(response);
        }
    }
}

fn parse_and_check(response_string: &str) -> Result<()> {
    let mut response_headers = [EMPTY_HEADER; MAXIMUM_RESPONSE_HEADERS];
    let mut response = Response::new(&mut response_headers);
    response.parse(response_string.as_bytes())?;

    if response.code.is_some_and(|code| code != 200) {
        return Err(anyhow::anyhow!("Proxy response error"));
    }

    Ok(())
}
