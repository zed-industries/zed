macro_rules! ready {
    ($e:expr) => {
        match $e {
            std::task::Poll::Ready(v) => v,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}

pub(crate) mod buf;
#[cfg(all(feature = "server", any(feature = "http1", feature = "http2")))]
pub(crate) mod date;
#[cfg(all(feature = "server", any(feature = "http1", feature = "http2")))]
pub(crate) mod drain;
#[cfg(any(feature = "http1", feature = "http2", feature = "server"))]
pub(crate) mod exec;
pub(crate) mod io;
#[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
mod lazy;
#[cfg(any(
    feature = "stream",
    all(feature = "client", any(feature = "http1", feature = "http2"))
))]
pub(crate) mod sync_wrapper;
#[cfg(feature = "http1")]
pub(crate) mod task;
pub(crate) mod watch;

#[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
pub(crate) use self::lazy::{lazy, Started as Lazy};
