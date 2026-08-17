use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Number {
    Zero,
    NonZero(u8),
}

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
enum Colour {
    Red { intensity: u8 },
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive)]
#[repr(u8)]
enum Meaningless {
    Beep(),
}

fn main() {}
