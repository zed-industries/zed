//! Parallel iterator types for [options]
//!
//! You will rarely need to interact with this module directly unless you need
//! to name one of the iterator types.
//!
//! [options]: std::option

use crate::iter::plumbing::*;
use crate::iter::*;
use std::sync::atomic::{AtomicBool, Ordering};

/// A parallel iterator over the value in [`Some`] variant of an [`Option`].
///
/// The iterator yields one value if the [`Option`] is a [`Some`], otherwise none.
///
/// This `struct` is created by the [`into_par_iter`] function.
///
/// [`into_par_iter`]: IntoParallelIterator::into_par_iter()
#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    opt: Option<T>,
}

impl<T: Send> IntoParallelIterator for Option<T> {
    type Item = T;
    type Iter = IntoIter<T>;

    fn into_par_iter(self) -> Self::Iter {
        IntoIter { opt: self }
    }
}

impl<T: Send> ParallelIterator for IntoIter<T> {
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.drive(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<T: Send> IndexedParallelIterator for IntoIter<T> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        let mut folder = consumer.into_folder();
        if let Some(item) = self.opt {
            folder = folder.consume(item);
        }
        folder.complete()
    }

    fn len(&self) -> usize {
        match self.opt {
            Some(_) => 1,
            None => 0,
        }
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        callback.callback(OptionProducer { opt: self.opt })
    }
}

/// A parallel iterator over a reference to the [`Some`] variant of an [`Option`].
///
/// The iterator yields one value if the [`Option`] is a [`Some`], otherwise none.
///
/// This `struct` is created by the [`par_iter`] function.
///
/// [`par_iter`]: IntoParallelRefIterator::par_iter()
#[derive(Debug)]
pub struct Iter<'a, T> {
    inner: IntoIter<&'a T>,
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T: Sync> IntoParallelIterator for &'a Option<T> {
    type Item = &'a T;
    type Iter = Iter<'a, T>;

    fn into_par_iter(self) -> Self::Iter {
        Iter {
            inner: self.as_ref().into_par_iter(),
        }
    }
}

delegate_indexed_iterator! {
    Iter<'a, T> => &'a T,
    impl<'a, T: Sync>
}

/// A parallel iterator over a mutable reference to the [`Some`] variant of an [`Option`].
///
/// The iterator yields one value if the [`Option`] is a [`Some`], otherwise none.
///
/// This `struct` is created by the [`par_iter_mut`] function.
///
/// [`par_iter_mut`]: IntoParallelRefMutIterator::par_iter_mut()
#[derive(Debug)]
pub struct IterMut<'a, T> {
    inner: IntoIter<&'a mut T>,
}

impl<'a, T: Send> IntoParallelIterator for &'a mut Option<T> {
    type Item = &'a mut T;
    type Iter = IterMut<'a, T>;

    fn into_par_iter(self) -> Self::Iter {
        IterMut {
            inner: self.as_mut().into_par_iter(),
        }
    }
}

delegate_indexed_iterator! {
    IterMut<'a, T> => &'a mut T,
    impl<'a, T: Send>
}

/// Private producer for an option
struct OptionProducer<T: Send> {
    opt: Option<T>,
}

impl<T: Send> Producer for OptionProducer<T> {
    type Item = T;
    type IntoIter = std::option::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.opt.into_iter()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        debug_assert!(index <= 1);
        let none = OptionProducer { opt: None };
        if index == 0 {
            (none, self)
        } else {
            (self, none)
        }
    }
}

/// Collect an arbitrary `Option`-wrapped collection.
///
/// If any item is `None`, then all previous items collected are discarded,
/// and it returns only `None`.
impl<C, T> FromParallelIterator<Option<T>> for Option<C>
where
    C: FromParallelIterator<T>,
    T: Send,
{
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: IntoParallelIterator<Item = Option<T>>,
    {
        fn check<T>(found_none: &AtomicBool) -> impl Fn(&Option<T>) + '_ {
            move |item| {
                if item.is_none() {
                    found_none.store(true, Ordering::Relaxed);
                }
            }
        }

        let found_none = AtomicBool::new(false);
        let collection = par_iter
            .into_par_iter()
            .inspect(check(&found_none))
            .while_some()
            .collect();

        if found_none.load(Ordering::Relaxed) {
            None
        } else {
            Some(collection)
        }
    }
}
