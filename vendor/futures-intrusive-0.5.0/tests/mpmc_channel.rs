use futures::task::{Context, Poll};
use futures::{
    future::{FusedFuture, Future},
    stream::{FusedStream, Stream},
};
use futures_intrusive::channel::{
    ChannelSendError, LocalChannel, LocalUnbufferedChannel,
};
use futures_test::task::{new_count_waker, panic_waker};
use pin_utils::pin_mut;

#[derive(Debug)]
struct DropCounterInner {
    count: std::collections::HashMap<usize, usize>,
}

#[derive(Clone, Debug)]
struct DropCounter {
    inner: std::sync::Arc<std::sync::Mutex<DropCounterInner>>,
}

impl DropCounter {
    fn new() -> DropCounter {
        DropCounter {
            inner: std::sync::Arc::new(std::sync::Mutex::new(
                DropCounterInner {
                    count: std::collections::HashMap::new(),
                },
            )),
        }
    }

    fn register_drop(&self, id: usize) {
        let mut guard = self.inner.lock().unwrap();
        *guard.count.entry(id).or_insert(0) += 1;
    }

    fn clear(&self) {
        let mut guard = self.inner.lock().unwrap();
        guard.count.clear();
    }

    fn drops(&self, id: usize) -> usize {
        let guard = self.inner.lock().unwrap();
        *(guard.count.get(&id).unwrap_or(&0))
    }
}

#[derive(Debug, PartialEq)]
struct CountedElemInner {
    id: usize,
}

#[derive(Debug, Clone)]
struct CountedElem {
    drop_counter: DropCounter,
    inner: std::sync::Arc<std::sync::Mutex<CountedElemInner>>,
}

impl PartialEq for CountedElem {
    fn eq(&self, other: &CountedElem) -> bool {
        self.id() == other.id()
    }
}

impl CountedElem {
    fn new(id: usize, drop_counter: DropCounter) -> CountedElem {
        CountedElem {
            inner: std::sync::Arc::new(std::sync::Mutex::new(
                CountedElemInner { id },
            )),
            drop_counter,
        }
    }

    fn id(&self) -> usize {
        let guard = self.inner.lock().unwrap();
        guard.id
    }

    fn strong_count(&self) -> usize {
        std::sync::Arc::strong_count(&self.inner)
    }
}

impl Drop for CountedElem {
    fn drop(&mut self) {
        self.drop_counter.register_drop(self.id())
    }
}

fn assert_send_done<FutureType, T>(
    cx: &mut Context,
    send_fut: &mut core::pin::Pin<&mut FutureType>,
    expected: Result<(), ChannelSendError<T>>,
) where
    FutureType: Future<Output = Result<(), ChannelSendError<T>>> + FusedFuture,
    T: PartialEq + core::fmt::Debug,
{
    match send_fut.as_mut().poll(cx) {
        Poll::Pending => panic!("future is not ready"),
        Poll::Ready(res) => {
            if res != expected {
                panic!("Unexpected send result: {:?}", res);
            }
        }
    };
    assert!(send_fut.as_mut().is_terminated());
}

fn assert_next_done<S, T>(
    cx: &mut Context,
    stream_fut: &mut core::pin::Pin<&mut S>,
    value: Option<T>,
) where
    S: Stream<Item = T>,
    T: PartialEq + core::fmt::Debug,
{
    match stream_fut.as_mut().poll_next(cx) {
        Poll::Pending => panic!("future is not ready"),
        Poll::Ready(res) => {
            if res != value {
                panic!("Unexpected value {:?}", res);
            }
        }
    };
}

// A stream future shouldn't terminate until the stream
// terminates.
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

