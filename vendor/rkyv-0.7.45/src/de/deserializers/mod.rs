//! Deserializers that can be used standalone and provide basic capabilities.

#[cfg(feature = "alloc")]
mod alloc;

#[doc(inline)]
#[cfg(feature = "alloc")]
pub use self::alloc::*;
