use std::future::Future;

/// Convert a type into a `Future`.
///
/// # Examples
///
/// ```
/// use async_std::future::{Future, IntoFuture};
/// use async_std::io;
/// use async_std::pin::Pin;
///
/// struct Client;
///
/// impl Client {
///     pub async fn send(self) -> io::Result<()> {
///         // Send a request
///         Ok(())
///     }
/// }
///
/// impl IntoFuture for Client {
///     type Output = io::Result<()>;
///
///     type Future = Pin<Box<dyn Future<Output = Self::Output>>>;
///
///     fn into_future(self) -> Self::Future {
///         Box::pin(async {
///             self.send().await
///         })
///     }
/// }
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
pub trait IntoFuture {
    /// The type of value produced on completion.
    type Output;

    /// Which kind of future are we turning this into?
    type Future: Future<Output = Self::Output>;

    /// Create a future from a value
    fn into_future(self) -> Self::Future;
}

impl<T: Future> IntoFuture for T {
    type Output = T::Output;
    type Future = T;

    fn into_future(self) -> Self::Future {
        self
    }
}
