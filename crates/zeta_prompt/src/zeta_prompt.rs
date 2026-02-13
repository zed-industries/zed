use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use strum::{EnumIter, IntoEnumIterator as _, IntoStaticStr};

pub const CURSOR_MARKER: &str = "<|user_cursor|>";
pub const MAX_PROMPT_TOKENS: usize = 4096;

/// Use up to this amount of the editable region for prefill.
/// Larger values may result in more robust generation, but
/// this region becomes non-editable.
pub const PREFILL_RATIO: f64 = 0.1; // 10%

fn estimate_tokens(bytes: usize) -> usize {
    bytes / 3
}

/// The client's preferred edit prediction model. The server may override this.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditPredictionModelKind {
    Zeta1,
    Zeta2,
}

/// Pre-computed byte offset ranges within `cursor_excerpt` for different
/// editable and context token budgets. Allows the server to select the
/// appropriate ranges for whichever model it uses.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExcerptRanges {
    /// Editable region computed with a 150-token budget.
    pub editable_150: Range<usize>,
    /// Editable region computed with a 180-token budget.
    pub editable_180: Range<usize>,
    /// Editable region computed with a 350-token budget.
    pub editable_350: Range<usize>,
    /// Context boundary when using editable_150 with 350 tokens of additional context.
    pub editable_150_context_350: Range<usize>,
    /// Context boundary when using editable_180 with 350 tokens of additional context.
    pub editable_180_context_350: Range<usize>,
    /// Context boundary when using editable_350 with 150 tokens of additional context.
    pub editable_350_context_150: Range<usize>,
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
    /// When set, the excerpt was computed with a larger budget (~512 tokens)
    /// and these ranges let the server select model-appropriate subsets.
    /// When absent, the excerpt IS the context region and
    /// `editable_range_in_excerpt` is the only editable range.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excerpt_ranges: Option<ExcerptRanges>,
    /// Client's preferred model. The server may override.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_model: Option<EditPredictionModelKind>,
    #[serde(default)]
    pub in_open_source_repo: bool,
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
pub enum ZetaFormat {
    V0112MiddleAtEnd,
    V0113Ordered,
    #[default]
    V0114180EditableRegion,
    V0120GitMergeMarkers,
    V0131GitMergeMarkersPrefix,
    V0211Prefill,
    V0211SeedCoder,
}

impl std::fmt::Display for ZetaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl ZetaFormat {
    pub fn parse(format_name: &str) -> Result<Self> {
        let mut results = ZetaFormat::iter().filter(|version| {
            <&'static str>::from(version)
                .to_lowercase()
                .contains(&format_name.to_lowercase())
        });
        let Some(result) = results.next() else {
            anyhow::bail!(
                "`{format_name}` did not match any of:\n{}",
                Self::options_as_string()
            );
        };
        if results.next().is_some() {
            anyhow::bail!(
                "`{format_name}` matched more than one of:\n{}",
                Self::options_as_string()
            );
        }
        Ok(result)
    }

