use crate::error::ValidationError;
use ahash::AHashMap;
use data_encoding::{BASE32, BASE32HEX, BASE64, BASE64URL, HEXUPPER};
use std::sync::LazyLock;

pub(crate) type ContentEncodingCheckType = fn(&str) -> bool;
pub(crate) type ContentEncodingConverterType =
    fn(&str) -> Result<Option<String>, ValidationError<'static>>;

// RFC 4648 §4: Base 64 Encoding
// https://datatracker.ietf.org/doc/html/rfc4648#section-4
pub(crate) fn is_base64(instance_string: &str) -> bool {
    BASE64.decode(instance_string.as_bytes()).is_ok()
}

pub(crate) fn from_base64(
    instance_string: &str,
) -> Result<Option<String>, ValidationError<'static>> {
    match BASE64.decode(instance_string.as_bytes()) {
        Ok(value) => Ok(Some(String::from_utf8(value)?)),
        Err(_) => Ok(None),
    }
}

// RFC 4648 §5: Base 64 Encoding with URL and Filename Safe Alphabet
// https://datatracker.ietf.org/doc/html/rfc4648#section-5
pub(crate) fn is_base64url(instance_string: &str) -> bool {
    BASE64URL.decode(instance_string.as_bytes()).is_ok()
}

pub(crate) fn from_base64url(
    instance_string: &str,
) -> Result<Option<String>, ValidationError<'static>> {
    match BASE64URL.decode(instance_string.as_bytes()) {
        Ok(value) => Ok(Some(String::from_utf8(value)?)),
        Err(_) => Ok(None),
    }
}

// RFC 4648 §6: Base 32 Encoding
// https://datatracker.ietf.org/doc/html/rfc4648#section-6
pub(crate) fn is_base32(instance_string: &str) -> bool {
    BASE32.decode(instance_string.as_bytes()).is_ok()
}

pub(crate) fn from_base32(
    instance_string: &str,
) -> Result<Option<String>, ValidationError<'static>> {
    match BASE32.decode(instance_string.as_bytes()) {
        Ok(value) => Ok(Some(String::from_utf8(value)?)),
        Err(_) => Ok(None),
    }
}

// RFC 4648 §7: Base 32 Encoding with Extended Hex Alphabet
// https://datatracker.ietf.org/doc/html/rfc4648#section-7
pub(crate) fn is_base32hex(instance_string: &str) -> bool {
    BASE32HEX.decode(instance_string.as_bytes()).is_ok()
}

pub(crate) fn from_base32hex(
    instance_string: &str,
) -> Result<Option<String>, ValidationError<'static>> {
    match BASE32HEX.decode(instance_string.as_bytes()) {
        Ok(value) => Ok(Some(String::from_utf8(value)?)),
        Err(_) => Ok(None),
    }
}

// RFC 4648 §8: Base 16 Encoding
// https://datatracker.ietf.org/doc/html/rfc4648#section-8
pub(crate) fn is_base16(instance_string: &str) -> bool {
    HEXUPPER.decode(instance_string.as_bytes()).is_ok()
        || HEXUPPER
            .decode(instance_string.to_uppercase().as_bytes())
            .is_ok()
}

pub(crate) fn from_base16(
    instance_string: &str,
) -> Result<Option<String>, ValidationError<'static>> {
    // Base16 is case-insensitive per RFC 4648
    let result = HEXUPPER
        .decode(instance_string.as_bytes())
        .or_else(|_| HEXUPPER.decode(instance_string.to_uppercase().as_bytes()));
    match result {
        Ok(value) => Ok(Some(String::from_utf8(value)?)),
        Err(_) => Ok(None),
    }
}

// Supported in JSON Schema Draft 6, 7, 2019-09, and 2020-12
// Per JSON Schema Validation spec §8.3, encoding values are defined in:
// - RFC 4648 (base16, base32, base32hex, base64, base64url)
// - RFC 2045 §6.7-6.8 (quoted-printable, 7bit, 8bit, binary)
// We implement the RFC 4648 encodings as they are transformation encodings.
pub(crate) static DEFAULT_CONTENT_ENCODING_CHECKS_AND_CONVERTERS: LazyLock<
    AHashMap<&'static str, (ContentEncodingCheckType, ContentEncodingConverterType)>,
