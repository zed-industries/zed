use shell_command_parser::extract_commands;
use std::path::{Path, PathBuf};
use url::Url;

/// Normalize path separators to forward slashes for consistent cross-platform patterns.
fn normalize_separators(path_str: &str) -> String {
    path_str.replace('\\', "/")
}

/// Extracts the command name from a shell command using the shell parser.
///
/// This parses the command properly to extract just the command name (first word),
/// handling shell syntax correctly. Returns `None` if parsing fails or if the
/// command name contains path separators (for security reasons).
fn extract_command_name(command: &str) -> Option<String> {
    let commands = extract_commands(command)?;
    let first_command = commands.first()?;

    let first_token = first_command.split_whitespace().next()?;

    // Only allow alphanumeric commands with hyphens/underscores.
    // Reject paths like "./script.sh" or "/usr/bin/python" to prevent
    // users from accidentally allowing arbitrary script execution.
    if first_token
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        Some(first_token.to_string())
    } else {
        None
    }
}

/// Extracts a regex pattern from a terminal command based on the first token (command name).
///
/// Returns `None` for commands starting with `./`, `/`, or other path-like prefixes.
/// This is a deliberate security decision: we only allow pattern-based "always allow"
/// rules for well-known command names (like `cargo`, `npm`, `git`), not for arbitrary
/// scripts or absolute paths which could be manipulated by an attacker.
pub fn extract_terminal_pattern(command: &str) -> Option<String> {
    let command_name = extract_command_name(command)?;
    Some(format!("^{}\\b", regex::escape(&command_name)))
}

pub fn extract_terminal_pattern_display(command: &str) -> Option<String> {
    extract_command_name(command)
}

pub fn extract_path_pattern(path: &str) -> Option<String> {
    let parent = Path::new(path).parent()?;
    let parent_str = normalize_separators(parent.to_str()?);
    if parent_str.is_empty() || parent_str == "/" {
        return None;
    }
    Some(format!("^{}/", regex::escape(&parent_str)))
}

pub fn extract_path_pattern_display(path: &str) -> Option<String> {
    let parent = Path::new(path).parent()?;
    let parent_str = normalize_separators(parent.to_str()?);
    if parent_str.is_empty() || parent_str == "/" {
        return None;
    }
    Some(format!("{}/", parent_str))
}

fn common_parent_dir(path_a: &str, path_b: &str) -> Option<PathBuf> {
    let parent_a = Path::new(path_a).parent()?;
    let parent_b = Path::new(path_b).parent()?;

    let components_a: Vec<_> = parent_a.components().collect();
    let components_b: Vec<_> = parent_b.components().collect();

    let common_count = components_a
        .iter()
        .zip(components_b.iter())
        .take_while(|(a, b)| a == b)
        .count();

    if common_count == 0 {
        return None;
    }

    let common: PathBuf = components_a[..common_count].iter().collect();
    Some(common)
}

pub fn extract_copy_move_pattern(input: &str) -> Option<String> {
    let (source, dest) = input.split_once('\n')?;
    let common = common_parent_dir(source, dest)?;
    let common_str = normalize_separators(common.to_str()?);
    if common_str.is_empty() || common_str == "/" {
        return None;
    }
    Some(format!("^{}/", regex::escape(&common_str)))
}

pub fn extract_copy_move_pattern_display(input: &str) -> Option<String> {
    let (source, dest) = input.split_once('\n')?;
    let common = common_parent_dir(source, dest)?;
    let common_str = normalize_separators(common.to_str()?);
    if common_str.is_empty() || common_str == "/" {
        return None;
    }
    Some(format!("{}/", common_str))
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
            Some("^cargo\\b".to_string())
        );
        assert_eq!(
            extract_terminal_pattern("npm install"),
            Some("^npm\\b".to_string())
        );
        assert_eq!(
            extract_terminal_pattern("git-lfs pull"),
            Some("^git\\-lfs\\b".to_string())
        );
        assert_eq!(
            extract_terminal_pattern("my_script arg"),
            Some("^my_script\\b".to_string())
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

    #[test]
    fn test_extract_copy_move_pattern_same_directory() {
        assert_eq!(
            extract_copy_move_pattern(
                "/Users/alice/project/src/old.rs\n/Users/alice/project/src/new.rs"
            ),
            Some("^/Users/alice/project/src/".to_string())
        );
    }

    #[test]
    fn test_extract_copy_move_pattern_sibling_directories() {
        assert_eq!(
            extract_copy_move_pattern(
                "/Users/alice/project/src/old.rs\n/Users/alice/project/dst/new.rs"
            ),
            Some("^/Users/alice/project/".to_string())
        );
    }

    #[test]
    fn test_extract_copy_move_pattern_no_common_prefix() {
        assert_eq!(
            extract_copy_move_pattern("/home/file.txt\n/tmp/file.txt"),
            None
        );
    }

    #[test]
    fn test_extract_copy_move_pattern_relative_paths() {
        assert_eq!(
            extract_copy_move_pattern("src/old.rs\nsrc/new.rs"),
            Some("^src/".to_string())
        );
    }

    #[test]
    fn test_extract_copy_move_pattern_display() {
        assert_eq!(
            extract_copy_move_pattern_display(
                "/Users/alice/project/src/old.rs\n/Users/alice/project/dst/new.rs"
            ),
            Some("/Users/alice/project/".to_string())
        );
    }

    #[test]
    fn test_extract_copy_move_pattern_no_arrow() {
        assert_eq!(extract_copy_move_pattern("just/a/path.rs"), None);
        assert_eq!(extract_copy_move_pattern_display("just/a/path.rs"), None);
    }
}
