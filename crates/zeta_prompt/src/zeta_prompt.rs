use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use strum::{EnumIter, IntoEnumIterator as _, IntoStaticStr};

pub const CURSOR_MARKER: &str = "<|user_cursor|>";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZetaPromptInput {
    pub cursor_path: Arc<Path>,
    pub cursor_excerpt: Arc<str>,
    pub editable_range_in_excerpt: Range<usize>,
    pub cursor_offset_in_excerpt: usize,
    pub events: Vec<Arc<Event>>,
    pub related_files: Vec<RelatedFile>,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, EnumIter, IntoStaticStr)]
#[allow(non_camel_case_types)]
pub enum ZetaVersion {
    V0112_MiddleAtEnd,
    #[default]
    V0113_Ordered,
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

    fn options_as_string() -> String {
        ZetaVersion::iter()
            .map(|version| format!("- {}\n", <&'static str>::from(version)))
            .collect::<Vec<_>>()
            .concat()
    }

    pub fn default_as_string() -> String {
        <&'static str>::from(Self::default()).to_string()
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
    let mut prompt = String::new();
    write_related_files(&mut prompt, &input.related_files);
    write_edit_history_section(&mut prompt, input);

    match version {
        ZetaVersion::V0112_MiddleAtEnd => {
            v0112_middle_at_end::write_cursor_excerpt_section(&mut prompt, input);
        }
        ZetaVersion::V0113_Ordered => {
            v0113_ordered::write_cursor_excerpt_section(&mut prompt, input)
        }
    }

    prompt
}

pub fn write_related_files(prompt: &mut String, related_files: &[RelatedFile]) {
    for file in related_files {
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
    }
}

fn write_edit_history_section(prompt: &mut String, input: &ZetaPromptInput) {
    prompt.push_str("<|file_sep|>edit history\n");
    for event in &input.events {
        write_event(prompt, event);
    }
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
