use crate::udiff::DiffLine;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Write as _, mem, ops::Range, path::Path, sync::Arc};
use telemetry_events::EditPredictionRating;

pub const CURSOR_POSITION_MARKER: &str = "[CURSOR_POSITION]";
pub const SELECTION_MARKER: &str = "[SELECTION]";
pub const INLINE_CURSOR_MARKER: &str = "<|user_cursor|>";
pub const INLINE_SELECTION_START_MARKER: &str = "<|selection_start|>";

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
    pub in_open_source_repo: bool,
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
            in_open_source_repo: false,
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

    /// Returns the excerpt of text around the cursor, and the selection range within that excerpt.
    ///
    /// The cursor's position is marked with a special comment that appears
    /// below the cursor line, which contains the string `[CURSOR_POSITION]`,
    /// preceded by an arrow marking the cursor's column. The arrow can be
    /// either:
    /// - `^` - The cursor column is at the position of the `^` character (pointing up to the cursor)
    /// - `<` - The cursor column is at the first non-whitespace character on that line.
    ///
    /// For backwards compatibility, this also supports returning a cursor position (empty selection).
    pub fn cursor_excerpt_with_selection(&self) -> Result<(String, Option<Range<usize>>)> {
        let input = &self.cursor_position;

        // Check for inline cursor markers first
        if input.contains(INLINE_CURSOR_MARKER) {
            let selections = extract_inline_selections(input);
            let excerpt = input
                .replace(INLINE_SELECTION_START_MARKER, "")
                .replace(INLINE_CURSOR_MARKER, "");
            return Ok((excerpt, selections.into_iter().next()));
        }

        if !input.contains(CURSOR_POSITION_MARKER) && !input.contains(SELECTION_MARKER) {
            return Ok((input.clone(), None));
        }

        let mut selection = None;
        let mut clean_lines: Vec<&str> = Vec::new();
        let mut clean_byte_offset = 0usize;
        let mut last_content_line_start = 0usize;

        for line in input.split('\n') {
            let is_marker = (line.contains(CURSOR_POSITION_MARKER)
                || line.contains(SELECTION_MARKER))
                && (line.contains('^') || line.contains('<'));

            if is_marker {
                if selection.is_none() {
                    let (cursor_column, selection_start_column) = parse_marker_line_columns(line)?;
                    let cursor_offset = last_content_line_start + cursor_column;
                    let selection_start_offset = last_content_line_start + selection_start_column;
                    selection = Some(selection_start_offset..cursor_offset);
                }
            } else {
                if !clean_lines.is_empty() {
                    clean_byte_offset += 1;
                }
                last_content_line_start = clean_byte_offset;
                clean_byte_offset += line.len();
                clean_lines.push(line);
            }
        }

        let excerpt = clean_lines.join("\n");
        let excerpt = excerpt.trim_end_matches('\n').to_string();

        Ok((excerpt, selection))
    }

    /// Sets the cursor position excerpt from a plain excerpt and a single selection range.
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
        // If the selection spans multiple lines, fall back to inline markers.
        if selection.start != selection.end {
            let start_line = excerpt[..selection.start].matches('\n').count();
            let end_line = excerpt[..selection.end].matches('\n').count();
            if start_line != end_line {
                self.cursor_position = embed_inline_selections(excerpt, &[selection]);
                return;
            }
        }

        // Compute line boundaries: (start_byte, end_byte) for each line's content
        // (end_byte is exclusive and does NOT include the '\n').
        let mut line_ranges: Vec<Range<usize>> = Vec::new();
        let mut offset = 0;
        for line in excerpt.split('\n') {
            let end = offset + line.len();
            line_ranges.push(offset..end);
            offset = end + 1;
        }

        let cursor_line_index = line_ranges
            .iter()
            .position(|r| selection.end >= r.start && selection.end <= r.end)
            .unwrap_or(line_ranges.len() - 1);

        let mut result = String::new();
        for (line_index, line_range) in line_ranges.iter().enumerate() {
            if line_index > 0 {
                result.push('\n');
            }
            result.push_str(&excerpt[line_range.clone()]);

            if line_index == cursor_line_index {
                let cursor_line = &excerpt[line_range.clone()];
                let cursor_line_indent =
                    &cursor_line[..cursor_line.len() - cursor_line.trim_start().len()];
                result.push('\n');
                let marker = build_marker_line(
                    &selection,
                    line_range.start,
                    cursor_line_indent,
                    line_comment_prefix,
                );
                result.push_str(&marker);
            }
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
    pub fn expected_patches_with_selections(&self) -> Vec<(String, Vec<Range<usize>>)> {
        self.expected_patches
            .iter()
            .map(|patch| {
                let mut clean_patch = String::new();
                let mut selections: Vec<Range<usize>> = Vec::new();
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
                            selections.push(selection_start..cursor_offset);
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

                (clean_patch, selections)
            })
            .collect()
    }

    pub fn set_expected_patches_with_selections(
        &mut self,
        patches: Vec<(String, Vec<Range<usize>>)>,
    ) {
        self.expected_patches = patches
            .into_iter()
            .map(|(patch, selections)| {
                if selections.is_empty() {
                    return patch;
                }

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

                            for selection in &selections {
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

fn build_marker_line(
    selection: &Range<usize>,
    line_start_offset: usize,
    line_indent: &str,
    line_comment_prefix: &str,
) -> String {
    let cursor_offset = selection.end;
    let selection_start = selection.start;
    let is_empty_selection = selection.start == selection.end;
    let cursor_column = cursor_offset - line_start_offset;
    let selection_start_column = selection_start.saturating_sub(line_start_offset);

    let mut marker = String::new();
    if selection_start_column == 0 && !is_empty_selection {
        marker.push_str(line_comment_prefix);
        marker.push('<');
        for _ in "<^".len()..cursor_column {
            marker.push('-');
        }
        marker.push('^');
        marker.push_str(SELECTION_MARKER);
    } else if cursor_column < line_comment_prefix.len() && is_empty_selection {
        for _ in 0..cursor_column {
            marker.push(' ');
        }
        marker.push_str(line_comment_prefix);
        write!(marker, " <{}", CURSOR_POSITION_MARKER).ok();
    } else {
        if cursor_column >= line_indent.len() + line_comment_prefix.len() {
            marker.push_str(line_indent);
        }
        marker.push_str(line_comment_prefix);

        for i in marker.len()..cursor_column {
            if i >= selection_start_column && !is_empty_selection {
                marker.push('-');
            } else {
                marker.push(' ');
            }
        }

        marker.push('^');
        if is_empty_selection {
            marker.push_str(CURSOR_POSITION_MARKER);
        } else {
            marker.push_str(SELECTION_MARKER);
        }
    }
    marker
}

fn parse_marker_line_columns(line: &str) -> Result<(usize, usize)> {
    if let Some(caret_pos) = line.find('^') {
        let dashes_before = line[..caret_pos]
            .chars()
            .rev()
            .take_while(|&c| c == '-')
            .count();

        let before_dashes_pos = caret_pos - dashes_before;
        let has_left_angle_bracket =
            before_dashes_pos > 0 && line[..before_dashes_pos].ends_with('<');

        let selection_start_col = if has_left_angle_bracket {
            0
        } else {
            caret_pos - dashes_before
        };

        Ok((caret_pos, selection_start_col))
    } else if let Some(_) = line.find('<') {
        let first_non_whitespace = line.find(|c: char| !c.is_whitespace()).unwrap_or(0);
        Ok((first_non_whitespace, first_non_whitespace))
    } else {
        anyhow::bail!("cursor position marker line must contain '^' or '<' before [SELECTION]");
    }
}

/// Extract all selection/cursor ranges from text containing inline
/// `<|selection_start|>` and `<|user_cursor|>` markers.
///
/// Returns byte-offset ranges in the marker-stripped text. Each range
/// is `start..end` where `start == end` means an empty cursor.
fn extract_inline_selections(text: &str) -> Vec<Range<usize>> {
    #[derive(Clone, Copy, PartialEq)]
    enum Kind {
        SelectionStart,
        UserCursor,
    }

    let sel_marker = INLINE_SELECTION_START_MARKER;
    let cur_marker = INLINE_CURSOR_MARKER;

    let mut markers: Vec<(usize, Kind)> = Vec::new();
    let mut pos = 0;
    while pos < text.len() {
        if text[pos..].starts_with(sel_marker) {
            markers.push((pos, Kind::SelectionStart));
            pos += sel_marker.len();
        } else if text[pos..].starts_with(cur_marker) {
            markers.push((pos, Kind::UserCursor));
            pos += cur_marker.len();
        } else {
            pos += 1;
        }
    }

    let mut clean_offsets = Vec::with_capacity(markers.len());
    let mut removed_bytes = 0usize;
    for &(raw_pos, kind) in &markers {
        clean_offsets.push(raw_pos - removed_bytes);
        removed_bytes += match kind {
            Kind::SelectionStart => sel_marker.len(),
            Kind::UserCursor => cur_marker.len(),
        };
    }

    let mut selections = Vec::new();
    let mut i = 0;
    while i < markers.len() {
        match markers[i].1 {
            Kind::SelectionStart => {
                if i + 1 < markers.len() && markers[i + 1].1 == Kind::UserCursor {
                    selections.push(clean_offsets[i]..clean_offsets[i + 1]);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            Kind::UserCursor => {
                if i + 1 < markers.len() && markers[i + 1].1 == Kind::SelectionStart {
                    selections.push(clean_offsets[i]..clean_offsets[i + 1]);
                    i += 2;
                } else {
                    selections.push(clean_offsets[i]..clean_offsets[i]);
                    i += 1;
                }
            }
        }
    }

    selections
}

/// Embed inline `<|selection_start|>` and `<|user_cursor|>` markers into
/// an excerpt for the given selection ranges.
fn embed_inline_selections(excerpt: &str, selections: &[Range<usize>]) -> String {
    // Collect insertion points sorted in reverse order so earlier insertions
    // don't shift later offsets.
    let mut insertions: Vec<(usize, &str)> = Vec::new();
    for selection in selections {
        if selection.start == selection.end {
            insertions.push((selection.start, INLINE_CURSOR_MARKER));
        } else {
            insertions.push((selection.end, INLINE_CURSOR_MARKER));
            insertions.push((selection.start, INLINE_SELECTION_START_MARKER));
        }
    }
    insertions.sort_by(|a, b| b.0.cmp(&a.0));

    let mut result = excerpt.to_string();
    for (offset, marker) in insertions {
        let clamped = offset.min(result.len());
        result.insert_str(clamped, marker);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use util::test::marked_text_ranges;

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
            (excerpt.to_string(), Some(offset..offset))
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
            (excerpt.to_string(), Some(offset..offset))
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
            (excerpt.to_string(), Some(offset..offset))
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
            (excerpt.to_string(), Some(offset..offset))
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
            (excerpt.to_string(), Some(offset..offset))
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
            (excerpt.to_string(), Some(offset..offset))
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
                Some(expected_offset..expected_offset)
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
                Some(expected_offset..expected_offset)
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
                Some(expected_offset..expected_offset)
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
            vec![cursor_offset..cursor_offset],
        )]);
        assert_eq!(spec.expected_patches, vec![encoded_patch]);

        let results = spec.expected_patches_with_selections();
        assert_eq!(
            results,
            vec![(clean_patch.clone(), vec![cursor_offset..cursor_offset])]
        );

        spec.set_expected_patches_with_selections(vec![(clean_patch.clone(), vec![])]);
        assert_eq!(spec.expected_patches, vec![clean_patch.clone()]);

        let results = spec.expected_patches_with_selections();
        assert_eq!(results, vec![(clean_patch, vec![])]);
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
            vec![selection_start..selection_end],
        )]);
        assert_eq!(spec.expected_patches, vec![encoded_patch]);

        let results = spec.expected_patches_with_selections();
        assert_eq!(
            results,
            vec![(clean_patch, vec![selection_start..selection_end])]
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
            (excerpt.to_string(), Some(selection_start..selection_end))
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
            (excerpt.to_string(), Some(selection_start..selection_end))
        );
    }

    fn empty_spec() -> ExampleSpec {
        ExampleSpec {
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
        }
    }

    #[test]
    fn test_round_trip_single_cursor() {
        let cases: &[(&str, &str, &str, &str)] = &[
            (
                "cursor mid-line indented",
                indoc! {"
                    fn main() {
                        let x = ˇ42;
                    }"},
                "//",
                indoc! {"
                    fn main() {
                        let x = 42;
                        //      ^[CURSOR_POSITION]
                    }"},
            ),
            (
                "cursor at column 0",
                indoc! {"
                    fn main() {
                    ˇ    let x = 42;
                    }"},
                "//",
                indoc! {"
                    fn main() {
                        let x = 42;
                    // <[CURSOR_POSITION]
                    }"},
            ),
            // Column 1 is still < "//".len(), so it also uses the `<` format
            (
                "cursor at column 1 (within comment prefix)",
                indoc! {"
                    fn main() {
                     ˇ   let x = 42;
                    }"},
                "//",
                indoc! {"
                    fn main() {
                        let x = 42;
                     // <[CURSOR_POSITION]
                    }"},
            ),
            (
                "cursor at end of line",
                indoc! {"
                    fn main() {
                        let x = 42;ˇ
                    }"},
                "//",
                indoc! {"
                    fn main() {
                        let x = 42;
                        //         ^[CURSOR_POSITION]
                    }"},
            ),
            (
                "cursor on first line",
                indoc! {"
                    fn ˇmain() {
                        let x = 42;
                    }"},
                "//",
                indoc! {"
                    fn main() {
                    // ^[CURSOR_POSITION]
                        let x = 42;
                    }"},
            ),
            (
                "cursor on last line (col 0)",
                indoc! {"
                    fn main() {
                        let x = 42;
                    ˇ}"},
                "//",
                indoc! {"
                    fn main() {
                        let x = 42;
                    }
                    // <[CURSOR_POSITION]"},
            ),
            (
                "single line cursor in middle",
                "helloˇ world",
                "//",
                "hello world\n//   ^[CURSOR_POSITION]",
            ),
            (
                "single line cursor at offset 0",
                "ˇhello world",
                "//",
                "hello world\n// <[CURSOR_POSITION]",
            ),
            (
                "single line cursor at end",
                "hello worldˇ",
                "//",
                "hello world\n//         ^[CURSOR_POSITION]",
            ),
            (
                "cursor on empty line",
                "a\nˇ\nb",
                "//",
                "a\n\n// <[CURSOR_POSITION]\nb",
            ),
            (
                "hash comment prefix",
                indoc! {"
                    def foo():
                        ˇpass"},
                "#",
                indoc! {"
                    def foo():
                        pass
                    #   ^[CURSOR_POSITION]"},
            ),
            (
                "double-dash comment prefix",
                "SELECT ˇ* FROM t",
                "--",
                "SELECT * FROM t\n--     ^[CURSOR_POSITION]",
            ),
        ];

        for (name, marked_excerpt, comment_prefix, expected_encoded) in cases {
            let (excerpt, selections) = marked_text_ranges(marked_excerpt, false);
            let selection = selections.into_iter().next().unwrap();
            let mut spec = empty_spec();

            spec.set_cursor_excerpt_with_selection(&excerpt, selection.clone(), comment_prefix);
            assert_eq!(
                spec.cursor_position, *expected_encoded,
                "case {name:?}: encoded form mismatch"
            );

            let (parsed_excerpt, parsed_selection) = spec
                .cursor_excerpt_with_selection()
                .unwrap_or_else(|e| panic!("case {name:?}: parse failed: {e}"));
            assert_eq!(
                parsed_excerpt, excerpt,
                "case {name:?}: round-trip excerpt mismatch"
            );
            assert_eq!(
                parsed_selection,
                Some(selection),
                "case {name:?}: round-trip selection mismatch"
            );
        }
    }

    #[test]
    fn test_round_trip_single_selection() {
        let cases: &[(&str, &str, &str, &str)] = &[
            (
                "selection mid-line",
                indoc! {"
                    import «module»
                    from other import thing"},
                "#",
                indoc! {"
                    import module
                    #      ------^[SELECTION]
                    from other import thing"},
            ),
            (
                "selection from column 0",
                indoc! {"
                    «import module»
                    from other import thing"},
                "#",
                indoc! {"
                    import module
                    #<-----------^[SELECTION]
                    from other import thing"},
            ),
            (
                "selection covering full second line from col 0",
                indoc! {"
                    import module
                    «from other import thing»"},
                "#",
                indoc! {"
                    import module
                    from other import thing
                    #<---------------------^[SELECTION]"},
            ),
            (
                "multi-line selection uses inline markers",
                indoc! {"
                    import «module
                    from other import »thing"},
                "#",
                &format!(
                    "import {INLINE_SELECTION_START_MARKER}module\nfrom other import {INLINE_CURSOR_MARKER}thing"
                ),
            ),
        ];

        for (name, marked_excerpt, comment_prefix, expected_encoded) in cases {
            let (excerpt, selections) = marked_text_ranges(marked_excerpt, false);
            let selection = selections.into_iter().next().unwrap();
            let mut spec = empty_spec();

            spec.set_cursor_excerpt_with_selection(&excerpt, selection.clone(), comment_prefix);
            assert_eq!(
                spec.cursor_position, *expected_encoded,
                "case {name:?}: encoded form mismatch"
            );

            let (parsed_excerpt, parsed_selection) = spec
                .cursor_excerpt_with_selection()
                .unwrap_or_else(|e| panic!("case {name:?}: parse failed: {e}"));
            assert_eq!(
                parsed_excerpt, excerpt,
                "case {name:?}: round-trip excerpt mismatch"
            );
            assert_eq!(
                parsed_selection,
                Some(selection),
                "case {name:?}: round-trip selection mismatch"
            );
        }
    }

    #[test]
    fn test_parse_inline_markers() {
        let cases: &[(&str, &str, &str, Option<Range<usize>>)] = &[
            (
                "single inline cursor",
                &format!("let x = {INLINE_CURSOR_MARKER}42;"),
                "let x = 42;",
                Some(8..8),
            ),
            (
                "inline cursor at start of file",
                &format!("{INLINE_CURSOR_MARKER}hello"),
                "hello",
                Some(0..0),
            ),
            (
                "inline cursor at end of file",
                &format!("hello{INLINE_CURSOR_MARKER}"),
                "hello",
                Some(5..5),
            ),
            (
                "inline selection",
                &format!("let x = {INLINE_SELECTION_START_MARKER}42{INLINE_CURSOR_MARKER};"),
                "let x = 42;",
                Some(8..10),
            ),
            (
                "inline selection spanning multiple lines",
                &format!(
                    "line1\n{INLINE_SELECTION_START_MARKER}line2\nli{INLINE_CURSOR_MARKER}ne3"
                ),
                "line1\nline2\nline3",
                Some(6..14),
            ),
        ];

        for (name, cursor_position, expected_excerpt, expected_selection) in cases {
            let mut spec = empty_spec();
            spec.cursor_position = cursor_position.to_string();

            let (parsed_excerpt, parsed_selection) = spec
                .cursor_excerpt_with_selection()
                .unwrap_or_else(|e| panic!("case {name:?}: parse failed: {e}"));
            assert_eq!(
                parsed_excerpt, *expected_excerpt,
                "case {name:?}: excerpt mismatch"
            );
            assert_eq!(
                parsed_selection, *expected_selection,
                "case {name:?}: selection mismatch"
            );
        }
    }

    #[test]
    fn test_parse_no_markers_returns_none() {
        let mut spec = empty_spec();
        spec.cursor_position = "just plain text\nno markers here".to_string();
        let (excerpt, selection) = spec.cursor_excerpt_with_selection().unwrap();
        assert_eq!(excerpt, "just plain text\nno markers here");
        assert_eq!(selection, None);
    }

    #[test]
    fn test_round_trip_deeply_indented_excerpt() {
        let cases: &[(&str, &str, &str)] = &[
            (
                "cursor on deeply indented line",
                indoc! {"
                    fn process(items: &[Item]) -> Result<()> {
                        for item in items {
                            if item.is_valid() {
                                item.execute()ˇ?;
                            }
                        }
                        Ok(())
                    }"},
                "//",
            ),
            (
                "selection on deeply indented line",
                indoc! {"
                    fn process(items: &[Item]) -> Result<()> {
                        for item in items {
                            if item.«is_valid»() {
                                item.execute()?;
                            }
                        }
                        Ok(())
                    }"},
                "//",
            ),
        ];

        for (name, marked_excerpt, comment_prefix) in cases {
            let (excerpt, selections) = marked_text_ranges(marked_excerpt, false);
            let selection = selections.into_iter().next().unwrap();
            let mut spec = empty_spec();

            spec.set_cursor_excerpt_with_selection(&excerpt, selection.clone(), comment_prefix);

            let (parsed_excerpt, parsed_selection) = spec
                .cursor_excerpt_with_selection()
                .unwrap_or_else(|e| panic!("case {name:?}: parse failed: {e}"));
            assert_eq!(
                parsed_excerpt, excerpt,
                "case {name:?}: round-trip excerpt mismatch"
            );
            assert_eq!(
                parsed_selection,
                Some(selection),
                "case {name:?}: round-trip selection mismatch"
            );
        }
    }
}
