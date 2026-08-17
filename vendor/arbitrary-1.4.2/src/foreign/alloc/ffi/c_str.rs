use {
    crate::{Arbitrary, Result, Unstructured},
    std::ffi::CString,
};

impl<'a> Arbitrary<'a> for CString {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        <Vec<u8> as Arbitrary>::arbitrary(u).map(|mut x| {
            x.retain(|&c| c != 0);
            // SAFETY: all zero bytes have been removed
            unsafe { Self::from_vec_unchecked(x) }
        })
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        <Vec<u8> as Arbitrary>::size_hint(depth)
    }
}
