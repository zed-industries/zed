#[cfg(not(all(
    feature = "debug",
    feature = "compare",
    feature = "rand",
    feature = "macros"
)))]
compile_error!(
    "Please enable the `debug`, `compare`, `rand` and `macros` features in order to compile and run the tests.
     Example: `cargo test --features debug,compare,rand,macros`"
);

#[cfg(all(feature = "debug", feature = "compare", feature = "rand"))]
#[macro_use]
extern crate approx;
extern crate nalgebra as na;
extern crate num_traits as num;
#[cfg(feature = "rand")]
extern crate rand_package as rand;

#[cfg(all(feature = "debug", feature = "compare", feature = "rand"))]
mod core;
#[cfg(all(feature = "debug", feature = "compare", feature = "rand"))]
mod geometry;
#[cfg(all(feature = "debug", feature = "compare", feature = "rand"))]
mod linalg;

#[cfg(all(feature = "debug", feature = "compare", feature = "rand"))]
#[cfg(feature = "proptest-support")]
mod proptest;

#[cfg(feature = "macros")]
mod macros;

//#[cfg(all(feature = "debug", feature = "compare", feature = "rand"))]
//#[cfg(feature = "sparse")]
//mod sparse;

mod utils {
    /// Checks if a slice is sorted in descending order.
    pub fn is_sorted_descending<T: PartialOrd>(slice: &[T]) -> bool {
        slice.windows(2).all(|elts| elts[0] >= elts[1])
    }
}
