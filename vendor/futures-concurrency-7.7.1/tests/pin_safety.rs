//! Pin safety regression tests for FutureGroup and StreamGroup.
//!
//! Check [#188](https://github.com/yoshuawuyts/futures-concurrency/issues/188) for context

#![cfg(feature = "alloc")]
#![allow(clippy::assertions_on_constants)]

use std::{
    future::Future,
    marker::PhantomPinned,
    pin::{pin, Pin},
    task::{Context, Poll},
};

use futures::{Stream, StreamExt};
use futures_concurrency::{future::FutureGroup, stream::StreamGroup};

async fn drain_to_completion(mut stream: Pin<&mut (impl Stream + ?Sized)>) {
    while stream.as_mut().next().await.is_some() {}
}

fn get_inner_mut<T>(pin: Pin<&mut T>) -> &mut T {
    unsafe { pin.get_unchecked_mut() }
}

struct PinCheckFuture {
    self_ptr: Option<*const Self>,
    remaining: usize,
    _pinned: PhantomPinned,
}

impl PinCheckFuture {
    fn polls(polls: usize) -> Self {
        Self {
            self_ptr: None,
            remaining: polls,
            _pinned: PhantomPinned,
        }
    }
}

impl Future for PinCheckFuture {
    type Output = usize;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<usize> {
        let this = unsafe { self.get_unchecked_mut() };
        let current = this as *const Self;
        let stored = *this.self_ptr.get_or_insert(current);
        assert_eq!(stored, current, "moved after pinning");

        if this.remaining > 0 {
            this.remaining -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(this.remaining)
        }
    }
}

struct PinCheckStream {
    remaining: usize,
    self_ptr: Option<*const Self>,
    _pinned: PhantomPinned,
}

impl PinCheckStream {
    fn polls(items: usize) -> Self {
        Self {
            remaining: items,
            self_ptr: None,
            _pinned: PhantomPinned,
        }
    }
}

impl Stream for PinCheckStream {
    type Item = usize;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<usize>> {
        let this = unsafe { self.get_unchecked_mut() };
        let current = this as *const Self;
        let stored = *this.self_ptr.get_or_insert(current);
        assert_eq!(stored, current, "moved after pinning");

        if this.remaining > 0 {
            this.remaining -= 1;
            Poll::Ready(Some(this.remaining))
        } else {
            Poll::Ready(None)
        }
    }
}

#[test]
fn future_group_no_move_on_growth() {
    futures_lite::future::block_on(async {
        let mut group = FutureGroup::new();
        for i in 0..10 {
            group.insert(PinCheckFuture::polls(i + 5));
        }

        let mut group = pin!(group);
        let _ = group.as_mut().take(5).collect::<Vec<_>>().await;

        let group_mut = get_inner_mut(group.as_mut());
        for i in 0..100 {
            group_mut.insert(PinCheckFuture::polls(i));
        }

        drain_to_completion(group.as_mut()).await;
        // no panic = all futures completed without moving!
    });
}

#[test]
fn stream_group_no_move_on_growth() {
    futures_lite::future::block_on(async {
        let mut group = StreamGroup::new();
        for _ in 0..10 {
            group.insert(PinCheckStream::polls(5));
        }

        let mut group = pin!(group);
        let _ = group.as_mut().take(25).collect::<Vec<_>>().await;

        let group_mut = get_inner_mut(group.as_mut());
        for i in 0..50 {
            group_mut.insert(PinCheckStream::polls(i));
        }

        drain_to_completion(group.as_mut()).await;
        // no panic = all futures completed without moving!
    });
}

#[test]
fn future_group_no_move_on_repeated_growth() {
    futures_lite::future::block_on(async {
        let mut group = FutureGroup::new();
        for i in 0..20 {
            group.insert(PinCheckFuture::polls(i));
        }

        let mut group = pin!(group);

        for wave in 1..5 {
            let _ = group.as_mut().take(10).collect::<Vec<_>>().await;

            let group_mut = get_inner_mut(group.as_mut());
            for i in 0..20 {
                group_mut.insert(PinCheckFuture::polls(wave * 20 + i));
            }
        }

        drain_to_completion(group.as_mut()).await;
        // no panic = all futures completed without moving!
    });
}

#[test]
fn stream_group_no_move_on_repeated_growth() {
    futures_lite::future::block_on(async {
        let mut group = StreamGroup::new();
        for _ in 0..10 {
            group.insert(PinCheckStream::polls(3));
        }

        let mut group = pin!(group);

        for _ in 1..5 {
            let _ = group.as_mut().take(15).collect::<Vec<_>>().await;

            let group_mut = get_inner_mut(group.as_mut());
            for _ in 0..10 {
                group_mut.insert(PinCheckStream::polls(3));
            }
        }

        drain_to_completion(group.as_mut()).await;
        // no panic = all futures completed without moving!
    });
}
