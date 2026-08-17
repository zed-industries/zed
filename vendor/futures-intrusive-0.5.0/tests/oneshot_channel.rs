use futures::future::{FusedFuture, Future};
use futures::task::{Context, Poll};
use futures_intrusive::channel::{ChannelSendError, LocalOneshotChannel};
use futures_test::task::{new_count_waker, panic_waker};
use pin_utils::pin_mut;

macro_rules! gen_oneshot_tests {
    ($mod_name:ident, $channel_type:ident) => {
        mod $mod_name {
            use super::*;

            fn assert_receive_done<FutureType, T>(
                cx: &mut Context,
                receive_fut: &mut core::pin::Pin<&mut FutureType>,
                value: Option<T>,
            ) where
                FutureType: Future<Output = Option<T>> + FusedFuture,
                T: PartialEq + core::fmt::Debug,
            {
                match receive_fut.as_mut().poll(cx) {
                    Poll::Pending => panic!("future is not ready"),
                    Poll::Ready(res) => {
                        if res != value {
                            panic!("Unexpected value {:?}", res);
                        }
                    }
                };
                assert!(receive_fut.as_mut().is_terminated());
            }

            #[test]
            fn send_on_closed_channel() {
                let channel = $channel_type::<i32>::new();
                assert!(channel.close().is_newly_closed());
                assert_eq!(Err(ChannelSendError(5)), channel.send(5));
            }

            #[test]
            fn close_status() {
                let channel = $channel_type::<i32>::new();

                assert!(channel.close().is_newly_closed());
                assert!(channel.close().is_already_closed());
                assert!(channel.close().is_already_closed());
            }

            #[test]
            fn close_unblocks_receive() {
                let channel = $channel_type::<i32>::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let fut = channel.receive();
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_pending());
                let fut2 = channel.receive();
                pin_mut!(fut2);
                assert!(fut2.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                assert!(channel.close().is_newly_closed());
                assert_eq!(count, 2);
                assert_receive_done(cx, &mut fut, None);
                assert_receive_done(cx, &mut fut2, None);
            }

            #[test]
            fn receive_after_send() {
                let channel = $channel_type::<i32>::new();
                let waker = &panic_waker();
                let cx = &mut Context::from_waker(&waker);

                channel.send(5).unwrap();

                let receive_fut = channel.receive();
                pin_mut!(receive_fut);
                assert!(!receive_fut.as_mut().is_terminated());

                assert_receive_done(cx, &mut receive_fut, Some(5));

                // A second receive attempt must yield None, since the
                // value was taken out of the channel
                let receive_fut2 = channel.receive();
                pin_mut!(receive_fut2);
                assert_receive_done(cx, &mut receive_fut2, None);
            }

            #[test]
            fn send_after_receive() {
                let channel = $channel_type::<i32>::new();
                let (waker, _) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let receive_fut1 = channel.receive();
                let receive_fut2 = channel.receive();
                pin_mut!(receive_fut1);
                pin_mut!(receive_fut2);
                assert!(!receive_fut1.as_mut().is_terminated());
                assert!(!receive_fut2.as_mut().is_terminated());

                let poll_res1 = receive_fut1.as_mut().poll(cx);
                let poll_res2 = receive_fut2.as_mut().poll(cx);
                assert!(poll_res1.is_pending());
                assert!(poll_res2.is_pending());

                channel.send(5).unwrap();

                assert_receive_done(cx, &mut receive_fut1, Some(5));
                // receive_fut2 isn't terminated, since it hasn't been polled
                assert!(!receive_fut2.as_mut().is_terminated());
                // When it gets polled, it must evaluate to None
                assert_receive_done(cx, &mut receive_fut2, None);
            }

            #[test]
            fn second_send_rejects_value() {
                let channel = $channel_type::<i32>::new();
                let (waker, _) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let receive_fut1 = channel.receive();
                pin_mut!(receive_fut1);
                assert!(!receive_fut1.as_mut().is_terminated());
                assert!(receive_fut1.as_mut().poll(cx).is_pending());

                // First send
                channel.send(5).unwrap();

                assert!(receive_fut1.as_mut().poll(cx).is_ready());

                // Second send
                let send_res = channel.send(7);
                match send_res {
                    Err(ChannelSendError(7)) => {} // expected
                    _ => panic!("Second second should reject"),
                }
            }

            #[test]
            fn cancel_mid_wait() {
                let channel = $channel_type::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                {
                    // Cancel a wait in between other waits
                    // In order to arbitrarily drop a non movable future we have to box and pin it
                    let mut poll1 = Box::pin(channel.receive());
                    let mut poll2 = Box::pin(channel.receive());
                    let mut poll3 = Box::pin(channel.receive());
                    let mut poll4 = Box::pin(channel.receive());
                    let mut poll5 = Box::pin(channel.receive());

                    assert!(poll1.as_mut().poll(cx).is_pending());
                    assert!(poll2.as_mut().poll(cx).is_pending());
                    assert!(poll3.as_mut().poll(cx).is_pending());
                    assert!(poll4.as_mut().poll(cx).is_pending());
                    assert!(poll5.as_mut().poll(cx).is_pending());
                    assert!(!poll1.is_terminated());
                    assert!(!poll2.is_terminated());
                    assert!(!poll3.is_terminated());
                    assert!(!poll4.is_terminated());
                    assert!(!poll5.is_terminated());

                    // Cancel 2 futures. Only the remaining ones should get completed
                    drop(poll2);
                    drop(poll4);

                    assert!(poll1.as_mut().poll(cx).is_pending());
                    assert!(poll3.as_mut().poll(cx).is_pending());
                    assert!(poll5.as_mut().poll(cx).is_pending());

                    assert_eq!(count, 0);
                    channel.send(7).unwrap();
                    assert_eq!(count, 3);

                    assert!(poll1.as_mut().poll(cx).is_ready());
                    assert!(poll3.as_mut().poll(cx).is_ready());
                    assert!(poll5.as_mut().poll(cx).is_ready());
                    assert!(poll1.is_terminated());
                    assert!(poll3.is_terminated());
                    assert!(poll5.is_terminated());
                }

                assert_eq!(count, 3)
            }

            #[test]
            fn cancel_end_wait() {
                let channel = $channel_type::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let poll1 = channel.receive();
                let poll2 = channel.receive();
                let poll3 = channel.receive();
                let poll4 = channel.receive();

                pin_mut!(poll1);
                pin_mut!(poll2);
                pin_mut!(poll3);
                pin_mut!(poll4);

                assert!(poll1.as_mut().poll(cx).is_pending());
                assert!(poll2.as_mut().poll(cx).is_pending());

                // Start polling some wait handles which get cancelled
                // before new ones are attached
                {
                    let poll5 = channel.receive();
                    let poll6 = channel.receive();
                    pin_mut!(poll5);
                    pin_mut!(poll6);
                    assert!(poll5.as_mut().poll(cx).is_pending());
                    assert!(poll6.as_mut().poll(cx).is_pending());
                }

                assert!(poll3.as_mut().poll(cx).is_pending());
                assert!(poll4.as_mut().poll(cx).is_pending());

                channel.send(99).unwrap();

                assert!(poll1.as_mut().poll(cx).is_ready());
                assert!(poll2.as_mut().poll(cx).is_ready());
                assert!(poll3.as_mut().poll(cx).is_ready());
                assert!(poll4.as_mut().poll(cx).is_ready());

                assert_eq!(count, 4)
            }

            #[test]
            fn poll_from_multiple_executors() {
                let (waker_1, count_1) = new_count_waker();
                let (waker_2, count_2) = new_count_waker();
                let channel = $channel_type::new();

                let cx_1 = &mut Context::from_waker(&waker_1);
                let cx_2 = &mut Context::from_waker(&waker_2);

                let fut = channel.receive();
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx_1).is_pending());
                assert!(fut.as_mut().poll(cx_2).is_pending());

                channel.send(99).unwrap();
                assert_eq!(count_1, 0);
                assert_eq!(count_2, 1);

                assert_receive_done(cx_2, &mut fut, Some(99));
            }
        }
    };
}

