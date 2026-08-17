#[macro_use]
extern crate arrayref;

const SIZE: usize = 10;
const SIZE_A: usize = 2;
const SIZE_B: usize = 3;
const SIZE_C: usize = 5;

fn main() {
    let x: [u8; SIZE] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let (a, b, c) = array_refs!(&x, SIZE_A, SIZE_B, SIZE_C);
    assert_eq!(SIZE_A, a.len());
    assert_eq!(SIZE_B, b.len());
    assert_eq!(SIZE_C, c.len());
    assert_eq!(*a, [0, 1]);
    assert_eq!(*b, [2, 3, 4]);
    assert_eq!(*c, [5, 6, 7, 8, 9]);
}
