use std::borrow::Cow;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WwwAuthenticate<'a> {
    pub realm: Option<Cow<'a, str>>,
    pub scope: Option<Cow<'a, str>>,
    pub error: Option<BearerError>,
    pub error_description: Option<Cow<'a, str>>,
    pub error_uri: Option<Cow<'a, str>>,
    pub resource_metadata: Option<Cow<'a, str>>,
}

const BEARER_SCHEME: &str = "Bearer";

impl<'a> WwwAuthenticate<'a> {
    pub fn parse(input: &'a str) -> Option<Self> {
        // Header format (simplified):
        //   Bearer realm="example", error="invalid_token", error_description="...", error_uri="..."
        let input = input.trim_ascii_start();

        let (scheme, mut input) = input
            .trim_start()
            .split_once(|c: char| c.is_ascii_whitespace())
            .unwrap_or((input, ""));

        // We only parse Bearer challenges as defined by RFC 6750 section 3.
        if !scheme.eq_ignore_ascii_case(BEARER_SCHEME) {
            return None;
        }

        let mut challenge = Self::default();

        loop {
            input = input.trim_ascii_start();

            if input.is_empty() {
                break;
            }

            // Stop at a subsequent Bearer challenge in a combined header.
            if let Some(sub) = input.strip_prefix(BEARER_SCHEME)
                && sub
                    .chars()
                    .next()
                    .is_some_and(|character| character.is_ascii_whitespace())
            {
                break;
            }

            let (name, rest) = parse_token(input)?;
            let mut rest = rest.trim_ascii_start();

            rest = rest.strip_prefix('=')?.trim_ascii_start();

            let (value, rest) = parse_value(rest)?;
            input = rest;

            match name {
                "realm" => challenge.realm = Some(value),
                "scope" => challenge.scope = Some(value),
                "error" => challenge.error = Some(BearerError::parse(&value)),
                "error_description" => challenge.error_description = Some(value),
                "error_uri" => {
                    challenge.error_uri = Some(value);
                }
                "resource_metadata" => {
                    challenge.resource_metadata = Some(value);
                }
                _ => {
                    // Ignore extension auth-params.
                }
            }

            input = input.trim_start();
            if let Some(after_comma) = input.strip_prefix(',') {
                input = after_comma;
            } else {
                // If there's no comma, we either reached the end or encountered something invalid.
                if !input.is_empty() {
                    return None;
                }
            }
        }

        Some(challenge)
    }
}

/// Error codes defined by RFC 6750 Section 3.1 for Bearer token authentication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BearerError {
    /// The request is missing a required parameter, includes an unsupported parameter
    /// or parameter value, repeats the same parameter, uses more than one method for
    /// including an access token, or is otherwise malformed.
    InvalidRequest,
    /// The access token provided is expired, revoked, malformed, or invalid for other reasons.
    InvalidToken,
    /// The request requires higher privileges than provided by the access token.
    InsufficientScope,
    /// An unrecognized error code (extension or future spec addition).
    Other,
}

impl BearerError {
    fn parse(value: &str) -> Self {
        match value {
            "invalid_request" => BearerError::InvalidRequest,
            "invalid_token" => BearerError::InvalidToken,
            "insufficient_scope" => BearerError::InsufficientScope,
            _ => BearerError::Other,
        }
    }

    /// Returns true if the error indicates the OAuth client registration may be invalid
    /// and should be discarded to force re-registration.
    pub fn indicates_invalid_client(&self) -> bool {
        match self {
            BearerError::InvalidRequest => false,
            BearerError::InsufficientScope => false,
            BearerError::InvalidToken => true,
            BearerError::Other => true,
        }
    }
}

fn parse_token(input: &str) -> Option<(&str, &str)> {
    let bytes = input.as_bytes();
    let mut end = 0;

    while end < bytes.len() && is_tchar(bytes[end]) {
        end += 1;
    }

    if end == 0 {
        return None;
    }

    Some((&input[..end], &input[end..]))
}

fn is_tchar(byte: u8) -> bool {
    matches!(
        byte,
        b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~'
            | b'0'..=b'9'
            | b'A'..=b'Z'
            | b'a'..=b'z'
    )
}

fn parse_value<'a>(input: &'a str) -> Option<(Cow<'a, str>, &'a str)> {
    if let Some(rest) = input.strip_prefix('"') {
        parse_quoted_value(rest)
    } else {
        let (token, rest) = parse_token(input)?;
        Some((Cow::Borrowed(token), rest))
    }
}

