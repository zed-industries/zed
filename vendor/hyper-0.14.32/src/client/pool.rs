use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::marker::Unpin;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll};

#[cfg(not(feature = "runtime"))]
use std::time::{Duration, Instant};

use futures_channel::oneshot;
#[cfg(feature = "runtime")]
use tokio::time::{Duration, Instant, Interval};
use tracing::{debug, trace};

use super::client::Ver;
use crate::common::exec::Exec;

// FIXME: allow() required due to `impl Trait` leaking types to this lint
#[allow(missing_debug_implementations)]
pub(super) struct Pool<T> {
    // If the pool is disabled, this is None.
    inner: Option<Arc<Mutex<PoolInner<T>>>>,
}

// Before using a pooled connection, make sure the sender is not dead.
//
// This is a trait to allow the `client::pool::tests` to work for `i32`.
//
// See https://github.com/hyperium/hyper/issues/1429
pub(super) trait Poolable: Unpin + Send + Sized + 'static {
    fn is_open(&self) -> bool;
    /// Reserve this connection.
    ///
    /// Allows for HTTP/2 to return a shared reservation.
    fn reserve(self) -> Reservation<Self>;
    fn can_share(&self) -> bool;
}

/// When checking out a pooled connection, it might be that the connection
/// only supports a single reservation, or it might be usable for many.
///
/// Specifically, HTTP/1 requires a unique reservation, but HTTP/2 can be
/// used for multiple requests.
// FIXME: allow() required due to `impl Trait` leaking types to this lint
#[allow(missing_debug_implementations)]
pub(super) enum Reservation<T> {
    /// This connection could be used multiple times, the first one will be
    /// reinserted into the `idle` pool, and the second will be given to
    /// the `Checkout`.
    #[cfg(feature = "http2")]
    Shared(T, T),
    /// This connection requires unique access. It will be returned after
    /// use is complete.
    Unique(T),
}

/// Simple type alias in case the key type needs to be adjusted.
pub(super) type Key = (http::uri::Scheme, http::uri::Authority); //Arc<String>;

struct PoolInner<T> {
    // A flag that a connection is being established, and the connection
    // should be shared. This prevents making multiple HTTP/2 connections
    // to the same host.
    connecting: HashSet<Key>,
    // These are internal Conns sitting in the event loop in the KeepAlive
    // state, waiting to receive a new Request to send on the socket.
    idle: HashMap<Key, Vec<Idle<T>>>,
    max_idle_per_host: usize,
    // These are outstanding Checkouts that are waiting for a socket to be
    // able to send a Request one. This is used when "racing" for a new
    // connection.
    //
    // The Client starts 2 tasks, 1 to connect a new socket, and 1 to wait
    // for the Pool to receive an idle Conn. When a Conn becomes idle,
    // this list is checked for any parked Checkouts, and tries to notify
    // them that the Conn could be used instead of waiting for a brand new
    // connection.
    waiters: HashMap<Key, VecDeque<oneshot::Sender<T>>>,
    // A oneshot channel is used to allow the interval to be notified when
    // the Pool completely drops. That way, the interval can cancel immediately.
    #[cfg(feature = "runtime")]
    idle_interval_ref: Option<oneshot::Sender<std::convert::Infallible>>,
    #[cfg(feature = "runtime")]
    exec: Exec,
    timeout: Option<Duration>,
}

// This is because `Weak::new()` *allocates* space for `T`, even if it
// doesn't need it!
struct WeakOpt<T>(Option<Weak<T>>);

#[derive(Clone, Copy, Debug)]
pub(super) struct Config {
    pub(super) idle_timeout: Option<Duration>,
    pub(super) max_idle_per_host: usize,
}

impl Config {
    pub(super) fn is_enabled(&self) -> bool {
        self.max_idle_per_host > 0
    }
}

