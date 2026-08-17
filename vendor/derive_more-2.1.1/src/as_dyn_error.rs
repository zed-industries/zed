//! Coercion into `dyn `[`Error`] used in macro expansions.
//!
//! # Credits
//!
//! The initial idea and implementation was taken from the [`thiserror`] crate and its
//! [`AsDynError`] trait implementation, and slightly modified for usage in derive [`derive_more`].
//!
//! The original code was dual licensed under [Apache License, Version 2.0][APACHE] and [MIT]
//! licenses.
//!
//! [`AsDynError`]: https://github.com/dtolnay/thiserror/blob/2.0.3/src/aserror.rs
//! [`derive_more`]: crate
//! [`thiserror`]: https://github.com/dtolnay/thiserror/blob/2.0.3
//! [APACHE]: https://github.com/dtolnay/thiserror/blob/2.0.3/LICENSE-APACHE
//! [MIT]: https://github.com/dtolnay/thiserror/blob/2.0.3/LICENSE-MIT

use core::{error::Error, panic::UnwindSafe};

#[doc(hidden)]
pub trait AsDynError<'a>: Sealed {
    fn __derive_more_as_dyn_error(&self) -> &(dyn Error + 'a);
}

impl<'a, T: Error + 'a> AsDynError<'a> for T {
    #[inline]
    fn __derive_more_as_dyn_error(&self) -> &(dyn Error + 'a) {
        self
    }
}

impl<'a> AsDynError<'a> for dyn Error + 'a {
    #[inline]
    fn __derive_more_as_dyn_error(&self) -> &(dyn Error + 'a) {
        self
    }
}

impl<'a> AsDynError<'a> for dyn Error + Send + 'a {
    #[inline]
    fn __derive_more_as_dyn_error(&self) -> &(dyn Error + 'a) {
        self
    }
}

impl<'a> AsDynError<'a> for dyn Error + Send + Sync + 'a {
    #[inline]
    fn __derive_more_as_dyn_error(&self) -> &(dyn Error + 'a) {
        self
    }
}

impl<'a> AsDynError<'a> for dyn Error + Send + Sync + UnwindSafe + 'a {
    #[inline]
    fn __derive_more_as_dyn_error(&self) -> &(dyn Error + 'a) {
        self
    }
}

#[doc(hidden)]
pub trait Sealed {}
impl<T: Error> Sealed for T {}
impl Sealed for dyn Error + '_ {}
impl Sealed for dyn Error + Send + '_ {}
impl Sealed for dyn Error + Send + Sync + '_ {}
impl Sealed for dyn Error + Send + Sync + UnwindSafe + '_ {}
