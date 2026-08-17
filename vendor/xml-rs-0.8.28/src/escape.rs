//! Contains functions for performing XML special characters escaping.

use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;

pub(crate) trait Escapes {
    fn escape(c: u8) -> Option<&'static str>;

    fn byte_needs_escaping(c: u8) -> bool {
        Self::escape(c).is_some()
    }

    fn str_needs_escaping(s: &str) -> bool {
        s.bytes().any(|c| Self::escape(c).is_some())
    }
}

pub(crate) struct Escaped<'a, E: Escapes> {
    _escape_phantom: PhantomData<E>,
    to_escape: &'a str,
}

impl<'a, E: Escapes> Escaped<'a, E> {
    pub const fn new(s: &'a str) -> Self {
        Escaped {
            _escape_phantom: PhantomData,
            to_escape: s,
        }
    }
}

impl<E: Escapes> Display for Escaped<'_, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut total_remaining = self.to_escape;

        // find the next occurence
        while let Some(n) = total_remaining.bytes().position(E::byte_needs_escaping) {
            let (start, remaining) = total_remaining.split_at(n);

            f.write_str(start)?;

            // unwrap is safe because we checked is_some for position n earlier
            let next_byte = remaining.bytes().next().unwrap();
            let replacement = E::escape(next_byte).unwrap_or("unexpected token");
            f.write_str(replacement)?;

            total_remaining = &remaining[1..];
        }

        f.write_str(total_remaining)
    }
}

fn escape_str<E: Escapes>(s: &str) -> Cow<'_, str> {
    if E::str_needs_escaping(s) {
        Cow::Owned(Escaped::<E>::new(s).to_string())
    } else {
        Cow::Borrowed(s)
    }
}

macro_rules! escapes {
    {
        $name: ident,
        $($k: expr => $v: expr),* $(,)?
    } => {
        pub(crate) struct $name;

        impl Escapes for $name {
            fn escape(c: u8) -> Option<&'static str> {
                match c {
                    $( $k => Some($v),)*
                    _ => None
                }
            }
        }
    };
}

escapes!(
    AttributeEscapes,
    b'<'  => "&lt;",
    b'>'  => "&gt;",
    b'"'  => "&quot;",
    b'\'' => "&apos;",
    b'&'  => "&amp;",
    b'\n' => "&#xA;",
    b'\r' => "&#xD;",
);

escapes!(
    PcDataEscapes,
    b'<' => "&lt;",
    b'>' => "&gt;",
    b'&' => "&amp;",
);

/// Performs escaping of common XML characters inside an attribute value.
///
/// This function replaces several important markup characters with their
/// entity equivalents:
///
/// * `<` → `&lt;`
/// * `>` → `&gt;`
/// * `"` → `&quot;`
/// * `'` → `&apos;`
/// * `&` → `&amp;`
///
/// The following characters are escaped so that attributes are printed on
/// a single line:
/// * `\n` → `&#xA;`
/// * `\r` → `&#xD;`
///
/// The resulting string is safe to use inside XML attribute values or in PCDATA sections.
///
/// Does not perform allocations if the given string does not contain escapable characters.
#[inline]
#[must_use]
pub fn escape_str_attribute(s: &str) -> Cow<'_, str> {
    escape_str::<AttributeEscapes>(s)
}

/// Performs escaping of common XML characters inside PCDATA.
///
/// This function replaces several important markup characters with their
/// entity equivalents:
///
/// * `<` → `&lt;`
/// * `&` → `&amp;`
///
/// The resulting string is safe to use inside PCDATA sections but NOT inside attribute values.
///
/// Does not perform allocations if the given string does not contain escapable characters.
#[inline]
#[must_use]
pub fn escape_str_pcdata(s: &str) -> Cow<'_, str> {
    escape_str::<PcDataEscapes>(s)
}

#[cfg(test)]
mod tests {
    use super::{escape_str_attribute, escape_str_pcdata};

    #[test]
    fn test_escape_str_attribute() {
        assert_eq!(escape_str_attribute("<>'\"&\n\r"), "&lt;&gt;&apos;&quot;&amp;&#xA;&#xD;");
        assert_eq!(escape_str_attribute("no_escapes"), "no_escapes");
    }

    #[test]
    fn test_escape_str_pcdata() {
        assert_eq!(escape_str_pcdata("<>&"), "&lt;&gt;&amp;");
        assert_eq!(escape_str_pcdata("no_escapes"), "no_escapes");
    }

    #[test]
    fn test_escape_multibyte_code_points() {
        assert_eq!(escape_str_attribute("☃<"), "☃&lt;");
        assert_eq!(escape_str_pcdata("☃<"), "☃&lt;");
    }
}
