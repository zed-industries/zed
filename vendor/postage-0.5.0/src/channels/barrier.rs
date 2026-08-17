//! Barriers transmit when the sender half is dropped, and can synchronize events in async tasks.
//!
//! The barrier can also be triggered with `tx.send(())`.

use std::fmt;
use std::sync::Arc;

use atomic::{Atomic, Ordering};
use static_assertions::{assert_impl_all, assert_not_impl_all};

use crate::{
    sink::{PollSend, Sink},
    stream::{PollRecv, Stream},
    sync::notifier::Notifier,
};

/// Constructs a pair of barrier endpoints, which transmits when the sender is dropped.
pub fn channel() -> (Sender, Receiver) {
    #[cfg(feature = "debug")]
    log::error!("Creating barrier channel");
    let shared = Arc::new(Shared {
        state: Atomic::new(State::Pending),
        notify_rx: Notifier::new(),
    });

    let sender = Sender {
        shared: shared.clone(),
    };

    let receiver = Receiver { shared };

    (sender, receiver)
}

/// The sender half of a barrier channel.  Dropping the sender transmits to the receiver.
///
/// Can also be triggered by sending `()` with the postage::Sink trait.
pub struct Sender {
    pub(in crate::channels::barrier) shared: Arc<Shared>,
}

assert_impl_all!(Sender: Send, Sync, fmt::Debug);
assert_not_impl_all!(Sender: Clone);

impl Sink for Sender {
    type Item = ();

    fn poll_send(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut crate::Context<'_>,
        _value: (),
    ) -> PollSend<Self::Item> {
        match self.shared.state.load(Ordering::Acquire) {
            State::Pending => {
                self.shared.close();
                PollSend::Ready
            }
            State::Sent => PollSend::Rejected(()),
        }
    }
}

impl fmt::Debug for Sender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sender").finish()
    }
}

#[cfg(feature = "futures-traits")]
mod impl_futures {
    use super::State;
    use crate::sink::SendError;
    use atomic::Ordering;
    use std::task::{Context, Poll};

    impl futures::sink::Sink<()> for super::Sender {
        type Error = SendError<()>;

        fn poll_ready(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> std::task::Poll<Result<(), Self::Error>> {
            match self.shared.state.load(Ordering::Acquire) {
                State::Pending => Poll::Ready(Ok(())),
                State::Sent => Poll::Ready(Err(SendError(()))),
            }
        }

        fn start_send(self: std::pin::Pin<&mut Self>, _item: ()) -> Result<(), Self::Error> {
            match self.shared.state.load(Ordering::Acquire) {
                State::Pending => {
                    self.shared.close();
                    Ok(())
                }
                State::Sent => Err(SendError(())),
            }
        }

        fn poll_flush(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
    }
}

impl Drop for Sender {
    fn drop(&mut self) {
        self.shared.close();
    }
}

/// A barrier reciever.  Can be used with the postage::Stream trait to return a `()` value when the Sender is dropped.
#[derive(Clone)]
pub struct Receiver {
    pub(in crate::channels::barrier) shared: Arc<Shared>,
}

assert_impl_all!(Receiver: Clone, Send, Sync, fmt::Debug);

#[derive(Copy, Clone)]
enum State {
    Pending,
    Sent,
}

struct Shared {
    state: Atomic<State>,
    notify_rx: Notifier,
}

impl Shared {
    pub fn close(&self) {
        self.state.store(State::Sent, Ordering::Release);
        self.notify_rx.notify();
    }
}

impl Stream for Receiver {
    type Item = ();

    fn poll_recv(
        self: std::pin::Pin<&mut Self>,
        cx: &mut crate::Context<'_>,
    ) -> PollRecv<Self::Item> {
        match self.shared.state.load(Ordering::Acquire) {
            State::Pending => {
                self.shared.notify_rx.subscribe(cx);

                if let State::Sent = self.shared.state.load(Ordering::Acquire) {
                    return PollRecv::Ready(());
                }

                PollRecv::Pending
            }
            State::Sent => PollRecv::Ready(()),
        }
    }
}

impl fmt::Debug for Receiver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Receiver").finish()
    }
}

