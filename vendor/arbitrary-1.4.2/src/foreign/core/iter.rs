use {
    crate::{Arbitrary, Result, Unstructured},
    core::iter::{empty, Empty},
};

impl<'a, A> Arbitrary<'a> for Empty<A>
where
    A: Arbitrary<'a>,
{
    fn arbitrary(_: &mut Unstructured<'a>) -> Result<Self> {
        Ok(empty())
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}
