use {
    crate::{Arbitrary, Result, Unstructured},
    std::collections::btree_map::BTreeMap,
};

impl<'a, K, V> Arbitrary<'a> for BTreeMap<K, V>
where
    K: Arbitrary<'a> + Ord,
    V: Arbitrary<'a>,
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
