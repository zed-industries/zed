//! Word-diff utilities for converting unified diffs to word-diff format.

use similar::{DiffTag, TextDiff};

/// Convert unified diff to word-diff format.
///
/// This transforms line-based diffs into word-level diffs where:
/// - Deletions are marked with `[-...-]`
/// - Insertions are marked with `{+...+}`
pub fn unified_to_word_diff(unified_diff: &str) -> String {
    let lines: Vec<&str> = unified_diff.lines().collect();
    let mut result = String::new();
    let mut old_lines: Vec<&str> = Vec::new();
    let mut new_lines: Vec<&str> = Vec::new();

    let flush_changes =
        |old_lines: &mut Vec<&str>, new_lines: &mut Vec<&str>, result: &mut String| {
            if old_lines.is_empty() && new_lines.is_empty() {
                return;
            }

            // Strip the leading '-' or '+' from each line
            let old_text: String = old_lines
                .iter()
                .map(|line| if line.len() > 1 { &line[1..] } else { "" })
                .collect::<Vec<_>>()
                .join("\n");

            let new_text: String = new_lines
                .iter()
                .map(|line| if line.len() > 1 { &line[1..] } else { "" })
                .collect::<Vec<_>>()
                .join("\n");

            if !old_text.is_empty() || !new_text.is_empty() {
                let word_diff = compute_word_diff(&old_text, &new_text);
                result.push_str(&word_diff);
            }

            old_lines.clear();
            new_lines.clear();
        };

    for line in lines {
        if line.starts_with("---") || line.starts_with("+++") {
            flush_changes(&mut old_lines, &mut new_lines, &mut result);
            result.push_str(line);
            result.push('\n');
        } else if line.starts_with("@@") {
            flush_changes(&mut old_lines, &mut new_lines, &mut result);
            result.push_str(line);
            result.push('\n');
        } else if line.starts_with('-') {
            old_lines.push(line);
        } else if line.starts_with('+') {
            new_lines.push(line);
        } else if line.starts_with(' ') || line.is_empty() {
            flush_changes(&mut old_lines, &mut new_lines, &mut result);
            result.push_str(line);
            result.push('\n');
        } else {
            // Header lines (diff --git, index, etc.)
            flush_changes(&mut old_lines, &mut new_lines, &mut result);
            result.push_str(line);
            result.push('\n');
        }
    }

    flush_changes(&mut old_lines, &mut new_lines, &mut result);
    result
}

/// Compute word-level diff between two text blocks.
///
/// Words and whitespace are treated as separate tokens. The output uses:
/// - `[-...-]` for deleted content
/// - `{+...+}` for inserted content
fn compute_word_diff(old_text: &str, new_text: &str) -> String {
    // Split into words while preserving whitespace
    let old_words = tokenize(old_text);
    let new_words = tokenize(new_text);

    let ops = diff_tokens(&old_words, &new_words);
    let mut result = String::new();

    for op in ops {
        match op {
            DiffOp::Equal(start, end) => {
                for token in &old_words[start..end] {
                    result.push_str(token);
                }
            }
            DiffOp::Delete(start, end) => {
                result.push_str("[-");
                for token in &old_words[start..end] {
                    result.push_str(token);
                }
                result.push_str("-]");
            }
            DiffOp::Insert(start, end) => {
                result.push_str("{+");
                for token in &new_words[start..end] {
                    result.push_str(token);
                }
                result.push_str("+}");
            }
            DiffOp::Replace {
                old_start,
                old_end,
                new_start,
                new_end,
            } => {
                result.push_str("[-");
                for token in &old_words[old_start..old_end] {
                    result.push_str(token);
                }
                result.push_str("-]");
                result.push_str("{+");
                for token in &new_words[new_start..new_end] {
                    result.push_str(token);
                }
                result.push_str("+}");
            }
        }
    }

    if !result.is_empty() && !result.ends_with('\n') {
        result.push('\n');
    }

    result
}

