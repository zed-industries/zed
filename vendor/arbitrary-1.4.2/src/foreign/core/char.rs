use crate::{Arbitrary, Result, Unstructured};

/// Returns '\0', not an error, if this `Unstructured` [is empty][Unstructured::is_empty].
impl<'a> Arbitrary<'a> for char {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        // The highest unicode code point is 0x11_FFFF
        const CHAR_END: u32 = 0x11_0000;
        // The size of the surrogate blocks
        const SURROGATES_START: u32 = 0xD800;
        let mut c = <u32 as Arbitrary<'a>>::arbitrary(u)? % CHAR_END;
        if let Some(c) = char::from_u32(c) {
            Ok(c)
        } else {
            // We found a surrogate, wrap and try again
            c -= SURROGATES_START;
            Ok(char::from_u32(c)
                .expect("Generated character should be valid! This is a bug in arbitrary-rs"))
        }
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        <u32 as Arbitrary<'a>>::size_hint(depth)
    }
}
