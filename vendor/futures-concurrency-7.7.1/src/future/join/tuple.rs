use super::Join as JoinTrait;
use crate::utils::{PollArray, WakerArray};

use core::fmt::{self, Debug};
use core::future::{Future, IntoFuture};
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::DerefMut;
use core::pin::Pin;
use core::task::{Context, Poll};

use pin_project::{pin_project, pinned_drop};

/// Generates the `poll` call for every `Future` inside `$futures`.
///
/// SAFETY: pretty please only call this after having made very sure that the future you're trying
/// to call is actually marked `ready!`. If Rust had unsafe macros, this would be one.
//
// This is implemented as a tt-muncher of the future name `$($F:ident)`
// and the future index `$($rest)`, taking advantage that we only support
// tuples up to  12 elements
//
// # References
// TT Muncher: https://veykril.github.io/tlborm/decl-macros/patterns/tt-muncher.html
macro_rules! unsafe_poll {
    // recursively iterate
    (@inner $iteration:ident, $this:ident, $futures:ident, $cx:ident, $fut_name:ident $($F:ident)* | $fut_idx:tt $($rest:tt)*) => {
        if $fut_idx == $iteration {

            if let Poll::Ready(value) = unsafe {
                $futures.$fut_name.as_mut()
                    .map_unchecked_mut(|t| t.deref_mut())
                    .poll(&mut $cx)
            } {
                $this.outputs.$fut_idx.write(value);
                *$this.completed += 1;
                $this.state[$fut_idx].set_ready();
                // SAFETY: the future state has been changed to "ready" which
                // means we'll no longer poll the future, so it's safe to drop
                unsafe { ManuallyDrop::drop($futures.$fut_name.as_mut().get_unchecked_mut()) };
            }
        }
        unsafe_poll!(@inner $iteration, $this, $futures, $cx, $($F)* | $($rest)*);
    };

    // base condition
    (@inner $iteration:ident, $this:ident, $futures:ident, $cx:ident, | $($rest:tt)*) => {};

    // macro start
    ($iteration:ident, $this:ident, $futures:ident, $cx:ident, $LEN:ident, $($F:ident,)+) => {
        unsafe_poll!(@inner $iteration, $this, $futures, $cx, $($F)+ | 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14);
    };
}

/// Drop all initialized values
macro_rules! drop_initialized_values {
    // recursively iterate
    (@drop $output:ident, $($rem_outs:ident,)* | $states:expr, $state_idx:tt, $($rem_idx:tt,)*) => {
        if $states[$state_idx].is_ready() {
            // SAFETY: we've just filtered down to *only* the initialized values.
            // We can assume they're initialized, and this is where we drop them.
            unsafe { $output.assume_init_drop() };
            $states[$state_idx].set_none();
        }
        drop_initialized_values!(@drop $($rem_outs,)* | $states, $($rem_idx,)*);
    };

    // base condition
    (@drop | $states:expr, $($rem_idx:tt,)*) => {};

    // macro start
    ($($outs:ident,)+ | $states:expr) => {
        drop_initialized_values!(@drop $($outs,)+ | $states, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,);
    };
}

/// Drop all pending futures
macro_rules! drop_pending_futures {
    // recursively iterate
    (@inner $states:ident, $futures:ident, $fut_name:ident $($F:ident)* | $fut_idx:tt $($rest:tt)*) => {
        if $states[$fut_idx].is_pending() {
            // SAFETY: We're accessing the value behind the pinned reference to drop it exactly once.
            let futures = unsafe { $futures.as_mut().get_unchecked_mut() };
            // SAFETY: we've just filtered down to *only* the initialized values.
            // We can assume they're initialized, and this is where we drop them.
            unsafe { ManuallyDrop::drop(&mut futures.$fut_name) };
        }
        drop_pending_futures!(@inner $states, $futures, $($F)* | $($rest)*);
    };

    // base condition
    (@inner $states:ident, $futures:ident, | $($rest:tt)*) => {};

    // macro start
    ($states:ident, $futures:ident, $($F:ident,)+) => {
        drop_pending_futures!(@inner $states, $futures, $($F)+ | 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14);
    };
}

