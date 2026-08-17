//! CORS Filters

use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

use headers::{
    AccessControlAllowHeaders, AccessControlAllowMethods, AccessControlExposeHeaders, HeaderMapExt,
};
use http::header::{self, HeaderName, HeaderValue};

use crate::filter::{Filter, WrapSealed};
use crate::reject::{CombineRejection, Rejection};
use crate::reply::Reply;

use self::internal::{CorsFilter, IntoOrigin, Seconds};

/// Create a wrapping [`Filter`](crate::Filter) that exposes [CORS][] behavior for a wrapped
/// filter.
///
/// [CORS]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let cors = warp::cors()
///     .allow_origin("https://hyper.rs")
///     .allow_methods(vec!["GET", "POST", "DELETE"]);
///
/// let route = warp::any()
///     .map(warp::reply)
///     .with(cors);
/// ```
/// If you want to allow any route:
/// ```
/// use warp::Filter;
/// let cors = warp::cors()
///     .allow_any_origin();
/// ```
/// You can find more usage examples [here](https://github.com/seanmonstar/warp/blob/7fa54eaecd0fe12687137372791ff22fc7995766/tests/cors.rs).
pub fn cors() -> Builder {
    Builder {
        credentials: false,
        allowed_headers: HashSet::new(),
        exposed_headers: HashSet::new(),
        max_age: None,
        methods: HashSet::new(),
        origins: None,
    }
}

/// A wrapping [`Filter`](crate::Filter) constructed via `warp::cors()`.
#[derive(Clone, Debug)]
pub struct Cors {
    config: Arc<Configured>,
}

/// A constructed via `warp::cors()`.
#[derive(Clone, Debug)]
pub struct Builder {
    credentials: bool,
    allowed_headers: HashSet<HeaderName>,
    exposed_headers: HashSet<HeaderName>,
    max_age: Option<u64>,
    methods: HashSet<http::Method>,
    origins: Option<HashSet<HeaderValue>>,
}

