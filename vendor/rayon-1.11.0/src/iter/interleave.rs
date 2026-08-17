use super::plumbing::*;
use super::*;
use std::iter::Fuse;

/// `Interleave` is an iterator that interleaves elements of iterators
/// `i` and `j` in one continuous iterator. This struct is created by
/// the [`interleave()`] method on [`IndexedParallelIterator`]
///
/// [`interleave()`]: IndexedParallelIterator::interleave()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Interleave<I, J> {
    i: I,
    j: J,
}

impl<I, J> Interleave<I, J> {
    /// Creates a new `Interleave` iterator
    pub(super) fn new(i: I, j: J) -> Self {
        Interleave { i, j }
    }
}

impl<I, J> ParallelIterator for Interleave<I, J>
where
    I: IndexedParallelIterator,
    J: IndexedParallelIterator<Item = I::Item>,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<I::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<I, J> IndexedParallelIterator for Interleave<I, J>
where
    I: IndexedParallelIterator,
    J: IndexedParallelIterator<Item = I::Item>,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.i.len().checked_add(self.j.len()).expect("overflow")
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        let (i_len, j_len) = (self.i.len(), self.j.len());
        return self.i.with_producer(CallbackI {
            callback,
            i_len,
            j_len,
            i_next: false,
            j: self.j,
        });

        struct CallbackI<CB, J> {
            callback: CB,
            i_len: usize,
            j_len: usize,
            i_next: bool,
            j: J,
        }

        impl<CB, J> ProducerCallback<J::Item> for CallbackI<CB, J>
        where
            J: IndexedParallelIterator,
            CB: ProducerCallback<J::Item>,
        {
            type Output = CB::Output;

            fn callback<I>(self, i_producer: I) -> Self::Output
            where
                I: Producer<Item = J::Item>,
            {
                self.j.with_producer(CallbackJ {
                    i_producer,
                    i_len: self.i_len,
                    j_len: self.j_len,
                    i_next: self.i_next,
                    callback: self.callback,
                })
            }
        }

        struct CallbackJ<CB, I> {
            callback: CB,
            i_len: usize,
            j_len: usize,
            i_next: bool,
            i_producer: I,
        }

        impl<CB, I> ProducerCallback<I::Item> for CallbackJ<CB, I>
        where
            I: Producer,
            CB: ProducerCallback<I::Item>,
        {
            type Output = CB::Output;

            fn callback<J>(self, j_producer: J) -> Self::Output
            where
                J: Producer<Item = I::Item>,
            {
                let producer = InterleaveProducer::new(
                    self.i_producer,
                    j_producer,
                    self.i_len,
                    self.j_len,
                    self.i_next,
                );
                self.callback.callback(producer)
            }
        }
    }
}

struct InterleaveProducer<I, J>
where
    I: Producer,
    J: Producer<Item = I::Item>,
{
    i: I,
    j: J,
    i_len: usize,
    j_len: usize,
    i_next: bool,
}

impl<I, J> InterleaveProducer<I, J>
where
    I: Producer,
    J: Producer<Item = I::Item>,
{
    fn new(i: I, j: J, i_len: usize, j_len: usize, i_next: bool) -> InterleaveProducer<I, J> {
        InterleaveProducer {
            i,
            j,
            i_len,
            j_len,
            i_next,
        }
    }
}