impl<T> Pool<T> {
    pub(super) fn new(config: Config, __exec: &Exec) -> Pool<T> {
        let inner = if config.is_enabled() {
            Some(Arc::new(Mutex::new(PoolInner {
                connecting: HashSet::new(),
                idle: HashMap::new(),
                #[cfg(feature = "runtime")]
                idle_interval_ref: None,
                max_idle_per_host: config.max_idle_per_host,
                waiters: HashMap::new(),
                #[cfg(feature = "runtime")]
                exec: __exec.clone(),
                timeout: config.idle_timeout.filter(|&t| t > Duration::ZERO),
            })))
        } else {
            None
        };

        Pool { inner }
    }

    fn is_enabled(&self) -> bool {
        self.inner.is_some()
    }

    #[cfg(test)]
    pub(super) fn no_timer(&self) {
        // Prevent an actual interval from being created for this pool...
        #[cfg(feature = "runtime")]
        {
            let mut inner = self.inner.as_ref().unwrap().lock().unwrap();
            assert!(inner.idle_interval_ref.is_none(), "timer already spawned");
            let (tx, _) = oneshot::channel();
            inner.idle_interval_ref = Some(tx);
        }
    }
}

impl<T: Poolable> Pool<T> {
    /// Returns a `Checkout` which is a future that resolves if an idle
    /// connection becomes available.
    pub(super) fn checkout(&self, key: Key) -> Checkout<T> {
        Checkout {
            key,
            pool: self.clone(),
            waiter: None,
        }
    }

    /// Ensure that there is only ever 1 connecting task for HTTP/2
    /// connections. This does nothing for HTTP/1.
    pub(super) fn connecting(&self, key: &Key, ver: Ver) -> Option<Connecting<T>> {
        if ver == Ver::Http2 {
            if let Some(ref enabled) = self.inner {
                let mut inner = enabled.lock().unwrap();
                return if inner.connecting.insert(key.clone()) {
                    let connecting = Connecting {
                        key: key.clone(),
                        pool: WeakOpt::downgrade(enabled),
                    };
                    Some(connecting)
                } else {
                    trace!("HTTP/2 connecting already in progress for {:?}", key);
                    None
                };
            }
        }

        // else
        Some(Connecting {
            key: key.clone(),
            // in HTTP/1's case, there is never a lock, so we don't
            // need to do anything in Drop.
            pool: WeakOpt::none(),
        })
    }

    #[cfg(test)]
    fn locked(&self) -> std::sync::MutexGuard<'_, PoolInner<T>> {
        self.inner.as_ref().expect("enabled").lock().expect("lock")
    }

    /* Used in client/tests.rs...
    #[cfg(feature = "runtime")]
    #[cfg(test)]
    pub(super) fn h1_key(&self, s: &str) -> Key {
        Arc::new(s.to_string())
    }

    #[cfg(feature = "runtime")]
    #[cfg(test)]
    pub(super) fn idle_count(&self, key: &Key) -> usize {
        self
            .locked()
            .idle
            .get(key)
            .map(|list| list.len())
            .unwrap_or(0)
    }
    */

    pub(super) fn pooled(
        &self,
        #[cfg_attr(not(feature = "http2"), allow(unused_mut))] mut connecting: Connecting<T>,
        value: T,
    ) -> Pooled<T> {
        let (value, pool_ref) = if let Some(ref enabled) = self.inner {
            match value.reserve() {
                #[cfg(feature = "http2")]
                Reservation::Shared(to_insert, to_return) => {
                    let mut inner = enabled.lock().unwrap();
                    inner.put(connecting.key.clone(), to_insert, enabled);
                    // Do this here instead of Drop for Connecting because we
                    // already have a lock, no need to lock the mutex twice.
                    inner.connected(&connecting.key);
                    // prevent the Drop of Connecting from repeating inner.connected()
                    connecting.pool = WeakOpt::none();

                    // Shared reservations don't need a reference to the pool,
                    // since the pool always keeps a copy.
                    (to_return, WeakOpt::none())
                }
                Reservation::Unique(value) => {
                    // Unique reservations must take a reference to the pool
                    // since they hope to reinsert once the reservation is
                    // completed
                    (value, WeakOpt::downgrade(enabled))
                }
            }
        } else {
            // If pool is not enabled, skip all the things...

            // The Connecting should have had no pool ref
            debug_assert!(connecting.pool.upgrade().is_none());

            (value, WeakOpt::none())
        };
        Pooled {
            key: connecting.key.clone(),
            is_reused: false,
            pool: pool_ref,
            value: Some(value),
        }
    }

    fn reuse(&self, key: &Key, value: T) -> Pooled<T> {
        debug!("reuse idle connection for {:?}", key);
        // TODO: unhack this
        // In Pool::pooled(), which is used for inserting brand new connections,
        // there's some code that adjusts the pool reference taken depending
        // on if the Reservation can be shared or is unique. By the time
        // reuse() is called, the reservation has already been made, and
        // we just have the final value, without knowledge of if this is
        // unique or shared. So, the hack is to just assume Ver::Http2 means
        // shared... :(
        let mut pool_ref = WeakOpt::none();
        if !value.can_share() {
            if let Some(ref enabled) = self.inner {
                pool_ref = WeakOpt::downgrade(enabled);
            }
        }

        Pooled {
            is_reused: true,
            key: key.clone(),
            pool: pool_ref,
            value: Some(value),
        }
    }
}