macro_rules! gen_mpmc_tests {
    ($mod_name:ident, $channel_type:ident, $unbuffered_channel_type:ident) => {
        mod $mod_name {
            use super::*;

            type ChannelType = $channel_type<i32, [i32; 3]>;
            type UnbufferedChannelType = $unbuffered_channel_type<i32>;

            fn assert_send(
                cx: &mut Context,
                channel: &ChannelType,
                value: i32,
            ) {
                let send_fut = channel.send(value);
                pin_mut!(send_fut);
                assert!(!send_fut.as_mut().is_terminated());

                assert_send_done(cx, &mut send_fut, Ok(()));
            }

            macro_rules! assert_receive {
                ($cx:ident, $channel:expr, $value: expr) => {
                    let receive_fut = $channel.receive();
                    pin_mut!(receive_fut);
                    assert!(!receive_fut.as_mut().is_terminated());

                    assert_receive_done($cx, &mut receive_fut, $value);
                };
            }

            #[test]
            fn send_on_closed_channel() {
                let channel = ChannelType::new();
                let waker = &panic_waker();
                let cx = &mut Context::from_waker(&waker);

                assert!(channel.close().is_newly_closed());
                let fut = channel.send(5);
                pin_mut!(fut);
                assert_send_done(cx, &mut fut, Err(ChannelSendError(5)));
            }

            #[test]
            fn unbuffered_try_receive() {
                let channel = UnbufferedChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let fut = channel.send(5);
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                channel.try_receive().unwrap();

                assert_eq!(count, 1);
                assert_send_done(cx, &mut fut, Ok(()));
            }

            #[test]
            fn try_send_recv_smoke_test() {
                let channel = ChannelType::with_capacity(3);

                for _ in 0..3 {
                    channel.try_send(5).unwrap();
                }

                channel.try_send(5).unwrap_err();

                for _ in 0..3 {
                    channel.try_receive().unwrap();
                }

                let err = channel.try_receive().unwrap_err();
                assert!(err.is_empty());
            }

            #[test]
            fn close_state() {
                let channel = ChannelType::with_capacity(3);

                assert!(channel.close().is_newly_closed());
                assert!(channel.close().is_already_closed());
                assert!(channel.close().is_already_closed());
                assert!(channel.close().is_already_closed());
            }

            #[test]
            fn try_send_full_channel() {
                let channel = ChannelType::with_capacity(3);

                for _ in 0..3 {
                    channel.try_send(5).unwrap();
                }

                let err = channel.try_send(5).unwrap_err();
                assert!(err.is_full());
            }

            #[test]
            fn try_send_on_closed_channel() {
                let channel = ChannelType::new();

                assert!(channel.close().is_newly_closed());

                let err = channel.try_send(5).unwrap_err();
                assert!(err.is_closed());
            }

            #[test]
            fn try_receive_empty_channel() {
                let channel = ChannelType::with_capacity(3);

                let err = channel.try_receive().unwrap_err();
                assert!(err.is_empty());
            }

            #[test]
            fn try_recv_on_closed_channel() {
                let channel = ChannelType::new();

                channel.try_send(5).unwrap();

                assert!(channel.close().is_newly_closed());

                channel.try_receive().unwrap();
                let err = channel.try_receive().unwrap_err();
                assert!(err.is_closed());
            }

            #[test]
            #[should_panic]
            fn try_send_unbuffered_panics() {
                let channel = UnbufferedChannelType::new();
                let _ = channel.try_send(5);
            }

            #[test]
            fn buffered_close_unblocks_send() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                // Fill the channel
                assert_send(cx, &channel, 5);
                assert_send(cx, &channel, 6);
                assert_send(cx, &channel, 7);

                let fut = channel.send(8);
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_pending());
                let fut2 = channel.send(9);
                pin_mut!(fut2);
                assert!(fut2.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                assert!(channel.close().is_newly_closed());
                assert_eq!(count, 2);
                assert_send_done(cx, &mut fut, Err(ChannelSendError(8)));
                assert_send_done(cx, &mut fut2, Err(ChannelSendError(9)));
            }

            #[test]
            fn unbuffered_close_unblocks_send() {
                let channel = UnbufferedChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let fut = channel.send(8);
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_pending());
                let fut2 = channel.send(9);
                pin_mut!(fut2);
                assert!(fut2.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                assert!(channel.close().is_newly_closed());
                assert_eq!(count, 2);
                assert_send_done(cx, &mut fut, Err(ChannelSendError(8)));
                assert_send_done(cx, &mut fut2, Err(ChannelSendError(9)));
            }

            #[test]
            fn close_unblocks_receive() {
                let channel = ChannelType::new();
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
                let channel = ChannelType::new();
                let waker = &panic_waker();
                let cx = &mut Context::from_waker(&waker);

                assert_send(cx, &channel, 1);
                assert_send(cx, &channel, 2);
                assert_receive!(cx, &channel, Some(1));
                assert_receive!(cx, &channel, Some(2));

                assert_send(cx, &channel, 5);
                assert_send(cx, &channel, 6);
                assert_send(cx, &channel, 7);
                assert!(channel.close().is_newly_closed());
                assert_receive!(cx, &channel, Some(5));
                assert_receive!(cx, &channel, Some(6));
                assert_receive!(cx, &channel, Some(7));
                assert_receive!(cx, &channel, None);
            }

            #[test]
            fn buffered_send_unblocks_receive() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let fut = channel.receive();
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                let fut2 = channel.receive();
                pin_mut!(fut2);
                assert!(fut2.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                assert_send(cx, &channel, 99);
                assert_eq!(count, 1);
                assert_receive_done(cx, &mut fut, Some(99));

                assert!(fut2.as_mut().poll(cx).is_pending());
                assert_send(cx, &channel, 111);
                assert_eq!(count, 2);
                assert_receive_done(cx, &mut fut2, Some(111));
            }

            #[test]
            fn unbuffered_send_unblocks_receive() {
                let channel = UnbufferedChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let fut = channel.receive();
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                let fut2 = channel.receive();
                pin_mut!(fut2);
                assert!(fut2.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                // In the unbuffered send case the send can't complete from the
                // Sender point-of-view until a receiver actively catches the
                // values. Therefore this returns pending.
                let futs1 = channel.send(99);
                let futs2 = channel.send(111);
                pin_mut!(futs1, futs2);
                assert!(futs1.as_mut().poll(cx).is_pending());

                assert_eq!(count, 1);
                assert_receive_done(cx, &mut fut, Some(99));
                assert_eq!(count, 2);
                assert_send_done(cx, &mut futs1, Ok(()));

                assert!(fut2.as_mut().poll(cx).is_pending());
                assert!(futs2.as_mut().poll(cx).is_pending());
                assert_eq!(count, 3);
                assert_receive_done(cx, &mut fut2, Some(111));
                assert_eq!(count, 4);
                assert_send_done(cx, &mut futs2, Ok(()));
            }

            #[test]
            fn buffered_receive_unblocks_send() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                // Fill the channel
                assert_send(cx, &channel, 1);
                assert_send(cx, &channel, 2);
                assert_send(cx, &channel, 3);

                let fut = channel.send(4);
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_pending());
                let fut2 = channel.send(5);
                pin_mut!(fut2);
                assert!(fut2.as_mut().poll(cx).is_pending());

                assert_eq!(count, 0);
                assert_receive!(cx, &channel, Some(1));
                assert_eq!(count, 1);

                assert_send_done(cx, &mut fut, Ok(()));
                assert!(fut.is_terminated());
                assert!(fut2.as_mut().poll(cx).is_pending());

                assert_receive!(cx, &channel, Some(2));
                assert_eq!(count, 2);
                assert_send_done(cx, &mut fut2, Ok(()));
                assert!(fut2.is_terminated());
            }

            #[test]
            fn unbuffered_receive_unblocks_send() {
                let channel = UnbufferedChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let fut = channel.send(4);
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_pending());
                let fut2 = channel.send(5);
                pin_mut!(fut2);
                assert!(fut2.as_mut().poll(cx).is_pending());

                assert_eq!(count, 0);
                assert_receive!(cx, &channel, Some(4));
                assert_eq!(count, 1);

                assert_send_done(cx, &mut fut, Ok(()));
                assert!(fut.is_terminated());
                assert!(fut2.as_mut().poll(cx).is_pending());

                assert_receive!(cx, &channel, Some(5));
                assert_eq!(count, 2);
                assert_send_done(cx, &mut fut2, Ok(()));
                assert!(fut2.is_terminated());
            }

            #[test]
            fn cancel_send_mid_wait() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                assert_send(cx, &channel, 5);
                assert_send(cx, &channel, 6);
                assert_send(cx, &channel, 7);

                {
                    // Cancel a wait in between other waits
                    // In order to arbitrarily drop a non movable future we have to box and pin it
                    let mut poll1 = Box::pin(channel.send(8));
                    let mut poll2 = Box::pin(channel.send(9));
                    let mut poll3 = Box::pin(channel.send(10));
                    let mut poll4 = Box::pin(channel.send(11));
                    let mut poll5 = Box::pin(channel.send(12));

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

                    assert_receive!(cx, &channel, Some(5));
                    assert_eq!(count, 1);
                    assert_send_done(cx, &mut poll1.as_mut(), Ok(()));
                    assert!(poll3.as_mut().poll(cx).is_pending());
                    assert!(poll5.as_mut().poll(cx).is_pending());

                    assert_receive!(cx, &channel, Some(6));
                    assert_receive!(cx, &channel, Some(7));
                    assert_eq!(count, 3);
                    assert_send_done(cx, &mut poll3.as_mut(), Ok(()));
                    assert_send_done(cx, &mut poll5.as_mut(), Ok(()));
                }

                assert_eq!(count, 3);
            }

            #[test]
            fn cancel_send_end_wait() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                assert_send(cx, &channel, 100);
                assert_send(cx, &channel, 101);
                assert_send(cx, &channel, 102);

                let poll1 = channel.send(1);
                let poll2 = channel.send(2);
                let poll3 = channel.send(3);
                let poll4 = channel.send(4);

                pin_mut!(poll1);
                pin_mut!(poll2);
                pin_mut!(poll3);
                pin_mut!(poll4);

                assert!(poll1.as_mut().poll(cx).is_pending());
                assert!(poll2.as_mut().poll(cx).is_pending());

                // Start polling some wait handles which get cancelled
                // before new ones are attached
                {
                    let poll5 = channel.send(5);
                    let poll6 = channel.send(6);
                    pin_mut!(poll5);
                    pin_mut!(poll6);
                    assert!(poll5.as_mut().poll(cx).is_pending());
                    assert!(poll6.as_mut().poll(cx).is_pending());
                }

                assert!(poll3.as_mut().poll(cx).is_pending());
                assert!(poll4.as_mut().poll(cx).is_pending());

                assert_receive!(cx, &channel, Some(100));
                assert_receive!(cx, &channel, Some(101));
                assert_receive!(cx, &channel, Some(102));

                assert_send_done(cx, &mut poll1, Ok(()));
                assert_send_done(cx, &mut poll2, Ok(()));
                assert_send_done(cx, &mut poll3, Ok(()));

                assert!(channel.close().is_newly_closed());
                assert_receive!(cx, &channel, Some(1));
                assert_receive!(cx, &channel, Some(2));
                assert_receive!(cx, &channel, Some(3));
                assert_send_done(cx, &mut poll4, Err(ChannelSendError(4)));

                assert_eq!(count, 4);
            }

            #[test]
            fn cancel_receive_mid_wait() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                {
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

                    assert_send(cx, &channel, 1);
                    assert_eq!(count, 1);
                    assert_receive_done(cx, &mut poll1.as_mut(), Some(1));
                    assert!(poll3.as_mut().poll(cx).is_pending());
                    assert!(poll5.as_mut().poll(cx).is_pending());

                    assert_send(cx, &channel, 2);
                    assert_send(cx, &channel, 3);
                    assert_eq!(count, 3);
                    assert_receive_done(cx, &mut poll3.as_mut(), Some(2));
                    assert_receive_done(cx, &mut poll5.as_mut(), Some(3));
                }

                assert_eq!(count, 3);
            }

            #[test]
            fn cancel_receive_end_wait() {
                let channel = ChannelType::new();
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

                assert_send(cx, &channel, 0);
                assert_send(cx, &channel, 1);
                assert_send(cx, &channel, 2);

                assert_receive_done(cx, &mut poll1, Some(0));
                assert_receive_done(cx, &mut poll2, Some(1));
                assert_receive_done(cx, &mut poll3, Some(2));

                assert_send(cx, &channel, 3);
                assert_receive_done(cx, &mut poll4, Some(3));

                assert_eq!(count, 4);
            }

            #[test]
            fn drops_unread_elements() {
                let (waker, _) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let drop_counter = DropCounter::new();

                let elem1 = CountedElem::new(1, drop_counter.clone());
                let elem2 = CountedElem::new(2, drop_counter.clone());
                let elem3 = CountedElem::new(3, drop_counter.clone());

                {
                    let channel =
                        $channel_type::<CountedElem, [CountedElem; 3]>::new();

                    // Fill the channel
                    let fut1 = channel.send(elem1.clone());
                    let fut2 = channel.send(elem2.clone());
                    let fut3 = channel.send(elem3.clone());

                    assert_eq!(2, elem1.strong_count());
                    assert_eq!(2, elem2.strong_count());
                    assert_eq!(2, elem3.strong_count());

                    pin_mut!(fut1, fut2, fut3);
                    assert_send_done(cx, &mut fut1, Ok(()));
                    assert_send_done(cx, &mut fut2, Ok(()));
                    assert_send_done(cx, &mut fut3, Ok(()));

                    assert_eq!(2, elem1.strong_count());
                    assert_eq!(2, elem2.strong_count());
                    assert_eq!(2, elem3.strong_count());
                }

                assert_eq!(1, drop_counter.drops(1));
                assert_eq!(1, drop_counter.drops(2));
                assert_eq!(1, drop_counter.drops(3));

                assert_eq!(1, elem1.strong_count());
                assert_eq!(1, elem2.strong_count());
                assert_eq!(1, elem3.strong_count());

                drop_counter.clear();

                {
                    let channel =
                        $channel_type::<CountedElem, [CountedElem; 3]>::new();

                    // Fill the channel
                    let fut1 = channel.send(elem1.clone());
                    let fut2 = channel.send(elem2.clone());

                    let futr1 = channel.receive();
                    let futr2 = channel.receive();
                    pin_mut!(fut1, fut2, futr1, futr2);
                    assert_send_done(cx, &mut fut1, Ok(()));
                    assert_send_done(cx, &mut fut2, Ok(()));

                    let fut3 = channel.send(elem3.clone());
                    let fut4 = channel.send(elem2.clone());
                    pin_mut!(fut3, fut4);
                    assert_receive_done(cx, &mut futr1, Some(elem1.clone()));
                    assert_receive_done(cx, &mut futr2, Some(elem2.clone()));

                    assert_eq!(1, elem1.strong_count());
                    assert_eq!(2, elem2.strong_count());

                    assert_send_done(cx, &mut fut3, Ok(()));
                    assert_send_done(cx, &mut fut4, Ok(()));

                    assert_eq!(1, elem1.strong_count());
                    assert_eq!(2, elem2.strong_count());
                    assert_eq!(2, elem3.strong_count());

                    // 1 and 2 are dropped twice, since we create a copy
                    // through Option<T>
                    assert_eq!(2, drop_counter.drops(1));
                    assert_eq!(2, drop_counter.drops(2));
                    assert_eq!(0, drop_counter.drops(3));

                    drop_counter.clear();
                }

                assert_eq!(0, drop_counter.drops(1));
                assert_eq!(1, drop_counter.drops(2));
                assert_eq!(1, drop_counter.drops(3));

                assert_eq!(1, elem1.strong_count());
                assert_eq!(1, elem2.strong_count());
                assert_eq!(1, elem3.strong_count());
            }

            #[test]
            fn poll_from_multiple_executors_on_receive() {
                let (waker_1, count_1) = new_count_waker();
                let (waker_2, count_2) = new_count_waker();
                let channel = ChannelType::new();

                let cx_1 = &mut Context::from_waker(&waker_1);
                let cx_2 = &mut Context::from_waker(&waker_2);

                let fut = channel.receive();
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx_1).is_pending());
                assert!(fut.as_mut().poll(cx_2).is_pending());

                assert_send(cx_1, &channel, 99);
                assert_eq!(count_1, 0);
                assert_eq!(count_2, 1);

                assert_receive_done(cx_2, &mut fut, Some(99));
            }

            #[test]
            fn poll_from_multiple_executors_on_send() {
                let (waker_1, count_1) = new_count_waker();
                let (waker_2, count_2) = new_count_waker();
                let cx_1 = &mut Context::from_waker(&waker_1);
                let cx_2 = &mut Context::from_waker(&waker_2);

                let channel = ChannelType::new();

                // Fill the channel, so that send blocks
                assert_send(cx_1, &channel, 1);
                assert_send(cx_1, &channel, 2);
                assert_send(cx_1, &channel, 3);

                let fut = channel.send(4);
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx_1).is_pending());
                assert!(fut.as_mut().poll(cx_2).is_pending());

                assert_receive!(cx_2, &channel, Some(1));
                assert_eq!(count_2, 1);
                assert_eq!(count_1, 0);

                assert_send_done(cx_2, &mut fut, Ok(()));
            }

            #[test]
            fn buffered_starved_recv_does_not_deadlock() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                // Create enough futures to starve the permits.
                let recv_fut1 = channel.receive();
                let recv_fut2 = channel.receive();
                let recv_fut3 = channel.receive();
                let recv_fut4 = channel.receive();
                let recv_fut5 = channel.receive();
                pin_mut!(recv_fut1, recv_fut2, recv_fut3, recv_fut4, recv_fut5);
                assert!(recv_fut1.as_mut().poll(cx).is_pending());
                assert!(recv_fut2.as_mut().poll(cx).is_pending());
                assert!(recv_fut3.as_mut().poll(cx).is_pending());
                assert!(recv_fut3.as_mut().poll(cx).is_pending());
                assert!(recv_fut3.as_mut().poll(cx).is_pending());
                assert_eq!(count, 0);

                // Now send & recv while being starved.
                assert_send(cx, &channel, 1);
                assert_eq!(count, 1);
                assert_send(cx, &channel, 2);
                assert_eq!(count, 2);
                assert_send(cx, &channel, 3);
                assert_eq!(count, 3);

                let send_fut4 = channel.send(4);
                let send_fut5 = channel.send(5);
                pin_mut!(send_fut4, send_fut5);

                assert!(send_fut4.as_mut().poll(cx).is_pending());
                assert!(send_fut5.as_mut().poll(cx).is_pending());

                let recv_fut6 = channel.receive();
                let recv_fut7 = channel.receive();
                let recv_fut8 = channel.receive();
                let recv_fut9 = channel.receive();
                let recv_fut10 = channel.receive();
                pin_mut!(
                    recv_fut6, recv_fut7, recv_fut8, recv_fut9, recv_fut10
                );

                // Grab the buffered data.
                assert_receive_done(cx, &mut recv_fut6, Some(1));
                assert_eq!(count, 4);
                assert_receive_done(cx, &mut recv_fut7, Some(2));
                assert_eq!(count, 5);
                assert_receive_done(cx, &mut recv_fut8, Some(3));

                // Grab the pending data.
                assert_receive_done(cx, &mut recv_fut9, Some(4));
                assert_receive_done(cx, &mut recv_fut10, Some(5));
                assert_eq!(count, 5);

                // Do one last send & recv.
                let recv_fut11 = channel.receive();
                pin_mut!(recv_fut11);
                assert!(recv_fut11.as_mut().poll(cx).is_pending());

                assert_send(cx, &channel, 6);
                assert_receive_done(cx, &mut recv_fut11, Some(6));
                assert_eq!(count, 6);

                // Now resolve the starved futures.
                assert_send(cx, &channel, 7);
                assert_receive_done(cx, &mut recv_fut1, Some(7));
                assert_send(cx, &channel, 8);
                assert_receive_done(cx, &mut recv_fut2, Some(8));
                assert_send(cx, &channel, 9);
                assert_receive_done(cx, &mut recv_fut3, Some(9));
                assert_send(cx, &channel, 10);
                assert_receive_done(cx, &mut recv_fut4, Some(10));
                assert_send(cx, &channel, 11);
                assert_receive_done(cx, &mut recv_fut5, Some(11));
                assert_eq!(count, 6);
            }

            #[test]
            fn unbuffered_starved_send_does_not_deadlock() {
                let channel = UnbufferedChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                // Create enough futures to starve the permits.
                let send_fut1 = channel.send(99);
                let send_fut2 = channel.send(111);
                let send_fut3 = channel.send(69);
                pin_mut!(send_fut1, send_fut2, send_fut3);
                assert!(send_fut1.as_mut().poll(cx).is_pending());
                assert!(send_fut2.as_mut().poll(cx).is_pending());
                assert!(send_fut3.as_mut().poll(cx).is_pending());

                assert_eq!(count, 0);

                // Now send & recv while being starved.
                let recv_fut = channel.receive();
                pin_mut!(recv_fut);
                assert_receive_done(cx, &mut recv_fut, Some(99));
                assert_eq!(count, 1);

                let recv_fut = channel.receive();
                pin_mut!(recv_fut);
                assert_receive_done(cx, &mut recv_fut, Some(111));
                assert_eq!(count, 2);

                let recv_fut = channel.receive();
                pin_mut!(recv_fut);
                assert_receive_done(cx, &mut recv_fut, Some(69));
                assert_eq!(count, 3);

                assert_send_done(cx, &mut send_fut1, Ok(()));
                assert_send_done(cx, &mut send_fut2, Ok(()));
                assert_send_done(cx, &mut send_fut3, Ok(()));
            }

            #[test]
            fn buffered_stream_smoke_test() {
                let channel = ChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let stream = channel.stream();
                pin_mut!(stream);

                // Receive after send is immediately ready.
                for i in 0..10 {
                    let send_fut = channel.send(i);
                    pin_mut!(send_fut);

                    assert_send_done(cx, &mut send_fut, Ok(()));

                    assert_next_done(cx, &mut stream, Some(i));

                    // The should be not wakups since its immediate.
                    assert_eq!(count, 0);
                }

                // Send after receive is immediately ready.
                for i in 0..10 {
                    assert!(stream.as_mut().poll_next(cx).is_pending());
                    assert_eq!(count, i as usize);

                    assert_send(cx, &channel, i);

                    assert_next_done(cx, &mut stream, Some(i));
                    assert_eq!(count, i as usize + 1);
                }

                // This should block.
                assert!(stream.as_mut().poll_next(cx).is_pending());

                // This should terminate the stream.
                assert!(channel.close().is_newly_closed());

                // This should unblock.
                assert_next_done(cx, &mut stream, None);
                assert_eq!(count, 11);
                assert!(stream.is_terminated());

                // Future calls should all return `None`.
                for _ in 0..10 {
                    assert_next_done(cx, &mut stream, None);
                    assert_eq!(count, 11);
                }
            }

            #[test]
            fn unbuffered_stream_smoke_test() {
                let channel = UnbufferedChannelType::new();
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let stream = channel.stream();
                pin_mut!(stream);

                // Send waits for a receive.
                for i in 0..10 {
                    let send_fut = channel.send(i);
                    pin_mut!(send_fut);
                    assert!(send_fut.as_mut().poll(cx).is_pending());

                    assert_next_done(cx, &mut stream, Some(i));
                    assert_send_done(cx, &mut send_fut, Ok(()));

                    assert_eq!(count, i as usize + 1);
                }

                // Receive waits for a send.
                for i in 0..10 {
                    assert!(stream.as_mut().poll_next(cx).is_pending());
                    assert_eq!(count, 10 + i as usize * 2);

                    let send_fut = channel.send(i);
                    pin_mut!(send_fut);
                    assert!(send_fut.as_mut().poll(cx).is_pending());

                    assert_next_done(cx, &mut stream, Some(i));
                    assert_send_done(cx, &mut send_fut, Ok(()));
                    assert_eq!(count, 10 + i as usize * 2 + 2);
                }

                // This should block.
                assert!(stream.as_mut().poll_next(cx).is_pending());

                // This should terminate the stream.
                assert!(channel.close().is_newly_closed());

                // This should unblock.
                assert_next_done(cx, &mut stream, None);
                assert_eq!(count, 31);
                assert!(stream.is_terminated());

                // Future calls should all return `None`.
                for _ in 0..10 {
                    assert_next_done(cx, &mut stream, None);
                    assert_eq!(count, 31);
                }
            }
        }
    };
}

