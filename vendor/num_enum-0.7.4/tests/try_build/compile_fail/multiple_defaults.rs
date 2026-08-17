#[derive(num_enum::FromPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
enum Numbers {
    Zero,
    #[default]
    One,
    #[default]
    Two,
}

fn main() {

}