/// Pop off this list, looking for a usable connection that hasn't expired.
struct IdlePopper<'a, T> {
    key: &'a Key,
    list: &'a mut Vec<Idle<T>>,
}

impl<'a, T: Poolable + 'a> IdlePopper<'a, T> {
    fn pop(self, expiration: &Expiration) -> Option<Idle<T>> {
        while let Some(entry) = self.list.pop() {
            // If the connection has been closed, or is older than our idle
            // timeout, simply drop it and keep looking...
            if !entry.value.is_open() {
                trace!("removing closed connection for {:?}", self.key);
                continue;
            }
            // TODO: Actually, since the `idle` list is pushed to the end always,
            // that would imply that if *this* entry is expired, then anything
            // "earlier" in the list would *have* to be expired also... Right?
            //
            // In that case, we could just break out of the loop and drop the
            // whole list...
            if expiration.expires(entry.idle_at) {
                trace!("removing expired connection for {:?}", self.key);
                continue;
            }

            let value = match entry.value.reserve() {
                #[cfg(feature = "http2")]
                Reservation::Shared(to_reinsert, to_checkout) => {
                    self.list.push(Idle {
                        idle_at: Instant::now(),
                        value: to_reinsert,
                    });
                    to_checkout
                }
                Reservation::Unique(unique) => unique,
            };

            return Some(Idle {
                idle_at: entry.idle_at,
                value,
            });
        }

        None
    }
}

impl<T: Poolable> PoolInner<T> {
    fn put(&mut self, key: Key, value: T, __pool_ref: &Arc<Mutex<PoolInner<T>>>) {
        if value.can_share() && self.idle.contains_key(&key) {
            trace!("put; existing idle HTTP/2 connection for {:?}", key);
            return;
        }
        trace!("put; add idle connection for {:?}", key);
        let mut remove_waiters = false;
        let mut value = Some(value);
        if let Some(waiters) = self.waiters.get_mut(&key) {
            while let Some(tx) = waiters.pop_front() {
                if !tx.is_canceled() {
                    let reserved = value.take().expect("value already sent");
                    let reserved = match reserved.reserve() {
                        #[cfg(feature = "http2")]
                        Reservation::Shared(to_keep, to_send) => {
                            value = Some(to_keep);
                            to_send
                        }
                        Reservation::Unique(uniq) => uniq,
                    };
                    match tx.send(reserved) {
                        Ok(()) => {
                            if value.is_none() {
                                break;
                            } else {
                                continue;
                            }
                        }
                        Err(e) => {
                            value = Some(e);
                        }
                    }
                }

                trace!("put; removing canceled waiter for {:?}", key);
            }
            remove_waiters = waiters.is_empty();
        }
        if remove_waiters {
            self.waiters.remove(&key);
        }

        match value {
            Some(value) => {
                // borrow-check scope...
                {
                    let idle_list = self.idle.entry(key.clone()).or_insert_with(Vec::new);
                    if self.max_idle_per_host <= idle_list.len() {
                        trace!("max idle per host for {:?}, dropping connection", key);
                        return;
                    }

                    debug!("pooling idle connection for {:?}", key);
                    idle_list.push(Idle {
                        value,
                        idle_at: Instant::now(),
                    });
                }

                #[cfg(feature = "runtime")]
                {
                    self.spawn_idle_interval(__pool_ref);
                }
            }
            None => trace!("put; found waiter for {:?}", key),
        }
    }

