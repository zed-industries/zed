/// Helper trait used to add `unsigned()` methods to primitive signed integer
/// types.
///
/// The purpose of this trait is to signal the intent that the sign bit of a
/// signed integer is intended to be discarded and the value is instead
/// understood to be a "bag of bits" where the conversion to an unsigned number
/// is intended to be lossless. This can be used for example when converting a
/// signed integer into a larger width with zero-extension.
pub trait Unsigned {
    /// The unsigned integer for this type which has the same width.
    type Unsigned;

    /// View this signed integer as an unsigned integer of the same width.
    ///
    /// All bits are preserved.
    fn unsigned(self) -> Self::Unsigned;
}

impl Unsigned for i8 {
    type Unsigned = u8;

    #[inline]
    fn unsigned(self) -> u8 {
        self as u8
    }
}

impl Unsigned for i16 {
    type Unsigned = u16;

    #[inline]
    fn unsigned(self) -> u16 {
        self as u16
    }
}

impl Unsigned for i32 {
    type Unsigned = u32;

    #[inline]
    fn unsigned(self) -> u32 {
        self as u32
    }
}

impl Unsigned for i64 {
    type Unsigned = u64;

    #[inline]
    fn unsigned(self) -> u64 {
        self as u64
    }
}

impl Unsigned for i128 {
    type Unsigned = u128;

    #[inline]
    fn unsigned(self) -> u128 {
        self as u128
    }
}

impl Unsigned for isize {
    type Unsigned = usize;

    #[inline]
    fn unsigned(self) -> usize {
        self as usize
    }
}
