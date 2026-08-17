#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # mac
//!
//! A collection of great and ubiqutitous macros.
//!

pub mod test;
pub mod mem;
pub mod format;
pub mod syntax_ext;
pub mod matches;
pub mod inspect;
pub mod cfg;

/// Unwraps an `Option` or returns from the function with the specified return
/// value.
///
/// Can be used on `Result`s by first calling `.ok()` or `.err()` on them.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate mac;
/// fn take_pair<I:Iterator>(iter: &mut I) -> Option<(<I as Iterator>::Item, <I as Iterator>::Item)> {
///    let first = unwrap_or_return!(iter.next(), None);
///    Some((first, unwrap_or_return!(iter.next(), None)))
/// }
/// # fn main() { }
/// ```
#[macro_export]
macro_rules! unwrap_or_return {
    ($e:expr, $r:expr) => (match $e { Some(e) => e, None => return $r, })
}

/// Do-while loop.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate mac;
/// # fn main() {
/// let mut i = 0;
/// let mut n = 0;
///
/// do_while!({
///     n += i;
///     i += 1;
/// } while i < 5);
///
/// assert_eq!(n, 10);
/// # }
/// ```
///
/// The loop always executes at least once.
///
/// ```
/// # #[macro_use] extern crate mac;
/// # fn main() {
/// let mut ran = false;
/// do_while!({ ran = true } while false);
/// assert!(ran);
/// # }
/// ```
#[macro_export]
macro_rules! do_while {
    ($body:block while $condition:expr) => {
        while { $body; $condition } { }
    }
}
