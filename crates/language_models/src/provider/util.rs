use std::str::FromStr;

/// Parses tool call arguments JSON, treating empty strings as empty objects.
///
/// Many LLM providers return empty strings for tool calls with no arguments.
/// This helper normalizes that behavior by converting empty strings to `{}`.
pub fn parse_tool_arguments(arguments: &str) -> Result<serde_json::Value, serde_json::Error> {
    if arguments.is_empty() {
        Ok(serde_json::Value::Object(Default::default()))
    } else {
        serde_json::Value::from_str(arguments)
    }
}

/// `partial_json_fixer::fix_json` converts a trailing `\` inside a string into `\\`
/// (a literal backslash). When used for incremental parsing (comparing successive
/// parses to extract deltas), this produces a spurious backslash character that
/// doesn't exist in the final text, corrupting the output.
///
/// This function strips any trailing incomplete escape sequence before fixing,
/// so each intermediate parse produces a true prefix of the final string value.
pub fn fix_streamed_json(partial_json: &str) -> String {
    let json = strip_trailing_incomplete_escape(partial_json);
    partial_json_fixer::fix_json(json)
}

fn strip_trailing_incomplete_escape(json: &str) -> &str {
    let trailing_backslashes = json
        .as_bytes()
        .iter()
        .rev()
        .take_while(|&&b| b == b'\\')
        .count();
    if trailing_backslashes % 2 == 1 {
        &json[..json.len() - 1]
    } else {
        json
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_streamed_json_strips_incomplete_escape() {
        // Trailing `\` inside a string — incomplete escape sequence
        let fixed = fix_streamed_json(r#"{"text": "hello\"#);
        let parsed: serde_json::Value = serde_json::from_str(&fixed).expect("valid json");
        assert_eq!(parsed["text"], "hello");
    }

    #[test]
    fn test_fix_streamed_json_preserves_complete_escape() {
        // `\\` is a complete escape (literal backslash)
        let fixed = fix_streamed_json(r#"{"text": "hello\\"#);
        let parsed: serde_json::Value = serde_json::from_str(&fixed).expect("valid json");
        assert_eq!(parsed["text"], "hello\\");
    }

    #[test]
    fn test_fix_streamed_json_strips_escape_after_complete_escape() {
        // `\\\` = complete `\\` (literal backslash) + incomplete `\`
        let fixed = fix_streamed_json(r#"{"text": "hello\\\"#);
        let parsed: serde_json::Value = serde_json::from_str(&fixed).expect("valid json");
        assert_eq!(parsed["text"], "hello\\");
    }

    #[test]
    fn test_fix_streamed_json_no_escape_at_end() {
        let fixed = fix_streamed_json(r#"{"text": "hello"#);
        let parsed: serde_json::Value = serde_json::from_str(&fixed).expect("valid json");
        assert_eq!(parsed["text"], "hello");
    }

    #[test]
    fn test_fix_streamed_json_newline_escape_boundary() {
        // Simulates a stream boundary landing between `\` and `n`
        let fixed = fix_streamed_json(r#"{"text": "line1\"#);
        let parsed: serde_json::Value = serde_json::from_str(&fixed).expect("valid json");
        assert_eq!(parsed["text"], "line1");

        // Next chunk completes the escape
        let fixed = fix_streamed_json(r#"{"text": "line1\nline2"#);
        let parsed: serde_json::Value = serde_json::from_str(&fixed).expect("valid json");
        assert_eq!(parsed["text"], "line1\nline2");
    }

    #[test]
    fn test_fix_streamed_json_incremental_delta_correctness() {
        // This is the actual scenario that causes the bug:
        // chunk 1 ends mid-escape, chunk 2 completes it.
        let chunk1 = r#"{"replacement_text": "fn foo() {\"#;
        let fixed1 = fix_streamed_json(chunk1);
        let parsed1: serde_json::Value = serde_json::from_str(&fixed1).expect("valid json");
        let text1 = parsed1["replacement_text"].as_str().expect("string");
        assert_eq!(text1, "fn foo() {");

        let chunk2 = r#"{"replacement_text": "fn foo() {\n    return bar;\n}"}"#;
        let fixed2 = fix_streamed_json(chunk2);
        let parsed2: serde_json::Value = serde_json::from_str(&fixed2).expect("valid json");
        let text2 = parsed2["replacement_text"].as_str().expect("string");
        assert_eq!(text2, "fn foo() {\n    return bar;\n}");

        // The delta should be the newline + rest, with no spurious backslash
        let delta = &text2[text1.len()..];
        assert_eq!(delta, "\n    return bar;\n}");
    }
}
