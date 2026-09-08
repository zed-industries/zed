use language::LineIndent;
use std::{cmp, iter};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IndentDelta {
    Spaces(isize),
    Tabs(isize),
}

impl IndentDelta {
    pub fn character(&self) -> char {
        match self {
            IndentDelta::Spaces(_) => ' ',
            IndentDelta::Tabs(_) => '\t',
        }
    }

    pub fn len(&self) -> isize {
        match self {
            IndentDelta::Spaces(n) => *n,
            IndentDelta::Tabs(n) => *n,
        }
    }
}

pub fn compute_indent_delta(buffer_indent: LineIndent, query_indent: LineIndent) -> IndentDelta {
    if buffer_indent.tabs > 0 {
        IndentDelta::Tabs(buffer_indent.tabs as isize - query_indent.tabs as isize)
    } else {
        IndentDelta::Spaces(buffer_indent.spaces as isize - query_indent.spaces as isize)
    }
}

/// Computes the indent delta for the lines after the first, given per-line
/// `(buffer, query)` indents for those lines.
///
/// When the remaining lines agree on a consistent delta, that delta is
/// returned even if it differs from `first_line_delta`. This handles queries
/// where only the first line's indentation was stripped. When the remaining
/// lines are inconsistent (or all blank), falls back to `first_line_delta`,
/// preserving the uniform re-indentation behavior.
pub fn compute_rest_indent_delta(
    first_line_delta: IndentDelta,
    indent_pairs: impl IntoIterator<Item = (LineIndent, LineIndent)>,
) -> IndentDelta {
    let mut rest_delta = None;
    for (buffer_indent, query_indent) in indent_pairs {
        if buffer_indent.line_blank || query_indent.line_blank {
            continue;
        }
        let delta = compute_indent_delta(buffer_indent, query_indent);
        match rest_delta {
            None => rest_delta = Some(delta),
            Some(existing) if existing == delta => {}
            Some(_) => return first_line_delta,
        }
    }
    rest_delta.unwrap_or(first_line_delta)
}

/// Synchronous re-indentation adapter. Buffers incomplete lines and applies
/// an `IndentDelta` to each line's leading whitespace before emitting it.
///
/// Models sometimes omit the leading indentation only on the first line of
/// `old_text`/`new_text` (e.g. when copying from mid-line context), so the
/// first line and the remaining lines can require different deltas.
pub struct Reindenter {
    first_line_delta: IndentDelta,
    rest_delta: IndentDelta,
    buffer: String,
    in_leading_whitespace: bool,
    on_first_line: bool,
}

impl Reindenter {
    #[cfg(test)]
    fn uniform(delta: IndentDelta) -> Self {
        Self::with_deltas(delta, delta)
    }

    pub fn with_deltas(first_line_delta: IndentDelta, rest_delta: IndentDelta) -> Self {
        Self {
            first_line_delta,
            rest_delta,
            buffer: String::new(),
            in_leading_whitespace: true,
            on_first_line: true,
        }
    }

    /// Feed a chunk of text and return the re-indented portion that is
    /// ready to emit. Incomplete trailing lines are buffered internally.
    pub fn push(&mut self, chunk: &str) -> String {
        self.buffer.push_str(chunk);
        self.drain(false)
    }

    /// Flush any remaining buffered content (call when the stream is done).
    pub fn finish(&mut self) -> String {
        self.drain(true)
    }

