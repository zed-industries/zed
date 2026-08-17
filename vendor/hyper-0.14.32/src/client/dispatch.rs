#[cfg(feature = "http2")]
use std::future::Future;
use std::marker::Unpin;
#[cfg(feature = "http2")]
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::FutureExt;
use tokio::sync::{mpsc, oneshot};

pub(crate) type RetryPromise<T, U> = oneshot::Receiver<Result<U, (crate::Error, Option<T>)>>;
pub(crate) type Promise<T> = oneshot::Receiver<Result<T, crate::Error>>;

pub(crate) fn channel<T, U>() -> (Sender<T, U>, Receiver<T, U>) {
    let (tx, rx) = mpsc::unbounded_channel();
    let (giver, taker) = want::new();
    let tx = Sender {
        buffered_once: false,
        giver,
        inner: tx,
    };
    let rx = Receiver { inner: rx, taker };
    (tx, rx)
}

/// A bounded sender of requests and callbacks for when responses are ready.
///
/// While the inner sender is unbounded, the Giver is used to determine
/// if the Receiver is ready for another request.
pub(crate) struct Sender<T, U> {
    /// One message is always allowed, even if the Receiver hasn't asked
    /// for it yet. This boolean keeps track of whether we've sent one
    /// without notice.
    buffered_once: bool,
    /// The Giver helps watch that the the Receiver side has been polled
    /// when the queue is empty. This helps us know when a request and
    /// response have been fully processed, and a connection is ready
    /// for more.
    giver: want::Giver,
    /// Actually bounded by the Giver, plus `buffered_once`.
    inner: mpsc::UnboundedSender<Envelope<T, U>>,
}

/// An unbounded version.
///
/// Cannot poll the Giver, but can still use it to determine if the Receiver
/// has been dropped. However, this version can be cloned.
#[cfg(feature = "http2")]
pub(crate) struct UnboundedSender<T, U> {
    /// Only used for `is_closed`, since mpsc::UnboundedSender cannot be checked.
    giver: want::SharedGiver,
    inner: mpsc::UnboundedSender<Envelope<T, U>>,
}

impl<T, U> Sender<T, U> {
    pub(crate) fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<()>> {
        self.giver
            .poll_want(cx)
            .map_err(|_| crate::Error::new_closed())
    }

    pub(crate) fn is_ready(&self) -> bool {
        self.giver.is_wanting()
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.giver.is_canceled()
    }

    fn can_send(&mut self) -> bool {
        if self.giver.give() || !self.buffered_once {
            // If the receiver is ready *now*, then of course we can send.
            //
            // If the receiver isn't ready yet, but we don't have anything
            // in the channel yet, then allow one message.
            self.buffered_once = true;
            true
        } else {
            false
        }
    }

    pub(crate) fn try_send(&mut self, val: T) -> Result<RetryPromise<T, U>, T> {
        if !self.can_send() {
            return Err(val);
        }
        let (tx, rx) = oneshot::channel();
        self.inner
            .send(Envelope(Some((val, Callback::Retry(Some(tx))))))
            .map(move |_| rx)
            .map_err(|mut e| (e.0).0.take().expect("envelope not dropped").0)
    }

    pub(crate) fn send(&mut self, val: T) -> Result<Promise<U>, T> {
        if !self.can_send() {
            return Err(val);
        }
        let (tx, rx) = oneshot::channel();
        self.inner
            .send(Envelope(Some((val, Callback::NoRetry(Some(tx))))))
            .map(move |_| rx)
            .map_err(|mut e| (e.0).0.take().expect("envelope not dropped").0)
    }

    #[cfg(feature = "http2")]
    pub(crate) fn unbound(self) -> UnboundedSender<T, U> {
        UnboundedSender {
            giver: self.giver.shared(),
            inner: self.inner,
        }
    }
}

#[cfg(feature = "http2")]
impl<T, U> UnboundedSender<T, U> {
    pub(crate) fn is_ready(&self) -> bool {
        !self.giver.is_canceled()
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.giver.is_canceled()
    }

    pub(crate) fn try_send(&mut self, val: T) -> Result<RetryPromise<T, U>, T> {
        let (tx, rx) = oneshot::channel();
        self.inner
            .send(Envelope(Some((val, Callback::Retry(Some(tx))))))
            .map(move |_| rx)
            .map_err(|mut e| (e.0).0.take().expect("envelope not dropped").0)
    }

