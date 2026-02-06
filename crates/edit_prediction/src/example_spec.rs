use crate::udiff::DiffLine;
use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Write as _, mem, ops::Range, path::Path, sync::Arc};
use telemetry_events::EditPredictionRating;

pub const CURSOR_POSITION_MARKER: &str = "[CURSOR_POSITION]";
pub const SELECTION_MARKER: &str = "[SELECTION]";
pub const INLINE_CURSOR_MARKER: &str = "<|user_cursor|>";

/// Maximum cursor file size to capture (64KB).
/// Files larger than this will not have their content captured,
/// falling back to git-based loading.
pub const MAX_CURSOR_FILE_SIZE: usize = 64 * 1024;

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct ExampleSpec {
    #[serde(default)]
    pub name: String,
    pub repository_url: String,
    pub revision: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    #[serde(default)]
    pub uncommitted_diff: String,
    pub cursor_path: Arc<Path>,
    pub cursor_position: String,
    pub edit_history: String,
    pub expected_patches: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected_patch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_prompt_input: Option<CapturedPromptInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<TelemetrySource>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub human_feedback: Vec<HumanFeedback>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating: Option<EditPredictionRating>,
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct HumanFeedback {
    pub message: String,
}

/// Metadata for examples sourced from production telemetry (rejected predictions).
#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct TelemetrySource {
    pub request_id: String,
    pub device_id: String,
    pub time: String,
    pub rejection_reason: String,
    pub was_shown: bool,
}

/// All data needed to run format_prompt without loading the project.
#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct CapturedPromptInput {
    pub cursor_file_content: String,
    pub cursor_offset: usize,
    pub cursor_row: u32,
    pub cursor_column: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excerpt_start_row: Option<u32>,
    pub events: Vec<CapturedEvent>,
    pub related_files: Vec<CapturedRelatedFile>,
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct CapturedEvent {
    pub path: Arc<Path>,
    pub old_path: Arc<Path>,
    pub diff: String,
    pub predicted: bool,
    pub in_open_source_repo: bool,
}

