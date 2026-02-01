use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use strum::{EnumIter, IntoEnumIterator as _, IntoStaticStr};

pub const CURSOR_MARKER: &str = "<|user_cursor|>";
pub const MAX_PROMPT_TOKENS: usize = 4096;

fn estimate_tokens(bytes: usize) -> usize {
    bytes / 3
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZetaPromptInput {
    pub cursor_path: Arc<Path>,
    pub cursor_excerpt: Arc<str>,
    pub editable_range_in_excerpt: Range<usize>,
    pub cursor_offset_in_excerpt: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excerpt_start_row: Option<u32>,
    pub events: Vec<Arc<Event>>,
    pub related_files: Vec<RelatedFile>,
}

#[derive(
    Default,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumIter,
    IntoStaticStr,
    Serialize,
    Deserialize,
)]
#[allow(non_camel_case_types)]
pub enum ZetaVersion {
    V0112MiddleAtEnd,
    V0113Ordered,
    #[default]
    V0114180EditableRegion,
    V0120GitMergeMarkers,
}

impl std::fmt::Display for ZetaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl ZetaVersion {
    pub fn parse(version_string: &str) -> Result<Self> {
        let mut results = ZetaVersion::iter().filter(|version| {
            <&'static str>::from(version)
                .to_lowercase()
                .contains(&version_string.to_lowercase())
        });
        let Some(result) = results.next() else {
            anyhow::bail!(
                "`{version_string}` did not match any of:\n{}",
                Self::options_as_string()
            );
        };
        if results.next().is_some() {
            anyhow::bail!(
                "`{version_string}` matched more than one of:\n{}",
                Self::options_as_string()
            );
        }
        Ok(result)
    }

