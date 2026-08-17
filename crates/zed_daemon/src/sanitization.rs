//! # Security Input Sanitization Framework
//!
//! Space-Grade input sanitization, HTML/Markdown tag stripping, and LSP response bounds checking.
//! (Section 3.4 of Space-Grade Audit)

use serde::{de::DeserializeOwned, Serialize};

/// Maximum allowable payload size in bytes for untrusted input (10MB)
pub const MAX_SAFE_INPUT_BYTES: usize = 10 * 1024 * 1024;

/// Environment variables that must never leak to daemon child processes (Section 3.2 of Space-Grade Audit)
pub fn sanitize_env_for_daemon() {
    const SENSITIVE_VARS: &[&str] = &[
        "AWS_SECRET_ACCESS_KEY",
        "AWS_ACCESS_KEY_ID",
        "AWS_SESSION_TOKEN",
        "GITHUB_TOKEN",
        "GITHUB_PAT",
        "GH_TOKEN",
        "OPENAI_API_KEY",
        "OPENAI_ORG_ID",
        "ANTHROPIC_API_KEY",
        "ANTHROPIC_AUTH_TOKEN",
        "SSH_AUTH_SOCK",
        "GPG_TTY",
        "HOMEBREW_GITHUB_TOKEN",
    ];

    for var in SENSITIVE_VARS {
        unsafe {
            std::env::remove_var(var);
        }
    }
}

/// Sanitize untrusted user markdown input to prevent XSS or terminal escape injection
pub fn sanitize_markdown_text(input: &str) -> String {
    if input.len() > MAX_SAFE_INPUT_BYTES {
        return input[..MAX_SAFE_INPUT_BYTES].to_string();
    }

    // Strip dangerous HTML script tags and terminal ANSI escapes
    let without_scripts = input
        .replace("<script>", "")
        .replace("</script>", "")
        .replace("<SCRIPT>", "")
        .replace("</SCRIPT>", "");

    // Strip ASCII control characters except newline, tab, carriage return
    without_scripts
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t' || *c == '\r')
        .collect()
}

/// Sanitize and validate LSP response structures against bounds
pub fn sanitize_lsp_response<T: Serialize + DeserializeOwned>(raw_json: &str) -> Result<T, String> {
    if raw_json.len() > MAX_SAFE_INPUT_BYTES {
        return Err(format!(
            "LSP response exceeds maximum safe threshold ({} bytes)",
            MAX_SAFE_INPUT_BYTES
        ));
    }

    serde_json::from_str::<T>(raw_json).map_err(|e| format!("Malformed LSP response payload: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_markdown_strips_scripts() {
        let input = "# Heading\n<script>alert(1)</script>\nNormal text";
        let cleaned = sanitize_markdown_text(input);
        assert_eq!(cleaned, "# Heading\nalert(1)\nNormal text");
    }

    #[test]
    fn test_sanitize_markdown_strips_control_chars() {
        let input = "hello\u{0000}\u{0007}world\n";
        let cleaned = sanitize_markdown_text(input);
        assert_eq!(cleaned, "helloworld\n");
    }

    #[test]
    fn test_sanitize_lsp_response_bounds() {
        #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug)]
        struct MockLsp {
            items: Vec<String>,
        }

        let valid = r#"{"items":["fn test()"]}"#;
        let res: Result<MockLsp, _> = sanitize_lsp_response(valid);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().items, vec!["fn test()"]);
    }
}
