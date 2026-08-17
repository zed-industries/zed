//! `proptest`-related features for `nalgebra` data structures.
//!
//! **This module is only available when the `proptest-support` feature is enabled in `nalgebra`**.
//!
//! `proptest` is a library for *property-based testing*. While similar to `QuickCheck`,
//! which may be more familiar to some users, it has a more sophisticated design that
//! provides users with automatic invariant-preserving shrinking. This means that when using
//! `proptest`, you rarely need to write your own shrinkers - which is usually very difficult -
//! and can instead get this "for free". Moreover, `proptest` does not rely on a canonical
//! `Arbitrary` trait implementation like `QuickCheck`, though it does also provide this. For
//! more information, check out the [proptest docs](https://docs.rs/proptest/0.10.1/proptest/)
//! and the [proptest book](https://altsysrq.github.io/proptest-book/intro.html).
//!
//! This module provides users of `nalgebra` with tools to work with `nalgebra` types in
//! `proptest` tests. At present, this integration is at an early stage, and only
//! provides tools for generating matrices and vectors, and not any of the geometry types.
//! There are essentially two ways of using this functionality:
//!
//! - Using the [`matrix()`] function to generate matrices with constraints
//!   on dimensions and elements.
//! - Relying on the [`Arbitrary`] implementation of [`OMatrix`].
//!
//! The first variant is almost always preferred in practice. Read on to discover why.
//!
//! ### Using free function strategies
//!
//! In `proptest`, it is usually preferable to have free functions that generate *strategies*.
//! Currently, the [`matrix()`] function fills this role. The analogous function for
//! column vectors is [`vector()`]. Let's take a quick look at how it may be used:
//! ```
//! use nalgebra::proptest::matrix;
//! use proptest::prelude::*;
//!
//! proptest! {
//!     # /*
//!     #[test]
//!     # */
//!     fn my_test(a in matrix(-5 ..= 5, 2 ..= 4, 1..=4)) {
//!         // Generates matrices with elements in the range -5 ..= 5, rows in 2..=4 and
//!         // columns in 1..=4.
//!     }
//! }
//!
//! # fn main() { my_test(); }
//! ```
//!
//! In the above example, we generate matrices with constraints on the elements, as well as the
//! on the allowed dimensions. When a failing example is found, the resulting shrinking process
//! will preserve these invariants. We can use this to compose more advanced strategies.
//! For example, let's consider a toy example where we need to generate pairs of matrices
//! with exactly 3 rows fixed at compile-time and the same number of columns, but we want the
//! number of columns to vary. One way to do this is to use `proptest` combinators in combination
//! with [`matrix()`] as follows:
//!
//! ```
//! use nalgebra::{Dyn, OMatrix, Const};
//! use nalgebra::proptest::matrix;
//! use proptest::prelude::*;
//!
//! type MyMatrix = OMatrix<i32, Const::<3>, Dyn>;
//!
//! /// Returns a strategy for pairs of matrices with `U3` rows and the same number of
//! /// columns.
//! fn matrix_pairs() -> impl Strategy<Value=(MyMatrix, MyMatrix)> {
//!     matrix(-5 ..= 5, Const::<3>, 0 ..= 10)
//!         // We first generate the initial matrix `a`, and then depending on the concrete
//!         // instances of `a`, we pick a second matrix with the same number of columns
//!         .prop_flat_map(|a| {
//!             let b = matrix(-5 .. 5, Const::<3>, a.ncols());
//!             // This returns a new tuple strategy where we keep `a` fixed while
//!             // the second item is a strategy that generates instances with the same
//!             // dimensions as `a`
//!             (Just(a), b)
//!         })
//! }
//!
//! proptest! {
//!     # /*
//!     #[test]
//!     # */
//!     fn my_test((a, b) in matrix_pairs()) {
//!         // Let's double-check that the two matrices do indeed have the same number of
//!         // columns
//!         prop_assert_eq!(a.ncols(), b.ncols());
//!     }
//! }
//!
//! # fn main() { my_test(); }
//! ```
//!
//! ### The `Arbitrary` implementation
//!
//! If you don't care about the dimensions of matrices, you can write tests like these:
//!
//! ```
//! use nalgebra::{DMatrix, DVector, Dyn, Matrix3, OMatrix, Vector3, U3};
//! use proptest::prelude::*;
//!
//! proptest! {
//!     # /*
//!     #[test]
//!     # */
//!     fn test_dynamic(matrix: DMatrix<i32>) {
//!         // This will generate arbitrary instances of `DMatrix` and also attempt
//!         // to shrink/simplify them when test failures are encountered.
//!     }
//!
//!     # /*
//!     #[test]
//!     # */
//!     fn test_static_and_mixed(matrix: Matrix3<i32>, matrix2: OMatrix<i32, U3, Dyn>) {
//!         // Test some property involving these matrices
//!     }
//!
//!     # /*
//!     #[test]
//!     # */
//!     fn test_vectors(fixed_size_vector: Vector3<i32>, dyn_vector: DVector<i32>) {
//!         // Test some property involving these vectors
//!     }
//! }
//!
//! # fn main() { test_dynamic(); test_static_and_mixed(); test_vectors(); }
//! ```
//!
//! While this may be convenient, the default strategies for built-in types in `proptest` can
//! generate *any* number, including integers large enough to easily lead to overflow when used in
//! matrix operations, or even infinity or NaN values for floating-point types. Therefore
//! `Arbitrary` is rarely the method of choice for writing property-based tests.
//!
//! ### Notes on shrinking
//!
//! Due to some limitations of the current implementation, shrinking takes place by first
//! shrinking the matrix elements before trying to shrink the dimensions of the matrix.
//! This unfortunately often leads to the fact that a large number of shrinking iterations
//! are necessary to find a (nearly) minimal failing test case. As a workaround for this,
//! you can increase the maximum number of shrinking iterations when debugging. To do this,
//! simply set the `PROPTEST_MAX_SHRINK_ITERS` variable to a high number. For example:
//!
//! ```text
//! PROPTEST_MAX_SHRINK_ITERS=100000 cargo test my_failing_test
//! ```
use crate::allocator::Allocator;
use crate::{Const, DefaultAllocator, Dim, DimName, Dyn, OMatrix, Scalar, U1};
use proptest::arbitrary::Arbitrary;
use proptest::collection::vec;
use proptest::strategy::{BoxedStrategy, Just, NewTree, Strategy, ValueTree};
use proptest::test_runner::TestRunner;