    /// A `Connecting` task is complete. Not necessarily successfully,
    /// but the lock is going away, so clean up.
    fn connected(&mut self, key: &Key) {
        let existed = self.connecting.remove(key);
        debug_assert!(existed, "Connecting dropped, key not in pool.connecting");
        // cancel any waiters. if there are any, it's because
        // this Connecting task didn't complete successfully.
        // those waiters would never receive a connection.
        self.waiters.remove(key);
    }

    #[cfg(feature = "runtime")]
    fn spawn_idle_interval(&mut self, pool_ref: &Arc<Mutex<PoolInner<T>>>) {
        let (dur, rx) = {
            if self.idle_interval_ref.is_some() {
                return;
            }

            if let Some(dur) = self.timeout {
                let (tx, rx) = oneshot::channel();
                self.idle_interval_ref = Some(tx);
                (dur, rx)
            } else {
                return;
            }
        };

        let interval = IdleTask {
            interval: tokio::time::interval(dur),
            pool: WeakOpt::downgrade(pool_ref),
            pool_drop_notifier: rx,
        };

        self.exec.execute(interval);
    }
}

impl<T> PoolInner<T> {
    /// Any `FutureResponse`s that were created will have made a `Checkout`,
    /// and possibly inserted into the pool that it is waiting for an idle
    /// connection. If a user ever dropped that future, we need to clean out
    /// those parked senders.
    fn clean_waiters(&mut self, key: &Key) {
        let mut remove_waiters = false;
        if let Some(waiters) = self.waiters.get_mut(key) {
            waiters.retain(|tx| !tx.is_canceled());
            remove_waiters = waiters.is_empty();
        }
        if remove_waiters {
            self.waiters.remove(key);
        }
    }
}

#[cfg(feature = "runtime")]
impl<T: Poolable> PoolInner<T> {
    /// This should *only* be called by the IdleTask
    fn clear_expired(&mut self) {
        let dur = self.timeout.expect("interval assumes timeout");

        let now = Instant::now();
        //self.last_idle_check_at = now;

        self.idle.retain(|key, values| {
            values.retain(|entry| {
                if !entry.value.is_open() {
                    trace!("idle interval evicting closed for {:?}", key);
                    return false;
                }

                // Avoid `Instant::sub` to avoid issues like rust-lang/rust#86470.
                if now.saturating_duration_since(entry.idle_at) > dur {
                    trace!("idle interval evicting expired for {:?}", key);
                    return false;
                }

                // Otherwise, keep this value...
                true
            });

            // returning false evicts this key/val
            !values.is_empty()
        });
    }
}

impl<T> Clone for Pool<T> {
    fn clone(&self) -> Pool<T> {
        Pool {
            inner: self.inner.clone(),
        }
    }
}

/// A wrapped poolable value that tries to reinsert to the Pool on Drop.
// Note: The bounds `T: Poolable` is needed for the Drop impl.
pub(super) struct Pooled<T: Poolable> {
    value: Option<T>,
    is_reused: bool,
    key: Key,
    pool: WeakOpt<Mutex<PoolInner<T>>>,
}

