#[derive(num_enum::TryFromPrimitive)]
#[repr(u8)]
enum Numbers {
    Zero = 0,
    #[num_enum(alternatives = [3,1,4])]
    One = 1,
    Two = 2,
}

fn main() {}