use std::ops::RangeInclusive;

/// Parameters for arbitrary matrix generation.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MatrixParameters<NParameters, R, C> {
    /// The range of rows that may be generated.
    pub rows: DimRange<R>,
    /// The range of columns that may be generated.
    pub cols: DimRange<C>,
    /// Parameters for the `Arbitrary` implementation of the scalar values.
    pub value_parameters: NParameters,
}

/// A range of allowed dimensions for use in generation of matrices.
///
/// The `DimRange` type is used to encode the range of dimensions that can be used for generation
/// of matrices with `proptest`. In most cases, you do not need to concern yourself with
/// `DimRange` directly, as it supports conversion from other types such as `U3` or inclusive
/// ranges such as `5 ..= 6`. The latter example corresponds to dimensions from (inclusive)
/// `Dyn(5)` to `Dyn(6)` (inclusive).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DimRange<D = Dyn>(RangeInclusive<D>);

impl<D: Dim> DimRange<D> {
    /// The lower bound for dimensions generated.
    pub fn lower_bound(&self) -> D {
        *self.0.start()
    }

    /// The upper bound for dimensions generated.
    pub fn upper_bound(&self) -> D {
        *self.0.end()
    }
}

impl<D: Dim> From<D> for DimRange<D> {
    fn from(dim: D) -> Self {
        DimRange(dim..=dim)
    }
}

impl<D: Dim> From<RangeInclusive<D>> for DimRange<D> {
    fn from(range: RangeInclusive<D>) -> Self {
        DimRange(range)
    }
}

impl From<RangeInclusive<usize>> for DimRange<Dyn> {
    fn from(range: RangeInclusive<usize>) -> Self {
        DimRange::from(Dyn(*range.start())..=Dyn(*range.end()))
    }
}

impl<D: Dim> DimRange<D> {
    /// Converts the `DimRange` into an instance of `RangeInclusive`.
    pub fn to_range_inclusive(&self) -> RangeInclusive<usize> {
        self.lower_bound().value()..=self.upper_bound().value()
    }
}

