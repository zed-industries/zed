//! The async I/O prelude.
//!
//! The purpose of this module is to alleviate imports of many common I/O traits
//! by adding a glob import to the top of I/O heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use async_std::io::prelude::*;
//! ```

#[doc(no_inline)]
pub use crate::io::BufRead;
#[doc(no_inline)]
pub use crate::io::Read;
#[doc(no_inline)]
pub use crate::io::Seek;
#[doc(no_inline)]
pub use crate::io::Write;

#[doc(inline)]
pub use crate::io::buf_read::BufReadExt;
#[doc(inline)]
pub use crate::io::read::ReadExt;
#[doc(inline)]
pub use crate::io::seek::SeekExt;
#[doc(inline)]
pub use crate::io::write::WriteExt;
