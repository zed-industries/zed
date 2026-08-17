use super::plumbing::*;
use super::*;
use std::num::NonZeroUsize;
use std::{fmt, iter, mem};

/// Iterator adaptor for [the `repeat()` function].
///
/// [the `repeat()` function]: repeat()
#[derive(Debug, Clone)]
pub struct Repeat<T> {
    element: T,
}

/// Creates a parallel iterator that endlessly repeats `element` (by
/// cloning it). Note that this iterator has "infinite" length, so
/// typically you would want to use `zip` or `take` or some other
/// means to shorten it, or consider using
/// [the `repeat_n()` function] instead.
///
/// [the `repeat_n()` function]: repeat_n()
///
/// # Examples
///
/// ```
/// use rayon::prelude::*;
/// use rayon::iter::repeat;
/// let x: Vec<(i32, i32)> = repeat(22).zip(0..3).collect();
/// assert_eq!(x, vec![(22, 0), (22, 1), (22, 2)]);
/// ```
pub fn repeat<T: Clone + Send>(element: T) -> Repeat<T> {
    Repeat { element }
}

impl<T> Repeat<T>
where
    T: Clone + Send,
{
    /// Takes only `n` repeats of the element, similar to the general
    /// [`take()`].
    ///
    /// The resulting `RepeatN` is an `IndexedParallelIterator`, allowing
    /// more functionality than `Repeat` alone.
    ///
    /// [`take()`]: IndexedParallelIterator::take()
    pub fn take(self, n: usize) -> RepeatN<T> {
        repeat_n(self.element, n)
    }

    /// Iterates tuples, repeating the element with items from another
    /// iterator, similar to the general [`zip()`].
    ///
    /// [`zip()`]: IndexedParallelIterator::zip()
    pub fn zip<Z>(self, zip_op: Z) -> Zip<RepeatN<T>, Z::Iter>
    where
        Z: IntoParallelIterator<Iter: IndexedParallelIterator>,
    {
        let z = zip_op.into_par_iter();
        let n = z.len();
        self.take(n).zip(z)
    }
}

impl<T> ParallelIterator for Repeat<T>
where
    T: Clone + Send,
{
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let producer = RepeatProducer {
            element: self.element,
        };
        bridge_unindexed(producer, consumer)
    }
}

/// Unindexed producer for `Repeat`.
struct RepeatProducer<T: Clone + Send> {
    element: T,
}

impl<T: Clone + Send> UnindexedProducer for RepeatProducer<T> {
    type Item = T;

    fn split(self) -> (Self, Option<Self>) {
        (
            RepeatProducer {
                element: self.element.clone(),
            },
            Some(RepeatProducer {
                element: self.element,
            }),
        )
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<T>,
    {
        folder.consume_iter(iter::repeat(self.element))
    }
}

/// Iterator adaptor for [the `repeat_n()` function].
///
/// [the `repeat_n()` function]: repeat_n()
#[derive(Clone)]
pub struct RepeatN<T> {
    inner: RepeatNProducer<T>,
}

/// Creates a parallel iterator that produces `n` repeats of `element`
/// (by cloning it).
///
/// # Examples
///
/// ```
/// use rayon::prelude::*;
/// use rayon::iter::repeat_n;
/// let x: Vec<(i32, i32)> = repeat_n(22, 3).zip(0..3).collect();
/// assert_eq!(x, vec![(22, 0), (22, 1), (22, 2)]);
/// ```
pub fn repeat_n<T: Clone + Send>(element: T, n: usize) -> RepeatN<T> {
    let inner = match NonZeroUsize::new(n) {
        Some(count) => RepeatNProducer::Repeats(element, count),
        None => RepeatNProducer::Empty,
    };
    RepeatN { inner }
}

/// Creates a parallel iterator that produces `n` repeats of `element`
/// (by cloning it).
///
/// Deprecated in favor of [`repeat_n`] for consistency with the standard library.
#[deprecated(note = "use `repeat_n`")]
pub fn repeatn<T: Clone + Send>(element: T, n: usize) -> RepeatN<T> {
    repeat_n(element, n)
}

impl<T: fmt::Debug> fmt::Debug for RepeatN<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("RepeatN");
        if let RepeatNProducer::Repeats(element, count) = &self.inner {
            dbg.field("count", &count.get())
                .field("element", element)
                .finish()
        } else {
            dbg.field("count", &0usize).finish_non_exhaustive()
        }
    }
}

impl<T> ParallelIterator for RepeatN<T>
where
    T: Clone + Send,
{
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.inner.len())
    }
}

impl<T> IndexedParallelIterator for RepeatN<T>
where
    T: Clone + Send,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        callback.callback(self.inner)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Producer for `RepeatN`.
#[derive(Clone)]
enum RepeatNProducer<T> {
    Repeats(T, NonZeroUsize),
    Empty,
}

impl<T: Clone + Send> Producer for RepeatNProducer<T> {
    type Item = T;
    type IntoIter = Self;

    fn into_iter(self) -> Self::IntoIter {
        // We could potentially use `std::iter::RepeatN` with MSRV 1.82, but we have no way to
        // create an empty instance without a value in hand, like `repeat_n(value, 0)`.
        self
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        if let Self::Repeats(element, count) = self {
            assert!(index <= count.get());
            match (
                NonZeroUsize::new(index),
                NonZeroUsize::new(count.get() - index),
            ) {
                (Some(left), Some(right)) => (
                    Self::Repeats(element.clone(), left),
                    Self::Repeats(element, right),
                ),
                (Some(left), None) => (Self::Repeats(element, left), Self::Empty),
                (None, Some(right)) => (Self::Empty, Self::Repeats(element, right)),
                (None, None) => unreachable!(),
            }
        } else {
            assert!(index == 0);
            (Self::Empty, Self::Empty)
        }
    }
}

impl<T: Clone> Iterator for RepeatNProducer<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if let Self::Repeats(element, count) = self {
            if let Some(rem) = NonZeroUsize::new(count.get() - 1) {
                *count = rem;
                Some(element.clone())
            } else {
                match mem::replace(self, Self::Empty) {
                    Self::Repeats(element, _) => Some(element),
                    Self::Empty => unreachable!(),
                }
            }
        } else {
            None
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<T> {
        if let Self::Repeats(_, count) = self {
            if let Some(rem) = NonZeroUsize::new(count.get().saturating_sub(n)) {
                *count = rem;
                return self.next();
            }
            *self = Self::Empty;
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Clone> DoubleEndedIterator for RepeatNProducer<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.next()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<T> {
        self.nth(n)
    }
}

impl<T: Clone> ExactSizeIterator for RepeatNProducer<T> {
    #[inline]
    fn len(&self) -> usize {
        match self {
            Self::Repeats(_, count) => count.get(),
            Self::Empty => 0,
        }
    }
}
