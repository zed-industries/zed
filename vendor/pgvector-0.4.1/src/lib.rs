#![doc = include_str!("../README.md")]

mod bit;
mod sparsevec;
mod vector;

pub use bit::Bit;
pub use sparsevec::SparseVector;
pub use vector::Vector;

#[cfg(feature = "halfvec")]
mod halfvec;

#[cfg(feature = "halfvec")]
pub use halfvec::HalfVector;

#[cfg(feature = "postgres")]
mod postgres_ext;

#[cfg(feature = "sqlx")]
mod sqlx_ext;

#[cfg(feature = "diesel")]
mod diesel_ext;

#[cfg(feature = "diesel")]
pub mod sql_types {
    pub use super::diesel_ext::bit::BitType as Bit;
    pub use super::diesel_ext::sparsevec::SparseVectorType as SparseVector;
    pub use super::diesel_ext::vector::VectorType as Vector;

    #[cfg(feature = "halfvec")]
    pub use super::diesel_ext::halfvec::HalfVectorType as HalfVector;
}

#[cfg(feature = "diesel")]
pub use diesel_ext::expression_methods::VectorExpressionMethods;
