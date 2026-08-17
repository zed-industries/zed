#![allow(clippy::needless_doctest_main)]

pub mod confirm;
pub mod input;
pub mod multi_select;
pub mod select;
pub mod sort;

#[cfg(feature = "fuzzy-select")]
pub mod fuzzy_select;

#[cfg(feature = "password")]
pub mod password;
