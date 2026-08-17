use {
    crate::{Arbitrary, Result, Unstructured},
    core::marker::{PhantomData, PhantomPinned},
};

impl<'a, A> Arbitrary<'a> for PhantomData<A>
where
    A: ?Sized,
{
    fn arbitrary(_: &mut Unstructured<'a>) -> Result<Self> {
        Ok(PhantomData)
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

impl<'a> Arbitrary<'a> for PhantomPinned {
    fn arbitrary(_: &mut Unstructured<'a>) -> Result<Self> {
        Ok(PhantomPinned)
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}
