//! Traits and functions used to implement parallel iteration.  These are
//! low-level details -- users of parallel iterators should not need to
//! interact with them directly.  See [the `plumbing` README][r] for a general overview.
//!
//! [r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md

use crate::join_context;

use super::IndexedParallelIterator;

/// The `ProducerCallback` trait is a kind of generic closure,
/// [analogous to `FnOnce`][FnOnce]. See [the corresponding section in
/// the plumbing README][r] for more details.
///
/// [r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md#producer-callback
/// [FnOnce]: std::ops::FnOnce
pub trait ProducerCallback<T> {
    /// The type of value returned by this callback. Analogous to
    /// [`Output` from the `FnOnce` trait][Output].
    ///
    /// [Output]: std::ops::FnOnce::Output
    type Output;

    /// Invokes the callback with the given producer as argument. The
    /// key point of this trait is that this method is generic over
    /// `P`, and hence implementors must be defined for any producer.
    fn callback<P>(self, producer: P) -> Self::Output
    where
        P: Producer<Item = T>;
}

/// A `Producer` is effectively a "splittable `IntoIterator`". That
/// is, a producer is a value which can be converted into an iterator
/// at any time: at that point, it simply produces items on demand,
/// like any iterator. But what makes a `Producer` special is that,
/// *before* we convert to an iterator, we can also **split** it at a
/// particular point using the `split_at` method. This will yield up
/// two producers, one producing the items before that point, and one
/// producing the items after that point (these two producers can then
/// independently be split further, or be converted into iterators).
/// In Rayon, this splitting is used to divide between threads.
/// See [the `plumbing` README][r] for further details.
///
/// Note that each producer will always produce a fixed number of
/// items N. However, this number N is not queryable through the API;
/// the consumer is expected to track it.
///
/// NB. You might expect `Producer` to extend the `IntoIterator`
/// trait.  However, [rust-lang/rust#20671][20671] prevents us from
/// declaring the DoubleEndedIterator and ExactSizeIterator
/// constraints on a required IntoIterator trait, so we inline
/// IntoIterator here until that issue is fixed.
///
/// [r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md
/// [20671]: https://github.com/rust-lang/rust/issues/20671
pub trait Producer: Send + Sized {
    /// The type of item that will be produced by this producer once
    /// it is converted into an iterator.
    type Item;

    /// The type of iterator we will become.
    type IntoIter: Iterator<Item = Self::Item> + DoubleEndedIterator + ExactSizeIterator;

    /// Convert `self` into an iterator; at this point, no more parallel splits
    /// are possible.
    fn into_iter(self) -> Self::IntoIter;

    /// The minimum number of items that we will process
    /// sequentially. Defaults to 1, which means that we will split
    /// all the way down to a single item. This can be raised higher
    /// using the [`with_min_len`] method, which will force us to
    /// create sequential tasks at a larger granularity. Note that
    /// Rayon automatically normally attempts to adjust the size of
    /// parallel splits to reduce overhead, so this should not be
    /// needed.
    ///
    /// [`with_min_len`]: super::IndexedParallelIterator::with_min_len()
    fn min_len(&self) -> usize {
        1
    }

    /// The maximum number of items that we will process
    /// sequentially. Defaults to MAX, which means that we can choose
    /// not to split at all. This can be lowered using the
    /// [`with_max_len`] method, which will force us to create more
    /// parallel tasks. Note that Rayon automatically normally
    /// attempts to adjust the size of parallel splits to reduce
    /// overhead, so this should not be needed.
    ///
    /// [`with_max_len`]: super::IndexedParallelIterator::with_max_len()
    fn max_len(&self) -> usize {
        usize::MAX
    }

    /// Split into two producers; one produces items `0..index`, the
    /// other `index..N`. Index must be less than or equal to `N`.
    fn split_at(self, index: usize) -> (Self, Self);

    /// Iterate the producer, feeding each element to `folder`, and
    /// stop when the folder is full (or all elements have been consumed).
    ///
    /// The provided implementation is sufficient for most iterables.
    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        folder.consume_iter(self.into_iter())
    }
}

