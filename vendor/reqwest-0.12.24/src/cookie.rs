//! HTTP Cookies

use crate::header::{HeaderValue, SET_COOKIE};
use bytes::Bytes;
use std::convert::TryInto;
use std::fmt;
use std::sync::RwLock;
use std::time::SystemTime;

/// Actions for a persistent cookie store providing session support.
pub trait CookieStore: Send + Sync {
    /// Store a set of Set-Cookie header values received from `url`
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &url::Url);
    /// Get any Cookie values in the store for `url`
    fn cookies(&self, url: &url::Url) -> Option<HeaderValue>;
}

/// A single HTTP cookie.
pub struct Cookie<'a>(cookie_crate::Cookie<'a>);

/// A good default `CookieStore` implementation.
///
/// This is the implementation used when simply calling `cookie_store(true)`.
/// This type is exposed to allow creating one and filling it with some
/// existing cookies more easily, before creating a `Client`.
///
/// For more advanced scenarios, such as needing to serialize the store or
/// manipulate it between requests, you may refer to the
/// [reqwest_cookie_store crate](https://crates.io/crates/reqwest_cookie_store).
#[derive(Debug, Default)]
pub struct Jar(RwLock<cookie_store::CookieStore>);

// ===== impl Cookie =====

impl<'a> Cookie<'a> {
    fn parse(value: &'a HeaderValue) -> Result<Cookie<'a>, CookieParseError> {
        std::str::from_utf8(value.as_bytes())
            .map_err(cookie_crate::ParseError::from)
            .and_then(cookie_crate::Cookie::parse)
            .map_err(CookieParseError)
            .map(Cookie)
    }

    /// The name of the cookie.
    pub fn name(&self) -> &str {
        self.0.name()
    }

    /// The value of the cookie.
    pub fn value(&self) -> &str {
        self.0.value()
    }

    /// Returns true if the 'HttpOnly' directive is enabled.
    pub fn http_only(&self) -> bool {
        self.0.http_only().unwrap_or(false)
    }

    /// Returns true if the 'Secure' directive is enabled.
    pub fn secure(&self) -> bool {
        self.0.secure().unwrap_or(false)
    }

    /// Returns true if  'SameSite' directive is 'Lax'.
    pub fn same_site_lax(&self) -> bool {
        self.0.same_site() == Some(cookie_crate::SameSite::Lax)
    }

    /// Returns true if  'SameSite' directive is 'Strict'.
    pub fn same_site_strict(&self) -> bool {
        self.0.same_site() == Some(cookie_crate::SameSite::Strict)
    }

    /// Returns the path directive of the cookie, if set.
    pub fn path(&self) -> Option<&str> {
        self.0.path()
    }

    /// Returns the domain directive of the cookie, if set.
    pub fn domain(&self) -> Option<&str> {
        self.0.domain()
    }

    /// Get the Max-Age information.
    pub fn max_age(&self) -> Option<std::time::Duration> {
        self.0.max_age().map(|d| {
            d.try_into()
                .expect("time::Duration into std::time::Duration")
        })
    }

    /// The cookie expiration time.
    pub fn expires(&self) -> Option<SystemTime> {
        match self.0.expires() {
            Some(cookie_crate::Expiration::DateTime(offset)) => Some(SystemTime::from(offset)),
            None | Some(cookie_crate::Expiration::Session) => None,
        }
    }
}

impl<'a> fmt::Debug for Cookie<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub(crate) fn extract_response_cookie_headers<'a>(
    headers: &'a hyper::HeaderMap,
) -> impl Iterator<Item = &'a HeaderValue> + 'a {
    headers.get_all(SET_COOKIE).iter()
}

pub(crate) fn extract_response_cookies<'a>(
    headers: &'a hyper::HeaderMap,
) -> impl Iterator<Item = Result<Cookie<'a>, CookieParseError>> + 'a {
    headers
        .get_all(SET_COOKIE)
        .iter()
        .map(|value| Cookie::parse(value))
}

/// Error representing a parse failure of a 'Set-Cookie' header.
pub(crate) struct CookieParseError(cookie_crate::ParseError);

impl<'a> fmt::Debug for CookieParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> fmt::Display for CookieParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for CookieParseError {}

