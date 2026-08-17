use crate::{std::fmt, tags, Result, Stream, Value};

/**
An adapter that streams a slice of 8bit unsigned integers as binary.

For fixed-size arrays, see the [`BinaryArray`] type.
*/
#[repr(transparent)]
pub struct BinarySlice([u8]);

impl BinarySlice {
    /**
    Treat a slice of 8bit unsigned integers as binary.
    */
    #[inline(always)]
    pub const fn new<'a>(binary: &'a [u8]) -> &'a Self {
        // SAFETY: `Binary` and `[u8]` have the same ABI
        unsafe { &*(binary as *const _ as *const BinarySlice) }
    }

    /**
    Get a reference to the underlying slice.
    */
    #[inline(always)]
    pub const fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for BinarySlice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl AsRef<[u8]> for BinarySlice {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Value for BinarySlice {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.binary_begin(Some(self.0.len()))?;
        stream.binary_fragment(&self.0)?;
        stream.binary_end()
    }

    #[inline(always)]
    fn to_binary(&self) -> Option<&[u8]> {
        Some(&self.0)
    }
}

/**
An adapter that streams a slice of 8bit unsigned integers as binary with a fixed size.

This type is like [`BinarySlice`], but for fixed-size arrays.
*/
#[repr(transparent)]
pub struct BinaryArray<const N: usize>([u8; N]);

impl<const N: usize> BinaryArray<N> {
    /**
    Treat a slice of 8bit unsigned integers as binary.
    */
    #[inline(always)]
    pub const fn new<'a>(binary: &'a [u8; N]) -> &'a Self {
        // SAFETY: `Binary` and `[u8; N]` have the same ABI
        unsafe { &*(binary as *const _ as *const BinaryArray<N>) }
    }

    /**
    Get a reference to the underlying slice.
    */
    #[inline(always)]
    pub const fn as_slice(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> fmt::Debug for BinaryArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<const N: usize> AsRef<[u8; N]> for BinaryArray<N> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> Value for BinaryArray<N> {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.tagged_begin(Some(&tags::CONSTANT_SIZE), None, None)?;
        stream.binary_begin(Some(self.0.len()))?;
        stream.binary_fragment(&self.0)?;
        stream.binary_end()?;
        stream.tagged_end(Some(&tags::CONSTANT_SIZE), None, None)
    }

    #[inline(always)]
    fn to_binary(&self) -> Option<&[u8]> {
        Some(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_cast() {
        assert_eq!(Some(b"abc" as &[u8]), BinarySlice::new(b"abc").to_binary());
        assert_eq!(Some(b"abc" as &[u8]), BinaryArray::new(b"abc").to_binary());
    }
}
