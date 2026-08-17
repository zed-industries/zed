//! Parallel iterator types for [vectors][std::vec] (`Vec<T>`)
//!
//! You will rarely need to interact with this module directly unless you need
//! to name one of the iterator types.
//!
//! [std::vec]: https://doc.rust-lang.org/std/vec/index.html

use crate::concurrent_stream::{self, FromStream};
use crate::prelude::*;
use crate::utils::{from_iter, FromIter};
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;
use core::future::Ready;

pub use crate::future::join::vec::Join;
pub use crate::future::race::vec::Race;
pub use crate::future::race_ok::vec::{AggregateError, RaceOk};
pub use crate::future::try_join::vec::TryJoin;
pub use crate::stream::chain::vec::Chain;
pub use crate::stream::merge::vec::Merge;
pub use crate::stream::zip::vec::Zip;

/// Concurrent async iterator that moves out of a vector.
#[derive(Debug)]
pub struct IntoConcurrentStream<T>(FromStream<FromIter<alloc::vec::IntoIter<T>>>);

impl<T> ConcurrentStream for IntoConcurrentStream<T> {
    type Item = T;

    type Future = Ready<T>;

    async fn drive<C>(self, consumer: C) -> C::Output
    where
        C: concurrent_stream::Consumer<Self::Item, Self::Future>,
    {
        self.0.drive(consumer).await
    }

    fn concurrency_limit(&self) -> Option<core::num::NonZeroUsize> {
        self.0.concurrency_limit()
    }
}

impl<T> concurrent_stream::IntoConcurrentStream for Vec<T> {
    type Item = T;

    type IntoConcurrentStream = IntoConcurrentStream<T>;

    fn into_co_stream(self) -> Self::IntoConcurrentStream {
        let stream = from_iter(self);
        let co_stream = stream.co();
        IntoConcurrentStream(co_stream)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn collect() {
        futures_lite::future::block_on(async {
            let v: Vec<_> = vec![1, 2, 3, 4, 5].into_co_stream().collect().await;
            assert_eq!(v, &[1, 2, 3, 4, 5]);
        });
    }
}
