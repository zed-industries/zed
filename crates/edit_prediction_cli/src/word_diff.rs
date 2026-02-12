//! Word-diff utilities for converting unified diffs to word-diff format.

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

/// Tokenize text into words and whitespace sequences.
fn tokenize(text: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut chars = text.char_indices().peekable();

    while let Some((start, ch)) = chars.next() {
        if ch.is_whitespace() {
            // Collect contiguous whitespace
            let mut end = start + ch.len_utf8();
            while let Some(&(_, next_ch)) = chars.peek() {
                if next_ch.is_whitespace() {
                    end += next_ch.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(&text[start..end]);
        } else {
            // Collect contiguous non-whitespace
            let mut end = start + ch.len_utf8();
            while let Some(&(_, next_ch)) = chars.peek() {
                if !next_ch.is_whitespace() {
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
enum DiffOp {
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

/// Compute diff operations between two token sequences using a simple LCS-based algorithm.
fn diff_tokens<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<DiffOp> {
    // Build LCS table
    let m = old.len();
    let n = new.len();

    if m == 0 && n == 0 {
        return vec![];
    }
    if m == 0 {
        return vec![DiffOp::Insert(0, n)];
    }
    if n == 0 {
        return vec![DiffOp::Delete(0, m)];
    }

    // LCS dynamic programming
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if old[i - 1] == new[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Backtrack to find operations
    let mut ops = Vec::new();
    let mut i = m;
    let mut j = n;

    // We'll collect in reverse order, then reverse at the end
    let mut stack: Vec<(usize, usize, bool)> = Vec::new(); // (index, end, is_old)

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && old[i - 1] == new[j - 1] {
            stack.push((i - 1, i, true)); // Equal marker (using old index)
            stack.push((j - 1, j, false)); // Paired with new index
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || dp[i][j - 1] >= dp[i - 1][j]) {
            // Insert from new
            stack.push((j - 1, j, false));
            j -= 1;
        } else {
            // Delete from old
            stack.push((i - 1, i, true));
            i -= 1;
        }
    }

    // Process the stack to build proper DiffOps
    // This is a simplified approach - just iterate through and build ops
    let mut old_idx = 0;
    let mut new_idx = 0;

    while old_idx < m || new_idx < n {
        // Find next matching pair
        let mut old_match = None;
        let mut new_match = None;

        for oi in old_idx..m {
            for ni in new_idx..n {
                if old[oi] == new[ni] {
                    old_match = Some(oi);
                    new_match = Some(ni);
                    break;
                }
            }
            if old_match.is_some() {
                break;
            }
        }

        match (old_match, new_match) {
            (Some(om), Some(nm)) => {
                // Handle any deletions/insertions before the match
                if old_idx < om && new_idx < nm {
                    ops.push(DiffOp::Replace {
                        old_start: old_idx,
                        old_end: om,
                        new_start: new_idx,
                        new_end: nm,
                    });
                } else if old_idx < om {
                    ops.push(DiffOp::Delete(old_idx, om));
                } else if new_idx < nm {
                    ops.push(DiffOp::Insert(new_idx, nm));
                }

                // Find the extent of the equal sequence
                let mut eq_end_old = om;
                let mut eq_end_new = nm;
                while eq_end_old < m && eq_end_new < n && old[eq_end_old] == new[eq_end_new] {
                    eq_end_old += 1;
                    eq_end_new += 1;
                }

                ops.push(DiffOp::Equal(om, eq_end_old));
                old_idx = eq_end_old;
                new_idx = eq_end_new;
            }
            _ => {
                // No more matches, handle remaining
                if old_idx < m && new_idx < n {
                    ops.push(DiffOp::Replace {
                        old_start: old_idx,
                        old_end: m,
                        new_start: new_idx,
                        new_end: n,
                    });
                } else if old_idx < m {
                    ops.push(DiffOp::Delete(old_idx, m));
                } else if new_idx < n {
                    ops.push(DiffOp::Insert(new_idx, n));
                }
                break;
            }
        }
    }

    ops
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
