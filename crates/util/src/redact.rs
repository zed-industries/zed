const REDACTED_SUFFIXES: &[&str] = &[
    "KEY",
    "TOKEN",
    "PASSWORD",
    "SECRET",
    "PASS",
    "CREDENTIALS",
    "LICENSE",
];

/// Whether a given environment variable name should have its value redacted
pub fn should_redact(env_var_name: &str) -> bool {
    let name = env_var_name.to_ascii_uppercase();
    REDACTED_SUFFIXES
        .iter()
        .any(|suffix| name.ends_with(suffix))
}

pub fn is_valid_environment_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|character| character != '=' && !character.is_control())
}

fn is_name_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_name_character(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

// Some command debug strings escape their embedded quotes (\"value\").
// Find the closing quote for both literal and backslash-escaped delimiters.
fn closing_quote(bytes: &[u8], mut pos: usize, quote: u8, escaped: bool) -> Option<usize> {
    while pos < bytes.len() {
        if bytes[pos] == b'\\' && pos + 1 < bytes.len() {
            if escaped && bytes[pos + 1] == quote {
                return Some(pos);
            }
            if !escaped || bytes[pos + 1] == b'\\' {
                pos += 2;
                continue;
            }
        }
        if !escaped && bytes[pos] == quote {
            return Some(pos);
        }
        pos += 1;
    }
    None
}

// Consume a complete shell word, allowing quoted sections and escaped spaces.
// Never end a redaction in the middle of a quoted secret value.
fn value_end(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() {
        match bytes[pos] {
            byte if byte.is_ascii_whitespace() => break,
            quote @ (b'"' | b'\'') => {
                pos = closing_quote(bytes, pos + 1, quote, false)
                    .map_or(bytes.len(), |end| end + 1);
            }
            b'\\' if pos + 1 < bytes.len() && matches!(bytes[pos + 1], b'"' | b'\'') => {
                pos = closing_quote(bytes, pos + 2, bytes[pos + 1], true)
                    .map_or(bytes.len(), |end| end + 2);
            }
            b'\\' if pos + 1 < bytes.len() => pos += 2,
            _ => pos += 1,
        }
    }
    pos
}

// Scan quoted fragments independently. In particular, a name inside a quoted
// value must not consume a later assignment across the closing quote.
fn redact_fragment(command: &str, within_quotes: bool) -> String {
    let bytes = command.as_bytes();
    let mut redacted = String::with_capacity(command.len());
    let mut pos = 0;

    while pos < bytes.len() {
        let quoted = match bytes[pos] {
            b'"' | b'\'' => Some((bytes[pos], 1, false)),
            b'\\' if pos + 1 < bytes.len() && matches!(bytes[pos + 1], b'"' | b'\'') => {
                Some((bytes[pos + 1], 2, true))
            }
            _ => None,
        };
        if let Some((quote, width, escaped)) = quoted {
            if let Some(end) = closing_quote(bytes, pos + width, quote, escaped) {
                redacted.push_str(&command[pos..pos + width]);
                redacted.push_str(&redact_fragment(&command[pos + width..end], true));
                redacted.push_str(&command[end..end + width]);
                pos = end + width;
                continue;
            }
        }

        // Also match after separators such as --env= or ;, but do not start
        // in the middle of an existing environment variable name.
        if is_name_character(bytes[pos]) && (pos == 0 || !is_name_character(bytes[pos - 1])) {
            let mut name_start = pos;
            while name_start < bytes.len() && bytes[name_start].is_ascii_digit() {
                name_start += 1;
            }
            let mut name_end = name_start;
            while name_end < bytes.len() && is_name_character(bytes[name_end]) {
                name_end += 1;
            }

            if name_start < name_end
                && is_name_start(bytes[name_start])
                && bytes.get(name_end) == Some(&b'=')
                && should_redact(&command[name_start..name_end])
            {
                let value_start = name_end + 1;
                let value = &command[value_start..];
                // If an assignment appears unquoted inside a quoted argument,
                // its value may itself contain spaces. Conservatively hide the
                // rest of that fragment, never the next quoted argument.
                let end = if within_quotes
                    && !value.starts_with('"')
                    && !value.starts_with('\'')
                    && !value.starts_with("\\\"")
                    && !value.starts_with("\\'")
                {
                    bytes.len()
                } else {
                    value_end(bytes, value_start)
                };

                redacted.push_str(&command[pos..name_end]);
                redacted.push('=');
                if within_quotes {
                    // Preserve quotes already enclosing this fragment, as in
                    // a Command debug argument or another variable's value.
                    let value = &command[value_start..end];
                    if value.starts_with("\\\"") {
                        redacted.push_str("\\\"[REDACTED]\\\"");
                    } else if value.starts_with("\\'") {
                        redacted.push_str("\\'[REDACTED]\\'");
                    } else if value.starts_with('"') {
                        redacted.push_str("\"[REDACTED]\"");
                    } else if value.starts_with('\'') {
                        redacted.push_str("'[REDACTED]'");
                    } else {
                        redacted.push_str("[REDACTED]");
                    }
                } else {
                    redacted.push_str("\"[REDACTED]\"");
                }
                pos = end;
                continue;
            }

            redacted.push_str(&command[pos..name_end]);
            pos = name_end;
            continue;
        }

        // Leave escaped characters alone, including escaped Unicode chars.
        if bytes[pos] == b'\\' && pos + 1 < bytes.len() {
            let next = command[pos + 1..].chars().next().unwrap();
            let end = pos + 1 + next.len_utf8();
            redacted.push_str(&command[pos..end]);
            pos = end;
        } else {
            let next = command[pos..].chars().next().unwrap();
            redacted.push(next);
            pos += next.len_utf8();
        }
    }
    redacted
}

/// Redact a string which could include a command with environment variables
pub fn redact_command(command: &str) -> String {
    redact_fragment(command, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_string_with_multiple_env_vars() {
        let input = r#"failed to spawn command cd "/code/something" && ANTHROPIC_API_KEY="sk-ant-api03-WOOOO" COMMAND_MODE="unix2003" GEMINI_API_KEY="AIGEMINIFACE" HOME="/Users/foo""#;
        let result = redact_command(input);
        let expected = r#"failed to spawn command cd "/code/something" && ANTHROPIC_API_KEY="[REDACTED]" COMMAND_MODE="unix2003" GEMINI_API_KEY="[REDACTED]" HOME="/Users/foo""#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_should_redact_case_insensitive_names() {
        for name in ["api_key", "Access_Token", "clientSecret", "db_password"] {
            assert!(should_redact(name), "{name} should be redacted");
        }
        for name in ["home", "Log_Level", "command_mode"] {
            assert!(!should_redact(name), "{name} should not be redacted");
        }
    }

    #[test]
    fn test_redact_command_with_mixed_case_environment_variables() {
        let input = r#"command api_key="lower-secret" OpenAi_Access_Token='mixed-secret' clientSecret=plain-secret HOME="/home/me" log_level=debug"#;
        let expected = r#"command api_key="[REDACTED]" OpenAi_Access_Token="[REDACTED]" clientSecret="[REDACTED]" HOME="/home/me" log_level=debug"#;
        assert_eq!(redact_command(input), expected);
    }

    #[test]
    fn test_redact_command_with_nested_environment_assignments() {
        for (input, expected) in [
            (
                "docker run --env=API_KEY=synthetic-secret image",
                r#"docker run --env=API_KEY="[REDACTED]" image"#,
            ),
            (
                "mode=debug;API_KEY=synthetic-secret command",
                r#"mode=debug;API_KEY="[REDACTED]" command"#,
            ),
            (
                "docker run --env=api_key=lower-secret image",
                r#"docker run --env=api_key="[REDACTED]" image"#,
            ),
            (
                "mode=debug;Access_Token='mixed-secret' command",
                r#"mode=debug;Access_Token="[REDACTED]" command"#,
            ),
            (
                "docker run --env=HOME=/home/me image",
                "docker run --env=HOME=/home/me image",
            ),
            (
                "KEY=standalone TOKEN=standalone",
                r#"KEY="[REDACTED]" TOKEN="[REDACTED]""#,
            ),
        ] {
            assert_eq!(redact_command(input), expected, "input: {input}");
        }
    }

    #[test]
    fn test_redact_command_with_adjacent_quoted_secret_assignments() {
        for (input, expected) in [
            (
                "access_token=public;API_KEY='synthetic secret' command",
                r#"access_token="[REDACTED]" command"#,
            ),
            (
                r#"access_token=public;API_KEY="synthetic secret" command"#,
                r#"access_token="[REDACTED]" command"#,
            ),
            (
                "mode=debug;API_KEY='synthetic secret' command",
                r#"mode=debug;API_KEY="[REDACTED]" command"#,
            ),
            (
                "Access_Token=public&&api_key='synthetic secret' command",
                r#"Access_Token="[REDACTED]" command"#,
            ),
            (
                "access_token=public||API_KEY='synthetic secret' command",
                r#"access_token="[REDACTED]" command"#,
            ),
            (
                "access_token=public|API_KEY='synthetic secret' command",
                r#"access_token="[REDACTED]" command"#,
            ),
            (
                r"access_token=public\;escaped;API_KEY='synthetic secret' command",
                r#"access_token="[REDACTED]" command"#,
            ),
            (
                r"access_token=public\ with\ spaces;API_KEY='synthetic secret' command",
                r#"access_token="[REDACTED]" command"#,
            ),
            (
                "API_KEY='secret with; semicolon';access_token=plain",
                r#"API_KEY="[REDACTED]""#,
            ),
        ] {
            assert_eq!(redact_command(input), expected, "input: {input}");
        }
    }

    #[test]
    fn test_redact_command_with_unquoted_punctuation_and_quoted_segments() {
        for (input, expected) in [
            ("API_KEY=abc&def cmd", r#"API_KEY="[REDACTED]" cmd"#),
            ("api_key=abc;def cmd", r#"api_key="[REDACTED]" cmd"#),
            (
                "Access_Token=abc|def cmd",
                r#"Access_Token="[REDACTED]" cmd"#,
            ),
            (
                "clientSecret=abc,def cmd",
                r#"clientSecret="[REDACTED]" cmd"#,
            ),
            (r"API_KEY=abc\ def cmd", r#"API_KEY="[REDACTED]" cmd"#),
            (
                r#"API_KEY=abc" two words"def cmd"#,
                r#"API_KEY="[REDACTED]" cmd"#,
            ),
            (
                "access_token=public,API_KEY='synthetic secret' command",
                r#"access_token="[REDACTED]" command"#,
            ),
            (
                "mode=debug,API_KEY='synthetic secret' command",
                r#"mode=debug,API_KEY="[REDACTED]" command"#,
            ),
        ] {
            assert_eq!(redact_command(input), expected, "input: {input}");
        }
    }
    #[test]
    fn test_redact_command_preserves_nested_quote_boundaries() {
        for (input, expected) in [
            (
                r#"DATABASE_URL="postgres://db/app?password=dev" PGPASSWORD="correct horse battery staple""#,
                r#"DATABASE_URL="postgres://db/app?password=[REDACTED]" PGPASSWORD="[REDACTED]""#,
            ),
            (
                r#"DATABASE_URL="postgres://db/app?password=dev" pgpassword='correct horse battery staple'"#,
                r#"DATABASE_URL="postgres://db/app?password=[REDACTED]" pgpassword="[REDACTED]""#,
            ),
            (
                r#""DATABASE_URL=\"postgres://db/app?password=dev\" PGPASSWORD=\"correct horse battery staple\"""#,
                r#""DATABASE_URL=\"postgres://db/app?password=[REDACTED]\" PGPASSWORD=\"[REDACTED]\"""#,
            ),
            (
                r#""--env=api_key=\"correct horse battery staple\"""#,
                r#""--env=api_key=\"[REDACTED]\"""#,
            ),
            (
                r#""sh -c API_KEY=correct horse battery staple""#,
                r#""sh -c API_KEY=[REDACTED]""#,
            ),
            (
                r#"DATABASE_URL="host?PASSWORD=dev" PGPASSWORD="two words""#,
                r#"DATABASE_URL="host?PASSWORD=[REDACTED]" PGPASSWORD="[REDACTED]""#,
            ),
            (
                "API_KEY=🦀 PGPASSWORD='two words'",
                r#"API_KEY="[REDACTED]" PGPASSWORD="[REDACTED]""#,
            ),
        ] {
            assert_eq!(redact_command(input), expected, "input: {input}");
        }
    }
}
