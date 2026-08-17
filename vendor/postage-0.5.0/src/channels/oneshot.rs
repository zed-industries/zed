//! Oneshot channels transmit a single value between a sender and a reciever.  
//!
//! Neither can be cloned.  If the sender drops, the receiver recieves a `None` value.
use std::fmt;
use std::sync::Arc;

use super::SendMessage;
use crate::{
    sink::{PollSend, Sink},
    stream::{PollRecv, Stream},
    sync::transfer::Transfer,
};
use static_assertions::{assert_impl_all, assert_not_impl_all};

/// Constructs a pair of oneshot endpoints
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    #[cfg(feature = "debug")]
    log::error!("Creating oneshot channel");

    let shared = Arc::new(Transfer::new());
    let sender = Sender {
        shared: shared.clone(),
    };

    let receiver = Receiver { shared };

    (sender, receiver)
}

/// The sender half of a oneshot channel.  Can transmit a single message with the postage::Sink trait.
pub struct Sender<T> {
    pub(in crate::channels::oneshot) shared: Arc<Transfer<T>>,
}

assert_impl_all!(Sender<SendMessage>: Send, Sync, fmt::Debug);
assert_not_impl_all!(Sender<SendMessage>: Clone);

impl<T> Sink for Sender<T> {
    type Item = T;

    fn poll_send(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut crate::Context<'_>,
        value: Self::Item,
    ) -> PollSend<Self::Item> {
        match self.shared.send(value) {
            Ok(_) => PollSend::Ready,
            Err(v) => PollSend::Rejected(v),
        }
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sender").finish()
    }
}

#[cfg(feature = "futures-traits")]
mod impl_futures {
    use crate::sink::SendError;
    use std::task::Poll;

    impl<T> futures::sink::Sink<T> for super::Sender<T> {
        type Error = crate::sink::SendError<T>;

        fn poll_ready(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn start_send(self: std::pin::Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
            self.shared.send(item).map_err(|t| SendError(t))
        }

        fn poll_flush(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.shared.sender_disconnect();
    }
}

/// The receiver half of a oneshot channel.  Can recieve a single message (or none if the sender drops) with the postage::Stream trait.
pub struct Receiver<T> {
    pub(in crate::channels::oneshot) shared: Arc<Transfer<T>>,
}

assert_impl_all!(Sender<SendMessage>: Send, Sync, fmt::Debug);
assert_not_impl_all!(Sender<SendMessage>: Clone);

impl<T> Stream for Receiver<T> {
    type Item = T;

    fn poll_recv(
        self: std::pin::Pin<&mut Self>,
        cx: &mut crate::Context<'_>,
    ) -> PollRecv<Self::Item> {
        self.shared.recv(cx)
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receiver_disconnect();
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Receiver").finish()
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use crate::{
        sink::{PollSend, Sink},
        stream::{PollRecv, Stream},
        test::{noop_context, panic_context},
        Context,
    };
    use futures_test::task::new_count_waker;

    use super::channel;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Message(usize);

    #[test]
    fn send_accepted() {
        let mut cx = noop_context();
        let (mut tx, _rx) = channel();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );
        assert_eq!(
            PollSend::Rejected(Message(2)),
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );
    }

    #[test]
    fn send_recv() {
        let mut cx = noop_context();
        let (mut tx, mut rx) = channel();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );
        assert_eq!(PollRecv::Closed, Pin::new(&mut rx).poll_recv(&mut cx));
    }

    #[test]
    fn sender_disconnect() {
        let mut cx = noop_context();
        let (tx, mut rx) = channel::<Message>();

        drop(tx);

        assert_eq!(PollRecv::Closed, Pin::new(&mut rx).poll_recv(&mut cx));
    }