> = LazyLock::new(|| {
    let mut map: AHashMap<&'static str, (ContentEncodingCheckType, ContentEncodingConverterType)> =
        AHashMap::with_capacity(5);
    map.insert("base64", (is_base64, from_base64));
    map.insert("base64url", (is_base64url, from_base64url));
    map.insert("base32", (is_base32, from_base32));
    map.insert("base32hex", (is_base32hex, from_base32hex));
    map.insert("base16", (is_base16, from_base16));
    map
});

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // Test string: "foobar"
    const TEST_STRING: &str = "foobar";
    const TEST_BASE64: &str = "Zm9vYmFy";
    const TEST_BASE64URL: &str = "Zm9vYmFy"; // same as base64 for "foobar" (no +/)
    const TEST_BASE32: &str = "MZXW6YTBOI======";
    const TEST_BASE32HEX: &str = "CPNMUOJ1E8======";
    const TEST_BASE16_UPPER: &str = "666F6F626172";
    const TEST_BASE16_LOWER: &str = "666f6f626172";
    const TEST_BASE16_MIXED: &str = "666F6f626172";

    #[test_case(TEST_BASE64, true ; "valid base64")]
    #[test_case("not valid base64!!!", false ; "invalid base64 with special chars")]
    #[test_case("Zm9v====", false ; "invalid base64 padding")]
    fn test_is_base64(input: &str, expected: bool) {
        assert_eq!(is_base64(input), expected);
    }

    #[test_case(TEST_BASE64, Some(TEST_STRING) ; "decode valid base64")]
    #[test_case("invalid!", None ; "decode invalid base64")]
    fn test_from_base64(input: &str, expected: Option<&str>) {
        assert_eq!(
            from_base64(input).unwrap(),
            expected.map(std::string::ToString::to_string)
        );
    }

    #[test_case(TEST_BASE64URL, true ; "valid base64url")]
    #[test_case("PDw_Pz4-", true ; "base64url with url safe chars")]
    #[test_case("Zm9v+YmFy", false ; "base64 plus char invalid in base64url")]
    #[test_case("Zm9v/YmFy", false ; "base64 slash char invalid in base64url")]
    fn test_is_base64url(input: &str, expected: bool) {
        assert_eq!(is_base64url(input), expected);
    }

    #[test_case(TEST_BASE64URL, Some(TEST_STRING) ; "decode valid base64url")]
    #[test_case("invalid!", None ; "decode invalid base64url")]
    fn test_from_base64url(input: &str, expected: Option<&str>) {
        assert_eq!(
            from_base64url(input).unwrap(),
            expected.map(std::string::ToString::to_string)
        );
    }

    #[test_case(TEST_BASE32, true ; "valid base32")]
    #[test_case("not valid", false ; "invalid base32 text")]
    #[test_case("189", false ; "base32 invalid chars 1,8,9")]
    fn test_is_base32(input: &str, expected: bool) {
        assert_eq!(is_base32(input), expected);
    }

    #[test_case(TEST_BASE32, Some(TEST_STRING) ; "decode valid base32")]
    #[test_case("189!!!", None ; "decode invalid base32")]
    fn test_from_base32(input: &str, expected: Option<&str>) {
        assert_eq!(
            from_base32(input).unwrap(),
            expected.map(std::string::ToString::to_string)
        );
    }

    #[test_case(TEST_BASE32HEX, true ; "valid base32hex")]
    #[test_case("not valid", false ; "invalid base32hex text")]
    #[test_case("XYZ", false ; "base32hex invalid chars X,Y,Z")]
    fn test_is_base32hex(input: &str, expected: bool) {
        assert_eq!(is_base32hex(input), expected);
    }

    #[test_case(TEST_BASE32HEX, Some(TEST_STRING) ; "decode valid base32hex")]
    #[test_case("XYZ!!!", None ; "decode invalid base32hex")]
    fn test_from_base32hex(input: &str, expected: Option<&str>) {
        assert_eq!(
            from_base32hex(input).unwrap(),
            expected.map(std::string::ToString::to_string)
        );
    }

    #[test_case(TEST_BASE16_UPPER, true ; "valid base16 uppercase")]
    #[test_case(TEST_BASE16_LOWER, true ; "valid base16 lowercase")]
    #[test_case(TEST_BASE16_MIXED, true ; "valid base16 mixed case")]
    #[test_case("not valid", false ; "invalid base16 text")]
    #[test_case("GHIJ", false ; "base16 invalid chars G-J")]
    fn test_is_base16(input: &str, expected: bool) {
        assert_eq!(is_base16(input), expected);
    }

    #[test_case(TEST_BASE16_UPPER, Some(TEST_STRING) ; "decode base16 uppercase")]
    #[test_case(TEST_BASE16_LOWER, Some(TEST_STRING) ; "decode base16 lowercase")]
    #[test_case(TEST_BASE16_MIXED, Some(TEST_STRING) ; "decode base16 mixed")]
    #[test_case("GHIJ", None ; "decode invalid base16")]
    fn test_from_base16(input: &str, expected: Option<&str>) {
        assert_eq!(
            from_base16(input).unwrap(),
            expected.map(std::string::ToString::to_string)
        );
    }
}
