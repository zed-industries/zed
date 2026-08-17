/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use percent_encoding::{AsciiSet, CONTROLS};

/// base set of characters that must be URL encoded
pub(crate) const BASE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'/')
    // RFC-3986 §3.3 allows sub-delims (defined in section2.2) to be in the path component.
    // This includes both colon ':' and comma ',' characters.
    // Smithy protocol tests & AWS services percent encode these expected values. Signing
    // will fail if these values are not percent encoded
    .add(b':')
    .add(b',')
    .add(b'?')
    .add(b'#')
    .add(b'[')
    .add(b']')
    .add(b'{')
    .add(b'}')
    .add(b'|')
    .add(b'@')
    .add(b'!')
    .add(b'$')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b';')
    .add(b'=')
    .add(b'%')
    .add(b'<')
    .add(b'>')
    .add(b'"')
    .add(b'^')
    .add(b'`')
    .add(b'\\');

#[cfg(test)]
mod test {
    use crate::urlencode::BASE_SET;
    use percent_encoding::utf8_percent_encode;

    #[test]
    fn set_includes_mandatory_characters() {
        let chars = ":/?#[]@!$&'()*+,;=%";
        let escaped = utf8_percent_encode(chars, BASE_SET).to_string();
        assert_eq!(
            escaped,
            "%3A%2F%3F%23%5B%5D%40%21%24%26%27%28%29%2A%2B%2C%3B%3D%25"
        );

        // sanity check that every character is escaped
        assert_eq!(escaped.len(), chars.len() * 3);
    }
}