    pub fn options_as_string() -> String {
        ZetaFormat::iter()
            .map(|format| format!("- {}\n", <&'static str>::from(format)))
            .collect::<Vec<_>>()
            .concat()
    }

    pub fn special_tokens(&self) -> &'static [&'static str] {
        match self {
            ZetaFormat::V0112MiddleAtEnd
            | ZetaFormat::V0113Ordered
            | ZetaFormat::V0114180EditableRegion => &[
                "<|fim_prefix|>",
                "<|fim_suffix|>",
                "<|fim_middle|>",
                "<|file_sep|>",
                CURSOR_MARKER,
            ],
            ZetaFormat::V0120GitMergeMarkers => v0120_git_merge_markers::special_tokens(),
            ZetaFormat::V0131GitMergeMarkersPrefix | ZetaFormat::V0211Prefill => {
                v0131_git_merge_markers_prefix::special_tokens()
            }
            ZetaFormat::V0211SeedCoder => seed_coder::special_tokens(),
        }
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

impl Event {
    pub fn in_open_source_repo(&self) -> bool {
        match self {
            Event::BufferChange {
                in_open_source_repo,
                ..
            } => *in_open_source_repo,
        }
    }
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
    #[serde(default)]
    pub in_open_source_repo: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelatedExcerpt {
    pub row_range: Range<u32>,
    pub text: Arc<str>,
}

pub fn prompt_input_contains_special_tokens(input: &ZetaPromptInput, format: ZetaFormat) -> bool {
    format
        .special_tokens()
        .iter()
        .any(|token| input.cursor_excerpt.contains(token))
}

pub fn format_zeta_prompt(input: &ZetaPromptInput, format: ZetaFormat) -> String {
    format_zeta_prompt_with_budget(input, format, MAX_PROMPT_TOKENS)
}

/// Post-processes model output for the given zeta format by stripping format-specific suffixes.
pub fn clean_zeta2_model_output(output: &str, format: ZetaFormat) -> &str {
    match format {
        ZetaFormat::V0120GitMergeMarkers => output
            .strip_suffix(v0120_git_merge_markers::END_MARKER)
            .unwrap_or(output),
        ZetaFormat::V0131GitMergeMarkersPrefix => output
            .strip_suffix(v0131_git_merge_markers_prefix::END_MARKER)
            .unwrap_or(output),
        ZetaFormat::V0211SeedCoder => output
            .strip_suffix(seed_coder::END_MARKER)
            .unwrap_or(output),
        _ => output,
    }
}

fn resolve_cursor_region(
    input: &ZetaPromptInput,
    format: ZetaFormat,
) -> (&str, Range<usize>, usize) {
    let Some(ranges) = &input.excerpt_ranges else {
        return (
            &input.cursor_excerpt,
            input.editable_range_in_excerpt.clone(),
            input.cursor_offset_in_excerpt,
        );
    };

    let (editable_range, context_range) = match format {
        ZetaFormat::V0112MiddleAtEnd | ZetaFormat::V0113Ordered => (
            ranges.editable_150.clone(),
            ranges.editable_150_context_350.clone(),
        ),
        ZetaFormat::V0114180EditableRegion
        | ZetaFormat::V0120GitMergeMarkers
        | ZetaFormat::V0131GitMergeMarkersPrefix
        | ZetaFormat::V0211Prefill
        | ZetaFormat::V0211SeedCoder => (
            ranges.editable_180.clone(),
            ranges.editable_180_context_350.clone(),
        ),
    };

    let context_start = context_range.start;
    let context_text = &input.cursor_excerpt[context_range];
    let adjusted_editable =
        (editable_range.start - context_start)..(editable_range.end - context_start);
    let adjusted_cursor = input.cursor_offset_in_excerpt - context_start;

    (context_text, adjusted_editable, adjusted_cursor)
}

fn format_zeta_prompt_with_budget(
    input: &ZetaPromptInput,
    format: ZetaFormat,
    max_tokens: usize,
) -> String {
    let (context, editable_range, cursor_offset) = resolve_cursor_region(input, format);
    let path = &*input.cursor_path;

    let mut cursor_section = String::new();
    match format {
        ZetaFormat::V0112MiddleAtEnd => {
            v0112_middle_at_end::write_cursor_excerpt_section(
                &mut cursor_section,
                path,
                context,
                &editable_range,
                cursor_offset,
            );
        }
        ZetaFormat::V0113Ordered | ZetaFormat::V0114180EditableRegion => {
            v0113_ordered::write_cursor_excerpt_section(
                &mut cursor_section,
                path,
                context,
                &editable_range,
                cursor_offset,
            )
        }
        ZetaFormat::V0120GitMergeMarkers => v0120_git_merge_markers::write_cursor_excerpt_section(
            &mut cursor_section,
            path,
            context,
            &editable_range,
            cursor_offset,
        ),
        ZetaFormat::V0131GitMergeMarkersPrefix | ZetaFormat::V0211Prefill => {
            v0131_git_merge_markers_prefix::write_cursor_excerpt_section(
                &mut cursor_section,
                path,
                context,
                &editable_range,
                cursor_offset,
            )
        }
        ZetaFormat::V0211SeedCoder => {
            return seed_coder::format_prompt_with_budget(
                path,
                context,
                &editable_range,
                cursor_offset,
                &input.events,
                &input.related_files,
                max_tokens,
            );
        }
    }

    let cursor_tokens = estimate_tokens(cursor_section.len());
    let budget_after_cursor = max_tokens.saturating_sub(cursor_tokens);

    let edit_history_section = format_edit_history_within_budget(
        &input.events,
        "<|file_sep|>",
        "edit history",
        budget_after_cursor,
    );
    let edit_history_tokens = estimate_tokens(edit_history_section.len());
    let budget_after_edit_history = budget_after_cursor.saturating_sub(edit_history_tokens);

    let related_files_section = format_related_files_within_budget(
        &input.related_files,
        "<|file_sep|>",
        budget_after_edit_history,
    );

    let mut prompt = String::new();
    prompt.push_str(&related_files_section);
    prompt.push_str(&edit_history_section);
    prompt.push_str(&cursor_section);
    prompt
}

pub fn get_prefill(input: &ZetaPromptInput, format: ZetaFormat) -> String {
    match format {
        ZetaFormat::V0112MiddleAtEnd
        | ZetaFormat::V0113Ordered
        | ZetaFormat::V0114180EditableRegion
        | ZetaFormat::V0120GitMergeMarkers
        | ZetaFormat::V0131GitMergeMarkersPrefix
        | ZetaFormat::V0211SeedCoder => String::new(),
        ZetaFormat::V0211Prefill => v0211_prefill::get_prefill(input),
    }
}

fn format_edit_history_within_budget(
    events: &[Arc<Event>],
    file_marker: &str,
    edit_history_name: &str,
    max_tokens: usize,
) -> String {
    let header = format!("{}{}\n", file_marker, edit_history_name);
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

    let mut result = header;
    for event_str in event_strings.iter().rev() {
        result.push_str(event_str);
    }
    result
}

fn format_related_files_within_budget(
    related_files: &[RelatedFile],
    file_marker: &str,
    max_tokens: usize,
) -> String {
    let mut result = String::new();
    let mut total_tokens = 0;

    for file in related_files {
        let path_str = file.path.to_string_lossy();
        let header = format!("{}{}\n", file_marker, path_str);
        let header_tokens = estimate_tokens(header.len());

        if total_tokens + header_tokens > max_tokens {
            break;
        }

        let mut file_tokens = header_tokens;
        let mut excerpts_to_include = 0;

        for excerpt in &file.excerpts {
            let needs_newline = !excerpt.text.ends_with('\n');
            let needs_ellipsis = excerpt.row_range.end < file.max_row;
            let excerpt_len = excerpt.text.len()
                + if needs_newline { "\n".len() } else { 0 }
                + if needs_ellipsis { "...\n".len() } else { 0 };

            let excerpt_tokens = estimate_tokens(excerpt_len);
            if total_tokens + file_tokens + excerpt_tokens > max_tokens {
                break;
            }
            file_tokens += excerpt_tokens;
            excerpts_to_include += 1;
        }

        if excerpts_to_include > 0 {
            total_tokens += file_tokens;
            result.push_str(&header);
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

    pub fn write_cursor_excerpt_section(
        prompt: &mut String,
        path: &Path,
        context: &str,
        editable_range: &Range<usize>,
        cursor_offset: usize,
    ) {
        let path_str = path.to_string_lossy();
        write!(prompt, "<|file_sep|>{}\n", path_str).ok();

        prompt.push_str("<|fim_prefix|>\n");
        prompt.push_str(&context[..editable_range.start]);

        prompt.push_str("<|fim_suffix|>\n");
        prompt.push_str(&context[editable_range.end..]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_middle|>current\n");
        prompt.push_str(&context[editable_range.start..cursor_offset]);
        prompt.push_str(CURSOR_MARKER);
        prompt.push_str(&context[cursor_offset..editable_range.end]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_middle|>updated\n");
    }
}

mod v0113_ordered {
    use super::*;

    pub fn write_cursor_excerpt_section(
        prompt: &mut String,
        path: &Path,
        context: &str,
        editable_range: &Range<usize>,
        cursor_offset: usize,
    ) {
        let path_str = path.to_string_lossy();
        write!(prompt, "<|file_sep|>{}\n", path_str).ok();

        prompt.push_str("<|fim_prefix|>\n");
        prompt.push_str(&context[..editable_range.start]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_middle|>current\n");
        prompt.push_str(&context[editable_range.start..cursor_offset]);
        prompt.push_str(CURSOR_MARKER);
        prompt.push_str(&context[cursor_offset..editable_range.end]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_suffix|>\n");
        prompt.push_str(&context[editable_range.end..]);
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

    pub fn special_tokens() -> &'static [&'static str] {
        &[
            "<|fim_prefix|>",
            "<|fim_suffix|>",
            "<|fim_middle|>",
            "<|file_sep|>",
            START_MARKER,
            SEPARATOR,
            END_MARKER,
            CURSOR_MARKER,
        ]
    }

    pub fn write_cursor_excerpt_section(
        prompt: &mut String,
        path: &Path,
        context: &str,
        editable_range: &Range<usize>,
        cursor_offset: usize,
    ) {
        let path_str = path.to_string_lossy();
        write!(prompt, "<|file_sep|>{}\n", path_str).ok();

        prompt.push_str("<|fim_prefix|>");
        prompt.push_str(&context[..editable_range.start]);

        prompt.push_str("<|fim_suffix|>");
        prompt.push_str(&context[editable_range.end..]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_middle|>");
        prompt.push_str(START_MARKER);
        prompt.push_str(&context[editable_range.start..cursor_offset]);
        prompt.push_str(CURSOR_MARKER);
        prompt.push_str(&context[cursor_offset..editable_range.end]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }
        prompt.push_str(SEPARATOR);
    }
}

pub mod v0131_git_merge_markers_prefix {
    //! A prompt that uses git-style merge conflict markers to represent the editable region.
    //!
    //! Example prompt:
    //!
    //! <|file_sep|>path/to/target_file.py
    //! <|fim_prefix|>
    //! code before editable region
    //! <<<<<<< CURRENT
    //! code that
    //! needs to<|user_cursor|>
    //! be rewritten
    //! =======
    //! <|fim_suffix|>
    //! code after editable region
    //! <|fim_middle|>
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

    pub fn special_tokens() -> &'static [&'static str] {
        &[
            "<|fim_prefix|>",
            "<|fim_suffix|>",
            "<|fim_middle|>",
            "<|file_sep|>",
            START_MARKER,
            SEPARATOR,
            END_MARKER,
            CURSOR_MARKER,
        ]
    }

    pub fn write_cursor_excerpt_section(
        prompt: &mut String,
        path: &Path,
        context: &str,
        editable_range: &Range<usize>,
        cursor_offset: usize,
    ) {
        let path_str = path.to_string_lossy();
        write!(prompt, "<|file_sep|>{}\n", path_str).ok();

        prompt.push_str("<|fim_prefix|>");
        prompt.push_str(&context[..editable_range.start]);
        prompt.push_str(START_MARKER);
        prompt.push_str(&context[editable_range.start..cursor_offset]);
        prompt.push_str(CURSOR_MARKER);
        prompt.push_str(&context[cursor_offset..editable_range.end]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }
        prompt.push_str(SEPARATOR);

        prompt.push_str("<|fim_suffix|>");
        prompt.push_str(&context[editable_range.end..]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_middle|>");
    }
}

pub mod v0211_prefill {
    use super::*;

    pub fn get_prefill(input: &ZetaPromptInput) -> String {
        let editable_region = &input.cursor_excerpt
            [input.editable_range_in_excerpt.start..input.editable_range_in_excerpt.end];

        let prefill_len = (editable_region.len() as f64 * PREFILL_RATIO) as usize;
        let prefill_len = editable_region.floor_char_boundary(prefill_len);

        // Find a token boundary to avoid splitting tokens in the prefill.
        // In Qwen2.5-Coder, \n is always the END of a token (e.g. `;\n`,
        // ` {\n`), and \n\n / \n\n\n are single tokens, so we must include
        // the \n and consume any consecutive \n characters after it.
        let prefill = &editable_region[..prefill_len];
        match prefill.rfind('\n') {
            Some(pos) => {
                let mut end = pos + 1;
                while end < editable_region.len()
                    && editable_region.as_bytes().get(end) == Some(&b'\n')
                {
                    end += 1;
                }
                editable_region[..end].to_string()
            }
            // No newline found. Fall back to splitting before the last space
            // (word-level boundary)
            None => match prefill.rfind(' ') {
                Some(pos) => prefill[..pos].to_string(),
                None => prefill.to_string(),
            },
        }
    }
}

pub mod seed_coder {
    //! Seed-Coder prompt format using SPM (Suffix-Prefix-Middle) FIM mode.
    //!
    //! Seed-Coder uses different FIM tokens and order than Qwen:
    //! - SPM order: suffix comes FIRST, then prefix, then middle
    //! - Tokens: `<[fim-suffix]>`, `<[fim-prefix]>`, `<[fim-middle]>`
    //! - File markers: StarCoder-style `<filename>path` (single token + path)
    //!
    //! All context (related files, edit history) goes in the PREFIX section.
    //! The suffix contains only code after the editable region.
    //!
    //! Example prompt:
    //!
    //! <[fim-suffix]>
    //! code after editable region
    //! <[fim-prefix]><filename>related/file.py
    //! related file content
    //!
    //! <filename>edit_history
    //! --- a/some_file.py
    //! +++ b/some_file.py
    //! -old
    //! +new
    //!
    //! <filename>path/to/target_file.py
    //! code before editable region
    //! <<<<<<< CURRENT
    //! code that
    //! needs to<|user_cursor|>
    //! be rewritten
    //! =======
    //! <[fim-middle]>
    //!
    //! Expected output (model generates):
    //!
    //! updated
    //! code with
    //! changes applied
    //! >>>>>>> UPDATED

    use super::*;

    pub const FIM_SUFFIX: &str = "<[fim-suffix]>";
    pub const FIM_PREFIX: &str = "<[fim-prefix]>";
    pub const FIM_MIDDLE: &str = "<[fim-middle]>";
    pub const FILE_MARKER: &str = "<filename>";

    pub const START_MARKER: &str = "<<<<<<< CURRENT\n";
    pub const SEPARATOR: &str = "=======\n";
    pub const END_MARKER: &str = ">>>>>>> UPDATED\n";

    pub fn special_tokens() -> &'static [&'static str] {
        &[
            FIM_SUFFIX,
            FIM_PREFIX,
            FIM_MIDDLE,
            FILE_MARKER,
            START_MARKER,
            SEPARATOR,
            END_MARKER,
            CURSOR_MARKER,
        ]
    }

    pub fn format_prompt_with_budget(
        path: &Path,
        context: &str,
        editable_range: &Range<usize>,
        cursor_offset: usize,
        events: &[Arc<Event>],
        related_files: &[RelatedFile],
        max_tokens: usize,
    ) -> String {
        let suffix_section = build_suffix_section(context, editable_range);
        let cursor_prefix_section =
            build_cursor_prefix_section(path, context, editable_range, cursor_offset);

        let suffix_tokens = estimate_tokens(suffix_section.len());
        let cursor_prefix_tokens = estimate_tokens(cursor_prefix_section.len());
        let budget_after_cursor = max_tokens.saturating_sub(suffix_tokens + cursor_prefix_tokens);

        let edit_history_section = super::format_edit_history_within_budget(
            events,
            FILE_MARKER,
            "edit_history",
            budget_after_cursor,
        );
        let edit_history_tokens = estimate_tokens(edit_history_section.len());
        let budget_after_edit_history = budget_after_cursor.saturating_sub(edit_history_tokens);

        let related_files_section = super::format_related_files_within_budget(
            related_files,
            FILE_MARKER,
            budget_after_edit_history,
        );

        let mut prompt = String::new();
        prompt.push_str(&suffix_section);
        prompt.push_str(FIM_PREFIX);
        prompt.push_str(&related_files_section);
        if !related_files_section.is_empty() {
            prompt.push('\n');
        }
        prompt.push_str(&edit_history_section);
        if !edit_history_section.is_empty() {
            prompt.push('\n');
        }
        prompt.push_str(&cursor_prefix_section);
        prompt.push_str(FIM_MIDDLE);
        prompt
    }

    fn build_suffix_section(context: &str, editable_range: &Range<usize>) -> String {
        let mut section = String::new();
        section.push_str(FIM_SUFFIX);
        section.push_str(&context[editable_range.end..]);
        if !section.ends_with('\n') {
            section.push('\n');
        }
        section
    }

    fn build_cursor_prefix_section(
        path: &Path,
        context: &str,
        editable_range: &Range<usize>,
        cursor_offset: usize,
    ) -> String {
        let mut section = String::new();
        let path_str = path.to_string_lossy();
        write!(section, "{}{}\n", FILE_MARKER, path_str).ok();

        section.push_str(&context[..editable_range.start]);
        section.push_str(START_MARKER);
        section.push_str(&context[editable_range.start..cursor_offset]);
        section.push_str(CURSOR_MARKER);
        section.push_str(&context[cursor_offset..editable_range.end]);
        if !section.ends_with('\n') {
            section.push('\n');
        }
        section.push_str(SEPARATOR);
        section
    }
}

/// The zeta1 prompt format
pub mod zeta1 {
    use super::*;
    use std::fmt::Write;

    pub const CURSOR_MARKER: &str = "<|user_cursor_is_here|>";
    pub const START_OF_FILE_MARKER: &str = "<|start_of_file|>";
    pub const EDITABLE_REGION_START_MARKER: &str = "<|editable_region_start|>";
    pub const EDITABLE_REGION_END_MARKER: &str = "<|editable_region_end|>";

    const INSTRUCTION_HEADER: &str = concat!(
        "### Instruction:\n",
        "You are a code completion assistant and your task is to analyze user edits and then rewrite an ",
        "excerpt that the user provides, suggesting the appropriate edits within the excerpt, taking ",
        "into account the cursor location.\n\n",
        "### User Edits:\n\n"
    );
    const EXCERPT_HEADER: &str = "\n\n### User Excerpt:\n\n";
    const RESPONSE_HEADER: &str = "\n\n### Response:\n";

    /// Formats a complete zeta1 prompt from the input events and excerpt.
    pub fn format_zeta1_prompt(input_events: &str, input_excerpt: &str) -> String {
        let mut prompt = String::with_capacity(
            INSTRUCTION_HEADER.len()
                + input_events.len()
                + EXCERPT_HEADER.len()
                + input_excerpt.len()
                + RESPONSE_HEADER.len(),
        );
        prompt.push_str(INSTRUCTION_HEADER);
        prompt.push_str(input_events);
        prompt.push_str(EXCERPT_HEADER);
        prompt.push_str(input_excerpt);
        prompt.push_str(RESPONSE_HEADER);
        prompt
    }

    /// Formats a complete zeta1 prompt from a `ZetaPromptInput` using the given
    /// editable and context byte-offset ranges within `cursor_excerpt`.
    pub fn format_zeta1_from_input(
        input: &ZetaPromptInput,
        editable_range: Range<usize>,
        context_range: Range<usize>,
    ) -> String {
        let events = format_zeta1_events(&input.events);
        let excerpt = format_zeta1_excerpt(input, editable_range, context_range);
        format_zeta1_prompt(&events, &excerpt)
    }

    /// Formats events in zeta1 style (oldest first).
    fn format_zeta1_events(events: &[Arc<Event>]) -> String {
        let mut result = String::new();
        for event in events {
            let event_string = format_zeta1_event(event);
            if event_string.is_empty() {
                continue;
            }
            if !result.is_empty() {
                result.push_str("\n\n");
            }
            result.push_str(&event_string);
        }
        result
    }

    fn format_zeta1_event(event: &Event) -> String {
        match event {
            Event::BufferChange {
                path,
                old_path,
                diff,
                ..
            } => {
                let mut prompt = String::new();
                if old_path != path {
                    writeln!(
                        prompt,
                        "User renamed {} to {}\n",
                        old_path.display(),
                        path.display()
                    )
                    .ok();
                }
                if !diff.is_empty() {
                    write!(
                        prompt,
                        "User edited {}:\n```diff\n{}\n```",
                        path.display(),
                        diff
                    )
                    .ok();
                }
                prompt
            }
        }
    }

    /// Formats the excerpt section of a zeta1 prompt using byte-offset ranges
    /// within `cursor_excerpt`.
    fn format_zeta1_excerpt(
        input: &ZetaPromptInput,
        editable_range: Range<usize>,
        context_range: Range<usize>,
    ) -> String {
        let path_str = input.cursor_path.to_string_lossy();
        let excerpt = &*input.cursor_excerpt;
        let cursor_offset = input.cursor_offset_in_excerpt;

        let mut prompt = String::new();
        writeln!(&mut prompt, "```{path_str}").ok();

        let starts_at_file_beginning =
            input.excerpt_start_row == Some(0) && context_range.start == 0;
        if starts_at_file_beginning {
            writeln!(&mut prompt, "{START_OF_FILE_MARKER}").ok();
        }

        prompt.push_str(&excerpt[context_range.start..editable_range.start]);

        writeln!(&mut prompt, "{EDITABLE_REGION_START_MARKER}").ok();
        prompt.push_str(&excerpt[editable_range.start..cursor_offset]);
        prompt.push_str(CURSOR_MARKER);
        prompt.push_str(&excerpt[cursor_offset..editable_range.end]);
        write!(&mut prompt, "\n{EDITABLE_REGION_END_MARKER}").ok();

        prompt.push_str(&excerpt[editable_range.end..context_range.end]);
        write!(prompt, "\n```").ok();

        prompt
    }

    /// Cleans zeta1 model output by extracting content between editable region
    /// markers and converting the zeta1 cursor marker to the universal one.
    /// Returns `None` if the output doesn't contain the expected markers.
    pub fn clean_zeta1_model_output(output: &str) -> Option<String> {
        let content = output.replace(CURSOR_MARKER, "");

        let content_start = content
            .find(EDITABLE_REGION_START_MARKER)
            .map(|pos| pos + EDITABLE_REGION_START_MARKER.len())
            .map(|pos| {
                if content.as_bytes().get(pos) == Some(&b'\n') {
                    pos + 1
                } else {
                    pos
                }
            })
            .unwrap_or(0);

        let content_end = content
            .find(EDITABLE_REGION_END_MARKER)
            .map(|pos| {
                if pos > 0 && content.as_bytes().get(pos - 1) == Some(&b'\n') {
                    pos - 1
                } else {
                    pos
                }
            })
            .unwrap_or(content.len());

        if content_start > content_end {
            return Some(String::new());
        }

        let extracted = &content[content_start..content_end];

        let cursor_offset = output.find(CURSOR_MARKER).map(|zeta1_cursor_pos| {
            let text_before_cursor = output[..zeta1_cursor_pos].replace(CURSOR_MARKER, "");
            let text_before_cursor = text_before_cursor
                .find(EDITABLE_REGION_START_MARKER)
                .map(|pos| {
                    let after_marker = pos + EDITABLE_REGION_START_MARKER.len();
                    if text_before_cursor.as_bytes().get(after_marker) == Some(&b'\n') {
                        after_marker + 1
                    } else {
                        after_marker
                    }
                })
                .unwrap_or(0);
            let offset_in_extracted = zeta1_cursor_pos
                .saturating_sub(text_before_cursor)
                .min(extracted.len());
            offset_in_extracted
        });

        let mut result = String::with_capacity(extracted.len() + super::CURSOR_MARKER.len());
        if let Some(offset) = cursor_offset {
            result.push_str(&extracted[..offset]);
            result.push_str(super::CURSOR_MARKER);
            result.push_str(&extracted[offset..]);
        } else {
            result.push_str(extracted);
        }

        Some(result)
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
            excerpt_ranges: None,
            preferred_model: None,
            in_open_source_repo: false,
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
            in_open_source_repo: false,
        }
    }

    fn format_with_budget(input: &ZetaPromptInput, max_tokens: usize) -> String {
        format_zeta_prompt_with_budget(input, ZetaFormat::V0114180EditableRegion, max_tokens)
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
                in_open_source_repo: false,
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

    fn format_seed_coder(input: &ZetaPromptInput) -> String {
        format_zeta_prompt_with_budget(input, ZetaFormat::V0211SeedCoder, 10000)
    }

    fn format_seed_coder_with_budget(input: &ZetaPromptInput, max_tokens: usize) -> String {
        format_zeta_prompt_with_budget(input, ZetaFormat::V0211SeedCoder, max_tokens)
    }

    #[test]
    fn test_seed_coder_basic_format() {
        let input = make_input(
            "prefix\neditable\nsuffix",
            7..15,
            10,
            vec![make_event("a.rs", "-old\n+new\n")],
            vec![make_related_file("related.rs", "fn helper() {}\n")],
        );

        assert_eq!(
            format_seed_coder(&input),
            indoc! {r#"
                <[fim-suffix]>
                suffix
                <[fim-prefix]><filename>related.rs
                fn helper() {}

                <filename>edit_history
                --- a/a.rs
                +++ b/a.rs
                -old
                +new

                <filename>test.rs
                prefix
                <<<<<<< CURRENT
                edi<|user_cursor|>table
                =======
                <[fim-middle]>"#}
        );
    }

    #[test]
    fn test_seed_coder_no_context() {
        let input = make_input("before\nmiddle\nafter", 7..13, 10, vec![], vec![]);

        assert_eq!(
            format_seed_coder(&input),
            indoc! {r#"
                <[fim-suffix]>
                after
                <[fim-prefix]><filename>test.rs
                before
                <<<<<<< CURRENT
                mid<|user_cursor|>dle
                =======
                <[fim-middle]>"#}
        );
    }

    #[test]
    fn test_seed_coder_truncation_drops_context() {
        let input = make_input(
            "code",
            0..4,
            2,
            vec![make_event("a.rs", "-x\n+y\n")],
            vec![make_related_file("r1.rs", "content\n")],
        );

        // With large budget, everything is included
        assert_eq!(
            format_seed_coder(&input),
            indoc! {r#"
                <[fim-suffix]>
                <[fim-prefix]><filename>r1.rs
                content

                <filename>edit_history
                --- a/a.rs
                +++ b/a.rs
                -x
                +y

                <filename>test.rs
                <<<<<<< CURRENT
                co<|user_cursor|>de
                =======
                <[fim-middle]>"#}
        );

        // With tight budget, context is dropped but cursor section remains
        assert_eq!(
            format_seed_coder_with_budget(&input, 30),
            indoc! {r#"
                <[fim-suffix]>
                <[fim-prefix]><filename>test.rs
                <<<<<<< CURRENT
                co<|user_cursor|>de
                =======
                <[fim-middle]>"#}
        );
    }

    #[test]
    fn test_seed_coder_clean_output() {
        let output_with_marker = "new code\n>>>>>>> UPDATED\n";
        let output_without_marker = "new code\n";

        assert_eq!(
            clean_zeta2_model_output(output_with_marker, ZetaFormat::V0211SeedCoder),
            "new code\n"
        );
        assert_eq!(
            clean_zeta2_model_output(output_without_marker, ZetaFormat::V0211SeedCoder),
            "new code\n"
        );
    }

    #[test]
    fn test_format_zeta1_from_input_basic() {
        let excerpt = "fn before() {}\nfn foo() {\n    let x = 1;\n}\nfn after() {}\n";
        let input = ZetaPromptInput {
            cursor_path: Path::new("src/main.rs").into(),
            cursor_excerpt: excerpt.into(),
            editable_range_in_excerpt: 15..41,
            cursor_offset_in_excerpt: 30,
            excerpt_start_row: Some(0),
            events: vec![Arc::new(make_event("other.rs", "-old\n+new\n"))],
            related_files: vec![],
            excerpt_ranges: None,
            preferred_model: None,
            in_open_source_repo: false,
        };

        let prompt = zeta1::format_zeta1_from_input(&input, 15..41, 0..excerpt.len());

        assert_eq!(
            prompt,
            concat!(
                "### Instruction:\n",
                "You are a code completion assistant and your task is to analyze user edits and then rewrite an ",
                "excerpt that the user provides, suggesting the appropriate edits within the excerpt, taking ",
                "into account the cursor location.\n",
                "\n",
                "### User Edits:\n",
                "\n",
                "User edited other.rs:\n",
                "```diff\n",
                "-old\n",
                "+new\n",
                "\n",
                "```\n",
                "\n",
                "### User Excerpt:\n",
                "\n",
                "```src/main.rs\n",
                "<|start_of_file|>\n",
                "fn before() {}\n",
                "<|editable_region_start|>\n",
                "fn foo() {\n",
                "    <|user_cursor_is_here|>let x = 1;\n",
                "\n",
                "<|editable_region_end|>}\n",
                "fn after() {}\n",
                "\n",
                "```\n",
                "\n",
                "### Response:\n",
            ),
        );
    }

    #[test]
    fn test_format_zeta1_from_input_no_start_of_file() {
        let excerpt = "fn foo() {\n    let x = 1;\n}\n";
        let input = ZetaPromptInput {
            cursor_path: Path::new("src/main.rs").into(),
            cursor_excerpt: excerpt.into(),
            editable_range_in_excerpt: 0..28,
            cursor_offset_in_excerpt: 15,
            excerpt_start_row: Some(10),
            events: vec![],
            related_files: vec![],
            excerpt_ranges: None,
            preferred_model: None,
            in_open_source_repo: false,
        };

        let prompt = zeta1::format_zeta1_from_input(&input, 0..28, 0..28);

        assert_eq!(
            prompt,
            concat!(
                "### Instruction:\n",
                "You are a code completion assistant and your task is to analyze user edits and then rewrite an ",
                "excerpt that the user provides, suggesting the appropriate edits within the excerpt, taking ",
                "into account the cursor location.\n",
                "\n",
                "### User Edits:\n",
                "\n",
                "\n",
                "\n",
                "### User Excerpt:\n",
                "\n",
                "```src/main.rs\n",
                "<|editable_region_start|>\n",
                "fn foo() {\n",
                "    <|user_cursor_is_here|>let x = 1;\n",
                "}\n",
                "\n",
                "<|editable_region_end|>\n",
                "```\n",
                "\n",
                "### Response:\n",
            ),
        );
    }

    #[test]
    fn test_format_zeta1_from_input_with_sub_ranges() {
        let excerpt = "// prefix\nfn foo() {\n    let x = 1;\n}\n// suffix\n";
        let editable_range = 10..37;
        let context_range = 0..excerpt.len();

        let input = ZetaPromptInput {
            cursor_path: Path::new("test.rs").into(),
            cursor_excerpt: excerpt.into(),
            editable_range_in_excerpt: editable_range.clone(),
            cursor_offset_in_excerpt: 25,
            excerpt_start_row: Some(0),
            events: vec![],
            related_files: vec![],
            excerpt_ranges: None,
            preferred_model: None,
            in_open_source_repo: false,
        };

        let prompt = zeta1::format_zeta1_from_input(&input, editable_range, context_range);

        assert_eq!(
            prompt,
            concat!(
                "### Instruction:\n",
                "You are a code completion assistant and your task is to analyze user edits and then rewrite an ",
                "excerpt that the user provides, suggesting the appropriate edits within the excerpt, taking ",
                "into account the cursor location.\n",
                "\n",
                "### User Edits:\n",
                "\n",
                "\n",
                "\n",
                "### User Excerpt:\n",
                "\n",
                "```test.rs\n",
                "<|start_of_file|>\n",
                "// prefix\n",
                "<|editable_region_start|>\n",
                "fn foo() {\n",
                "    <|user_cursor_is_here|>let x = 1;\n",
                "}\n",
                "<|editable_region_end|>\n",
                "// suffix\n",
                "\n",
                "```\n",
                "\n",
                "### Response:\n",
            ),
        );
    }

    #[test]
    fn test_clean_zeta1_model_output_basic() {
        let output = indoc! {"
            <|editable_region_start|>
            fn main() {
                println!(\"hello\");
            }
            <|editable_region_end|>
        "};

        let cleaned = zeta1::clean_zeta1_model_output(output).unwrap();
        assert_eq!(cleaned, "fn main() {\n    println!(\"hello\");\n}");
    }

    #[test]
    fn test_clean_zeta1_model_output_with_cursor() {
        let output = indoc! {"
            <|editable_region_start|>
            fn main() {
                <|user_cursor_is_here|>println!(\"hello\");
            }
            <|editable_region_end|>
        "};

        let cleaned = zeta1::clean_zeta1_model_output(output).unwrap();
        assert_eq!(
            cleaned,
            "fn main() {\n    <|user_cursor|>println!(\"hello\");\n}"
        );
    }

    #[test]
    fn test_clean_zeta1_model_output_no_markers() {
        let output = "fn main() {}\n";
        let cleaned = zeta1::clean_zeta1_model_output(output).unwrap();
        assert_eq!(cleaned, "fn main() {}\n");
    }

    #[test]
    fn test_clean_zeta1_model_output_empty_region() {
        let output = "<|editable_region_start|>\n<|editable_region_end|>\n";
        let cleaned = zeta1::clean_zeta1_model_output(output).unwrap();
        assert_eq!(cleaned, "");
    }
}
