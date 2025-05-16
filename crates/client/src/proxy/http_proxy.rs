use url::Url;

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
