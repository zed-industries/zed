//! Parallel iterator types for [standard collections]
//!
//! You will rarely need to interact with this module directly unless you need
//! to name one of the iterator types.
//!
//! [standard collections]: std::collections

/// Convert an iterable collection into a parallel iterator by first
/// collecting into a temporary `Vec`, then iterating that.
macro_rules! into_par_vec {
    ($t:ty => $iter:ident<$($i:tt),*>, impl $($args:tt)*) => {
        impl $($args)* IntoParallelIterator for $t {
            type Item = <$t as IntoIterator>::Item;
            type Iter = $iter<$($i),*>;

            fn into_par_iter(self) -> Self::Iter {
                use std::iter::FromIterator;
                $iter { inner: Vec::from_iter(self).into_par_iter() }
            }
        }
    };
}

pub mod binary_heap;
pub mod btree_map;
pub mod btree_set;
pub mod hash_map;
pub mod hash_set;
pub mod linked_list;
pub mod vec_deque;

use self::drain_guard::DrainGuard;

mod drain_guard {
    use crate::iter::ParallelDrainRange;
    use std::mem;
    use std::ops::RangeBounds;

    /// A proxy for draining a collection by converting to a `Vec` and back.
    ///
    /// This is used for draining `BinaryHeap` and `VecDeque`, which both have
    /// zero-allocation conversions to/from `Vec`, though not zero-cost:
    /// - `BinaryHeap` will heapify from `Vec`, but at least that will be empty.
    /// - `VecDeque` has to shift items to offset 0 when converting to `Vec`.
    #[allow(missing_debug_implementations)]
    pub(super) struct DrainGuard<'a, T, C: From<Vec<T>>> {
        collection: &'a mut C,
        vec: Vec<T>,
    }

    impl<'a, T, C> DrainGuard<'a, T, C>
    where
        C: Default + From<Vec<T>>,
        Vec<T>: From<C>,
    {
        pub(super) fn new(collection: &'a mut C) -> Self {
            Self {
                // Temporarily steal the inner `Vec` so we can drain in place.
                vec: Vec::from(mem::take(collection)),
                collection,
            }
        }
    }

    impl<'a, T, C: From<Vec<T>>> Drop for DrainGuard<'a, T, C> {
        fn drop(&mut self) {
            // Restore the collection from the `Vec` with its original capacity.
            *self.collection = C::from(mem::take(&mut self.vec));
        }
    }

    impl<'a, T, C> ParallelDrainRange<usize> for &'a mut DrainGuard<'_, T, C>
    where
        T: Send,
        C: From<Vec<T>>,
    {
        type Iter = crate::vec::Drain<'a, T>;
        type Item = T;

        fn par_drain<R: RangeBounds<usize>>(self, range: R) -> Self::Iter {
            self.vec.par_drain(range)
        }
    }
}
