use super::Race as RaceTrait;
use crate::utils;

use core::fmt::{self, Debug};
use core::future::{Future, IntoFuture};
use core::pin::Pin;
use core::task::{Context, Poll};

use pin_project::pin_project;

macro_rules! impl_race_tuple {
    ($StructName:ident $($F:ident)+) => {
        /// A future which waits for the first future to complete.
        ///
        /// This `struct` is created by the [`race`] method on the [`Race`] trait. See
        /// its documentation for more.
        ///
        /// [`race`]: crate::future::Race::race
        /// [`Race`]: crate::future::Race
        #[pin_project]
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        #[allow(non_snake_case)]
        pub struct $StructName<T, $($F),*>
        where $(
            $F: Future<Output = T>,
        )* {
            done: bool,
            indexer: utils::Indexer<{ utils::tuple_len!($($F,)*) }>,
            $(#[pin] $F: $F,)*
        }

        impl<T, $($F),*> Debug for $StructName<T, $($F),*>
        where $(
            $F: Future<Output = T> + Debug,
        )* {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple("Race")
                    $(.field(&self.$F))*
                    .finish()
            }
        }

        impl<T, $($F),*> RaceTrait for ($($F,)*)
        where $(
            $F: IntoFuture<Output = T>,
        )* {
            type Output = T;
            type Future = $StructName<T, $($F::IntoFuture),*>;

            fn race(self) -> Self::Future {
                let ($($F,)*): ($($F,)*) = self;
                $StructName {
                    done: false,
                    indexer: utils::Indexer::new(),
                    $($F: $F.into_future()),*
                }
            }
        }

        impl<T, $($F: Future),*> Future for $StructName<T, $($F),*>
        where
            $($F: Future<Output = T>),*
        {
            type Output = T;

            fn poll(
                self: Pin<&mut Self>, cx: &mut Context<'_>
            ) -> Poll<Self::Output> {
                let mut this = self.project();
                assert!(!*this.done, "Futures must not be polled after completing");

                #[repr(usize)]
                enum Indexes {
                    $($F),*
                }

                for i in this.indexer.iter() {
                    utils::gen_conditions!(i, this, cx, poll, $((Indexes::$F as usize; $F, {
                        Poll::Ready(output) => {
                            *this.done = true;
                            return Poll::Ready(output);
                        },
                        _ => continue,
                    }))*);
                }

                Poll::Pending
            }
        }
    };
}

impl_race_tuple! { Race1 A }
impl_race_tuple! { Race2 A B }
impl_race_tuple! { Race3 A B C }
impl_race_tuple! { Race4 A B C D }
impl_race_tuple! { Race5 A B C D E }
impl_race_tuple! { Race6 A B C D E F }
impl_race_tuple! { Race7 A B C D E F G }
impl_race_tuple! { Race8 A B C D E F G H }
impl_race_tuple! { Race9 A B C D E F G H I }
impl_race_tuple! { Race10 A B C D E F G H I J }
impl_race_tuple! { Race11 A B C D E F G H I J K }
impl_race_tuple! { Race12 A B C D E F G H I J K L }

#[cfg(test)]
mod test {
    use super::*;
    use core::future;

    #[test]
    fn race_1() {
        futures_lite::future::block_on(async {
            let a = future::ready("world");
            assert_eq!((a,).race().await, "world");
        });
    }

    #[test]
    fn race_2() {
        futures_lite::future::block_on(async {
            let a = future::pending();
            let b = future::ready("world");
            assert_eq!((a, b).race().await, "world");
        });
    }

    #[test]
    fn race_3() {
        futures_lite::future::block_on(async {
            let a = future::pending();
            let b = future::ready("hello");
            let c = future::ready("world");
            let result = (a, b, c).race().await;
            assert!(matches!(result, "hello" | "world"));
        });
    }
}
