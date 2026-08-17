use super::plumbing::*;
use super::*;

/// This trait abstracts the different ways we can "unzip" one parallel
/// iterator into two distinct consumers, which we can handle almost
/// identically apart from how to process the individual items.
trait UnzipOp<T>: Sync + Send {
    /// The type of item expected by the left consumer.
    type Left: Send;

    /// The type of item expected by the right consumer.
    type Right: Send;

    /// Consumes one item and feeds it to one or both of the underlying folders.
    fn consume<FA, FB>(&self, item: T, left: FA, right: FB) -> (FA, FB)
    where
        FA: Folder<Self::Left>,
        FB: Folder<Self::Right>;

    /// Reports whether this op may support indexed consumers.
    /// - e.g. true for `unzip` where the item count passed through directly.
    /// - e.g. false for `partition` where the sorting is not yet known.
    fn indexable() -> bool {
        false
    }
}

/// Runs an unzip-like operation into default `ParallelExtend` collections.
fn execute<I, OP, FromA, FromB>(pi: I, op: OP) -> (FromA, FromB)
where
    I: ParallelIterator,
    OP: UnzipOp<I::Item>,
    FromA: Default + Send + ParallelExtend<OP::Left>,
    FromB: Default + Send + ParallelExtend<OP::Right>,
{
    let mut a = FromA::default();
    let mut b = FromB::default();
    execute_into(&mut a, &mut b, pi, op);
    (a, b)
}

/// Runs an unzip-like operation into `ParallelExtend` collections.
fn execute_into<I, OP, FromA, FromB>(a: &mut FromA, b: &mut FromB, pi: I, op: OP)
where
    I: ParallelIterator,
    OP: UnzipOp<I::Item>,
    FromA: Send + ParallelExtend<OP::Left>,
    FromB: Send + ParallelExtend<OP::Right>,
{
    // We have no idea what the consumers will look like for these
    // collections' `par_extend`, but we can intercept them in our own
    // `drive_unindexed`.  Start with the left side, type `A`:
    let iter = UnzipA { base: pi, op, b };
    a.par_extend(iter);
}

/// Unzips the items of a parallel iterator into a pair of arbitrary
/// `ParallelExtend` containers.
///
/// This is called by `ParallelIterator::unzip`.
pub(super) fn unzip<I, A, B, FromA, FromB>(pi: I) -> (FromA, FromB)
where
    I: ParallelIterator<Item = (A, B)>,
    FromA: Default + Send + ParallelExtend<A>,
    FromB: Default + Send + ParallelExtend<B>,
    A: Send,
    B: Send,
{
    execute(pi, Unzip)
}

/// Unzips an `IndexedParallelIterator` into two arbitrary `Consumer`s.
///
/// This is called by `super::collect::unzip_into_vecs`.
pub(super) fn unzip_indexed<I, A, B, CA, CB>(pi: I, left: CA, right: CB) -> (CA::Result, CB::Result)
where
    I: IndexedParallelIterator<Item = (A, B)>,
    CA: Consumer<A>,
    CB: Consumer<B>,
    A: Send,
    B: Send,
{
    let consumer = UnzipConsumer {
        op: &Unzip,
        left,
        right,
    };
    pi.drive(consumer)
}

/// An `UnzipOp` that splits a tuple directly into the two consumers.
struct Unzip;

impl<A: Send, B: Send> UnzipOp<(A, B)> for Unzip {
    type Left = A;
    type Right = B;

    fn consume<FA, FB>(&self, item: (A, B), left: FA, right: FB) -> (FA, FB)
    where
        FA: Folder<A>,
        FB: Folder<B>,
    {
        (left.consume(item.0), right.consume(item.1))
    }

    fn indexable() -> bool {
        true
    }
}

/// Partitions the items of a parallel iterator into a pair of arbitrary
/// `ParallelExtend` containers.
///
/// This is called by `ParallelIterator::partition`.
pub(super) fn partition<I, A, B, P>(pi: I, predicate: P) -> (A, B)
where
    I: ParallelIterator,
    A: Default + Send + ParallelExtend<I::Item>,
    B: Default + Send + ParallelExtend<I::Item>,
    P: Fn(&I::Item) -> bool + Sync + Send,
{
    execute(pi, Partition { predicate })
}

/// An `UnzipOp` that routes items depending on a predicate function.
struct Partition<P> {
    predicate: P,
}

impl<P, T> UnzipOp<T> for Partition<P>
where
    P: Fn(&T) -> bool + Sync + Send,
    T: Send,
{
    type Left = T;
    type Right = T;

    fn consume<FA, FB>(&self, item: T, left: FA, right: FB) -> (FA, FB)
    where
        FA: Folder<T>,
        FB: Folder<T>,
    {
        if (self.predicate)(&item) {
            (left.consume(item), right)
        } else {
            (left, right.consume(item))
        }
    }
}

