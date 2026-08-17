#[macro_use]
extern crate arrayref;

fn add_three(a: &[u8; 3]) -> u8 {
    a[0] + a[1] + a[2]
}

fn main() {
    let mut x = [0u8; 30];
    x[20] = 1;
    x[21] = 4;
    x[24] = 3;
    x[0] = add_three(array_mut_ref![x, 20, 3]);
    assert_eq!(x[0], 8);
}
