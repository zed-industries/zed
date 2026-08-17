use nalgebra::{matrix, stack};

fn main() {
    let m = matrix![1, 2; 3, 4];
    stack![0; m];
}