/// A consumer is effectively a [generalized "fold" operation][fold],
/// and in fact each consumer will eventually be converted into a
/// [`Folder`]. What makes a consumer special is that, like a
/// [`Producer`], it can be **split** into multiple consumers using
/// the `split_at` method. When a consumer is split, it produces two
/// consumers, as well as a **reducer**. The two consumers can be fed
/// items independently, and when they are done the reducer is used to
/// combine their two results into one. See [the `plumbing`
/// README][r] for further details.
///
/// [r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md
/// [fold]: Iterator::fold()
pub trait Consumer<Item>: Send + Sized {
    /// The type of folder that this consumer can be converted into.
    type Folder: Folder<Item, Result = Self::Result>;

    /// The type of reducer that is produced if this consumer is split.
    type Reducer: Reducer<Self::Result>;

    /// The type of result that this consumer will ultimately produce.
    type Result: Send;

    /// Divide the consumer into two consumers, one processing items
    /// `0..index` and one processing items from `index..`. Also
    /// produces a reducer that can be used to reduce the results at
    /// the end.
    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer);

    /// Convert the consumer into a folder that can consume items
    /// sequentially, eventually producing a final result.
    fn into_folder(self) -> Self::Folder;

    /// Hint whether this `Consumer` would like to stop processing
    /// further items, e.g. if a search has been completed.
    fn full(&self) -> bool;
}

/// The `Folder` trait encapsulates [the standard fold
/// operation][fold].  It can be fed many items using the `consume`
/// method. At the end, once all items have been consumed, it can then
/// be converted (using `complete`) into a final value.
///
/// [fold]: Iterator::fold()
pub trait Folder<Item>: Sized {
    /// The type of result that will ultimately be produced by the folder.
    type Result;

    /// Consume next item and return new sequential state.
    fn consume(self, item: Item) -> Self;

    /// Consume items from the iterator until full, and return new sequential state.
    ///
    /// This method is **optional**. The default simply iterates over
    /// `iter`, invoking `consume` and checking after each iteration
    /// whether `full` returns false.
    ///
    /// The main reason to override it is if you can provide a more
    /// specialized, efficient implementation.
    fn consume_iter<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = Item>,
    {
        for item in iter {
            self = self.consume(item);
            if self.full() {
                break;
            }
        }
        self
    }

    /// Finish consuming items, produce final result.
    fn complete(self) -> Self::Result;

    /// Hint whether this `Folder` would like to stop processing
    /// further items, e.g. if a search has been completed.
    fn full(&self) -> bool;
}

/// The reducer is the final step of a `Consumer` -- after a consumer
/// has been split into two parts, and each of those parts has been
/// fully processed, we are left with two results. The reducer is then
/// used to combine those two results into one. See [the `plumbing`
/// README][r] for further details.
///
/// [r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md
pub trait Reducer<Result> {
    /// Reduce two final results into one; this is executed after a
    /// split.
    fn reduce(self, left: Result, right: Result) -> Result;
}

/// A stateless consumer can be freely copied. These consumers can be
/// used like regular consumers, but they also support a
/// `split_off_left` method that does not take an index to split, but
/// simply splits at some arbitrary point (`for_each`, for example,
/// produces an unindexed consumer).
pub trait UnindexedConsumer<I>: Consumer<I> {
    /// Splits off a "left" consumer and returns it. The `self`
    /// consumer should then be used to consume the "right" portion of
    /// the data. (The ordering matters for methods like find_first --
    /// values produced by the returned value are given precedence
    /// over values produced by `self`.) Once the left and right
    /// halves have been fully consumed, you should reduce the results
    /// with the result of `to_reducer`.
    fn split_off_left(&self) -> Self;

    /// Creates a reducer that can be used to combine the results from
    /// a split consumer.
    fn to_reducer(&self) -> Self::Reducer;
}

