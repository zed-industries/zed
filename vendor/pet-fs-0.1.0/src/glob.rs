// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use glob::glob;
use std::path::PathBuf;

/// Characters that indicate a path contains glob pattern metacharacters.
const GLOB_METACHARACTERS: &[char] = &['*', '?', '[', ']'];

/// Checks whether a path string contains glob metacharacters or brace expansion.
///
/// # Examples
/// - `"/home/user/*"` → `true`
/// - `"/home/user/envs"` → `false`
/// - `"**/*.py"` → `true`
/// - `"/home/user/[abc]"` → `true`
/// - `"./**/{bin,Scripts}/python"` → `true`
pub fn is_glob_pattern(path: &str) -> bool {
    path.contains(GLOB_METACHARACTERS) || has_brace_pattern(path)
}

/// Returns true when a glob can traverse an unbounded number of path components.
pub fn is_recursive_glob_pattern(path: &str) -> bool {
    expand_braces(path)
        .iter()
        .any(|pattern| pattern.split(['/', '\\']).any(|segment| segment == "**"))
}

/// Checks if a string contains a valid brace expansion pattern `{a,b}`.
/// Requires an opening `{`, at least one `,`, and a closing `}`.
fn has_brace_pattern(path: &str) -> bool {
    let mut remaining = path;
    while let Some(open) = remaining.find('{') {
        let after_open = &remaining[open..];
        if let Some(close_offset) = after_open.find('}') {
            if after_open[..close_offset].contains(',') {
                return true;
            }
            remaining = &after_open[close_offset + 1..];
        } else {
            break;
        }
    }
    false
}

/// Maximum number of patterns produced by brace expansion.
/// Guards against exponential blowup from deeply nested or many brace groups.
const MAX_BRACE_EXPANSIONS: usize = 1024;

/// Expands brace expressions in a pattern string.
///
/// Handles patterns like `{a,b}` which expand to multiple strings.
/// Supports multiple brace groups and empty alternatives (e.g., `{,.exe}`).
/// Nested braces are not supported.
/// Expansion is capped at [`MAX_BRACE_EXPANSIONS`] patterns.
///
/// # Examples
/// - `"{bin,Scripts}/python"` → `["bin/python", "Scripts/python"]`
/// - `"python{,.exe}"` → `["python", "python.exe"]`
/// - `"{a,b}/{c,d}"` → `["a/c", "a/d", "b/c", "b/d"]`
fn expand_braces(pattern: &str) -> Vec<String> {
    let mut results = Vec::new();
    expand_braces_inner(pattern, &mut results);
    results
}

fn expand_braces_inner(pattern: &str, results: &mut Vec<String>) {
    if results.len() >= MAX_BRACE_EXPANSIONS {
        return;
    }
    // Find the first '{' and its matching '}'
    let Some(open) = pattern.find('{') else {
        results.push(pattern.to_string());
        return;
    };
    let Some(close) = pattern[open..].find('}') else {
        // Unmatched brace, return as-is
        results.push(pattern.to_string());
        return;
    };
    let close = open + close;

    let prefix = &pattern[..open];
    let suffix = &pattern[close + 1..];
    let alternatives = &pattern[open + 1..close];

    for alt in alternatives.split(',') {
        if results.len() >= MAX_BRACE_EXPANSIONS {
            log::warn!(
                "Brace expansion exceeded {} patterns for '{}', truncating",
                MAX_BRACE_EXPANSIONS,
                pattern
            );
            return;
        }
        let expanded = format!("{prefix}{alt}{suffix}");
        // Recursively expand any remaining brace groups
        expand_braces_inner(&expanded, results);
    }
}