impl From<usize> for DimRange<Dyn> {
    fn from(dim: usize) -> Self {
        DimRange::from(Dyn(dim))
    }
}

/// The default range used for Dyn dimensions when generating arbitrary matrices.
fn dynamic_dim_range() -> DimRange<Dyn> {
    DimRange::from(0..=6)
}

/// Create a strategy to generate matrices containing values drawn from the given strategy,
/// with rows and columns in the provided ranges.
///
/// ## Examples
/// ```
/// use nalgebra::proptest::matrix;
/// use nalgebra::{OMatrix, Const, Dyn};
/// use proptest::prelude::*;
///
/// proptest! {
///     # /*
///     #[test]
///     # */
///     fn my_test(a in matrix(0 .. 5i32, Const::<3>, 0 ..= 5)) {
///         // Let's make sure we've got the correct type first
///         let a: OMatrix<_, Const::<3>, Dyn> = a;
///         prop_assert!(a.nrows() == 3);
///         prop_assert!(a.ncols() <= 5);
///         prop_assert!(a.iter().all(|x_ij| *x_ij >= 0 && *x_ij < 5));
///     }
/// }
///
/// # fn main() { my_test(); }
/// ```
///
/// ## Limitations
/// The current implementation has some limitations that lead to suboptimal shrinking behavior.
/// See the [module-level documentation](index.html) for more.
pub fn matrix<R, C, ScalarStrategy>(
    value_strategy: ScalarStrategy,
    rows: impl Into<DimRange<R>>,
    cols: impl Into<DimRange<C>>,
) -> MatrixStrategy<ScalarStrategy, R, C>
where
    ScalarStrategy: Strategy + Clone + 'static,
    ScalarStrategy::Value: Scalar,
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<R, C>,
{
    matrix_(value_strategy, rows.into(), cols.into())
}

/// Same as `matrix`, but without the additional anonymous generic types
fn matrix_<R, C, ScalarStrategy>(
    value_strategy: ScalarStrategy,
    rows: DimRange<R>,
    cols: DimRange<C>,
) -> MatrixStrategy<ScalarStrategy, R, C>
where
    ScalarStrategy: Strategy + Clone + 'static,
    ScalarStrategy::Value: Scalar,
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<R, C>,
{
    let nrows = rows.lower_bound().value()..=rows.upper_bound().value();
    let ncols = cols.lower_bound().value()..=cols.upper_bound().value();

    // Even though we can use this function to generate fixed-size matrices,
    // we currently generate all matrices with heap allocated Vec data.
    // TODO: Avoid heap allocation for fixed-size matrices.
    // Doing this *properly* would probably require us to implement a custom
    // strategy and valuetree with custom shrinking logic, which is not trivial

    // Perhaps more problematic, however, is the poor shrinking behavior the current setup leads to.
    // Shrinking in proptest basically happens in "reverse" of the combinators, so
    // by first generating the dimensions and then the elements, we get shrinking that first
    // tries to completely shrink the individual elements before trying to reduce the dimension.
    // This is clearly the opposite of what we want. I can't find any good way around this
    // short of writing our own custom value tree, which we should probably do at some point.
    // TODO: Custom implementation of value tree for better shrinking behavior.

    let strategy = nrows
        .prop_flat_map(move |nrows| (Just(nrows), ncols.clone()))
        .prop_flat_map(move |(nrows, ncols)| {
            (
                Just(nrows),
                Just(ncols),
                vec(value_strategy.clone(), nrows * ncols),
            )
        })
        .prop_map(|(nrows, ncols, values)| {
            // Note: R/C::from_usize will panic if nrows/ncols does not fit in the dimension type.
            // However, this should never fail, because we should only be generating
            // this stuff in the first place
            OMatrix::from_iterator_generic(R::from_usize(nrows), C::from_usize(ncols), values)
        })
        .boxed();

    MatrixStrategy { strategy }
}

