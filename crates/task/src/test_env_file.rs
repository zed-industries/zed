//! Parser for the per-language `test_env_file` setting.
//!
//! Reads `.env`-formatted content into a key/value map. Lines that fail to
//! parse are collected as warnings; the rest of the file still applies.

use collections::HashMap;
use std::io::Read;

/// Parse a `.env`-formatted reader into a key/value map plus a list of
/// human-readable warnings for lines that failed to parse.
///
/// Variable expansion (`${VAR}`) inside the file is performed by `dotenvy`
/// against environment variables already set in the current process at the
/// time of parsing. Callers that need expansion against an arbitrary base
/// environment must seed that environment into the process, or rely on the
/// inherited project shell env.
pub fn parse_env_file<R: Read>(reader: R) -> (HashMap<String, String>, Vec<String>) {
    let mut env = HashMap::default();
    let mut warnings = Vec::new();
    for entry in dotenvy::from_read_iter(reader) {
        match entry {
            Ok((key, value)) => {
                env.insert(key, value);
            }
            Err(err) => warnings.push(err.to_string()),
        }
    }
    (env, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> (HashMap<String, String>, Vec<String>) {
        parse_env_file(input.as_bytes())
    }

    #[test]
    fn parses_basic_kv() {
        let (env, warnings) = parse("FOO=bar\nBAZ=qux\n");
        assert_eq!(env.get("FOO").map(String::as_str), Some("bar"));
        assert_eq!(env.get("BAZ").map(String::as_str), Some("qux"));
        assert!(warnings.is_empty());
    }

    #[test]
    fn ignores_comments_and_blank_lines() {
        let (env, warnings) = parse("# a comment\n\nFOO=bar\n# another\n");
        assert_eq!(env.len(), 1);
        assert_eq!(env.get("FOO").map(String::as_str), Some("bar"));
        assert!(warnings.is_empty());
    }

    #[test]
    fn handles_export_prefix() {
        let (env, warnings) = parse("export FOO=bar\n");
        assert_eq!(env.get("FOO").map(String::as_str), Some("bar"));
        assert!(warnings.is_empty());
    }

    #[test]
    fn handles_quoted_values() {
        let (env, warnings) = parse("FOO=\"hello world\"\nBAR='single quoted'\n");
        assert_eq!(env.get("FOO").map(String::as_str), Some("hello world"));
        assert_eq!(env.get("BAR").map(String::as_str), Some("single quoted"));
        assert!(warnings.is_empty());
    }

    #[test]
    fn malformed_line_collected_as_warning_parsing_continues() {
        let (env, warnings) = parse("VALID=ok\nthis is not a kv pair\nALSO_VALID=yes\n");
        assert_eq!(env.get("VALID").map(String::as_str), Some("ok"));
        assert_eq!(env.get("ALSO_VALID").map(String::as_str), Some("yes"));
        assert!(
            !warnings.is_empty(),
            "expected a warning for the malformed line"
        );
    }

    #[test]
    fn later_keys_in_same_file_override_earlier() {
        let (env, warnings) = parse("FOO=first\nFOO=second\n");
        assert_eq!(env.get("FOO").map(String::as_str), Some("second"));
        assert!(warnings.is_empty());
    }

    #[test]
    fn empty_input_produces_empty_map() {
        let (env, warnings) = parse("");
        assert!(env.is_empty());
        assert!(warnings.is_empty());
    }
}