impl Builder {
    /// Sets whether to add the `Access-Control-Allow-Credentials` header.
    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.credentials = allow;
        self
    }

    /// Adds a method to the existing list of allowed request methods.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `http::Method`.
    pub fn allow_method<M>(mut self, method: M) -> Self
    where
        http::Method: TryFrom<M>,
    {
        let method = match TryFrom::try_from(method) {
            Ok(m) => m,
            Err(_) => panic!("illegal Method"),
        };
        self.methods.insert(method);
        self
    }

    /// Adds multiple methods to the existing list of allowed request methods.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `http::Method`.
    pub fn allow_methods<I>(mut self, methods: I) -> Self
    where
        I: IntoIterator,
        http::Method: TryFrom<I::Item>,
    {
        let iter = methods.into_iter().map(|m| match TryFrom::try_from(m) {
            Ok(m) => m,
            Err(_) => panic!("illegal Method"),
        });
        self.methods.extend(iter);
        self
    }

    /// Adds a header to the list of allowed request headers.
    ///
    /// **Note**: These should match the values the browser sends via `Access-Control-Request-Headers`, e.g. `content-type`.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `http::header::HeaderName`.
    pub fn allow_header<H>(mut self, header: H) -> Self
    where
        HeaderName: TryFrom<H>,
    {
        let header = match TryFrom::try_from(header) {
            Ok(m) => m,
            Err(_) => panic!("illegal Header"),
        };
        self.allowed_headers.insert(header);
        self
    }

    /// Adds multiple headers to the list of allowed request headers.
    ///
    /// **Note**: These should match the values the browser sends via `Access-Control-Request-Headers`, e.g.`content-type`.
    ///
    /// # Panics
    ///
    /// Panics if any of the headers are not a valid `http::header::HeaderName`.
    pub fn allow_headers<I>(mut self, headers: I) -> Self
    where
        I: IntoIterator,
        HeaderName: TryFrom<I::Item>,
    {
        let iter = headers.into_iter().map(|h| match TryFrom::try_from(h) {
            Ok(h) => h,
            Err(_) => panic!("illegal Header"),
        });
        self.allowed_headers.extend(iter);
        self
    }

    /// Adds a header to the list of exposed headers.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `http::header::HeaderName`.
    pub fn expose_header<H>(mut self, header: H) -> Self
    where
        HeaderName: TryFrom<H>,
    {
        let header = match TryFrom::try_from(header) {
            Ok(m) => m,
            Err(_) => panic!("illegal Header"),
        };
        self.exposed_headers.insert(header);
        self
    }

    /// Adds multiple headers to the list of exposed headers.
    ///
    /// # Panics
    ///
    /// Panics if any of the headers are not a valid `http::header::HeaderName`.
    pub fn expose_headers<I>(mut self, headers: I) -> Self
    where
        I: IntoIterator,
        HeaderName: TryFrom<I::Item>,
    {
        let iter = headers.into_iter().map(|h| match TryFrom::try_from(h) {
            Ok(h) => h,
            Err(_) => panic!("illegal Header"),
        });
        self.exposed_headers.extend(iter);
        self
    }

    /// Sets that *any* `Origin` header is allowed.
    ///
    /// # Warning
    ///
    /// This can allow websites you didn't intend to access this resource,
    /// it is usually better to set an explicit list.
    pub fn allow_any_origin(mut self) -> Self {
        self.origins = None;
        self
    }

    /// Add an origin to the existing list of allowed `Origin`s.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `Origin`.
    pub fn allow_origin(self, origin: impl IntoOrigin) -> Self {
        self.allow_origins(Some(origin))
    }

    /// Add multiple origins to the existing list of allowed `Origin`s.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `Origin`.
    pub fn allow_origins<I>(mut self, origins: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoOrigin,
    {
        let iter = origins
            .into_iter()
            .map(IntoOrigin::into_origin)
            .map(|origin| {
                origin
                    .to_string()
                    .parse()
                    .expect("Origin is always a valid HeaderValue")
            });

        self.origins.get_or_insert_with(HashSet::new).extend(iter);

        self
    }

    /// Sets the `Access-Control-Max-Age` header.
    ///
    /// # Example
    ///
    ///
    /// ```
    /// use std::time::Duration;
    /// use warp::Filter;
    ///
    /// let cors = warp::cors()
    ///     .max_age(30) // 30u32 seconds
    ///     .max_age(Duration::from_secs(30)); // or a Duration
    /// ```
    pub fn max_age(mut self, seconds: impl Seconds) -> Self {
        self.max_age = Some(seconds.seconds());
        self
    }

    /// Builds the `Cors` wrapper from the configured settings.
    ///
    /// This step isn't *required*, as the `Builder` itself can be passed
    /// to `Filter::with`. This just allows constructing once, thus not needing
    /// to pay the cost of "building" every time.
    pub fn build(self) -> Cors {
        let expose_headers_header = if self.exposed_headers.is_empty() {
            None
        } else {
            Some(self.exposed_headers.iter().cloned().collect())
        };
        let allowed_headers_header = self.allowed_headers.iter().cloned().collect();
        let methods_header = self.methods.iter().cloned().collect();

        let config = Arc::new(Configured {
            cors: self,
            allowed_headers_header,
            expose_headers_header,
            methods_header,
        });

        Cors { config }
    }
}

impl<F> WrapSealed<F> for Builder
where
    F: Filter + Clone + Send + Sync + 'static,
    F::Extract: Reply,
    F::Error: CombineRejection<Rejection>,
    <F::Error as CombineRejection<Rejection>>::One: CombineRejection<Rejection>,
{
    type Wrapped = CorsFilter<F>;

    fn wrap(&self, inner: F) -> Self::Wrapped {
        let Cors { config } = self.clone().build();

        CorsFilter { config, inner }
    }
}

impl<F> WrapSealed<F> for Cors
where
    F: Filter + Clone + Send + Sync + 'static,
    F::Extract: Reply,
    F::Error: CombineRejection<Rejection>,
    <F::Error as CombineRejection<Rejection>>::One: CombineRejection<Rejection>,
{
    type Wrapped = CorsFilter<F>;

    fn wrap(&self, inner: F) -> Self::Wrapped {
        let config = self.config.clone();

        CorsFilter { config, inner }
    }
}

/// An error used to reject requests that are forbidden by a `cors` filter.
pub struct CorsForbidden {
    kind: Forbidden,
}

#[derive(Debug)]
enum Forbidden {
    OriginNotAllowed,
    MethodNotAllowed,
    HeaderNotAllowed,
}

impl fmt::Debug for CorsForbidden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CorsForbidden").field(&self.kind).finish()
    }
}

