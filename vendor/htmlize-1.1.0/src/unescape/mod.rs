//! # Functions to unescape HTML into raw text
//!
//! ```rust
//! use htmlize::{unescape, unescape_in, Context};
//! # use assert2::check as assert;
//!
//! assert!(unescape("1&times2&lt;3") == "1×2<3");
//! assert!(unescape_in("1&times2&lt;3", Context::Attribute) == "1&times2<3");
//! assert!(unescape_in("3 &times 5 &lt; 16", Context::Attribute) == "3 × 5 < 16");
//! ```
//!
//! See the normative reference for HTML5 entities:
//! <https://html.spec.whatwg.org/multipage/named-characters.html#named-character-references>
//!
//! Entities do not always require a trailing semicolon, though the exact rules
//! depend on whether the entity appears in an attribute value or somewhere else.
//! See [`unescape_in()`] for more information.
//!
//! Some entities are prefixes for multiple other entities. For example:
//!   &times &times; &timesb; &timesbar; &timesd;
#![expect(
    clippy::doc_paragraphs_missing_punctuation,
    reason = "false alarm on module docs"
)]

use std::borrow::Cow;

/// The context for an input string.
///
/// See [`unescape_in()`] for usage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Context {
    /// Anywhere outside of an HTML attribute, e.g. regular text. This is
    /// generally what you want.
    General,
    /// From an HTML attribute.
    Attribute,
}

/// Expand all valid entities.
///
/// ```rust
/// assert!(htmlize::unescape("1&times2&lt;3") == "1×2<3");
/// ```
///
/// This is appropriate to use on any text outside of an attribute. See
/// [`unescape_in()`] for more information.
///
/// To work with bytes (`[u8]`) instead of strings, see [`unescape_bytes_in()`].
pub fn unescape<'a, S: Into<Cow<'a, str>>>(escaped: S) -> Cow<'a, str> {
    #[cfg(feature = "unescape_fast")]
    return internal::unescape_in(
        (internal::Matchgen, internal::ContextGeneral),
        escaped,
    );

    #[cfg(all(feature = "unescape", not(feature = "unescape_fast")))]
    return internal::unescape_in(
        (internal::Phf, internal::ContextGeneral),
        escaped,
    );
}

/// Expand all valid entities in an attribute.
///
/// ```rust
/// use htmlize::unescape_attribute;
/// # use assert2::check as assert;
///
/// assert!(unescape_attribute("1&times2&lt;3") == "1&times2<3");
/// assert!(unescape_attribute("1 &times 2 &lt; 3") == "1 × 2 < 3");
/// ```
///
/// This is only appropriate for the value of an attribute. See
/// [`unescape_in()`] for more information.
///
/// To work with bytes (`[u8]`) instead of strings, see [`unescape_bytes_in()`].
pub fn unescape_attribute<'a, S: Into<Cow<'a, str>>>(
    escaped: S,
) -> Cow<'a, str> {
    #[cfg(feature = "unescape_fast")]
    return internal::unescape_in(
        (internal::Matchgen, internal::ContextAttribute),
        escaped,
    );

    #[cfg(all(feature = "unescape", not(feature = "unescape_fast")))]
    return internal::unescape_in(
        (internal::Phf, internal::ContextAttribute),
        escaped,
    );
}