/// A variant on `Producer` which does not know its exact length or
/// cannot represent it in a `usize`. These producers act like
/// ordinary producers except that they cannot be told to split at a
/// particular point. Instead, you just ask them to split 'somewhere'.
///
/// (In principle, `Producer` could extend this trait; however, it
/// does not because to do so would require producers to carry their
/// own length with them.)
pub trait UnindexedProducer: Send + Sized {
    /// The type of item returned by this producer.
    type Item;

    /// Split midway into a new producer if possible, otherwise return `None`.
    fn split(self) -> (Self, Option<Self>);

    /// Iterate the producer, feeding each element to `folder`, and
    /// stop when the folder is full (or all elements have been consumed).
    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>;
}

/// A splitter controls the policy for splitting into smaller work items.
///
/// Thief-splitting is an adaptive policy that starts by splitting into
/// enough jobs for every worker thread, and then resets itself whenever a
/// job is actually stolen into a different thread.
#[derive(Clone, Copy)]
struct Splitter {
    /// The `splits` tell us approximately how many remaining times we'd
    /// like to split this job.  We always just divide it by two though, so
    /// the effective number of pieces will be `next_power_of_two()`.
    splits: usize,
}

impl Splitter {
    #[inline]
    fn new() -> Splitter {
        Splitter {
            splits: crate::current_num_threads(),
        }
    }

    #[inline]
    fn try_split(&mut self, stolen: bool) -> bool {
        let Splitter { splits } = *self;

        if stolen {
            // This job was stolen!  Reset the number of desired splits to the
            // thread count, if that's more than we had remaining anyway.
            self.splits = Ord::max(crate::current_num_threads(), self.splits / 2);
            true
        } else if splits > 0 {
            // We have splits remaining, make it so.
            self.splits /= 2;
            true
        } else {
            // Not stolen, and no more splits -- we're done!
            false
        }
    }
}

/// The length splitter is built on thief-splitting, but additionally takes
/// into account the remaining length of the iterator.
#[derive(Clone, Copy)]
struct LengthSplitter {
    inner: Splitter,

    /// The smallest we're willing to divide into.  Usually this is just 1,
    /// but you can choose a larger working size with `with_min_len()`.
    min: usize,
}

impl LengthSplitter {
    /// Creates a new splitter based on lengths.
    ///
    /// The `min` is a hard lower bound.  We'll never split below that, but
    /// of course an iterator might start out smaller already.
    ///
    /// The `max` is an upper bound on the working size, used to determine
    /// the minimum number of times we need to split to get under that limit.
    /// The adaptive algorithm may very well split even further, but never
    /// smaller than the `min`.
    #[inline]
    fn new(min: usize, max: usize, len: usize) -> LengthSplitter {
        let mut splitter = LengthSplitter {
            inner: Splitter::new(),
            min: Ord::max(min, 1),
        };

        // Divide the given length by the max working length to get the minimum
        // number of splits we need to get under that max.  This rounds down,
        // but the splitter actually gives `next_power_of_two()` pieces anyway.
        // e.g. len 12345 / max 100 = 123 min_splits -> 128 pieces.
        let min_splits = len / Ord::max(max, 1);

        // Only update the value if it's not splitting enough already.
        if min_splits > splitter.inner.splits {
            splitter.inner.splits = min_splits;
        }

        splitter
    }

    #[inline]
    fn try_split(&mut self, len: usize, stolen: bool) -> bool {
        // If splitting wouldn't make us too small, try the inner splitter.
        len / 2 >= self.min && self.inner.try_split(stolen)
    }
}

