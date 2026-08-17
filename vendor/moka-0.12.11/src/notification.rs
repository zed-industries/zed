//! Common data types for notifications.

#[cfg(feature = "sync")]
pub(crate) mod notifier;

use std::{future::Future, pin::Pin, sync::Arc};

/// A future returned by an eviction listener.
///
/// You can use the [`boxed` method][boxed-method] of `FutureExt` trait to convert a
/// regular `Future` object into `ListenerFuture`.
///
/// [boxed-method]: ../future/trait.FutureExt.html#method.boxed
pub type ListenerFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

#[cfg(feature = "sync")]
pub(crate) type EvictionListener<K, V> =
    Arc<dyn Fn(Arc<K>, V, RemovalCause) + Send + Sync + 'static>;

#[cfg(feature = "future")]
pub(crate) type AsyncEvictionListener<K, V> =
    Box<dyn Fn(Arc<K>, V, RemovalCause) -> ListenerFuture + Send + Sync + 'static>;

// NOTE: Currently, dropping the cache will drop all entries without sending
// notifications. Calling `invalidate_all` method of the cache will trigger
// the notifications, but currently there is no way to know when all entries
// have been invalidated and their notifications have been sent.

/// Indicates the reason why a cached entry was removed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemovalCause {
    /// The entry's expiration timestamp has passed.
    Expired,
    /// The entry was manually removed by the user.
    Explicit,
    /// The entry itself was not actually removed, but its value was replaced by
    /// the user.
    Replaced,
    /// The entry was evicted due to size constraints.
    Size,
}

impl RemovalCause {
    pub fn was_evicted(&self) -> bool {
        matches!(self, Self::Expired | Self::Size)
    }
}