impl<T: Poolable> Pooled<T> {
    pub(super) fn is_reused(&self) -> bool {
        self.is_reused
    }

    pub(super) fn is_pool_enabled(&self) -> bool {
        self.pool.0.is_some()
    }

    fn as_ref(&self) -> &T {
        self.value.as_ref().expect("not dropped")
    }

    fn as_mut(&mut self) -> &mut T {
        self.value.as_mut().expect("not dropped")
    }
}

impl<T: Poolable> Deref for Pooled<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.as_ref()
    }
}

impl<T: Poolable> DerefMut for Pooled<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T: Poolable> Drop for Pooled<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            if !value.is_open() {
                // If we *already* know the connection is done here,
                // it shouldn't be re-inserted back into the pool.
                return;
            }

            if let Some(pool) = self.pool.upgrade() {
                if let Ok(mut inner) = pool.lock() {
                    inner.put(self.key.clone(), value, &pool);
                }
            } else if !value.can_share() {
                trace!("pool dropped, dropping pooled ({:?})", self.key);
            }
            // Ver::Http2 is already in the Pool (or dead), so we wouldn't
            // have an actual reference to the Pool.
        }
    }
}

impl<T: Poolable> fmt::Debug for Pooled<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pooled").field("key", &self.key).finish()
    }
}

struct Idle<T> {
    idle_at: Instant,
    value: T,
}

// FIXME: allow() required due to `impl Trait` leaking types to this lint
#[allow(missing_debug_implementations)]
pub(super) struct Checkout<T> {
    key: Key,
    pool: Pool<T>,
    waiter: Option<oneshot::Receiver<T>>,
}

#[derive(Debug)]
pub(super) struct CheckoutIsClosedError;

impl StdError for CheckoutIsClosedError {}

impl fmt::Display for CheckoutIsClosedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("checked out connection was closed")
    }
}

impl<T: Poolable> Checkout<T> {
    fn poll_waiter(&mut self, cx: &mut Context<'_>) -> Poll<Option<crate::Result<Pooled<T>>>> {
        if let Some(mut rx) = self.waiter.take() {
            match Pin::new(&mut rx).poll(cx) {
                Poll::Ready(Ok(value)) => {
                    if value.is_open() {
                        Poll::Ready(Some(Ok(self.pool.reuse(&self.key, value))))
                    } else {
                        Poll::Ready(Some(Err(
                            crate::Error::new_canceled().with(CheckoutIsClosedError)
                        )))
                    }
                }
                Poll::Pending => {
                    self.waiter = Some(rx);
                    Poll::Pending
                }
                Poll::Ready(Err(_canceled)) => Poll::Ready(Some(Err(
                    crate::Error::new_canceled().with("request has been canceled")
                ))),
            }
        } else {
            Poll::Ready(None)
        }
    }

    fn checkout(&mut self, cx: &mut Context<'_>) -> Option<Pooled<T>> {
        let entry = {
            let mut inner = self.pool.inner.as_ref()?.lock().unwrap();
            let expiration = Expiration::new(inner.timeout);
            let maybe_entry = inner.idle.get_mut(&self.key).and_then(|list| {
                trace!("take? {:?}: expiration = {:?}", self.key, expiration.0);
                // A block to end the mutable borrow on list,
                // so the map below can check is_empty()
                {
                    let popper = IdlePopper {
                        key: &self.key,
                        list,
                    };
                    popper.pop(&expiration)
                }
                .map(|e| (e, list.is_empty()))
            });

            let (entry, empty) = if let Some((e, empty)) = maybe_entry {
                (Some(e), empty)
            } else {
                // No entry found means nuke the list for sure.
                (None, true)
            };
            if empty {
                //TODO: This could be done with the HashMap::entry API instead.
                inner.idle.remove(&self.key);
            }

            if entry.is_none() && self.waiter.is_none() {
                let (tx, mut rx) = oneshot::channel();
                trace!("checkout waiting for idle connection: {:?}", self.key);
                inner
                    .waiters
                    .entry(self.key.clone())
                    .or_insert_with(VecDeque::new)
                    .push_back(tx);

                // register the waker with this oneshot
                assert!(Pin::new(&mut rx).poll(cx).is_pending());
                self.waiter = Some(rx);
            }

            entry
        };

        entry.map(|e| self.pool.reuse(&self.key, e.value))
    }
}

