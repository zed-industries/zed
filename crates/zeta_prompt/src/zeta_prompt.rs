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
    let budget_after_cursor = MAX_PROMPT_TOKENS.saturating_sub(cursor_tokens);

    let edit_history_section = format_edit_history_within_budget(&input.events, budget_after_cursor);
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

    event_strings.reverse();

    let mut result = String::from(header);
    for event_str in event_strings {
        result.push_str(&event_str);
    }
    result
}

fn format_related_files_within_budget(related_files: &[RelatedFile], max_tokens: usize) -> String {
    let mut result = String::new();
    let mut total_tokens = 0;

    for file in related_files {
        let mut file_str = String::new();
        let path_str = file.path.to_string_lossy();
        write!(file_str, "<|file_sep|>{}\n", path_str).ok();
        for excerpt in &file.excerpts {
            file_str.push_str(&excerpt.text);
            if !file_str.ends_with('\n') {
                file_str.push('\n');
            }
            if excerpt.row_range.end < file.max_row {
                file_str.push_str("...\n");
            }
        }

        let file_tokens = estimate_tokens(file_str.len());
        if total_tokens + file_tokens > max_tokens {
            break;
        }
        total_tokens += file_tokens;
        result.push_str(&file_str);
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
