use super::Merge as MergeTrait;
use crate::stream::IntoStream;
use crate::utils::{self, PollArray, WakerArray};

use core::fmt;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_core::Stream;

macro_rules! poll_stream {
    ($stream_idx:tt, $iteration:ident, $this:ident, $streams:ident . $stream_member:ident, $cx:ident, $len_streams:ident) => {
        if $stream_idx == $iteration {
            match unsafe { Pin::new_unchecked(&mut $streams.$stream_member) }.poll_next(&mut $cx) {
                Poll::Ready(Some(item)) => {
                    // Mark ourselves as ready again because we need to poll for the next item.
                    $this.wakers.readiness().set_ready($stream_idx);
                    return Poll::Ready(Some(item));
                }
                Poll::Ready(None) => {
                    *$this.completed += 1;
                    $this.state[$stream_idx].set_none();
                    if *$this.completed == $len_streams {
                        return Poll::Ready(None);
                    }
                }
                Poll::Pending => {}
            }
        }
    };
}

macro_rules! impl_merge_tuple {
    ($ignore:ident $StructName:ident) => {
        /// A stream that merges multiple streams into a single stream.
        ///
        /// This `struct` is created by the [`merge`] method on the [`Merge`] trait. See its
        /// documentation for more.
        ///
        /// [`merge`]: trait.Merge.html#method.merge
        /// [`Merge`]: trait.Merge.html
        pub struct $StructName {}

        impl fmt::Debug for $StructName {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple("Merge").finish()
            }
        }

        impl Stream for $StructName {
            type Item = core::convert::Infallible; // TODO: convert to `never` type in the stdlib

            fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                Poll::Ready(None)
            }
        }

        impl MergeTrait for () {
            type Item = core::convert::Infallible; // TODO: convert to `never` type in the stdlib
            type Stream = $StructName;

            fn merge(self) -> Self::Stream {
                $StructName { }
            }
        }
    };
    ($mod_name:ident $StructName:ident $($F:ident)+) => {
        mod $mod_name {
            #[pin_project::pin_project]
            pub(super) struct Streams<$($F,)+> { $(#[pin] pub(super) $F: $F),+ }

            #[repr(usize)]
            pub(super) enum Indexes { $($F),+ }

            pub(super) const LEN: usize = [$(Indexes::$F),+].len();
        }

        /// A stream that merges multiple streams into a single stream.
        ///
        /// This `struct` is created by the [`merge`] method on the [`Merge`] trait. See its
        /// documentation for more.
        ///
        /// [`merge`]: trait.Merge.html#method.merge
        /// [`Merge`]: trait.Merge.html
        #[pin_project::pin_project]
        pub struct $StructName<T, $($F),*>
        where $(
            $F: Stream<Item = T>,
        )* {
            #[pin] streams: $mod_name::Streams<$($F,)+>,
            indexer: utils::Indexer<{ utils::tuple_len!($($F,)*) }>,
            wakers: WakerArray<{$mod_name::LEN}>,
            state: PollArray<{$mod_name::LEN}>,
            completed: u8,
        }

        impl<T, $($F),*> fmt::Debug for $StructName<T, $($F),*>
        where
            $( $F: Stream<Item = T> + fmt::Debug, )*
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple("Merge")
                    $( .field(&self.streams.$F) )* // Hides implementation detail of Streams struct
                    .finish()
            }
        }

        impl<T, $($F),*> Stream for $StructName<T, $($F),*>
        where $(
            $F: Stream<Item = T>,
        )* {
            type Item = T;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                let this = self.project();

                let mut readiness = this.wakers.readiness();
                readiness.set_waker(cx.waker());

                const LEN: u8 = $mod_name::LEN as u8;

                let mut streams = this.streams.project();

                // Iterate over our streams one-by-one. If a stream yields a value,
                // we exit early. By default we'll return `Poll::Ready(None)`, but
                // this changes if we encounter a `Poll::Pending`.
                for index in this.indexer.iter() {
                    if !readiness.any_ready() {
                        // Nothing is ready yet
                        return Poll::Pending;
                    } else if !readiness.clear_ready(index) || this.state[index].is_none() {
                        continue;
                    }

                    // unlock readiness so we don't deadlock when polling
                    #[allow(clippy::drop_non_drop)]
                    drop(readiness);

                    // Obtain the intermediate waker.
                    let mut cx = Context::from_waker(this.wakers.get(index).unwrap());

                    $(
                        let stream_index = $mod_name::Indexes::$F as usize;
                        poll_stream!(
                            stream_index,
                            index,
                            this,
                            streams . $F,
                            cx,
                            LEN
                        );
                    )+

                    // Lock readiness so we can use it again
                    readiness = this.wakers.readiness();
                }

                Poll::Pending
            }
        }

        impl<T, $($F),*> MergeTrait for ($($F,)*)
        where $(
            $F: IntoStream<Item = T>,
        )* {
            type Item = T;
            type Stream = $StructName<T, $($F::IntoStream),*>;

            fn merge(self) -> Self::Stream {
                let ($($F,)*): ($($F,)*) = self;
                $StructName {
                    streams: $mod_name::Streams { $($F: $F.into_stream()),+ },
                    indexer: utils::Indexer::new(),
                    wakers: WakerArray::new(),
                    state: PollArray::new_pending(),
                    completed: 0,
                }
            }
        }
    };
}