fn parse_quoted_value<'a>(input: &'a str) -> Option<(Cow<'a, str>, &'a str)> {
    let mut output: Option<String> = None;
    let mut segment_start: usize = 0;

    let mut iter = input.as_bytes().iter().enumerate();

    while let Some((index, byte)) = iter.next() {
        match byte {
            b'"' => {
                let remainder = &input[index + 1..];

                if let Some(mut output) = output {
                    output.push_str(&input[segment_start..index]);
                    return Some((Cow::Owned(output), remainder));
                }

                return Some((Cow::Borrowed(&input[..index]), remainder));
            }
            b'\\' => {
                let (escaped_index, escaped_byte) = iter.next()?;

                let output = output.get_or_insert_with(String::new);
                output.push_str(&input[segment_start..index]);
                output.push(*escaped_byte as char);

                segment_start = escaped_index + 1;
            }
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_empty_bearer_challenge() {
        let challenge = WwwAuthenticate::parse("Bearer").expect("should parse Bearer scheme");
        assert_eq!(challenge, WwwAuthenticate::default());
    }

    #[test]
    fn rejects_non_bearer_scheme() {
        assert!(WwwAuthenticate::parse("Basic realm=\"example\"").is_none());
        assert!(WwwAuthenticate::parse("Digest realm=\"example\"").is_none());
    }

    #[test]
    fn parses_known_parameters_with_quoted_strings_and_tokens() {
        let challenge = WwwAuthenticate::parse(
            "Bearer realm=\"example\", scope=\"read write\", error=invalid_token, error_description=\"The access token expired\"",
        )
        .expect("should parse");

        assert_eq!(
            challenge,
            WwwAuthenticate {
                realm: Some(Cow::Borrowed("example")),
                scope: Some(Cow::Borrowed("read write")),
                error: Some(BearerError::InvalidToken),
                error_description: Some(Cow::Borrowed("The access token expired")),
                ..Default::default()
            }
        );
    }

    #[test]
    fn quoted_string_allows_commas_and_backslash_escapes() {
        let challenge = WwwAuthenticate::parse(
            "Bearer error_description=\"contains, comma and a quote: \\\" and a backslash: \\\\\"",
        )
        .expect("should parse");

        assert_eq!(
            challenge,
            WwwAuthenticate {
                error_description: Some(Cow::Owned(
                    "contains, comma and a quote: \" and a backslash: \\".to_string()
                )),
                ..Default::default()
            }
        );
    }

    #[test]
    fn ignores_unknown_extension_parameters() {
        let challenge =
            WwwAuthenticate::parse("Bearer realm=\"example\", foo=\"bar\"").expect("should parse");

        assert_eq!(
            challenge,
            WwwAuthenticate {
                realm: Some(Cow::Borrowed("example")),
                ..Default::default()
            }
        );
    }

    #[test]
    fn stops_at_subsequent_bearer_challenge_in_combined_header_value() {
        let challenge = WwwAuthenticate::parse(
            "Bearer realm=\"first\", error=\"invalid_token\", Bearer realm=\"second\"",
        )
        .expect("should parse");

        assert_eq!(
            challenge,
            WwwAuthenticate {
                realm: Some(Cow::Borrowed("first")),
                error: Some(BearerError::InvalidToken),
                ..Default::default()
            }
        );
    }

    #[test]
    fn parses_all_standard_error_codes() {
        let invalid_request =
            WwwAuthenticate::parse("Bearer error=invalid_request").expect("should parse");
        assert_eq!(invalid_request.error, Some(BearerError::InvalidRequest));

        let invalid_token =
            WwwAuthenticate::parse("Bearer error=invalid_token").expect("should parse");
        assert_eq!(invalid_token.error, Some(BearerError::InvalidToken));

        let insufficient_scope =
            WwwAuthenticate::parse("Bearer error=insufficient_scope").expect("should parse");
        assert_eq!(
            insufficient_scope.error,
            Some(BearerError::InsufficientScope)
        );
    }

    #[test]
    fn parses_unknown_error_as_other() {
        let challenge =
            WwwAuthenticate::parse("Bearer error=some_future_error").expect("should parse");
        assert_eq!(challenge.error, Some(BearerError::Other));
    }

    #[test]
    fn indicates_invalid_client_for_appropriate_errors() {
        assert!(!BearerError::InvalidRequest.indicates_invalid_client());
        assert!(!BearerError::InsufficientScope.indicates_invalid_client());
        assert!(BearerError::InvalidToken.indicates_invalid_client());
        assert!(BearerError::Other.indicates_invalid_client());
    }

    #[test]
    fn returns_none_on_invalid_trailing_garbage() {
        assert!(WwwAuthenticate::parse("Bearer realm=\"example\" garbage").is_none());
    }

    #[test]
    fn returns_none_on_missing_equals() {
        assert!(WwwAuthenticate::parse("Bearer realm \"example\"").is_none());
    }

    #[test]
    fn returns_none_on_unterminated_quoted_string() {
        assert!(WwwAuthenticate::parse("Bearer realm=\"example").is_none());
    }
}