    fn drain(&mut self, is_final: bool) -> String {
        let mut indented = String::new();
        let mut start_ix = 0;
        let mut newlines = self.buffer.match_indices('\n');
        loop {
            let (line_end, is_pending_line) = match newlines.next() {
                Some((ix, _)) => (ix, false),
                None => (self.buffer.len(), true),
            };
            let line = &self.buffer[start_ix..line_end];
            let delta = if self.on_first_line {
                self.first_line_delta
            } else {
                self.rest_delta
            };

            if self.in_leading_whitespace {
                if let Some(non_whitespace_ix) = line.find(|c| delta.character() != c) {
                    // We found a non-whitespace character, adjust indentation
                    // based on the delta.
                    let new_indent_len =
                        cmp::max(0, non_whitespace_ix as isize + delta.len()) as usize;
                    indented.extend(iter::repeat(delta.character()).take(new_indent_len));
                    indented.push_str(&line[non_whitespace_ix..]);
                    self.in_leading_whitespace = false;
                } else if is_pending_line && !is_final {
                    // We're still in leading whitespace and this line is incomplete.
                    // Stop processing until we receive more input.
                    break;
                } else {
                    // This line is entirely whitespace. Push it without indentation.
                    indented.push_str(line);
                }
            } else {
                indented.push_str(line);
            }

            if is_pending_line {
                start_ix = line_end;
                break;
            } else {
                self.in_leading_whitespace = true;
                self.on_first_line = false;
                indented.push('\n');
                start_ix = line_end + 1;
            }
        }
        self.buffer.replace_range(..start_ix, "");
        if is_final {
            indented.push_str(&self.buffer);
            self.buffer.clear();
        }
        indented
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent_single_chunk() {
        let mut r = Reindenter::uniform(IndentDelta::Spaces(2));
        let out = r.push("    abc\n  def\n      ghi");
        // All three lines are emitted: "ghi" starts with spaces but
        // contains non-whitespace, so it's processed immediately.
        assert_eq!(out, "      abc\n    def\n        ghi");
        let out = r.finish();
        assert_eq!(out, "");
    }

    #[test]
    fn test_outdent_tabs() {
        let mut r = Reindenter::uniform(IndentDelta::Tabs(-2));
        let out = r.push("\t\t\t\tabc\n\t\tdef\n\t\t\t\t\t\tghi");
        assert_eq!(out, "\t\tabc\ndef\n\t\t\t\tghi");
        let out = r.finish();
        assert_eq!(out, "");
    }

    #[test]
    fn test_incremental_chunks() {
        let mut r = Reindenter::uniform(IndentDelta::Spaces(2));
        // Feed "    ab" — the `a` is non-whitespace, so the line is
        // processed immediately even without a trailing newline.
        let out = r.push("    ab");
        assert_eq!(out, "      ab");
        // Feed "c\n" — appended to the already-processed line (no longer
        // in leading whitespace).
        let out = r.push("c\n");
        assert_eq!(out, "c\n");
        let out = r.finish();
        assert_eq!(out, "");
    }

    #[test]
    fn test_zero_delta() {
        let mut r = Reindenter::uniform(IndentDelta::Spaces(0));
        let out = r.push("  hello\n  world\n");
        assert_eq!(out, "  hello\n  world\n");
        let out = r.finish();
        assert_eq!(out, "");
    }

    #[test]
    fn test_clamp_negative_indent() {
        let mut r = Reindenter::uniform(IndentDelta::Spaces(-10));
        let out = r.push("  abc\n");
        // max(0, 2 - 10) = 0, so no leading spaces.
        assert_eq!(out, "abc\n");
        let out = r.finish();
        assert_eq!(out, "");
    }

    #[test]
    fn test_whitespace_only_lines() {
        let mut r = Reindenter::uniform(IndentDelta::Spaces(2));
        let out = r.push("   \n  code\n");
        // First line is all whitespace — emitted verbatim. Second line is indented.
        assert_eq!(out, "   \n    code\n");
        let out = r.finish();
        assert_eq!(out, "");
    }

    #[test]
    fn test_distinct_first_line_delta() {
        // First line's indentation was stripped in the query (delta +8),
        // while the remaining lines are already correct (delta 0). Chunks
        // split mid-line and mid-indentation to exercise the streaming path,
        // and the blank line is passed through verbatim.
        let mut r = Reindenter::with_deltas(IndentDelta::Spaces(8), IndentDelta::Spaces(0));
        let mut out = r.push("self.target_a = ");
        out.push_str(&r.push("\"after\"\n    "));
        out.push_str(&r.push("    self.target_b = \"after\"\n"));
        out.push_str(&r.push("\n        self.target_c = \"after\""));
        out.push_str(&r.finish());
        assert_eq!(
            out,
            concat!(
                "        self.target_a = \"after\"\n",
                "        self.target_b = \"after\"\n",
                "\n",
                "        self.target_c = \"after\"",
            )
        );
    }

    fn line_indent(text: &str) -> LineIndent {
        LineIndent::from_iter(text.chars())
    }

    #[test]
    fn test_compute_rest_indent_delta() {
        let first_line_delta = IndentDelta::Spaces(8);

        // Remaining lines that agree on a delta override the first-line
        // delta, and blank lines are skipped when forming the consensus.
        assert_eq!(
            compute_rest_indent_delta(
                first_line_delta,
                vec![
                    (line_indent("        b"), line_indent("        b")),
                    (line_indent(""), line_indent("")),
                    (line_indent("        c"), line_indent("        c")),
                ],
            ),
            IndentDelta::Spaces(0)
        );
        assert_eq!(
            compute_rest_indent_delta(
                first_line_delta,
                vec![
                    (line_indent("        b"), line_indent("    b")),
                    (line_indent("   "), line_indent("")),
                    (line_indent("        c"), line_indent("    c")),
                ],
            ),
            IndentDelta::Spaces(4)
        );
        assert_eq!(
            compute_rest_indent_delta(
                first_line_delta,
                vec![(line_indent("\t\tb"), line_indent("\tb"))],
            ),
            IndentDelta::Tabs(1)
        );

        // Inconsistent remaining lines fall back to the first-line delta...
        assert_eq!(
            compute_rest_indent_delta(
                first_line_delta,
                vec![
                    (line_indent("        b"), line_indent("        b")),
                    (line_indent("        c"), line_indent("    c")),
                ],
            ),
            first_line_delta
        );

        // ...and so do all-blank and empty pairings.
        assert_eq!(
            compute_rest_indent_delta(
                first_line_delta,
                vec![(line_indent("   "), line_indent(""))],
            ),
            first_line_delta
        );
        assert_eq!(
            compute_rest_indent_delta(first_line_delta, vec![]),
            first_line_delta
        );
    }

    #[test]
    fn test_compute_indent_delta_spaces() {
        let buffer = LineIndent {
            tabs: 0,
            spaces: 8,
            line_blank: false,
        };
        let query = LineIndent {
            tabs: 0,
            spaces: 4,
            line_blank: false,
        };
        let delta = compute_indent_delta(buffer, query);
        assert_eq!(delta.len(), 4);
        assert_eq!(delta.character(), ' ');
    }

    #[test]
    fn test_compute_indent_delta_tabs() {
        let buffer = LineIndent {
            tabs: 2,
            spaces: 0,
            line_blank: false,
        };
        let query = LineIndent {
            tabs: 3,
            spaces: 0,
            line_blank: false,
        };
        let delta = compute_indent_delta(buffer, query);
        assert_eq!(delta.len(), -1);
        assert_eq!(delta.character(), '\t');
    }
}
