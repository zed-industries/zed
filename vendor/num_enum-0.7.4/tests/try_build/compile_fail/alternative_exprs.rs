const THREE: u8 = 3;

#[derive(num_enum::TryFromPrimitive)]
#[repr(i8)]
enum Numbers {
    Zero = 0,
    #[num_enum(alternatives = [-1, 2, THREE])]
    One = 1,
}

fn main() {

}
