//! Provides a thread-safe, concurrent asynchronous (futures aware) cache
//! implementation.
//!
//! To use this module, enable a crate feature called "future".

use crossbeam_channel::Sender;
use futures_util::future::{BoxFuture, Shared};
use std::{future::Future, hash::Hash, sync::Arc};

use crate::common::{concurrent::WriteOp, time::Instant};

mod base_cache;
mod builder;
mod cache;
mod entry_selector;
mod housekeeper;
mod invalidator;
mod key_lock;
mod notifier;
mod value_initializer;

pub use {
    builder::CacheBuilder,
    cache::Cache,
    entry_selector::{OwnedKeyEntrySelector, RefKeyEntrySelector},
};

/// The type of the unique ID to identify a predicate used by
/// [`Cache::invalidate_entries_if`][invalidate-if] method.
///
/// A `PredicateId` is a `String` of UUID (version 4).
///
/// [invalidate-if]: ./struct.Cache.html#method.invalidate_entries_if
pub type PredicateId = String;

pub(crate) type PredicateIdStr<'a> = &'a str;

// Empty struct to be used in `InitResult::InitErr` to represent the Option None.
pub(crate) struct OptionallyNone;

// Empty struct to be used in `InitResult::InitErr` to represent the Compute None.
pub(crate) struct ComputeNone;

impl<T: ?Sized> FutureExt for T where T: Future {}

pub trait FutureExt: Future {
    fn boxed<'a, T>(self) -> BoxFuture<'a, T>
    where
        Self: Future<Output = T> + Sized + Send + 'a,
    {
        Box::pin(self)
    }
}

/// Iterator visiting all key-value pairs in a cache in arbitrary order.
///
/// Call [`Cache::iter`](./struct.Cache.html#method.iter) method to obtain an `Iter`.
pub struct Iter<'i, K, V>(crate::common::iter::Iter<'i, K, V>);

impl<'i, K, V> Iter<'i, K, V> {
    pub(crate) fn new(inner: crate::common::iter::Iter<'i, K, V>) -> Self {
        Self(inner)
    }
}

impl<K, V> Iterator for Iter<'_, K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    type Item = (Arc<K>, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Operation that has been interrupted (stopped polling) by async cancellation.
pub(crate) enum InterruptedOp<K, V> {
    CallEvictionListener {
        ts: Instant,
        // 'static means that the future can capture only owned value and/or static
        // references. No non-static references are allowed.
        future: Shared<BoxFuture<'static, ()>>,
        op: WriteOp<K, V>,
    },
    SendWriteOp {
        ts: Instant,
        op: WriteOp<K, V>,
    },
}

/// Drop guard for an async task being performed. If this guard is dropped while it
/// is still having the shared `future` or the write `op`, it will convert them to an
/// `InterruptedOp` and send it to the interrupted operations channel. Later, the
/// interrupted op will be retried by `retry_interrupted_ops` method of
/// `BaseCache`.
struct CancelGuard<'a, K, V> {
    interrupted_op_ch: &'a Sender<InterruptedOp<K, V>>,
    ts: Instant,
    future: Option<Shared<BoxFuture<'static, ()>>>,
    op: Option<WriteOp<K, V>>,
}

impl<'a, K, V> CancelGuard<'a, K, V> {
    fn new(interrupted_op_ch: &'a Sender<InterruptedOp<K, V>>, ts: Instant) -> Self {
        Self {
            interrupted_op_ch,
            ts,
            future: None,
            op: None,
        }
    }

    fn set_future_and_op(&mut self, future: Shared<BoxFuture<'static, ()>>, op: WriteOp<K, V>) {
        self.future = Some(future);
        self.op = Some(op);
    }

    fn set_op(&mut self, op: WriteOp<K, V>) {
        self.op = Some(op);
    }

    fn unset_future(&mut self) {
        self.future = None;
    }

    fn clear(&mut self) {
        self.future = None;
        self.op = None;
    }
}

impl<K, V> Drop for CancelGuard<'_, K, V> {
    fn drop(&mut self) {
        let interrupted_op = match (self.future.take(), self.op.take()) {
            (Some(future), Some(op)) => InterruptedOp::CallEvictionListener {
                ts: self.ts,
                future,
                op,
            },
            (None, Some(op)) => InterruptedOp::SendWriteOp { ts: self.ts, op },
            _ => return,
        };

        self.interrupted_op_ch
            .send(interrupted_op)
            .expect("Failed to send a pending op");
    }
}
