use super::Merge as MergeTrait;
use crate::stream::IntoStream;
use crate::utils::{self, Indexer, PollArray, WakerArray};

use core::fmt;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_core::Stream;

/// A stream that merges multiple streams into a single stream.
///
/// This `struct` is created by the [`merge`] method on the [`Merge`] trait. See its
/// documentation for more.
///
/// [`merge`]: trait.Merge.html#method.merge
/// [`Merge`]: trait.Merge.html
#[pin_project::pin_project]
pub struct Merge<S, const N: usize>
where
    S: Stream,
{
    #[pin]
    streams: [S; N],
    indexer: Indexer<N>,
    wakers: WakerArray<N>,
    state: PollArray<N>,
    complete: usize,
    done: bool,
}

impl<S, const N: usize> Merge<S, N>
where
    S: Stream,
{
    pub(crate) fn new(streams: [S; N]) -> Self {
        Self {
            streams,
            indexer: Indexer::new(),
            wakers: WakerArray::new(),
            state: PollArray::new_pending(),
            complete: 0,
            done: false,
        }
    }
}

impl<S, const N: usize> fmt::Debug for Merge<S, N>
where
    S: Stream + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.streams.iter()).finish()
    }
}

impl<S, const N: usize> Stream for Merge<S, N>
where
    S: Stream,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        // return early if all streams are complete (handles empty array)
        if *this.complete == N {
            return Poll::Ready(None);
        }

        let mut readiness = this.wakers.readiness();
        readiness.set_waker(cx.waker());

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

            let stream = utils::get_pin_mut(this.streams.as_mut(), index).unwrap();
            match stream.poll_next(&mut cx) {
                Poll::Ready(Some(item)) => {
                    // Mark ourselves as ready again because we need to poll for the next item.
                    this.wakers.readiness().set_ready(index);
                    return Poll::Ready(Some(item));
                }
                Poll::Ready(None) => {
                    *this.complete += 1;
                    this.state[index].set_none();
                    if *this.complete == this.streams.len() {
                        return Poll::Ready(None);
                    }
                }
                Poll::Pending => {}
            }

            // Lock readiness so we can use it again
            readiness = this.wakers.readiness();
        }

        Poll::Pending
    }
}

impl<S, const N: usize> MergeTrait for [S; N]
where
    S: IntoStream,
{
    type Item = <Merge<S::IntoStream, N> as Stream>::Item;
    type Stream = Merge<S::IntoStream, N>;

    fn merge(self) -> Self::Stream {
        Merge::new(self.map(|i| i.into_stream()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_lite::future::block_on;
    use futures_lite::prelude::*;
    use futures_lite::stream;

    #[test]
    fn empty_array() {
        block_on(async {
            let streams: [stream::Once<i32>; 0] = [];
            let mut s = streams.merge();
            let result = s.next().await;
            assert_eq!(result, None);
        })
    }

    #[test]
    fn merge_array_4() {
        block_on(async {
            let a = stream::once(1);
            let b = stream::once(2);
            let c = stream::once(3);
            let d = stream::once(4);
            let mut s = [a, b, c, d].merge();

            let mut counter = 0;
            while let Some(n) = s.next().await {
                counter += n;
            }
            assert_eq!(counter, 10);
        })
    }

    #[test]
    fn merge_array_2x2() {
        block_on(async {
            let a = stream::repeat(1).take(2);
            let b = stream::repeat(2).take(2);
            let mut s = [a, b].merge();

            let mut counter = 0;
            while let Some(n) = s.next().await {
                counter += n;
            }
            assert_eq!(counter, 6);
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

        use crate::future::join::Join;
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
                        [receive1, receive2, receive3]
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
