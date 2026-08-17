use super::TryJoin as TryJoinTrait;
use crate::utils::{PollArray, WakerArray};

use core::fmt::{self, Debug};
use core::future::{Future, IntoFuture};
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::mem::MaybeUninit;
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
                *$this.completed += 1;

                // Check the value, short-circuit on error.
                match value {
                    Ok(value) => {
                        $this.outputs.$fut_idx.write(value);

                        // SAFETY: We're marking the state as "ready", which
                        // means the future has been consumed, and data is
                        // now available to be consumed. The future will no
                        // longer be used after this point so it's safe to drop.
                        $this.state[$fut_idx].set_ready();
                        unsafe { ManuallyDrop::drop($futures.$fut_name.as_mut().get_unchecked_mut()) };
                    }
                    Err(err) => {
                        // The future should no longer be polled after we're done here
                        *$this.consumed = true;

                        // SAFETY: We're about to return the error value
                        // from the future, and drop the entire future.
                        // We're marking the future as consumed, and then
                        // proceeding to drop all other futures and
                        // initiatlized values in the destructor.
                        $this.state[$fut_idx].set_none();
                        unsafe { ManuallyDrop::drop($futures.$fut_name.as_mut().get_unchecked_mut()) };

                        return Poll::Ready(Err(err));
                    }
                }
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

macro_rules! impl_try_join_tuple {
    // `Impl TryJoin for ()`
    ($mod_name:ident $StructName:ident) => {
        /// A future which waits for similarly-typed futures to complete, or aborts early on error.
        ///
        /// This `struct` is created by the [`try_join`] method on the [`TryJoin`] trait. See
        /// its documentation for more.
        ///
        /// [`try_join`]: crate::future::TryJoin::try_join
        /// [`TryJoin`]: crate::future::Join
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        #[allow(non_snake_case)]
        pub struct $StructName {}

        impl fmt::Debug for $StructName {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple("TryJoin").finish()
            }
        }

        impl Future for $StructName {
            type Output = Result<(), core::convert::Infallible>;

            fn poll(
                self: Pin<&mut Self>, _cx: &mut Context<'_>
            ) -> Poll<Self::Output> {
                Poll::Ready(Ok(()))
            }
        }

        impl TryJoinTrait for () {
            type Output = ();
            type Error = core::convert::Infallible;
            type Future = $StructName;
            fn try_join(self) -> Self::Future {
                $StructName {}
            }
        }
    };

    // `Impl TryJoin for (F..)`
    ($mod_name:ident $StructName:ident $(($F:ident $T:ident))+) => {
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

        /// Waits for many similarly-typed futures to complete or abort early on error.
        ///
        /// This `struct` is created by the [`try_join`] method on the [`TryJoin`] trait. See
        /// its documentation for more.
        ///
        /// [`try_join`]: crate::future::TryJoin::try_join
        /// [`TryJoin`]: crate::future::Join
        #[pin_project(PinnedDrop)]
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        #[allow(non_snake_case)]
        pub struct $StructName<$($F, $T,)+ Err> {
            #[pin]
            futures: $mod_name::Futures<$($F,)+>,
            outputs: ($(MaybeUninit<$T>,)+),
            // trace the state of outputs, marking them as ready or consumed
            // then, drop the non-consumed values, if any
            state: PollArray<{$mod_name::LEN}>,
            wakers: WakerArray<{$mod_name::LEN}>,
            completed: usize,
            consumed: bool,
            _phantom: PhantomData<Err>,
        }

        impl<$($F, $T,)+ Err> Debug for $StructName<$($F, $T,)+ Err>
        where
            $( $F: Future + Debug, )+
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple("TryJoin")
                    $(.field(&self.futures.$F))+
                    .finish()
            }
        }

        #[allow(unused_mut)]
        #[allow(unused_parens)]
        #[allow(unused_variables)]
        impl<$($F, $T,)+ Err> Future for $StructName<$($F, $T,)+ Err>
        where $(
            $F: Future<Output = Result<$T, Err>>,
        )+ {
            type Output = Result<($($T,)+), Err>;

            fn poll(
                self: Pin<&mut Self>, cx: &mut Context<'_>
            ) -> Poll<Self::Output> {
                const LEN: usize = $mod_name::LEN;

                let mut this = self.project();
                assert!(!*this.consumed, "Futures must not be polled after completing");

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
                            let mut out = ($(MaybeUninit::<$T>::uninit(),)+);
                            core::mem::swap(&mut out, this.outputs);
                            let ($($F,)+) = out;
                            unsafe { ($($F.assume_init(),)+) }
                        };

                        this.state.set_all_none();
                        *this.consumed = true;

                        return Poll::Ready(Ok(out));
                    }
                    readiness = this.wakers.readiness();
                }

                Poll::Pending
            }
        }

        #[pinned_drop]
        impl<$($F, $T,)+ Err> PinnedDrop for $StructName<$($F, $T,)+ Err> {
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
        impl<$($F, $T,)+ Err> TryJoinTrait for ($($F,)+)
        where $(
            $F: IntoFuture<Output = Result<$T, Err>>,
        )+ {
            type Output = ($($T,)+);
            type Error = Err;
            type Future = $StructName<$($F::IntoFuture, $T,)+ Err>;

            fn try_join(self) -> Self::Future {
                let ($($F,)+): ($($F,)+) = self;
                $StructName {
                    futures: $mod_name::Futures {$(
                        $F: ManuallyDrop::new($F.into_future()),
                    )+},
                    state: PollArray::new_pending(),
                    outputs: ($(MaybeUninit::<$T>::uninit(),)+),
                    wakers: WakerArray::new(),
                    completed: 0,
                    consumed: false,
                    _phantom: PhantomData,
                }
            }
        }
    };
}

