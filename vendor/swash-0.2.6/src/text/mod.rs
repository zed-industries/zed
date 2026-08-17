/*!
Character properties and textual analysis.
*/

// Avoid errors for generated Unicode data.
#![allow(clippy::upper_case_acronyms)]

mod analyze;
mod compose;
mod lang;
mod lang_data;
mod unicode;
mod unicode_data;

pub mod cluster;

pub use analyze::{analyze, Analyze, WordBreakStrength};
pub use lang::{Cjk, Language};
pub use unicode::*;
