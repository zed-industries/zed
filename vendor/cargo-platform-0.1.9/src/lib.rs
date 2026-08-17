//! Platform definition used by Cargo.
//!
//! This defines a [`Platform`] type which is used in Cargo to specify a target platform.
//! There are two kinds, a named target like `x86_64-apple-darwin`, and a "cfg expression"
//! like `cfg(any(target_os = "macos", target_os = "ios"))`.
//!
//! See `examples/matches.rs` for an example of how to match against a `Platform`.
//!
//! > This crate is maintained by the Cargo team for use by the wider
//! > ecosystem. This crate follows semver compatibility for its APIs.
//!
//! [`Platform`]: enum.Platform.html

use std::fmt;
use std::str::FromStr;

mod cfg;
mod error;

pub use cfg::{Cfg, CfgExpr};
pub use error::{ParseError, ParseErrorKind};

/// Platform definition.
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Debug)]
pub enum Platform {
    /// A named platform, like `x86_64-apple-darwin`.
    Name(String),
    /// A cfg expression, like `cfg(windows)`.
    Cfg(CfgExpr),
}

impl Platform {
    /// Returns whether the Platform matches the given target and cfg.
    ///
    /// The named target and cfg values should be obtained from `rustc`.
    pub fn matches(&self, name: &str, cfg: &[Cfg]) -> bool {
        match *self {
            Platform::Name(ref p) => p == name,
            Platform::Cfg(ref p) => p.matches(cfg),
        }
    }

    fn validate_named_platform(name: &str) -> Result<(), ParseError> {
        if let Some(ch) = name
            .chars()
            .find(|&c| !(c.is_alphanumeric() || c == '_' || c == '-' || c == '.'))
        {
            if name.chars().any(|c| c == '(') {
                return Err(ParseError::new(
                    name,
                    ParseErrorKind::InvalidTarget(
                        "unexpected `(` character, cfg expressions must start with `cfg(`"
                            .to_string(),
                    ),
                ));
            }
            return Err(ParseError::new(
                name,
                ParseErrorKind::InvalidTarget(format!(
                    "unexpected character {} in target name",
                    ch
                )),
            ));
        }
        Ok(())
    }

    pub fn check_cfg_attributes(&self, warnings: &mut Vec<String>) {
        fn check_cfg_expr(expr: &CfgExpr, warnings: &mut Vec<String>) {
            match *expr {
                CfgExpr::Not(ref e) => check_cfg_expr(e, warnings),
                CfgExpr::All(ref e) | CfgExpr::Any(ref e) => {
                    for e in e {
                        check_cfg_expr(e, warnings);
                    }
                }
                CfgExpr::Value(ref e) => match e {
                    Cfg::Name(name) => match name.as_str() {
                        "test" | "debug_assertions" | "proc_macro" =>
                            warnings.push(format!(
                                "Found `{}` in `target.'cfg(...)'.dependencies`. \
                                 This value is not supported for selecting dependencies \
                                 and will not work as expected. \
                                 To learn more visit \
                                 https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies",
                                 name
                            )),
                        _ => (),
                    },
                    Cfg::KeyPair(name, _) => if name.as_str() == "feature" {
                        warnings.push(String::from(
                            "Found `feature = ...` in `target.'cfg(...)'.dependencies`. \
                             This key is not supported for selecting dependencies \
                             and will not work as expected. \
                             Use the [features] section instead: \
                             https://doc.rust-lang.org/cargo/reference/features.html"
                        ))
                    },
                }
            }
        }

        if let Platform::Cfg(cfg) = self {
            check_cfg_expr(cfg, warnings);
        }
    }
}

impl serde::Serialize for Platform {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for Platform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for Platform {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Platform, ParseError> {
        if let Some(s) = s.strip_prefix("cfg(").and_then(|s| s.strip_suffix(')')) {
            s.parse().map(Platform::Cfg)
        } else {
            Platform::validate_named_platform(s)?;
            Ok(Platform::Name(s.to_string()))
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Platform::Name(ref n) => n.fmt(f),
            Platform::Cfg(ref e) => write!(f, "cfg({})", e),
        }
    }
}
