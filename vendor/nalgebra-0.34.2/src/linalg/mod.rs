//! [Reexported at the root of this crate.] Factorization of real matrices.

pub mod balancing;
mod bidiagonal;
mod cholesky;
mod convolution;
mod determinant;
// TODO: this should not be needed. However, the exp uses
// explicit float operations on `f32` and `f64`. We need to
// get rid of these to allow exp to be used on a no-std context.
mod col_piv_qr;
mod decomposition;
#[cfg(feature = "std")]
mod exp;
mod full_piv_lu;
pub mod givens;
mod hessenberg;
pub mod householder;
mod inverse;
mod lu;
mod permutation_sequence;
mod pow;
mod qr;
mod schur;
mod solve;
mod svd;
mod svd2;
mod svd3;
mod symmetric_eigen;
mod symmetric_tridiagonal;
mod udu;

// TODO: Not complete enough for publishing.
// This handles only cases where each eigenvalue has multiplicity one.
// mod eigen;

pub use self::bidiagonal::*;
pub use self::cholesky::*;
pub use self::col_piv_qr::*;
pub use self::full_piv_lu::*;
pub use self::hessenberg::*;
pub use self::lu::*;
pub use self::permutation_sequence::*;
pub use self::qr::*;
pub use self::schur::*;
pub use self::svd::*;
pub use self::symmetric_eigen::*;
pub use self::symmetric_tridiagonal::*;
pub use self::udu::*;
