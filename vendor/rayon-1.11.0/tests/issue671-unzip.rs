#![type_length_limit = "10000"]

use rayon::prelude::*;

#[test]
fn type_length_limit() {
    let input = vec![1, 2, 3, 4, 5];
    let (indexes, (squares, cubes)): (Vec<_>, (Vec<_>, Vec<_>)) = input
        .par_iter()
        .map(|x| (x * x, x * x * x))
        .enumerate()
        .unzip();

    drop(indexes);
    drop(squares);
    drop(cubes);
}
