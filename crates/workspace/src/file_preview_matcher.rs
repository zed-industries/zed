use settings::FilePreviewModeRule;
use std::path::Path;

/// Matches a file path against a list of preview mode rules.
/// Returns the command to execute if a match is found.
///
/// Rules are processed in order - first match wins.
///
/// # Pattern Matching Priority
/// 1. Exact filename match (e.g., "README.md")
/// 2. Path pattern match (e.g., "docs/**/*.md")
/// 3. Extension wildcard match (e.g., "*.md")
///
/// # Examples
///
/// ```
/// use workspace::file_preview_matcher::match_file_pattern;
/// use settings::FilePreviewModeRule;
/// use std::path::Path;
///
/// let rules = vec![
///     FilePreviewModeRule {
///         filter: "*.md".to_string(),
///         command: "markdown::OpenPreview".to_string(),
///     },
/// ];
///
/// let path = Path::new("README.md");
/// assert_eq!(
///     match_file_pattern(path, &rules),
///     Some("markdown::OpenPreview".to_string())
/// );
/// ```
pub fn match_file_pattern(path: &Path, rules: &[FilePreviewModeRule]) -> Option<String> {
    // Extract filename for matching
    let filename = path.file_name()?.to_str()?;
    let path_str = path.to_str()?;

    // Process rules in order - first match wins
    for rule in rules {
        if matches_pattern(filename, path_str, &rule.filter) {
            return Some(rule.command.clone());
        }
    }

    None
}

/// Checks if a path matches a glob pattern.
///
/// Supports:
/// - Exact filename: "README.md"
/// - Extension wildcards: "*.md"
/// - Path patterns: "docs/**/*.md"
/// - Case-insensitive matching on appropriate platforms
fn matches_pattern(filename: &str, full_path: &str, pattern: &str) -> bool {
    // Exact filename match (highest priority)
    if pattern == filename {
        return true;
    }

    // Check if pattern contains path separators
    let has_path_separator = pattern.contains('/') || pattern.contains('\\');

    if has_path_separator {
        // Path pattern - match against full path
        matches_glob(full_path, pattern)
    } else {
        // Simple pattern - match against filename only
        matches_glob(filename, pattern)
    }
}

/// Simple glob matching supporting * and ** wildcards.
///
/// This is a simplified implementation that handles common cases:
/// - `*` matches any sequence of characters except path separators
/// - `**` matches any sequence including path separators
/// - `?` matches a single character
fn matches_glob(text: &str, pattern: &str) -> bool {
    // Handle case-insensitive matching on case-insensitive platforms
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        let text_lower = text.to_lowercase();
        let pattern_lower = pattern.to_lowercase();
        glob_match_impl(&text_lower, &pattern_lower)
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        glob_match_impl(text, pattern)
    }
}

