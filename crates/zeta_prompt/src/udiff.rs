use std::{
    borrow::Cow,
    fmt::{Display, Write},
    mem,
    ops::Range,
};

use anyhow::{Context as _, Result, anyhow};

pub fn strip_diff_path_prefix<'a>(diff: &'a str, prefix: &str) -> Cow<'a, str> {
    if prefix.is_empty() {
        return Cow::Borrowed(diff);
    }

    let prefix_with_slash = format!("{}/", prefix);
    let mut needs_rewrite = false;

    for line in diff.lines() {
        match DiffLine::parse(line) {
            DiffLine::OldPath { path } | DiffLine::NewPath { path } => {
                if path.starts_with(&prefix_with_slash) {
                    needs_rewrite = true;
                    break;
                }
            }
            _ => {}
        }
    }

    if !needs_rewrite {
        return Cow::Borrowed(diff);
    }

    let mut result = String::with_capacity(diff.len());
    for line in diff.lines() {
        match DiffLine::parse(line) {
            DiffLine::OldPath { path } => {
                let stripped = path
                    .strip_prefix(&prefix_with_slash)
                    .unwrap_or(path.as_ref());
                result.push_str(&format!("--- a/{}\n", stripped));
            }
            DiffLine::NewPath { path } => {
                let stripped = path
                    .strip_prefix(&prefix_with_slash)
                    .unwrap_or(path.as_ref());
                result.push_str(&format!("+++ b/{}\n", stripped));
            }
            _ => {
                result.push_str(line);
                result.push('\n');
            }
        }
    }

    Cow::Owned(result)
}

/// Strip unnecessary git metadata lines from a diff, keeping only the lines
/// needed for patch application: path headers (--- and +++), hunk headers (@@),
/// and content lines (+, -, space).
pub fn strip_diff_metadata(diff: &str) -> String {
    let mut result = String::new();

    for line in diff.lines() {
        let dominated = DiffLine::parse(line);
        match dominated {
            // Keep path headers, hunk headers, and content lines
            DiffLine::OldPath { .. }
            | DiffLine::NewPath { .. }
            | DiffLine::HunkHeader(_)
            | DiffLine::Context(_)
            | DiffLine::Deletion(_)
            | DiffLine::Addition(_)
            | DiffLine::NoNewlineAtEOF => {
                result.push_str(line);
                result.push('\n');
            }
            // Skip garbage lines (diff --git, index, etc.)
            DiffLine::Garbage(_) => {}
        }
    }

    result
}

/// Marker used to encode cursor position in patch comment lines.
pub const CURSOR_POSITION_MARKER: &str = "[CURSOR_POSITION]";

/// Extract cursor offset from a patch and return `(clean_patch, cursor_offset)`.
///
/// Cursor position is encoded as a comment line (starting with `#`) containing
/// `[CURSOR_POSITION]`. A `^` in the line indicates the cursor column; a `<`
/// indicates column 0. The offset is computed relative to addition (`+`) and
/// context (` `) lines accumulated so far in the hunk, which represent the
/// cursor position within the new text contributed by the hunk.
pub fn extract_cursor_from_patch(patch: &str) -> (String, Option<usize>) {
    let mut clean_patch = String::new();
    let mut cursor_offset: Option<usize> = None;
    let mut line_start_offset = 0usize;
    let mut prev_line_start_offset = 0usize;

    for line in patch.lines() {
        let diff_line = DiffLine::parse(line);

        match &diff_line {
            DiffLine::Garbage(content)
                if content.starts_with('#') && content.contains(CURSOR_POSITION_MARKER) =>
            {
                let caret_column = if let Some(caret_pos) = content.find('^') {
                    caret_pos
                } else if content.find('<').is_some() {
                    0
                } else {
                    continue;
                };
                let cursor_column = caret_column.saturating_sub('#'.len_utf8());
                cursor_offset = Some(prev_line_start_offset + cursor_column);
            }
            _ => {
                if !clean_patch.is_empty() {
                    clean_patch.push('\n');
                }
                clean_patch.push_str(line);

                match diff_line {
                    DiffLine::Addition(content) | DiffLine::Context(content) => {
                        prev_line_start_offset = line_start_offset;
                        line_start_offset += content.len() + 1;
                    }
                    _ => {}
                }
            }
        }
    }

    if patch.ends_with('\n') && !clean_patch.is_empty() {
        clean_patch.push('\n');
    }

    (clean_patch, cursor_offset)
}