#[cfg(test)]
mod tests {
    use std::{pin::Pin, task::Context};

    use crate::{
        sink::{PollSend, Sink},
        stream::{PollRecv, Stream},
        test::{noop_context, panic_context},
    };
    use futures_test::task::new_count_waker;

    use super::channel;

    #[test]
    fn send_accepted() {
        let mut cx = noop_context();
        let (mut tx, _rx) = channel();

        assert_eq!(PollSend::Ready, Pin::new(&mut tx).poll_send(&mut cx, ()));
        assert_eq!(
            PollSend::Rejected(()),
            Pin::new(&mut tx).poll_send(&mut cx, ())
        );
    }

    #[test]
    fn send_recv() {
        let mut cx = noop_context();
        let (mut tx, mut rx) = channel();

        assert_eq!(PollSend::Ready, Pin::new(&mut tx).poll_send(&mut cx, ()));

        assert_eq!(PollRecv::Ready(()), Pin::new(&mut rx).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(()), Pin::new(&mut rx).poll_recv(&mut cx));
    }

    #[test]
    fn sender_disconnect() {
        let mut cx = noop_context();
        let (tx, mut rx) = channel();

        drop(tx);

        assert_eq!(PollRecv::Ready(()), Pin::new(&mut rx).poll_recv(&mut cx));
    }

    #[test]
    fn send_then_disconnect() {
        let mut cx = noop_context();
        let (mut tx, mut rx) = channel();

        assert_eq!(PollSend::Ready, Pin::new(&mut tx).poll_send(&mut cx, ()));

        drop(tx);

        assert_eq!(PollRecv::Ready(()), Pin::new(&mut rx).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(()), Pin::new(&mut rx).poll_recv(&mut cx));
    }

    #[test]
    fn receiver_disconnect() {
        let mut cx = noop_context();
        let (mut tx, rx) = channel();

        drop(rx);

        assert_eq!(PollSend::Ready, Pin::new(&mut tx).poll_send(&mut cx, ()));
    }

    #[test]
    fn receiver_clone() {
        let mut cx = noop_context();
        let (mut tx, mut rx) = channel();
        let mut rx2 = rx.clone();

        assert_eq!(PollSend::Ready, Pin::new(&mut tx).poll_send(&mut cx, ()));

        assert_eq!(PollRecv::Ready(()), Pin::new(&mut rx).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(()), Pin::new(&mut rx2).poll_recv(&mut cx));
    }

    #[test]
    fn receiver_send_then_clone() {
        let mut cx = noop_context();
        let (mut tx, mut rx) = channel();

        assert_eq!(PollSend::Ready, Pin::new(&mut tx).poll_send(&mut cx, ()));

        let mut rx2 = rx.clone();

        assert_eq!(PollRecv::Ready(()), Pin::new(&mut rx).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(()), Pin::new(&mut rx2).poll_recv(&mut cx));
    }

    #[test]
    fn wake_receiver() {
        let mut cx = panic_context();
        let (mut tx, mut rx) = channel();

        let (w, w_count) = new_count_waker();
        let w_context = Context::from_waker(&w);

        assert_eq!(
            PollRecv::Pending,
            Pin::new(&mut rx).poll_recv(&mut w_context.into())
        );

        assert_eq!(0, w_count.get());

        assert_eq!(PollSend::Ready, Pin::new(&mut tx).poll_send(&mut cx, ()));

        assert_eq!(1, w_count.get());

        assert_eq!(
            PollSend::Rejected(()),
            Pin::new(&mut tx).poll_send(&mut cx, ())
        );

        assert_eq!(1, w_count.get());
    }

    #[test]
    fn wake_receiver_on_disconnect() {
        let (tx, mut rx) = channel();

        let (w1, w1_count) = new_count_waker();
        let w1_context = Context::from_waker(&w1);

        assert_eq!(
            PollRecv::Pending,
            Pin::new(&mut rx).poll_recv(&mut w1_context.into())
        );

        assert_eq!(0, w1_count.get());

        drop(tx);

        assert_eq!(1, w1_count.get());
    }
}