impl fmt::Display for CorsForbidden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let detail = match self.kind {
            Forbidden::OriginNotAllowed => "origin not allowed",
            Forbidden::MethodNotAllowed => "request-method not allowed",
            Forbidden::HeaderNotAllowed => "header not allowed",
        };
        write!(f, "CORS request forbidden: {}", detail)
    }
}

impl StdError for CorsForbidden {}

#[derive(Clone, Debug)]
struct Configured {
    cors: Builder,
    allowed_headers_header: AccessControlAllowHeaders,
    expose_headers_header: Option<AccessControlExposeHeaders>,
    methods_header: AccessControlAllowMethods,
}

enum Validated {
    Preflight(HeaderValue),
    Simple(HeaderValue),
    NotCors,
}

impl Configured {
    fn check_request(
        &self,
        method: &http::Method,
        headers: &http::HeaderMap,
    ) -> Result<Validated, Forbidden> {
        match (headers.get(header::ORIGIN), method) {
            (Some(origin), &http::Method::OPTIONS) => {
                // OPTIONS requests are preflight CORS requests...

                if !self.is_origin_allowed(origin) {
                    return Err(Forbidden::OriginNotAllowed);
                }

                if let Some(req_method) = headers.get(header::ACCESS_CONTROL_REQUEST_METHOD) {
                    if !self.is_method_allowed(req_method) {
                        return Err(Forbidden::MethodNotAllowed);
                    }
                } else {
                    tracing::trace!(
                        "preflight request missing access-control-request-method header"
                    );
                    return Err(Forbidden::MethodNotAllowed);
                }

                if let Some(req_headers) = headers.get(header::ACCESS_CONTROL_REQUEST_HEADERS) {
                    let headers = req_headers
                        .to_str()
                        .map_err(|_| Forbidden::HeaderNotAllowed)?;
                    for header in headers.split(',') {
                        if !self.is_header_allowed(header.trim()) {
                            return Err(Forbidden::HeaderNotAllowed);
                        }
                    }
                }

                Ok(Validated::Preflight(origin.clone()))
            }
            (Some(origin), _) => {
                // Any other method, simply check for a valid origin...

                tracing::trace!("origin header: {:?}", origin);
                if self.is_origin_allowed(origin) {
                    Ok(Validated::Simple(origin.clone()))
                } else {
                    Err(Forbidden::OriginNotAllowed)
                }
            }
            (None, _) => {
                // No `ORIGIN` header means this isn't CORS!
                Ok(Validated::NotCors)
            }
        }
    }

    fn is_method_allowed(&self, header: &HeaderValue) -> bool {
        http::Method::from_bytes(header.as_bytes())
            .map(|method| self.cors.methods.contains(&method))
            .unwrap_or(false)
    }

    fn is_header_allowed(&self, header: &str) -> bool {
        HeaderName::from_bytes(header.as_bytes())
            .map(|header| self.cors.allowed_headers.contains(&header))
            .unwrap_or(false)
    }

    fn is_origin_allowed(&self, origin: &HeaderValue) -> bool {
        if let Some(ref allowed) = self.cors.origins {
            allowed.contains(origin)
        } else {
            true
        }
    }

    fn append_preflight_headers(&self, headers: &mut http::HeaderMap) {
        self.append_common_headers(headers);

        headers.typed_insert(self.allowed_headers_header.clone());
        headers.typed_insert(self.methods_header.clone());

        if let Some(max_age) = self.cors.max_age {
            headers.insert(header::ACCESS_CONTROL_MAX_AGE, max_age.into());
        }
    }

    fn append_common_headers(&self, headers: &mut http::HeaderMap) {
        if self.cors.credentials {
            headers.insert(
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                HeaderValue::from_static("true"),
            );
        }
        if let Some(expose_headers_header) = &self.expose_headers_header {
            headers.typed_insert(expose_headers_header.clone())
        }
    }
}

mod internal {
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::task::{Context, Poll};

    use futures_util::{future, ready, TryFuture};
    use headers::Origin;
    use http::header;
    use pin_project::pin_project;

    use super::{Configured, CorsForbidden, Validated};
    use crate::filter::{Filter, FilterBase, Internal, One};
    use crate::generic::Either;
    use crate::reject::{CombineRejection, Rejection};
    use crate::route;