/// Find all byte offsets where `hunk.context` occurs as a substring of `text`.
///
/// If no exact matches are found and the context ends with `'\n'` but `text`
/// does not, retries without the trailing newline, accepting only a match at
/// the very end of `text`. When this fallback fires, the hunk's context is
/// trimmed and its edit ranges are clamped so that downstream code doesn't
/// index past the end of the matched region. This handles diffs that are
/// missing a `\ No newline at end of file` marker: the parser always appends
/// `'\n'` via `writeln!`, so the context can have a trailing newline that
/// doesn't exist in the source text.
pub fn find_context_candidates(text: &str, hunk: &mut Hunk) -> Vec<usize> {
    let candidates: Vec<usize> = text
        .match_indices(&hunk.context)
        .map(|(offset, _)| offset)
        .collect();

    if !candidates.is_empty() {
        return candidates;
    }

    if hunk.context.ends_with('\n') && !hunk.context.is_empty() {
        let old_len = hunk.context.len();
        hunk.context.pop();
        let new_len = hunk.context.len();

        if !hunk.context.is_empty() {
            let candidates: Vec<usize> = text
                .match_indices(&hunk.context)
                .filter(|(offset, _)| offset + new_len == text.len())
                .map(|(offset, _)| offset)
                .collect();

            if !candidates.is_empty() {
                for edit in &mut hunk.edits {
                    let touched_phantom = edit.range.end > new_len;
                    edit.range.start = edit.range.start.min(new_len);
                    edit.range.end = edit.range.end.min(new_len);
                    if touched_phantom {
                        // The replacement text was also written with a
                        // trailing '\n' that corresponds to the phantom
                        // newline we just removed from the context.
                        if edit.text.ends_with('\n') {
                            edit.text.pop();
                        }
                    }
                }
                return candidates;
            }

            // Restore if fallback didn't help either.
            hunk.context.push('\n');
            debug_assert_eq!(hunk.context.len(), old_len);
        } else {
            hunk.context.push('\n');
        }
    }

    Vec::new()
}

/// Given multiple candidate offsets where context matches, use line numbers to disambiguate.
/// Returns the offset that matches the expected line, or None if no match or no line number available.
pub fn disambiguate_by_line_number(
    candidates: &[usize],
    expected_line: Option<u32>,
    offset_to_line: &dyn Fn(usize) -> u32,
) -> Option<usize> {
    match candidates.len() {
        0 => None,
        1 => Some(candidates[0]),
        _ => {
            let expected = expected_line?;
            candidates
                .iter()
                .copied()
                .find(|&offset| offset_to_line(offset) == expected)
        }
    }
}

pub fn apply_diff_to_string(diff_str: &str, text: &str) -> Result<String> {
    apply_diff_to_string_with_hunk_offset(diff_str, text).map(|(text, _)| text)
}

/// Applies a diff to a string and returns the result along with the offset where
/// the first hunk's context matched in the original text. This offset can be used
/// to adjust cursor positions that are relative to the hunk's content.
pub fn apply_diff_to_string_with_hunk_offset(
    diff_str: &str,
    text: &str,
) -> Result<(String, Option<usize>)> {
    let mut diff = DiffParser::new(diff_str);

    let mut text = text.to_string();
    let mut first_hunk_offset = None;

    while let Some(event) = diff.next().context("Failed to parse diff")? {
        match event {
            DiffEvent::Hunk {
                mut hunk,
                path: _,
                status: _,
            } => {
                let candidates = find_context_candidates(&text, &mut hunk);

                let hunk_offset =
                    disambiguate_by_line_number(&candidates, hunk.start_line, &|offset| {
                        text[..offset].matches('\n').count() as u32
                    })
                    .ok_or_else(|| anyhow!("couldn't resolve hunk"))?;

                if first_hunk_offset.is_none() {
                    first_hunk_offset = Some(hunk_offset);
                }

                for edit in hunk.edits.iter().rev() {
                    let range = (hunk_offset + edit.range.start)..(hunk_offset + edit.range.end);
                    text.replace_range(range, &edit.text);
                }
            }
            DiffEvent::FileEnd { .. } => {}
        }
    }

    Ok((text, first_hunk_offset))
}