/// Expands a single glob pattern to matching paths.
///
/// Supports brace expansion (e.g., `{bin,Scripts}`) in addition to standard
/// glob metacharacters (`*`, `?`, `[...]`). Brace groups are expanded first,
/// then each resulting pattern is matched against the filesystem.
///
/// If the path does not contain glob metacharacters or braces, returns it
/// unchanged (to let downstream code handle non-existent paths).
///
/// # Examples
/// - `"/home/user/envs"` → `["/home/user/envs"]`
/// - `"/home/user/*/venv"` → `["/home/user/project1/venv", "/home/user/project2/venv"]`
/// - `"**/.venv"` → All `.venv` directories recursively
/// - `"./**/{bin,Scripts}/python"` → Python executables in bin or Scripts dirs
pub fn expand_glob_pattern(pattern: &str) -> Vec<PathBuf> {
    if !is_glob_pattern(pattern) {
        // Not a glob pattern, return as-is
        return vec![PathBuf::from(pattern)];
    }

    // Expand brace groups first, then glob each resulting pattern
    let patterns = expand_braces(pattern);
    let mut result = Vec::new();
    for pat in &patterns {
        if !pat.contains(GLOB_METACHARACTERS) {
            // After brace expansion this variant has no glob metacharacters;
            // return as a literal path (same behavior as a non-glob input).
            result.push(PathBuf::from(pat));
            continue;
        }
        log::trace!("Expanding glob pattern '{}'", pat);
        let start = std::time::Instant::now();
        match glob(pat) {
            Ok(paths) => {
                let mut count: usize = 0;
                for entry in paths {
                    match entry {
                        Ok(path) => {
                            count += 1;
                            if count.is_multiple_of(100) {
                                log::trace!(
                                    "Glob '{}': found {} matches so far ({:?} elapsed)",
                                    pat,
                                    count,
                                    start.elapsed()
                                );
                            }
                            result.push(path);
                        }
                        Err(e) => {
                            log::debug!("Failed to read glob entry: {}", e);
                        }
                    }
                }
                log::trace!(
                    "Glob '{}': completed with {} matches in {:?}",
                    pat,
                    count,
                    start.elapsed()
                );
            }
            Err(e) => {
                log::warn!("Invalid glob pattern '{}': {}", pat, e);
            }
        }
    }
    if result.is_empty() {
        log::debug!("Glob pattern '{}' matched no paths", pattern);
    }
    result
}