    pub fn options_as_string() -> String {
        ZetaVersion::iter()
            .map(|version| format!("- {}\n", <&'static str>::from(version)))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Event {
    BufferChange {
        path: Arc<Path>,
        old_path: Arc<Path>,
        diff: String,
        predicted: bool,
        in_open_source_repo: bool,
    },
}

pub fn write_event(prompt: &mut String, event: &Event) {
    fn write_path_as_unix_str(prompt: &mut String, path: &Path) {
        for component in path.components() {
            prompt.push('/');
            write!(prompt, "{}", component.as_os_str().display()).ok();
        }
    }
    match event {
        Event::BufferChange {
            path,
            old_path,
            diff,
            predicted,
            in_open_source_repo: _,
        } => {
            if *predicted {
                prompt.push_str("// User accepted prediction:\n");
            }
            prompt.push_str("--- a");
            write_path_as_unix_str(prompt, old_path.as_ref());
            prompt.push_str("\n+++ b");
            write_path_as_unix_str(prompt, path.as_ref());
            prompt.push('\n');
            prompt.push_str(diff);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelatedFile {
    pub path: Arc<Path>,
    pub max_row: u32,
    pub excerpts: Vec<RelatedExcerpt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelatedExcerpt {
    pub row_range: Range<u32>,
    pub text: Arc<str>,
}

pub fn format_zeta_prompt(input: &ZetaPromptInput, version: ZetaVersion) -> String {
    format_zeta_prompt_with_budget(input, version, MAX_PROMPT_TOKENS)
}

fn format_zeta_prompt_with_budget(
    input: &ZetaPromptInput,
    version: ZetaVersion,
    max_tokens: usize,
) -> String {
    let mut cursor_section = String::new();
    match version {
        ZetaVersion::V0112MiddleAtEnd => {
            v0112_middle_at_end::write_cursor_excerpt_section(&mut cursor_section, input);
        }
        ZetaVersion::V0113Ordered | ZetaVersion::V0114180EditableRegion => {
            v0113_ordered::write_cursor_excerpt_section(&mut cursor_section, input)
        }
        ZetaVersion::V0120GitMergeMarkers => {
            v0120_git_merge_markers::write_cursor_excerpt_section(&mut cursor_section, input)
        }
    }

    let cursor_tokens = estimate_tokens(cursor_section.len());
    let budget_after_cursor = max_tokens.saturating_sub(cursor_tokens);

    let edit_history_section =
        format_edit_history_within_budget(&input.events, budget_after_cursor);
    let edit_history_tokens = estimate_tokens(edit_history_section.len());
    let budget_after_edit_history = budget_after_cursor.saturating_sub(edit_history_tokens);

    let related_files_section =
        format_related_files_within_budget(&input.related_files, budget_after_edit_history);

    let mut prompt = String::new();
    prompt.push_str(&related_files_section);
    prompt.push_str(&edit_history_section);
    prompt.push_str(&cursor_section);
    prompt
}

fn format_edit_history_within_budget(events: &[Arc<Event>], max_tokens: usize) -> String {
    let header = "<|file_sep|>edit history\n";
    let header_tokens = estimate_tokens(header.len());
    if header_tokens >= max_tokens {
        return String::new();
    }

    let mut event_strings: Vec<String> = Vec::new();
    let mut total_tokens = header_tokens;

    for event in events.iter().rev() {
        let mut event_str = String::new();
        write_event(&mut event_str, event);
        let event_tokens = estimate_tokens(event_str.len());

        if total_tokens + event_tokens > max_tokens {
            break;
        }
        total_tokens += event_tokens;
        event_strings.push(event_str);
    }

    if event_strings.is_empty() {
        return String::new();
    }

    let mut result = String::from(header);
    for event_str in event_strings.iter().rev() {
        result.push_str(&event_str);
    }
    result
}

fn format_related_files_within_budget(related_files: &[RelatedFile], max_tokens: usize) -> String {
    let mut result = String::new();
    let mut total_tokens = 0;

    for file in related_files {
        let path_str = file.path.to_string_lossy();
        let header_len = "<|file_sep|>".len() + path_str.len() + 1;
        let header_tokens = estimate_tokens(header_len);

        if total_tokens + header_tokens > max_tokens {
            break;
        }

        let mut file_tokens = header_tokens;
        let mut excerpts_to_include = 0;

        for excerpt in &file.excerpts {
            let needs_newline = !excerpt.text.ends_with('\n');
            let needs_ellipsis = excerpt.row_range.end < file.max_row;
            let excerpt_len = excerpt.text.len()
                + if needs_newline { "\n".len() } else { "".len() }
                + if needs_ellipsis {
                    "...\n".len()
                } else {
                    "".len()
                };

            let excerpt_tokens = estimate_tokens(excerpt_len);
            if total_tokens + file_tokens + excerpt_tokens > max_tokens {
                break;
            }
            file_tokens += excerpt_tokens;
            excerpts_to_include += 1;
        }

        if excerpts_to_include > 0 {
            total_tokens += file_tokens;
            write!(result, "<|file_sep|>{}\n", path_str).ok();
            for excerpt in file.excerpts.iter().take(excerpts_to_include) {
                result.push_str(&excerpt.text);
                if !result.ends_with('\n') {
                    result.push('\n');
                }
                if excerpt.row_range.end < file.max_row {
                    result.push_str("...\n");
                }
            }
        }
    }

    result
}

pub fn write_related_files(
    prompt: &mut String,
    related_files: &[RelatedFile],
) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    for file in related_files {
        let start = prompt.len();
        let path_str = file.path.to_string_lossy();
        write!(prompt, "<|file_sep|>{}\n", path_str).ok();
        for excerpt in &file.excerpts {
            prompt.push_str(&excerpt.text);
            if !prompt.ends_with('\n') {
                prompt.push('\n');
            }
            if excerpt.row_range.end < file.max_row {
                prompt.push_str("...\n");
            }
        }
        let end = prompt.len();
        ranges.push(start..end);
    }
    ranges
}

mod v0112_middle_at_end {
    use super::*;

    pub fn write_cursor_excerpt_section(prompt: &mut String, input: &ZetaPromptInput) {
        let path_str = input.cursor_path.to_string_lossy();
        write!(prompt, "<|file_sep|>{}\n", path_str).ok();

        prompt.push_str("<|fim_prefix|>\n");
        prompt.push_str(&input.cursor_excerpt[..input.editable_range_in_excerpt.start]);

        prompt.push_str("<|fim_suffix|>\n");
        prompt.push_str(&input.cursor_excerpt[input.editable_range_in_excerpt.end..]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_middle|>current\n");
        prompt.push_str(
            &input.cursor_excerpt
                [input.editable_range_in_excerpt.start..input.cursor_offset_in_excerpt],
        );
        prompt.push_str(CURSOR_MARKER);
        prompt.push_str(
            &input.cursor_excerpt
                [input.cursor_offset_in_excerpt..input.editable_range_in_excerpt.end],
        );
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_middle|>updated\n");
    }
}

mod v0113_ordered {
    use super::*;

    pub fn write_cursor_excerpt_section(prompt: &mut String, input: &ZetaPromptInput) {
        let path_str = input.cursor_path.to_string_lossy();
        write!(prompt, "<|file_sep|>{}\n", path_str).ok();

        prompt.push_str("<|fim_prefix|>\n");
        prompt.push_str(&input.cursor_excerpt[..input.editable_range_in_excerpt.start]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_middle|>current\n");
        prompt.push_str(
            &input.cursor_excerpt
                [input.editable_range_in_excerpt.start..input.cursor_offset_in_excerpt],
        );
        prompt.push_str(CURSOR_MARKER);
        prompt.push_str(
            &input.cursor_excerpt
                [input.cursor_offset_in_excerpt..input.editable_range_in_excerpt.end],
        );
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_suffix|>\n");
        prompt.push_str(&input.cursor_excerpt[input.editable_range_in_excerpt.end..]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_middle|>updated\n");
    }
}

pub mod v0120_git_merge_markers {
    //! A prompt that uses git-style merge conflict markers to represent the editable region.
    //!
    //! Example prompt:
    //!
    //! <|file_sep|>path/to/target_file.py
    //! <|fim_prefix|>
    //! code before editable region
    //! <|fim_suffix|>
    //! code after editable region
    //! <|fim_middle|>
    //! <<<<<<< CURRENT
    //! code that
    //! needs to<|user_cursor|>
    //! be rewritten
    //! =======
    //!
    //! Expected output (should be generated by the model):
    //!
    //! updated
    //! code with
    //! changes applied
    //! >>>>>>> UPDATED

    use super::*;

    pub const START_MARKER: &str = "<<<<<<< CURRENT\n";
    pub const SEPARATOR: &str = "=======\n";
    pub const END_MARKER: &str = ">>>>>>> UPDATED\n";

    pub fn write_cursor_excerpt_section(prompt: &mut String, input: &ZetaPromptInput) {
        let path_str = input.cursor_path.to_string_lossy();
        write!(prompt, "<|file_sep|>{}\n", path_str).ok();

        prompt.push_str("<|fim_prefix|>");
        prompt.push_str(&input.cursor_excerpt[..input.editable_range_in_excerpt.start]);

        prompt.push_str("<|fim_suffix|>");
        prompt.push_str(&input.cursor_excerpt[input.editable_range_in_excerpt.end..]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_middle|>");
        prompt.push_str(START_MARKER);
        prompt.push_str(
            &input.cursor_excerpt
                [input.editable_range_in_excerpt.start..input.cursor_offset_in_excerpt],
        );
        prompt.push_str(CURSOR_MARKER);
        prompt.push_str(
            &input.cursor_excerpt
                [input.cursor_offset_in_excerpt..input.editable_range_in_excerpt.end],
        );
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }
        prompt.push_str(SEPARATOR);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn make_input(
        cursor_excerpt: &str,
        editable_range: Range<usize>,
        cursor_offset: usize,
        events: Vec<Event>,
        related_files: Vec<RelatedFile>,
    ) -> ZetaPromptInput {
        ZetaPromptInput {
            cursor_path: Path::new("test.rs").into(),
            cursor_excerpt: cursor_excerpt.into(),
            editable_range_in_excerpt: editable_range,
            cursor_offset_in_excerpt: cursor_offset,
            excerpt_start_row: None,
            events: events.into_iter().map(Arc::new).collect(),
            related_files,
        }
    }

    fn make_event(path: &str, diff: &str) -> Event {
        Event::BufferChange {
            path: Path::new(path).into(),
            old_path: Path::new(path).into(),
            diff: diff.to_string(),
            predicted: false,
            in_open_source_repo: false,
        }
    }

    fn make_related_file(path: &str, content: &str) -> RelatedFile {
        RelatedFile {
            path: Path::new(path).into(),
            max_row: content.lines().count() as u32,
            excerpts: vec![RelatedExcerpt {
                row_range: 0..content.lines().count() as u32,
                text: content.into(),
            }],
        }
    }

    fn format_with_budget(input: &ZetaPromptInput, max_tokens: usize) -> String {
        format_zeta_prompt_with_budget(input, ZetaVersion::V0114180EditableRegion, max_tokens)
    }

    #[test]
    fn test_no_truncation_when_within_budget() {
        let input = make_input(
            "prefix\neditable\nsuffix",
            7..15,
            10,
            vec![make_event("a.rs", "-old\n+new\n")],
            vec![make_related_file("related.rs", "fn helper() {}\n")],
        );

        assert_eq!(
            format_with_budget(&input, 10000),
            indoc! {r#"
                <|file_sep|>related.rs
                fn helper() {}
                <|file_sep|>edit history
                --- a/a.rs
                +++ b/a.rs
                -old
                +new
                <|file_sep|>test.rs
                <|fim_prefix|>
                prefix
                <|fim_middle|>current
                edi<|user_cursor|>table
                <|fim_suffix|>

                suffix
                <|fim_middle|>updated
            "#}
        );
    }

    #[test]
    fn test_truncation_drops_edit_history_when_budget_tight() {
        let input = make_input(
            "code",
            0..4,
            2,
            vec![make_event("a.rs", "-x\n+y\n")],
            vec![
                make_related_file("r1.rs", "a\n"),
                make_related_file("r2.rs", "b\n"),
            ],
        );

        assert_eq!(
            format_with_budget(&input, 10000),
            indoc! {r#"
                <|file_sep|>r1.rs
                a
                <|file_sep|>r2.rs
                b
                <|file_sep|>edit history
                --- a/a.rs
                +++ b/a.rs
                -x
                +y
                <|file_sep|>test.rs
                <|fim_prefix|>
                <|fim_middle|>current
                co<|user_cursor|>de
                <|fim_suffix|>
                <|fim_middle|>updated
            "#}
        );

        assert_eq!(
            format_with_budget(&input, 50),
            indoc! {r#"
                <|file_sep|>r1.rs
                a
                <|file_sep|>r2.rs
                b
                <|file_sep|>test.rs
                <|fim_prefix|>
                <|fim_middle|>current
                co<|user_cursor|>de
                <|fim_suffix|>
                <|fim_middle|>updated
            "#}
        );
    }

    #[test]
    fn test_truncation_includes_partial_excerpts() {
        let input = make_input(
            "x",
            0..1,
            0,
            vec![],
            vec![RelatedFile {
                path: Path::new("big.rs").into(),
                max_row: 30,
                excerpts: vec![
                    RelatedExcerpt {
                        row_range: 0..10,
                        text: "first excerpt\n".into(),
                    },
                    RelatedExcerpt {
                        row_range: 10..20,
                        text: "second excerpt\n".into(),
                    },
                    RelatedExcerpt {
                        row_range: 20..30,
                        text: "third excerpt\n".into(),
                    },
                ],
            }],
        );

        assert_eq!(
            format_with_budget(&input, 10000),
            indoc! {r#"
                <|file_sep|>big.rs
                first excerpt
                ...
                second excerpt
                ...
                third excerpt
                <|file_sep|>test.rs
                <|fim_prefix|>
                <|fim_middle|>current
                <|user_cursor|>x
                <|fim_suffix|>
                <|fim_middle|>updated
            "#}
        );

        assert_eq!(
            format_with_budget(&input, 50),
            indoc! {r#"
                <|file_sep|>big.rs
                first excerpt
                ...
                <|file_sep|>test.rs
                <|fim_prefix|>
                <|fim_middle|>current
                <|user_cursor|>x
                <|fim_suffix|>
                <|fim_middle|>updated
            "#}
        );
    }

    #[test]
    fn test_truncation_drops_older_events_first() {
        let input = make_input(
            "x",
            0..1,
            0,
            vec![make_event("old.rs", "-1\n"), make_event("new.rs", "-2\n")],
            vec![],
        );

        assert_eq!(
            format_with_budget(&input, 10000),
            indoc! {r#"
                <|file_sep|>edit history
                --- a/old.rs
                +++ b/old.rs
                -1
                --- a/new.rs
                +++ b/new.rs
                -2
                <|file_sep|>test.rs
                <|fim_prefix|>
                <|fim_middle|>current
                <|user_cursor|>x
                <|fim_suffix|>
                <|fim_middle|>updated
            "#}
        );

        assert_eq!(
            format_with_budget(&input, 55),
            indoc! {r#"
                <|file_sep|>edit history
                --- a/new.rs
                +++ b/new.rs
                -2
                <|file_sep|>test.rs
                <|fim_prefix|>
                <|fim_middle|>current
                <|user_cursor|>x
                <|fim_suffix|>
                <|fim_middle|>updated
            "#}
        );
    }

    #[test]
    fn test_cursor_excerpt_always_included_with_minimal_budget() {
        let input = make_input(
            "fn main() {}",
            0..12,
            3,
            vec![make_event("a.rs", "-old\n+new\n")],
            vec![make_related_file("related.rs", "helper\n")],
        );

        assert_eq!(
            format_with_budget(&input, 30),
            indoc! {r#"
                <|file_sep|>test.rs
                <|fim_prefix|>
                <|fim_middle|>current
                fn <|user_cursor|>main() {}
                <|fim_suffix|>
                <|fim_middle|>updated
            "#}
        );
    }
}
