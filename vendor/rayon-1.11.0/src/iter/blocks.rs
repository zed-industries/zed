use super::plumbing::*;
use super::*;

struct BlocksCallback<S, C> {
    sizes: S,
    consumer: C,
    len: usize,
}

impl<T, S, C> ProducerCallback<T> for BlocksCallback<S, C>
where
    C: UnindexedConsumer<T>,
    S: Iterator<Item = usize>,
{
    type Output = C::Result;

    fn callback<P: Producer<Item = T>>(mut self, mut producer: P) -> Self::Output {
        let mut remaining_len = self.len;
        let mut consumer = self.consumer;

        // we need a local variable for the accumulated results
        // we call the reducer's identity by splitting at 0
        let (left_consumer, right_consumer, _) = consumer.split_at(0);
        let mut leftmost_res = left_consumer.into_folder().complete();
        consumer = right_consumer;

        // now we loop on each block size
        while remaining_len > 0 && !consumer.full() {
            // we compute the next block's size
            let size = self.sizes.next().unwrap_or(usize::MAX);
            let capped_size = remaining_len.min(size);
            remaining_len -= capped_size;

            // split the producer
            let (left_producer, right_producer) = producer.split_at(capped_size);
            producer = right_producer;

            // split the consumer
            let (left_consumer, right_consumer, _) = consumer.split_at(capped_size);
            consumer = right_consumer;

            leftmost_res = consumer.to_reducer().reduce(
                leftmost_res,
                bridge_producer_consumer(capped_size, left_producer, left_consumer),
            );
        }
        leftmost_res
    }
}

/// `ExponentialBlocks` is a parallel iterator that consumes itself as a sequence
/// of parallel blocks of increasing sizes (exponentially).
///
/// This struct is created by the [`by_exponential_blocks()`] method on [`IndexedParallelIterator`]
///
/// [`by_exponential_blocks()`]: IndexedParallelIterator::by_exponential_blocks()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct ExponentialBlocks<I> {
    base: I,
}

impl<I> ExponentialBlocks<I> {
    pub(super) fn new(base: I) -> Self {
        Self { base }
    }
}

impl<I> ParallelIterator for ExponentialBlocks<I>
where
    I: IndexedParallelIterator,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let first = crate::current_num_threads();
        let callback = BlocksCallback {
            consumer,
            sizes: std::iter::successors(Some(first), exponential_size),
            len: self.base.len(),
        };
        self.base.with_producer(callback)
    }
}

fn exponential_size(size: &usize) -> Option<usize> {
    Some(size.saturating_mul(2))
}

/// `UniformBlocks` is a parallel iterator that consumes itself as a sequence
/// of parallel blocks of constant sizes.
///
/// This struct is created by the [`by_uniform_blocks()`] method on [`IndexedParallelIterator`]
///
/// [`by_uniform_blocks()`]: IndexedParallelIterator::by_uniform_blocks()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct UniformBlocks<I> {
    base: I,
    block_size: usize,
}

impl<I> UniformBlocks<I> {
    pub(super) fn new(base: I, block_size: usize) -> Self {
        Self { base, block_size }
    }
}

impl<I> ParallelIterator for UniformBlocks<I>
where
    I: IndexedParallelIterator,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let callback = BlocksCallback {
            consumer,
            sizes: std::iter::repeat(self.block_size),
            len: self.base.len(),
        };
        self.base.with_producer(callback)
    }
}
