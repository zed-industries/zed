//! URI handling utilities for JSON Schema references.
use fluent_uri::{
    pct_enc::{encoder::Fragment, EStr, Encoder},
    Uri, UriRef,
};
use std::sync::LazyLock;

use crate::Error;
pub use fluent_uri::pct_enc::encoder::Path;

/// Resolves the URI reference against the given base URI and returns the target URI.
///
/// # Errors
///
/// Returns an error if base has not schema or there is a fragment.
pub fn resolve_against(base: &Uri<&str>, uri: &str) -> Result<Uri<String>, Error> {
    if uri.starts_with('#') && base.as_str().ends_with(uri) {
        return Ok(base.to_owned());
    }
    Ok(UriRef::parse(uri)
        .map_err(|error| Error::uri_reference_parsing_error(uri, error))?
        .resolve_against(base)
        .map_err(|error| Error::uri_resolving_error(uri, *base, error))?
        .normalize())
}

/// Parses a URI reference from a string into a [`crate::Uri`].
///
/// # Errors
///
/// Returns an error if the input string does not conform to URI-reference from RFC 3986.
pub fn from_str(uri: &str) -> Result<Uri<String>, Error> {
    let uriref = UriRef::parse(uri)
        .map_err(|error| Error::uri_reference_parsing_error(uri, error))?
        .normalize();
    if uriref.has_scheme() {
        Ok(Uri::try_from(uriref.as_str())
            .map_err(|error| Error::uri_parsing_error(uriref.as_str(), error))?
            .into())
    } else {
        Ok(uriref
            .resolve_against(&DEFAULT_ROOT_URI.borrow())
            .map_err(|error| Error::uri_resolving_error(uri, DEFAULT_ROOT_URI.borrow(), error))?)
    }
}

pub(crate) static DEFAULT_ROOT_URI: LazyLock<Uri<String>> =
    LazyLock::new(|| Uri::parse("json-schema:///".to_string()).expect("Invalid URI"));

pub type EncodedString = EStr<Fragment>;

// Adapted from `https://github.com/yescallop/fluent-uri-rs/blob/main/src/encoding/table.rs#L153`
pub fn encode_to(input: &str, buffer: &mut String) {
    const HEX_TABLE: [u8; 512] = {
        const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

        let mut i = 0;
        let mut table = [0; 512];
        while i < 256 {
            table[i * 2] = HEX_DIGITS[i >> 4];
            table[i * 2 + 1] = HEX_DIGITS[i & 0b1111];
            i += 1;
        }
        table
    };

    for ch in input.chars() {
        if Path::TABLE.allows(ch) {
            buffer.push(ch);
        } else {
            for x in ch.encode_utf8(&mut [0; 4]).bytes() {
                buffer.push('%');
                buffer.push(HEX_TABLE[x as usize * 2] as char);
                buffer.push(HEX_TABLE[x as usize * 2 + 1] as char);
            }
        }
    }
}
