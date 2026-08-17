use {
    crate::{Arbitrary, Result, Unstructured},
    core::str,
};

fn arbitrary_str<'a>(u: &mut Unstructured<'a>, size: usize) -> Result<&'a str> {
    match str::from_utf8(u.peek_bytes(size).unwrap()) {
        Ok(s) => {
            u.bytes(size).unwrap();
            Ok(s)
        }
        Err(e) => {
            let i = e.valid_up_to();
            let valid = u.bytes(i).unwrap();
            let s = unsafe {
                debug_assert!(str::from_utf8(valid).is_ok());
                str::from_utf8_unchecked(valid)
            };
            Ok(s)
        }
    }
}

impl<'a> Arbitrary<'a> for &'a str {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let size = u.arbitrary_len::<u8>()?;
        arbitrary_str(u, size)
    }

    fn arbitrary_take_rest(mut u: Unstructured<'a>) -> Result<Self> {
        let size = u.len();
        arbitrary_str(&mut u, size)
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (0, None)
    }
}
