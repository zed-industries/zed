use core::fmt;
use core::pin::Pin;
use core::task::{Context, Poll};

use futures_core::Stream;

use super::Chain;

macro_rules! impl_chain_for_tuple {
    ($mod_name: ident $StructName:ident $($F:ident)+) => {
        mod $mod_name {
            #[repr(usize)]
            enum Indexes {
                $($F,)+
            }

            $(
                pub(super) const $F: usize = Indexes::$F as usize;
            )+

            pub(super) const LEN: usize = [$(Indexes::$F,)+].len();
        }

        #[pin_project::pin_project]
        pub struct $StructName<$($F,)+> {
            index: usize,
            done: bool,
            $( #[pin] $F: $F,)+
        }

        impl<T, $($F,)+> Stream for $StructName<$($F,)+>
        where
            $($F: Stream<Item = T>,)+
        {
            type Item = T;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                let mut this = self.project();

                assert!(!*this.done, "Stream should not be polled after completion");

                loop {
                    if *this.index == $mod_name::LEN {
                        *this.done = true;
                        return Poll::Ready(None);
                    }

                    match *this.index {
                        $(
                            $mod_name::$F => {
                                let fut = unsafe { Pin::new_unchecked(&mut this.$F) };
                                match fut.poll_next(cx) {
                                    Poll::Ready(None) => {
                                        *this.index += 1;
                                        continue;
                                    }
                                    v @ (Poll::Pending | Poll::Ready(Some(_))) => return v,
                                }
                            },
                        )+
                        _  => unreachable!(),
                    }
                }
            }
        }

        impl<$($F,)+> fmt::Debug for $StructName<$($F,)+>
        where
            $($F: fmt::Debug,)+
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple("Chain")
                    $(.field(&self.$F))+
                    .finish()
            }
        }

        impl<T, $($F,)+> Chain for ($($F,)+)
        where
            $($F: Stream<Item = T>,)+
        {
            type Item = T;

            type Stream = $StructName<$($F,)+>;

            fn chain(self) -> Self::Stream {
                let ($($F,)*): ($($F,)*) = self;
                Self::Stream {
                    done: false,
                    index: 0,
                    $($F,)+
                }
            }
        }
    }
}

impl_chain_for_tuple! { chain_1 Chain1 A }
impl_chain_for_tuple! { chain_2 Chain2 A B }
impl_chain_for_tuple! { chain_3 Chain3 A B C }
impl_chain_for_tuple! { chain_4 Chain4 A B C D }
impl_chain_for_tuple! { chain_5 Chain5 A B C D E }
impl_chain_for_tuple! { chain_6 Chain6 A B C D E F }
impl_chain_for_tuple! { chain_7 Chain7 A B C D E F G }
impl_chain_for_tuple! { chain_8 Chain8 A B C D E F G H }
impl_chain_for_tuple! { chain_9 Chain9 A B C D E F G H I }
impl_chain_for_tuple! { chain_10 Chain10 A B C D E F G H I J }
impl_chain_for_tuple! { chain_11 Chain11 A B C D E F G H I J K }
impl_chain_for_tuple! { chain_12 Chain12 A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use super::*;

    use futures_lite::future::block_on;
    use futures_lite::prelude::*;
    use futures_lite::stream;

    #[test]
    fn chain_3() {
        block_on(async {
            let a = stream::once(1);
            let b = stream::once(2);
            let c = stream::once(3);
            let mut s = (a, b, c).chain();

            assert_eq!(s.next().await, Some(1));
            assert_eq!(s.next().await, Some(2));
            assert_eq!(s.next().await, Some(3));
            assert_eq!(s.next().await, None);
        })
    }
}
