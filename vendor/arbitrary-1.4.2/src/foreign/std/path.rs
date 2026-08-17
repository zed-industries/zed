use {
    crate::{Arbitrary, Result, Unstructured},
    std::{ffi::OsString, path::PathBuf},
};

impl<'a> Arbitrary<'a> for PathBuf {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        <OsString as Arbitrary>::arbitrary(u).map(From::from)
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        <OsString as Arbitrary>::size_hint(depth)
    }
}