    #[cfg(all(feature = "backports", feature = "http2"))]
    pub(crate) fn send(&mut self, val: T) -> Result<Promise<U>, T> {
        let (tx, rx) = oneshot::channel();
        self.inner
            .send(Envelope(Some((val, Callback::NoRetry(Some(tx))))))
            .map(move |_| rx)
            .map_err(|mut e| (e.0).0.take().expect("envelope not dropped").0)
    }
}

#[cfg(feature = "http2")]
impl<T, U> Clone for UnboundedSender<T, U> {
    fn clone(&self) -> Self {
        UnboundedSender {
            giver: self.giver.clone(),
            inner: self.inner.clone(),
        }
    }
}

pub(crate) struct Receiver<T, U> {
    inner: mpsc::UnboundedReceiver<Envelope<T, U>>,
    taker: want::Taker,
}

impl<T, U> Receiver<T, U> {
    pub(crate) fn poll_recv(&mut self, cx: &mut Context<'_>) -> Poll<Option<(T, Callback<T, U>)>> {
        match self.inner.poll_recv(cx) {
            Poll::Ready(item) => {
                Poll::Ready(item.map(|mut env| env.0.take().expect("envelope not dropped")))
            }
            Poll::Pending => {
                self.taker.want();
                Poll::Pending
            }
        }
    }

    #[cfg(feature = "http1")]
    pub(crate) fn close(&mut self) {
        self.taker.cancel();
        self.inner.close();
    }

    #[cfg(feature = "http1")]
    pub(crate) fn try_recv(&mut self) -> Option<(T, Callback<T, U>)> {
        match self.inner.recv().now_or_never() {
            Some(Some(mut env)) => env.0.take(),
            _ => None,
        }
    }
}

impl<T, U> Drop for Receiver<T, U> {
    fn drop(&mut self) {
        // Notify the giver about the closure first, before dropping
        // the mpsc::Receiver.
        self.taker.cancel();
    }
}

struct Envelope<T, U>(Option<(T, Callback<T, U>)>);

impl<T, U> Drop for Envelope<T, U> {
    fn drop(&mut self) {
        if let Some((val, cb)) = self.0.take() {
            cb.send(Err((
                crate::Error::new_canceled().with("connection closed"),
                Some(val),
            )));
        }
    }
}

pub(crate) enum Callback<T, U> {
    Retry(Option<oneshot::Sender<Result<U, (crate::Error, Option<T>)>>>),
    NoRetry(Option<oneshot::Sender<Result<U, crate::Error>>>),
}

impl<T, U> Drop for Callback<T, U> {
    fn drop(&mut self) {
        // FIXME(nox): What errors do we want here?
        let error = crate::Error::new_user_dispatch_gone().with(if std::thread::panicking() {
            "user code panicked"
        } else {
            "runtime dropped the dispatch task"
        });

        match self {
            Callback::Retry(tx) => {
                if let Some(tx) = tx.take() {
                    let _ = tx.send(Err((error, None)));
                }
            }
            Callback::NoRetry(tx) => {
                if let Some(tx) = tx.take() {
                    let _ = tx.send(Err(error));
                }
            }
        }
    }
}

impl<T, U> Callback<T, U> {
    #[cfg(feature = "http2")]
    pub(crate) fn is_canceled(&self) -> bool {
        match *self {
            Callback::Retry(Some(ref tx)) => tx.is_closed(),
            Callback::NoRetry(Some(ref tx)) => tx.is_closed(),
            _ => unreachable!(),
        }
    }

    pub(crate) fn poll_canceled(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        match *self {
            Callback::Retry(Some(ref mut tx)) => tx.poll_closed(cx),
            Callback::NoRetry(Some(ref mut tx)) => tx.poll_closed(cx),
            _ => unreachable!(),
        }
    }

    pub(crate) fn send(mut self, val: Result<U, (crate::Error, Option<T>)>) {
        match self {
            Callback::Retry(ref mut tx) => {
                let _ = tx.take().unwrap().send(val);
            }
            Callback::NoRetry(ref mut tx) => {
                let _ = tx.take().unwrap().send(val.map_err(|e| e.0));
            }
        }
    }