impl<T: Poolable> Future for Checkout<T> {
    type Output = crate::Result<Pooled<T>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(pooled) = ready!(self.poll_waiter(cx)?) {
            return Poll::Ready(Ok(pooled));
        }

        if let Some(pooled) = self.checkout(cx) {
            Poll::Ready(Ok(pooled))
        } else if !self.pool.is_enabled() {
            Poll::Ready(Err(crate::Error::new_canceled().with("pool is disabled")))
        } else {
            // There's a new waiter, already registered in self.checkout()
            debug_assert!(self.waiter.is_some());
            Poll::Pending
        }
    }
}

impl<T> Drop for Checkout<T> {
    fn drop(&mut self) {
        if self.waiter.take().is_some() {
            trace!("checkout dropped for {:?}", self.key);
            if let Some(Ok(mut inner)) = self.pool.inner.as_ref().map(|i| i.lock()) {
                inner.clean_waiters(&self.key);
            }
        }
    }
}

// FIXME: allow() required due to `impl Trait` leaking types to this lint
#[allow(missing_debug_implementations)]
pub(super) struct Connecting<T: Poolable> {
    key: Key,
    pool: WeakOpt<Mutex<PoolInner<T>>>,
}

impl<T: Poolable> Connecting<T> {
    pub(super) fn alpn_h2(self, pool: &Pool<T>) -> Option<Self> {
        debug_assert!(
            self.pool.0.is_none(),
            "Connecting::alpn_h2 but already Http2"
        );

        pool.connecting(&self.key, Ver::Http2)
    }
}

impl<T: Poolable> Drop for Connecting<T> {
    fn drop(&mut self) {
        if let Some(pool) = self.pool.upgrade() {
            // No need to panic on drop, that could abort!
            if let Ok(mut inner) = pool.lock() {
                inner.connected(&self.key);
            }
        }
    }
}

struct Expiration(Option<Duration>);

impl Expiration {
    fn new(dur: Option<Duration>) -> Expiration {
        Expiration(dur)
    }

    fn expires(&self, instant: Instant) -> bool {
        match self.0 {
            // Avoid `Instant::elapsed` to avoid issues like rust-lang/rust#86470.
            Some(timeout) => Instant::now().saturating_duration_since(instant) > timeout,
            None => false,
        }
    }
}

#[cfg(feature = "runtime")]
pin_project_lite::pin_project! {
    struct IdleTask<T> {
        #[pin]
        interval: Interval,
        pool: WeakOpt<Mutex<PoolInner<T>>>,
        // This allows the IdleTask to be notified as soon as the entire
        // Pool is fully dropped, and shutdown. This channel is never sent on,
        // but Err(Canceled) will be received when the Pool is dropped.
        #[pin]
        pool_drop_notifier: oneshot::Receiver<std::convert::Infallible>,
    }
}

#[cfg(feature = "runtime")]
impl<T: Poolable + 'static> Future for IdleTask<T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            match this.pool_drop_notifier.as_mut().poll(cx) {
                Poll::Ready(Ok(n)) => match n {},
                Poll::Pending => (),
                Poll::Ready(Err(_canceled)) => {
                    trace!("pool closed, canceling idle interval");
                    return Poll::Ready(());
                }
            }

            ready!(this.interval.as_mut().poll_tick(cx));

            if let Some(inner) = this.pool.upgrade() {
                if let Ok(mut inner) = inner.lock() {
                    trace!("idle interval checking for expired");
                    inner.clear_expired();
                    continue;
                }
            }
            return Poll::Ready(());
        }
    }
}