/// Expand all valid entities in a given context.
///
/// `context` may be:
///
///   * `Context::General`: use the rules for text outside of an attribute.
///     This is usually what you want.
///   * `Context::Attribute`: use the rules for attribute values.
///
/// This uses the [algorithm described] in the WHATWG spec. In attributes,
/// [named entities] without trailing semicolons are not expanded when followed
/// by an alphanumeric character or `=`.
///
/// For example:
///
/// ```rust
/// use htmlize::{unescape_in, Context};
/// # use assert2::check as assert;
///
/// assert!(unescape_in("&times",   Context::General)   == "×");
/// assert!(unescape_in("&times",   Context::Attribute) == "×");
/// assert!(unescape_in("&times;X", Context::General)   == "×X");
/// assert!(unescape_in("&times;X", Context::Attribute) == "×X");
/// assert!(unescape_in("&timesX",  Context::General)   == "×X");
/// assert!(unescape_in("&timesX",  Context::Attribute) == "&timesX");
/// assert!(unescape_in("&times=",  Context::General)   == "×=");
/// assert!(unescape_in("&times=",  Context::Attribute) == "&times=");
/// assert!(unescape_in("&times#",  Context::General)   == "×#");
/// assert!(unescape_in("&times#",  Context::Attribute) == "×#");
/// ```
///
/// To work with bytes (`[u8]`) instead of strings, see [`unescape_bytes_in()`].
///
/// [algorithm described]: https://html.spec.whatwg.org/multipage/parsing.html#character-reference-state
/// [named entities]: https://html.spec.whatwg.org/multipage/parsing.html#named-character-reference-state
pub fn unescape_in<'a, S: Into<Cow<'a, str>>>(
    escaped: S,
    context: Context,
) -> Cow<'a, str> {
    match context {
        Context::Attribute => {
            #[cfg(feature = "unescape_fast")]
            return internal::unescape_in(
                (internal::Matchgen, internal::ContextAttribute),
                escaped,
            );

            #[cfg(all(feature = "unescape", not(feature = "unescape_fast")))]
            return internal::unescape_in(
                (internal::Phf, internal::ContextAttribute),
                escaped,
            );
        }
        Context::General => {
            #[cfg(feature = "unescape_fast")]
            return internal::unescape_in(
                (internal::Matchgen, internal::ContextGeneral),
                escaped,
            );

            #[cfg(all(feature = "unescape", not(feature = "unescape_fast")))]
            return internal::unescape_in(
                (internal::Phf, internal::ContextGeneral),
                escaped,
            );
        }
    }
}

/// Expand all valid entities in a given context.
///
/// `context` may be:
///
///   * `Context::General`: use the rules for text outside of an attribute.
///     This is usually what you want.
///   * `Context::Attribute`: use the rules for attribute values.
///
/// This uses the [algorithm described] in the WHATWG spec. In attributes,
/// [named entities] without trailing semicolons are treated differently. They
/// not expanded if they are followed by an alphanumeric character or or `=`.
///
/// For example:
///
/// ```rust
/// use htmlize::*;
/// # use assert2::check as assert;
///
/// assert!(unescape_bytes_in(&b"&times"[..],   Context::General)   == "×".as_bytes());
/// assert!(unescape_bytes_in(&b"&times"[..],   Context::Attribute) == "×".as_bytes());
/// assert!(unescape_bytes_in(&b"&times;X"[..], Context::General)   == "×X".as_bytes());
/// assert!(unescape_bytes_in(&b"&times;X"[..], Context::Attribute) == "×X".as_bytes());
/// assert!(unescape_bytes_in(&b"&timesX"[..],  Context::General)   == "×X".as_bytes());
/// assert!(unescape_bytes_in(&b"&timesX"[..],  Context::Attribute) == "&timesX".as_bytes());
/// assert!(unescape_bytes_in(&b"&times="[..],  Context::General)   == "×=".as_bytes());
/// assert!(unescape_bytes_in(&b"&times="[..],  Context::Attribute) == "&times=".as_bytes());
/// assert!(unescape_bytes_in(&b"&times#"[..],  Context::General)   == "×#".as_bytes());
/// assert!(unescape_bytes_in(&b"&times#"[..],  Context::Attribute) == "×#".as_bytes());
/// ```
///
/// To work with `String` instead of bytes, see [`unescape_in()`].
///
/// [algorithm described]: https://html.spec.whatwg.org/multipage/parsing.html#character-reference-state
/// [named entities]: https://html.spec.whatwg.org/multipage/parsing.html#named-character-reference-state
pub fn unescape_bytes_in<'a, S: Into<Cow<'a, [u8]>>>(
    escaped: S,
    context: Context,
) -> Cow<'a, [u8]> {
    match context {
        Context::Attribute => {
            #[cfg(feature = "unescape_fast")]
            return internal::unescape_bytes_in(
                (internal::Matchgen, internal::ContextAttribute),
                escaped,
            );

            #[cfg(all(feature = "unescape", not(feature = "unescape_fast")))]
            return internal::unescape_bytes_in(
                (internal::Phf, internal::ContextAttribute),
                escaped,
            );
        }
        Context::General => {
            #[cfg(feature = "unescape_fast")]
            return internal::unescape_bytes_in(
                (internal::Matchgen, internal::ContextGeneral),
                escaped,
            );

            #[cfg(all(feature = "unescape", not(feature = "unescape_fast")))]
            return internal::unescape_bytes_in(
                (internal::Phf, internal::ContextGeneral),
                escaped,
            );
        }
    }
}

// Need these to be public for benchmarks
#[cfg(all(feature = "bench", not(doc)))]
pub mod internal;

#[cfg(not(all(feature = "bench", not(doc))))]
mod internal;

pub use internal::REPLACEMENT_CHAR_BYTES;