macro_rules! impl_join_tuple {
    ($mod_name:ident $StructName:ident) => {
        /// A future which waits for two similarly-typed futures to complete.
        ///
        /// This `struct` is created by the [`join`] method on the [`Join`] trait. See
        /// its documentation for more.
        ///
        /// [`join`]: crate::future::Join::join
        /// [`Join`]: crate::future::Join
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        #[allow(non_snake_case)]
        pub struct $StructName {}

        impl fmt::Debug for $StructName {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple("Join").finish()
            }
        }

        impl Future for $StructName {
            type Output = ();

            fn poll(
                self: Pin<&mut Self>, _cx: &mut Context<'_>
            ) -> Poll<Self::Output> {
                Poll::Ready(())
            }
        }

        impl JoinTrait for () {
            type Output = ();
            type Future = $StructName;
            fn join(self) -> Self::Future {
                $StructName {}
            }
        }
    };
    ($mod_name:ident $StructName:ident $($F:ident)+) => {
        mod $mod_name {
            use core::mem::ManuallyDrop;

            #[pin_project::pin_project]
            pub(super) struct Futures<$($F,)+> {$(
                #[pin]
                pub(super) $F: ManuallyDrop<$F>,
            )+}

            #[repr(u8)]
            pub(super) enum Indexes { $($F,)+ }

            pub(super) const LEN: usize = [$(Indexes::$F,)+].len();
        }

        /// Waits for many similarly-typed futures to complete.
        ///
        /// This `struct` is created by the [`join`] method on the [`Join`] trait. See
        /// its documentation for more.
        ///
        /// [`join`]: crate::future::Join::join
        /// [`Join`]: crate::future::Join
        #[pin_project(PinnedDrop)]
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        #[allow(non_snake_case)]
        pub struct $StructName<$($F: Future),+> {
            #[pin]
            futures: $mod_name::Futures<$($F,)+>,
            outputs: ($(MaybeUninit<$F::Output>,)+),
            // trace the state of outputs, marking them as ready or consumed
            // then, drop the non-consumed values, if any
            state: PollArray<{$mod_name::LEN}>,
            wakers: WakerArray<{$mod_name::LEN}>,
            completed: usize,
        }

        impl<$($F),+> Debug for $StructName<$($F),+>
        where
            $( $F: Future + Debug, )+
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple("Join")
                    $(.field(&self.futures.$F))+
                    .finish()
            }
        }

        #[allow(unused_mut)]
        #[allow(unused_parens)]
        #[allow(unused_variables)]
        impl<$($F: Future),+> Future for $StructName<$($F),+> {
            type Output = ($($F::Output,)+);

            fn poll(
                self: Pin<&mut Self>, cx: &mut Context<'_>
            ) -> Poll<Self::Output> {
                const LEN: usize = $mod_name::LEN;

                let mut this = self.project();
                let all_completed = !(*this.completed == LEN);
                assert!(all_completed, "Futures must not be polled after completing");

                let mut futures = this.futures.project();

                let mut readiness = this.wakers.readiness();
                readiness.set_waker(cx.waker());

                for index in 0..LEN {
                    if !readiness.any_ready() {
                        // nothing ready yet
                        return Poll::Pending;
                    }
                    if !readiness.clear_ready(index) || this.state[index].is_ready() {
                        // future not ready yet or already polled to completion, skip
                        continue;
                    }

                    // unlock readiness so we don't deadlock when polling
                    #[allow(clippy::drop_non_drop)]
                    drop(readiness);

                    // obtain the intermediate waker
                    let mut cx = Context::from_waker(this.wakers.get(index).unwrap());

                    // generate the needed code to poll `futures.{index}`
                    // SAFETY: the future's state should be "pending", so it's safe to poll
                    unsafe_poll!(index, this, futures, cx, LEN, $($F,)+);

                    if *this.completed == LEN {
                        let out = {
                            let mut out = ($(MaybeUninit::<$F::Output>::uninit(),)+);
                            core::mem::swap(&mut out, this.outputs);
                            let ($($F,)+) = out;
                            unsafe { ($($F.assume_init(),)+) }
                        };

                        this.state.set_all_none();

                        return Poll::Ready(out);
                    }
                    readiness = this.wakers.readiness();
                }

                Poll::Pending
            }
        }

        #[pinned_drop]
        impl<$($F: Future),+> PinnedDrop for $StructName<$($F),+> {
            fn drop(self: Pin<&mut Self>) {
                let this = self.project();

                let &mut ($(ref mut $F,)+) = this.outputs;

                let states = this.state;
                let mut futures = this.futures;
                drop_initialized_values!($($F,)+ | states);
                drop_pending_futures!(states, futures, $($F,)+);
            }
        }

        #[allow(unused_parens)]
        impl<$($F),+> JoinTrait for ($($F,)+)
        where $(
            $F: IntoFuture,
        )+ {
            type Output = ($($F::Output,)*);
            type Future = $StructName<$($F::IntoFuture),*>;

            fn join(self) -> Self::Future {
                let ($($F,)+): ($($F,)+) = self;
                $StructName {
                    futures: $mod_name::Futures {$($F: ManuallyDrop::new($F.into_future()),)+},
                    state: PollArray::new_pending(),
                    outputs: ($(MaybeUninit::<$F::Output>::uninit(),)+),
                    wakers: WakerArray::new(),
                    completed: 0,
                }
            }
        }
    };
}

