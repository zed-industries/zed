#![allow(unused_macros)]

extern crate nalgebra as na;
extern crate rand_package as rand;

#[macro_use]
extern crate criterion;

use na::{DMatrix, SMatrix, Scalar};
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use rand_isaac::IsaacRng;

pub mod core;
pub mod geometry;
pub mod linalg;

#[allow(dead_code)]
fn reproducible_smatrix<T, const R: usize, const C: usize>() -> SMatrix<T, R, C>
where
    T: Scalar,
    StandardUniform: Distribution<T>,
{
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);
    SMatrix::from_fn(|_, _| rng.random())
}

#[allow(dead_code)]
fn reproducible_dmatrix(nrows: usize, ncols: usize) -> DMatrix<f64> {
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);
    DMatrix::<f64>::from_fn(nrows, ncols, |_, _| rng.random())
}

criterion_main!(
    core::matrix,
    core::vector,
    geometry::quaternion,
    linalg::bidiagonal,
    linalg::cholesky,
    linalg::full_piv_lu,
    linalg::hessenberg,
    linalg::lu,
    linalg::qr,
    linalg::schur,
    linalg::solve,
    linalg::svd,
    linalg::symmetric_eigen,
);
