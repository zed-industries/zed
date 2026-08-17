const TWOFIVEFIVE: u8 = 255;

#[derive(num_enum::TryFromPrimitive)]
#[repr(u8)]
enum Numbers {
    Zero = 0,
    #[num_enum(alternatives = [2..=TWOFIVEFIVE])]
    NonZero = 1,
}

fn main() {

}