impl<T> WeakOpt<T> {
    fn none() -> Self {
        WeakOpt(None)
    }

    fn downgrade(arc: &Arc<T>) -> Self {
        WeakOpt(Some(Arc::downgrade(arc)))
    }

    fn upgrade(&self) -> Option<Arc<T>> {
        self.0.as_ref().and_then(Weak::upgrade)
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::Context;
    use std::task::Poll;
    use std::time::Duration;

    use super::{Connecting, Key, Pool, Poolable, Reservation, WeakOpt};
    use crate::common::exec::Exec;

    /// Test unique reservations.
    #[derive(Debug, PartialEq, Eq)]
    struct Uniq<T>(T);

    impl<T: Send + 'static + Unpin> Poolable for Uniq<T> {
        fn is_open(&self) -> bool {
            true
        }

        fn reserve(self) -> Reservation<Self> {
            Reservation::Unique(self)
        }

        fn can_share(&self) -> bool {
            false
        }
    }

    fn c<T: Poolable>(key: Key) -> Connecting<T> {
        Connecting {
            key,
            pool: WeakOpt::none(),
        }
    }

    fn host_key(s: &str) -> Key {
        (http::uri::Scheme::HTTP, s.parse().expect("host key"))
    }

    fn pool_no_timer<T>() -> Pool<T> {
        pool_max_idle_no_timer(::std::usize::MAX)
    }

    fn pool_max_idle_no_timer<T>(max_idle: usize) -> Pool<T> {
        let pool = Pool::new(
            super::Config {
                idle_timeout: Some(Duration::from_millis(100)),
                max_idle_per_host: max_idle,
            },
            &Exec::Default,
        );
        pool.no_timer();
        pool
    }

    #[tokio::test]
    async fn test_pool_checkout_smoke() {
        let pool = pool_no_timer();
        let key = host_key("foo");
        let pooled = pool.pooled(c(key.clone()), Uniq(41));

        drop(pooled);

        match pool.checkout(key).await {
            Ok(pooled) => assert_eq!(*pooled, Uniq(41)),
            Err(_) => panic!("not ready"),
        };
    }

