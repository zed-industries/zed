#[derive(num_enum::TryFromPrimitive)]
#[repr(u8)]
enum Numbers {
    Zero = 0,
    #[num_enum(garbage)]
    One = 1,
    Two = 2,
}

fn main() {

}
