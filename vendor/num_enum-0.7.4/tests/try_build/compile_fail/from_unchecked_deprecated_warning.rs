#![deny(deprecated)]

use num_enum::UnsafeFromPrimitive;

#[derive(Debug, Eq, PartialEq, UnsafeFromPrimitive)]
#[repr(u8)]
enum Enum {
    Zero,
    One,
}

fn main() {
    unsafe {
        assert_eq!(Enum::from_unchecked(0_u8), Enum::Zero);
        assert_eq!(Enum::from_unchecked(1_u8), Enum::One);
    }
}