#[cfg(test)]
mod tokio_tests {
    use std::time::Duration;

    use tokio::{task::spawn, time::timeout};

    use crate::{
        sink::Sink,
        stream::Stream,
        test::{CHANNEL_TEST_ITERATIONS, CHANNEL_TEST_RECEIVERS, TEST_TIMEOUT},
    };

    use super::Receiver;

    async fn assert_rx(mut rx: Receiver) {
        if let Err(_e) = timeout(Duration::from_millis(100), rx.recv()).await {
            panic!("Timeout waiting for barrier");
        }
    }

    #[tokio::test]
    async fn simple() {
        for _ in 0..CHANNEL_TEST_ITERATIONS {
            let (mut tx, rx) = super::channel();

            spawn(async move {
                tx.send(()).await.expect("Should send message");
            });

            timeout(TEST_TIMEOUT, async move {
                assert_rx(rx).await;
            })
            .await
            .expect("test timeout");
        }
    }

    #[tokio::test]
    async fn simple_drop() {
        for _ in 0..CHANNEL_TEST_ITERATIONS {
            let (tx, rx) = super::channel();

            spawn(async move {
                drop(tx);
            });

            timeout(TEST_TIMEOUT, async move {
                assert_rx(rx).await;
            })
            .await
            .expect("test timeout");
        }
    }

    #[tokio::test]
    async fn multi_receiver() {
        for _ in 0..CHANNEL_TEST_ITERATIONS {
            let (tx, rx) = super::channel();

            let handles = (0..CHANNEL_TEST_RECEIVERS).map(move |_| {
                let rx2 = rx.clone();

                spawn(async move {
                    assert_rx(rx2).await;
                })
            });

            spawn(async move {
                drop(tx);
            });

            let rx_handle = spawn(async move {
                for handle in handles {
                    handle.await.expect("Assertion failure");
                }
            });

            timeout(TEST_TIMEOUT, rx_handle)
                .await
                .expect("test timeout")
                .expect("join error");
        }
    }
}

#[cfg(test)]
mod async_std_tests {
    use std::time::Duration;

    use async_std::{future::timeout, task::spawn};

    use crate::{
        sink::Sink,
        stream::Stream,
        test::{CHANNEL_TEST_ITERATIONS, CHANNEL_TEST_RECEIVERS, TEST_TIMEOUT},
    };

    use super::Receiver;

    async fn assert_rx(mut rx: Receiver) {
        if let Err(_e) = timeout(Duration::from_millis(100), rx.recv()).await {
            panic!("Timeout waiting for barrier");
        }
    }

    #[async_std::test]
    async fn simple() {
        for _ in 0..CHANNEL_TEST_ITERATIONS {
            let (mut tx, rx) = super::channel();

            spawn(async move {
                tx.send(()).await.expect("Should send message");
            });

            timeout(TEST_TIMEOUT, async move {
                assert_rx(rx).await;
            })
            .await
            .expect("test timeout");
        }
    }

    #[async_std::test]
    async fn simple_drop() {
        // crate::logging::enable_log();

        for _ in 0..CHANNEL_TEST_ITERATIONS {
            let (tx, rx) = super::channel();

            spawn(async move {
                drop(tx);
            });

            timeout(TEST_TIMEOUT, async move {
                assert_rx(rx).await;
            })
            .await
            .expect("test timeout");
        }
    }

    #[async_std::test]
    async fn multi_receiver() {
        for _ in 0..CHANNEL_TEST_ITERATIONS {
            let (tx, rx) = super::channel();

            let handles = (0..CHANNEL_TEST_RECEIVERS).map(|_| {
                let rx2 = rx.clone();

                spawn(async move {
                    assert_rx(rx2).await;
                })
            });

            drop(tx);

            timeout(TEST_TIMEOUT, async move {
                for handle in handles {
                    handle.await;
                }
            })
            .await
            .expect("test timeout");
        }
    }
}
