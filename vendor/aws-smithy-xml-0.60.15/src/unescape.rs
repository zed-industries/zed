/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::decode::XmlDecodeError;
use std::borrow::Cow;

/// Unescape XML encoded characters
///
/// This function will unescape the 4 literal escapes:
/// - `&lt;`, `&gt;`, `&amp;`, `&quot;`, and `&apos;`
/// - Decimal escapes: `&#123;`
/// - Hex escapes: `&#xD;`
///
/// If no escape sequences are present, Cow<&'str> will be returned, avoiding the need
/// to copy the String.
pub(crate) fn unescape(s: &str) -> Result<Cow<'_, str>, XmlDecodeError> {
    // no &, no need to escape anything
    if !s.contains('&') {
        return Ok(Cow::Borrowed(s));
    }
    // this will be strictly larger than required avoiding the need for another allocation
    let mut res = String::with_capacity(s.len());
    // could consider memchr as performance optimization
    let mut sections = s.split('&');
    // push content before the first &
    if let Some(prefix) = sections.next() {
        res.push_str(prefix);
    }
    for section in sections {
        // entities look like &<somedata>;
        match section.find(';') {
            Some(idx) => {
                let entity = &section[..idx];
                match entity {
                    "lt" => res.push('<'),
                    "gt" => res.push('>'),
                    "amp" => res.push('&'),
                    "quot" => res.push('"'),
                    "apos" => res.push('\''),
                    entity => {
                        // e.g. &#xD;
                        let (entity, radix) = if let Some(entity) = entity.strip_prefix("#x") {
                            (entity, 16)
                        } else if let Some(entity) = entity.strip_prefix('#') {
                            // e.g. &#123;
                            (entity, 10)
                        } else {
                            return Err(XmlDecodeError::invalid_escape(entity));
                        };
                        let char_code = u32::from_str_radix(entity, radix).map_err(|_| {
                            XmlDecodeError::invalid_escape(format!(
                                "expected numeric escape in base {}; got: {}",
                                radix, &entity
                            ))
                        })?;
                        let chr = std::char::from_u32(char_code).ok_or_else(|| {
                            XmlDecodeError::invalid_escape(format!(
                                "invalid char code: {char_code}"
                            ))
                        })?;
                        res.push(chr);
                    }
                }
                // push everything from the `;` to the next `&`
                res.push_str(&section[idx + 1..])
            }
            None => return Err(XmlDecodeError::invalid_escape("unterminated pattern")),
        }
    }
    Ok(Cow::Owned(res))
}

#[cfg(test)]
mod test {
    use crate::unescape::unescape;
    use std::borrow::Cow;

    #[test]
    fn basic_unescape() {
        assert_eq!(
            unescape("&lt; &gt; &apos; &quot; &amp;").unwrap(),
            "< > ' \" &"
        );
        assert_eq!(
            unescape("Since a &gt; b, b is less than a").unwrap(),
            "Since a > b, b is less than a"
        );
    }

    #[test]
    fn no_need_to_escape() {
        assert_eq!(unescape("hello 🍕!").unwrap(), Cow::Borrowed("hello 🍕!"));
    }

    #[test]
    fn complex_unescape() {
        // Test cases adapted from Apache Commons StringEscapeUtilsTest.java
        assert_eq!(
            unescape("a&lt;b&gt;c&quot;d&apos;e&amp;f;;").unwrap(),
            "a<b>c\"d'e&f;;"
        );
        assert_eq!(unescape("&amp;lt;").unwrap(), "&lt;")
    }

    #[test]
    fn newline_encoding() {
        assert_eq!(unescape("&#10;").unwrap(), "\n");
        assert_eq!(unescape("&#xD;").unwrap(), "\r");
    }

    #[test]
    fn xml_eol_encoding() {
        assert_eq!(unescape("&#xA; &#xA;").unwrap(), "\n \n");
        assert_eq!(
            unescape("a&#xD;&#xA; b&#xA; c&#xD;").unwrap(),
            "a\r\n b\n c\r"
        );
        assert_eq!(
            unescape("a&#xD;&#x85; b&#x85;").unwrap(),
            "a\r\u{0085} b\u{0085}"
        );
        assert_eq!(
            unescape("a&#xD;&#x2028; b&#x85; c&#x2028;").unwrap(),
            "a\r\u{2028} b\u{0085} c\u{2028}"
        );
    }

    #[test]
    fn invalid_escapes() {
        unescape("&lte;").expect_err("lte does not make a ≤");
        unescape("&lt").expect_err("unterminated escape sequence");
        unescape("&#Q1234;").expect_err("Q does not began a numeric sequence");
        unescape("&#3.14;").expect_err("decimal escape");
        unescape("&#xZZ").expect_err("Z is not hex");
        unescape("here is a & but without an escape sequence...").expect_err("naked &");
    }

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn no_panics(s: String) {
            let unescaped = unescape(&s);
            // if the string needed to be escaped, we
            if s.contains('&') {
                assert!(
                    matches!(unescaped, Ok(Cow::Owned(_)) | Err(_))
                );
            }
        }
    }
}
