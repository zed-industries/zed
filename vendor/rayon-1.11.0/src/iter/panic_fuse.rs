use super::plumbing::*;
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

/// `PanicFuse` is an adaptor that wraps an iterator with a fuse in case
/// of panics, to halt all threads as soon as possible.
///
/// This struct is created by the [`panic_fuse()`] method on [`ParallelIterator`]
///
/// [`panic_fuse()`]: ParallelIterator::panic_fuse()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct PanicFuse<I> {
    base: I,
}

/// Helper that sets a bool to `true` if dropped while unwinding.
#[derive(Clone)]
struct Fuse<'a>(&'a AtomicBool);

impl<'a> Drop for Fuse<'a> {
    #[inline]
    fn drop(&mut self) {
        if thread::panicking() {
            self.0.store(true, Ordering::Relaxed);
        }
    }
}

impl<'a> Fuse<'a> {
    #[inline]
    fn panicked(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

impl<I> PanicFuse<I> {
    /// Creates a new `PanicFuse` iterator.
    pub(super) fn new(base: I) -> PanicFuse<I> {
        PanicFuse { base }
    }
}

impl<I> ParallelIterator for PanicFuse<I>
where
    I: ParallelIterator,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let panicked = AtomicBool::new(false);
        let consumer1 = PanicFuseConsumer {
            base: consumer,
            fuse: Fuse(&panicked),
        };
        self.base.drive_unindexed(consumer1)
    }

    fn opt_len(&self) -> Option<usize> {
        self.base.opt_len()
    }
}

impl<I> IndexedParallelIterator for PanicFuse<I>
where
    I: IndexedParallelIterator,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        let panicked = AtomicBool::new(false);
        let consumer1 = PanicFuseConsumer {
            base: consumer,
            fuse: Fuse(&panicked),
        };
        self.base.drive(consumer1)
    }

    fn len(&self) -> usize {
        self.base.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        return self.base.with_producer(Callback { callback });

        struct Callback<CB> {
            callback: CB,
        }

        impl<T, CB> ProducerCallback<T> for Callback<CB>
        where
            CB: ProducerCallback<T>,
        {
            type Output = CB::Output;

            fn callback<P>(self, base: P) -> CB::Output
            where
                P: Producer<Item = T>,
            {
                let panicked = AtomicBool::new(false);
                let producer = PanicFuseProducer {
                    base,
                    fuse: Fuse(&panicked),
                };
                self.callback.callback(producer)
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////
// Producer implementation

struct PanicFuseProducer<'a, P> {
    base: P,
    fuse: Fuse<'a>,
}

impl<'a, P> Producer for PanicFuseProducer<'a, P>
where
    P: Producer,
{
    type Item = P::Item;
    type IntoIter = PanicFuseIter<'a, P::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        PanicFuseIter {
            base: self.base.into_iter(),
            fuse: self.fuse,
        }
    }

    fn min_len(&self) -> usize {
        self.base.min_len()
    }
    fn max_len(&self) -> usize {
        self.base.max_len()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.base.split_at(index);
        (
            PanicFuseProducer {
                base: left,
                fuse: self.fuse.clone(),
            },
            PanicFuseProducer {
                base: right,
                fuse: self.fuse,
            },
        )
    }

    fn fold_with<G>(self, folder: G) -> G
    where
        G: Folder<Self::Item>,
    {
        let folder1 = PanicFuseFolder {
            base: folder,
            fuse: self.fuse,
        };
        self.base.fold_with(folder1).base
    }
}

struct PanicFuseIter<'a, I> {
    base: I,
    fuse: Fuse<'a>,
}

impl<'a, I> Iterator for PanicFuseIter<'a, I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fuse.panicked() {
            None
        } else {
            self.base.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

impl<'a, I> DoubleEndedIterator for PanicFuseIter<'a, I>
where
    I: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.fuse.panicked() {
            None
        } else {
            self.base.next_back()
        }
    }
}

impl<'a, I> ExactSizeIterator for PanicFuseIter<'a, I>
where
    I: ExactSizeIterator,
{
    fn len(&self) -> usize {
        self.base.len()
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct PanicFuseConsumer<'a, C> {
    base: C,
    fuse: Fuse<'a>,
}

impl<'a, T, C> Consumer<T> for PanicFuseConsumer<'a, C>
where
    C: Consumer<T>,
{
    type Folder = PanicFuseFolder<'a, C::Folder>;
    type Reducer = PanicFuseReducer<'a, C::Reducer>;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            PanicFuseConsumer {
                base: left,
                fuse: self.fuse.clone(),
            },
            PanicFuseConsumer {
                base: right,
                fuse: self.fuse.clone(),
            },
            PanicFuseReducer {
                base: reducer,
                _fuse: self.fuse,
            },
        )
    }

    fn into_folder(self) -> Self::Folder {
        PanicFuseFolder {
            base: self.base.into_folder(),
            fuse: self.fuse,
        }
    }

    fn full(&self) -> bool {
        self.fuse.panicked() || self.base.full()
    }
}

impl<'a, T, C> UnindexedConsumer<T> for PanicFuseConsumer<'a, C>
where
    C: UnindexedConsumer<T>,
{
    fn split_off_left(&self) -> Self {
        PanicFuseConsumer {
            base: self.base.split_off_left(),
            fuse: self.fuse.clone(),
        }
    }

    fn to_reducer(&self) -> Self::Reducer {
        PanicFuseReducer {
            base: self.base.to_reducer(),
            _fuse: self.fuse.clone(),
        }
    }
}

struct PanicFuseFolder<'a, C> {
    base: C,
    fuse: Fuse<'a>,
}

impl<'a, T, C> Folder<T> for PanicFuseFolder<'a, C>
where
    C: Folder<T>,
{
    type Result = C::Result;

    fn consume(mut self, item: T) -> Self {
        self.base = self.base.consume(item);
        self
    }

    fn consume_iter<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        fn cool<'a, T>(fuse: &'a Fuse<'_>) -> impl Fn(&T) -> bool + 'a {
            move |_| !fuse.panicked()
        }

        self.base = {
            let fuse = &self.fuse;
            let iter = iter.into_iter().take_while(cool(fuse));
            self.base.consume_iter(iter)
        };
        self
    }

    fn complete(self) -> C::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.fuse.panicked() || self.base.full()
    }
}

struct PanicFuseReducer<'a, C> {
    base: C,
    _fuse: Fuse<'a>,
}

impl<'a, T, C> Reducer<T> for PanicFuseReducer<'a, C>
where
    C: Reducer<T>,
{
    fn reduce(self, left: T, right: T) -> T {
        self.base.reduce(left, right)
    }
}
