use {
    crate::{Arbitrary, Result, Unstructured},
    std::{
        collections::hash_set::HashSet,
        hash::{BuildHasher, Hash},
    },
};

impl<'a, A, S> Arbitrary<'a> for HashSet<A, S>
where
    A: Arbitrary<'a> + Eq + Hash,
    S: BuildHasher + Default,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.arbitrary_iter()?.collect()
    }

    fn arbitrary_take_rest(u: Unstructured<'a>) -> Result<Self> {
        u.arbitrary_take_rest_iter()?.collect()
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (0, None)
    }
}
