//! Helpful misc. utilities such as a function to debug print a tree
mod math;
mod resolve;
pub(crate) mod sys;

pub use math::MaybeMath;
pub use resolve::{MaybeResolve, ResolveOrZero};

#[doc(hidden)]
#[macro_use]
pub(crate) mod debug;

#[cfg(feature = "std")]
mod print;
#[cfg(feature = "std")]
pub use print::print_tree;
#[cfg(feature = "std")]
pub use print::write_tree;

#[cfg(feature = "parse")]
pub(crate) mod parse;
#[cfg(feature = "parse")]
pub use parse::{ParseError, ParseResult};

/// Deserialize a type `S` by deserializing a string, then using the `FromStr`
/// impl of `S` to create the result. The generic type `S` is not required to
/// implement `Deserialize`.
#[cfg(feature = "serde")]
pub(crate) fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: for<'a> From<&'a str>,
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    Ok(S::from(&s))
}
