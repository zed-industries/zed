#[derive(Debug, Eq, PartialEq, num_enum::FromPrimitive)]
#[repr(u8)]
enum Enum {
    #[num_enum(default)]
    Zero = 0,
    #[num_enum(catch_all)]
    NonZero(u8),
}

fn main() {}
