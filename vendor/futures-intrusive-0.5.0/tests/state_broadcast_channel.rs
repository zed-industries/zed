use futures::future::{FusedFuture, Future};
use futures::task::{Context, Poll};
use futures_intrusive::channel::{
    ChannelSendError, LocalStateBroadcastChannel, StateId,
};
use futures_test::task::{new_count_waker, panic_waker};
use pin_utils::pin_mut;

macro_rules! gen_state_broadcast_tests {
    ($mod_name:ident, $channel_type:ident) => {
        mod $mod_name {
            use super::*;

            type ChannelType = $channel_type<i32>;

            fn assert_send(channel: &ChannelType, value: i32) {
                assert_eq!(Ok(()), channel.send(value));
            }

            fn assert_receive_value<FutureType, T>(
                cx: &mut Context,
                receive_fut: &mut core::pin::Pin<&mut FutureType>,
                expected: T,
            ) -> StateId
            where
                FutureType: Future<Output = Option<(StateId, T)>> + FusedFuture,
                T: PartialEq + core::fmt::Debug,
            {
                let id = match receive_fut.as_mut().poll(cx) {
                    Poll::Pending => panic!("future is not ready"),
                    Poll::Ready(None) => panic!("channel is closed"),
                    Poll::Ready(Some((id, val))) => {
                        if val != expected {
                            panic!("Unexpected value {:?}", val);
                        }
                        id
                    }
                };
                assert!(receive_fut.as_mut().is_terminated());
                id
            }

            fn assert_receive_closed<FutureType, T>(
                cx: &mut Context,
                receive_fut: &mut core::pin::Pin<&mut FutureType>,
            ) where
                FutureType: Future<Output = Option<(StateId, T)>> + FusedFuture,
                T: PartialEq + core::fmt::Debug,
            {
                match receive_fut.as_mut().poll(cx) {
                    Poll::Pending => panic!("future is not ready"),
                    Poll::Ready(None) => {}
                    Poll::Ready(Some(_)) => panic!("future has a value"),
                };
                assert!(receive_fut.as_mut().is_terminated());
            }

            macro_rules! assert_receive {
                ($cx:ident, $channel:expr, $expected: expr, $state_id: expr) => {{
                    let receive_fut = $channel.receive($state_id);
                    pin_mut!(receive_fut);
                    assert!(!receive_fut.as_mut().is_terminated());

                    assert_receive_value($cx, &mut receive_fut, $expected)
                }};
            }

            #[test]
            fn close_status() {
                let channel = ChannelType::new();
                assert!(channel.close().is_newly_closed());
                assert!(channel.close().is_already_closed());
                assert!(channel.close().is_already_closed());
                assert!(channel.close().is_already_closed());
            }

            #[test]
            fn send_on_closed_channel() {
                let channel = ChannelType::new();
                assert!(channel.close().is_newly_closed());
                assert_eq!(Err(ChannelSendError(5)), channel.send(5));
            }

            #[test]
            fn close_unblocks_receive() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let fut = channel.receive(Default::default());
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_pending());
                let fut2 = channel.receive(Default::default());
                pin_mut!(fut2);
                assert!(fut2.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                assert!(channel.close().is_newly_closed());
                assert_eq!(count, 2);
                assert_receive_closed(cx, &mut fut);
                assert_receive_closed(cx, &mut fut2);
            }

            #[test]
            fn receive_after_send() {
                let channel = ChannelType::new();
                let waker = &panic_waker();
                let cx = &mut Context::from_waker(&waker);
                let mut state_id = StateId::new();

                assert_send(&channel, 1);
                assert_send(&channel, 2);

                assert_receive!(cx, &channel, 2, state_id);
                state_id = assert_receive!(cx, &channel, 2, state_id);

                assert_send(&channel, 5);
                assert_send(&channel, 6);
                assert_send(&channel, 7);
                assert!(channel.close().is_newly_closed());

                assert_receive!(cx, &channel, 7, state_id);
                assert_receive!(cx, &channel, 7, state_id);
                state_id = assert_receive!(cx, &channel, 7, state_id);

                let receive_fut = channel.receive(state_id);
                pin_mut!(receive_fut);
                assert_receive_closed(cx, &mut receive_fut);
            }

            #[test]
            fn try_receive() {
                let channel = ChannelType::new();
                let state_id = StateId::new();

                assert!(channel.try_receive(state_id).is_none());
                assert_send(&channel, 0);

                let (state_id, _) = channel.try_receive(state_id).unwrap();
                assert!(channel.try_receive(state_id).is_none());
            }

            #[test]
            fn send_unblocks_receive() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let fut = channel.receive(Default::default());
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                let fut2 = channel.receive(Default::default());
                pin_mut!(fut2);
                assert!(fut2.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                assert_send(&channel, 99);
                assert_eq!(count, 2);
                let next_state_id = assert_receive_value(cx, &mut fut, 99);
                assert_eq!(next_state_id, assert_receive_value(cx, &mut fut2, 99));
            }

            #[test]
            fn get_increasing_state_id() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);
                let state_id_0 = StateId::new();

                let fut01 = channel.receive(state_id_0);
                let fut02 = channel.receive(state_id_0);
                pin_mut!(fut01, fut02);
                assert!(fut01.as_mut().poll(cx).is_pending());
                assert!(fut02.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);
                assert_send(&channel, 99);
                let state_id_1 = assert_receive_value(cx, &mut fut01, 99);
                assert_eq!(state_id_1, assert_receive_value(cx, &mut fut02, 99));
                assert!(state_id_1 != state_id_0);

                let fut11 = channel.receive(state_id_1);
                let fut12 = channel.receive(state_id_1);
                pin_mut!(fut11, fut12);
                assert!(fut11.as_mut().poll(cx).is_pending());
                assert!(fut12.as_mut().poll(cx).is_pending());
                assert_eq!(count, 2);
                assert_send(&channel, 100);
                let state_id_2 = assert_receive_value(cx, &mut fut11, 100);
                assert_eq!(state_id_2, assert_receive_value(cx, &mut fut12, 100));
                assert!(state_id_2 != state_id_1);

                let fut21 = channel.receive(state_id_2);
                let fut22 = channel.receive(state_id_2);
                pin_mut!(fut21, fut22);
                assert!(fut21.as_mut().poll(cx).is_pending());
                assert!(fut22.as_mut().poll(cx).is_pending());
                assert_eq!(count, 4);
                assert_send(&channel, 101);
                let state_id_3 = assert_receive_value(cx, &mut fut21, 101);
                assert_eq!(state_id_3, assert_receive_value(cx, &mut fut22, 101));
                assert!(state_id_3 != state_id_2);

                let fut31 = channel.receive(state_id_3);
                pin_mut!(fut31);
                assert!(fut31.as_mut().poll(cx).is_pending());
                assert!(channel.close().is_newly_closed());
                assert_receive_closed(cx, &mut fut31);
            }

            #[test]
            fn get_same_element_for_same_state_id() {
                let channel = ChannelType::new();
                let (waker, _count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);
                let state_id = StateId::new();

                assert_send(&channel, 1);

                let receive_fut = channel.receive(state_id);
                pin_mut!(receive_fut);
                let (state_id_21, val) = match receive_fut.as_mut().poll(cx) {
                    Poll::Ready(Some(res)) => res,
                    _ => panic!("future is not ready or closed"),
                };
                assert_eq!(1, val);
                assert!(state_id != state_id_21);

                let receive_fut_2 = channel.receive(state_id);
                pin_mut!(receive_fut_2);
                let (state_id_22, val) = match receive_fut_2.as_mut().poll(cx) {
                    Poll::Ready(Some(res)) => res,
                    _ => panic!("future is not ready or closed"),
                };
                assert_eq!(1, val);
                assert!(state_id != state_id_22);

                assert_eq!(state_id_21, state_id_22);
            }

            #[test]
            fn cancel_receive_mid_wait() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                {
                    let mut poll1 = Box::pin(channel.receive(Default::default()));
                    let mut poll2 = Box::pin(channel.receive(Default::default()));
                    let mut poll3 = Box::pin(channel.receive(Default::default()));
                    let mut poll4 = Box::pin(channel.receive(Default::default()));
                    let mut poll5 = Box::pin(channel.receive(Default::default()));

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

                    assert_send(&channel, 1);
                    assert_eq!(count, 3);
                    assert_receive_value(cx, &mut poll1.as_mut(), 1);
                    assert_receive_value(cx, &mut poll3.as_mut(), 1);
                    assert_receive_value(cx, &mut poll5.as_mut(), 1);
                }
            }

            #[test]
            fn cancel_receive_end_wait() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let poll1 = channel.receive(Default::default());
                let poll2 = channel.receive(Default::default());
                let poll3 = channel.receive(Default::default());
                let poll4 = channel.receive(Default::default());

                pin_mut!(poll1);
                pin_mut!(poll2);
                pin_mut!(poll3);
                pin_mut!(poll4);

                assert!(poll1.as_mut().poll(cx).is_pending());
                assert!(poll2.as_mut().poll(cx).is_pending());

                // Start polling some wait handles which get cancelled
                // before new ones are attached
                {
                    let poll5 = channel.receive(Default::default());
                    let poll6 = channel.receive(Default::default());
                    pin_mut!(poll5);
                    pin_mut!(poll6);
                    assert!(poll5.as_mut().poll(cx).is_pending());
                    assert!(poll6.as_mut().poll(cx).is_pending());
                }

                assert!(poll3.as_mut().poll(cx).is_pending());
                assert!(poll4.as_mut().poll(cx).is_pending());

                assert_send(&channel, 0);
                assert_send(&channel, 1);
                assert_send(&channel, 2);

                assert_receive_value(cx, &mut poll1, 2);
                assert_receive_value(cx, &mut poll2, 2);
                assert_receive_value(cx, &mut poll3, 2);

                assert_send(&channel, 3);
                assert_receive_value(cx, &mut poll4, 3);

                assert_eq!(count, 4);
            }

            #[test]
            fn poll_from_multiple_executors() {
                let (waker_1, count_1) = new_count_waker();
                let (waker_2, count_2) = new_count_waker();
                let channel = ChannelType::new();

                let cx_1 = &mut Context::from_waker(&waker_1);
                let cx_2 = &mut Context::from_waker(&waker_2);

                let fut = channel.receive(Default::default());
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx_1).is_pending());
                assert!(fut.as_mut().poll(cx_2).is_pending());

                assert_send(&channel, 99);
                assert_eq!(count_1, 0);
                assert_eq!(count_2, 1);

                let _next_state_id = assert_receive_value(cx_2, &mut fut, 99);
            }
        }
    };
}