impl CapturedEvent {
    pub fn to_event(&self) -> zeta_prompt::Event {
        zeta_prompt::Event::BufferChange {
            path: self.path.clone(),
            old_path: self.old_path.clone(),
            diff: self.diff.clone(),
            predicted: self.predicted,
            in_open_source_repo: self.in_open_source_repo,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct CapturedRelatedFile {
    pub path: Arc<Path>,
    pub max_row: u32,
    pub excerpts: Vec<CapturedRelatedExcerpt>,
}

impl CapturedRelatedFile {
    pub fn to_related_file(&self) -> zeta_prompt::RelatedFile {
        zeta_prompt::RelatedFile {
            path: self.path.clone(),
            max_row: self.max_row,
            excerpts: self
                .excerpts
                .iter()
                .map(|e| zeta_prompt::RelatedExcerpt {
                    row_range: e.row_range.clone(),
                    text: e.text.clone().into(),
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct CapturedRelatedExcerpt {
    pub row_range: Range<u32>,
    pub text: String,
}

const REASONING_HEADING: &str = "Reasoning";
const UNCOMMITTED_DIFF_HEADING: &str = "Uncommitted Diff";
const EDIT_HISTORY_HEADING: &str = "Edit History";
const CURSOR_POSITION_HEADING: &str = "Cursor Position";
const EXPECTED_PATCH_HEADING: &str = "Expected Patch";
const REJECTED_PATCH_HEADING: &str = "Rejected Patch";

#[derive(Serialize, Deserialize)]
struct FrontMatter<'a> {
    repository_url: Cow<'a, str>,
    revision: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
}

impl ExampleSpec {
    /// Generate a sanitized filename for this example.
    pub fn filename(&self) -> String {
        self.name
            .chars()
            .map(|c| match c {
                ' ' | ':' | '~' | '^' | '?' | '*' | '[' | '\\' | '@' | '{' | '/' | '<' | '>'
                | '|' | '"' => '-',
                c => c,
            })
            .collect()
    }

    /// Format this example spec as markdown.
    pub fn to_markdown(&self) -> String {
        use std::fmt::Write as _;

        let front_matter = FrontMatter {
            repository_url: Cow::Borrowed(&self.repository_url),
            revision: Cow::Borrowed(&self.revision),
            tags: self.tags.clone(),
        };
        let front_matter_toml =
            toml::to_string_pretty(&front_matter).unwrap_or_else(|_| String::new());

        let mut markdown = String::new();

        _ = writeln!(markdown, "+++");
        markdown.push_str(&front_matter_toml);
        if !markdown.ends_with('\n') {
            markdown.push('\n');
        }
        _ = writeln!(markdown, "+++");
        markdown.push('\n');

        _ = writeln!(markdown, "# {}", self.name);
        markdown.push('\n');

        if let Some(reasoning) = &self.reasoning {
            _ = writeln!(markdown, "## {}", REASONING_HEADING);
            markdown.push('\n');
            markdown.push_str(reasoning);
            if !markdown.ends_with('\n') {
                markdown.push('\n');
            }
            markdown.push('\n');
        }

        if !self.uncommitted_diff.is_empty() {
            _ = writeln!(markdown, "## {}", UNCOMMITTED_DIFF_HEADING);
            _ = writeln!(markdown);
            _ = writeln!(markdown, "```diff");
            markdown.push_str(&self.uncommitted_diff);
            if !markdown.ends_with('\n') {
                markdown.push('\n');
            }
            _ = writeln!(markdown, "```");
            markdown.push('\n');
        }

        _ = writeln!(markdown, "## {}", EDIT_HISTORY_HEADING);
        _ = writeln!(markdown);

        if self.edit_history.is_empty() {
            _ = writeln!(markdown, "(No edit history)");
            _ = writeln!(markdown);
        } else {
            _ = writeln!(markdown, "```diff");
            markdown.push_str(&self.edit_history);
            if !markdown.ends_with('\n') {
                markdown.push('\n');
            }
            _ = writeln!(markdown, "```");
            markdown.push('\n');
        }

        _ = writeln!(markdown, "## {}", CURSOR_POSITION_HEADING);
        _ = writeln!(markdown);
        _ = writeln!(markdown, "```{}", self.cursor_path.to_string_lossy());
        markdown.push_str(&self.cursor_position);
        if !markdown.ends_with('\n') {
            markdown.push('\n');
        }
        _ = writeln!(markdown, "```");
        markdown.push('\n');

        _ = writeln!(markdown, "## {}", EXPECTED_PATCH_HEADING);
        markdown.push('\n');
        for patch in &self.expected_patches {
            _ = writeln!(markdown, "```diff");
            markdown.push_str(patch);
            if !markdown.ends_with('\n') {
                markdown.push('\n');
            }
            _ = writeln!(markdown, "```");
            markdown.push('\n');
        }

        if let Some(rejected_patch) = &self.rejected_patch {
            _ = writeln!(markdown, "## {}", REJECTED_PATCH_HEADING);
            markdown.push('\n');
            _ = writeln!(markdown, "```diff");
            markdown.push_str(rejected_patch);
            if !markdown.ends_with('\n') {
                markdown.push('\n');
            }
            _ = writeln!(markdown, "```");
            markdown.push('\n');
        }

        markdown
    }

    /// Parse an example spec from markdown.
    pub fn from_markdown(mut input: &str) -> anyhow::Result<Self> {
        use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Parser, Tag, TagEnd};

        let mut spec = ExampleSpec {
            name: String::new(),
            repository_url: String::new(),
            revision: String::new(),
            tags: Vec::new(),
            reasoning: None,
            uncommitted_diff: String::new(),
            cursor_path: Path::new("").into(),
            cursor_position: String::new(),
            edit_history: String::new(),
            expected_patches: Vec::new(),
            rejected_patch: None,
            captured_prompt_input: None,
            telemetry: None,
            human_feedback: Vec::new(),
            rating: None,
        };

        if let Some(rest) = input.strip_prefix("+++\n")
            && let Some((front_matter, rest)) = rest.split_once("+++\n")
        {
            if let Ok(data) = toml::from_str::<FrontMatter<'_>>(front_matter) {
                spec.repository_url = data.repository_url.into_owned();
                spec.revision = data.revision.into_owned();
                spec.tags = data.tags;
            }
            input = rest.trim_start();
        }

        let parser = Parser::new(input);
        let mut text = String::new();
        let mut block_info: CowStr = "".into();

        #[derive(PartialEq)]
        enum Section {
            Start,
            UncommittedDiff,
            EditHistory,
            CursorPosition,
            ExpectedPatch,
            RejectedPatch,
            Other,
        }

        let mut current_section = Section::Start;

        for event in parser {
            match event {
                Event::Text(line) => {
                    text.push_str(&line);
                }
                Event::End(TagEnd::Heading(HeadingLevel::H1)) => {
                    spec.name = mem::take(&mut text);
                }
                Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                    let title = mem::take(&mut text);
                    current_section = if title.eq_ignore_ascii_case(UNCOMMITTED_DIFF_HEADING) {
                        Section::UncommittedDiff
                    } else if title.eq_ignore_ascii_case(EDIT_HISTORY_HEADING) {
                        Section::EditHistory
                    } else if title.eq_ignore_ascii_case(CURSOR_POSITION_HEADING) {
                        Section::CursorPosition
                    } else if title.eq_ignore_ascii_case(EXPECTED_PATCH_HEADING) {
                        Section::ExpectedPatch
                    } else if title.eq_ignore_ascii_case(REJECTED_PATCH_HEADING) {
                        Section::RejectedPatch
                    } else {
                        Section::Other
                    };
                }
                Event::End(TagEnd::Heading(HeadingLevel::H3)) => {
                    mem::take(&mut text);
                }
                Event::End(TagEnd::Heading(HeadingLevel::H4)) => {
                    mem::take(&mut text);
                }
                Event::End(TagEnd::Heading(level)) => {
                    anyhow::bail!("Unexpected heading level: {level}");
                }
                Event::Start(Tag::CodeBlock(kind)) => {
                    match kind {
                        CodeBlockKind::Fenced(info) => {
                            block_info = info;
                        }
                        CodeBlockKind::Indented => {
                            anyhow::bail!("Unexpected indented codeblock");
                        }
                    };
                }
                Event::Start(_) => {
                    text.clear();
                    block_info = "".into();
                }
                Event::End(TagEnd::CodeBlock) => {
                    let block_info = block_info.trim();
                    match current_section {
                        Section::UncommittedDiff => {
                            spec.uncommitted_diff = mem::take(&mut text);
                        }
                        Section::EditHistory => {
                            spec.edit_history.push_str(&mem::take(&mut text));
                        }
                        Section::CursorPosition => {
                            spec.cursor_path = Path::new(block_info).into();
                            spec.cursor_position = mem::take(&mut text);
                        }
                        Section::ExpectedPatch => {
                            spec.expected_patches.push(mem::take(&mut text));
                        }
                        Section::RejectedPatch => {
                            spec.rejected_patch = Some(mem::take(&mut text));
                        }
                        Section::Start | Section::Other => {}
                    }
                }
                _ => {}
            }
        }

        if spec.cursor_path.as_ref() == Path::new("") || spec.cursor_position.is_empty() {
            anyhow::bail!("Missing cursor position codeblock");
        }

        Ok(spec)
    }

    /// Returns the excerpt of text around the cursor, and the offset of the cursor within that
    /// excerpt.
    ///
    /// The cursor's position is marked with a special comment that appears
    /// below the cursor line, which contains the string `[CURSOR_POSITION]`,
    /// preceded by an arrow marking the cursor's column. The arrow can be
    /// either:
    /// - `^` - The cursor column is at the position of the `^` character (pointing up to the cursor)
    /// - `<` - The cursor column is at the first non-whitespace character on that line.
    /// Returns the cursor excerpt and selection range.
    ///
    /// For backwards compatibility, this also supports returning a cursor position (empty selection).
    pub fn cursor_excerpt_with_selection(&self) -> Result<(String, Range<usize>)> {
        let input = &self.cursor_position;

        // Check for inline cursor marker first
        if let Some(inline_offset) = input.find(INLINE_CURSOR_MARKER) {
            let excerpt = input[..inline_offset].to_string()
                + &input[inline_offset + INLINE_CURSOR_MARKER.len()..];
            return Ok((excerpt, inline_offset..inline_offset));
        }

        let marker_offset = input
            .find(SELECTION_MARKER)
            .or_else(|| input.find(CURSOR_POSITION_MARKER))
            .context("missing [SELECTION] marker")?;
        let marker_line_start = input[..marker_offset]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0);
        let marker_line_end = input[marker_line_start..]
            .find('\n')
            .map(|pos| marker_line_start + pos + 1)
            .unwrap_or(input.len());
        let marker_line = &input[marker_line_start..marker_line_end].trim_end_matches('\n');

        let (cursor_column, selection_start_column) = if let Some(caret_pos) = marker_line.find('^')
        {
            // Count dashes before the caret to determine selection start.
            // The cursor is always at the end of the selection (at the caret position).
            let dashes_before = marker_line[..caret_pos]
                .chars()
                .rev()
                .take_while(|&c| c == '-')
                .count();

            // Check if there's a `<` before the dashes, indicating selection starts at column 0.
            // Format: `#<---^` means selection from column 0 to cursor position.
            let before_dashes_pos = caret_pos - dashes_before;
            let has_left_angle_bracket =
                before_dashes_pos > 0 && marker_line[..before_dashes_pos].ends_with('<');

            let selection_start_col = if has_left_angle_bracket {
                0
            } else {
                caret_pos - dashes_before
            };

            (caret_pos, selection_start_col)
        } else if let Some(left_angle_pos) = marker_line.find('<') {
            let first_non_whitespace = marker_line
                .find(|c: char| !c.is_whitespace())
                .unwrap_or(left_angle_pos);
            (first_non_whitespace, first_non_whitespace)
        } else {
            anyhow::bail!("cursor position marker line must contain '^' or '<' before [SELECTION]");
        };

        let mut excerpt = input[..marker_line_start].to_string() + &input[marker_line_end..];
        excerpt.truncate(excerpt.trim_end_matches('\n').len());

        // The cursor is on the line above the marker line.
        let cursor_line_end = marker_line_start.saturating_sub(1);
        let cursor_line_start = excerpt[..cursor_line_end]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0);
        let cursor_offset = cursor_line_start + cursor_column;
        let selection_start_offset = cursor_line_start + selection_start_column;

        Ok((excerpt, selection_start_offset..cursor_offset))
    }

    /// Sets the cursor position excerpt from a plain excerpt and selection range.
    ///
    /// The `line_comment_prefix` is used to format the marker line as a comment.
    /// If the selection starts at column 0, the `<` format is used.
    /// Otherwise, the `^` format is used with dashes for selected text.
    ///
    /// For an empty selection (cursor only), pass a range where start == end.
    pub fn set_cursor_excerpt_with_selection(
        &mut self,
        excerpt: &str,
        selection: Range<usize>,
        line_comment_prefix: &str,
    ) {
        let cursor_offset = selection.end;
        let selection_start = selection.start;
        let is_empty_selection = selection.start == selection.end;

        // Find which line the cursor is on and its column.
        let cursor_line_start = excerpt[..cursor_offset]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0);
        let cursor_line_end = excerpt[cursor_line_start..]
            .find('\n')
            .map(|pos| cursor_line_start + pos + 1)
            .unwrap_or(excerpt.len());
        let cursor_line = &excerpt[cursor_line_start..cursor_line_end];
        let cursor_line_indent = &cursor_line[..cursor_line.len() - cursor_line.trim_start().len()];
        let cursor_column = cursor_offset - cursor_line_start;
        let selection_start_column = selection_start.saturating_sub(cursor_line_start);

        // Build the marker line.
        let mut marker_line = String::new();
        if selection_start_column == 0 && !is_empty_selection {
            // Selection starts at column 0, use `<` format.
            // The `<` represents column 0, dashes cover columns 1 to cursor_column-1,
            // and `^` points to cursor_column.
            marker_line.push_str(line_comment_prefix);
            marker_line.push('<');
            for _ in "<^".len()..cursor_column {
                marker_line.push('-');
            }
            marker_line.push('^');
            marker_line.push_str(SELECTION_MARKER);
        } else if cursor_column < line_comment_prefix.len() && is_empty_selection {
            // Cursor at column 0 with empty selection.
            for _ in 0..cursor_column {
                marker_line.push(' ');
            }
            marker_line.push_str(line_comment_prefix);
            write!(marker_line, " <{}", CURSOR_POSITION_MARKER).unwrap();
        } else {
            // Normal case with `^` format.
            if cursor_column >= cursor_line_indent.len() + line_comment_prefix.len() {
                marker_line.push_str(cursor_line_indent);
            }
            marker_line.push_str(line_comment_prefix);

            // Write spaces and dashes up to cursor position.
            for i in marker_line.len()..cursor_column {
                if i >= selection_start_column && !is_empty_selection {
                    marker_line.push('-');
                } else {
                    marker_line.push(' ');
                }
            }

            marker_line.push('^');
            if is_empty_selection {
                marker_line.push_str(CURSOR_POSITION_MARKER);
            } else {
                marker_line.push_str(SELECTION_MARKER);
            }
        }

        // Build the final cursor_position string.
        let mut result = String::with_capacity(excerpt.len() + marker_line.len() + 2);
        result.push_str(&excerpt[..cursor_line_end]);
        if !result.ends_with('\n') {
            result.push('\n');
        }
        result.push_str(&marker_line);
        if cursor_line_end < excerpt.len() {
            result.push('\n');
            result.push_str(&excerpt[cursor_line_end..]);
        }

        self.cursor_position = result;
    }

    /// Returns all of the possible expected patches for this example, each with an optional
    /// cursor offset.
    ///
    /// The cursor offset is an offset within the new text (after applying the patch), relative
    /// to the start of the hunk.
    ///
    /// In the serialized representation of this example, the cursor position is represented
    /// using a comment line in the diff, beginning with `#`, and containing a `[CURSOR_POSITION]`
    /// or `[SELECTION]` marker with the same format as the [`Self::cursor_excerpt`].
    ///
    /// For selections, the format uses `-` characters to indicate selected text and `^` for the
    /// cursor position. For example:
    /// ```text
    /// +import module
    /// #       ------^[SELECTION]
    /// ```
    /// This indicates "module" is selected with the cursor at the end.
    pub fn expected_patches_with_selections(&self) -> Vec<(String, Option<Range<usize>>)> {
        self.expected_patches
            .iter()
            .map(|patch| {
                let mut clean_patch = String::new();
                let mut selection: Option<Range<usize>> = None;
                let mut line_start_offset = 0usize;
                let mut prev_line_start_offset = 0usize;

                for line in patch.lines() {
                    let diff_line = DiffLine::parse(line);

                    match &diff_line {
                        DiffLine::Garbage(content)
                            if content.starts_with('#')
                                && (content.contains(CURSOR_POSITION_MARKER)
                                    || content.contains(SELECTION_MARKER)) =>
                        {
                            let caret_column = if let Some(caret_pos) = content.find('^') {
                                caret_pos
                            } else if content.find('<').is_some() {
                                0
                            } else {
                                continue;
                            };

                            // Count dashes before the caret to determine selection start.
                            // The cursor is always at the end of the selection.
                            let dashes_before = content[..caret_column]
                                .chars()
                                .rev()
                                .take_while(|&c| c == '-')
                                .count();

                            let cursor_column = caret_column.saturating_sub('#'.len_utf8());
                            let cursor_offset = prev_line_start_offset + cursor_column;

                            let selection_start = cursor_offset - dashes_before;
                            selection = Some(selection_start..cursor_offset);
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

                (clean_patch, selection)
            })
            .collect()
    }

    pub fn set_expected_patches_with_selections(
        &mut self,
        patches: Vec<(String, Option<Range<usize>>)>,
    ) {
        self.expected_patches = patches
            .into_iter()
            .map(|(patch, selection)| {
                let Some(selection) = selection else {
                    return patch;
                };

                let mut result = String::new();
                let mut line_start_offset = 0usize;

                for line in patch.lines() {
                    if !result.is_empty() {
                        result.push('\n');
                    }
                    result.push_str(line);

                    match DiffLine::parse(line) {
                        DiffLine::Addition(content) => {
                            let line_end_offset = line_start_offset + content.len();

                            // Check if the selection head (end) falls within this line.
                            let cursor_offset = selection.end;
                            if cursor_offset >= line_start_offset
                                && cursor_offset <= line_end_offset
                            {
                                let cursor_column = cursor_offset - line_start_offset;
                                let selection_start_column =
                                    selection.start.saturating_sub(line_start_offset);
                                let is_empty_selection = selection.start == selection.end;

                                result.push('\n');
                                result.push('#');

                                // Write dashes for selection before cursor.
                                for i in 0..cursor_column {
                                    if i >= selection_start_column && !is_empty_selection {
                                        result.push('-');
                                    } else {
                                        result.push(' ');
                                    }
                                }

                                result.push('^');
                                if is_empty_selection {
                                    write!(result, "{}", CURSOR_POSITION_MARKER).unwrap();
                                } else {
                                    write!(result, "{}", SELECTION_MARKER).unwrap();
                                }
                            }

                            line_start_offset = line_end_offset + 1;
                        }
                        DiffLine::Context(content) => {
                            line_start_offset += content.len() + 1;
                        }
                        _ => {}
                    }
                }

                if patch.ends_with('\n') {
                    result.push('\n');
                }

                result
            })
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_cursor_excerpt_with_selection_caret() {
        let mut spec = ExampleSpec {
            name: String::new(),
            repository_url: String::new(),
            revision: String::new(),
            tags: Vec::new(),
            reasoning: None,
            uncommitted_diff: String::new(),
            cursor_path: Path::new("test.rs").into(),
            cursor_position: String::new(),
            edit_history: String::new(),
            expected_patches: Vec::new(),
            rejected_patch: None,
            captured_prompt_input: None,
            telemetry: None,
            human_feedback: Vec::new(),
            rating: None,
        };

        // Cursor before `42`
        let excerpt = indoc! {"
            fn main() {
                let x = 42;
                println!(\"{}\", x);
            }"
        };
        let offset = excerpt.find("42").unwrap();
        let position_string = indoc! {"
            fn main() {
                let x = 42;
                //      ^[CURSOR_POSITION]
                println!(\"{}\", x);
            }"
        }
        .to_string();

        spec.set_cursor_excerpt_with_selection(excerpt, offset..offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt_with_selection().unwrap(),
            (excerpt.to_string(), offset..offset)
        );

        // Cursor after `l` in `let`
        let offset = excerpt.find("et x").unwrap();
        let position_string = indoc! {"
            fn main() {
                let x = 42;
            //   ^[CURSOR_POSITION]
                println!(\"{}\", x);
            }"
        }
        .to_string();

        spec.set_cursor_excerpt_with_selection(excerpt, offset..offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt_with_selection().unwrap(),
            (excerpt.to_string(), offset..offset)
        );

        // Cursor before `let`
        let offset = excerpt.find("let").unwrap();
        let position_string = indoc! {"
            fn main() {
                let x = 42;
            //  ^[CURSOR_POSITION]
                println!(\"{}\", x);
            }"
        }
        .to_string();

        spec.set_cursor_excerpt_with_selection(excerpt, offset..offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt_with_selection().unwrap(),
            (excerpt.to_string(), offset..offset)
        );

        // Cursor at beginning of the line with `let`
        let offset = excerpt.find("    let").unwrap();
        let position_string = indoc! {"
            fn main() {
                let x = 42;
            // <[CURSOR_POSITION]
                println!(\"{}\", x);
            }"
        }
        .to_string();

        spec.set_cursor_excerpt_with_selection(excerpt, offset..offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt_with_selection().unwrap(),
            (excerpt.to_string(), offset..offset)
        );

        // Cursor at end of line, after the semicolon
        let offset = excerpt.find(';').unwrap() + 1;
        let position_string = indoc! {"
            fn main() {
                let x = 42;
                //         ^[CURSOR_POSITION]
                println!(\"{}\", x);
            }"
        }
        .to_string();

        spec.set_cursor_excerpt_with_selection(excerpt, offset..offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt_with_selection().unwrap(),
            (excerpt.to_string(), offset..offset)
        );

        // Caret at end of file (no trailing newline)
        let excerpt = indoc! {"
            fn main() {
                let x = 42;"
        };
        let offset = excerpt.find(';').unwrap() + 1;
        let position_string = indoc! {"
            fn main() {
                let x = 42;
                //         ^[CURSOR_POSITION]"
        }
        .to_string();

        spec.set_cursor_excerpt_with_selection(excerpt, offset..offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt_with_selection().unwrap(),
            (excerpt.to_string(), offset..offset)
        );
    }

    #[test]
    fn test_cursor_excerpt_with_inline_marker() {
        let mut spec = ExampleSpec {
            name: String::new(),
            repository_url: String::new(),
            revision: String::new(),
            tags: Vec::new(),
            reasoning: None,
            uncommitted_diff: String::new(),
            cursor_path: Path::new("test.rs").into(),
            cursor_position: String::new(),
            edit_history: String::new(),
            expected_patches: Vec::new(),
            rejected_patch: None,
            captured_prompt_input: None,
            telemetry: None,
            human_feedback: Vec::new(),
            rating: None,
        };

        // Cursor before `42` using inline marker
        spec.cursor_position = indoc! {"
            fn main() {
                let x = <|user_cursor|>42;
                println!(\"{}\", x);
            }"
        }
        .to_string();

        let expected_excerpt = indoc! {"
            fn main() {
                let x = 42;
                println!(\"{}\", x);
            }"
        };
        let expected_offset = expected_excerpt.find("42").unwrap();

        assert_eq!(
            spec.cursor_excerpt_with_selection().unwrap(),
            (
                expected_excerpt.to_string(),
                expected_offset..expected_offset
            )
        );

        // Cursor at beginning of line
        spec.cursor_position = indoc! {"
            fn main() {
            <|user_cursor|>    let x = 42;
            }"
        }
        .to_string();

        let expected_excerpt = indoc! {"
            fn main() {
                let x = 42;
            }"
        };
        let expected_offset = expected_excerpt.find("    let").unwrap();

        assert_eq!(
            spec.cursor_excerpt_with_selection().unwrap(),
            (
                expected_excerpt.to_string(),
                expected_offset..expected_offset
            )
        );

        // Cursor at end of file
        spec.cursor_position = "fn main() {}<|user_cursor|>".to_string();
        let expected_excerpt = "fn main() {}";
        let expected_offset = expected_excerpt.len();

        assert_eq!(
            spec.cursor_excerpt_with_selection().unwrap(),
            (
                expected_excerpt.to_string(),
                expected_offset..expected_offset
            )
        );
    }

    #[test]
    fn test_expected_patches_with_selections() {
        let mut spec = ExampleSpec {
            name: String::new(),
            repository_url: String::new(),
            revision: String::new(),
            tags: Vec::new(),
            reasoning: None,
            uncommitted_diff: String::new(),
            cursor_path: Path::new("test.rs").into(),
            cursor_position: String::new(),
            edit_history: String::new(),
            expected_patches: Vec::new(),
            rejected_patch: None,
            captured_prompt_input: None,
            telemetry: None,
            human_feedback: Vec::new(),
            rating: None,
        };

        // Test cursor-only (empty selection).
        let new_content = indoc! {r#"
            // prints a greeting
            fn main() {
                println!("hello, {}", );
                let x = 42;
            }
        "#};
        let cursor_offset = new_content.find(");").unwrap();

        let clean_patch = indoc! {r#"
            --- a/test.rs
            +++ b/test.rs
            @@ -1,3 +1,4 @@
            +// prints a greeting
             fn main() {
            -    println!("hi");
            +    println!("hello, {}", );
                 let x = 42;
             }
        "#}
        .to_string();

        let encoded_patch = indoc! {r#"
            --- a/test.rs
            +++ b/test.rs
            @@ -1,3 +1,4 @@
            +// prints a greeting
             fn main() {
            -    println!("hi");
            +    println!("hello, {}", );
            #                          ^[CURSOR_POSITION]
                 let x = 42;
             }
        "#}
        .to_string();

        spec.set_expected_patches_with_selections(vec![(
            clean_patch.clone(),
            Some(cursor_offset..cursor_offset),
        )]);
        assert_eq!(spec.expected_patches, vec![encoded_patch]);

        let results = spec.expected_patches_with_selections();
        assert_eq!(
            results,
            vec![(clean_patch.clone(), Some(cursor_offset..cursor_offset))]
        );

        spec.set_expected_patches_with_selections(vec![(clean_patch.clone(), None)]);
        assert_eq!(spec.expected_patches, vec![clean_patch.clone()]);

        let results = spec.expected_patches_with_selections();
        assert_eq!(results, vec![(clean_patch, None)]);
    }

    #[test]
    fn test_expected_patches_with_non_empty_selection() {
        let mut spec = ExampleSpec {
            name: String::new(),
            repository_url: String::new(),
            revision: String::new(),
            tags: Vec::new(),
            reasoning: None,
            uncommitted_diff: String::new(),
            cursor_path: Path::new("test.py").into(),
            cursor_position: String::new(),
            edit_history: String::new(),
            expected_patches: Vec::new(),
            rejected_patch: None,
            captured_prompt_input: None,
            telemetry: None,
            human_feedback: Vec::new(),
            rating: None,
        };

        // The new file content after applying the diff.
        let new_content = indoc! {r#"
            from __future__ import annotations

            import logging
            import module
            from werkzeug.local import LocalProxy

            from .globals import request
        "#};

        // Find "module" in the line "import module".
        let module_line_start = new_content.find("import module").unwrap();
        let selection_start = module_line_start + "import ".len(); // Start of "module".
        let selection_end = module_line_start + "import module".len(); // End of "module".

        let clean_patch = indoc! {r#"
            --- a/src/flask/logging.py
            +++ b/src/flask/logging.py
            @@ -1,7 +1,8 @@
             from __future__ import annotations

             import logging
            -imfrom werkzeug.local import LocalProxy
            +import module
            +from werkzeug.local import LocalProxy

             from .globals import request
        "#}
        .to_string();

        let encoded_patch = indoc! {r#"
            --- a/src/flask/logging.py
            +++ b/src/flask/logging.py
            @@ -1,7 +1,8 @@
             from __future__ import annotations

             import logging
            -imfrom werkzeug.local import LocalProxy
            +import module
            #       ------^[SELECTION]
            +from werkzeug.local import LocalProxy

             from .globals import request
        "#}
        .to_string();

        spec.set_expected_patches_with_selections(vec![(
            clean_patch.clone(),
            Some(selection_start..selection_end),
        )]);
        assert_eq!(spec.expected_patches, vec![encoded_patch]);

        let results = spec.expected_patches_with_selections();
        assert_eq!(
            results,
            vec![(clean_patch, Some(selection_start..selection_end))]
        );
    }

    #[test]
    fn test_cursor_excerpt_with_non_empty_selection() {
        let mut spec = ExampleSpec {
            name: String::new(),
            repository_url: String::new(),
            revision: String::new(),
            tags: Vec::new(),
            reasoning: None,
            uncommitted_diff: String::new(),
            cursor_path: Path::new("test.py").into(),
            cursor_position: String::new(),
            edit_history: String::new(),
            expected_patches: Vec::new(),
            rejected_patch: None,
            captured_prompt_input: None,
            telemetry: None,
            human_feedback: Vec::new(),
            rating: None,
        };

        // Test selection in the middle of a line: "module" is selected.
        let excerpt = indoc! {"
            import module
            from other import thing"
        };
        let selection_start = "import ".len(); // Start of "module".
        let selection_end = "import module".len(); // End of "module".

        let position_string = indoc! {"
            import module
            #      ------^[SELECTION]
            from other import thing"
        }
        .to_string();

        spec.set_cursor_excerpt_with_selection(excerpt, selection_start..selection_end, "#");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt_with_selection().unwrap(),
            (excerpt.to_string(), selection_start..selection_end)
        );

        // Test selection starting at column 0.
        let excerpt = indoc! {"
            this_whole_thing_is_selected
            next line"
        };
        let selection_start = 0;
        let selection_end = "this_whole_thing_is_selected".len();

        let position_string = indoc! {"
            this_whole_thing_is_selected
            #<--------------------------^[SELECTION]
            next line"
        }
        .to_string();

        spec.set_cursor_excerpt_with_selection(excerpt, selection_start..selection_end, "#");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt_with_selection().unwrap(),
            (excerpt.to_string(), selection_start..selection_end)
        );
    }
}
