use url::Url;

/// Extracts a regex pattern from a terminal command based on the first token (command name).
///
/// Returns `None` for commands starting with `./`, `/`, or other path-like prefixes.
/// This is a deliberate security decision: we only allow pattern-based "always allow"
/// rules for well-known command names (like `cargo`, `npm`, `git`), not for arbitrary
/// scripts or absolute paths which could be manipulated by an attacker.
pub fn extract_terminal_pattern(command: &str) -> Option<String> {
    let first_token = command.split_whitespace().next()?;
    // Only allow alphanumeric commands with hyphens/underscores.
    // Reject paths like "./script.sh" or "/usr/bin/python" to prevent
    // users from accidentally allowing arbitrary script execution.
    if first_token
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        Some(format!("^{}\\s", regex::escape(first_token)))
    } else {
        None
    }
}

pub fn extract_terminal_pattern_display(command: &str) -> Option<String> {
    let first_token = command.split_whitespace().next()?;
    if first_token
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        Some(first_token.to_string())
    } else {
        None
    }
}

pub fn extract_path_pattern(path: &str) -> Option<String> {
    let parent = std::path::Path::new(path).parent()?;
    let parent_str = parent.to_str()?;
    if parent_str.is_empty() || parent_str == "/" {
        return None;
    }
    Some(format!("^{}/", regex::escape(parent_str)))
}

pub fn extract_path_pattern_display(path: &str) -> Option<String> {
    let parent = std::path::Path::new(path).parent()?;
    let parent_str = parent.to_str()?;
    if parent_str.is_empty() || parent_str == "/" {
        return None;
    }
    Some(format!("{}/", parent_str))
}

pub fn extract_url_pattern(url: &str) -> Option<String> {
    let parsed = Url::parse(url).ok()?;
    let domain = parsed.host_str()?;
    Some(format!("^https?://{}", regex::escape(domain)))
}

pub fn extract_url_pattern_display(url: &str) -> Option<String> {
    let parsed = Url::parse(url).ok()?;
    let domain = parsed.host_str()?;
    Some(domain.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_terminal_pattern() {
        assert_eq!(
            extract_terminal_pattern("cargo build --release"),
            Some("^cargo\\s".to_string())
        );
        assert_eq!(
            extract_terminal_pattern("npm install"),
            Some("^npm\\s".to_string())
        );
        assert_eq!(
            extract_terminal_pattern("git-lfs pull"),
            Some("^git\\-lfs\\s".to_string())
        );
        assert_eq!(
            extract_terminal_pattern("my_script arg"),
            Some("^my_script\\s".to_string())
        );
        assert_eq!(extract_terminal_pattern("./script.sh arg"), None);
        assert_eq!(extract_terminal_pattern("/usr/bin/python arg"), None);
    }

    #[test]
    fn test_extract_terminal_pattern_display() {
        assert_eq!(
            extract_terminal_pattern_display("cargo build --release"),
            Some("cargo".to_string())
        );
        assert_eq!(
            extract_terminal_pattern_display("npm install"),
            Some("npm".to_string())
        );
    }

    #[test]
    fn test_extract_path_pattern() {
        assert_eq!(
            extract_path_pattern("/Users/alice/project/src/main.rs"),
            Some("^/Users/alice/project/src/".to_string())
        );
        assert_eq!(
            extract_path_pattern("src/lib.rs"),
            Some("^src/".to_string())
        );
        assert_eq!(extract_path_pattern("file.txt"), None);
        assert_eq!(extract_path_pattern("/file.txt"), None);
    }

    #[test]
    fn test_extract_path_pattern_display() {
        assert_eq!(
            extract_path_pattern_display("/Users/alice/project/src/main.rs"),
            Some("/Users/alice/project/src/".to_string())
        );
        assert_eq!(
            extract_path_pattern_display("src/lib.rs"),
            Some("src/".to_string())
        );
    }

    #[test]
    fn test_extract_url_pattern() {
        assert_eq!(
            extract_url_pattern("https://github.com/user/repo"),
            Some("^https?://github\\.com".to_string())
        );
        assert_eq!(
            extract_url_pattern("http://example.com/path?query=1"),
            Some("^https?://example\\.com".to_string())
        );
        assert_eq!(extract_url_pattern("not a url"), None);
    }

    #[test]
    fn test_extract_url_pattern_display() {
        assert_eq!(
            extract_url_pattern_display("https://github.com/user/repo"),
            Some("github.com".to_string())
        );
        assert_eq!(
            extract_url_pattern_display("http://api.example.com/v1/users"),
            Some("api.example.com".to_string())
        );
    }

    #[test]
    fn test_special_chars_are_escaped() {
        assert_eq!(
            extract_path_pattern("/path/with (parens)/file.txt"),
            Some("^/path/with \\(parens\\)/".to_string())
        );
        assert_eq!(
            extract_url_pattern("https://test.example.com/path"),
            Some("^https?://test\\.example\\.com".to_string())
        );
    }
}
