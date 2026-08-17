//! Less used details of `CxxVector`.
//!
//! `CxxVector` itself is exposed at the crate root.

pub use crate::cxx_vector::{Iter, IterMut, VectorElement};
#[doc(inline)]
pub use crate::Vector;
#[doc(no_inline)]
pub use cxx::CxxVector;