gen_mpmc_tests!(
    local_mpmc_channel_tests,
    LocalChannel,
    LocalUnbufferedChannel
);

#[cfg(feature = "std")]
mod if_std {
    use super::*;
    use futures_intrusive::channel::{
        shared::channel, shared::ChannelReceiveFuture,
        shared::ChannelSendFuture, shared::Receiver, shared::Sender, Channel,
        UnbufferedChannel,
    };

    gen_mpmc_tests!(mpmc_channel_tests, Channel, UnbufferedChannel);

    macro_rules! assert_receive {
        ($cx:ident, $receiver:expr, $value: expr) => {
            let receive_fut = $receiver.receive();
            pin_mut!(receive_fut);
            assert!(!receive_fut.as_mut().is_terminated());

            assert_receive_done($cx, &mut receive_fut, $value);
        };
    }

    macro_rules! assert_send {
        ($cx:ident, $sender:expr, $value: expr) => {
            let send_fut = $sender.send($value);
            pin_mut!(send_fut);
            assert!(!send_fut.as_mut().is_terminated());

            assert_send_done($cx, &mut send_fut, Ok(()));
        };
    }

    fn is_send<T: Send>(_: &T) {}

    fn is_send_value<T: Send>(_: T) {}

    fn is_sync<T: Sync>(_: &T) {}

