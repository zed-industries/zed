//! Some wrapper types.
//!
//! # Features
//!
//! This module is only exported with the "fmt" feature.

#[cfg(feature = "fmt")]
pub(crate) mod ascii_str;

pub(crate) mod pwrapper;

#[cfg(feature = "fmt")]
pub(crate) mod sliced;

#[cfg(feature = "fmt")]
pub use self::ascii_str::NotAsciiError;

#[doc(no_inline)]
#[cfg(feature = "fmt")]
pub use crate::{AsciiStr, Sliced};

#[doc(no_inline)]
pub use crate::PWrapper;