impl_join_tuple! { join0 Join0 }
impl_join_tuple! { join1 Join1 A }
impl_join_tuple! { join2 Join2 A B }
impl_join_tuple! { join3 Join3 A B C }
impl_join_tuple! { join4 Join4 A B C D }
impl_join_tuple! { join5 Join5 A B C D E }
impl_join_tuple! { join6 Join6 A B C D E F }
impl_join_tuple! { join7 Join7 A B C D E F G }
impl_join_tuple! { join8 Join8 A B C D E F G H }
impl_join_tuple! { join9 Join9 A B C D E F G H I }
impl_join_tuple! { join10 Join10 A B C D E F G H I J }
impl_join_tuple! { join11 Join11 A B C D E F G H I J K }
impl_join_tuple! { join12 Join12 A B C D E F G H I J K L }
impl_join_tuple! { join13 Join13 A B C D E F G H I J K L M }
impl_join_tuple! { join14 Join14 A B C D E F G H I J K L M N }
impl_join_tuple! { join15 Join15 A B C D E F G H I J K L M N O }

#[cfg(test)]
mod test {
    use super::*;
    use core::future;

    #[test]
    #[allow(clippy::unit_cmp)]
    fn join_0() {
        futures_lite::future::block_on(async {
            assert_eq!(().join().await, ());
        });
    }

    #[test]
    fn join_1() {
        futures_lite::future::block_on(async {
            let a = future::ready("hello");
            assert_eq!((a,).join().await, ("hello",));
        });
    }

    #[test]
    fn join_2() {
        futures_lite::future::block_on(async {
            let a = future::ready("hello");
            let b = future::ready(12);
            assert_eq!((a, b).join().await, ("hello", 12));
        });
    }

    #[test]
    fn join_3() {
        futures_lite::future::block_on(async {
            let a = future::ready("hello");
            let b = future::ready("world");
            let c = future::ready(12);
            assert_eq!((a, b, c).join().await, ("hello", "world", 12));
        });
    }

    #[test]
    #[cfg(feature = "std")]
    fn does_not_leak_memory() {
        use core::cell::RefCell;
        use futures_lite::future::pending;

        thread_local! {
            static NOT_LEAKING: RefCell<bool> = const { RefCell::new(false) };
        };

        struct FlipFlagAtDrop;
        impl Drop for FlipFlagAtDrop {
            fn drop(&mut self) {
                NOT_LEAKING.with(|v| {
                    *v.borrow_mut() = true;
                });
            }
        }

        futures_lite::future::block_on(async {
            // this will trigger Miri if we don't drop the memory
            let string = future::ready("memory leak".to_owned());

            // this will not flip the thread_local flag if we don't drop the memory
            let flip = future::ready(FlipFlagAtDrop);

            let leak = (string, flip, pending::<u8>()).join();

            _ = futures_lite::future::poll_once(leak).await;
        });

        NOT_LEAKING.with(|flag| {
            assert!(*flag.borrow());
        })
    }
}