    #[test]
    fn channel_futures_are_send() {
        let channel = Channel::<i32, [i32; 3]>::new();
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
        let (sender, receiver) = channel::<i32>(1);
        is_sync(&sender);
        is_sync(&receiver);
        is_send_value(sender.clone());
        is_send_value(receiver.clone());
        let recv_fut = receiver.receive();
        is_send(&recv_fut);
        pin_mut!(recv_fut);
        is_send(&recv_fut);
        let send_fut = sender.send(3);
        is_send(&send_fut);
        pin_mut!(send_fut);
        is_send(&send_fut);
    }

    // Check if SharedChannel can be used in traits
    pub trait StreamTrait {
        type Output;
        type Next: Future<Output = Self::Output>;

        fn next(&self) -> Self::Next;
    }

    pub trait Sink {
        type Input;
        type Error;
        type Next: Future<Output = Result<(), Self::Error>>;

        fn send(&self, value: Self::Input) -> Self::Next;
    }

    impl<T> StreamTrait for Receiver<T>
    where
        T: 'static,
    {
        type Output = Option<T>;
        type Next = ChannelReceiveFuture<parking_lot::RawMutex, T>;

        fn next(&self) -> Self::Next {
            self.receive()
        }
    }

    impl<T> Sink for Sender<T>
    where
        T: 'static,
    {
        type Input = T;
        type Error = ChannelSendError<T>;
        type Next = ChannelSendFuture<parking_lot::RawMutex, T>;

        fn send(&self, value: T) -> Self::Next {
            self.send(value)
        }
    }

