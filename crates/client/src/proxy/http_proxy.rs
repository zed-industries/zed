use anyhow::Result;
use tokio::net::TcpStream;
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
        Some(HttpProxyAuthorization { username, password }) => {}
        None => {}
    }
    Err(anyhow::anyhow!("Not implemented"))
}
