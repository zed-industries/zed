//! This crate provide parsing fontconfig file but not yet complete all features
//!
//! see <https://www.freedesktop.org/software/fontconfig/fontconfig-user.html> for more detail infomation of fontconfig file
//!
//! # Example
//!
//! ```no_run
//! use fontconfig_parser::FontConfig;
//!
//! let mut config = FontConfig::default();
//!
//! config.merge_config("/etc/fonts/fonts.conf").unwrap();
//! ```

#[macro_use]
mod util;

mod error;
mod parser;
mod types;

pub type Result<T> = core::result::Result<T, Error>;

pub use crate::error::Error;
pub use crate::types::*;

/// Parse as raw config parts use this when you want custom handling config file
///
/// Otherwise, you may want [`FontConfig::merge_config`]
pub fn parse_config_parts(s: &str) -> Result<Vec<ConfigPart>> {
    crate::parser::parse_config(&roxmltree::Document::parse_with_options(
        s,
        roxmltree::ParsingOptions {
            allow_dtd: true,
            ..Default::default()
        },
    )?)?
    .collect()
}

#[cfg(test)]
mod tests {}