/// Internal glob matching implementation.
fn glob_match_impl(text: &str, pattern: &str) -> bool {
    let mut text_idx = 0;
    let mut pattern_idx = 0;
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    while pattern_idx < pattern_bytes.len() {
        match pattern_bytes[pattern_idx] {
            b'*' => {
                pattern_idx += 1;

                // Check for **
                if pattern_idx < pattern_bytes.len() && pattern_bytes[pattern_idx] == b'*' {
                    pattern_idx += 1;
                    // ** matches everything including path separators
                    let remaining_pattern = &pattern[pattern_idx..];
                    if remaining_pattern.is_empty() {
                        return true;
                    }

                    // Try to match the rest of the pattern at every position
                    for i in text_idx..=text.len() {
                        if glob_match_impl(&text[i..], remaining_pattern) {
                            return true;
                        }
                    }
                    return false;
                } else {
                    // Single * matches any sequence except path separators
                    let remaining_pattern = &pattern[pattern_idx..];
                    if remaining_pattern.is_empty() {
                        // Pattern ends with *, consume rest of text (no path separators)
                        return !text[text_idx..].contains('/') && !text[text_idx..].contains('\\');
                    }

                    // Try to match at each position until we hit a path separator
                    for i in text_idx..=text.len() {
                        if i < text.len() {
                            let ch = text_bytes[i] as char;
                            if ch == '/' || ch == '\\' {
                                break;
                            }
                        }
                        if glob_match_impl(&text[i..], remaining_pattern) {
                            return true;
                        }
                    }
                    return false;
                }
            }
            b'?' => {
                pattern_idx += 1;
                if text_idx >= text.len() {
                    return false;
                }
                text_idx += 1;
            }
            _ => {
                if text_idx >= text.len() || text_bytes[text_idx] != pattern_bytes[pattern_idx] {
                    return false;
                }
                pattern_idx += 1;
                text_idx += 1;
            }
        }
    }

    text_idx == text.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rules(entries: &[(&str, &str)]) -> Vec<FilePreviewModeRule> {
        entries
            .iter()
            .map(|(filter, command)| FilePreviewModeRule {
                filter: filter.to_string(),
                command: command.to_string(),
            })
            .collect()
    }

    #[test]
    fn test_exact_filename_match() {
        let rules = make_rules(&[
            ("README.md", "markdown::OpenFollowingPreview"),
            ("*.md", "markdown::OpenPreview"),
        ]);

        assert_eq!(
            match_file_pattern(Path::new("README.md"), &rules),
            Some("markdown::OpenFollowingPreview".to_string())
        );
    }

    #[test]
    fn test_extension_wildcard() {
        let rules = make_rules(&[("*.md", "markdown::OpenPreview")]);

        assert_eq!(
            match_file_pattern(Path::new("README.md"), &rules),
            Some("markdown::OpenPreview".to_string())
        );

        assert_eq!(
            match_file_pattern(Path::new("docs/guide.md"), &rules),
            Some("markdown::OpenPreview".to_string())
        );

        assert_eq!(match_file_pattern(Path::new("README.txt"), &rules), None);
    }

    #[test]
    fn test_path_pattern() {
        let rules = make_rules(&[("docs/**/*.md", "markdown::OpenPreview")]);

        assert_eq!(
            match_file_pattern(Path::new("docs/guide.md"), &rules),
            Some("markdown::OpenPreview".to_string())
        );

        assert_eq!(
            match_file_pattern(Path::new("docs/api/reference.md"), &rules),
            Some("markdown::OpenPreview".to_string())
        );

        assert_eq!(match_file_pattern(Path::new("README.md"), &rules), None);
    }

    #[test]
    fn test_first_match_wins() {
        let rules = make_rules(&[
            ("README.md", "markdown::OpenFollowingPreview"),
            ("*.md", "markdown::OpenPreview"),
        ]);

        // First rule should match
        assert_eq!(
            match_file_pattern(Path::new("README.md"), &rules),
            Some("markdown::OpenFollowingPreview".to_string())
        );

        // Second rule should match
        assert_eq!(
            match_file_pattern(Path::new("other.md"), &rules),
            Some("markdown::OpenPreview".to_string())
        );
    }

    #[test]
    fn test_no_match() {
        let rules = make_rules(&[("*.md", "markdown::OpenPreview")]);

        assert_eq!(match_file_pattern(Path::new("README.txt"), &rules), None);
    }

    #[test]
    fn test_empty_rules() {
        let rules = vec![];
        assert_eq!(match_file_pattern(Path::new("README.md"), &rules), None);
    }

    #[test]
    fn test_multiple_file_types() {
        let rules = make_rules(&[
            ("*.md", "markdown::OpenPreview"),
            ("*.svg", "svg::OpenPreviewToTheSide"),
        ]);

        assert_eq!(
            match_file_pattern(Path::new("README.md"), &rules),
            Some("markdown::OpenPreview".to_string())
        );

        assert_eq!(
            match_file_pattern(Path::new("icon.svg"), &rules),
            Some("svg::OpenPreviewToTheSide".to_string())
        );
    }

    #[test]
    fn test_case_sensitivity() {
        let rules = make_rules(&[("*.md", "markdown::OpenPreview")]);

        // Case handling depends on platform
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            assert_eq!(
                match_file_pattern(Path::new("README.MD"), &rules),
                Some("markdown::OpenPreview".to_string())
            );
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            assert_eq!(match_file_pattern(Path::new("README.MD"), &rules), None);
        }
    }

    #[test]
    fn test_glob_matching() {
        assert!(matches_glob("README.md", "*.md"));
        assert!(matches_glob("README.md", "README.*"));
        assert!(matches_glob("README.md", "README.md"));
        assert!(!matches_glob("README.txt", "*.md"));

        assert!(matches_glob("docs/guide.md", "docs/*.md"));
        assert!(matches_glob("docs/api/ref.md", "docs/**/*.md"));
        assert!(!matches_glob("src/main.rs", "docs/**/*.md"));
    }

    #[test]
    fn test_pattern_ordering() {
        let rules = make_rules(&[
            ("docs/**/*.md", "markdown::OpenPreviewToTheSide"),
            ("*.md", "markdown::OpenPreview"),
        ]);

        // More specific pattern first
        assert_eq!(
            match_file_pattern(Path::new("docs/guide.md"), &rules),
            Some("markdown::OpenPreviewToTheSide".to_string())
        );

        // Less specific pattern for other files
        assert_eq!(
            match_file_pattern(Path::new("README.md"), &rules),
            Some("markdown::OpenPreview".to_string())
        );
    }
}