    async fn send_stream<S: Sink<Input = i32>>(stream: &S, value: i32) -> () {
        assert!(stream.send(value).await.is_ok());
    }

    async fn read_stream<S: StreamTrait<Output = Option<i32>>>(
        stream: &S,
    ) -> Option<i32> {
        stream.next().await
    }

    #[test]
    fn shared_channel_can_be_used_in_trait() {
        let (waker, _) = new_count_waker();
        let cx = &mut Context::from_waker(&waker);

        let (sender, receiver) = channel::<i32>(5);

        let stream = || async move {
            send_stream(&sender, 2).await;
            send_stream(&sender, 7).await;
            assert!(sender.close().is_newly_closed());
        };

        let drain = || async move {
            let mut sum = 0;
            loop {
                match read_stream(&receiver).await {
                    None => return sum,
                    Some(v) => sum += v,
                }
            }
        };

        let stream_fut = stream();
        pin_mut!(stream_fut);
        let drain_fut = drain();
        pin_mut!(drain_fut);

        let mut do_drain = true;
        let mut do_stream = true;
        while do_drain || do_stream {
            if do_stream {
                match stream_fut.as_mut().poll(cx) {
                    Poll::Ready(_) => {
                        do_stream = false;
                    }
                    Poll::Pending => {
                        if !do_drain {
                            panic!("Expected channel to be closed");
                        }
                    }
                }
            }
            if do_drain {
                match drain_fut.as_mut().poll(cx) {
                    Poll::Ready(res) => {
                        assert_eq!(9, res);
                        do_drain = false;
                    }
                    Poll::Pending => {}
                }
            }
        }
    }