/// Expands a list of paths, where each path may be a glob pattern.
///
/// Non-glob paths are passed through as-is.
/// Glob patterns are expanded to all matching paths.
/// Duplicate paths are preserved (caller should deduplicate if needed).
///
/// # Examples
/// ```ignore
/// let paths = vec![
///     PathBuf::from("/home/user/project"),
///     PathBuf::from("/home/user/*/venv"),
/// ];
/// let expanded = expand_glob_patterns(&paths);
/// // expanded contains "/home/user/project" plus all matching venv dirs
/// ```
pub fn expand_glob_patterns(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut result = Vec::new();
    for path in paths {
        let path_str = path.to_string_lossy();
        let expanded = expand_glob_pattern(&path_str);
        result.extend(expanded);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_glob_pattern_with_asterisk() {
        assert!(is_glob_pattern("/home/user/*"));
        assert!(is_glob_pattern("**/*.py"));
        assert!(is_glob_pattern("*.txt"));
    }

    #[test]
    fn test_is_recursive_glob_pattern() {
        assert!(is_recursive_glob_pattern("**/.venv"));
        assert!(is_recursive_glob_pattern("/home/user/**/venv"));
        assert!(!is_recursive_glob_pattern(".venv"));
        assert!(!is_recursive_glob_pattern("*/.venv"));
        assert!(!is_recursive_glob_pattern("foo**bar/.venv"));
        assert!(is_recursive_glob_pattern("C:\\workspace\\**\\.venv"));
        assert!(is_recursive_glob_pattern("{foo,**}/.venv"));
    }
    #[test]
    fn test_is_glob_pattern_with_question_mark() {
        assert!(is_glob_pattern("/home/user/file?.txt"));
        assert!(is_glob_pattern("test?"));
    }

    #[test]
    fn test_is_glob_pattern_with_brackets() {
        assert!(is_glob_pattern("/home/user/[abc]"));
        assert!(is_glob_pattern("file[0-9].txt"));
    }

    #[test]
    fn test_is_glob_pattern_no_metacharacters() {
        assert!(!is_glob_pattern("/home/user/envs"));
        assert!(!is_glob_pattern("simple_path"));
        assert!(!is_glob_pattern("/usr/local/bin/python3"));
    }

    #[test]
    fn test_expand_non_glob_path() {
        let path = "/some/literal/path";
        let result = expand_glob_pattern(path);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], PathBuf::from(path));
    }

    #[test]
    fn test_expand_glob_pattern_no_matches() {
        let pattern = "/this/path/definitely/does/not/exist/*";
        let result = expand_glob_pattern(pattern);
        assert!(result.is_empty());
    }

    #[test]
    fn test_expand_glob_pattern_with_matches() {
        // Create temp directories for testing
        let temp_dir = std::env::temp_dir().join("pet_glob_test");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(temp_dir.join("project1")).unwrap();
        fs::create_dir_all(temp_dir.join("project2")).unwrap();
        fs::create_dir_all(temp_dir.join("other")).unwrap();

        let pattern = format!("{}/project*", temp_dir.to_string_lossy());
        let result = expand_glob_pattern(&pattern);

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|p| p.ends_with("project1")));
        assert!(result.iter().any(|p| p.ends_with("project2")));
        assert!(!result.iter().any(|p| p.ends_with("other")));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_expand_glob_patterns_mixed() {
        let temp_dir = std::env::temp_dir().join("pet_glob_test_mixed");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(temp_dir.join("dir1")).unwrap();
        fs::create_dir_all(temp_dir.join("dir2")).unwrap();

        let paths = vec![
            PathBuf::from("/literal/path"),
            PathBuf::from(format!("{}/dir*", temp_dir.to_string_lossy())),
        ];

        let result = expand_glob_patterns(&paths);

        // Should have literal path + 2 expanded directories
        assert_eq!(result.len(), 3);
        assert!(result.contains(&PathBuf::from("/literal/path")));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_expand_glob_pattern_recursive() {
        // Create nested temp directories for testing **
        let temp_dir = std::env::temp_dir().join("pet_glob_test_recursive");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(temp_dir.join("a/b/.venv")).unwrap();
        fs::create_dir_all(temp_dir.join("c/.venv")).unwrap();
        fs::create_dir_all(temp_dir.join(".venv")).unwrap();

        let pattern = format!("{}/**/.venv", temp_dir.to_string_lossy());
        let result = expand_glob_pattern(&pattern);

        // Should find .venv at multiple levels (behavior depends on glob crate version)
        assert!(!result.is_empty());
        assert!(result.iter().all(|p| p.ends_with(".venv")));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_expand_glob_pattern_filename_patterns() {
        // Create temp files for testing filename patterns like python_* and python.*
        let temp_dir = std::env::temp_dir().join("pet_glob_test_filenames");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // Create files matching python_* pattern
        fs::write(temp_dir.join("python_foo"), "").unwrap();
        fs::write(temp_dir.join("python_bar"), "").unwrap();
        fs::write(temp_dir.join("python_3.12"), "").unwrap();
        fs::write(temp_dir.join("other_file"), "").unwrap();

        // Test python_* pattern
        let pattern = format!("{}/python_*", temp_dir.to_string_lossy());
        let result = expand_glob_pattern(&pattern);

        assert_eq!(result.len(), 3);
        assert!(result.iter().any(|p| p.ends_with("python_foo")));
        assert!(result.iter().any(|p| p.ends_with("python_bar")));
        assert!(result.iter().any(|p| p.ends_with("python_3.12")));
        assert!(!result.iter().any(|p| p.ends_with("other_file")));

        // Create files matching python.* pattern
        fs::write(temp_dir.join("python.exe"), "").unwrap();
        fs::write(temp_dir.join("python.sh"), "").unwrap();
        fs::write(temp_dir.join("pythonrc"), "").unwrap();

        // Test python.* pattern
        let pattern = format!("{}/python.*", temp_dir.to_string_lossy());
        let result = expand_glob_pattern(&pattern);

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|p| p.ends_with("python.exe")));
        assert!(result.iter().any(|p| p.ends_with("python.sh")));
        assert!(!result.iter().any(|p| p.ends_with("pythonrc")));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_expand_braces_no_braces() {
        assert_eq!(expand_braces("no_braces"), vec!["no_braces"]);
        assert_eq!(expand_braces("/usr/bin/python"), vec!["/usr/bin/python"]);
    }

    #[test]
    fn test_expand_braces_single_group() {
        let mut result = expand_braces("{bin,Scripts}/python");
        result.sort();
        assert_eq!(result, vec!["Scripts/python", "bin/python"]);
    }

    #[test]
    fn test_expand_braces_empty_alternative() {
        let mut result = expand_braces("python{,.exe}");
        result.sort();
        assert_eq!(result, vec!["python", "python.exe"]);
    }

    #[test]
    fn test_expand_braces_multiple_groups() {
        let mut result = expand_braces("{a,b}/{c,d}");
        result.sort();
        assert_eq!(result, vec!["a/c", "a/d", "b/c", "b/d"]);
    }

    #[test]
    fn test_expand_braces_unmatched_brace() {
        assert_eq!(expand_braces("{unmatched"), vec!["{unmatched"]);
    }

    #[test]
    fn test_expand_braces_real_pattern() {
        let mut result = expand_braces("./**/{bin,Scripts}/python{,.exe}");
        result.sort();
        assert_eq!(
            result,
            vec![
                "./**/Scripts/python",
                "./**/Scripts/python.exe",
                "./**/bin/python",
                "./**/bin/python.exe",
            ]
        );
    }

    #[test]
    fn test_is_glob_pattern_with_braces() {
        assert!(is_glob_pattern("{bin,Scripts}/python"));
        assert!(is_glob_pattern("python{,.exe}"));
        assert!(is_glob_pattern("./**/{bin,Scripts}/python{,.exe}"));
    }

    #[test]
    fn test_is_glob_pattern_lone_brace_not_detected() {
        // A lone `{` without matching `}` or without comma is not a brace pattern
        assert!(!is_glob_pattern("/home/user/my{project"));
        assert!(!is_glob_pattern("{nocomma}"));
        // But a proper `{a,b}` pair is detected even without glob metacharacters
        assert!(is_glob_pattern("{a,b}"));
    }

    #[test]
    fn test_is_glob_pattern_brace_after_non_brace() {
        // A valid brace group after a non-expansion group should still be detected
        assert!(is_glob_pattern("{nocomma}/path/{a,b}"));
    }

    #[test]
    fn test_expand_braces_empty_braces() {
        // `{}` has no comma, so expand_braces treats it as a single empty alternative
        assert_eq!(expand_braces("prefix{}suffix"), vec!["prefixsuffix"]);
    }

    #[test]
    fn test_expand_braces_single_alternative() {
        assert_eq!(expand_braces("{a}"), vec!["a"]);
    }

    #[test]
    fn test_expand_braces_capped() {
        // Build a pattern that would produce 2^20 = 1M+ expansions without cap
        let pattern = "{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}";
        let result = expand_braces(pattern);
        assert_eq!(result.len(), MAX_BRACE_EXPANSIONS);
    }

    #[test]
    fn test_expand_glob_pattern_with_braces() {
        // Create temp directories with bin and Scripts subdirs
        let temp_dir = std::env::temp_dir().join("pet_glob_test_braces");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(temp_dir.join("env1/bin")).unwrap();
        fs::create_dir_all(temp_dir.join("env2/Scripts")).unwrap();
        fs::write(temp_dir.join("env1/bin/python"), "").unwrap();
        fs::write(temp_dir.join("env2/Scripts/python.exe"), "").unwrap();

        let pattern = format!(
            "{}/**/{{bin,Scripts}}/python{{,.exe}}",
            temp_dir.to_string_lossy()
        );
        let result = expand_glob_pattern(&pattern);

        assert_eq!(result.len(), 2);
        assert!(result
            .iter()
            .any(|p| p.ends_with("bin/python") || p.ends_with("bin\\python")));
        assert!(result
            .iter()
            .any(|p| p.ends_with("Scripts/python.exe") || p.ends_with("Scripts\\python.exe")));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    // ── expand_braces: additional edge cases ──

    #[test]
    fn test_expand_braces_three_alternatives() {
        let mut result = expand_braces("{a,b,c}");
        result.sort();
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_expand_braces_preserves_surrounding_text() {
        let mut result = expand_braces("prefix/{a,b}/suffix");
        result.sort();
        assert_eq!(result, vec!["prefix/a/suffix", "prefix/b/suffix"]);
    }

    #[test]
    fn test_expand_braces_adjacent_groups() {
        // Two brace groups with no separator: {a,b}{c,d} → ac, ad, bc, bd
        let mut result = expand_braces("{a,b}{c,d}");
        result.sort();
        assert_eq!(result, vec!["ac", "ad", "bc", "bd"]);
    }

    #[test]
    fn test_expand_braces_with_dots_and_extensions() {
        let mut result = expand_braces("file{.txt,.md,.rs}");
        result.sort();
        assert_eq!(result, vec!["file.md", "file.rs", "file.txt"]);
    }

    #[test]
    fn test_expand_braces_empty_in_middle() {
        // {a,,b} should produce "a", "", "b" (prefix/suffix applied)
        let mut result = expand_braces("x{a,,b}y");
        result.sort();
        assert_eq!(result, vec!["xay", "xby", "xy"]);
    }

    #[test]
    fn test_expand_braces_single_char_alternatives() {
        let mut result = expand_braces("{x,y,z}");
        result.sort();
        assert_eq!(result, vec!["x", "y", "z"]);
    }

    #[test]
    fn test_expand_braces_path_separators() {
        let mut result = expand_braces("/home/{user1,user2}/.local/bin");
        result.sort();
        assert_eq!(
            result,
            vec!["/home/user1/.local/bin", "/home/user2/.local/bin",]
        );
    }

    #[test]
    fn test_expand_braces_windows_style_paths() {
        let mut result = expand_braces("C:\\envs\\{venv1,venv2}\\{Scripts,bin}\\python.exe");
        result.sort();
        assert_eq!(
            result,
            vec![
                "C:\\envs\\venv1\\Scripts\\python.exe",
                "C:\\envs\\venv1\\bin\\python.exe",
                "C:\\envs\\venv2\\Scripts\\python.exe",
                "C:\\envs\\venv2\\bin\\python.exe",
            ]
        );
    }

    #[test]
    fn test_expand_braces_only_empty_alternatives() {
        // {,} should produce two empty strings → prefix+suffix twice
        let result = expand_braces("a{,}b");
        assert_eq!(result, vec!["ab", "ab"]);
    }

    #[test]
    fn test_expand_braces_mixed_with_glob_chars() {
        // Braces with glob metacharacters inside alternatives
        let mut result = expand_braces("{*.py,*.rs}");
        result.sort();
        assert_eq!(result, vec!["*.py", "*.rs"]);
    }

    // ── is_glob_pattern: additional edge cases ──

    #[test]
    fn test_is_glob_not_glob_empty_string() {
        assert!(!is_glob_pattern(""));
    }

    #[test]
    fn test_is_glob_brace_no_close() {
        assert!(!is_glob_pattern("path/{open,but,no,close"));
    }

    #[test]
    fn test_is_glob_close_before_open() {
        // Stray `}` before any `{` — no valid brace pattern at all
        assert!(!is_glob_pattern("path}/no/braces"));
        // But a stray `}` followed by a valid `{a,b}` IS a brace pattern
        assert!(is_glob_pattern("path}/then/{a,b}"));
    }

    #[test]
    fn test_is_glob_multiple_groups_only_second_valid() {
        assert!(is_glob_pattern("{single}/{a,b}"));
    }

    // ── expand_braces: cap behavior ──

    #[test]
    fn test_expand_braces_cap_stops_at_limit() {
        // 3^7 = 2187 > 1024, should be capped
        let pattern = "{a,b,c}/{a,b,c}/{a,b,c}/{a,b,c}/{a,b,c}/{a,b,c}/{a,b,c}";
        let result = expand_braces(pattern);
        assert_eq!(result.len(), MAX_BRACE_EXPANSIONS);
        // All results should be valid path-like strings
        assert!(result.iter().all(|s| s.contains('/')));
    }

    #[test]
    fn test_expand_braces_just_under_cap() {
        // 2^10 = 1024, exactly at the cap
        let pattern = "{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}";
        let result = expand_braces(pattern);
        assert_eq!(result.len(), MAX_BRACE_EXPANSIONS);
    }

    #[test]
    fn test_expand_braces_well_under_cap() {
        // 2^3 = 8, well under cap
        let pattern = "{a,b}/{a,b}/{a,b}";
        let result = expand_braces(pattern);
        assert_eq!(result.len(), 8);
    }

    // ── Filesystem: brace expansion + glob integration ──

    #[test]
    fn test_expand_glob_braces_with_nested_dirs() {
        let temp_dir = std::env::temp_dir().join("pet_glob_test_nested_braces");
        let _ = fs::remove_dir_all(&temp_dir);

        // Simulate a workspace with multiple envs, each having bin or Scripts
        fs::create_dir_all(temp_dir.join("proj1/.venv/bin")).unwrap();
        fs::create_dir_all(temp_dir.join("proj2/.venv/Scripts")).unwrap();
        fs::create_dir_all(temp_dir.join("proj3/.conda/bin")).unwrap();
        fs::write(temp_dir.join("proj1/.venv/bin/python"), "").unwrap();
        fs::write(temp_dir.join("proj2/.venv/Scripts/python.exe"), "").unwrap();
        fs::write(temp_dir.join("proj3/.conda/bin/python"), "").unwrap();
        // Decoy file that should NOT match
        fs::write(temp_dir.join("proj3/.conda/bin/pip"), "").unwrap();

        let pattern = format!(
            "{}/**/{{bin,Scripts}}/python{{,.exe}}",
            temp_dir.to_string_lossy()
        );
        let result = expand_glob_pattern(&pattern);

        assert_eq!(
            result.len(),
            3,
            "Expected 3 python executables, got: {:?}",
            result
        );

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_expand_glob_braces_no_matching_alternative() {
        let temp_dir = std::env::temp_dir().join("pet_glob_test_braces_nomatch");
        let _ = fs::remove_dir_all(&temp_dir);

        // Only create bin, not Scripts
        fs::create_dir_all(temp_dir.join("env/bin")).unwrap();
        fs::write(temp_dir.join("env/bin/python"), "").unwrap();

        let pattern = format!("{}/**/{{bin,Scripts}}/python", temp_dir.to_string_lossy());
        let result = expand_glob_pattern(&pattern);

        // Only bin/python should match, Scripts/python shouldn't exist
        assert_eq!(result.len(), 1);
        assert!(result[0].ends_with("python"));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_expand_glob_braces_empty_alternative_finds_both() {
        let temp_dir = std::env::temp_dir().join("pet_glob_test_braces_empty_alt");
        let _ = fs::remove_dir_all(&temp_dir);

        fs::create_dir_all(temp_dir.join("bin")).unwrap();
        fs::write(temp_dir.join("bin/python"), "").unwrap();
        fs::write(temp_dir.join("bin/python.exe"), "").unwrap();

        let pattern = format!("{}/bin/python{{,.exe}}", temp_dir.to_string_lossy());
        let result = expand_glob_pattern(&pattern);

        assert_eq!(
            result.len(),
            2,
            "Expected both python and python.exe, got: {:?}",
            result
        );

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_expand_glob_brace_only_pattern_returns_literals() {
        // A brace-only pattern (no glob metacharacters after expansion)
        // should return literal paths without filesystem validation
        let result = expand_glob_pattern("{python,python3}");
        assert_eq!(result.len(), 2);
        assert!(result.contains(&PathBuf::from("python")));
        assert!(result.contains(&PathBuf::from("python3")));
    }

    #[test]
    fn test_expand_glob_patterns_with_braces_in_list() {
        let temp_dir = std::env::temp_dir().join("pet_glob_test_patterns_list");
        let _ = fs::remove_dir_all(&temp_dir);

        fs::create_dir_all(temp_dir.join("a/bin")).unwrap();
        fs::write(temp_dir.join("a/bin/python"), "").unwrap();

        let paths = vec![
            PathBuf::from("/literal/path"),
            PathBuf::from(format!(
                "{}/**/{{bin,Scripts}}/python",
                temp_dir.to_string_lossy()
            )),
        ];
        let result = expand_glob_patterns(&paths);

        // literal + 1 glob match
        assert_eq!(result.len(), 2);
        assert!(result.contains(&PathBuf::from("/literal/path")));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    // ── Performance tests ──

    #[test]
    fn test_expand_braces_performance_many_alternatives() {
        // 100 alternatives in a single group — should be instant
        let alts: Vec<String> = (0..100).map(|i| format!("alt{i}")).collect();
        let pattern = format!("{{{}}}", alts.join(","));

        let start = std::time::Instant::now();
        let result = expand_braces(&pattern);
        let elapsed = start.elapsed();

        assert_eq!(result.len(), 100);
        assert!(
            elapsed.as_millis() < 100,
            "Expanding 100 alternatives took {:?}, expected < 100ms",
            elapsed
        );
    }

    #[test]
    fn test_expand_braces_performance_multiple_groups() {
        // 4 groups of 4 alternatives = 256 patterns
        let pattern = "{a,b,c,d}/{e,f,g,h}/{i,j,k,l}/{m,n,o,p}";

        let start = std::time::Instant::now();
        let result = expand_braces(pattern);
        let elapsed = start.elapsed();

        assert_eq!(result.len(), 256);
        assert!(
            elapsed.as_millis() < 100,
            "Expanding 4x4 groups (256 patterns) took {:?}, expected < 100ms",
            elapsed
        );
    }

    #[test]
    fn test_expand_braces_performance_cap_is_fast() {
        // Pattern that would produce 2^20 = 1M+ expansions without the cap.
        // The cap should make this complete quickly.
        let pattern = "{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}/{a,b}";

        let start = std::time::Instant::now();
        let result = expand_braces(pattern);
        let elapsed = start.elapsed();

        assert_eq!(result.len(), MAX_BRACE_EXPANSIONS);
        assert!(
            elapsed.as_millis() < 100,
            "Capped expansion (2^20 input) took {:?}, expected < 100ms",
            elapsed
        );
    }

    #[test]
    fn test_expand_glob_performance_braces_with_filesystem() {
        // Create a moderately deep directory tree and time glob with braces
        let temp_dir = std::env::temp_dir().join("pet_glob_test_perf");
        let _ = fs::remove_dir_all(&temp_dir);

        // Create 50 project dirs, each with bin/python and Scripts/python.exe
        for i in 0..50 {
            let proj = temp_dir.join(format!("project{i}/.venv"));
            fs::create_dir_all(proj.join("bin")).unwrap();
            fs::create_dir_all(proj.join("Scripts")).unwrap();
            fs::write(proj.join("bin/python"), "").unwrap();
            fs::write(proj.join("Scripts/python.exe"), "").unwrap();
        }

        let pattern = format!(
            "{}/**/{{bin,Scripts}}/python{{,.exe}}",
            temp_dir.to_string_lossy()
        );

        let start = std::time::Instant::now();
        let result = expand_glob_pattern(&pattern);
        let elapsed = start.elapsed();

        // Each project has bin/python + Scripts/python.exe = 2, * 50 projects = 100
        assert_eq!(
            result.len(),
            100,
            "Expected 100 matches, got {}: {:?}",
            result.len(),
            result
        );
        assert!(
            elapsed.as_secs() < 5,
            "Glob with braces over 50 projects took {:?}, expected < 5s",
            elapsed
        );

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_expand_glob_performance_no_braces_comparison() {
        // Same structure as above but using plain glob (no braces) for comparison
        let temp_dir = std::env::temp_dir().join("pet_glob_test_perf_no_braces");
        let _ = fs::remove_dir_all(&temp_dir);

        for i in 0..50 {
            let proj = temp_dir.join(format!("project{i}/.venv/bin"));
            fs::create_dir_all(&proj).unwrap();
            fs::write(proj.join("python"), "").unwrap();
        }

        let pattern = format!("{}/**/bin/python", temp_dir.to_string_lossy());

        let start = std::time::Instant::now();
        let result = expand_glob_pattern(&pattern);
        let elapsed = start.elapsed();

        assert_eq!(result.len(), 50);
        assert!(
            elapsed.as_secs() < 5,
            "Plain glob over 50 projects took {:?}, expected < 5s",
            elapsed
        );

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