gen_state_broadcast_tests!(
    local_state_broadcast_channel_tests,
    LocalStateBroadcastChannel
);

#[cfg(feature = "std")]
mod if_std {
    use super::*;
    use futures_intrusive::channel::{
        shared::state_broadcast_channel, StateBroadcastChannel,
    };

    gen_state_broadcast_tests!(
        state_broadcast_channel_tests,
        StateBroadcastChannel
    );

    fn is_send<T: Send>(_: &T) {}

    fn is_send_value<T: Send>(_: T) {}

    fn is_sync<T: Sync>(_: &T) {}

    #[test]
    fn channel_futures_are_send() {
        let channel = StateBroadcastChannel::<i32>::new();
        is_sync(&channel);
        {
            let state_id = StateId::new();
            let recv_fut = channel.receive(state_id);
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
        let (sender, receiver) = state_broadcast_channel::<i32>();
        is_sync(&sender);
        is_sync(&receiver);
        is_send_value(sender.clone());
        is_send_value(receiver.clone());
        let state_id = StateId::new();
        let recv_fut = receiver.receive(state_id);
        is_send(&recv_fut);
        pin_mut!(recv_fut);
        is_send(&recv_fut);
        let send_fut = sender.send(3);
        is_send(&send_fut);
        pin_mut!(send_fut);
        is_send(&send_fut);
    }

    #[test]
    fn dropping_shared_channel_senders_closes_channel() {
        let (waker, _) = new_count_waker();
        let cx = &mut Context::from_waker(&waker);
        let state_id = StateId::new();

        let (sender, receiver) = state_broadcast_channel::<i32>();
        let sender2 = sender.clone();
        let receiver2 = receiver.clone();

        let fut = receiver.receive(state_id);
        pin_mut!(fut);
        assert!(fut.as_mut().poll(cx).is_pending());
        let fut2 = receiver2.receive(state_id);
        pin_mut!(fut2);
        assert!(fut2.as_mut().poll(cx).is_pending());

        drop(sender);
        assert!(fut.as_mut().poll(cx).is_pending());
        assert!(fut2.as_mut().poll(cx).is_pending());

        drop(sender2);
        match fut.as_mut().poll(cx) {
            Poll::Ready(None) => {}
            Poll::Ready(Some(_)) => panic!("Expected no value"),
            Poll::Pending => panic!("Expected channel to be closed"),
        }
        match fut2.as_mut().poll(cx) {
            Poll::Ready(None) => {}
            Poll::Ready(Some(_)) => panic!("Expected no value"),
            Poll::Pending => panic!("Expected channel to be closed"),
        }
    }

    #[test]
    fn dropping_shared_channel_receivers_closes_channel() {
        let (sender, receiver) = state_broadcast_channel::<i32>();
        let sender2 = sender.clone();
        let receiver2 = receiver.clone();

        drop(receiver);
        assert_eq!(Ok(()), sender.send(5));
        assert_eq!(Ok(()), sender2.send(7));

        drop(receiver2);
        assert_eq!(Err(ChannelSendError(5)), sender.send(5));
        assert_eq!(Err(ChannelSendError(7)), sender2.send(7));
    }

    #[test]
    fn try_receive() {
        let (sender, receiver) = state_broadcast_channel::<i32>();
        let state_id = StateId::new();

        assert!(receiver.try_receive(state_id).is_none());
        sender.send(1).unwrap();

        let (state_id, _) = receiver.try_receive(state_id).unwrap();
        assert!(receiver.try_receive(state_id).is_none());
    }
}
