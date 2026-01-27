use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Write as _, mem, ops::Range, path::Path, sync::Arc};

pub const CURSOR_POSITION_MARKER: &str = "[CURSOR_POSITION]";
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
    pub fn cursor_excerpt(&self) -> Result<(String, usize)> {
        let input = &self.cursor_position;

        // Check for inline cursor marker first
        if let Some(inline_offset) = input.find(INLINE_CURSOR_MARKER) {
            let excerpt = input[..inline_offset].to_string()
                + &input[inline_offset + INLINE_CURSOR_MARKER.len()..];
            return Ok((excerpt, inline_offset));
        }

        let marker_offset = input
            .find(CURSOR_POSITION_MARKER)
            .context("missing [CURSOR_POSITION] marker")?;
        let marker_line_start = input[..marker_offset]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0);
        let marker_line_end = input[marker_line_start..]
            .find('\n')
            .map(|pos| marker_line_start + pos + 1)
            .unwrap_or(input.len());
        let marker_line = &input[marker_line_start..marker_line_end].trim_end_matches('\n');

        let cursor_column = if let Some(cursor_offset) = marker_line.find('^') {
            cursor_offset
        } else if let Some(less_than_pos) = marker_line.find('<') {
            marker_line
                .find(|c: char| !c.is_whitespace())
                .unwrap_or(less_than_pos)
        } else {
            anyhow::bail!(
                "cursor position marker line must contain '^' or '<' before [CURSOR_POSITION]"
            );
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

        Ok((excerpt, cursor_offset))
    }

    /// Sets the cursor position excerpt from a plain excerpt and cursor byte offset.
    ///
    /// The `line_comment_prefix` is used to format the marker line as a comment.
    /// If the cursor column is less than the comment prefix length, the `<` format is used.
    /// Otherwise, the `^` format is used.
    pub fn set_cursor_excerpt(
        &mut self,
        excerpt: &str,
        cursor_offset: usize,
        line_comment_prefix: &str,
    ) {
        // Find which line the cursor is on and its column
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

        // Build the marker line
        let mut marker_line = String::new();
        if cursor_column < line_comment_prefix.len() {
            for _ in 0..cursor_column {
                marker_line.push(' ');
            }
            marker_line.push_str(line_comment_prefix);
            write!(marker_line, " <{}", CURSOR_POSITION_MARKER).unwrap();
        } else {
            if cursor_column >= cursor_line_indent.len() + line_comment_prefix.len() {
                marker_line.push_str(cursor_line_indent);
            }
            marker_line.push_str(line_comment_prefix);
            while marker_line.len() < cursor_column {
                marker_line.push(' ');
            }
            write!(marker_line, "^{}", CURSOR_POSITION_MARKER).unwrap();
        }

        // Build the final cursor_position string
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_cursor_excerpt_with_caret() {
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

        spec.set_cursor_excerpt(excerpt, offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt().unwrap(),
            (excerpt.to_string(), offset)
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

        spec.set_cursor_excerpt(excerpt, offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt().unwrap(),
            (excerpt.to_string(), offset)
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

        spec.set_cursor_excerpt(excerpt, offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt().unwrap(),
            (excerpt.to_string(), offset)
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

        spec.set_cursor_excerpt(excerpt, offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt().unwrap(),
            (excerpt.to_string(), offset)
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

        spec.set_cursor_excerpt(excerpt, offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt().unwrap(),
            (excerpt.to_string(), offset)
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

        spec.set_cursor_excerpt(excerpt, offset, "//");
        assert_eq!(spec.cursor_position, position_string);
        assert_eq!(
            spec.cursor_excerpt().unwrap(),
            (excerpt.to_string(), offset)
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
            spec.cursor_excerpt().unwrap(),
            (expected_excerpt.to_string(), expected_offset)
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
            spec.cursor_excerpt().unwrap(),
            (expected_excerpt.to_string(), expected_offset)
        );

        // Cursor at end of file
        spec.cursor_position = "fn main() {}<|user_cursor|>".to_string();
        let expected_excerpt = "fn main() {}";
        let expected_offset = expected_excerpt.len();

        assert_eq!(
            spec.cursor_excerpt().unwrap(),
            (expected_excerpt.to_string(), expected_offset)
        );
    }
}
