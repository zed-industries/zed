//! Provides a lossless, MPMC channel.  All receivers are guaranteed to recieve each message.
//!
//! When a receiver is cloned, the new receive will observe the same series of messages as the original.
//! When a receiver is created with `Sender::subscribe`, it will observe new messages.

use std::fmt;

use super::SendMessage;
use static_assertions::assert_impl_all;

use crate::{
    sink::{PollSend, Sink},
    stream::{PollRecv, Stream},
    sync::{
        mpmc_circular_buffer::{BufferReader, MpmcCircularBuffer, TryRead, TryWrite},
        shared, ReceiverShared, SenderShared,
    },
};

/// Constructs a pair of broadcast endpoints, with a fixed-size buffer of the given capacity
pub fn channel<T: Clone>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    #[cfg(feature = "debug")]
    log::error!("Creating broadcast channel with capacity {}", capacity);
    // we add one spare capacity so that receivers have an empty slot to wait on
    let (buffer, reader) = MpmcCircularBuffer::new(capacity);

    let (tx_shared, rx_shared) = shared(buffer);
    let sender = Sender { shared: tx_shared };

    let receiver = Receiver::new(rx_shared, reader);

    (sender, receiver)
}

/// A broadcast sender that can be used with the postage::Sink trait.  Can be cloned.
///
/// The sender task is suspended when the internal buffer is filled.
///
/// Note: no implementation of the `futures::Sink` trait is provided for the broadcast Sender.
pub struct Sender<T> {
    pub(in crate::channels::broadcast) shared: SenderShared<MpmcCircularBuffer<T>>,
}

unsafe impl<T: Send> Send for Sender<T> {}
unsafe impl<T: Send> Sync for Sender<T> {}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            shared: self.shared.clone(),
        }
    }
}

assert_impl_all!(Sender<SendMessage>: Send, Sync, Clone, fmt::Debug);

impl<T> Sink for Sender<T>
where
    T: Clone,
{
    type Item = T;

    fn poll_send(
        self: std::pin::Pin<&mut Self>,
        cx: &mut crate::Context<'_>,
        value: Self::Item,
    ) -> PollSend<Self::Item> {
        // if all receivers have disconnected, we return Rejected like other channels.
        // tx.subscribe() can be used to produce a new receiver.
        // however, it would not receive this item, as it would need to be called
        //   before the message is sent.
        if self.shared.is_closed() {
            return PollSend::Rejected(value);
        }

        // start at the head
        // if the next element has references,
        //   register for wakeup
        // else
        //   overwrite the element
        let buffer = self.shared.extension();
        match buffer.try_write(value, cx) {
            TryWrite::Pending(value) => PollSend::Pending(value),
            TryWrite::Ready => PollSend::Ready,
        }
    }
}

impl<T> Sender<T> {
    /// Subscribes to the channel, creating a new receiver.  The receiver
    /// will observe all messages sent after the call to subscribe.
    ///
    /// Messages currently in the buffer are not received.
    pub fn subscribe(&self) -> Receiver<T> {
        let shared = self.shared.clone_receiver();
        let reader = shared.extension().new_reader();
        self.shared.notify_self();

        Receiver::new(shared, reader)
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sender").finish()
    }
}

/// A broadcast receiver that can be used with the postage::Stream trait.
///
/// When cloned, the new receiver will begin processing messages at the same location as the original.
pub struct Receiver<T> {
    shared: ReceiverShared<MpmcCircularBuffer<T>>,
    reader: BufferReader,
}

unsafe impl<T: Send> Send for Receiver<T> {}
unsafe impl<T: Send> Sync for Receiver<T> {}

assert_impl_all!(Receiver<SendMessage>: Send, Sync, Clone, fmt::Debug);

impl<T> Receiver<T> {
    fn new(shared: ReceiverShared<MpmcCircularBuffer<T>>, reader: BufferReader) -> Self {
        Self { shared, reader }
    }
}