    #[cfg(feature = "http2")]
    pub(crate) async fn send_when(
        self,
        mut when: impl Future<Output = Result<U, (crate::Error, Option<T>)>> + Unpin,
    ) {
        use futures_util::future;
        use tracing::trace;

        let mut cb = Some(self);

        // "select" on this callback being canceled, and the future completing
        future::poll_fn(move |cx| {
            match Pin::new(&mut when).poll(cx) {
                Poll::Ready(Ok(res)) => {
                    cb.take().expect("polled after complete").send(Ok(res));
                    Poll::Ready(())
                }
                Poll::Pending => {
                    // check if the callback is canceled
                    ready!(cb.as_mut().unwrap().poll_canceled(cx));
                    trace!("send_when canceled");
                    Poll::Ready(())
                }
                Poll::Ready(Err(err)) => {
                    cb.take().expect("polled after complete").send(Err(err));
                    Poll::Ready(())
                }
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "nightly")]
    extern crate test;

    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use super::{channel, Callback, Receiver};

    #[derive(Debug)]
    struct Custom(i32);

    impl<T, U> Future for Receiver<T, U> {
        type Output = Option<(T, Callback<T, U>)>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.poll_recv(cx)
        }
    }

    /// Helper to check if the future is ready after polling once.
    struct PollOnce<'a, F>(&'a mut F);

    impl<F, T> Future for PollOnce<'_, F>
    where
        F: Future<Output = T> + Unpin,
    {
        type Output = Option<()>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match Pin::new(&mut self.0).poll(cx) {
                Poll::Ready(_) => Poll::Ready(Some(())),
                Poll::Pending => Poll::Ready(None),
            }
        }
    }

    #[tokio::test]
    async fn drop_receiver_sends_cancel_errors() {
        let _ = pretty_env_logger::try_init();

        let (mut tx, mut rx) = channel::<Custom, ()>();

        // must poll once for try_send to succeed
        assert!(PollOnce(&mut rx).await.is_none(), "rx empty");

        let promise = tx.try_send(Custom(43)).unwrap();
        drop(rx);

        let fulfilled = promise.await;
        let err = fulfilled
            .expect("fulfilled")
            .expect_err("promise should error");
        match (err.0.kind(), err.1) {
            (&crate::error::Kind::Canceled, Some(_)) => (),
            e => panic!("expected Error::Cancel(_), found {:?}", e),
        }
    }

    #[tokio::test]
    async fn sender_checks_for_want_on_send() {
        let (mut tx, mut rx) = channel::<Custom, ()>();

        // one is allowed to buffer, second is rejected
        let _ = tx.try_send(Custom(1)).expect("1 buffered");
        tx.try_send(Custom(2)).expect_err("2 not ready");

        assert!(PollOnce(&mut rx).await.is_some(), "rx once");

        // Even though 1 has been popped, only 1 could be buffered for the
        // lifetime of the channel.
        tx.try_send(Custom(2)).expect_err("2 still not ready");

        assert!(PollOnce(&mut rx).await.is_none(), "rx empty");

        let _ = tx.try_send(Custom(2)).expect("2 ready");
    }

    #[cfg(feature = "http2")]
    #[test]
    fn unbounded_sender_doesnt_bound_on_want() {
        let (tx, rx) = channel::<Custom, ()>();
        let mut tx = tx.unbound();

        let _ = tx.try_send(Custom(1)).unwrap();
        let _ = tx.try_send(Custom(2)).unwrap();
        let _ = tx.try_send(Custom(3)).unwrap();

        drop(rx);

        let _ = tx.try_send(Custom(4)).unwrap_err();
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn giver_queue_throughput(b: &mut test::Bencher) {
        use crate::{Body, Request, Response};

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let (mut tx, mut rx) = channel::<Request<Body>, Response<Body>>();

        b.iter(move || {
            let _ = tx.send(Request::default()).unwrap();
            rt.block_on(async {
                loop {
                    let poll_once = PollOnce(&mut rx);
                    let opt = poll_once.await;
                    if opt.is_none() {
                        break;
                    }
                }
            });
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn giver_queue_not_ready(b: &mut test::Bencher) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let (_tx, mut rx) = channel::<i32, ()>();
        b.iter(move || {
            rt.block_on(async {
                let poll_once = PollOnce(&mut rx);
                assert!(poll_once.await.is_none());
            });
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn giver_queue_cancel(b: &mut test::Bencher) {
        let (_tx, mut rx) = channel::<i32, ()>();

        b.iter(move || {
            rx.taker.cancel();
        })
    }
}
