use nalgebra::{matrix, stack};

fn main() {
    // Use multi-letter names for checking that the reported span comes out correctly
    let a11 = matrix![1, 2;
                      3, 4];
    let a12 = matrix![5, 6;
                      7, 8];
    let a21 = matrix![9, 10];
    let a22 = matrix![11, 12;
                      13, 14];
    stack![a11, a12;
           a21, a22];
}