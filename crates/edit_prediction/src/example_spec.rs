use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Write as _, mem, path::Path, sync::Arc};
use telemetry_events::EditPredictionRating;

pub use zeta_prompt::udiff::{
    CURSOR_POSITION_MARKER, encode_cursor_in_patch, extract_cursor_from_patch,
};

use crate::data_collection::format_cursor_excerpt;
pub const INLINE_CURSOR_MARKER: &str = "<|user_cursor|>";

/// Maximum cursor file size to capture (64KB).
/// Files larger than this will not have their content captured,
/// falling back to git-based loading.
pub const MAX_CURSOR_FILE_SIZE: usize = 64 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecentFile {
    pub path: Arc<Path>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor_position: Option<usize>,
}

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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recently_opened_files: Vec<RecentFile>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recently_viewed_files: Vec<RecentFile>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub uncommitted_diff_contains_edit_history: bool,
    pub cursor_path: Arc<Path>,
    pub cursor_position: String,
    pub edit_history: String,
    pub expected_patches: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected_patch: Option<String>,
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

const REASONING_HEADING: &str = "Reasoning";
const UNCOMMITTED_DIFF_HEADING: &str = "Uncommitted Diff";
const RECENTLY_OPENED_FILES_HEADING: &str = "Recently Opened Files";
const RECENTLY_VIEWED_FILES_HEADING: &str = "Recently Viewed Files";
const EDIT_HISTORY_HEADING: &str = "Edit History";
const CURSOR_POSITION_HEADING: &str = "Cursor Position";
const EXPECTED_PATCH_HEADING: &str = "Expected Patch";
const REJECTED_PATCH_HEADING: &str = "Rejected Patch";
const ACCEPTED_PREDICTION_MARKER: &str = "// User accepted prediction:";

fn write_path_list(markdown: &mut String, heading: &str, files: &[RecentFile]) {
    if files.is_empty() {
        return;
    }

    _ = writeln!(markdown, "## {heading}");
    _ = writeln!(markdown);
    _ = writeln!(markdown, "```");
    for file in files {
        _ = write!(markdown, "{}", file.path.display());
        if let Some(position) = file.cursor_position {
            _ = write!(markdown, "\t{position}");
        }
        _ = writeln!(markdown);
    }
    _ = writeln!(markdown, "```");
    markdown.push('\n');
}

fn parse_path_list(text: &str) -> Vec<RecentFile> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (path, cursor_position) = line
                .rsplit_once('\t')
                .map(|(path, position)| (path, position.parse().ok()))
                .unwrap_or((line, None));
            RecentFile {
                path: Path::new(path).into(),
                cursor_position,
            }
        })
        .collect()
}

#[derive(Serialize, Deserialize)]
struct FrontMatter<'a> {
    repository_url: Cow<'a, str>,
    revision: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    uncommitted_diff_requires_edit_history_rollback: bool,
}