struct PatchFile<'a> {
    old_path: Cow<'a, str>,
    new_path: Cow<'a, str>,
}

pub struct DiffParser<'a> {
    current_file: Option<PatchFile<'a>>,
    current_line: Option<(&'a str, DiffLine<'a>)>,
    hunk: Hunk,
    diff: std::str::Lines<'a>,
    pending_start_line: Option<u32>,
    processed_no_newline: bool,
    last_diff_op: LastDiffOp,
}

#[derive(Clone, Copy, Default)]
enum LastDiffOp {
    #[default]
    None,
    Context,
    Deletion,
    Addition,
}

#[derive(Debug, PartialEq)]
pub enum DiffEvent<'a> {
    Hunk {
        path: Cow<'a, str>,
        hunk: Hunk,
        status: FileStatus,
    },
    FileEnd {
        renamed_to: Option<Cow<'a, str>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileStatus {
    Created,
    Modified,
    Deleted,
}

#[derive(Debug, Default, PartialEq)]
pub struct Hunk {
    pub context: String,
    pub edits: Vec<Edit>,
    pub start_line: Option<u32>,
}

impl Hunk {
    pub fn is_empty(&self) -> bool {
        self.context.is_empty() && self.edits.is_empty()
    }
}

#[derive(Debug, PartialEq)]
pub struct Edit {
    pub range: Range<usize>,
    pub text: String,
}

impl<'a> DiffParser<'a> {
    pub fn new(diff: &'a str) -> Self {
        let mut diff = diff.lines();
        let current_line = diff.next().map(|line| (line, DiffLine::parse(line)));
        DiffParser {
            current_file: None,
            hunk: Hunk::default(),
            current_line,
            diff,
            pending_start_line: None,
            processed_no_newline: false,
            last_diff_op: LastDiffOp::None,
        }
    }

    pub fn next(&mut self) -> Result<Option<DiffEvent<'a>>> {
        loop {
            let (hunk_done, file_done) = match self.current_line.as_ref().map(|e| &e.1) {
                Some(DiffLine::OldPath { .. }) | Some(DiffLine::Garbage(_)) | None => (true, true),
                Some(DiffLine::HunkHeader(_)) => (true, false),
                _ => (false, false),
            };

            if hunk_done {
                if let Some(file) = &self.current_file
                    && !self.hunk.is_empty()
                {
                    let status = if file.old_path == "/dev/null" {
                        FileStatus::Created
                    } else if file.new_path == "/dev/null" {
                        FileStatus::Deleted
                    } else {
                        FileStatus::Modified
                    };
                    let path = if status == FileStatus::Created {
                        file.new_path.clone()
                    } else {
                        file.old_path.clone()
                    };
                    let mut hunk = mem::take(&mut self.hunk);
                    hunk.start_line = self.pending_start_line.take();
                    self.processed_no_newline = false;
                    self.last_diff_op = LastDiffOp::None;
                    return Ok(Some(DiffEvent::Hunk { path, hunk, status }));
                }
            }

            if file_done {
                if let Some(PatchFile { old_path, new_path }) = self.current_file.take() {
                    return Ok(Some(DiffEvent::FileEnd {
                        renamed_to: if old_path != new_path && old_path != "/dev/null" {
                            Some(new_path)
                        } else {
                            None
                        },
                    }));
                }
            }

            let Some((line, parsed_line)) = self.current_line.take() else {
                break;
            };

            (|| {
                match parsed_line {
                    DiffLine::OldPath { path } => {
                        self.current_file = Some(PatchFile {
                            old_path: path,
                            new_path: "".into(),
                        });
                    }
                    DiffLine::NewPath { path } => {
                        if let Some(current_file) = &mut self.current_file {
                            current_file.new_path = path
                        }
                    }
                    DiffLine::HunkHeader(location) => {
                        if let Some(loc) = location {
                            self.pending_start_line = Some(loc.start_line_old);
                        }
                    }
                    DiffLine::Context(ctx) => {
                        if self.current_file.is_some() {
                            writeln!(&mut self.hunk.context, "{ctx}")?;
                            self.last_diff_op = LastDiffOp::Context;
                        }
                    }
                    DiffLine::Deletion(del) => {
                        if self.current_file.is_some() {
                            let range = self.hunk.context.len()
                                ..self.hunk.context.len() + del.len() + '\n'.len_utf8();
                            if let Some(last_edit) = self.hunk.edits.last_mut()
                                && last_edit.range.end == range.start
                            {
                                last_edit.range.end = range.end;
                            } else {
                                self.hunk.edits.push(Edit {
                                    range,
                                    text: String::new(),
                                });
                            }
                            writeln!(&mut self.hunk.context, "{del}")?;
                            self.last_diff_op = LastDiffOp::Deletion;
                        }
                    }
                    DiffLine::Addition(add) => {
                        if self.current_file.is_some() {
                            let range = self.hunk.context.len()..self.hunk.context.len();
                            if let Some(last_edit) = self.hunk.edits.last_mut()
                                && last_edit.range.end == range.start
                            {
                                writeln!(&mut last_edit.text, "{add}").unwrap();
                            } else {
                                self.hunk.edits.push(Edit {
                                    range,
                                    text: format!("{add}\n"),
                                });
                            }
                            self.last_diff_op = LastDiffOp::Addition;
                        }
                    }
                    DiffLine::NoNewlineAtEOF => {
                        if !self.processed_no_newline {
                            self.processed_no_newline = true;
                            match self.last_diff_op {
                                LastDiffOp::Addition => {
                                    // Remove trailing newline from the last addition
                                    if let Some(last_edit) = self.hunk.edits.last_mut() {
                                        last_edit.text.pop();
                                    }
                                }
                                LastDiffOp::Deletion => {
                                    // Remove trailing newline from context (which includes the deletion)
                                    self.hunk.context.pop();
                                    if let Some(last_edit) = self.hunk.edits.last_mut() {
                                        last_edit.range.end -= 1;
                                    }
                                }
                                LastDiffOp::Context | LastDiffOp::None => {
                                    // Remove trailing newline from context
                                    self.hunk.context.pop();
                                }
                            }
                        }
                    }
                    DiffLine::Garbage(_) => {}
                }

                anyhow::Ok(())
            })()
            .with_context(|| format!("on line:\n\n```\n{}```", line))?;

            self.current_line = self.diff.next().map(|line| (line, DiffLine::parse(line)));
        }