    #[test]
    fn try_send_recv_smoke_test() {
        let (sender, receiver) = channel::<i32>(3);

        for _ in 0..3 {
            sender.try_send(5).unwrap();
        }

        sender.try_send(5).unwrap_err();

        for _ in 0..3 {
            receiver.try_receive().unwrap();
        }

        let err = receiver.try_receive().unwrap_err();
        assert!(err.is_empty());
    }

    #[test]
    fn try_send_full_channel() {
        let (sender, _receiver) = channel::<i32>(3);

        for _ in 0..3 {
            sender.try_send(5).unwrap();
        }

        let err = sender.try_send(5).unwrap_err();
        assert!(err.is_full());
    }

    #[test]
    fn try_send_on_closed_channel() {
        let (sender, receiver) = channel::<i32>(3);

        assert!(receiver.close().is_newly_closed());
        let err = sender.try_send(5).unwrap_err();
        assert!(err.is_closed());
    }

    #[test]
    fn try_receive_empty_channel() {
        let (_sender, receiver) = channel::<i32>(3);

        let err = receiver.try_receive().unwrap_err();
        assert!(err.is_empty());
    }

    #[test]
    fn try_recv_on_closed_channel() {
        let (sender, receiver) = channel::<i32>(3);

        sender.try_send(5).unwrap();

        assert!(sender.close().is_newly_closed());

        receiver.try_receive().unwrap();
        let err = receiver.try_receive().unwrap_err();
        assert!(err.is_closed());
    }

