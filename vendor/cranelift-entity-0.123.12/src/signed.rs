/// Helper trait used to add `signed()` methods to primitive unsigned integer
/// types.
///
/// The purpose of this trait is to signal the intent that the sign bit of an
/// unsigned integer is intended to be discarded and the value is instead
/// understood to be a "bag of bits" where the conversion to a signed number
/// is intended to be lossless bit-wise. This can be used for example when
/// converting an unsigned integer into a signed integer for constrained reasons
/// outside the scope of the code in question.
pub trait Signed {
    /// The signed integer for this type which has the same width.
    type Signed;

    /// View this unsigned integer as a signed integer of the same width.
    ///
    /// All bits are preserved.
    fn signed(self) -> Self::Signed;
}

macro_rules! impls {
    ($($unsigned:ident => $signed:ident)*) => {$(
        impl Signed for $unsigned {
            type Signed = $signed;

            #[inline]
            fn signed(self) -> $signed {
                self as $signed
            }
        }
    )*}
}

impls! {
    u8 => i8
    u16 => i16
    u32 => i32
    u64 => i64
    u128 => i128
    usize => isize
}