gen_oneshot_tests!(local_oneshot_channel_tests, LocalOneshotChannel);

#[cfg(feature = "std")]
mod if_std {
    use super::*;
    use futures_intrusive::channel::shared::oneshot_channel;
    use futures_intrusive::channel::OneshotChannel;

    gen_oneshot_tests!(oneshot_channel_tests, OneshotChannel);

    fn is_send<T: Send>(_: &T) {}

    fn is_send_value<T: Send>(_: T) {}

    fn is_sync<T: Sync>(_: &T) {}

    #[test]
    fn channel_futures_are_send() {
        let channel = OneshotChannel::<i32>::new();
        is_sync(&channel);
        {
            let recv_fut = channel.receive();
            is_send(&recv_fut);
            pin_mut!(recv_fut);
            is_send(&recv_fut);
            let send_fut = channel.send(3);
            is_send(&send_fut);
            pin_mut!(send_fut);
            is_send(&send_fut);
        }
        is_send_value(channel);
    }

    #[test]
    fn shared_channel_futures_are_send() {
        let (sender, receiver) = oneshot_channel::<i32>();
        is_sync(&sender);
        is_sync(&receiver);
        let recv_fut = receiver.receive();
        is_send(&recv_fut);
        pin_mut!(recv_fut);
        is_send(&recv_fut);
        let send_fut = sender.send(3);
        is_send(&send_fut);
        pin_mut!(send_fut);
        is_send(&send_fut);

        is_send_value(sender);
        is_send_value(receiver);
    }

    #[test]
    fn dropping_shared_channel_senders_closes_channel() {
        let (waker, _) = new_count_waker();
        let cx = &mut Context::from_waker(&waker);

        let (sender, receiver) = oneshot_channel::<i32>();

        let fut = receiver.receive();
        pin_mut!(fut);
        assert!(fut.as_mut().poll(cx).is_pending());

        drop(sender);

        match fut.as_mut().poll(cx) {
            Poll::Ready(None) => {}
            Poll::Ready(Some(_)) => panic!("Expected no value"),
            Poll::Pending => panic!("Expected channel to be closed"),
        }
    }

    #[test]
    fn dropping_shared_channel_receivers_closes_channel() {
        let (sender, receiver) = oneshot_channel::<i32>();
        drop(receiver);

        assert_eq!(Err(ChannelSendError(5)), sender.send(5));
    }
}