    #[test]
    fn dropping_shared_channel_receivers_but_not_senders_drops_content() {
        use std::sync::{
            atomic::{AtomicU64, Ordering},
            Arc,
        };
        #[derive(Debug, Clone)]
        struct Count(Arc<AtomicU64>);

        impl Count {
            fn new() -> Self {
                Self(Arc::new(AtomicU64::new(0)))
            }

            fn load(&self) -> u64 {
                self.0.load(Ordering::Acquire)
            }
        }

        impl Drop for Count {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }

        let count = Count::new();
        assert_eq!(count.load(), 0);
        let (sender, receiver) = channel::<Count>(10);
        sender.try_send(count.clone()).unwrap();
        sender.try_send(count.clone()).unwrap();
        sender.try_send(count.clone()).unwrap();

        drop(receiver);
        assert_eq!(count.load(), 3);
    }

    #[test]
    fn dropping_shared_channel_senders_closes_channel() {
        let (waker, _) = new_count_waker();
        let cx = &mut Context::from_waker(&waker);

        let (sender, receiver) = channel::<i32>(5);
        let sender2 = sender.clone();
        let receiver2 = receiver.clone();

        let fut = receiver.receive();
        pin_mut!(fut);
        assert!(fut.as_mut().poll(cx).is_pending());
        let fut2 = receiver2.receive();
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
        let (waker, _) = new_count_waker();
        let cx = &mut Context::from_waker(&waker);

        let (sender, receiver) = channel::<i32>(3);
        let sender2 = sender.clone();
        let receiver2 = receiver.clone();

        // Fill the channel
        for _ in 0..3 {
            let send_fut = sender.send(3);
            pin_mut!(send_fut);
            match send_fut.as_mut().poll(cx) {
                Poll::Pending => panic!("future is not ready"),
                Poll::Ready(res) => {
                    if res.is_err() {
                        panic!("Unexpected send result: {:?}", res);
                    }
                }
            };
        }

        let fut = sender.send(27);
        pin_mut!(fut);
        assert!(fut.as_mut().poll(cx).is_pending());
        let fut2 = sender2.send(49);
        pin_mut!(fut2);
        assert!(fut2.as_mut().poll(cx).is_pending());

        drop(receiver);
        assert!(fut.as_mut().poll(cx).is_pending());
        assert!(fut2.as_mut().poll(cx).is_pending());

        drop(receiver2);
        match fut.as_mut().poll(cx) {
            Poll::Ready(Err(ChannelSendError(27))) => {}
            Poll::Ready(v) => panic!("Unexpected value {:?}", v),
            Poll::Pending => panic!("Expected channel to be closed"),
        }
        match fut2.as_mut().poll(cx) {
            Poll::Ready(Err(ChannelSendError(49))) => {}
            Poll::Ready(v) => panic!("Unexpected value {:?}", v),
            Poll::Pending => panic!("Expected channel to be closed"),
        }
    }