        anyhow::Ok(None)
    }
}

#[derive(Debug, PartialEq)]
pub enum DiffLine<'a> {
    OldPath { path: Cow<'a, str> },
    NewPath { path: Cow<'a, str> },
    HunkHeader(Option<HunkLocation>),
    Context(&'a str),
    Deletion(&'a str),
    Addition(&'a str),
    NoNewlineAtEOF,
    Garbage(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct HunkLocation {
    pub start_line_old: u32,
    pub count_old: u32,
    pub start_line_new: u32,
    pub count_new: u32,
}

impl<'a> DiffLine<'a> {
    pub fn parse(line: &'a str) -> Self {
        Self::try_parse(line).unwrap_or(Self::Garbage(line))
    }

    fn try_parse(line: &'a str) -> Option<Self> {
        if line.starts_with("\\ No newline") {
            return Some(Self::NoNewlineAtEOF);
        }
        if let Some(header) = line.strip_prefix("---").and_then(eat_required_whitespace) {
            let path = parse_header_path("a/", header);
            Some(Self::OldPath { path })
        } else if let Some(header) = line.strip_prefix("+++").and_then(eat_required_whitespace) {
            Some(Self::NewPath {
                path: parse_header_path("b/", header),
            })
        } else if let Some(header) = line.strip_prefix("@@").and_then(eat_required_whitespace) {
            if header.starts_with("...") {
                return Some(Self::HunkHeader(None));
            }

            let mut tokens = header.split_whitespace();
            let old_range = tokens.next()?.strip_prefix('-')?;
            let new_range = tokens.next()?.strip_prefix('+')?;

            let (start_line_old, count_old) = old_range.split_once(',').unwrap_or((old_range, "1"));
            let (start_line_new, count_new) = new_range.split_once(',').unwrap_or((new_range, "1"));

            Some(Self::HunkHeader(Some(HunkLocation {
                start_line_old: start_line_old.parse::<u32>().ok()?.saturating_sub(1),
                count_old: count_old.parse().ok()?,
                start_line_new: start_line_new.parse::<u32>().ok()?.saturating_sub(1),
                count_new: count_new.parse().ok()?,
            })))
        } else if let Some(deleted_header) = line.strip_prefix("-") {
            Some(Self::Deletion(deleted_header))
        } else if line.is_empty() {
            Some(Self::Context(""))
        } else if let Some(context) = line.strip_prefix(" ") {
            Some(Self::Context(context))
        } else {
            Some(Self::Addition(line.strip_prefix("+")?))
        }
    }
}

impl<'a> Display for DiffLine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiffLine::OldPath { path } => write!(f, "--- {path}"),
            DiffLine::NewPath { path } => write!(f, "+++ {path}"),
            DiffLine::HunkHeader(Some(hunk_location)) => {
                write!(
                    f,
                    "@@ -{},{} +{},{} @@",
                    hunk_location.start_line_old + 1,
                    hunk_location.count_old,
                    hunk_location.start_line_new + 1,
                    hunk_location.count_new
                )
            }
            DiffLine::HunkHeader(None) => write!(f, "@@ ... @@"),
            DiffLine::Context(content) => write!(f, " {content}"),
            DiffLine::Deletion(content) => write!(f, "-{content}"),
            DiffLine::Addition(content) => write!(f, "+{content}"),
            DiffLine::NoNewlineAtEOF => write!(f, "\\ No newline at end of file"),
            DiffLine::Garbage(line) => write!(f, "{line}"),
        }
    }
}

fn parse_header_path<'a>(strip_prefix: &'static str, header: &'a str) -> Cow<'a, str> {
    if !header.contains(['"', '\\']) {
        let path = header.split_ascii_whitespace().next().unwrap_or(header);
        return Cow::Borrowed(path.strip_prefix(strip_prefix).unwrap_or(path));
    }

    let mut path = String::with_capacity(header.len());
    let mut in_quote = false;
    let mut chars = header.chars().peekable();
    let mut strip_prefix = Some(strip_prefix);

    while let Some(char) = chars.next() {
        if char == '"' {
            in_quote = !in_quote;
        } else if char == '\\' {
            let Some(&next_char) = chars.peek() else {
                break;
            };
            chars.next();
            path.push(next_char);
        } else if char.is_ascii_whitespace() && !in_quote {
            break;
        } else {
            path.push(char);
        }

        if let Some(prefix) = strip_prefix
            && path == prefix
        {
            strip_prefix.take();
            path.clear();
        }
    }

    Cow::Owned(path)
}

fn eat_required_whitespace(header: &str) -> Option<&str> {
    let trimmed = header.trim_ascii_start();

    if trimmed.len() == header.len() {
        None
    } else {
        Some(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_lines_simple() {
        let input = indoc! {"
            diff --git a/text.txt b/text.txt
            index 86c770d..a1fd855 100644
            --- a/file.txt
            +++ b/file.txt
            @@ -1,2 +1,3 @@
             context
            -deleted
            +inserted
            garbage

            --- b/file.txt
            +++ a/file.txt
        "};

        let lines = input.lines().map(DiffLine::parse).collect::<Vec<_>>();

        assert_eq!(
            lines,
            &[
                DiffLine::Garbage("diff --git a/text.txt b/text.txt"),
                DiffLine::Garbage("index 86c770d..a1fd855 100644"),
                DiffLine::OldPath {
                    path: "file.txt".into()
                },
                DiffLine::NewPath {
                    path: "file.txt".into()
                },
                DiffLine::HunkHeader(Some(HunkLocation {
                    start_line_old: 0,
                    count_old: 2,
                    start_line_new: 0,
                    count_new: 3
                })),
                DiffLine::Context("context"),
                DiffLine::Deletion("deleted"),
                DiffLine::Addition("inserted"),
                DiffLine::Garbage("garbage"),
                DiffLine::Context(""),
                DiffLine::OldPath {
                    path: "b/file.txt".into()
                },
                DiffLine::NewPath {
                    path: "a/file.txt".into()
                },
            ]
        );
    }

    #[test]
    fn file_header_extra_space() {
        let options = ["--- file", "---   file", "---\tfile"];

        for option in options {
            assert_eq!(
                DiffLine::parse(option),
                DiffLine::OldPath {
                    path: "file".into()
                },
                "{option}",
            );
        }
    }

    #[test]
    fn hunk_header_extra_space() {
        let options = [
            "@@ -1,2 +1,3 @@",
            "@@  -1,2  +1,3 @@",
            "@@\t-1,2\t+1,3\t@@",
            "@@ -1,2  +1,3 @@",
            "@@ -1,2   +1,3 @@",
            "@@ -1,2 +1,3   @@",
            "@@ -1,2 +1,3 @@ garbage",
        ];

        for option in options {
            assert_eq!(
                DiffLine::parse(option),
                DiffLine::HunkHeader(Some(HunkLocation {
                    start_line_old: 0,
                    count_old: 2,
                    start_line_new: 0,
                    count_new: 3
                })),
                "{option}",
            );
        }
    }

    #[test]
    fn hunk_header_without_location() {
        assert_eq!(DiffLine::parse("@@ ... @@"), DiffLine::HunkHeader(None));
    }

    #[test]
    fn test_parse_path() {
        assert_eq!(parse_header_path("a/", "foo.txt"), "foo.txt");
        assert_eq!(
            parse_header_path("a/", "foo/bar/baz.txt"),
            "foo/bar/baz.txt"
        );
        assert_eq!(parse_header_path("a/", "a/foo.txt"), "foo.txt");
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt"),
            "foo/bar/baz.txt"
        );

        // Extra
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt  2025"),
            "foo/bar/baz.txt"
        );
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt\t2025"),
            "foo/bar/baz.txt"
        );
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt \""),
            "foo/bar/baz.txt"
        );

        // Quoted
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/\"baz quox.txt\""),
            "foo/bar/baz quox.txt"
        );
        assert_eq!(
            parse_header_path("a/", "\"a/foo/bar/baz quox.txt\""),
            "foo/bar/baz quox.txt"
        );
        assert_eq!(
            parse_header_path("a/", "\"foo/bar/baz quox.txt\""),
            "foo/bar/baz quox.txt"
        );
        assert_eq!(parse_header_path("a/", "\"whatever 🤷\""), "whatever 🤷");
        assert_eq!(
            parse_header_path("a/", "\"foo/bar/baz quox.txt\"  2025"),
            "foo/bar/baz quox.txt"
        );
        // unescaped quotes are dropped
        assert_eq!(parse_header_path("a/", "foo/\"bar\""), "foo/bar");

        // Escaped
        assert_eq!(
            parse_header_path("a/", "\"foo/\\\"bar\\\"/baz.txt\""),
            "foo/\"bar\"/baz.txt"
        );
        assert_eq!(
            parse_header_path("a/", "\"C:\\\\Projects\\\\My App\\\\old file.txt\""),
            "C:\\Projects\\My App\\old file.txt"
        );
    }

    #[test]
    fn test_parse_diff_with_leading_and_trailing_garbage() {
        let diff = indoc! {"
            I need to make some changes.

            I'll change the following things:
            - one
              - two
            - three

            ```
            --- a/file.txt
            +++ b/file.txt
             one
            +AND
             two
            ```

            Summary of what I did:
            - one
              - two
            - three

            That's about it.
        "};

        let mut events = Vec::new();
        let mut parser = DiffParser::new(diff);
        while let Some(event) = parser.next().unwrap() {
            events.push(event);
        }

        assert_eq!(
            events,
            &[
                DiffEvent::Hunk {
                    path: "file.txt".into(),
                    hunk: Hunk {
                        context: "one\ntwo\n".into(),
                        edits: vec![Edit {
                            range: 4..4,
                            text: "AND\n".into()
                        }],
                        start_line: None,
                    },
                    status: FileStatus::Modified,
                },
                DiffEvent::FileEnd { renamed_to: None }
            ],
        )
    }

    #[test]
    fn test_no_newline_at_eof() {
        let diff = indoc! {"
            --- a/file.py
            +++ b/file.py
            @@ -55,7 +55,3 @@ class CustomDataset(Dataset):
                         torch.set_rng_state(state)
                         mask = self.transform(mask)

            -        if self.mode == 'Training':
            -            return (img, mask, name)
            -        else:
            -            return (img, mask, name)
            \\ No newline at end of file
        "};

        let mut events = Vec::new();
        let mut parser = DiffParser::new(diff);
        while let Some(event) = parser.next().unwrap() {
            events.push(event);
        }

        assert_eq!(
            events,
            &[
                DiffEvent::Hunk {
                    path: "file.py".into(),
                    hunk: Hunk {
                        context: concat!(
                            "            torch.set_rng_state(state)\n",
                            "            mask = self.transform(mask)\n",
                            "\n",
                            "        if self.mode == 'Training':\n",
                            "            return (img, mask, name)\n",
                            "        else:\n",
                            "            return (img, mask, name)",
                        )
                        .into(),
                        edits: vec![Edit {
                            range: 80..203,
                            text: "".into()
                        }],
                        start_line: Some(54), // @@ -55,7 -> line 54 (0-indexed)
                    },
                    status: FileStatus::Modified,
                },
                DiffEvent::FileEnd { renamed_to: None }
            ],
        );
    }

    #[test]
    fn test_no_newline_at_eof_addition() {
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,2 +1,3 @@
             context
            -deleted
            +added line
            \\ No newline at end of file
        "};

        let mut events = Vec::new();
        let mut parser = DiffParser::new(diff);
        while let Some(event) = parser.next().unwrap() {
            events.push(event);
        }

        assert_eq!(
            events,
            &[
                DiffEvent::Hunk {
                    path: "file.txt".into(),
                    hunk: Hunk {
                        context: "context\ndeleted\n".into(),
                        edits: vec![Edit {
                            range: 8..16,
                            text: "added line".into()
                        }],
                        start_line: Some(0), // @@ -1,2 -> line 0 (0-indexed)
                    },
                    status: FileStatus::Modified,
                },
                DiffEvent::FileEnd { renamed_to: None }
            ],
        );
    }

    #[test]
    fn test_double_no_newline_at_eof() {
        // Two consecutive "no newline" markers - the second should be ignored
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,3 +1,3 @@
             line1
            -old
            +new
             line3
            \\ No newline at end of file
            \\ No newline at end of file
        "};

        let mut events = Vec::new();
        let mut parser = DiffParser::new(diff);
        while let Some(event) = parser.next().unwrap() {
            events.push(event);
        }

        assert_eq!(
            events,
            &[
                DiffEvent::Hunk {
                    path: "file.txt".into(),
                    hunk: Hunk {
                        context: "line1\nold\nline3".into(), // Only one newline removed
                        edits: vec![Edit {
                            range: 6..10, // "old\n" is 4 bytes
                            text: "new\n".into()
                        }],
                        start_line: Some(0),
                    },
                    status: FileStatus::Modified,
                },
                DiffEvent::FileEnd { renamed_to: None }
            ],
        );
    }

    #[test]
    fn test_no_newline_after_context_not_addition() {
        // "No newline" after context lines should remove newline from context,
        // not from an earlier addition
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,4 +1,4 @@
             line1
            -old
            +new
             line3
             line4
            \\ No newline at end of file
        "};

        let mut events = Vec::new();
        let mut parser = DiffParser::new(diff);
        while let Some(event) = parser.next().unwrap() {
            events.push(event);
        }

        assert_eq!(
            events,
            &[
                DiffEvent::Hunk {
                    path: "file.txt".into(),
                    hunk: Hunk {
                        // newline removed from line4 (context), not from "new" (addition)
                        context: "line1\nold\nline3\nline4".into(),
                        edits: vec![Edit {
                            range: 6..10,         // "old\n" is 4 bytes
                            text: "new\n".into()  // Still has newline
                        }],
                        start_line: Some(0),
                    },
                    status: FileStatus::Modified,
                },
                DiffEvent::FileEnd { renamed_to: None }
            ],
        );
    }

    #[test]
    fn test_strip_diff_metadata() {
        let diff_with_metadata = indoc! {r#"
            diff --git a/file.txt b/file.txt
            index 1234567..abcdefg 100644
            --- a/file.txt
            +++ b/file.txt
            @@ -1,3 +1,4 @@
             context line
            -removed line
            +added line
             more context
        "#};

        let stripped = strip_diff_metadata(diff_with_metadata);

        assert_eq!(
            stripped,
            indoc! {r#"
                --- a/file.txt
                +++ b/file.txt
                @@ -1,3 +1,4 @@
                 context line
                -removed line
                +added line
                 more context
            "#}
        );
    }

    #[test]
    fn test_apply_diff_to_string_no_trailing_newline() {
        // Text without trailing newline; diff generated without
        // `\ No newline at end of file` marker.
        let text = "line1\nline2\nline3";
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,3 +1,3 @@
             line1
            -line2
            +replaced
             line3
        "};

        let result = apply_diff_to_string(diff, text).unwrap();
        assert_eq!(result, "line1\nreplaced\nline3");
    }

    #[test]
    fn test_apply_diff_to_string_trailing_newline_present() {
        // When text has a trailing newline, exact matching still works and
        // the fallback is never needed.
        let text = "line1\nline2\nline3\n";
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,3 +1,3 @@
             line1
            -line2
            +replaced
             line3
        "};

        let result = apply_diff_to_string(diff, text).unwrap();
        assert_eq!(result, "line1\nreplaced\nline3\n");
    }

    #[test]
    fn test_apply_diff_to_string_deletion_at_end_no_trailing_newline() {
        // Deletion of the last line when text has no trailing newline.
        // The edit range must be clamped so it doesn't index past the
        // end of the text.
        let text = "line1\nline2\nline3";
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,3 +1,2 @@
             line1
             line2
            -line3
        "};

        let result = apply_diff_to_string(diff, text).unwrap();
        assert_eq!(result, "line1\nline2\n");
    }

    #[test]
    fn test_apply_diff_to_string_replace_last_line_no_trailing_newline() {
        // Replace the last line when text has no trailing newline.
        let text = "aaa\nbbb\nccc";
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,3 +1,3 @@
             aaa
             bbb
            -ccc
            +ddd
        "};

        let result = apply_diff_to_string(diff, text).unwrap();
        assert_eq!(result, "aaa\nbbb\nddd");
    }

    #[test]
    fn test_apply_diff_to_string_multibyte_no_trailing_newline() {
        // Multi-byte UTF-8 characters near the end; ensures char boundary
        // safety when the fallback clamps edit ranges.
        let text = "hello\n세계";
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,2 +1,2 @@
             hello
            -세계
            +world
        "};

        let result = apply_diff_to_string(diff, text).unwrap();
        assert_eq!(result, "hello\nworld");
    }

    #[test]
    fn test_find_context_candidates_no_false_positive_mid_text() {
        // The stripped fallback must only match at the end of text, not in
        // the middle where a real newline exists.
        let text = "aaa\nbbb\nccc\n";
        let mut hunk = Hunk {
            context: "bbb\n".into(),
            edits: vec![],
            start_line: None,
        };

        let candidates = find_context_candidates(text, &mut hunk);
        // Exact match at offset 4 — the fallback is not used.
        assert_eq!(candidates, vec![4]);
    }

    #[test]
    fn test_find_context_candidates_fallback_at_end() {
        let text = "aaa\nbbb";
        let mut hunk = Hunk {
            context: "bbb\n".into(),
            edits: vec![],
            start_line: None,
        };

        let candidates = find_context_candidates(text, &mut hunk);
        assert_eq!(candidates, vec![4]);
        // Context should be stripped.
        assert_eq!(hunk.context, "bbb");
    }

    #[test]
    fn test_find_context_candidates_no_fallback_mid_text() {
        // "bbb" appears mid-text followed by a newline, so the exact
        // match succeeds. Verify the stripped fallback doesn't produce a
        // second, spurious candidate.
        let text = "aaa\nbbb\nccc";
        let mut hunk = Hunk {
            context: "bbb\nccc\n".into(),
            edits: vec![],
            start_line: None,
        };

        let candidates = find_context_candidates(text, &mut hunk);
        // No exact match (text ends without newline after "ccc"), but the
        // stripped context "bbb\nccc" matches at offset 4, which is the end.
        assert_eq!(candidates, vec![4]);
        assert_eq!(hunk.context, "bbb\nccc");
    }

    #[test]
    fn test_find_context_candidates_clamps_edit_ranges() {
        let text = "aaa\nbbb";
        let mut hunk = Hunk {
            context: "aaa\nbbb\n".into(),
            edits: vec![Edit {
                range: 4..8, // "bbb\n" — end points at the trailing \n
                text: "ccc\n".into(),
            }],
            start_line: None,
        };

        let candidates = find_context_candidates(text, &mut hunk);
        assert_eq!(candidates, vec![0]);
        // Edit range end should be clamped to 7 (new context length).
        assert_eq!(hunk.edits[0].range, 4..7);
    }
}