impl<I, J> Producer for InterleaveProducer<I, J>
where
    I: Producer,
    J: Producer<Item = I::Item>,
{
    type Item = I::Item;
    type IntoIter = InterleaveSeq<I::IntoIter, J::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        InterleaveSeq {
            i: self.i.into_iter().fuse(),
            j: self.j.into_iter().fuse(),
            i_next: self.i_next,
        }
    }

    fn min_len(&self) -> usize {
        Ord::max(self.i.min_len(), self.j.min_len())
    }

    fn max_len(&self) -> usize {
        Ord::min(self.i.max_len(), self.j.max_len())
    }

    /// We know 0 < index <= self.i_len + self.j_len
    ///
    /// Find a, b satisfying:
    ///
    ///  (1) 0 < a <= self.i_len
    ///  (2) 0 < b <= self.j_len
    ///  (3) a + b == index
    ///
    /// For even splits, set a = b = index/2.
    /// For odd splits, set a = (index/2)+1, b = index/2, if `i`
    /// should yield the next element, otherwise, if `j` should yield
    /// the next element, set a = index/2 and b = (index/2)+1
    fn split_at(self, index: usize) -> (Self, Self) {
        #[inline]
        fn odd_offset(flag: bool) -> usize {
            (!flag) as usize
        }

        let even = index % 2 == 0;
        let idx = index >> 1;

        // desired split
        let (i_idx, j_idx) = (
            idx + odd_offset(even || self.i_next),
            idx + odd_offset(even || !self.i_next),
        );

        let (i_split, j_split) = if self.i_len >= i_idx && self.j_len >= j_idx {
            (i_idx, j_idx)
        } else if self.i_len >= i_idx {
            // j too short
            (index - self.j_len, self.j_len)
        } else {
            // i too short
            (self.i_len, index - self.i_len)
        };

        let trailing_i_next = even == self.i_next;
        let (i_left, i_right) = self.i.split_at(i_split);
        let (j_left, j_right) = self.j.split_at(j_split);

        (
            InterleaveProducer::new(i_left, j_left, i_split, j_split, self.i_next),
            InterleaveProducer::new(
                i_right,
                j_right,
                self.i_len - i_split,
                self.j_len - j_split,
                trailing_i_next,
            ),
        )
    }
}

/// Wrapper for Interleave to implement DoubleEndedIterator and
/// ExactSizeIterator.
///
/// This iterator is fused.
struct InterleaveSeq<I, J> {
    i: Fuse<I>,
    j: Fuse<J>,

    /// Flag to control which iterator should provide the next element. When
    /// `false` then `i` produces the next element, otherwise `j` produces the
    /// next element.
    i_next: bool,
}

/// Iterator implementation for InterleaveSeq. This implementation is
/// taken more or less verbatim from itertools. It is replicated here
/// (instead of calling itertools directly), because we also need to
/// implement `DoubledEndedIterator` and `ExactSizeIterator`.
impl<I, J> Iterator for InterleaveSeq<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.i_next = !self.i_next;
        if self.i_next {
            match self.i.next() {
                None => self.j.next(),
                r => r,
            }
        } else {
            match self.j.next() {
                None => self.i.next(),
                r => r,
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (ih, jh) = (self.i.size_hint(), self.j.size_hint());
        let min = ih.0.saturating_add(jh.0);
        let max = match (ih.1, jh.1) {
            (Some(x), Some(y)) => x.checked_add(y),
            _ => None,
        };
        (min, max)
    }
}

// The implementation for DoubleEndedIterator requires
// ExactSizeIterator to provide `next_back()`. The last element will
// come from the iterator that runs out last (ie has the most elements
// in it). If the iterators have the same number of elements, then the
// last iterator will provide the last element.
impl<I, J> DoubleEndedIterator for InterleaveSeq<I, J>
where
    I: DoubleEndedIterator + ExactSizeIterator,
    J: DoubleEndedIterator<Item = I::Item> + ExactSizeIterator<Item = I::Item>,
{
    #[inline]
    fn next_back(&mut self) -> Option<I::Item> {
        match self.i.len().cmp(&self.j.len()) {
            Ordering::Less => self.j.next_back(),
            Ordering::Equal => {
                if self.i_next {
                    self.i.next_back()
                } else {
                    self.j.next_back()
                }
            }
            Ordering::Greater => self.i.next_back(),
        }
    }
}

impl<I, J> ExactSizeIterator for InterleaveSeq<I, J>
where
    I: ExactSizeIterator,
    J: ExactSizeIterator<Item = I::Item>,
{
    #[inline]
    fn len(&self) -> usize {
        self.i.len() + self.j.len()
    }
}