    #[test]
    fn shared_stream_smoke_test() {
        let (sender, receiver) = channel::<i32>(3);
        let (waker, count) = new_count_waker();
        let cx = &mut Context::from_waker(&waker);

        let stream = receiver.into_stream();
        pin_mut!(stream);

        // Receive after send is immediately ready.
        for i in 0..10 {
            let send_fut = sender.send(i);
            pin_mut!(send_fut);

            assert_send_done(cx, &mut send_fut, Ok(()));

            assert_next_done(cx, &mut stream, Some(i));

            // The should be not wakups since its immediate.
            assert_eq!(count, 0);
        }

        // Send after receive is immediately ready.
        for i in 0..10 {
            assert!(stream.as_mut().poll_next(cx).is_pending());
            assert_eq!(count, i as usize);

            let send_fut = sender.send(i);
            pin_mut!(send_fut);
            assert_send_done(cx, &mut send_fut, Ok(()));

            assert_next_done(cx, &mut stream, Some(i));
            assert_eq!(count, i as usize + 1);
        }

        // This should block.
        assert!(stream.as_mut().poll_next(cx).is_pending());

        // This should terminate the stream.
        assert!(stream.close().is_newly_closed());

        // This should unblock.
        assert_next_done(cx, &mut stream, None);
        assert_eq!(count, 11);
        assert!(stream.is_terminated());

        // Future calls should all return `None`.
        for _ in 0..10 {
            assert_next_done(cx, &mut stream, None);
            assert_eq!(count, 11);
        }
    }

    #[test]
    fn cancel_send_mid_wait() {
        let (sender, receiver) = channel::<i32>(3);
        let (waker, count) = new_count_waker();
        let cx = &mut Context::from_waker(&waker);

        assert_send!(cx, &sender, 5);
        assert_send!(cx, &sender, 6);
        assert_send!(cx, &sender, 7);

        {
            // Cancel a wait in between other waits
            // In order to arbitrarily drop a non movable future we have to box and pin it
            let mut poll1 = Box::pin(sender.send(8));
            let mut poll2 = Box::pin(sender.send(9));
            let mut poll3 = Box::pin(sender.send(10));
            let mut poll4 = Box::pin(sender.send(11));
            let mut poll5 = Box::pin(sender.send(12));

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
            // Safety: We are not using these pins again so this is safe.
            unsafe { core::pin::Pin::into_inner_unchecked(poll2).cancel() };
            unsafe { core::pin::Pin::into_inner_unchecked(poll4).cancel() };

            assert!(poll1.as_mut().poll(cx).is_pending());
            assert!(poll3.as_mut().poll(cx).is_pending());
            assert!(poll5.as_mut().poll(cx).is_pending());

            assert_receive!(cx, &receiver, Some(5));

            assert_eq!(count, 1);
            assert_send_done(cx, &mut poll1.as_mut(), Ok(()));
            assert!(poll3.as_mut().poll(cx).is_pending());
            assert!(poll5.as_mut().poll(cx).is_pending());

            assert_receive!(cx, &receiver, Some(6));
            assert_receive!(cx, &receiver, Some(7));

            assert_eq!(count, 3);
            assert_send_done(cx, &mut poll3.as_mut(), Ok(()));
            assert_send_done(cx, &mut poll5.as_mut(), Ok(()));
        }

        assert_eq!(count, 3);
    }

    #[test]
    fn cancel_send_end_wait() {
        let (sender, receiver) = channel::<i32>(3);
        let (waker, count) = new_count_waker();
        let cx = &mut Context::from_waker(&waker);

        assert_send!(cx, &sender, 100);
        assert_send!(cx, &sender, 101);
        assert_send!(cx, &sender, 102);

        let poll1 = sender.send(1);
        let poll2 = sender.send(2);
        let poll3 = sender.send(3);
        let poll4 = sender.send(4);

        pin_mut!(poll1);
        pin_mut!(poll2);
        pin_mut!(poll3);
        pin_mut!(poll4);

        assert!(poll1.as_mut().poll(cx).is_pending());
        assert!(poll2.as_mut().poll(cx).is_pending());

        // Start polling some wait handles which get cancelled
        // before new ones are attached
        let poll5 = sender.send(5);
        let poll6 = sender.send(6);
        pin_mut!(poll5);
        pin_mut!(poll6);
        assert!(poll5.as_mut().poll(cx).is_pending());
        assert!(poll6.as_mut().poll(cx).is_pending());

        // Cancel 2 futures. Only the remaining ones should get completed
        // Safety: We are not using these pins again so this is safe.
        unsafe { core::pin::Pin::into_inner_unchecked(poll5).cancel() };
        unsafe { core::pin::Pin::into_inner_unchecked(poll6).cancel() };

        assert!(poll3.as_mut().poll(cx).is_pending());
        assert!(poll4.as_mut().poll(cx).is_pending());

        assert_receive!(cx, &receiver, Some(100));
        assert_receive!(cx, &receiver, Some(101));
        assert_receive!(cx, &receiver, Some(102));

        assert_send_done(cx, &mut poll1, Ok(()));
        assert_send_done(cx, &mut poll2, Ok(()));
        assert_send_done(cx, &mut poll3, Ok(()));

        assert!(receiver.close().is_newly_closed());
        assert_receive!(cx, &receiver, Some(1));
        assert_receive!(cx, &receiver, Some(2));
        assert_receive!(cx, &receiver, Some(3));
        assert_send_done(cx, &mut poll4, Err(ChannelSendError(4)));

        assert_eq!(count, 4);
    }
}