/// Create a strategy to generate column vectors containing values drawn from the given strategy,
/// with length in the provided range.
///
/// This is a convenience function for calling
/// [`matrix(value_strategy, length, U1)`](crate::matrix) and should
/// be used when you only want to generate column vectors, as it's simpler and makes the intent
/// clear.
pub fn vector<D, ScalarStrategy>(
    value_strategy: ScalarStrategy,
    length: impl Into<DimRange<D>>,
) -> MatrixStrategy<ScalarStrategy, D, U1>
where
    ScalarStrategy: Strategy + Clone + 'static,
    ScalarStrategy::Value: Scalar,
    D: Dim,
    DefaultAllocator: Allocator<D>,
{
    matrix_(value_strategy, length.into(), Const::<1>.into())
}

impl<NParameters, R, C> Default for MatrixParameters<NParameters, R, C>
where
    NParameters: Default,
    R: DimName,
    C: DimName,
{
    fn default() -> Self {
        Self {
            rows: DimRange::from(R::name()),
            cols: DimRange::from(C::name()),
            value_parameters: NParameters::default(),
        }
    }
}

impl<NParameters, R> Default for MatrixParameters<NParameters, R, Dyn>
where
    NParameters: Default,
    R: DimName,
{
    fn default() -> Self {
        Self {
            rows: DimRange::from(R::name()),
            cols: dynamic_dim_range(),
            value_parameters: NParameters::default(),
        }
    }
}

impl<NParameters, C> Default for MatrixParameters<NParameters, Dyn, C>
where
    NParameters: Default,
    C: DimName,
{
    fn default() -> Self {
        Self {
            rows: dynamic_dim_range(),
            cols: DimRange::from(C::name()),
            value_parameters: NParameters::default(),
        }
    }
}

impl<NParameters> Default for MatrixParameters<NParameters, Dyn, Dyn>
where
    NParameters: Default,
{
    fn default() -> Self {
        Self {
            rows: dynamic_dim_range(),
            cols: dynamic_dim_range(),
            value_parameters: NParameters::default(),
        }
    }
}

impl<T, R, C> Arbitrary for OMatrix<T, R, C>
where
    T: Scalar + Arbitrary,
    <T as Arbitrary>::Strategy: Clone,
    R: Dim,
    C: Dim,
    MatrixParameters<T::Parameters, R, C>: Default,
    DefaultAllocator: Allocator<R, C>,
{
    type Parameters = MatrixParameters<T::Parameters, R, C>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let value_strategy = T::arbitrary_with(args.value_parameters);
        matrix(value_strategy, args.rows, args.cols)
    }

    type Strategy = MatrixStrategy<T::Strategy, R, C>;
}

/// A strategy for generating matrices.
#[derive(Debug, Clone)]
pub struct MatrixStrategy<NStrategy, R: Dim, C: Dim>
where
    NStrategy: Strategy,
    NStrategy::Value: Scalar,
    DefaultAllocator: Allocator<R, C>,
{
    // For now we only internally hold a boxed strategy. The reason for introducing this
    // separate wrapper struct is so that we can replace the strategy logic with custom logic
    // later down the road without introducing significant breaking changes
    strategy: BoxedStrategy<OMatrix<NStrategy::Value, R, C>>,
}

impl<NStrategy, R, C> Strategy for MatrixStrategy<NStrategy, R, C>
where
    NStrategy: Strategy,
    NStrategy::Value: Scalar,
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<R, C>,
{
    type Tree = MatrixValueTree<NStrategy::Value, R, C>;
    type Value = OMatrix<NStrategy::Value, R, C>;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        let underlying_tree = self.strategy.new_tree(runner)?;
        Ok(MatrixValueTree {
            value_tree: underlying_tree,
        })
    }
}

/// A value tree for matrices.
pub struct MatrixValueTree<T, R, C>
where
    T: Scalar,
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<R, C>,
{
    // For now we only wrap a boxed value tree. The reason for wrapping is that this allows us
    // to swap out the value tree logic down the road without significant breaking changes.
    value_tree: Box<dyn ValueTree<Value = OMatrix<T, R, C>>>,
}

impl<T, R, C> ValueTree for MatrixValueTree<T, R, C>
where
    T: Scalar,
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<R, C>,
{
    type Value = OMatrix<T, R, C>;

    fn current(&self) -> Self::Value {
        self.value_tree.current()
    }

    fn simplify(&mut self) -> bool {
        self.value_tree.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.value_tree.complicate()
    }
}
