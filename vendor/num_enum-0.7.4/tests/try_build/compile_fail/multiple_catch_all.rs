#[derive(Debug, Eq, PartialEq, num_enum::FromPrimitive)]
#[repr(u8)]
enum Enum {
    #[num_enum(catch_all)]
    Zero(u8),
    #[num_enum(catch_all)]
    NonZero(u8),
}

fn main() {}
