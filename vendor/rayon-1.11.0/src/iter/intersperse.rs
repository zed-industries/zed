use super::plumbing::*;
use super::*;
use std::cell::Cell;
use std::iter::{self, Fuse};

/// `Intersperse` is an iterator that inserts a particular item between each
/// item of the adapted iterator.  This struct is created by the
/// [`intersperse()`] method on [`ParallelIterator`]
///
/// [`intersperse()`]: ParallelIterator::intersperse()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct Intersperse<I>
where
    I: ParallelIterator,
{
    base: I,
    item: I::Item,
}

impl<I> Intersperse<I>
where
    I: ParallelIterator<Item: Clone>,
{
    /// Creates a new `Intersperse` iterator
    pub(super) fn new(base: I, item: I::Item) -> Self {
        Intersperse { base, item }
    }
}

impl<I> ParallelIterator for Intersperse<I>
where
    I: ParallelIterator<Item: Clone>,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<I::Item>,
    {
        let consumer1 = IntersperseConsumer::new(consumer, self.item);
        self.base.drive_unindexed(consumer1)
    }

    fn opt_len(&self) -> Option<usize> {
        match self.base.opt_len()? {
            0 => Some(0),
            len => len.checked_add(len - 1),
        }
    }
}

impl<I> IndexedParallelIterator for Intersperse<I>
where
    I: IndexedParallelIterator<Item: Clone>,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        let consumer1 = IntersperseConsumer::new(consumer, self.item);
        self.base.drive(consumer1)
    }

    fn len(&self) -> usize {
        let len = self.base.len();
        if len > 0 {
            len.checked_add(len - 1).expect("overflow")
        } else {
            0
        }
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        let len = self.len();
        return self.base.with_producer(Callback {
            callback,
            item: self.item,
            len,
        });

        struct Callback<CB, T> {
            callback: CB,
            item: T,
            len: usize,
        }

        impl<T, CB> ProducerCallback<T> for Callback<CB, T>
        where
            CB: ProducerCallback<T>,
            T: Clone + Send,
        {
            type Output = CB::Output;

            fn callback<P>(self, base: P) -> CB::Output
            where
                P: Producer<Item = T>,
            {
                let producer = IntersperseProducer::new(base, self.item, self.len);
                self.callback.callback(producer)
            }
        }
    }
}

struct IntersperseProducer<P>
where
    P: Producer,
{
    base: P,
    item: P::Item,
    len: usize,
    clone_first: bool,
}

impl<P> IntersperseProducer<P>
where
    P: Producer,
{
    fn new(base: P, item: P::Item, len: usize) -> Self {
        IntersperseProducer {
            base,
            item,
            len,
            clone_first: false,
        }
    }
}

impl<P> Producer for IntersperseProducer<P>
where
    P: Producer<Item: Clone + Send>,
{
    type Item = P::Item;
    type IntoIter = IntersperseIter<P::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        IntersperseIter {
            base: self.base.into_iter().fuse(),
            item: self.item,
            clone_first: self.len > 0 && self.clone_first,

            // If there's more than one item, then even lengths end the opposite
            // of how they started with respect to interspersed clones.
            clone_last: self.len > 1 && ((self.len & 1 == 0) ^ self.clone_first),
        }
    }

    fn min_len(&self) -> usize {
        self.base.min_len()
    }
    fn max_len(&self) -> usize {
        self.base.max_len()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        debug_assert!(index <= self.len);

        // The left needs half of the items from the base producer, and the
        // other half will be our interspersed item.  If we're not leading with
        // a cloned item, then we need to round up the base number of items,
        // otherwise round down.
        let base_index = (index + !self.clone_first as usize) / 2;
        let (left_base, right_base) = self.base.split_at(base_index);

        let left = IntersperseProducer {
            base: left_base,
            item: self.item.clone(),
            len: index,
            clone_first: self.clone_first,
        };

        let right = IntersperseProducer {
            base: right_base,
            item: self.item,
            len: self.len - index,

            // If the index is odd, the right side toggles `clone_first`.
            clone_first: (index & 1 == 1) ^ self.clone_first,
        };

        (left, right)
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        let folder1 = IntersperseFolder {
            base: folder,
            item: self.item,
            clone_first: self.clone_first,
        };
        self.base.fold_with(folder1).base
    }
}

