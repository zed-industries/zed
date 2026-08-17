#[derive(num_enum::FromPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
enum Numbers {
    Zero,
    #[default]
    One,
    #[num_enum(default)]
    Two,
}

fn main() {

}
