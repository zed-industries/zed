/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// Abstraction for avoiding a dependency from cssparser to an encoding library
pub trait EncodingSupport {
    /// One character encoding
    type Encoding;

    /// https://encoding.spec.whatwg.org/#concept-encoding-get
    fn from_label(ascii_label: &[u8]) -> Option<Self::Encoding>;

    /// Return the UTF-8 encoding
    fn utf8() -> Self::Encoding;

    /// Whether the given encoding is UTF-16BE or UTF-16LE
    fn is_utf16_be_or_le(encoding: &Self::Encoding) -> bool;
}

/// Determine the character encoding of a CSS stylesheet.
///
/// This is based on the presence of a BOM (Byte Order Mark), an `@charset` rule, and
/// encoding meta-information.
///
/// * `css_bytes`: A byte string.
/// * `protocol_encoding`: The encoding label, if any, defined by HTTP or equivalent protocol.
///   (e.g. via the `charset` parameter of the `Content-Type` header.)
/// * `environment_encoding`: An optional `Encoding` object for the [environment encoding]
///   (https://drafts.csswg.org/css-syntax/#environment-encoding), if any.
///
/// Returns the encoding to use.
pub fn stylesheet_encoding<E>(
    css: &[u8],
    protocol_encoding_label: Option<&[u8]>,
    environment_encoding: Option<E::Encoding>,
) -> E::Encoding
where
    E: EncodingSupport,
{
    // https://drafts.csswg.org/css-syntax/#the-input-byte-stream
    if let Some(label) = protocol_encoding_label {
        if let Some(protocol_encoding) = E::from_label(label) {
            return protocol_encoding;
        };
    };

    let prefix = b"@charset \"";
    if css.starts_with(prefix) {
        let rest = &css[prefix.len()..];
        if let Some(label_length) = rest.iter().position(|&b| b == b'"') {
            if rest[label_length..].starts_with(b"\";") {
                let label = &rest[..label_length];
                if let Some(charset_encoding) = E::from_label(label) {
                    if E::is_utf16_be_or_le(&charset_encoding) {
                        return E::utf8();
                    } else {
                        return charset_encoding;
                    }
                }
            }
        }
    }
    environment_encoding.unwrap_or_else(E::utf8)
}
