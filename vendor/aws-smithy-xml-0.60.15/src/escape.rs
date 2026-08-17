/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::borrow::Cow;
use std::fmt::Write;

const ESCAPES: &[char] = &[
    '&', '\'', '\"', '<', '>', '\u{00D}', '\u{00A}', '\u{0085}', '\u{2028}',
];

pub(crate) fn escape(s: &str) -> Cow<'_, str> {
    let mut remaining = s;
    if !s.contains(ESCAPES) {
        return Cow::Borrowed(s);
    }
    let mut out = String::new();
    while let Some(idx) = remaining.find(ESCAPES) {
        out.push_str(&remaining[..idx]);
        remaining = &remaining[idx..];
        let mut idxs = remaining.char_indices();
        let (_, chr) = idxs.next().expect("must not be none");
        match chr {
            '>' => out.push_str("&gt;"),
            '<' => out.push_str("&lt;"),
            '\'' => out.push_str("&apos;"),
            '"' => out.push_str("&quot;"),
            '&' => out.push_str("&amp;"),
            // push a hex escape sequence
            other => {
                write!(&mut out, "&#x{:X};", other as u32).expect("write to string cannot fail")
            }
        };
        match idxs.next() {
            None => remaining = "",
            Some((idx, _)) => remaining = &remaining[idx..],
        }
    }
    out.push_str(remaining);
    Cow::Owned(out)
}

#[cfg(test)]
mod test {
    #[test]
    fn escape_basic() {
        let inp = "<helo>&\"'";
        assert_eq!(escape(inp), "&lt;helo&gt;&amp;&quot;&apos;");
    }

    #[test]
    fn escape_eol_encoding_sep() {
        let test_cases = vec![
            ("CiAK", "&#xA; &#xA;"),                                      // '\n \n'
            ("YQ0KIGIKIGMN", "a&#xD;&#xA; b&#xA; c&#xD;"),                // 'a\r\n b\n c\r'
            ("YQ3ChSBiwoU", "a&#xD;&#x85; b&#x85;"),                      // 'a\r\u0085 b\u0085'
            ("YQ3igKggYsKFIGPigKg=", "a&#xD;&#x2028; b&#x85; c&#x2028;"), // 'a\r\u2028 b\u0085 c\u2028'
        ];
        for (base64_encoded, expected_xml_output) in test_cases {
            let bytes = base64::decode(base64_encoded).expect("valid base64");
            let input = String::from_utf8(bytes).expect("valid utf-8");
            assert_eq!(escape(&input), expected_xml_output);
        }
    }

    use crate::escape::escape;
    use proptest::proptest;
    proptest! {
        /// Test that arbitrary strings round trip after being escaped and unescaped
        #[test]
        fn round_trip(s: String) {
            let encoded = escape(&s);
            let decoded = crate::unescape::unescape(&encoded).expect("encoded should be valid decoded");
            assert_eq!(decoded, s);
        }
    }
}