/// Classify a character into one of three token classes:
/// - 0: identifier (alphanumeric or `_`)
/// - 1: whitespace
/// - 2: punctuation (everything else, each character becomes its own token)
fn char_class(ch: char) -> u8 {
    if ch.is_alphanumeric() || ch == '_' {
        0
    } else if ch.is_whitespace() {
        1
    } else {
        2
    }
}

/// Tokenize text into identifier words, whitespace runs, and individual punctuation characters.
///
/// This splitting aligns with the syntactic atoms of source code so that the
/// LCS-based diff can produce fine-grained, meaningful change regions.
pub(crate) fn tokenize(text: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut chars = text.char_indices().peekable();

    while let Some((start, ch)) = chars.next() {
        let class = char_class(ch);
        if class == 2 {
            // Punctuation: each character is a separate token
            tokens.push(&text[start..start + ch.len_utf8()]);
        } else {
            // Identifier or whitespace: collect contiguous run of same class
            let mut end = start + ch.len_utf8();
            while let Some(&(_, next_ch)) = chars.peek() {
                if char_class(next_ch) == class {
                    end += next_ch.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(&text[start..end]);
        }
    }

    tokens
}

#[derive(Debug)]
pub(crate) enum DiffOp {
    Equal(usize, usize),
    Delete(usize, usize),
    Insert(usize, usize),
    Replace {
        old_start: usize,
        old_end: usize,
        new_start: usize,
        new_end: usize,
    },
}

/// Compute diff operations between two token sequences using `similar`'s Myers diff.
pub(crate) fn diff_tokens<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<DiffOp> {
    let diff = TextDiff::from_slices(old, new);
    diff.ops()
        .iter()
        .map(|op| {
            let tag = op.tag();
            let old_range = op.old_range();
            let new_range = op.new_range();
            match tag {
                DiffTag::Equal => DiffOp::Equal(old_range.start, old_range.end),
                DiffTag::Delete => DiffOp::Delete(old_range.start, old_range.end),
                DiffTag::Insert => DiffOp::Insert(new_range.start, new_range.end),
                DiffTag::Replace => DiffOp::Replace {
                    old_start: old_range.start,
                    old_end: old_range.end,
                    new_start: new_range.start,
                    new_end: new_range.end,
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("hello world");
        assert_eq!(tokens, vec!["hello", " ", "world"]);

        let tokens = tokenize("  multiple   spaces  ");
        assert_eq!(tokens, vec!["  ", "multiple", "   ", "spaces", "  "]);

        let tokens = tokenize("self.name");
        assert_eq!(tokens, vec!["self", ".", "name"]);

        let tokens = tokenize("foo(bar, baz)");
        assert_eq!(tokens, vec!["foo", "(", "bar", ",", " ", "baz", ")"]);

        let tokens = tokenize("hello_world");
        assert_eq!(tokens, vec!["hello_world"]);

        let tokens = tokenize("fn();");
        assert_eq!(tokens, vec!["fn", "(", ")", ";"]);

        let tokens = tokenize("foo_bar123 + baz");
        assert_eq!(tokens, vec!["foo_bar123", " ", "+", " ", "baz"]);

        let tokens = tokenize("print(\"hello\")");
        assert_eq!(tokens, vec!["print", "(", "\"", "hello", "\"", ")"]);
    }

    #[test]
    fn test_compute_word_diff_simple() {
        let result = compute_word_diff("hello world", "hello there");
        assert!(result.contains("[-world-]"));
        assert!(result.contains("{+there+}"));
    }

    #[test]
    fn test_unified_to_word_diff() {
        let unified = "\
--- a/file.txt
+++ b/file.txt
@@ -1,3 +1,3 @@
 context line
-old text here
+new text here
 more context";

        let result = unified_to_word_diff(unified);
        assert!(result.contains("--- a/file.txt"));
        assert!(result.contains("+++ b/file.txt"));
        assert!(result.contains("@@"));
    }
}