// ===== impl Jar =====

impl Jar {
    /// Add a cookie to this jar.
    ///
    /// # Example
    ///
    /// ```
    /// use reqwest::{cookie::Jar, Url};
    ///
    /// let cookie = "foo=bar; Domain=yolo.local";
    /// let url = "https://yolo.local".parse::<Url>().unwrap();
    ///
    /// let jar = Jar::default();
    /// jar.add_cookie_str(cookie, &url);
    ///
    /// // and now add to a `ClientBuilder`?
    /// ```
    pub fn add_cookie_str(&self, cookie: &str, url: &url::Url) {
        let cookies = cookie_crate::Cookie::parse(cookie)
            .ok()
            .map(|c| c.into_owned())
            .into_iter();
        self.0.write().unwrap().store_response_cookies(cookies, url);
    }
}

impl CookieStore for Jar {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &url::Url) {
        let iter =
            cookie_headers.filter_map(|val| Cookie::parse(val).map(|c| c.0.into_owned()).ok());

        self.0.write().unwrap().store_response_cookies(iter, url);
    }

    fn cookies(&self, url: &url::Url) -> Option<HeaderValue> {
        let s = self
            .0
            .read()
            .unwrap()
            .get_request_values(url)
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; ");

        if s.is_empty() {
            return None;
        }

        HeaderValue::from_maybe_shared(Bytes::from(s)).ok()
    }
}

pub(crate) mod service {
    use crate::cookie;
    use http::{Request, Response};
    use http_body::Body;
    use pin_project_lite::pin_project;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::task::ready;
    use std::task::Context;
    use std::task::Poll;
    use tower::Service;
    use url::Url;

    /// A [`Service`] that adds cookie support to a lower-level [`Service`].
    #[derive(Clone)]
    pub struct CookieService<S> {
        inner: S,
        cookie_store: Option<Arc<dyn cookie::CookieStore>>,
    }

    impl<S> CookieService<S> {
        /// Create a new [`CookieService`].
        pub fn new(inner: S, cookie_store: Option<Arc<dyn cookie::CookieStore>>) -> Self {
            Self {
                inner,
                cookie_store,
            }
        }
    }

    impl<ReqBody, ResBody, S> Service<Request<ReqBody>> for CookieService<S>
    where
        S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone,
        ReqBody: Body + Default,
    {
        type Response = Response<ResBody>;
        type Error = S::Error;
        type Future = ResponseFuture<S, ReqBody>;

        #[inline]
        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
            let clone = self.inner.clone();
            let mut inner = std::mem::replace(&mut self.inner, clone);
            let url = Url::parse(req.uri().to_string().as_str()).expect("invalid URL");
            if let Some(cookie_store) = self.cookie_store.as_ref() {
                if req.headers().get(crate::header::COOKIE).is_none() {
                    let headers = req.headers_mut();
                    crate::util::add_cookie_header(headers, &**cookie_store, &url);
                }
            }

            let cookie_store = self.cookie_store.clone();
            ResponseFuture {
                future: inner.call(req),
                cookie_store,
                url,
            }
        }
    }

    pin_project! {
        #[allow(missing_debug_implementations)]
        #[derive(Clone)]
        /// A [`Future`] that adds cookie support to a lower-level [`Future`].
        pub struct ResponseFuture<S, B>
        where
            S: Service<Request<B>>,
        {
            #[pin]
            future: S::Future,
            cookie_store: Option<Arc<dyn cookie::CookieStore>>,
            url: Url,
        }
    }

    impl<S, ReqBody, ResBody> Future for ResponseFuture<S, ReqBody>
    where
        S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone,
        ReqBody: Body + Default,
    {
        type Output = Result<Response<ResBody>, S::Error>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let cookie_store = self.cookie_store.clone();
            let url = self.url.clone();
            let res = ready!(self.project().future.as_mut().poll(cx)?);

            if let Some(cookie_store) = cookie_store.as_ref() {
                let mut cookies = cookie::extract_response_cookie_headers(res.headers()).peekable();
                if cookies.peek().is_some() {
                    cookie_store.set_cookies(&mut cookies, &url);
                }
            }
            Poll::Ready(Ok(res))
        }
    }
}