    /// Helper to check if the future is ready after polling once.
    struct PollOnce<'a, F>(&'a mut F);

    impl<F, T, U> Future for PollOnce<'_, F>
    where
        F: Future<Output = Result<T, U>> + Unpin,
    {
        type Output = Option<()>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match Pin::new(&mut self.0).poll(cx) {
                Poll::Ready(Ok(_)) => Poll::Ready(Some(())),
                Poll::Ready(Err(_)) => Poll::Ready(Some(())),
                Poll::Pending => Poll::Ready(None),
            }
        }
    }

    #[tokio::test]
    async fn test_pool_checkout_returns_none_if_expired() {
        let pool = pool_no_timer();
        let key = host_key("foo");
        let pooled = pool.pooled(c(key.clone()), Uniq(41));

        drop(pooled);
        tokio::time::sleep(pool.locked().timeout.unwrap()).await;
        let mut checkout = pool.checkout(key);
        let poll_once = PollOnce(&mut checkout);
        let is_not_ready = poll_once.await.is_none();
        assert!(is_not_ready);
    }

    #[cfg(feature = "runtime")]
    #[tokio::test]
    async fn test_pool_checkout_removes_expired() {
        let pool = pool_no_timer();
        let key = host_key("foo");

        pool.pooled(c(key.clone()), Uniq(41));
        pool.pooled(c(key.clone()), Uniq(5));
        pool.pooled(c(key.clone()), Uniq(99));

        assert_eq!(
            pool.locked().idle.get(&key).map(|entries| entries.len()),
            Some(3)
        );
        tokio::time::sleep(pool.locked().timeout.unwrap()).await;

        let mut checkout = pool.checkout(key.clone());
        let poll_once = PollOnce(&mut checkout);
        // checkout.await should clean out the expired
        poll_once.await;
        assert!(pool.locked().idle.get(&key).is_none());
    }

    #[test]
    fn test_pool_max_idle_per_host() {
        let pool = pool_max_idle_no_timer(2);
        let key = host_key("foo");

        pool.pooled(c(key.clone()), Uniq(41));
        pool.pooled(c(key.clone()), Uniq(5));
        pool.pooled(c(key.clone()), Uniq(99));

        // pooled and dropped 3, max_idle should only allow 2
        assert_eq!(
            pool.locked().idle.get(&key).map(|entries| entries.len()),
            Some(2)
        );
    }

    #[cfg(feature = "runtime")]
    #[tokio::test]
    async fn test_pool_timer_removes_expired() {
        let _ = pretty_env_logger::try_init();
        tokio::time::pause();

        let pool = Pool::new(
            super::Config {
                idle_timeout: Some(Duration::from_millis(10)),
                max_idle_per_host: std::usize::MAX,
            },
            &Exec::Default,
        );

        let key = host_key("foo");

        pool.pooled(c(key.clone()), Uniq(41));
        pool.pooled(c(key.clone()), Uniq(5));
        pool.pooled(c(key.clone()), Uniq(99));

        assert_eq!(
            pool.locked().idle.get(&key).map(|entries| entries.len()),
            Some(3)
        );

        // Let the timer tick passed the expiration...
        tokio::time::advance(Duration::from_millis(30)).await;
        // Yield so the Interval can reap...
        tokio::task::yield_now().await;

        assert!(pool.locked().idle.get(&key).is_none());
    }

    #[tokio::test]
    async fn test_pool_checkout_task_unparked() {
        use futures_util::future::join;
        use futures_util::FutureExt;

        let pool = pool_no_timer();
        let key = host_key("foo");
        let pooled = pool.pooled(c(key.clone()), Uniq(41));

        let checkout = join(pool.checkout(key), async {
            // the checkout future will park first,
            // and then this lazy future will be polled, which will insert
            // the pooled back into the pool
            //
            // this test makes sure that doing so will unpark the checkout
            drop(pooled);
        })
        .map(|(entry, _)| entry);

        assert_eq!(*checkout.await.unwrap(), Uniq(41));
    }

    #[tokio::test]
    async fn test_pool_checkout_drop_cleans_up_waiters() {
        let pool = pool_no_timer::<Uniq<i32>>();
        let key = host_key("foo");

        let mut checkout1 = pool.checkout(key.clone());
        let mut checkout2 = pool.checkout(key.clone());

        let poll_once1 = PollOnce(&mut checkout1);
        let poll_once2 = PollOnce(&mut checkout2);

        // first poll needed to get into Pool's parked
        poll_once1.await;
        assert_eq!(pool.locked().waiters.get(&key).unwrap().len(), 1);
        poll_once2.await;
        assert_eq!(pool.locked().waiters.get(&key).unwrap().len(), 2);

        // on drop, clean up Pool
        drop(checkout1);
        assert_eq!(pool.locked().waiters.get(&key).unwrap().len(), 1);

        drop(checkout2);
        assert!(pool.locked().waiters.get(&key).is_none());
    }

    #[derive(Debug)]
    struct CanClose {
        #[allow(unused)]
        val: i32,
        closed: bool,
    }

    impl Poolable for CanClose {
        fn is_open(&self) -> bool {
            !self.closed
        }

        fn reserve(self) -> Reservation<Self> {
            Reservation::Unique(self)
        }

        fn can_share(&self) -> bool {
            false
        }
    }

    #[test]
    fn pooled_drop_if_closed_doesnt_reinsert() {
        let pool = pool_no_timer();
        let key = host_key("foo");
        pool.pooled(
            c(key.clone()),
            CanClose {
                val: 57,
                closed: true,
            },
        );

        assert!(!pool.locked().idle.contains_key(&key));
    }
}