/// Partitions and maps the items of a parallel iterator into a pair of
/// arbitrary `ParallelExtend` containers.
///
/// This called by `ParallelIterator::partition_map`.
pub(super) fn partition_map<I, A, B, P, L, R>(pi: I, predicate: P) -> (A, B)
where
    I: ParallelIterator,
    A: Default + Send + ParallelExtend<L>,
    B: Default + Send + ParallelExtend<R>,
    P: Fn(I::Item) -> Either<L, R> + Sync + Send,
    L: Send,
    R: Send,
{
    execute(pi, PartitionMap { predicate })
}

/// An `UnzipOp` that routes items depending on how they are mapped `Either`.
struct PartitionMap<P> {
    predicate: P,
}

impl<P, L, R, T> UnzipOp<T> for PartitionMap<P>
where
    P: Fn(T) -> Either<L, R> + Sync + Send,
    L: Send,
    R: Send,
{
    type Left = L;
    type Right = R;

    fn consume<FA, FB>(&self, item: T, left: FA, right: FB) -> (FA, FB)
    where
        FA: Folder<L>,
        FB: Folder<R>,
    {
        match (self.predicate)(item) {
            Either::Left(item) => (left.consume(item), right),
            Either::Right(item) => (left, right.consume(item)),
        }
    }
}

/// A fake iterator to intercept the `Consumer` for type `A`.
struct UnzipA<'b, I, OP, FromB> {
    base: I,
    op: OP,
    b: &'b mut FromB,
}

impl<'b, I, OP, FromB> ParallelIterator for UnzipA<'b, I, OP, FromB>
where
    I: ParallelIterator,
    OP: UnzipOp<I::Item>,
    FromB: Send + ParallelExtend<OP::Right>,
{
    type Item = OP::Left;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let mut result = None;
        {
            // Now it's time to find the consumer for type `B`
            let iter = UnzipB {
                base: self.base,
                op: self.op,
                left_consumer: consumer,
                left_result: &mut result,
            };
            self.b.par_extend(iter);
        }
        // NB: If for some reason `b.par_extend` doesn't actually drive the
        // iterator, then we won't have a result for the left side to return
        // at all.  We can't fake an arbitrary consumer's result, so panic.
        result.expect("unzip consumers didn't execute!")
    }

    fn opt_len(&self) -> Option<usize> {
        if OP::indexable() {
            self.base.opt_len()
        } else {
            None
        }
    }
}

/// A fake iterator to intercept the `Consumer` for type `B`.
struct UnzipB<'r, I, OP, CA>
where
    I: ParallelIterator,
    OP: UnzipOp<I::Item>,
    CA: UnindexedConsumer<OP::Left>,
{
    base: I,
    op: OP,
    left_consumer: CA,
    left_result: &'r mut Option<CA::Result>,
}

impl<'r, I, OP, CA> ParallelIterator for UnzipB<'r, I, OP, CA>
where
    I: ParallelIterator,
    OP: UnzipOp<I::Item>,
    CA: UnindexedConsumer<OP::Left>,
{
    type Item = OP::Right;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        // Now that we have two consumers, we can unzip the real iterator.
        let consumer = UnzipConsumer {
            op: &self.op,
            left: self.left_consumer,
            right: consumer,
        };

        let result = self.base.drive_unindexed(consumer);
        *self.left_result = Some(result.0);
        result.1
    }

    fn opt_len(&self) -> Option<usize> {
        if OP::indexable() {
            self.base.opt_len()
        } else {
            None
        }
    }
}

/// `Consumer` that unzips into two other `Consumer`s
struct UnzipConsumer<'a, OP, CA, CB> {
    op: &'a OP,
    left: CA,
    right: CB,
}

impl<'a, T, OP, CA, CB> Consumer<T> for UnzipConsumer<'a, OP, CA, CB>
where
    OP: UnzipOp<T>,
    CA: Consumer<OP::Left>,
    CB: Consumer<OP::Right>,
{
    type Folder = UnzipFolder<'a, OP, CA::Folder, CB::Folder>;
    type Reducer = UnzipReducer<CA::Reducer, CB::Reducer>;
    type Result = (CA::Result, CB::Result);

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left1, left2, left_reducer) = self.left.split_at(index);
        let (right1, right2, right_reducer) = self.right.split_at(index);

        (
            UnzipConsumer {
                op: self.op,
                left: left1,
                right: right1,
            },
            UnzipConsumer {
                op: self.op,
                left: left2,
                right: right2,
            },
            UnzipReducer {
                left: left_reducer,
                right: right_reducer,
            },
        )
    }

    fn into_folder(self) -> Self::Folder {
        UnzipFolder {
            op: self.op,
            left: self.left.into_folder(),
            right: self.right.into_folder(),
        }
    }

    fn full(&self) -> bool {
        // don't stop until everyone is full
        self.left.full() && self.right.full()
    }
}

impl<'a, T, OP, CA, CB> UnindexedConsumer<T> for UnzipConsumer<'a, OP, CA, CB>
where
    OP: UnzipOp<T>,
    CA: UnindexedConsumer<OP::Left>,
    CB: UnindexedConsumer<OP::Right>,
{
    fn split_off_left(&self) -> Self {
        UnzipConsumer {
            op: self.op,
            left: self.left.split_off_left(),
            right: self.right.split_off_left(),
        }
    }

    fn to_reducer(&self) -> Self::Reducer {
        UnzipReducer {
            left: self.left.to_reducer(),
            right: self.right.to_reducer(),
        }
    }
}

