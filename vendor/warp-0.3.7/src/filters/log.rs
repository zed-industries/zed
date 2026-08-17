//! Logger Filters

use std::fmt;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use http::{header, StatusCode};

use crate::filter::{Filter, WrapSealed};
use crate::reject::IsReject;
use crate::reply::Reply;
use crate::route::Route;

use self::internal::WithLog;

/// Create a wrapping [`Filter`](crate::Filter) with the specified `name` as the `target`.
///
/// This uses the default access logging format, and log records produced
/// will have their `target` set to `name`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// // If using something like `pretty_env_logger`,
/// // view logs by setting `RUST_LOG=example::api`.
/// let log = warp::log("example::api");
/// let route = warp::any()
///     .map(warp::reply)
///     .with(log);
/// ```
pub fn log(name: &'static str) -> Log<impl Fn(Info<'_>) + Copy> {
    let func = move |info: Info<'_>| {
        // TODO?
        // - response content length?
        log::info!(
            target: name,
            "{} \"{} {} {:?}\" {} \"{}\" \"{}\" {:?}",
            OptFmt(info.route.remote_addr()),
            info.method(),
            info.path(),
            info.route.version(),
            info.status().as_u16(),
            OptFmt(info.referer()),
            OptFmt(info.user_agent()),
            info.elapsed(),
        );
    };
    Log { func }
}

/// Create a wrapping [`Filter`](crate::Filter) that receives `warp::log::Info`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let log = warp::log::custom(|info| {
///     // Use a log macro, or slog, or println, or whatever!
///     eprintln!(
///         "{} {} {}",
///         info.method(),
///         info.path(),
///         info.status(),
///     );
/// });
/// let route = warp::any()
///     .map(warp::reply)
///     .with(log);
/// ```
pub fn custom<F>(func: F) -> Log<F>
where
    F: Fn(Info<'_>),
{
    Log { func }
}

/// Decorates a [`Filter`] to log requests and responses.
#[derive(Clone, Copy, Debug)]
pub struct Log<F> {
    func: F,
}

/// Information about the request/response that can be used to prepare log lines.
#[allow(missing_debug_implementations)]
pub struct Info<'a> {
    route: &'a Route,
    start: Instant,
    status: StatusCode,
}

impl<FN, F> WrapSealed<F> for Log<FN>
where
    FN: Fn(Info<'_>) + Clone + Send,
    F: Filter + Clone + Send,
    F::Extract: Reply,
    F::Error: IsReject,
{
    type Wrapped = WithLog<FN, F>;

    fn wrap(&self, filter: F) -> Self::Wrapped {
        WithLog {
            filter,
            log: self.clone(),
        }
    }
}

impl<'a> Info<'a> {
    /// View the remote `SocketAddr` of the request.
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.route.remote_addr()
    }

    /// View the `http::Method` of the request.
    pub fn method(&self) -> &http::Method {
        self.route.method()
    }

    /// View the URI path of the request.
    pub fn path(&self) -> &str {
        self.route.full_path()
    }

    /// View the `http::Version` of the request.
    pub fn version(&self) -> http::Version {
        self.route.version()
    }

    /// View the `http::StatusCode` of the response.
    pub fn status(&self) -> http::StatusCode {
        self.status
    }

    /// View the referer of the request.
    pub fn referer(&self) -> Option<&str> {
        self.route
            .headers()
            .get(header::REFERER)
            .and_then(|v| v.to_str().ok())
    }

    /// View the user agent of the request.
    pub fn user_agent(&self) -> Option<&str> {
        self.route
            .headers()
            .get(header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
    }

    /// View the `Duration` that elapsed for the request.
    pub fn elapsed(&self) -> Duration {
        tokio::time::Instant::now().into_std() - self.start
    }

    /// View the host of the request
    pub fn host(&self) -> Option<&str> {
        self.route
            .headers()
            .get(header::HOST)
            .and_then(|v| v.to_str().ok())
    }

    /// Access the full headers of the request
    pub fn request_headers(&self) -> &http::HeaderMap {
        self.route.headers()
    }
}

struct OptFmt<T>(Option<T>);

impl<T: fmt::Display> fmt::Display for OptFmt<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref t) = self.0 {
            fmt::Display::fmt(t, f)
        } else {
            f.write_str("-")
        }
    }
}

mod internal {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::Instant;

    use futures_util::{ready, TryFuture};
    use pin_project::pin_project;

    use super::{Info, Log};
    use crate::filter::{Filter, FilterBase, Internal};
    use crate::reject::IsReject;
    use crate::reply::{Reply, Response};
    use crate::route;

    #[allow(missing_debug_implementations)]
    pub struct Logged(pub(super) Response);

    impl Reply for Logged {
        #[inline]
        fn into_response(self) -> Response {
            self.0
        }
    }

    #[allow(missing_debug_implementations)]
    #[derive(Clone, Copy)]
    pub struct WithLog<FN, F> {
        pub(super) filter: F,
        pub(super) log: Log<FN>,
    }

    impl<FN, F> FilterBase for WithLog<FN, F>
    where
        FN: Fn(Info<'_>) + Clone + Send,
        F: Filter + Clone + Send,
        F::Extract: Reply,
        F::Error: IsReject,
    {
        type Extract = (Logged,);
        type Error = F::Error;
        type Future = WithLogFuture<FN, F::Future>;

        fn filter(&self, _: Internal) -> Self::Future {
            let started = tokio::time::Instant::now().into_std();
            WithLogFuture {
                log: self.log.clone(),
                future: self.filter.filter(Internal),
                started,
            }
        }
    }

    #[allow(missing_debug_implementations)]
    #[pin_project]
    pub struct WithLogFuture<FN, F> {
        log: Log<FN>,
        #[pin]
        future: F,
        started: Instant,
    }

    impl<FN, F> Future for WithLogFuture<FN, F>
    where
        FN: Fn(Info<'_>),
        F: TryFuture,
        F::Ok: Reply,
        F::Error: IsReject,
    {
        type Output = Result<(Logged,), F::Error>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let pin = self.as_mut().project();
            let (result, status) = match ready!(pin.future.try_poll(cx)) {
                Ok(reply) => {
                    let resp = reply.into_response();
                    let status = resp.status();
                    (Poll::Ready(Ok((Logged(resp),))), status)
                }
                Err(reject) => {
                    let status = reject.status();
                    (Poll::Ready(Err(reject)), status)
                }
            };

            route::with(|route| {
                (self.log.func)(Info {
                    route,
                    start: self.started,
                    status,
                });
            });

            result
        }
    }
}
