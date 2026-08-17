#[derive(num_enum::FromPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
enum Numbers {
    Zero,
    #[num_enum(default)]
    One,
    #[num_enum(default)]
    Two,
}

fn main() {

}
