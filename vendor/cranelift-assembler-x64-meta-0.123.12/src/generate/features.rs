//! Generate feature-related Rust code.

use super::{Formatter, fmtln};
use crate::{dsl, generate::generate_derive};

impl dsl::Feature {
    /// `pub enum Feature { ... }`
    ///
    /// This function recreates the `Feature` struct itself in the generated
    /// code.
    pub fn generate_enum(f: &mut Formatter) {
        fmtln!(f, "#[doc(hidden)]");
        generate_derive(f);
        fmtln!(f, "#[derive(PartialEq)]"); // Add more helpful derives.
        f.add_block("pub enum Feature", |f| {
            for feature in dsl::ALL_FEATURES {
                fmtln!(f, "{feature},");
            }
        });
    }
}
