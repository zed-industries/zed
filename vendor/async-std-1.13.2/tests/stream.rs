use std::convert::identity;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;

use async_std::channel::bounded as channel;
use async_std::prelude::*;
use async_std::stream;
use async_std::task;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(not(target_os = "unknown"))]
use async_std::task::spawn;
#[cfg(target_os = "unknown")]
use async_std::task::spawn_local as spawn;

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
/// Checks that streams are merged fully even if one of the components
/// experiences delay.
fn merging_delayed_streams_work() {
    let (sender, receiver) = channel::<i32>(10);

    let mut s = receiver.merge(stream::empty());
    let t = spawn(async move {
        let mut xs = Vec::new();
        while let Some(x) = s.next().await {
            xs.push(x);
        }
        xs
    });

    task::block_on(async move {
        task::sleep(std::time::Duration::from_millis(500)).await;
        sender.send(92).await.unwrap();
        drop(sender);
        let xs = t.await;
        assert_eq!(xs, vec![92])
    });

    let (sender, receiver) = channel::<i32>(10);

    let mut s = stream::empty().merge(receiver);
    let t = spawn(async move {
        let mut xs = Vec::new();
        while let Some(x) = s.next().await {
            xs.push(x);
        }
        xs
    });

    task::block_on(async move {
        task::sleep(std::time::Duration::from_millis(500)).await;
        sender.send(92).await.unwrap();
        drop(sender);
        let xs = t.await;
        assert_eq!(xs, vec![92])
    });
}

pin_project! {
    /// The opposite of `Fuse`: makes the stream panic if polled after termination.
    struct Explode<S> {
        #[pin]
        done: bool,
        #[pin]
        inner: S,
    }
}

impl<S: Stream> Stream for Explode<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        if *this.done {
            panic!("KABOOM!")
        }
        let res = this.inner.poll_next(cx);
        if let Poll::Ready(None) = &res {
            *this.done = true;
        }
        res
    }
}

fn explode<S: Stream>(s: S) -> Explode<S> {
    Explode {
        done: false,
        inner: s,
    }
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn merge_works_with_unfused_streams() {
    let s1 = explode(stream::once(92));
    let s2 = explode(stream::once(92));
    let mut s = s1.merge(s2);

    task::block_on(async move {
        let mut xs = Vec::new();
        while let Some(x) = s.next().await {
            xs.push(x)
        }
        assert_eq!(xs, vec![92, 92]);
    });
}

struct S<T>(T);

impl<T: Stream + Unpin> Stream for S<T> {
    type Item = T::Item;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<Self::Item>> {
        unsafe { Pin::new_unchecked(&mut self.0) }.poll_next(ctx)
    }
}

struct StrictOnce {
    polled: bool,
}

impl Stream for StrictOnce {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context) -> Poll<Option<Self::Item>> {
        assert!(!self.polled, "Polled after completion!");
        self.polled = true;
        Poll::Ready(None)
    }
}

struct Interchanger {
    polled: bool,
}

impl Stream for Interchanger {
    type Item = S<Box<dyn Stream<Item = ()> + Unpin>>;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<Self::Item>> {
        if self.polled {
            self.polled = false;
            ctx.waker().wake_by_ref();
            Poll::Pending
        } else {
            self.polled = true;
            Poll::Ready(Some(S(Box::new(StrictOnce { polled: false }))))
        }
    }
}

#[test]
fn flat_map_doesnt_poll_completed_inner_stream() {
    task::block_on(async {
        assert_eq!(
            Interchanger { polled: false }
                .take(2)
                .flat_map(identity)
                .count()
                .await,
            0
        );
    });
}

#[test]
fn flatten_doesnt_poll_completed_inner_stream() {
    task::block_on(async {
        assert_eq!(
            Interchanger { polled: false }
                .take(2)
                .flatten()
                .count()
                .await,
            0
        );
    });
}
