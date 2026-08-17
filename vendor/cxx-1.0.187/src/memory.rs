//! Less used details of `UniquePtr` and `SharedPtr`.
//!
//! The pointer types themselves are exposed at the crate root.

pub use crate::shared_ptr::SharedPtrTarget;
pub use crate::unique_ptr::UniquePtrTarget;
pub use crate::weak_ptr::WeakPtrTarget;
#[doc(no_inline)]
pub use cxx::{SharedPtr, UniquePtr};