/// This helper function is used to "connect" a parallel iterator to a
/// consumer. It will convert the `par_iter` into a producer P and
/// then pull items from P and feed them to `consumer`, splitting and
/// creating parallel threads as needed.
///
/// This is useful when you are implementing your own parallel
/// iterators: it is often used as the definition of the
/// [`drive_unindexed`] or [`drive`] methods.
///
/// [`drive_unindexed`]: super::ParallelIterator::drive_unindexed()
/// [`drive`]: super::IndexedParallelIterator::drive()
pub fn bridge<I, C>(par_iter: I, consumer: C) -> C::Result
where
    I: IndexedParallelIterator,
    C: Consumer<I::Item>,
{
    let len = par_iter.len();
    return par_iter.with_producer(Callback { len, consumer });

    struct Callback<C> {
        len: usize,
        consumer: C,
    }

    impl<C, I> ProducerCallback<I> for Callback<C>
    where
        C: Consumer<I>,
    {
        type Output = C::Result;
        fn callback<P>(self, producer: P) -> C::Result
        where
            P: Producer<Item = I>,
        {
            bridge_producer_consumer(self.len, producer, self.consumer)
        }
    }
}

/// This helper function is used to "connect" a producer and a
/// consumer. You may prefer to call [`bridge()`], which wraps this
/// function. This function will draw items from `producer` and feed
/// them to `consumer`, splitting and creating parallel tasks when
/// needed.
///
/// This is useful when you are implementing your own parallel
/// iterators: it is often used as the definition of the
/// [`drive_unindexed`] or [`drive`] methods.
///
/// [`drive_unindexed`]: super::ParallelIterator::drive_unindexed()
/// [`drive`]: super::IndexedParallelIterator::drive()
pub fn bridge_producer_consumer<P, C>(len: usize, producer: P, consumer: C) -> C::Result
where
    P: Producer,
    C: Consumer<P::Item>,
{
    let splitter = LengthSplitter::new(producer.min_len(), producer.max_len(), len);
    return helper(len, false, splitter, producer, consumer);

    fn helper<P, C>(
        len: usize,
        migrated: bool,
        mut splitter: LengthSplitter,
        producer: P,
        consumer: C,
    ) -> C::Result
    where
        P: Producer,
        C: Consumer<P::Item>,
    {
        if consumer.full() {
            consumer.into_folder().complete()
        } else if splitter.try_split(len, migrated) {
            let mid = len / 2;
            let (left_producer, right_producer) = producer.split_at(mid);
            let (left_consumer, right_consumer, reducer) = consumer.split_at(mid);
            let (left_result, right_result) = join_context(
                |context| {
                    helper(
                        mid,
                        context.migrated(),
                        splitter,
                        left_producer,
                        left_consumer,
                    )
                },
                |context| {
                    helper(
                        len - mid,
                        context.migrated(),
                        splitter,
                        right_producer,
                        right_consumer,
                    )
                },
            );
            reducer.reduce(left_result, right_result)
        } else {
            producer.fold_with(consumer.into_folder()).complete()
        }
    }
}

/// A variant of [`bridge_producer_consumer()`] where the producer is an unindexed producer.
pub fn bridge_unindexed<P, C>(producer: P, consumer: C) -> C::Result
where
    P: UnindexedProducer,
    C: UnindexedConsumer<P::Item>,
{
    let splitter = Splitter::new();
    bridge_unindexed_producer_consumer(false, splitter, producer, consumer)
}

fn bridge_unindexed_producer_consumer<P, C>(
    migrated: bool,
    mut splitter: Splitter,
    producer: P,
    consumer: C,
) -> C::Result
where
    P: UnindexedProducer,
    C: UnindexedConsumer<P::Item>,
{
    if consumer.full() {
        consumer.into_folder().complete()
    } else if splitter.try_split(migrated) {
        match producer.split() {
            (left_producer, Some(right_producer)) => {
                let (reducer, left_consumer, right_consumer) =
                    (consumer.to_reducer(), consumer.split_off_left(), consumer);
                let bridge = bridge_unindexed_producer_consumer;
                let (left_result, right_result) = join_context(
                    |context| bridge(context.migrated(), splitter, left_producer, left_consumer),
                    |context| bridge(context.migrated(), splitter, right_producer, right_consumer),
                );
                reducer.reduce(left_result, right_result)
            }
            (producer, None) => producer.fold_with(consumer.into_folder()).complete(),
        }
    } else {
        producer.fold_with(consumer.into_folder()).complete()
    }
}