impl_try_join_tuple! { try_join0 TryJoin0 }
impl_try_join_tuple! { try_join_1 TryJoin1 (A ResA) }
impl_try_join_tuple! { try_join_2 TryJoin2 (A ResA) (B ResB) }
impl_try_join_tuple! { try_join_3 TryJoin3 (A ResA) (B ResB) (C ResC) }
impl_try_join_tuple! { try_join_4 TryJoin4 (A ResA) (B ResB) (C ResC) (D ResD) }
impl_try_join_tuple! { try_join_5 TryJoin5 (A ResA) (B ResB) (C ResC) (D ResD) (E ResE) }
impl_try_join_tuple! { try_join_6 TryJoin6 (A ResA) (B ResB) (C ResC) (D ResD) (E ResE) (F ResF) }
impl_try_join_tuple! { try_join_7 TryJoin7 (A ResA) (B ResB) (C ResC) (D ResD) (E ResE) (F ResF) (G ResG) }
impl_try_join_tuple! { try_join_8 TryJoin8 (A ResA) (B ResB) (C ResC) (D ResD) (E ResE) (F ResF) (G ResG) (H ResH) }
impl_try_join_tuple! { try_join_9 TryJoin9 (A ResA) (B ResB) (C ResC) (D ResD) (E ResE) (F ResF) (G ResG) (H ResH) (I ResI) }
impl_try_join_tuple! { try_join_10 TryJoin10 (A ResA) (B ResB) (C ResC) (D ResD) (E ResE) (F ResF) (G ResG) (H ResH) (I ResI) (J ResJ) }
impl_try_join_tuple! { try_join_11 TryJoin11 (A ResA) (B ResB) (C ResC) (D ResD) (E ResE) (F ResF) (G ResG) (H ResH) (I ResI) (J ResJ) (K ResK) }
impl_try_join_tuple! { try_join_12 TryJoin12 (A ResA) (B ResB) (C ResC) (D ResD) (E ResE) (F ResF) (G ResG) (H ResH) (I ResI) (J ResJ) (K ResK) (L ResL) }
impl_try_join_tuple! { try_join_13 TryJoin13 (A ResA) (B ResB) (C ResC) (D ResD) (E ResE) (F ResF) (G ResG) (H ResH) (I ResI) (J ResJ) (K ResK) (L ResL) (M ResM) }
impl_try_join_tuple! { try_join_14 TryJoin14 (A ResA) (B ResB) (C ResC) (D ResD) (E ResE) (F ResF) (G ResG) (H ResH) (I ResI) (J ResJ) (K ResK) (L ResL) (M ResM) (N ResN) }
impl_try_join_tuple! { try_join_15 TryJoin15 (A ResA) (B ResB) (C ResC) (D ResD) (E ResE) (F ResF) (G ResG) (H ResH) (I ResI) (J ResJ) (K ResK) (L ResL) (M ResM) (N ResN) (O ResO) }

#[cfg(test)]
mod test {
    use super::*;

    use core::convert::Infallible;
    use core::future;

    #[test]
    fn all_ok() {
        futures_lite::future::block_on(async {
            let a = async { Ok::<_, Infallible>("aaaa") };
            let b = async { Ok::<_, Infallible>(1) };
            let c = async { Ok::<_, Infallible>('z') };

            let result = (a, b, c).try_join().await;
            assert_eq!(result, Ok(("aaaa", 1, 'z')));
        })
    }

    #[test]
    fn one_err() {
        futures_lite::future::block_on(async {
            let res: Result<(_, char), ()> = (future::ready(Ok("hello")), future::ready(Err(())))
                .try_join()
                .await;
            assert_eq!(res, Err(()));
        })
    }

    #[test]
    fn issue_135_resume_after_completion() {
        use futures_lite::future::yield_now;
        futures_lite::future::block_on(async {
            let ok = async { Ok::<_, ()>(()) };
            let err = async {
                yield_now().await;
                Ok::<_, ()>(())
            };

            let res = (ok, err).try_join().await;

            assert_eq!(res.unwrap(), ((), ()));
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
            let string = future::ready(Result::Ok("memory leak".to_owned()));

            // this will not flip the thread_local flag if we don't drop the memory
            let flip = future::ready(Result::Ok(FlipFlagAtDrop));

            let leak = (string, flip, pending::<Result<u8, ()>>()).try_join();

            _ = futures_lite::future::poll_once(leak).await;
        });

        NOT_LEAKING.with(|flag| {
            assert!(*flag.borrow());
        })
    }
}
