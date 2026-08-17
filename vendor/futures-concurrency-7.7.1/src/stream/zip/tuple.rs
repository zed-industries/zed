use core::fmt;
use core::mem::MaybeUninit;
use core::pin::Pin;
use core::task::{Context, Poll};

use futures_core::Stream;

use super::Zip;
use crate::utils::{PollArray, WakerArray};

macro_rules! impl_zip_for_tuple {
    ($mod_name: ident $StructName: ident $($F: ident)+) => {
        mod $mod_name {
            pub(super) struct Output<$($F,)+>
            where
                $($F: super::Stream,)+
            {
                $(pub(super) $F: core::mem::MaybeUninit<<$F as super::Stream>::Item>,)+
            }

            impl<$($F,)+> Default for Output<$($F,)+>
            where
                $($F: super::Stream,)+
            {
                fn default() -> Self {
                    Self {
                        $($F: core::mem::MaybeUninit::uninit(),)+
                    }
                }
            }

            #[repr(usize)]
            enum Indexes {
                $($F,)+
            }

            $(
                pub(super) const $F: usize = Indexes::$F as usize;
            )+

            pub(super) const LEN: usize = [$(Indexes::$F,)+].len();
        }

        #[pin_project::pin_project(PinnedDrop)]
        pub struct $StructName<$($F,)+>
        where
            $($F: Stream,)+
        {
            done: bool,
            output: $mod_name::Output<$($F,)+>,
            state: PollArray<{ $mod_name::LEN }>,
            wakers: WakerArray<{ $mod_name::LEN }>,
            $( #[pin] $F: $F,)+

        }

        impl<$($F,)+> fmt::Debug for $StructName<$($F,)+>
        where
            $($F: Stream + fmt::Debug,)+
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple("Zip")
                    $(.field(&self.$F))+
                    .finish()
            }
        }

        impl<$($F,)+> Stream for $StructName<$($F,)+>
        where
            $($F: Stream,)+
        {
            type Item = (
                $(<$F as Stream>::Item,)+
            );

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                let mut this = self.project();

                const LEN: usize = $mod_name::LEN;

                assert!(!*this.done, "Stream should not be polled after completion");

                let mut readiness = this.wakers.readiness();
                readiness.set_waker(cx.waker());

                for index in 0..LEN {
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

                    let all_ready = match index {
                        $(
                            $mod_name::$F => {
                                let stream = unsafe { Pin::new_unchecked(&mut this.$F) };

                                match stream.poll_next(&mut cx) {
                                    Poll::Pending => false,
                                    Poll::Ready(None) => {
                                        // If one stream returns `None`, we can no longer return
                                        // pairs - meaning the stream is over.
                                        *this.done = true;
                                        return Poll::Ready(None);
                                    }
                                    Poll::Ready(Some(item)) => {
                                        this.output.$F = MaybeUninit::new(item);
                                        this.state[$mod_name::$F].set_ready();

                                        this.state.iter().all(|state| state.is_ready())
                                    }
                                }
                            },
                        )+
                        _ => unreachable!(),
                    };

                    if all_ready {
                        // Reset the future's state.
                        readiness = this.wakers.readiness();
                        readiness.set_all_ready();
                        this.state.set_all_pending();

                        // Take the output
                        //
                        // SAFETY: we just validated all our data is populated, meaning
                        // we can assume this is initialized.
                        let mut output = $mod_name::Output::default();
                        core::mem::swap(this.output, &mut output);

                        match output {
                            $mod_name::Output {
                                $($F,)+
                            } => return Poll::Ready(Some((
                                $(unsafe { $F.assume_init() },)+
                            )))
                        }
                    }

                    // Lock readiness so we can use it again
                    readiness = this.wakers.readiness();
                }

                Poll::Pending
            }
        }

        impl<$($F,)+> Zip for ($($F,)+)
        where
            $($F: Stream,)+
        {
            type Item = (
                $(<$F as Stream>::Item,)+
            );

            type Stream = $StructName<$($F,)+>;

            fn zip(self) -> Self::Stream {
                let ($($F,)*): ($($F,)*) = self;
                Self::Stream {
                    done: false,
                    output: Default::default(),
                    state: PollArray::new_pending(),
                    wakers: WakerArray::new(),
                    $($F,)+
                }
            }
        }

        #[pin_project::pinned_drop]
        impl<$($F,)+> PinnedDrop for $StructName<$($F,)+>
        where
            $($F: Stream,)+
        {
            fn drop(self: Pin<&mut Self>) {
                let this = self.project();

                $(
                    if this.state[$mod_name::$F].is_ready() {
                        // SAFETY: we've just filtered down to *only* the initialized values.
                        unsafe { this.output.$F.assume_init_drop() };
                    }
                )+
            }
        }
    };
}

impl_zip_for_tuple! { zip_1 Zip1 A }
impl_zip_for_tuple! { zip_2 Zip2 A B }
impl_zip_for_tuple! { zip_3 Zip3 A B C }
impl_zip_for_tuple! { zip_4 Zip4 A B C D }
impl_zip_for_tuple! { zip_5 Zip5 A B C D E }
impl_zip_for_tuple! { zip_6 Zip6 A B C D E F }
impl_zip_for_tuple! { zip_7 Zip7 A B C D E F G }
impl_zip_for_tuple! { zip_8 Zip8 A B C D E F G H }
impl_zip_for_tuple! { zip_9 Zip9 A B C D E F G H I }
impl_zip_for_tuple! { zip_10 Zip10 A B C D E F G H I J }
impl_zip_for_tuple! { zip_11 Zip11 A B C D E F G H I J K }
impl_zip_for_tuple! { zip_12 Zip12 A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use futures_lite::future::block_on;
    use futures_lite::prelude::*;
    use futures_lite::stream;

    use crate::stream::Zip;

    #[test]
    fn zip_tuple_3() {
        block_on(async {
            let a = stream::repeat(1).take(2);
            let b = stream::repeat("hello").take(2);
            let c = stream::repeat(("a", "b")).take(2);
            let mut s = Zip::zip((a, b, c));

            assert_eq!(s.next().await, Some((1, "hello", ("a", "b"))));
            assert_eq!(s.next().await, Some((1, "hello", ("a", "b"))));
            assert_eq!(s.next().await, None);
        })
    }
}
