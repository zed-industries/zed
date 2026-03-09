use core::fmt::Debug;

use proptest::{prelude::*, sample::SizeRange};

use crate::{Item, SumTree, Summary};

impl<T> Arbitrary for SumTree<T>
where
    T: Debug + Arbitrary + Item + 'static,
    T::Summary: Debug + Summary<Context<'static> = ()>,
{
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        any::<Vec<T>>()
            .prop_map(|vec| SumTree::from_iter(vec, ()))
            .boxed()
    }
}

/// A strategy for producing a [`SumTree`] with a given size.
///
/// Equivalent to [`proptest::collection::vec`].
pub fn sum_tree<S, T>(values: S, size: impl Into<SizeRange>) -> impl Strategy<Value = SumTree<T>>
where
    T: Debug + Arbitrary + Item + 'static,
    T::Summary: Debug + Summary<Context<'static> = ()>,
    S: Strategy<Value = T>,
{
    proptest::collection::vec(values, size).prop_map(|vec| SumTree::from_iter(vec, ()))
}