/// `Folder` that unzips into two other `Folder`s
struct UnzipFolder<'a, OP, FA, FB> {
    op: &'a OP,
    left: FA,
    right: FB,
}

impl<'a, T, OP, FA, FB> Folder<T> for UnzipFolder<'a, OP, FA, FB>
where
    OP: UnzipOp<T>,
    FA: Folder<OP::Left>,
    FB: Folder<OP::Right>,
{
    type Result = (FA::Result, FB::Result);

    fn consume(self, item: T) -> Self {
        let (left, right) = self.op.consume(item, self.left, self.right);
        UnzipFolder {
            op: self.op,
            left,
            right,
        }
    }

    fn complete(self) -> Self::Result {
        (self.left.complete(), self.right.complete())
    }

    fn full(&self) -> bool {
        // don't stop until everyone is full
        self.left.full() && self.right.full()
    }
}

/// `Reducer` that unzips into two other `Reducer`s
struct UnzipReducer<RA, RB> {
    left: RA,
    right: RB,
}

impl<A, B, RA, RB> Reducer<(A, B)> for UnzipReducer<RA, RB>
where
    RA: Reducer<A>,
    RB: Reducer<B>,
{
    fn reduce(self, left: (A, B), right: (A, B)) -> (A, B) {
        (
            self.left.reduce(left.0, right.0),
            self.right.reduce(left.1, right.1),
        )
    }
}

impl<A, B, FromA, FromB> ParallelExtend<(A, B)> for (FromA, FromB)
where
    A: Send,
    B: Send,
    FromA: Send + ParallelExtend<A>,
    FromB: Send + ParallelExtend<B>,
{
    fn par_extend<I>(&mut self, pi: I)
    where
        I: IntoParallelIterator<Item = (A, B)>,
    {
        execute_into(&mut self.0, &mut self.1, pi.into_par_iter(), Unzip);
    }
}

impl<L, R, A, B> ParallelExtend<Either<L, R>> for (A, B)
where
    L: Send,
    R: Send,
    A: Send + ParallelExtend<L>,
    B: Send + ParallelExtend<R>,
{
    fn par_extend<I>(&mut self, pi: I)
    where
        I: IntoParallelIterator<Item = Either<L, R>>,
    {
        execute_into(&mut self.0, &mut self.1, pi.into_par_iter(), UnEither);
    }
}

/// An `UnzipOp` that routes items depending on their `Either` variant.
struct UnEither;

impl<L, R> UnzipOp<Either<L, R>> for UnEither
where
    L: Send,
    R: Send,
{
    type Left = L;
    type Right = R;

    fn consume<FL, FR>(&self, item: Either<L, R>, left: FL, right: FR) -> (FL, FR)
    where
        FL: Folder<L>,
        FR: Folder<R>,
    {
        match item {
            Either::Left(item) => (left.consume(item), right),
            Either::Right(item) => (left, right.consume(item)),
        }
    }
}

impl<A, B, FromA, FromB> FromParallelIterator<(A, B)> for (FromA, FromB)
where
    A: Send,
    B: Send,
    FromA: Send + FromParallelIterator<A>,
    FromB: Send + FromParallelIterator<B>,
{
    fn from_par_iter<I>(pi: I) -> Self
    where
        I: IntoParallelIterator<Item = (A, B)>,
    {
        let (a, b): (Collector<FromA>, Collector<FromB>) = pi.into_par_iter().unzip();
        (a.result.unwrap(), b.result.unwrap())
    }
}

impl<L, R, A, B> FromParallelIterator<Either<L, R>> for (A, B)
where
    L: Send,
    R: Send,
    A: Send + FromParallelIterator<L>,
    B: Send + FromParallelIterator<R>,
{
    fn from_par_iter<I>(pi: I) -> Self
    where
        I: IntoParallelIterator<Item = Either<L, R>>,
    {
        fn identity<T>(x: T) -> T {
            x
        }

        let (a, b): (Collector<A>, Collector<B>) = pi.into_par_iter().partition_map(identity);
        (a.result.unwrap(), b.result.unwrap())
    }
}

/// Shim to implement a one-time `ParallelExtend` using `FromParallelIterator`.
struct Collector<FromT> {
    result: Option<FromT>,
}

impl<FromT> Default for Collector<FromT> {
    fn default() -> Self {
        Collector { result: None }
    }
}

impl<T, FromT> ParallelExtend<T> for Collector<FromT>
where
    T: Send,
    FromT: Send + FromParallelIterator<T>,
{
    fn par_extend<I>(&mut self, pi: I)
    where
        I: IntoParallelIterator<Item = T>,
    {
        debug_assert!(self.result.is_none());
        self.result = Some(pi.into_par_iter().collect());
    }
}