impl<T> Stream for Receiver<T>
where
    T: Clone,
{
    type Item = T;

    fn poll_recv(
        self: std::pin::Pin<&mut Self>,
        cx: &mut crate::Context<'_>,
    ) -> PollRecv<Self::Item> {
        // unpin self, so Rust can infer that the borrows of reader and buffer are disjoint
        let this = self.get_mut();
        let reader = &mut this.reader;
        let buffer = this.shared.extension();

        match reader.try_read(buffer, cx) {
            TryRead::Pending => {
                this.shared.subscribe_send(cx);

                if this.shared.is_closed() {
                    return PollRecv::Closed;
                }

                PollRecv::Pending
            }
            TryRead::Ready(value) => PollRecv::Ready(value),
        }
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        let buffer = self.shared.extension();
        let reader = self.reader.clone_with(buffer);

        Self::new(self.shared.clone(), reader)
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let buffer = self.shared.extension();
        self.reader.drop_with(buffer);
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

    use super::{channel, Receiver, Sender};

    //TODO: add test covering rx location when cloned on an in-progress channel (exercising tail)
    fn pin<'a, 'b>(
        chan: &mut (Sender<Message>, Receiver<Message>),
    ) -> (Pin<&mut Sender<Message>>, Pin<&mut Receiver<Message>>) {
        let tx = Pin::new(&mut chan.0);
        let rx = Pin::new(&mut chan.1);

        (tx, rx)
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Message(usize);

    #[test]
    fn send_accepted() {
        // crate::logging::enable_log();
        let mut cx = panic_context();
        let mut chan = channel(2);
        let (tx, _rx) = pin(&mut chan);

        assert_eq!(PollSend::Ready, tx.poll_send(&mut cx, Message(1)));
    }

    #[test]
    fn full_send_blocks() {
        // SimpleLogger::new().init().unwrap();
        let mut cx = panic_context();
        let (mut tx, _rx) = channel(2);

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );

        assert_eq!(
            PollSend::Pending(Message(3)),
            Pin::new(&mut tx).poll_send(&mut noop_context(), Message(3))
        );
    }

    #[test]
    fn empty_send_recv() {
        let mut cx = noop_context();
        let (mut tx, mut rx) = channel(0);

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );
    }

    #[test]
    fn send_recv() {
        let mut cx = noop_context();
        let mut chan = channel(2);
        let (tx, rx) = pin(&mut chan);

        assert_eq!(PollSend::Ready, tx.poll_send(&mut cx, Message(1)));
        assert_eq!(PollRecv::Ready(Message(1)), rx.poll_recv(&mut cx));
    }

    #[test]
    fn sender_subscribe_same_read() {
        // crate::logging::enable_log();
        let mut cx = noop_context();
        let (mut tx, mut rx) = channel(2);

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );
        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );

        let mut rx2 = tx.subscribe();
        assert_eq!(PollRecv::Pending, Pin::new(&mut rx2).poll_recv(&mut cx));
        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );
        assert_eq!(
            PollRecv::Ready(Message(2)),
            Pin::new(&mut rx2).poll_recv(&mut cx)
        );
    }

    #[test]
    fn sender_subscribe_different_read() {
        // SimpleLogger::new().init().unwrap();

        let mut cx = noop_context();
        let (mut tx, _rx) = channel(2);

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        let mut rx2 = tx.subscribe();
        assert_eq!(PollRecv::Pending, Pin::new(&mut rx2).poll_recv(&mut cx));
        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );
        assert_eq!(
            PollRecv::Ready(Message(2)),
            Pin::new(&mut rx2).poll_recv(&mut cx)
        );
    }

    #[test]
    fn two_senders_recv() {
        // SimpleLogger::new().init().unwrap();

        let mut cx = panic_context();
        let (mut tx, mut rx) = channel(2);
        let mut tx2 = tx.clone();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );
        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx2).poll_send(&mut cx, Message(2))
        );

        assert_eq!(
            PollSend::Pending(Message(3)),
            Pin::new(&mut tx).poll_send(&mut noop_context(), Message(3))
        );
        assert_eq!(
            PollSend::Pending(Message(3)),
            Pin::new(&mut tx2).poll_send(&mut noop_context(), Message(3))
        );

        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );
        assert_eq!(
            PollRecv::Ready(Message(2)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );

        assert_eq!(
            PollRecv::Pending,
            Pin::new(&mut rx).poll_recv(&mut noop_context())
        );
    }

    #[test]
    fn two_receivers() {
        // SimpleLogger::new().init().unwrap();

        let mut cx = panic_context();
        let (mut tx, mut rx) = channel(2);
        let mut rx2 = rx.clone();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );
        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );
        assert_eq!(
            PollSend::Pending(Message(3)),
            Pin::new(&mut tx).poll_send(&mut noop_context(), Message(3))
        );

        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );
        assert_eq!(
            PollRecv::Ready(Message(2)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );
        assert_eq!(
            PollRecv::Pending,
            Pin::new(&mut rx).poll_recv(&mut noop_context())
        );

        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx2).poll_recv(&mut cx)
        );
        assert_eq!(
            PollRecv::Ready(Message(2)),
            Pin::new(&mut rx2).poll_recv(&mut cx)
        );
        assert_eq!(
            PollRecv::Pending,
            Pin::new(&mut rx2).poll_recv(&mut noop_context())
        );
    }

    #[test]
    fn sender_disconnect() {
        let mut cx = panic_context();
        let (mut tx, mut rx) = channel(100);
        let mut tx2 = tx.clone();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );
        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx2).poll_send(&mut cx, Message(2))
        );
        drop(tx);
        drop(tx2);
        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );
        assert_eq!(
            PollRecv::Ready(Message(2)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );
        assert_eq!(PollRecv::Closed, Pin::new(&mut rx).poll_recv(&mut cx));
    }

    #[test]
    fn receiver_disconnect() {
        let mut cx = panic_context();
        let (mut tx, rx) = channel(100);
        let rx2 = rx.clone();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );
        drop(rx);
        drop(rx2);
        assert_eq!(
            PollSend::Rejected(Message(2)),
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );
    }

    #[test]
    fn receiver_reconnect() {
        let mut cx = panic_context();
        let (mut tx, rx) = channel(100);

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );
        drop(rx);

        let (w2, w2_count) = new_count_waker();
        let mut w2_context = Context::from_waker(&w2);
        assert_eq!(
            PollSend::Rejected(Message(2)),
            Pin::new(&mut tx).poll_send(&mut w2_context, Message(2))
        );

        let _rx = tx.subscribe();
        assert_eq!(0, w2_count.get());
    }

    #[test]
    fn wake_sender() {
        // SimpleLogger::new().init().unwrap();

        let mut cx = panic_context();
        let (mut tx, mut rx) = channel(2);

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        let (w2, w2_count) = new_count_waker();
        let w2_context = Context::from_waker(&w2);
        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );
        assert_eq!(
            PollSend::Pending(Message(3)),
            Pin::new(&mut tx).poll_send(&mut w2_context.into(), Message(3))
        );

        assert_eq!(0, w2_count.get());

        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );

        assert_eq!(
            PollRecv::Ready(Message(2)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );

        assert_eq!(1, w2_count.get());
        assert_eq!(
            PollRecv::Pending,
            Pin::new(&mut rx).poll_recv(&mut noop_context())
        );

        assert_eq!(1, w2_count.get());
    }

    #[test]
    fn wake_receiver() {
        let mut cx = panic_context();
        let (mut tx, mut rx) = channel(100);

        let (w1, w1_count) = new_count_waker();
        let w1_context = Context::from_waker(&w1);

        assert_eq!(
            PollRecv::Pending,
            Pin::new(&mut rx).poll_recv(&mut w1_context.into())
        );

        assert_eq!(0, w1_count.get());

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        assert_eq!(1, w1_count.get());

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );

        assert_eq!(1, w1_count.get());
    }

    #[test]
    fn dropping_receiver_does_not_block() {
        let mut cx = panic_context();
        let (mut tx, mut rx) = channel(2);
        let rx2 = rx.clone();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );

        drop(rx2);
        assert_eq!(
            PollSend::Pending(Message(2)),
            Pin::new(&mut tx).poll_send(&mut noop_context(), Message(2))
        );

        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(4))
        );
    }

    #[test]
    fn drop_receiver_frees_slot() {
        // crate::logging::enable_log();
        let mut cx = panic_context();
        let (mut tx, mut rx) = channel(2);
        let rx2 = rx.clone();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );

        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );

        drop(rx2);

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(3))
        );
    }

    #[test]
    fn wake_sender_on_disconnect() {
        let (mut tx, rx) = channel(2);

        let (w1, w1_count) = new_count_waker();
        let w1_context = Context::from_waker(&w1);
        let mut w1_context: crate::Context<'_> = w1_context.into();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut w1_context, Message(1))
        );

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut w1_context, Message(2))
        );

        assert_eq!(
            PollSend::Pending(Message(3)),
            Pin::new(&mut tx).poll_send(&mut w1_context, Message(3))
        );

        assert_eq!(0, w1_count.get());

        drop(rx);

        assert_eq!(1, w1_count.get());
    }

    #[test]
    fn wake_receiver_on_disconnect() {
        let (tx, mut rx) = channel::<()>(100);

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

    #[test]
    fn reader_bounds_bug() {
        let mut cx = noop_context();
        let (mut tx, mut rx) = channel(2);

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );
        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );
        assert_eq!(
            PollRecv::Ready(Message(2)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );
    }

    #[test]
    fn drop_subscribe_bug() {
        // SimpleLogger::new().init().unwrap();

        let mut cx = noop_context();
        let (mut tx, rx) = channel(2);

        drop(rx);
        let mut rx2 = tx.subscribe();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );
        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx2).poll_recv(&mut cx)
        );
    }

    #[test]
    fn skips_intermediate_bug() {
        let mut cx = noop_context();
        let (mut tx, rx) = channel(2);

        drop(rx);
        let mut rx2 = tx.subscribe();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );
        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );
        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx2).poll_recv(&mut cx)
        );
    }

    #[test]
    fn drop_subscribe_ignores_queued() {
        let mut cx = noop_context();
        let (mut tx, rx) = channel(2);

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        drop(rx);
        let mut rx2 = tx.subscribe();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );
        assert_eq!(
            PollRecv::Ready(Message(2)),
            Pin::new(&mut rx2).poll_recv(&mut cx)
        );
    }

    #[test]
    fn drop_preserves_read() {
        let mut cx = noop_context();
        let (mut tx, mut rx) = channel(2);

        let _rx_pin_message_1 = rx.clone();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(1))
        );

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, Message(2))
        );

        assert_eq!(
            PollRecv::Ready(Message(1)),
            Pin::new(&mut rx).poll_recv(&mut cx)
        );

        let rx2 = rx.clone();
        drop(rx2);

        assert_eq!(
            PollSend::Pending(Message(3)),
            Pin::new(&mut tx).poll_send(&mut cx, Message(3))
        );
    }
}