struct IntersperseIter<I>
where
    I: Iterator,
{
    base: Fuse<I>,
    item: I::Item,
    clone_first: bool,
    clone_last: bool,
}

impl<I> Iterator for IntersperseIter<I>
where
    I: DoubleEndedIterator<Item: Clone> + ExactSizeIterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.clone_first {
            self.clone_first = false;
            Some(self.item.clone())
        } else if let next @ Some(_) = self.base.next() {
            // If there are any items left, we'll need another clone in front.
            self.clone_first = self.base.len() != 0;
            next
        } else if self.clone_last {
            self.clone_last = false;
            Some(self.item.clone())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<I> DoubleEndedIterator for IntersperseIter<I>
where
    I: DoubleEndedIterator<Item: Clone> + ExactSizeIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.clone_last {
            self.clone_last = false;
            Some(self.item.clone())
        } else if let next_back @ Some(_) = self.base.next_back() {
            // If there are any items left, we'll need another clone in back.
            self.clone_last = self.base.len() != 0;
            next_back
        } else if self.clone_first {
            self.clone_first = false;
            Some(self.item.clone())
        } else {
            None
        }
    }
}

impl<I> ExactSizeIterator for IntersperseIter<I>
where
    I: DoubleEndedIterator<Item: Clone> + ExactSizeIterator,
{
    fn len(&self) -> usize {
        let len = self.base.len();
        len + len.saturating_sub(1) + self.clone_first as usize + self.clone_last as usize
    }
}

struct IntersperseConsumer<C, T> {
    base: C,
    item: T,
    clone_first: Cell<bool>,
}

impl<C, T> IntersperseConsumer<C, T>
where
    C: Consumer<T>,
{
    fn new(base: C, item: T) -> Self {
        IntersperseConsumer {
            base,
            item,
            clone_first: false.into(),
        }
    }
}

impl<C, T> Consumer<T> for IntersperseConsumer<C, T>
where
    C: Consumer<T>,
    T: Clone + Send,
{
    type Folder = IntersperseFolder<C::Folder, T>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(mut self, index: usize) -> (Self, Self, Self::Reducer) {
        // We'll feed twice as many items to the base consumer, except if we're
        // not currently leading with a cloned item, then it's one less.
        let base_index = index + index.saturating_sub(!self.clone_first.get() as usize);
        let (left, right, reducer) = self.base.split_at(base_index);

        let right = IntersperseConsumer {
            base: right,
            item: self.item.clone(),
            clone_first: true.into(),
        };
        self.base = left;
        (self, right, reducer)
    }

    fn into_folder(self) -> Self::Folder {
        IntersperseFolder {
            base: self.base.into_folder(),
            item: self.item,
            clone_first: self.clone_first.get(),
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<C, T> UnindexedConsumer<T> for IntersperseConsumer<C, T>
where
    C: UnindexedConsumer<T>,
    T: Clone + Send,
{
    fn split_off_left(&self) -> Self {
        let left = IntersperseConsumer {
            base: self.base.split_off_left(),
            item: self.item.clone(),
            clone_first: self.clone_first.clone(),
        };
        self.clone_first.set(true);
        left
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct IntersperseFolder<C, T> {
    base: C,
    item: T,
    clone_first: bool,
}

impl<C, T> Folder<T> for IntersperseFolder<C, T>
where
    C: Folder<T>,
    T: Clone,
{
    type Result = C::Result;

    fn consume(mut self, item: T) -> Self {
        if self.clone_first {
            self.base = self.base.consume(self.item.clone());
            if self.base.full() {
                return self;
            }
        } else {
            self.clone_first = true;
        }
        self.base = self.base.consume(item);
        self
    }

    fn consume_iter<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut clone_first = self.clone_first;
        let between_item = self.item;
        let base = self.base.consume_iter(iter.into_iter().flat_map(|item| {
            let first = if clone_first {
                Some(between_item.clone())
            } else {
                clone_first = true;
                None
            };
            first.into_iter().chain(iter::once(item))
        }));
        IntersperseFolder {
            base,
            item: between_item,
            clone_first,
        }
    }

    fn complete(self) -> C::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}
