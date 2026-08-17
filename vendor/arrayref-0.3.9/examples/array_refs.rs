#[macro_use]
extern crate arrayref;

fn main() {
    let x = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let (a, b, c) = array_refs!(&x, 2, 3, 5);
    assert_eq!(2, a.len());
    assert_eq!(3, b.len());
    assert_eq!(5, c.len());
    assert_eq!(*a, [0, 1]);
    assert_eq!(*b, [2, 3, 4]);
    assert_eq!(*c, [5, 6, 7, 8, 9]);
}
