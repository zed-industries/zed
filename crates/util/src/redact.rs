use std::sync::LazyLock;

const REDACTED_SUFFIXES: &[&str] = &[
    "KEY",
    "TOKEN",
    "PASSWORD",
    "SECRET",
    "PASS",
    "CREDENTIALS",
    "LICENSE",
];

static REDACT_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    // Only match sensitive names so a non-sensitive assignment does not
    // consume a nested or adjacent secret assignment as part of its value.
    regex::Regex::new(&format!(
        r#"(?i)((?:[a-z_][a-z0-9_]*)?(?:{}))=(?:"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|\S+)"#,
        REDACTED_SUFFIXES.join("|"),
    ))
    .unwrap()
});

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

/// Redact a string which could include a command with environment variables
pub fn redact_command(command: &str) -> String {
    REDACT_REGEX
        .replace_all(command, r#"${1}="[REDACTED]""#)
        .to_string()
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
}
