use super::Zip as ZipTrait;
use crate::stream::IntoStream;
use crate::utils::{self, PollArray, WakerArray};

use core::array;
use core::fmt;
use core::mem::{self, MaybeUninit};
use core::pin::Pin;
use core::task::{Context, Poll};

use futures_core::Stream;
use pin_project::{pin_project, pinned_drop};

/// A stream that ‘zips up’ multiple streams into a single stream of pairs.
///
/// This `struct` is created by the [`zip`] method on the [`Zip`] trait. See its
/// documentation for more.
///
/// [`zip`]: trait.Zip.html#method.zip
/// [`Zip`]: trait.Zip.html
#[pin_project(PinnedDrop)]
pub struct Zip<S, const N: usize>
where
    S: Stream,
{
    #[pin]
    streams: [S; N],
    output: [MaybeUninit<<S as Stream>::Item>; N],
    wakers: WakerArray<N>,
    state: PollArray<N>,
    done: bool,
}

impl<S, const N: usize> Zip<S, N>
where
    S: Stream,
{
    pub(crate) fn new(streams: [S; N]) -> Self {
        Self {
            streams,
            output: array::from_fn(|_| MaybeUninit::uninit()),
            state: PollArray::new_pending(),
            wakers: WakerArray::new(),
            done: false,
        }
    }
}

impl<S, const N: usize> fmt::Debug for Zip<S, N>
where
    S: Stream + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.streams.iter()).finish()
    }
}

impl<S, const N: usize> Stream for Zip<S, N>
where
    S: Stream,
{
    type Item = [S::Item; N];

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        assert!(!*this.done, "Stream should not be polled after completion");

        let mut readiness = this.wakers.readiness();
        readiness.set_waker(cx.waker());
        for index in 0..N {
            if !readiness.any_ready() {
                // Nothing is ready yet
                return Poll::Pending;
            } else if this.state[index].is_ready() || !readiness.clear_ready(index) {
                // We already have data stored for this stream,
                // Or this waker isn't ready yet
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
                    this.output[index] = MaybeUninit::new(item);
                    this.state[index].set_ready();

                    let all_ready = this.state.iter().all(|state| state.is_ready());
                    if all_ready {
                        // Reset the future's state.
                        readiness = this.wakers.readiness();
                        readiness.set_all_ready();
                        this.state.set_all_pending();

                        // Take the output
                        //
                        // SAFETY: we just validated all our data is populated, meaning
                        // we can assume this is initialized.
                        let mut output = array::from_fn(|_| MaybeUninit::uninit());
                        mem::swap(this.output, &mut output);
                        let output = unsafe { array_assume_init(output) };
                        return Poll::Ready(Some(output));
                    }
                }
                Poll::Ready(None) => {
                    // If one stream returns `None`, we can no longer return
                    // pairs - meaning the stream is over.
                    *this.done = true;
                    return Poll::Ready(None);
                }
                Poll::Pending => {}
            }

            // Lock readiness so we can use it again
            readiness = this.wakers.readiness();
        }
        Poll::Pending
    }
}

/// Drop the already initialized values on cancellation.
#[pinned_drop]
impl<S, const N: usize> PinnedDrop for Zip<S, N>
where
    S: Stream,
{
    fn drop(self: Pin<&mut Self>) {
        let this = self.project();

        for (state, output) in this.state.iter_mut().zip(this.output.iter_mut()) {
            if state.is_ready() {
                // SAFETY: we've just filtered down to *only* the initialized values.
                // We can assume they're initialized, and this is where we drop them.
                unsafe { output.assume_init_drop() };
            }
        }
    }
}

impl<S, const N: usize> ZipTrait for [S; N]
where
    S: IntoStream,
{
    type Item = <Zip<S::IntoStream, N> as Stream>::Item;
    type Stream = Zip<S::IntoStream, N>;

    fn zip(self) -> Self::Stream {
        Zip::new(self.map(|i| i.into_stream()))
    }
}

// Inlined version of the unstable `MaybeUninit::array_assume_init` feature.
// FIXME: replace with `utils::array_assume_init`
unsafe fn array_assume_init<T, const N: usize>(array: [MaybeUninit<T>; N]) -> [T; N] {
    // SAFETY:
    // * The caller guarantees that all elements of the array are initialized
    // * `MaybeUninit<T>` and T are guaranteed to have the same layout
    // * `MaybeUninit` does not drop, so there are no double-frees
    // And thus the conversion is safe
    let ret = unsafe { (&array as *const _ as *const [T; N]).read() };
    #[allow(clippy::forget_non_drop)]
    mem::forget(array);
    ret
}

#[cfg(test)]
mod tests {
    use crate::stream::Zip;
    use futures_lite::future::block_on;
    use futures_lite::prelude::*;
    use futures_lite::stream;

    #[test]
    fn zip_array_3() {
        block_on(async {
            let a = stream::repeat(1).take(2);
            let b = stream::repeat(2).take(2);
            let c = stream::repeat(3).take(2);
            let mut s = Zip::zip([a, b, c]);

            assert_eq!(s.next().await, Some([1, 2, 3]));
            assert_eq!(s.next().await, Some([1, 2, 3]));
            assert_eq!(s.next().await, None);
        })
    }
}
