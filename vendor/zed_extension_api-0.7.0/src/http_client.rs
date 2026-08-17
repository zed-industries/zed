//! An HTTP client.

pub use crate::wit::zed::extension::http_client::{
    HttpMethod, HttpRequest, HttpResponse, HttpResponseStream, RedirectPolicy, fetch, fetch_stream,
};

impl HttpRequest {
    /// Returns a builder for an [`HttpRequest`].
    pub fn builder() -> HttpRequestBuilder {
        HttpRequestBuilder::new()
    }

    /// Executes the [`HttpRequest`] with [`fetch`].
    pub fn fetch(&self) -> Result<HttpResponse, String> {
        fetch(self)
    }

    /// Executes the [`HttpRequest`] with [`fetch_stream`].
    pub fn fetch_stream(&self) -> Result<HttpResponseStream, String> {
        fetch_stream(self)
    }
}

/// A builder for an [`HttpRequest`].
#[derive(Clone)]
pub struct HttpRequestBuilder {
    method: Option<HttpMethod>,
    url: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
    redirect_policy: RedirectPolicy,
}

impl Default for HttpRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpRequestBuilder {
    /// Returns a new [`HttpRequestBuilder`].
    pub fn new() -> Self {
        HttpRequestBuilder {
            method: None,
            url: None,
            headers: Vec::new(),
            body: None,
            redirect_policy: RedirectPolicy::NoFollow,
        }
    }

    /// Sets the HTTP method for the request.
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = Some(method);
        self
    }

    /// Sets the URL for the request.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Adds a header to the request.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    /// Adds the specified headers to the request.
    pub fn headers(mut self, headers: impl IntoIterator<Item = (String, String)>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Sets the body of the request.
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Sets the redirect policy for the request.
    pub fn redirect_policy(mut self, policy: RedirectPolicy) -> Self {
        self.redirect_policy = policy;
        self
    }

    /// Builds the [`HttpRequest`].
    pub fn build(self) -> Result<HttpRequest, String> {
        let method = self.method.ok_or_else(|| "Method not set".to_string())?;
        let url = self.url.ok_or_else(|| "URL not set".to_string())?;

        Ok(HttpRequest {
            method,
            url,
            headers: self.headers,
            body: self.body,
            redirect_policy: self.redirect_policy,
        })
    }
}
