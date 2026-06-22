use language::LineIndent;
use std::{cmp, iter};

#[derive(Copy, Clone, Debug)]
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

/// Synchronous re-indentation adapter. Buffers incomplete lines and applies
/// an `IndentDelta` to each line's leading whitespace before emitting it.
pub struct Reindenter {
    delta: IndentDelta,
    buffer: String,
    in_leading_whitespace: bool,
}

impl Reindenter {
    pub fn new(delta: IndentDelta) -> Self {
        Self {
            delta,
            buffer: String::new(),
            in_leading_whitespace: true,
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

            if self.in_leading_whitespace {
                if let Some(non_whitespace_ix) = line.find(|c| self.delta.character() != c) {
                    // We found a non-whitespace character, adjust indentation
                    // based on the delta.
                    let new_indent_len =
                        cmp::max(0, non_whitespace_ix as isize + self.delta.len()) as usize;
                    indented.extend(iter::repeat(self.delta.character()).take(new_indent_len));
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
        let mut r = Reindenter::new(IndentDelta::Spaces(2));
        let out = r.push("    abc\n  def\n      ghi");
        // All three lines are emitted: "ghi" starts with spaces but
        // contains non-whitespace, so it's processed immediately.
        assert_eq!(out, "      abc\n    def\n        ghi");
        let out = r.finish();
        assert_eq!(out, "");
    }

    #[test]
    fn test_outdent_tabs() {
        let mut r = Reindenter::new(IndentDelta::Tabs(-2));
        let out = r.push("\t\t\t\tabc\n\t\tdef\n\t\t\t\t\t\tghi");
        assert_eq!(out, "\t\tabc\ndef\n\t\t\t\tghi");
        let out = r.finish();
        assert_eq!(out, "");
    }

    #[test]
    fn test_incremental_chunks() {
        let mut r = Reindenter::new(IndentDelta::Spaces(2));
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
        let mut r = Reindenter::new(IndentDelta::Spaces(0));
        let out = r.push("  hello\n  world\n");
        assert_eq!(out, "  hello\n  world\n");
        let out = r.finish();
        assert_eq!(out, "");
    }

    #[test]
    fn test_clamp_negative_indent() {
        let mut r = Reindenter::new(IndentDelta::Spaces(-10));
        let out = r.push("  abc\n");
        // max(0, 2 - 10) = 0, so no leading spaces.
        assert_eq!(out, "abc\n");
        let out = r.finish();
        assert_eq!(out, "");
    }

    #[test]
    fn test_whitespace_only_lines() {
        let mut r = Reindenter::new(IndentDelta::Spaces(2));
        let out = r.push("   \n  code\n");
        // First line is all whitespace — emitted verbatim. Second line is indented.
        assert_eq!(out, "   \n    code\n");
        let out = r.finish();
        assert_eq!(out, "");
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