    #[derive(Clone, Debug)]
    pub struct CorsFilter<F> {
        pub(super) config: Arc<Configured>,
        pub(super) inner: F,
    }

    impl<F> FilterBase for CorsFilter<F>
    where
        F: Filter,
        F::Extract: Send,
        F::Future: Future,
        F::Error: CombineRejection<Rejection>,
    {
        type Extract =
            One<Either<One<Preflight>, One<Either<One<Wrapped<F::Extract>>, F::Extract>>>>;
        type Error = <F::Error as CombineRejection<Rejection>>::One;
        type Future = future::Either<
            future::Ready<Result<Self::Extract, Self::Error>>,
            WrappedFuture<F::Future>,
        >;

        fn filter(&self, _: Internal) -> Self::Future {
            let validated =
                route::with(|route| self.config.check_request(route.method(), route.headers()));

            match validated {
                Ok(Validated::Preflight(origin)) => {
                    let preflight = Preflight {
                        config: self.config.clone(),
                        origin,
                    };
                    future::Either::Left(future::ok((Either::A((preflight,)),)))
                }
                Ok(Validated::Simple(origin)) => future::Either::Right(WrappedFuture {
                    inner: self.inner.filter(Internal),
                    wrapped: Some((self.config.clone(), origin)),
                }),
                Ok(Validated::NotCors) => future::Either::Right(WrappedFuture {
                    inner: self.inner.filter(Internal),
                    wrapped: None,
                }),
                Err(err) => {
                    let rejection = crate::reject::known(CorsForbidden { kind: err });
                    future::Either::Left(future::err(rejection.into()))
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct Preflight {
        config: Arc<Configured>,
        origin: header::HeaderValue,
    }

    impl crate::reply::Reply for Preflight {
        fn into_response(self) -> crate::reply::Response {
            let mut res = crate::reply::Response::default();
            self.config.append_preflight_headers(res.headers_mut());
            res.headers_mut()
                .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, self.origin);
            res
        }
    }

    #[derive(Debug)]
    pub struct Wrapped<R> {
        config: Arc<Configured>,
        inner: R,
        origin: header::HeaderValue,
    }

    impl<R> crate::reply::Reply for Wrapped<R>
    where
        R: crate::reply::Reply,
    {
        fn into_response(self) -> crate::reply::Response {
            let mut res = self.inner.into_response();
            self.config.append_common_headers(res.headers_mut());
            res.headers_mut()
                .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, self.origin);
            res
        }
    }

    #[pin_project]
    #[derive(Debug)]
    pub struct WrappedFuture<F> {
        #[pin]
        inner: F,
        wrapped: Option<(Arc<Configured>, header::HeaderValue)>,
    }

    impl<F> Future for WrappedFuture<F>
    where
        F: TryFuture,
        F::Error: CombineRejection<Rejection>,
    {
        type Output = Result<
            One<Either<One<Preflight>, One<Either<One<Wrapped<F::Ok>>, F::Ok>>>>,
            <F::Error as CombineRejection<Rejection>>::One,
        >;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let pin = self.project();
            match ready!(pin.inner.try_poll(cx)) {
                Ok(inner) => {
                    let item = if let Some((config, origin)) = pin.wrapped.take() {
                        (Either::A((Wrapped {
                            config,
                            inner,
                            origin,
                        },)),)
                    } else {
                        (Either::B(inner),)
                    };
                    let item = (Either::B(item),);
                    Poll::Ready(Ok(item))
                }
                Err(err) => Poll::Ready(Err(err.into())),
            }
        }
    }

    pub trait Seconds {
        fn seconds(self) -> u64;
    }

    impl Seconds for u32 {
        fn seconds(self) -> u64 {
            self.into()
        }
    }

    impl Seconds for ::std::time::Duration {
        fn seconds(self) -> u64 {
            self.as_secs()
        }
    }

    pub trait IntoOrigin {
        fn into_origin(self) -> Origin;
    }

    impl<'a> IntoOrigin for &'a str {
        fn into_origin(self) -> Origin {
            let mut parts = self.splitn(2, "://");
            let scheme = parts.next().expect("missing scheme");
            let rest = parts.next().expect("missing scheme");

            Origin::try_from_parts(scheme, rest, None).expect("invalid Origin")
        }
    }
}
