use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;

pub const CURSOR_MARKER: &str = "<|user_cursor|>";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZetaPromptInput {
    pub cursor_path: Arc<Path>,
    pub cursor_excerpt: Arc<str>,
    pub editable_range_in_excerpt: Range<usize>,
    pub cursor_offset_in_excerpt: usize,
    pub events: Vec<Arc<Event>>,
    pub related_files: Arc<[RelatedFile]>,
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
    pub text: String,
}

pub fn format_zeta_prompt(input: &ZetaPromptInput) -> String {
    let mut prompt = String::new();
    write_related_files(&mut prompt, &input.related_files);
    write_edit_history_section(&mut prompt, input);
    write_cursor_excerpt_section(&mut prompt, input);
    prompt
}

pub fn write_related_files(prompt: &mut String, related_files: &[RelatedFile]) {
    push_delimited(prompt, "related_files", &[], |prompt| {
        for file in related_files {
            let path_str = file.path.to_string_lossy();
            push_delimited(prompt, "related_file", &[("path", &path_str)], |prompt| {
                for excerpt in &file.excerpts {
                    push_delimited(
                        prompt,
                        "related_excerpt",
                        &[(
                            "lines",
                            &format!(
                                "{}-{}",
                                excerpt.row_range.start + 1,
                                excerpt.row_range.end + 1
                            ),
                        )],
                        |prompt| {
                            prompt.push_str(&excerpt.text);
                            prompt.push('\n');
                        },
                    );
                }
            });
        }
    });
}

fn write_edit_history_section(prompt: &mut String, input: &ZetaPromptInput) {
    push_delimited(prompt, "edit_history", &[], |prompt| {
        if input.events.is_empty() {
            prompt.push_str("(No edit history)");
        } else {
            for event in &input.events {
                write_event(prompt, event);
            }
        }
    });
}

fn write_cursor_excerpt_section(prompt: &mut String, input: &ZetaPromptInput) {
    push_delimited(prompt, "cursor_excerpt", &[], |prompt| {
        let path_str = input.cursor_path.to_string_lossy();
        push_delimited(prompt, "file", &[("path", &path_str)], |prompt| {
            prompt.push_str(&input.cursor_excerpt[..input.editable_range_in_excerpt.start]);
            push_delimited(prompt, "editable_region", &[], |prompt| {
                prompt.push_str(
                    &input.cursor_excerpt
                        [input.editable_range_in_excerpt.start..input.cursor_offset_in_excerpt],
                );
                prompt.push_str(CURSOR_MARKER);
                prompt.push_str(
                    &input.cursor_excerpt
                        [input.cursor_offset_in_excerpt..input.editable_range_in_excerpt.end],
                );
            });
            prompt.push_str(&input.cursor_excerpt[input.editable_range_in_excerpt.end..]);
        });
    });
}

fn push_delimited(
    prompt: &mut String,
    tag: &'static str,
    arguments: &[(&str, &str)],
    cb: impl FnOnce(&mut String),
) {
    if !prompt.ends_with("\n") {
        prompt.push('\n');
    }
    prompt.push('<');
    prompt.push_str(tag);
    for (arg_name, arg_value) in arguments {
        write!(prompt, " {}=\"{}\"", arg_name, arg_value).ok();
    }
    prompt.push_str(">\n");

    cb(prompt);

    if !prompt.ends_with('\n') {
        prompt.push('\n');
    }
    prompt.push_str("</");
    prompt.push_str(tag);
    prompt.push_str(">\n");
}