#[cfg(test)]
mod tokio_tests {
    use std::time::Duration;

    use tokio::{
        task::{spawn, JoinHandle},
        time::{self, timeout},
    };

    use crate::sink::Sink;
    use crate::{
        stream::{Stream, TryRecvError},
        test::{
            capacity_iter, Channel, Channels, Message, CHANNEL_TEST_RECEIVERS,
            CHANNEL_TEST_SENDERS, TEST_TIMEOUT,
        },
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn simple() {
        // crate::logging::enable_log();
        for cap in capacity_iter() {
            let (mut tx, mut rx) = super::channel(cap);

            spawn(async move {
                for message in Message::new_iter(0) {
                    tx.send(message).await.expect("send failed");
                }
            });

            let rx_handle = spawn(async move {
                let mut channel = Channel::new(0);
                while let Some(message) = rx.recv().await {
                    channel.assert_message(&message);
                }
            });

            timeout(TEST_TIMEOUT, rx_handle)
                .await
                .expect("test timeout")
                .expect("join failure");
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn multi_sender() {
        // crate::logging::enable_log();
        for cap in capacity_iter() {
            let (tx, mut rx) = super::channel(cap);

            for i in 0..CHANNEL_TEST_SENDERS {
                let mut tx2 = tx.clone();
                spawn(async move {
                    for message in Message::new_multi_sender(i) {
                        tx2.send(message).await.expect("send failed");
                    }
                });
            }

            drop(tx);

            let rx_handle = spawn(async move {
                let mut channels = Channels::new(CHANNEL_TEST_SENDERS);
                while let Some(message) = rx.recv().await {
                    channels.assert_message(&message);
                }
            });

            timeout(TEST_TIMEOUT, rx_handle)
                .await
                .expect("test timeout")
                .expect("join failure");
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn multi_receiver() {
        // crate::logging::enable_log();
        for cap in capacity_iter() {
            let (mut tx, rx) = super::channel(cap);

            spawn(async move {
                for message in Message::new_iter(0) {
                    tx.send(message).await.expect("send failed");
                }
            });

            let handles: Vec<JoinHandle<()>> = (0..CHANNEL_TEST_RECEIVERS)
                .map(|_| {
                    let mut rx2 = rx.clone();
                    let mut channels = Channels::new(1);

                    spawn(async move {
                        while let Some(message) = rx2.recv().await {
                            channels.assert_message(&message);
                        }
                    })
                })
                .collect();

            drop(rx);

            let rx_handle = spawn(async move {
                for handle in handles {
                    handle.await.expect("Assertion failure");
                }
            });

            timeout(TEST_TIMEOUT, rx_handle)
                .await
                .expect("test timeout")
                .expect("join failure");
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn multi_sender_multi_receiver() {
        // crate::logging::enable_log();
        for cap in capacity_iter() {
            let (tx, rx) = super::channel(cap);

            for i in 0..CHANNEL_TEST_SENDERS {
                let mut tx2 = tx.clone();
                spawn(async move {
                    for message in Message::new_multi_sender(i) {
                        tx2.send(message).await.expect("send failed");
                    }
                });
            }

            drop(tx);

            let handles: Vec<JoinHandle<()>> = (0..CHANNEL_TEST_RECEIVERS)
                .map(|_i| {
                    let mut rx2 = rx.clone();
                    let mut channels = Channels::new(CHANNEL_TEST_SENDERS);

                    spawn(async move {
                        while let Some(message) = rx2.recv().await {
                            channels.assert_message(&message);
                        }
                    })
                })
                .collect();

            drop(rx);

            let rx_handle = spawn(async move {
                for handle in handles {
                    handle.await.expect("Assertion failure");
                }
            });

            timeout(TEST_TIMEOUT, rx_handle)
                .await
                .expect("test timeout")
                .expect("join failure");
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn clone_monster() {
        // crate::logging::enable_log();

        for cap in capacity_iter() {
            let (tx, mut rx) = super::channel(cap);
            let (mut barrier, mut sender_quit) = crate::barrier::channel();

            let mut tx2 = tx.clone();
            spawn(async move {
                for message in Message::new_iter(0) {
                    tx2.send(message).await.expect("send failed");
                }

                barrier.send(()).await.expect("clone task shutdown failed");
            });

            let mut rx2 = rx.clone();
            spawn(async move {
                loop {
                    let next = rx2.try_recv();

                    if let Ok(_) = next {
                        continue;
                    }

                    if let Err(TryRecvError::Closed) = next {
                        break;
                    }

                    if let Ok(_) = sender_quit.try_recv() {
                        break;
                    }

                    let tx3 = tx.clone();
                    let rx3 = rx2.clone();
                    let rx4 = tx.subscribe();
                    time::sleep(Duration::from_micros(100)).await;
                    drop(tx3);
                    drop(rx3);
                    drop(rx4);
                    time::sleep(Duration::from_micros(50)).await;
                }

                drop(tx);
            });

            let rx_handle = spawn(async move {
                let mut channel = Channel::new(0);
                while let Some(message) = rx.recv().await {
                    channel.assert_message(&message);
                }
            });

            timeout(TEST_TIMEOUT, rx_handle)
                .await
                .expect("test timeout")
                .expect("join failure");
        }
    }
}

#[cfg(test)]
mod async_std_tests {
    use std::time::Duration;

    use async_std::{
        future::timeout,
        task::{self, spawn, JoinHandle},
    };

    use crate::{
        sink::Sink,
        stream::{Stream, TryRecvError},
        test::{
            capacity_iter, Channel, Channels, Message, CHANNEL_TEST_RECEIVERS,
            CHANNEL_TEST_SENDERS, TEST_TIMEOUT,
        },
    };

    #[async_std::test]
    async fn simple() {
        crate::logging::enable_log();
        for cap in capacity_iter() {
            let (mut tx, mut rx) = super::channel(cap);

            spawn(async move {
                for message in Message::new_iter(0) {
                    tx.send(message).await.expect("send failed");
                }
            });

            let rx_handle = spawn(async move {
                let mut channel = Channel::new(0);
                while let Some(message) = rx.recv().await {
                    channel.assert_message(&message);
                }
            });

            timeout(TEST_TIMEOUT, rx_handle)
                .await
                .expect("test timeout");
        }
    }

    #[async_std::test]
    async fn multi_sender() {
        // crate::logging::enable_log();

        for cap in capacity_iter() {
            let (tx, mut rx) = super::channel(cap);

            for i in 0..CHANNEL_TEST_SENDERS {
                let mut tx2 = tx.clone();
                spawn(async move {
                    for message in Message::new_multi_sender(i) {
                        tx2.send(message).await.expect("send failed");
                    }
                });
            }

            drop(tx);

            let rx_handle = spawn(async move {
                let mut channels = Channels::new(CHANNEL_TEST_SENDERS);
                while let Some(message) = rx.recv().await {
                    channels.assert_message(&message);
                }
            });

            timeout(TEST_TIMEOUT, rx_handle)
                .await
                .expect("test timeout");
        }
    }

    #[async_std::test]
    async fn multi_receiver() {
        // crate::logging::enable_log();
        for cap in capacity_iter() {
            let (mut tx, rx) = super::channel(cap);

            spawn(async move {
                for message in Message::new_iter(0) {
                    tx.send(message).await.expect("send failed");
                }
            });

            let handles: Vec<JoinHandle<()>> = (0..CHANNEL_TEST_RECEIVERS)
                .map(|_| {
                    let mut rx2 = rx.clone();
                    let mut channels = Channels::new(1);

                    spawn(async move {
                        while let Some(message) = rx2.recv().await {
                            channels.assert_message(&message);
                        }
                    })
                })
                .collect();

            drop(rx);

            let rx_handle = spawn(async move {
                for handle in handles {
                    handle.await;
                }
            });

            timeout(TEST_TIMEOUT, rx_handle)
                .await
                .expect("test timeout");
        }
    }

    #[async_std::test]
    async fn multi_sender_multi_receiver() {
        // crate::logging::enable_log();

        for cap in capacity_iter() {
            let (tx, rx) = super::channel(cap);

            for i in 0..CHANNEL_TEST_SENDERS {
                let mut tx2 = tx.clone();
                spawn(async move {
                    for message in Message::new_multi_sender(i) {
                        tx2.send(message).await.expect("send failed");
                    }
                });
            }

            drop(tx);

            let handles: Vec<JoinHandle<()>> = (0..CHANNEL_TEST_RECEIVERS)
                .map(|_i| {
                    let mut rx2 = rx.clone();
                    let mut channels = Channels::new(CHANNEL_TEST_SENDERS);

                    spawn(async move {
                        while let Some(message) = rx2.recv().await {
                            channels.assert_message(&message);
                        }
                    })
                })
                .collect();

            drop(rx);

            let rx_handle = spawn(async move {
                for handle in handles {
                    handle.await;
                }
            });

            timeout(TEST_TIMEOUT, rx_handle)
                .await
                .expect("test timeout");
        }
    }

    #[async_std::test]
    async fn clone_monster() {
        // crate::logging::enable_log();

        for cap in capacity_iter() {
            let (tx, mut rx) = super::channel(cap);
            let (mut barrier, mut sender_quit) = crate::barrier::channel();

            let mut tx2 = tx.clone();
            spawn(async move {
                for message in Message::new_iter(0) {
                    tx2.send(message).await.expect("send failed");
                }

                barrier.send(()).await.expect("clone task shutdown failed");
            });

            let mut rx2 = rx.clone();
            spawn(async move {
                loop {
                    let next = rx2.try_recv();

                    if let Ok(_) = next {
                        continue;
                    }

                    if let Err(TryRecvError::Closed) = next {
                        break;
                    }

                    if let Ok(_) = sender_quit.try_recv() {
                        break;
                    }

                    let tx3 = tx.clone();
                    let rx3 = rx2.clone();
                    let rx4 = tx.subscribe();
                    task::sleep(Duration::from_micros(100)).await;
                    drop(tx3);
                    drop(rx3);
                    drop(rx4);
                    task::sleep(Duration::from_micros(50)).await;
                }

                drop(tx);
            });

            let rx_handle = spawn(async move {
                let mut channel = Channel::new(0);
                while let Some(message) = rx.recv().await {
                    channel.assert_message(&message);
                }
            });

            timeout(TEST_TIMEOUT, rx_handle)
                .await
                .expect("test timeout");
        }
    }
}
