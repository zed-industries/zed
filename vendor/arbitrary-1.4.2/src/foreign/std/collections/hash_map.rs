use {
    crate::{Arbitrary, Result, Unstructured},
    std::{
        collections::hash_map::HashMap,
        hash::{BuildHasher, Hash},
    },
};

impl<'a, K, V, S> Arbitrary<'a> for HashMap<K, V, S>
where
    K: Arbitrary<'a> + Eq + Hash,
    V: Arbitrary<'a>,
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
