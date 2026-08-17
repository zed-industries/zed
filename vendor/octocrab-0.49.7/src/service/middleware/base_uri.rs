//! Set base URI of requests.
use http::{uri, Request, Uri};
use tower::{Layer, Service};

/// Layer that applies [`BaseUri`] which makes all requests relative to the URI.
///
/// Path in the base URI is preseved.
#[derive(Debug, Clone)]
pub struct BaseUriLayer {
    base_uri: http::Uri,
}

impl BaseUriLayer {
    /// Set base URI of requests.
    pub fn new(base_uri: http::Uri) -> Self {
        Self { base_uri }
    }
}

impl<S> Layer<S> for BaseUriLayer {
    type Service = BaseUri<S>;

    fn layer(&self, inner: S) -> Self::Service {
        BaseUri {
            base_uri: self.base_uri.clone(),
            inner,
        }
    }
}

/// Middleware that sets base URI so that all requests are relative to it.
#[derive(Debug, Clone)]
pub struct BaseUri<S> {
    base_uri: http::Uri,
    inner: S,
}

impl<S> BaseUri<S> {
    pub fn set_base_uri(&mut self, base_uri: http::Uri) {
        self.base_uri = base_uri;
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for BaseUri<S>
where
    S: Service<Request<ReqBody>>,
{
    type Error = S::Error;
    type Future = S::Future;
    type Response = S::Response;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let (mut parts, body) = req.into_parts();
        parts.uri = overwrite_base_uri(&self.base_uri, parts.uri);
        self.inner.call(Request::from_parts(parts, body))
    }
}

// Join base URI and Path+Query, preserving any path in the base.
fn overwrite_base_uri(base_uri: &http::Uri, current_uri: Uri) -> http::Uri {
    let req_pandq = current_uri.path_and_query();
    let mut builder = uri::Builder::new();
    match current_uri.scheme() {
        Some(scheme) => builder = builder.scheme(scheme.as_str()),
        None => {
            if let Some(scheme) = base_uri.scheme() {
                builder = builder.scheme(scheme.as_str());
            }
        }
    }
    match current_uri.authority() {
        Some(authority) => builder = builder.authority(authority.as_str()),
        None => {
            if let Some(authority) = base_uri.authority() {
                builder = builder.authority(authority.as_str());
            }
        }
    }

    if let Some(pandq) = base_uri.path_and_query() {
        builder = if let Some(req_pandq) = req_pandq {
            // Remove any trailing slashes and join.
            // `PathAndQuery` always starts with a slash.
            let base_path = pandq.path().trim_end_matches('/');
            if !req_pandq.as_str().starts_with(base_path) {
                builder.path_and_query(format!("{base_path}{req_pandq}"))
            } else {
                builder.path_and_query(req_pandq.as_str())
            }
        } else {
            builder.path_and_query(pandq.as_str())
        };
    } else if let Some(req_pandq) = req_pandq {
        builder = builder.path_and_query(req_pandq.as_str());
    }

    // Joining a valid Uri and valid PathAndQuery should result in a valid Uri.
    builder.build().expect("Valid Uri")
}

#[cfg(test)]
mod tests {
    #[test]
    fn normal_host() {
        let base_uri = http::Uri::from_static("https://192.168.1.65:8443");
        let apipath = http::Uri::from_static("/api/v1/nodes?hi=yes");
        assert_eq!(
            super::overwrite_base_uri(&base_uri, apipath),
            "https://192.168.1.65:8443/api/v1/nodes?hi=yes"
        );
    }

    #[test]
    fn rancher_host() {
        // in rancher, kubernetes server names are not hostnames, but a host with a path:
        let base_uri = http::Uri::from_static("https://example.com/foo/bar");
        let api_path = http::Uri::from_static("/api/v1/nodes?hi=yes");
        assert_eq!(
            super::overwrite_base_uri(&base_uri, api_path),
            "https://example.com/foo/bar/api/v1/nodes?hi=yes"
        );
    }
}
