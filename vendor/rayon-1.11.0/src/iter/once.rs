use crate::iter::plumbing::*;
use crate::iter::*;

/// Creates a parallel iterator that produces an element exactly once.
///
/// This admits no parallelism on its own, but it could be chained to existing
/// parallel iterators to extend their contents, or otherwise used for any code
/// that deals with generic parallel iterators.
///
/// # Examples
///
/// ```
/// use rayon::prelude::*;
/// use rayon::iter::once;
///
/// let pi = (0..1234).into_par_iter()
///     .chain(once(-1))
///     .chain(1234..10_000);
///
/// assert_eq!(pi.clone().count(), 10_001);
/// assert_eq!(pi.clone().filter(|&x| x < 0).count(), 1);
/// assert_eq!(pi.position_any(|x| x < 0), Some(1234));
/// ```
pub fn once<T: Send>(item: T) -> Once<T> {
    Once { item }
}

/// Iterator adaptor for [the `once()` function].
///
/// [the `once()` function]: once()
#[derive(Clone, Debug)]
pub struct Once<T> {
    item: T,
}

impl<T: Send> ParallelIterator for Once<T> {
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.drive(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(1)
    }
}

impl<T: Send> IndexedParallelIterator for Once<T> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        consumer.into_folder().consume(self.item).complete()
    }

    fn len(&self) -> usize {
        1
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        // Let `OptionProducer` handle it.
        Some(self.item).into_par_iter().with_producer(callback)
    }
}