impl_merge_tuple! { merge0 Merge0  }
impl_merge_tuple! { merge1 Merge1  A }
impl_merge_tuple! { merge2 Merge2  A B }
impl_merge_tuple! { merge3 Merge3  A B C }
impl_merge_tuple! { merge4 Merge4  A B C D }
impl_merge_tuple! { merge5 Merge5  A B C D E }
impl_merge_tuple! { merge6 Merge6  A B C D E F }
impl_merge_tuple! { merge7 Merge7  A B C D E F G }
impl_merge_tuple! { merge8 Merge8  A B C D E F G H }
impl_merge_tuple! { merge9 Merge9  A B C D E F G H I }
impl_merge_tuple! { merge10 Merge10 A B C D E F G H I J }
impl_merge_tuple! { merge11 Merge11 A B C D E F G H I J K }
impl_merge_tuple! { merge12 Merge12 A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use super::*;
    use futures_lite::future::block_on;
    use futures_lite::prelude::*;
    use futures_lite::stream;

    #[test]
    fn merge_tuple_0() {
        block_on(async {
            let mut s = ().merge();

            let mut called = false;
            while s.next().await.is_some() {
                called = true;
            }
            assert!(!called);
        })
    }

    #[test]
    fn merge_tuple_1() {
        block_on(async {
            let a = stream::once(1);
            let mut s = (a,).merge();

            let mut counter = 0;
            while let Some(n) = s.next().await {
                counter += n;
            }
            assert_eq!(counter, 1);
        })
    }

    #[test]
    fn merge_tuple_2() {
        block_on(async {
            let a = stream::once(1);
            let b = stream::once(2);
            let mut s = (a, b).merge();

            let mut counter = 0;
            while let Some(n) = s.next().await {
                counter += n;
            }
            assert_eq!(counter, 3);
        })
    }

    #[test]
    fn merge_tuple_3() {
        block_on(async {
            let a = stream::once(1);
            let b = stream::once(2);
            let c = stream::once(3);
            let mut s = (a, b, c).merge();

            let mut counter = 0;
            while let Some(n) = s.next().await {
                counter += n;
            }
            assert_eq!(counter, 6);
        })
    }

    #[test]
    fn merge_tuple_4() {
        block_on(async {
            let a = stream::once(1);
            let b = stream::once(2);
            let c = stream::once(3);
            let d = stream::once(4);
            let mut s = (a, b, c, d).merge();

            let mut counter = 0;
            while let Some(n) = s.next().await {
                counter += n;
            }
            assert_eq!(counter, 10);
        })
    }

    /// This test case uses channels so we'll have streams that return Pending from time to time.
    ///
    /// The purpose of this test is to make sure we have the waking logic working.
    #[test]
    #[cfg(feature = "alloc")]
    fn merge_channels() {
        use alloc::rc::Rc;
        use core::cell::RefCell;

        use futures::executor::LocalPool;
        use futures::task::LocalSpawnExt;

        use crate::future::Join;
        use crate::utils::channel::local_channel;

        let mut pool = LocalPool::new();

        let done = Rc::new(RefCell::new(false));
        let done2 = done.clone();

        pool.spawner()
            .spawn_local(async move {
                let (send1, receive1) = local_channel();
                let (send2, receive2) = local_channel();
                let (send3, receive3) = local_channel();

                let (count, ()) = (
                    async {
                        (receive1, receive2, receive3)
                            .merge()
                            .fold(0, |a, b| a + b)
                            .await
                    },
                    async {
                        for i in 1..=4 {
                            send1.send(i);
                            send2.send(i);
                            send3.send(i);
                        }
                        drop(send1);
                        drop(send2);
                        drop(send3);
                    },
                )
                    .join()
                    .await;

                assert_eq!(count, 30);

                *done2.borrow_mut() = true;
            })
            .unwrap();

        while !*done.borrow() {
            pool.run_until_stalled()
        }
    }
}
