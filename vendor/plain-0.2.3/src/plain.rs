use Error;

/// A trait for plain data types that can be safely read from a byte slice.
///
/// A type can be [`Plain`](trait.Plain.html) if it is `#repr(C)` and only contains
/// data with no possible invalid values. Types that _can't_ be `Plain`
/// include, but are not limited to, `bool`, `char`, `enum`s, tuples,
/// pointers and references.
///
/// At this moment, `Drop` types are also not legal, because
/// compiler adds a special "drop flag" into the type. This is slated
/// to change in the future.
///
/// On the other hand, arrays of a `Plain` type, and
/// structures where all members are plain (and not `Drop`), are okay.
///
/// Structures that are not `#repr(C)`, while not necessarily illegal
/// in principle, are largely useless because they don't have a stable
/// layout. For example, the compiler is allowed to reorder fields
/// arbitrarily.
///
/// All methods of this trait are implemented automatically as wrappers
/// for crate-level funtions.
///
pub unsafe trait Plain {
    #[inline(always)]
    fn from_bytes(bytes: &[u8]) -> Result<&Self, Error>
    where
        Self: Sized,
    {
        ::from_bytes(bytes)
    }

    #[inline(always)]
    fn slice_from_bytes(bytes: &[u8]) -> Result<&[Self], Error>
    where
        Self: Sized,
    {
        ::slice_from_bytes(bytes)
    }

    #[inline(always)]
    fn slice_from_bytes_len(bytes: &[u8], len: usize) -> Result<&[Self], Error>
    where
        Self: Sized,
    {
        ::slice_from_bytes_len(bytes, len)
    }

    #[inline(always)]
    fn from_mut_bytes(bytes: &mut [u8]) -> Result<&mut Self, Error>
    where
        Self: Sized,
    {
        ::from_mut_bytes(bytes)
    }

    #[inline(always)]
    fn slice_from_mut_bytes(bytes: &mut [u8]) -> Result<&mut [Self], Error>
    where
        Self: Sized,
    {
        ::slice_from_mut_bytes(bytes)
    }

    #[inline(always)]
    fn slice_from_mut_bytes_len(bytes: &mut [u8], len: usize) -> Result<&mut [Self], Error>
    where
        Self: Sized,
    {
        ::slice_from_mut_bytes_len(bytes, len)
    }

    #[inline(always)]
    fn copy_from_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        ::copy_from_bytes(self, bytes)
    }
}

unsafe impl Plain for u8 {}
unsafe impl Plain for u16 {}
unsafe impl Plain for u32 {}
unsafe impl Plain for u64 {}
unsafe impl Plain for usize {}

unsafe impl Plain for i8 {}
unsafe impl Plain for i16 {}
unsafe impl Plain for i32 {}
unsafe impl Plain for i64 {}
unsafe impl Plain for isize {}

unsafe impl<S> Plain for [S]
where
    S: Plain,
{
}