fn is_false(value: &bool) -> bool {
    !*value
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
            uncommitted_diff_requires_edit_history_rollback: self
                .uncommitted_diff_contains_edit_history,
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

        write_path_list(
            &mut markdown,
            RECENTLY_OPENED_FILES_HEADING,
            &self.recently_opened_files,
        );
        write_path_list(
            &mut markdown,
            RECENTLY_VIEWED_FILES_HEADING,
            &self.recently_viewed_files,
        );

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
            recently_opened_files: Vec::new(),
            recently_viewed_files: Vec::new(),
            uncommitted_diff_contains_edit_history: false,
            cursor_path: Path::new("").into(),
            cursor_position: String::new(),
            edit_history: String::new(),
            expected_patches: Vec::new(),
            rejected_patch: None,
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
                spec.uncommitted_diff_contains_edit_history =
                    data.uncommitted_diff_requires_edit_history_rollback;
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
            RecentlyOpenedFiles,
            RecentlyViewedFiles,
            EditHistory,
            CursorPosition,
            ExpectedPatch,
            RejectedPatch,
            Other,
        }

        let mut current_section = Section::Start;
        let mut next_edit_predicted = false;

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
                    } else if title.eq_ignore_ascii_case(RECENTLY_OPENED_FILES_HEADING) {
                        Section::RecentlyOpenedFiles
                    } else if title.eq_ignore_ascii_case(RECENTLY_VIEWED_FILES_HEADING) {
                        Section::RecentlyViewedFiles
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
                    if current_section == Section::EditHistory
                        && text.trim() == ACCEPTED_PREDICTION_MARKER
                    {
                        next_edit_predicted = true;
                    }
                    text.clear();
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
                        Section::RecentlyOpenedFiles => {
                            spec.recently_opened_files = parse_path_list(&text);
                            text.clear();
                        }
                        Section::RecentlyViewedFiles => {
                            spec.recently_viewed_files = parse_path_list(&text);
                            text.clear();
                        }
                        Section::EditHistory => {
                            if next_edit_predicted {
                                spec.edit_history
                                    .push_str(&format!("{}\n", ACCEPTED_PREDICTION_MARKER));
                                next_edit_predicted = false;
                            }
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
        self.cursor_position = format_cursor_excerpt(excerpt, cursor_offset, line_comment_prefix);
    }

    /// Returns all of the possible expected patches for this example, each with an optional
    /// cursor offset.
    ///
    /// The cursor offset is an offset within the new text (after applying the patch), relative
    /// to the start of the hunk.
    ///
    /// In the serialized representation of this example, the cursor position is represented
    /// using a comment line in the diff, beginning with `#`, and containing a `[CURSOR_POSITION]`
    /// marker with the same format as the [`Self::cursor_excerpt`].
    pub fn expected_patches_with_cursor_positions(&self) -> Vec<(String, Option<usize>)> {
        self.expected_patches
            .iter()
            .map(|patch| extract_cursor_from_patch(patch))
            .collect()
    }

    pub fn set_expected_patches_with_cursor_positions(
        &mut self,
        patches: Vec<(String, Option<usize>)>,
    ) {
        self.expected_patches = patches
            .into_iter()
            .map(|(patch, cursor_offset)| encode_cursor_in_patch(&patch, cursor_offset))
            .collect();
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
            recently_opened_files: Vec::new(),
            recently_viewed_files: Vec::new(),
            uncommitted_diff_contains_edit_history: false,
            cursor_path: Path::new("test.rs").into(),
            cursor_position: String::new(),
            edit_history: String::new(),
            expected_patches: Vec::new(),
            rejected_patch: None,
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
            recently_opened_files: Vec::new(),
            recently_viewed_files: Vec::new(),
            uncommitted_diff_contains_edit_history: false,
            cursor_path: Path::new("test.rs").into(),
            cursor_position: String::new(),
            edit_history: String::new(),
            expected_patches: Vec::new(),
            rejected_patch: None,
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

    #[test]
    fn test_expected_patches_with_cursor_positions() {
        let mut spec = ExampleSpec {
            name: String::new(),
            repository_url: String::new(),
            revision: String::new(),
            tags: Vec::new(),
            reasoning: None,
            uncommitted_diff: String::new(),
            recently_opened_files: Vec::new(),
            recently_viewed_files: Vec::new(),
            uncommitted_diff_contains_edit_history: false,
            cursor_path: Path::new("test.rs").into(),
            cursor_position: String::new(),
            edit_history: String::new(),
            expected_patches: Vec::new(),
            rejected_patch: None,
            telemetry: None,
            human_feedback: Vec::new(),
            rating: None,
        };

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

        spec.set_expected_patches_with_cursor_positions(vec![(
            clean_patch.clone(),
            Some(cursor_offset),
        )]);
        assert_eq!(spec.expected_patches, vec![encoded_patch]);

        let results = spec.expected_patches_with_cursor_positions();
        assert_eq!(results, vec![(clean_patch.clone(), Some(cursor_offset))]);

        spec.set_expected_patches_with_cursor_positions(vec![(clean_patch.clone(), None)]);
        assert_eq!(spec.expected_patches, vec![clean_patch.clone()]);

        let results = spec.expected_patches_with_cursor_positions();
        assert_eq!(results, vec![(clean_patch, None)]);
    }

    #[test]
    fn test_encode_cursor_in_patch_is_idempotent() {
        let patch = indoc! {r#"
            --- a/test.rs
            +++ b/test.rs
            @@ -1,2 +1,2 @@
            -fn old() {}
            +fn new_name() {}
            #       ^[CURSOR_POSITION]
        "#};

        let cursor_offset = "fn new_name() {}".find("name").unwrap();
        let encoded_once = encode_cursor_in_patch(patch, Some(cursor_offset));
        let encoded_twice = encode_cursor_in_patch(&encoded_once, Some(cursor_offset));

        assert_eq!(encoded_once, encoded_twice);
        assert_eq!(
            encoded_once
                .lines()
                .filter(|line| line.contains(CURSOR_POSITION_MARKER))
                .count(),
            1
        );
    }

    #[test]
    fn test_from_markdown_accepted_prediction_marker() {
        let markdown = indoc! {r#"
            +++
            repository_url = "https://github.com/example/repo"
            revision = "abc123"
            +++

            ## Edit History

            ```diff
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,3 +1,3 @@
            -fn hello() {}
            +fn hello_world() {}
            ```

            // User accepted prediction:
            ```diff
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,3 +1,3 @@
            -fn hello_world() {}
            +fn hello_world() { println!("hi"); }
            ```

            ```diff
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,3 +1,3 @@
            -fn hello_world() { println!("hi"); }
            +fn hello_world() { println!("hello"); }
            ```

            ## Cursor Position

            ```src/main.rs
            fn hello_world() { println!("hello"); }
            #                                    ^[CURSOR_POSITION]
            ```

            ## Expected Patch

            ```diff
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,3 +1,3 @@
            -fn hello_world() { println!("hello"); }
            +fn hello_world() { println!("hello, world!"); }
            ```
        "#};

        let spec = ExampleSpec::from_markdown(markdown).unwrap();

        // The first diff should NOT have the marker
        assert!(spec.edit_history.starts_with("--- a/src/main.rs"));

        // The second diff should be preceded by the accepted prediction marker
        assert!(
            spec.edit_history
                .contains("// User accepted prediction:\n--- a/src/main.rs")
        );

        // Count occurrences of the marker - should be exactly one
        let marker_count = spec
            .edit_history
            .matches("// User accepted prediction:")
            .count();
        assert_eq!(marker_count, 1);

        // The third diff should NOT have the marker
        // Verify all three diffs are present
        let diff_count = spec.edit_history.matches("--- a/src/main.rs").count();
        assert_eq!(diff_count, 3);
    }
}
