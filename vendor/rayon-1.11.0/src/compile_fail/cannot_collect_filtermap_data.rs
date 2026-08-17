/*! ```compile_fail,E0599

use rayon::prelude::*;

// zip requires data of exact size, but filter yields only bounded
// size, so check that we cannot apply it.

let a: Vec<usize> = (0..1024).collect();
let mut v = vec![];
a.par_iter()
    .filter_map(|&x| Some(x as f32))
    .collect_into_vec(&mut v); //~ ERROR no method

``` */