    #[test]
    fn sender_disconnect_after_poll() {
        let mut cx = noop_context();
        let (tx, mut rx) = channel::<Message>();

        assert_eq!(PollRecv::Pending, Pin::new(&mut rx).poll_recv(&mut cx));

        drop(tx);
        assert_eq!(PollRecv::Closed, Pin::new(&mut rx).poll_recv(&mut cx));
    }

    #[test]
    fn send_then_disconnect() {
        let mut cx = noop_context();
        let (mut tx, mut rx) = channel();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        drop(tx);

        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );

        assert_eq!(PollRecv::Closed, Pin::new(&mut rx).poll_recv(&mut cx));
    }

    #[test]
    fn receiver_disconnect() {
        let mut cx = noop_context();
        let (mut tx, rx) = channel();

        drop(rx);

        assert_eq!(
            PollSend::Rejected(Message(1)),
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );
    }

    #[test]
    fn wake_receiver() {
        let mut cx = panic_context();
        let (mut tx, mut rx) = channel();

        let (w1, w1_count) = new_count_waker();
        let mut w1_context = Context::from_waker(&w1);

        assert_eq!(
            PollRecv::Pending,
            Pin::new(&mut rx).poll_recv(&mut w1_context)
        );

        assert_eq!(0, w1_count.get());

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        assert_eq!(1, w1_count.get());

        assert_eq!(
            PollSend::Rejected(Message(2)),
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );

        assert_eq!(1, w1_count.get());
    }

    #[test]
    fn sender_disconnect_wakes_receiver() {
        let (tx, mut rx) = channel::<usize>();

        let (w1, w1_count) = new_count_waker();
        let mut w1_context = Context::from_waker(&w1);

        assert_eq!(
            PollRecv::Pending,
            Pin::new(&mut rx).poll_recv(&mut w1_context)
        );

        assert_eq!(0, w1_count.get());

        drop(tx);

        assert_eq!(1, w1_count.get());

        assert_eq!(
            PollRecv::Closed,
            Pin::new(&mut rx).poll_recv(&mut w1_context)
        );
    }
}

#[cfg(test)]
mod tokio_tests {
    use std::time::Duration;

    use tokio::{task::spawn, time::timeout};

    use crate::{
        sink::Sink,
        stream::Stream,
        test::{CHANNEL_TEST_ITERATIONS, TEST_TIMEOUT},
    };

    use super::channel;

    #[tokio::test]
    async fn simple() {
        for _ in 0..CHANNEL_TEST_ITERATIONS {
            let (mut tx, mut rx) = channel();

            spawn(async move { tx.send(100usize).await });

            let msg = timeout(TEST_TIMEOUT, async move { rx.recv().await })
                .await
                .expect("test timeout");

            assert_eq!(Some(100usize), msg);
        }
    }

    #[tokio::test]
    async fn sender_disconnect() {
        for _ in 0..CHANNEL_TEST_ITERATIONS {
            let (tx, mut rx) = channel::<usize>();

            spawn(async move { drop(tx) });

            let msg = timeout(Duration::from_millis(100), async move { rx.recv().await })
                .await
                .expect("test timeout");

            assert_eq!(None, msg);
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
        test::{CHANNEL_TEST_ITERATIONS, TEST_TIMEOUT},
    };

    use super::channel;

    #[async_std::test]
    async fn simple() {
        for i in 0..CHANNEL_TEST_ITERATIONS {
            let (mut tx, mut rx) = channel();

            spawn(async move { tx.send(i).await });

            let msg = timeout(TEST_TIMEOUT, async move { rx.recv().await })
                .await
                .expect("test timeout");

            assert_eq!(Some(i), msg);
        }
    }

    #[async_std::test]
    async fn sender_disconnect() {
        for _ in 0..CHANNEL_TEST_ITERATIONS {
            let (tx, mut rx) = channel::<usize>();

            spawn(async move { drop(tx) });

            let msg = timeout(Duration::from_millis(100), async move { rx.recv().await })
                .await
                .expect("test timeout");

            assert_eq!(None, msg);
        }
    }
}
