//! Macros for delegating newtype iterators to inner types.

// Note: these place `impl` bounds at the end, as token gobbling is the only way
// I know how to consume an arbitrary list of constraints, with `$($args:tt)*`.

/// Creates a parallel iterator implementation which simply wraps an inner type
/// and delegates all methods inward.  The actual struct must already be
/// declared with an `inner` field.
///
/// The implementation of `IntoParallelIterator` should be added separately.
macro_rules! delegate_iterator {
    ($iter:ty => $item:ty ,
     impl $( $args:tt )*
     ) => {
        impl $( $args )* ParallelIterator for $iter {
            type Item = $item;

            fn drive_unindexed<C>(self, consumer: C) -> C::Result
                where C: UnindexedConsumer<Self::Item>
            {
                self.inner.drive_unindexed(consumer)
            }

            fn opt_len(&self) -> Option<usize> {
                self.inner.opt_len()
            }
        }
    }
}

/// Creates an indexed parallel iterator implementation which simply wraps an
/// inner type and delegates all methods inward.  The actual struct must already
/// be declared with an `inner` field.
macro_rules! delegate_indexed_iterator {
    ($iter:ty => $item:ty ,
     impl $( $args:tt )*
     ) => {
        delegate_iterator!{
            $iter => $item ,
            impl $( $args )*
        }

        impl $( $args )* IndexedParallelIterator for $iter {
            fn drive<C>(self, consumer: C) -> C::Result
                where C: Consumer<Self::Item>
            {
                self.inner.drive(consumer)
            }

            fn len(&self) -> usize {
                self.inner.len()
            }

            fn with_producer<CB>(self, callback: CB) -> CB::Output
                where CB: ProducerCallback<Self::Item>
            {
                self.inner.with_producer(callback)
            }
        }
    }
}

#[test]
fn unindexed_example() {
    use crate::collections::btree_map::IntoIter;
    use crate::iter::plumbing::*;
    use crate::prelude::*;

    use std::collections::BTreeMap;

    struct MyIntoIter<T: Ord + Send, U: Send> {
        inner: IntoIter<T, U>,
    }

    delegate_iterator! {
        MyIntoIter<T, U> => (T, U),
        impl<T: Ord + Send, U: Send>
    }

    let map = BTreeMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let iter = MyIntoIter {
        inner: map.into_par_iter(),
    };
    let vec: Vec<_> = iter.map(|(k, _)| k).collect();
    assert_eq!(vec, &[1, 2, 3]);
}

#[test]
fn indexed_example() {
    use crate::iter::plumbing::*;
    use crate::prelude::*;
    use crate::vec::IntoIter;

    struct MyIntoIter<T: Send> {
        inner: IntoIter<T>,
    }

    delegate_indexed_iterator! {
        MyIntoIter<T> => T,
        impl<T: Send>
    }

    let iter = MyIntoIter {
        inner: vec![1, 2, 3].into_par_iter(),
    };
    let mut vec = vec![];
    iter.collect_into_vec(&mut vec);
    assert_eq!(vec, &[1, 2, 3]);
}
