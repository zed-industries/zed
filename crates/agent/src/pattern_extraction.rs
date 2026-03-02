use shell_command_parser::extract_commands;
use std::path::{Path, PathBuf};
use url::Url;

/// Normalize path separators to forward slashes for consistent cross-platform patterns.
fn normalize_separators(path_str: &str) -> String {
    path_str.replace('\\', "/")
}

/// Returns true if the token looks like a command name or subcommand — i.e. it
/// contains only alphanumeric characters, hyphens, and underscores, and does not
/// start with a hyphen (which would make it a flag).
fn is_plain_command_token(token: &str) -> bool {
    !token.starts_with('-')
        && token
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

struct CommandPrefix {
    command: String,
    subcommand: Option<String>,
}

/// Extracts the command name and optional subcommand from a shell command using
/// the shell parser.
///
/// This parses the command properly to extract the command name and optional
/// subcommand (e.g. "cargo" and "test" from "cargo test -p search"), handling shell
/// syntax correctly. Returns `None` if parsing fails or if the command name
/// contains path separators (for security reasons).
fn extract_command_prefix(command: &str) -> Option<CommandPrefix> {
    let commands = extract_commands(command)?;
    let first_command = commands.first()?;

    let mut tokens = first_command.split_whitespace();
    let first_token = tokens.next()?;

    // Only allow alphanumeric commands with hyphens/underscores.
    // Reject paths like "./script.sh" or "/usr/bin/python" to prevent
    // users from accidentally allowing arbitrary script execution.
    if !is_plain_command_token(first_token) {
        return None;
    }

    // Include the subcommand (second non-flag token) when present, to produce
    // more specific patterns like "cargo test" instead of just "cargo".
    let subcommand = tokens
        .next()
        .filter(|second_token| is_plain_command_token(second_token))
        .map(|second_token| second_token.to_string());

    Some(CommandPrefix {
        command: first_token.to_string(),
        subcommand,
    })
}

/// Extracts a regex pattern from a terminal command based on the first token (command name).
///
/// Returns `None` for commands starting with `./`, `/`, or other path-like prefixes.
/// This is a deliberate security decision: we only allow pattern-based "always allow"
/// rules for well-known command names (like `cargo`, `npm`, `git`), not for arbitrary
/// scripts or absolute paths which could be manipulated by an attacker.
pub fn extract_terminal_pattern(command: &str) -> Option<String> {
    let prefix = extract_command_prefix(command)?;
    let escaped_command = regex::escape(&prefix.command);
    Some(match &prefix.subcommand {
        Some(subcommand) => {
            format!(
                "^{}\\s+{}(\\s|$)",
                escaped_command,
                regex::escape(subcommand)
            )
        }
        None => format!("^{}\\b", escaped_command),
    })
}

pub fn extract_terminal_pattern_display(command: &str) -> Option<String> {
    let prefix = extract_command_prefix(command)?;
    match prefix.subcommand {
        Some(subcommand) => Some(format!("{} {}", prefix.command, subcommand)),
        None => Some(prefix.command),
    }
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
            Some("^cargo\\s+build(\\s|$)".to_string())
        );
        assert_eq!(
            extract_terminal_pattern("cargo test -p search"),
            Some("^cargo\\s+test(\\s|$)".to_string())
        );
        assert_eq!(
            extract_terminal_pattern("npm install"),
            Some("^npm\\s+install(\\s|$)".to_string())
        );
        assert_eq!(
            extract_terminal_pattern("git-lfs pull"),
            Some("^git\\-lfs\\s+pull(\\s|$)".to_string())
        );
        assert_eq!(
            extract_terminal_pattern("my_script arg"),
            Some("^my_script\\s+arg(\\s|$)".to_string())
        );

        // Flags as second token: only the command name is used
        assert_eq!(
            extract_terminal_pattern("ls -la"),
            Some("^ls\\b".to_string())
        );
        assert_eq!(
            extract_terminal_pattern("rm --force foo"),
            Some("^rm\\b".to_string())
        );

        // Single-word commands
        assert_eq!(extract_terminal_pattern("ls"), Some("^ls\\b".to_string()));

        // Subcommand pattern does not match a hyphenated extension of the subcommand
        // (e.g. approving "cargo build" should not approve "cargo build-foo")
        assert_eq!(
            extract_terminal_pattern("cargo build"),
            Some("^cargo\\s+build(\\s|$)".to_string())
        );
        let pattern = regex::Regex::new(&extract_terminal_pattern("cargo build").unwrap()).unwrap();
        assert!(pattern.is_match("cargo build --release"));
        assert!(pattern.is_match("cargo build"));
        assert!(!pattern.is_match("cargo build-foo"));
        assert!(!pattern.is_match("cargo builder"));

        // Path-like commands are rejected
        assert_eq!(extract_terminal_pattern("./script.sh arg"), None);
        assert_eq!(extract_terminal_pattern("/usr/bin/python arg"), None);
    }

    #[test]
    fn test_extract_terminal_pattern_display() {
        assert_eq!(
            extract_terminal_pattern_display("cargo build --release"),
            Some("cargo build".to_string())
        );
        assert_eq!(
            extract_terminal_pattern_display("cargo test -p search"),
            Some("cargo test".to_string())
        );
        assert_eq!(
            extract_terminal_pattern_display("npm install"),
            Some("npm install".to_string())
        );
        assert_eq!(
            extract_terminal_pattern_display("ls -la"),
            Some("ls".to_string())
        );
        assert_eq!(
            extract_terminal_pattern_display("ls"),
            Some("ls".to_string())
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
