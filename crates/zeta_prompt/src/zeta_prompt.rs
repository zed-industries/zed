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

/// Pre-computed byte offset ranges within `cursor_excerpt` for different
/// editable and context token budgets. Allows the server to select the
/// appropriate ranges for whichever model it uses.
#[derive(Clone, Debug, Default, PartialEq, Hash, Serialize, Deserialize)]
pub struct ExcerptRanges {
    /// Editable region computed with a 150-token budget.
    pub editable_150: Range<usize>,
    /// Editable region computed with a 180-token budget.
    pub editable_180: Range<usize>,
    /// Editable region computed with a 350-token budget.
    pub editable_350: Range<usize>,
    /// Editable region computed with a 350-token budget.
    pub editable_512: Option<Range<usize>>,
    /// Context boundary when using editable_150 with 350 tokens of additional context.
    pub editable_150_context_350: Range<usize>,
    /// Context boundary when using editable_180 with 350 tokens of additional context.
    pub editable_180_context_350: Range<usize>,
    /// Context boundary when using editable_350 with 150 tokens of additional context.
    pub editable_350_context_150: Range<usize>,
    pub editable_350_context_512: Option<Range<usize>>,
    pub editable_350_context_1024: Option<Range<usize>>,
    pub context_4096: Option<Range<usize>>,
    pub context_8192: Option<Range<usize>>,
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct ZetaPromptInput {
    pub cursor_path: Arc<Path>,
    pub cursor_excerpt: Arc<str>,
    pub cursor_offset_in_excerpt: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excerpt_start_row: Option<u32>,
    pub events: Vec<Arc<Event>>,
    pub related_files: Vec<RelatedFile>,
    /// These ranges let the server select model-appropriate subsets.
    pub excerpt_ranges: ExcerptRanges,
    /// The name of the edit prediction model experiment to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experiment: Option<String>,
    #[serde(default)]
    pub in_open_source_repo: bool,
    #[serde(default)]
    pub can_collect_data: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,
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
    V0114180EditableRegion,
    V0120GitMergeMarkers,
    #[default]
    V0131GitMergeMarkersPrefix,
    V0211Prefill,
    V0211SeedCoder,
    v0226Hashline,
    V0304SeedNoEdits,
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
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct RelatedFile {
    pub path: Arc<Path>,
    pub max_row: u32,
    pub excerpts: Vec<RelatedExcerpt>,
    #[serde(default)]
    pub in_open_source_repo: bool,
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct RelatedExcerpt {
    pub row_range: Range<u32>,
    pub text: Arc<str>,
    #[serde(default)]
    pub order: usize,
}

pub fn prompt_input_contains_special_tokens(input: &ZetaPromptInput, format: ZetaFormat) -> bool {
    special_tokens_for_format(format)
        .iter()
        .any(|token| input.cursor_excerpt.contains(token))
}

pub fn format_zeta_prompt(input: &ZetaPromptInput, format: ZetaFormat) -> String {
    format_prompt_with_budget_for_format(input, format, MAX_PROMPT_TOKENS)
}

pub fn special_tokens_for_format(format: ZetaFormat) -> &'static [&'static str] {
    match format {
        ZetaFormat::V0112MiddleAtEnd => v0112_middle_at_end::special_tokens(),
        ZetaFormat::V0113Ordered => v0113_ordered::special_tokens(),
        ZetaFormat::V0114180EditableRegion => v0114180_editable_region::special_tokens(),
        ZetaFormat::V0120GitMergeMarkers => v0120_git_merge_markers::special_tokens(),
        ZetaFormat::V0131GitMergeMarkersPrefix => v0131_git_merge_markers_prefix::special_tokens(),
        ZetaFormat::V0211Prefill => v0211_prefill::special_tokens(),
        ZetaFormat::V0211SeedCoder => seed_coder::special_tokens(),
        ZetaFormat::v0226Hashline => hashline::special_tokens(),
        ZetaFormat::V0304SeedNoEdits => seed_coder::special_tokens(),
    }
}

pub fn excerpt_ranges_for_format(
    format: ZetaFormat,
    ranges: &ExcerptRanges,
) -> (Range<usize>, Range<usize>) {
    match format {
        ZetaFormat::V0112MiddleAtEnd | ZetaFormat::V0113Ordered => (
            ranges.editable_150.clone(),
            ranges.editable_150_context_350.clone(),
        ),
        ZetaFormat::V0114180EditableRegion => (
            ranges.editable_180.clone(),
            ranges.editable_180_context_350.clone(),
        ),
        ZetaFormat::V0120GitMergeMarkers
        | ZetaFormat::V0131GitMergeMarkersPrefix
        | ZetaFormat::V0211Prefill
        | ZetaFormat::V0211SeedCoder
        | ZetaFormat::v0226Hashline
        | ZetaFormat::V0304SeedNoEdits => (
            ranges.editable_350.clone(),
            ranges.editable_350_context_150.clone(),
        ),
    }
}

pub fn write_cursor_excerpt_section_for_format(
    format: ZetaFormat,
    prompt: &mut String,
    path: &Path,
    context: &str,
    editable_range: &Range<usize>,
    cursor_offset: usize,
) {
    match format {
        ZetaFormat::V0112MiddleAtEnd => v0112_middle_at_end::write_cursor_excerpt_section(
            prompt,
            path,
            context,
            editable_range,
            cursor_offset,
        ),
        ZetaFormat::V0113Ordered | ZetaFormat::V0114180EditableRegion => {
            v0113_ordered::write_cursor_excerpt_section(
                prompt,
                path,
                context,
                editable_range,
                cursor_offset,
            )
        }
        ZetaFormat::V0120GitMergeMarkers => v0120_git_merge_markers::write_cursor_excerpt_section(
            prompt,
            path,
            context,
            editable_range,
            cursor_offset,
        ),
        ZetaFormat::V0131GitMergeMarkersPrefix | ZetaFormat::V0211Prefill => {
            v0131_git_merge_markers_prefix::write_cursor_excerpt_section(
                prompt,
                path,
                context,
                editable_range,
                cursor_offset,
            )
        }
        ZetaFormat::V0211SeedCoder | ZetaFormat::V0304SeedNoEdits => {
            seed_coder::write_cursor_excerpt_section(
                prompt,
                path,
                context,
                editable_range,
                cursor_offset,
            )
        }
        ZetaFormat::v0226Hashline => hashline::write_cursor_excerpt_section(
            prompt,
            path,
            context,
            editable_range,
            cursor_offset,
        ),
    }
}

fn offset_range_to_row_range(text: &str, range: Range<usize>) -> Range<u32> {
    let start_row = text[0..range.start].matches('\n').count() as u32;
    let mut end_row = start_row + text[range.clone()].matches('\n').count() as u32;
    if !text[..range.end].ends_with('\n') {
        end_row += 1;
    }
    return start_row..end_row;
}

pub fn format_prompt_with_budget_for_format(
    input: &ZetaPromptInput,
    format: ZetaFormat,
    max_tokens: usize,
) -> String {
    let (context, editable_range, context_range, cursor_offset) =
        resolve_cursor_region(input, format);
    let path = &*input.cursor_path;

    let related_files = if let Some(cursor_excerpt_start_row) = input.excerpt_start_row {
        let relative_row_range = offset_range_to_row_range(context, context_range);
        let row_range = relative_row_range.start + cursor_excerpt_start_row
            ..relative_row_range.end + cursor_excerpt_start_row;
        &filter_redundant_excerpts(
            input.related_files.clone(),
            input.cursor_path.as_ref(),
            row_range,
        )
    } else {
        &input.related_files
    };

    match format {
        ZetaFormat::V0211SeedCoder | ZetaFormat::V0304SeedNoEdits => {
            seed_coder::format_prompt_with_budget(
                path,
                context,
                &editable_range,
                cursor_offset,
                &input.events,
                related_files,
                max_tokens,
            )
        }
        _ => {
            let mut cursor_section = String::new();
            write_cursor_excerpt_section_for_format(
                format,
                &mut cursor_section,
                path,
                context,
                &editable_range,
                cursor_offset,
            );

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
                &related_files,
                "<|file_sep|>",
                "",
                budget_after_edit_history,
            );

            let mut prompt = String::new();
            prompt.push_str(&related_files_section);
            prompt.push_str(&edit_history_section);
            prompt.push_str(&cursor_section);
            prompt
        }
    }
}

pub fn filter_redundant_excerpts(
    mut related_files: Vec<RelatedFile>,
    cursor_path: &Path,
    cursor_row_range: Range<u32>,
) -> Vec<RelatedFile> {
    for file in &mut related_files {
        if file.path.as_ref() == cursor_path {
            file.excerpts.retain(|excerpt| {
                excerpt.row_range.start < cursor_row_range.start
                    || excerpt.row_range.end > cursor_row_range.end
            });
        }
    }
    related_files.retain(|file| !file.excerpts.is_empty());
    related_files
}

pub fn get_prefill_for_format(
    format: ZetaFormat,
    context: &str,
    editable_range: &Range<usize>,
) -> String {
    match format {
        ZetaFormat::V0211Prefill => v0211_prefill::get_prefill(context, editable_range),
        ZetaFormat::V0112MiddleAtEnd
        | ZetaFormat::V0113Ordered
        | ZetaFormat::V0114180EditableRegion
        | ZetaFormat::V0120GitMergeMarkers
        | ZetaFormat::V0131GitMergeMarkersPrefix
        | ZetaFormat::V0211SeedCoder
        | ZetaFormat::v0226Hashline
        | ZetaFormat::V0304SeedNoEdits => String::new(),
    }
}

pub fn output_end_marker_for_format(format: ZetaFormat) -> Option<&'static str> {
    match format {
        ZetaFormat::V0120GitMergeMarkers => Some(v0120_git_merge_markers::END_MARKER),
        ZetaFormat::V0131GitMergeMarkersPrefix => Some(v0131_git_merge_markers_prefix::END_MARKER),
        ZetaFormat::V0211Prefill => Some(v0131_git_merge_markers_prefix::END_MARKER),
        ZetaFormat::V0211SeedCoder | ZetaFormat::V0304SeedNoEdits => Some(seed_coder::END_MARKER),
        ZetaFormat::V0112MiddleAtEnd
        | ZetaFormat::V0113Ordered
        | ZetaFormat::V0114180EditableRegion
        | ZetaFormat::v0226Hashline => None,
    }
}

pub fn current_region_markers_for_format(format: ZetaFormat) -> (&'static str, &'static str) {
    match format {
        ZetaFormat::V0112MiddleAtEnd => ("<|fim_middle|>current\n", "<|fim_middle|>updated"),
        ZetaFormat::V0113Ordered
        | ZetaFormat::V0114180EditableRegion
        | ZetaFormat::v0226Hashline => ("<|fim_middle|>current\n", "<|fim_suffix|>"),
        ZetaFormat::V0120GitMergeMarkers
        | ZetaFormat::V0131GitMergeMarkersPrefix
        | ZetaFormat::V0211Prefill => (
            v0120_git_merge_markers::START_MARKER,
            v0120_git_merge_markers::SEPARATOR,
        ),
        ZetaFormat::V0211SeedCoder | ZetaFormat::V0304SeedNoEdits => {
            (seed_coder::START_MARKER, seed_coder::SEPARATOR)
        }
    }
}

pub fn clean_extracted_region_for_format(format: ZetaFormat, region: &str) -> String {
    match format {
        ZetaFormat::v0226Hashline => hashline::strip_hashline_prefixes(region),
        _ => region.to_string(),
    }
}

pub fn encode_patch_as_output_for_format(
    format: ZetaFormat,
    old_editable_region: &str,
    patch: &str,
    cursor_offset: Option<usize>,
) -> Result<Option<String>> {
    match format {
        ZetaFormat::v0226Hashline => {
            hashline::patch_to_edit_commands(old_editable_region, patch, cursor_offset).map(Some)
        }
        ZetaFormat::V0304SeedNoEdits => Ok(seed_coder::no_edits(patch)),
        _ => Ok(None),
    }
}

pub fn output_with_context_for_format(
    format: ZetaFormat,
    old_editable_region: &str,
    output: &str,
) -> Result<Option<String>> {
    match format {
        ZetaFormat::v0226Hashline => {
            if hashline::output_has_edit_commands(output) {
                Ok(Some(hashline::apply_edit_commands(
                    old_editable_region,
                    output,
                )))
            } else {
                Ok(None)
            }
        }
        ZetaFormat::V0304SeedNoEdits => {
            if output.starts_with(seed_coder::NO_EDITS) {
                Ok(Some(old_editable_region.to_owned()))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

/// Post-processes model output for the given zeta format by stripping format-specific suffixes.
pub fn clean_zeta2_model_output(output: &str, format: ZetaFormat) -> &str {
    match output_end_marker_for_format(format) {
        Some(marker) => output.strip_suffix(marker).unwrap_or(output),
        None => output,
    }
}

pub fn excerpt_range_for_format(
    format: ZetaFormat,
    ranges: &ExcerptRanges,
) -> (Range<usize>, Range<usize>) {
    excerpt_ranges_for_format(format, ranges)
}

pub fn resolve_cursor_region(
    input: &ZetaPromptInput,
    format: ZetaFormat,
) -> (&str, Range<usize>, Range<usize>, usize) {
    let (editable_range, context_range) = excerpt_range_for_format(format, &input.excerpt_ranges);
    let context_start = context_range.start;
    let context_text = &input.cursor_excerpt[context_range.clone()];
    let adjusted_editable =
        (editable_range.start - context_start)..(editable_range.end - context_start);
    let adjusted_cursor = input.cursor_offset_in_excerpt - context_start;
    let adjusted_context =
        (context_range.start - context_start)..(context_range.end - context_start);

    (
        context_text,
        adjusted_editable,
        adjusted_context,
        adjusted_cursor,
    )
}

pub fn get_prefill(input: &ZetaPromptInput, format: ZetaFormat) -> String {
    let (context, editable_range, _, _) = resolve_cursor_region(input, format);
    get_prefill_for_format(format, context, &editable_range)
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

fn excerpt_rendered_tokens(excerpt: &RelatedExcerpt, file_max_row: u32) -> usize {
    let needs_newline = !excerpt.text.ends_with('\n');
    let needs_ellipsis = excerpt.row_range.end < file_max_row;
    let len = excerpt.text.len()
        + if needs_newline { "\n".len() } else { 0 }
        + if needs_ellipsis { "...\n".len() } else { 0 };
    estimate_tokens(len)
}

pub fn format_related_files_within_budget(
    related_files: &[RelatedFile],
    file_prefix: &str,
    file_suffix: &str,
    max_tokens: usize,
) -> String {
    struct ExcerptCandidate {
        file_ix: usize,
        excerpt_ix: usize,
        order: usize,
    }

    let mut excerpt_candidates: Vec<ExcerptCandidate> = related_files
        .iter()
        .enumerate()
        .flat_map(|(file_ix, file)| {
            file.excerpts
                .iter()
                .enumerate()
                .map(move |(excerpt_ix, e)| ExcerptCandidate {
                    file_ix,
                    excerpt_ix,
                    order: e.order,
                })
        })
        .collect();

    // Pre-compute file header strings and their token costs.
    let file_headers: Vec<String> = related_files
        .iter()
        .map(|file| {
            let path_str = file.path.to_string_lossy();
            format!("{}{}\n", file_prefix, path_str)
        })
        .collect();

    // Sort the excerpts by their order and determine how many fit within the budget.
    let mut total_tokens = 0;
    let mut included_excerpt_count = 0_usize;
    let mut included_file_indices = vec![false; related_files.len()];
    excerpt_candidates.sort_by_key(|e| (e.order, e.file_ix, e.excerpt_ix));
    for candidate in &excerpt_candidates {
        let file = &related_files[candidate.file_ix];
        let excerpt = &file.excerpts[candidate.excerpt_ix];
        let file_already_included = included_file_indices[candidate.file_ix];
        let header_cost = if file_already_included {
            0
        } else {
            estimate_tokens(file_headers[candidate.file_ix].len() + file_suffix.len())
        };
        let excerpt_cost = excerpt_rendered_tokens(excerpt, file.max_row);
        if total_tokens + header_cost + excerpt_cost > max_tokens {
            break;
        }
        total_tokens += header_cost + excerpt_cost;
        if !file_already_included {
            included_file_indices[candidate.file_ix] = true;
        }
        included_excerpt_count += 1;
    }

    excerpt_candidates.truncate(included_excerpt_count);
    excerpt_candidates.sort_unstable_by_key(|c| (c.file_ix, c.excerpt_ix));

    // Render all of the files that fit within the token budget, in the original order.
    let mut result = String::new();
    let mut last_file_ix = None;
    for candidate in &excerpt_candidates {
        if last_file_ix != Some(candidate.file_ix) {
            if last_file_ix.is_some() {
                result.push_str(file_suffix);
            }
            result.push_str(&file_headers[candidate.file_ix]);
            last_file_ix = Some(candidate.file_ix);
        }
        let file = &related_files[candidate.file_ix];
        let excerpt = &file.excerpts[candidate.excerpt_ix];
        result.push_str(&excerpt.text);
        if !result.ends_with('\n') {
            result.push('\n');
        }
        if excerpt.row_range.end < file.max_row {
            result.push_str("...\n");
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

    pub fn special_tokens() -> &'static [&'static str] {
        &[
            "<|fim_prefix|>",
            "<|fim_suffix|>",
            "<|fim_middle|>",
            "<|file_sep|>",
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

    pub fn special_tokens() -> &'static [&'static str] {
        &[
            "<|fim_prefix|>",
            "<|fim_suffix|>",
            "<|fim_middle|>",
            "<|file_sep|>",
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

mod v0114180_editable_region {
    use super::*;

    pub fn special_tokens() -> &'static [&'static str] {
        v0113_ordered::special_tokens()
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

    pub fn special_tokens() -> &'static [&'static str] {
        v0131_git_merge_markers_prefix::special_tokens()
    }

    pub fn get_prefill(context: &str, editable_range: &Range<usize>) -> String {
        let editable_region = &context[editable_range.start..editable_range.end];

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

pub mod hashline {

    use std::fmt::Display;

    pub const END_MARKER: &str = "<|fim_middle|>updated";
    pub const START_MARKER: &str = "<|fim_middle|>current";

    use super::*;

    const SET_COMMAND_MARKER: &str = "<|set|>";
    const INSERT_COMMAND_MARKER: &str = "<|insert|>";

    pub fn special_tokens() -> &'static [&'static str] {
        return &[
            SET_COMMAND_MARKER,
            "<|set_range|>",
            INSERT_COMMAND_MARKER,
            CURSOR_MARKER,
            "<|file_sep|>",
            "<|fim_prefix|>",
            "<|fim_suffix|>",
            "<|fim_middle|>",
        ];
    }

    /// A parsed line reference like `3:c3` (line index 3 with hash 0xc3).
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct LineRef {
        index: usize,
        hash: u8,
    }

    impl Display for LineRef {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{:02x}", self.index, self.hash)
        }
    }

    pub fn hash_line(line: &[u8]) -> u8 {
        let mut h: u8 = 0;
        for &byte in line {
            h = h.wrapping_add(byte);
        }
        return h;
    }

    /// Write the hashline-encoded editable region into `out`. Each line of
    /// `editable_text` is prefixed with `{line_index}:{hash}|` and the cursor
    /// marker is inserted at `cursor_offset_in_editable` (byte offset relative
    /// to the start of `editable_text`).
    pub fn write_hashline_editable_region(
        out: &mut String,
        editable_text: &str,
        cursor_offset_in_editable: usize,
    ) {
        let mut offset = 0;
        for (i, line) in editable_text.lines().enumerate() {
            let (head, cursor, tail) = if cursor_offset_in_editable > offset
                && cursor_offset_in_editable < offset + line.len()
            {
                (
                    &line[..cursor_offset_in_editable - offset],
                    CURSOR_MARKER,
                    &line[cursor_offset_in_editable - offset..],
                )
            } else {
                (line, "", "")
            };
            write!(
                out,
                "\n{}|{head}{cursor}{tail}",
                LineRef {
                    index: i,
                    hash: hash_line(line.as_bytes())
                }
            )
            .unwrap();
            offset += line.len() + 1;
        }
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

        prompt.push_str("<|fim_prefix|>\n");
        prompt.push_str(&context[..editable_range.start]);
        prompt.push_str(START_MARKER);

        let cursor_offset_in_editable = cursor_offset.saturating_sub(editable_range.start);
        let editable_region = &context[editable_range.clone()];
        write_hashline_editable_region(prompt, editable_region, cursor_offset_in_editable);

        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("<|fim_suffix|>\n");
        prompt.push_str(&context[editable_range.end..]);
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str(END_MARKER);
    }

    /// A single edit command parsed from the model output.
    #[derive(Debug)]
    enum EditCommand<'a> {
        /// Replace a range of lines (inclusive on both ends). Single-line set is
        /// represented by `start == end`.
        Set {
            start: LineRef,
            end: LineRef,
            content: &'a str,
        },
        /// Insert new lines after the given line, or before the first line if
        /// `after` is `None`.
        Insert {
            after: Option<LineRef>,
            content: &'a str,
        },
    }

    /// Parse a line reference like `3:c3` into a `LineRef`.
    fn parse_line_ref(s: &str) -> Option<LineRef> {
        let (idx_str, hash_str) = s.split_once(':')?;
        let index = idx_str.parse::<usize>().ok()?;
        let hash = u8::from_str_radix(hash_str, 16).ok()?;
        Some(LineRef { index, hash })
    }

    /// Parse the model output into a list of `EditCommand`s.
    fn parse_edit_commands(model_output: &str) -> Vec<EditCommand<'_>> {
        let mut commands = Vec::new();
        let mut offset = 0usize;

        while offset < model_output.len() {
            let next_nl = model_output[offset..]
                .find('\n')
                .map(|i| offset + i)
                .unwrap_or(model_output.len());
            let line = &model_output[offset..next_nl];
            let line_end = if next_nl < model_output.len() {
                next_nl + 1
            } else {
                next_nl
            };

            let trimmed = line.trim();
            let (is_set, specifier) = if let Some(spec) = trimmed.strip_prefix(SET_COMMAND_MARKER) {
                (true, spec)
            } else if let Some(spec) = trimmed.strip_prefix(INSERT_COMMAND_MARKER) {
                (false, spec)
            } else {
                offset = line_end;
                continue;
            };

            let mut content_end = line_end;
            let mut scan = line_end;

            while scan < model_output.len() {
                let body_nl = model_output[scan..]
                    .find('\n')
                    .map(|i| scan + i)
                    .unwrap_or(model_output.len());
                let body_line = &model_output[scan..body_nl];
                if body_line.trim().starts_with(SET_COMMAND_MARKER)
                    || body_line.trim().starts_with(INSERT_COMMAND_MARKER)
                {
                    break;
                }
                scan = if body_nl < model_output.len() {
                    body_nl + 1
                } else {
                    body_nl
                };
                content_end = scan;
            }

            let content = &model_output[line_end..content_end];

            if is_set {
                if let Some((start_str, end_str)) = specifier.split_once('-') {
                    if let (Some(start), Some(end)) =
                        (parse_line_ref(start_str), parse_line_ref(end_str))
                    {
                        commands.push(EditCommand::Set {
                            start,
                            end,
                            content,
                        });
                    }
                } else if let Some(target) = parse_line_ref(specifier) {
                    commands.push(EditCommand::Set {
                        start: target.clone(),
                        end: target,
                        content,
                    });
                }
            } else {
                let after = parse_line_ref(specifier);
                commands.push(EditCommand::Insert { after, content });
            }

            offset = scan;
        }

        commands
    }

    /// Returns `true` if the model output contains `<|set|>` or `<|insert|>` commands
    /// (as opposed to being a plain full-replacement output).
    /// Strip the `{line_num}:{hash}|` prefixes from each line of a hashline-encoded
    /// editable region, returning the plain text content.
    pub fn strip_hashline_prefixes(region: &str) -> String {
        let mut decoded: String = region
            .lines()
            .map(|line| line.find('|').map_or(line, |pos| &line[pos + 1..]))
            .collect::<Vec<_>>()
            .join("\n");
        if region.ends_with('\n') {
            decoded.push('\n');
        }
        decoded
    }

    pub fn output_has_edit_commands(model_output: &str) -> bool {
        model_output.contains(SET_COMMAND_MARKER) || model_output.contains(INSERT_COMMAND_MARKER)
    }

    /// Apply `<|set|>` and `<|insert|>` edit commands from the model output to the
    /// original editable region text.
    ///
    /// `editable_region` is the original text of the editable region (without hash
    /// prefixes). `model_output` is the raw model response containing edit commands.
    ///
    /// Returns the full replacement text for the editable region.
    pub fn apply_edit_commands(editable_region: &str, model_output: &str) -> String {
        let original_lines: Vec<&str> = editable_region.lines().collect();
        let old_hashes: Vec<u8> = original_lines
            .iter()
            .map(|line| hash_line(line.as_bytes()))
            .collect();

        let commands = parse_edit_commands(model_output);

        // For set operations: indexed by start line → Some((end line index, content))
        // For insert operations: indexed by line index → vec of content to insert after
        // Insert-before-first is tracked separately.
        let mut set_ops: Vec<Option<(usize, &str)>> = vec![None; original_lines.len()];
        let mut insert_before_first: Vec<&str> = Vec::new();
        let mut insert_after: Vec<Vec<&str>> = vec![Vec::new(); original_lines.len()];

        for command in &commands {
            match command {
                EditCommand::Set {
                    start,
                    end,
                    content,
                } => {
                    if start.index < old_hashes.len()
                        && end.index < old_hashes.len()
                        && start.index <= end.index
                        && old_hashes[start.index] == start.hash
                        && old_hashes[end.index] == end.hash
                    {
                        set_ops[start.index] = Some((end.index, *content));
                    }
                }
                EditCommand::Insert { after, content } => match after {
                    None => insert_before_first.push(*content),
                    Some(line_ref) => {
                        if line_ref.index < old_hashes.len()
                            && old_hashes[line_ref.index] == line_ref.hash
                        {
                            insert_after[line_ref.index].push(*content);
                        }
                    }
                },
            }
        }

        let mut result = String::new();

        // Emit any insertions before the first line
        for content in &insert_before_first {
            result.push_str(content);
            if !content.ends_with('\n') {
                result.push('\n');
            }
        }

        let mut i = 0;
        while i < original_lines.len() {
            if let Some((end_index, replacement)) = set_ops[i].as_ref() {
                // Replace lines i..=end_index with the replacement content
                result.push_str(replacement);
                if !replacement.is_empty() && !replacement.ends_with('\n') {
                    result.push('\n');
                }
                // Emit any insertions after the end of this set range
                if *end_index < insert_after.len() {
                    for content in &insert_after[*end_index] {
                        result.push_str(content);
                        if !content.ends_with('\n') {
                            result.push('\n');
                        }
                    }
                }
                i = end_index + 1;
            } else {
                // Keep the original line
                result.push_str(original_lines[i]);
                result.push('\n');
                // Emit any insertions after this line
                for content in &insert_after[i] {
                    result.push_str(content);
                    if !content.ends_with('\n') {
                        result.push('\n');
                    }
                }
                i += 1;
            }
        }

        // Preserve trailing newline behavior: if the original ended with a
        // newline the result already has one; if it didn't, trim the extra one
        // we added.
        if !editable_region.ends_with('\n') && result.ends_with('\n') {
            result.pop();
        }

        result
    }

    /// Convert a unified diff patch into hashline edit commands.
    ///
    /// Parses the unified diff `patch` directly to determine which lines of
    /// `old_text` are deleted/replaced and what new lines are added, then emits
    /// `<|set|>` and `<|insert|>` edit commands referencing old lines by their
    /// `{index}:{hash}` identifiers.
    ///
    /// `cursor_offset` is an optional byte offset into the first hunk's new
    /// text (context + additions) where the cursor marker should be placed.
    pub fn patch_to_edit_commands(
        old_text: &str,
        patch: &str,
        cursor_offset: Option<usize>,
    ) -> Result<String> {
        let old_lines: Vec<&str> = old_text.lines().collect();
        let old_hashes: Vec<u8> = old_lines
            .iter()
            .map(|line| hash_line(line.as_bytes()))
            .collect();

        let mut result = String::new();
        let mut first_hunk = true;

        struct Hunk<'a> {
            line_range: Range<usize>,
            new_text_lines: Vec<&'a str>,
            cursor_line_offset_in_new_text: Option<(usize, usize)>,
        }

        // Parse the patch line by line. We only care about hunk headers,
        // context, deletions, and additions.
        let mut old_line_index: usize = 0;
        let mut current_hunk: Option<Hunk> = None;
        // Byte offset tracking within the hunk's new text for cursor placement.
        let mut new_text_byte_offset: usize = 0;
        // The line index of the last old line seen before/in the current hunk
        // (used for insert-after reference).
        let mut last_old_line_before_hunk: Option<usize> = None;

        fn flush_hunk(
            hunk: Hunk,
            last_old_line: Option<usize>,
            result: &mut String,
            old_hashes: &[u8],
        ) {
            if hunk.line_range.is_empty() {
                // Pure insertion — reference the old line to insert after when in bounds.
                if let Some(after) = last_old_line
                    && let Some(&hash) = old_hashes.get(after)
                {
                    write!(
                        result,
                        "{INSERT_COMMAND_MARKER}{}\n",
                        LineRef { index: after, hash }
                    )
                    .unwrap();
                } else {
                    result.push_str(INSERT_COMMAND_MARKER);
                    result.push('\n');
                }
            } else {
                let start = hunk.line_range.start;
                let end_exclusive = hunk.line_range.end;
                let deleted_line_count = end_exclusive.saturating_sub(start);

                if deleted_line_count == 1 {
                    if let Some(&hash) = old_hashes.get(start) {
                        write!(
                            result,
                            "{SET_COMMAND_MARKER}{}\n",
                            LineRef { index: start, hash }
                        )
                        .unwrap();
                    } else {
                        result.push_str(SET_COMMAND_MARKER);
                        result.push('\n');
                    }
                } else {
                    let end_inclusive = end_exclusive - 1;
                    match (
                        old_hashes.get(start).copied(),
                        old_hashes.get(end_inclusive).copied(),
                    ) {
                        (Some(start_hash), Some(end_hash)) => {
                            write!(
                                result,
                                "{SET_COMMAND_MARKER}{}-{}\n",
                                LineRef {
                                    index: start,
                                    hash: start_hash
                                },
                                LineRef {
                                    index: end_inclusive,
                                    hash: end_hash
                                }
                            )
                            .unwrap();
                        }
                        _ => {
                            result.push_str(SET_COMMAND_MARKER);
                            result.push('\n');
                        }
                    }
                }
            }
            for (line_offset, line) in hunk.new_text_lines.iter().enumerate() {
                if let Some((cursor_line_offset, char_offset)) = hunk.cursor_line_offset_in_new_text
                    && line_offset == cursor_line_offset
                {
                    result.push_str(&line[..char_offset]);
                    result.push_str(CURSOR_MARKER);
                    result.push_str(&line[char_offset..]);
                    continue;
                }

                result.push_str(line);
            }
        }

        for raw_line in patch.split_inclusive('\n') {
            if raw_line.starts_with("@@") {
                // Flush any pending change hunk from a previous patch hunk.
                if let Some(hunk) = current_hunk.take() {
                    flush_hunk(hunk, last_old_line_before_hunk, &mut result, &old_hashes);
                }

                // Parse hunk header: @@ -old_start[,old_count] +new_start[,new_count] @@
                // We intentionally do not trust old_start as a direct local index into `old_text`,
                // because some patches are produced against a larger file region and carry
                // non-local line numbers. We keep indexing local by advancing from parsed patch lines.
                if first_hunk {
                    new_text_byte_offset = 0;
                    first_hunk = false;
                }
                continue;
            }

            if raw_line.starts_with("---") || raw_line.starts_with("+++") {
                continue;
            }
            if raw_line.starts_with("\\ No newline") {
                continue;
            }

            if raw_line.starts_with('-') {
                // Extend or start a change hunk with this deleted old line.
                match &mut current_hunk {
                    Some(Hunk {
                        line_range: range, ..
                    }) => range.end = old_line_index + 1,
                    None => {
                        current_hunk = Some(Hunk {
                            line_range: old_line_index..old_line_index + 1,
                            new_text_lines: Vec::new(),
                            cursor_line_offset_in_new_text: None,
                        });
                    }
                }
                old_line_index += 1;
            } else if let Some(added_content) = raw_line.strip_prefix('+') {
                // Place cursor marker if cursor_offset falls within this line.
                let mut cursor_line_offset = None;
                if let Some(cursor_off) = cursor_offset
                    && (first_hunk
                        || cursor_off >= new_text_byte_offset
                            && cursor_off <= new_text_byte_offset + added_content.len())
                {
                    let line_offset = added_content.floor_char_boundary(
                        cursor_off
                            .saturating_sub(new_text_byte_offset)
                            .min(added_content.len()),
                    );
                    cursor_line_offset = Some(line_offset);
                }

                new_text_byte_offset += added_content.len();

                let hunk = current_hunk.get_or_insert(Hunk {
                    line_range: old_line_index..old_line_index,
                    new_text_lines: vec![],
                    cursor_line_offset_in_new_text: None,
                });
                hunk.new_text_lines.push(added_content);
                hunk.cursor_line_offset_in_new_text = cursor_line_offset
                    .map(|offset_in_line| (hunk.new_text_lines.len() - 1, offset_in_line));
            } else {
                // Context line (starts with ' ' or is empty).
                if let Some(hunk) = current_hunk.take() {
                    flush_hunk(hunk, last_old_line_before_hunk, &mut result, &old_hashes);
                }
                last_old_line_before_hunk = Some(old_line_index);
                old_line_index += 1;
                let content = raw_line.strip_prefix(' ').unwrap_or(raw_line);
                new_text_byte_offset += content.len();
            }
        }

        // Flush final group.
        if let Some(hunk) = current_hunk.take() {
            flush_hunk(hunk, last_old_line_before_hunk, &mut result, &old_hashes);
        }

        // Trim a single trailing newline.
        if result.ends_with('\n') {
            result.pop();
        }

        Ok(result)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use indoc::indoc;

        #[test]
        fn test_format_cursor_region() {
            struct Case {
                name: &'static str,
                context: &'static str,
                editable_range: Range<usize>,
                cursor_offset: usize,
                expected: &'static str,
            }

            let cases = [
                Case {
                    name: "basic_cursor_placement",
                    context: "hello world\n",
                    editable_range: 0..12,
                    cursor_offset: 5,
                    expected: indoc! {"
                    <|file_sep|>test.rs
                    <|fim_prefix|>
                    <|fim_middle|>current
                    0:5c|hello<|user_cursor|> world
                    <|fim_suffix|>
                    <|fim_middle|>updated"},
                },
                Case {
                    name: "multiline_cursor_on_second_line",
                    context: "aaa\nbbb\nccc\n",
                    editable_range: 0..12,
                    cursor_offset: 5, // byte 5 → 1 byte into "bbb"
                    expected: indoc! {"
                    <|file_sep|>test.rs
                    <|fim_prefix|>
                    <|fim_middle|>current
                    0:23|aaa
                    1:26|b<|user_cursor|>bb
                    2:29|ccc
                    <|fim_suffix|>
                    <|fim_middle|>updated"},
                },
                Case {
                    name: "no_trailing_newline_in_context",
                    context: "line1\nline2",
                    editable_range: 0..11,
                    cursor_offset: 3,
                    expected: indoc! {"
                    <|file_sep|>test.rs
                    <|fim_prefix|>
                    <|fim_middle|>current
                    0:d9|lin<|user_cursor|>e1
                    1:da|line2
                    <|fim_suffix|>
                    <|fim_middle|>updated"},
                },
                Case {
                    name: "leading_newline_in_editable_region",
                    context: "\nabc\n",
                    editable_range: 0..5,
                    cursor_offset: 2, // byte 2 = 'a' in "abc" (after leading \n)
                    expected: indoc! {"
                    <|file_sep|>test.rs
                    <|fim_prefix|>
                    <|fim_middle|>current
                    0:00|
                    1:26|a<|user_cursor|>bc
                    <|fim_suffix|>
                    <|fim_middle|>updated"},
                },
                Case {
                    name: "with_suffix",
                    context: "abc\ndef",
                    editable_range: 0..4, // editable region = "abc\n", suffix = "def"
                    cursor_offset: 2,
                    expected: indoc! {"
                    <|file_sep|>test.rs
                    <|fim_prefix|>
                    <|fim_middle|>current
                    0:26|ab<|user_cursor|>c
                    <|fim_suffix|>
                    def
                    <|fim_middle|>updated"},
                },
                Case {
                    name: "unicode_two_byte_chars",
                    context: "héllo\n",
                    editable_range: 0..7,
                    cursor_offset: 3, // byte 3 = after "hé" (h=1 byte, é=2 bytes), before "llo"
                    expected: indoc! {"
                    <|file_sep|>test.rs
                    <|fim_prefix|>
                    <|fim_middle|>current
                    0:1b|hé<|user_cursor|>llo
                    <|fim_suffix|>
                    <|fim_middle|>updated"},
                },
                Case {
                    name: "unicode_three_byte_chars",
                    context: "日本語\n",
                    editable_range: 0..10,
                    cursor_offset: 6, // byte 6 = after "日本" (3+3 bytes), before "語"
                    expected: indoc! {"
                    <|file_sep|>test.rs
                    <|fim_prefix|>
                    <|fim_middle|>current
                    0:80|日本<|user_cursor|>語
                    <|fim_suffix|>
                    <|fim_middle|>updated"},
                },
                Case {
                    name: "unicode_four_byte_chars",
                    context: "a🌍b\n",
                    editable_range: 0..7,
                    cursor_offset: 5, // byte 5 = after "a🌍" (1+4 bytes), before "b"
                    expected: indoc! {"
                    <|file_sep|>test.rs
                    <|fim_prefix|>
                    <|fim_middle|>current
                    0:6b|a🌍<|user_cursor|>b
                    <|fim_suffix|>
                    <|fim_middle|>updated"},
                },
                Case {
                    name: "cursor_at_start_of_region_not_placed",
                    context: "abc\n",
                    editable_range: 0..4,
                    cursor_offset: 0, // cursor_offset(0) > offset(0) is false → cursor not placed
                    expected: indoc! {"
                    <|file_sep|>test.rs
                    <|fim_prefix|>
                    <|fim_middle|>current
                    0:26|abc
                    <|fim_suffix|>
                    <|fim_middle|>updated"},
                },
                Case {
                    name: "cursor_at_end_of_line_not_placed",
                    context: "abc\ndef\n",
                    editable_range: 0..8,
                    cursor_offset: 3, // byte 3 = the \n after "abc" → falls between lines, not placed
                    expected: indoc! {"
                    <|file_sep|>test.rs
                    <|fim_prefix|>
                    <|fim_middle|>current
                    0:26|abc
                    1:2f|def
                    <|fim_suffix|>
                    <|fim_middle|>updated"},
                },
                Case {
                    name: "cursor_offset_relative_to_context_not_editable_region",
                    // cursor_offset is relative to `context`, so when editable_range.start > 0,
                    // write_cursor_excerpt_section must subtract it before comparing against
                    // per-line offsets within the editable region.
                    context: "pre\naaa\nbbb\nsuf\n",
                    editable_range: 4..12, // editable region = "aaa\nbbb\n"
                    cursor_offset: 9,      // byte 9 in context = second 'b' in "bbb"
                    expected: indoc! {"
                    <|file_sep|>test.rs
                    <|fim_prefix|>
                    pre
                    <|fim_middle|>current
                    0:23|aaa
                    1:26|b<|user_cursor|>bb
                    <|fim_suffix|>
                    suf
                    <|fim_middle|>updated"},
                },
            ];

            for case in &cases {
                let mut prompt = String::new();
                hashline::write_cursor_excerpt_section(
                    &mut prompt,
                    Path::new("test.rs"),
                    case.context,
                    &case.editable_range,
                    case.cursor_offset,
                );
                assert_eq!(prompt, case.expected, "failed case: {}", case.name);
            }
        }

        #[test]
        fn test_apply_edit_commands() {
            struct Case {
                name: &'static str,
                original: &'static str,
                model_output: &'static str,
                expected: &'static str,
            }

            let cases = vec![
                Case {
                    name: "set_single_line",
                    original: indoc! {"
                    let mut total = 0;
                    for product in products {
                        total += ;
                    }
                    total
                "},
                    model_output: indoc! {"
                    <|set|>2:87
                        total += product.price;
                "},
                    expected: indoc! {"
                    let mut total = 0;
                    for product in products {
                        total += product.price;
                    }
                    total
                "},
                },
                Case {
                    name: "set_range",
                    original: indoc! {"
                    fn foo() {
                        let x = 1;
                        let y = 2;
                        let z = 3;
                    }
                "},
                    model_output: indoc! {"
                    <|set|>1:46-3:4a
                        let sum = 6;
                "},
                    expected: indoc! {"
                    fn foo() {
                        let sum = 6;
                    }
                "},
                },
                Case {
                    name: "insert_after_line",
                    original: indoc! {"
                    fn main() {
                        let x = 1;
                    }
                "},
                    model_output: indoc! {"
                    <|insert|>1:46
                        let y = 2;
                "},
                    expected: indoc! {"
                    fn main() {
                        let x = 1;
                        let y = 2;
                    }
                "},
                },
                Case {
                    name: "insert_before_first",
                    original: indoc! {"
                    let x = 1;
                    let y = 2;
                "},
                    model_output: indoc! {"
                    <|insert|>
                    use std::io;
                "},
                    expected: indoc! {"
                    use std::io;
                    let x = 1;
                    let y = 2;
                "},
                },
                Case {
                    name: "set_with_cursor_marker",
                    original: indoc! {"
                    fn main() {
                        println!();
                    }
                "},
                    model_output: indoc! {"
                    <|set|>1:34
                        eprintln!(\"<|user_cursor|>\");
                "},
                    expected: indoc! {"
                    fn main() {
                        eprintln!(\"<|user_cursor|>\");
                    }
                "},
                },
                Case {
                    name: "multiple_set_commands",
                    original: indoc! {"
                    aaa
                    bbb
                    ccc
                    ddd
                "},
                    model_output: indoc! {"
                    <|set|>0:23
                    AAA
                    <|set|>2:29
                    CCC
                "},
                    expected: indoc! {"
                    AAA
                    bbb
                    CCC
                    ddd
                "},
                },
                Case {
                    name: "set_range_multiline_replacement",
                    original: indoc! {"
                    fn handle_submit() {
                    }

                    fn handle_keystroke() {
                "},
                    model_output: indoc! {"
                    <|set|>0:3f-1:7d
                    fn handle_submit(modal_state: &mut ModalState) {
                        <|user_cursor|>
                    }
                "},
                    expected: indoc! {"
                    fn handle_submit(modal_state: &mut ModalState) {
                        <|user_cursor|>
                    }

                    fn handle_keystroke() {
                "},
                },
                Case {
                    name: "no_edit_commands_returns_original",
                    original: indoc! {"
                    hello
                    world
                "},
                    model_output: "some random text with no commands",
                    expected: indoc! {"
                    hello
                    world
                "},
                },
                Case {
                    name: "wrong_hash_set_ignored",
                    original: indoc! {"
                    aaa
                    bbb
                "},
                    model_output: indoc! {"
                    <|set|>0:ff
                    ZZZ
                "},
                    expected: indoc! {"
                    aaa
                    bbb
                "},
                },
                Case {
                    name: "insert_and_set_combined",
                    original: indoc! {"
                    alpha
                    beta
                    gamma
                "},
                    model_output: indoc! {"
                    <|set|>0:06
                    ALPHA
                    <|insert|>1:9c
                    beta_extra
                "},
                    expected: indoc! {"
                    ALPHA
                    beta
                    beta_extra
                    gamma
                "},
                },
                Case {
                    name: "no_trailing_newline_preserved",
                    original: "hello\nworld",
                    model_output: indoc! {"
                    <|set|>0:14
                    HELLO
                "},
                    expected: "HELLO\nworld",
                },
                Case {
                    name: "set_range_hash_mismatch_in_end_bound",
                    original: indoc! {"
                    one
                    two
                    three
                "},
                    model_output: indoc! {"
                    <|set|>0:42-2:ff
                    ONE_TWO_THREE
                "},
                    expected: indoc! {"
                    one
                    two
                    three
                "},
                },
                Case {
                    name: "set_range_start_greater_than_end_ignored",
                    original: indoc! {"
                    a
                    b
                    c
                "},
                    model_output: indoc! {"
                    <|set|>2:63-1:62
                    X
                "},
                    expected: indoc! {"
                    a
                    b
                    c
                "},
                },
                Case {
                    name: "insert_out_of_bounds_ignored",
                    original: indoc! {"
                    x
                    y
                "},
                    model_output: indoc! {"
                    <|insert|>99:aa
                    z
                "},
                    expected: indoc! {"
                    x
                    y
                "},
                },
                Case {
                    name: "set_out_of_bounds_ignored",
                    original: indoc! {"
                    x
                    y
                "},
                    model_output: indoc! {"
                    <|set|>99:aa
                    z
                "},
                    expected: indoc! {"
                    x
                    y
                "},
                },
                Case {
                    name: "malformed_set_command_ignored",
                    original: indoc! {"
                    alpha
                    beta
                "},
                    model_output: indoc! {"
                    <|set|>not-a-line-ref
                    UPDATED
                "},
                    expected: indoc! {"
                    alpha
                    beta
                "},
                },
                Case {
                    name: "malformed_insert_hash_treated_as_before_first",
                    original: indoc! {"
                    alpha
                    beta
                "},
                    model_output: indoc! {"
                    <|insert|>1:nothex
                    preamble
                "},
                    expected: indoc! {"
                    preamble
                    alpha
                    beta
                "},
                },
                Case {
                    name: "set_then_insert_same_target_orders_insert_after_replacement",
                    original: indoc! {"
                    cat
                    dog
                "},
                    model_output: indoc! {"
                    <|set|>0:38
                    CAT
                    <|insert|>0:38
                    TAIL
                "},
                    expected: indoc! {"
                    CAT
                    TAIL
                    dog
                "},
                },
                Case {
                    name: "overlapping_set_ranges_last_wins",
                    original: indoc! {"
                    a
                    b
                    c
                    d
                "},
                    model_output: indoc! {"
                    <|set|>0:61-2:63
                    FIRST
                    <|set|>1:62-3:64
                    SECOND
                "},
                    expected: indoc! {"
                    FIRST
                    d
                "},
                },
                Case {
                    name: "insert_before_first_and_after_line",
                    original: indoc! {"
                    a
                    b
                "},
                    model_output: indoc! {"
                    <|insert|>
                    HEAD
                    <|insert|>0:61
                    MID
                "},
                    expected: indoc! {"
                    HEAD
                    a
                    MID
                    b
                "},
                },
            ];

            for case in &cases {
                let result = hashline::apply_edit_commands(case.original, &case.model_output);
                assert_eq!(result, case.expected, "failed case: {}", case.name);
            }
        }

        #[test]
        fn test_output_has_edit_commands() {
            assert!(hashline::output_has_edit_commands(&format!(
                "{}0:ab\nnew",
                SET_COMMAND_MARKER
            )));
            assert!(hashline::output_has_edit_commands(&format!(
                "{}0:ab\nnew",
                INSERT_COMMAND_MARKER
            )));
            assert!(hashline::output_has_edit_commands(&format!(
                "some text\n{}1:cd\nstuff",
                SET_COMMAND_MARKER
            )));
            assert!(!hashline::output_has_edit_commands("just plain text"));
            assert!(!hashline::output_has_edit_commands("NO_EDITS"));
        }

        // ---- hashline::patch_to_edit_commands round-trip tests ----

        #[test]
        fn test_patch_to_edit_commands() {
            struct Case {
                name: &'static str,
                old: &'static str,
                patch: &'static str,
                expected_new: &'static str,
            }

            let cases = [
                Case {
                    name: "single_line_replacement",
                    old: indoc! {"
                    let mut total = 0;
                    for product in products {
                        total += ;
                    }
                    total
                "},
                    patch: indoc! {"
                    @@ -1,5 +1,5 @@
                     let mut total = 0;
                     for product in products {
                    -    total += ;
                    +    total += product.price;
                     }
                     total
                "},
                    expected_new: indoc! {"
                    let mut total = 0;
                    for product in products {
                        total += product.price;
                    }
                    total
                "},
                },
                Case {
                    name: "multiline_replacement",
                    old: indoc! {"
                    fn foo() {
                        let x = 1;
                        let y = 2;
                        let z = 3;
                    }
                "},
                    patch: indoc! {"
                    @@ -1,5 +1,3 @@
                     fn foo() {
                    -    let x = 1;
                    -    let y = 2;
                    -    let z = 3;
                    +    let sum = 1 + 2 + 3;
                     }
                "},
                    expected_new: indoc! {"
                    fn foo() {
                        let sum = 1 + 2 + 3;
                    }
                "},
                },
                Case {
                    name: "insertion",
                    old: indoc! {"
                    fn main() {
                        let x = 1;
                    }
                "},
                    patch: indoc! {"
                    @@ -1,3 +1,4 @@
                     fn main() {
                         let x = 1;
                    +    let y = 2;
                     }
                "},
                    expected_new: indoc! {"
                    fn main() {
                        let x = 1;
                        let y = 2;
                    }
                "},
                },
                Case {
                    name: "insertion_before_first",
                    old: indoc! {"
                    let x = 1;
                    let y = 2;
                "},
                    patch: indoc! {"
                    @@ -1,2 +1,3 @@
                    +use std::io;
                     let x = 1;
                     let y = 2;
                "},
                    expected_new: indoc! {"
                    use std::io;
                    let x = 1;
                    let y = 2;
                "},
                },
                Case {
                    name: "deletion",
                    old: indoc! {"
                    aaa
                    bbb
                    ccc
                    ddd
                "},
                    patch: indoc! {"
                    @@ -1,4 +1,2 @@
                     aaa
                    -bbb
                    -ccc
                     ddd
                "},
                    expected_new: indoc! {"
                    aaa
                    ddd
                "},
                },
                Case {
                    name: "multiple_changes",
                    old: indoc! {"
                    alpha
                    beta
                    gamma
                    delta
                    epsilon
                "},
                    patch: indoc! {"
                    @@ -1,5 +1,5 @@
                    -alpha
                    +ALPHA
                     beta
                     gamma
                    -delta
                    +DELTA
                     epsilon
                "},
                    expected_new: indoc! {"
                    ALPHA
                    beta
                    gamma
                    DELTA
                    epsilon
                "},
                },
                Case {
                    name: "replace_with_insertion",
                    old: indoc! {r#"
                    fn handle() {
                        modal_state.close();
                        modal_state.dismiss();
                "#},
                    patch: indoc! {r#"
                    @@ -1,3 +1,4 @@
                     fn handle() {
                         modal_state.close();
                    +    eprintln!("");
                         modal_state.dismiss();
                "#},
                    expected_new: indoc! {r#"
                    fn handle() {
                        modal_state.close();
                        eprintln!("");
                        modal_state.dismiss();
                "#},
                },
                Case {
                    name: "complete_replacement",
                    old: indoc! {"
                    aaa
                    bbb
                    ccc
                "},
                    patch: indoc! {"
                    @@ -1,3 +1,3 @@
                    -aaa
                    -bbb
                    -ccc
                    +xxx
                    +yyy
                    +zzz
                "},
                    expected_new: indoc! {"
                    xxx
                    yyy
                    zzz
                "},
                },
                Case {
                    name: "add_function_body",
                    old: indoc! {"
                    fn foo() {
                        modal_state.dismiss();
                    }

                    fn

                    fn handle_keystroke() {
                "},
                    patch: indoc! {"
                    @@ -1,6 +1,8 @@
                     fn foo() {
                         modal_state.dismiss();
                     }

                    -fn
                    +fn handle_submit() {
                    +    todo()
                    +}

                     fn handle_keystroke() {
                "},
                    expected_new: indoc! {"
                    fn foo() {
                        modal_state.dismiss();
                    }

                    fn handle_submit() {
                        todo()
                    }

                    fn handle_keystroke() {
                "},
                },
                Case {
                    name: "with_cursor_offset",
                    old: indoc! {r#"
                    fn main() {
                        println!();
                    }
                "#},
                    patch: indoc! {r#"
                    @@ -1,3 +1,3 @@
                     fn main() {
                    -    println!();
                    +    eprintln!("");
                     }
                "#},
                    expected_new: indoc! {r#"
                    fn main() {
                        eprintln!("<|user_cursor|>");
                    }
                "#},
                },
                Case {
                    name: "non_local_hunk_header_pure_insertion_repro",
                    old: indoc! {"
                    aaa
                    bbb
                "},
                    patch: indoc! {"
                    @@ -20,2 +20,3 @@
                     aaa
                    +xxx
                     bbb
                "},
                    expected_new: indoc! {"
                    aaa
                    xxx
                    bbb
                "},
                },
            ];

            for case in &cases {
                // The cursor_offset for patch_to_edit_commands is relative to
                // the first hunk's new text (context + additions). We compute
                // it by finding where the marker sits in the expected output
                // (which mirrors the new text of the hunk).
                let cursor_offset = case.expected_new.find(CURSOR_MARKER);

                let commands =
                    hashline::patch_to_edit_commands(case.old, case.patch, cursor_offset)
                        .unwrap_or_else(|e| panic!("failed case {}: {e}", case.name));

                assert!(
                    hashline::output_has_edit_commands(&commands),
                    "case {}: expected edit commands, got: {commands:?}",
                    case.name,
                );

                let applied = hashline::apply_edit_commands(case.old, &commands);
                assert_eq!(applied, case.expected_new, "case {}", case.name);
            }
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

    pub const NO_EDITS: &str = "NO_EDITS\n";

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

    pub fn write_cursor_excerpt_section(
        prompt: &mut String,
        path: &Path,
        context: &str,
        editable_range: &Range<usize>,
        cursor_offset: usize,
    ) {
        let section = build_cursor_prefix_section(path, context, editable_range, cursor_offset);
        prompt.push_str(&section);
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
            "",
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

    /// Format patch as containing no changes if it's empty; otherwise return None.
    pub(crate) fn no_edits(patch: &str) -> Option<String> {
        // Count lines in the patch
        let empty_patch = patch.lines().count() <= 3;
        if empty_patch {
            Some(format!("{NO_EDITS}{END_MARKER}"))
        } else {
            None
        }
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
        let context_range = 0..cursor_excerpt.len();
        ZetaPromptInput {
            cursor_path: Path::new("test.rs").into(),
            cursor_excerpt: cursor_excerpt.into(),
            cursor_offset_in_excerpt: cursor_offset,
            excerpt_start_row: None,
            events: events.into_iter().map(Arc::new).collect(),
            related_files,
            excerpt_ranges: ExcerptRanges {
                editable_150: editable_range.clone(),
                editable_180: editable_range.clone(),
                editable_350: editable_range,
                editable_150_context_350: context_range.clone(),
                editable_180_context_350: context_range.clone(),
                editable_350_context_150: context_range,
                ..Default::default()
            },
            experiment: None,
            in_open_source_repo: false,
            can_collect_data: false,
            repo_url: None,
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
                order: 0,
            }],
            in_open_source_repo: false,
        }
    }

    fn format_with_budget(input: &ZetaPromptInput, max_tokens: usize) -> String {
        format_prompt_with_budget_for_format(input, ZetaFormat::V0114180EditableRegion, max_tokens)
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
                        order: 0,
                    },
                    RelatedExcerpt {
                        row_range: 10..20,
                        text: "second excerpt\n".into(),
                        order: 0,
                    },
                    RelatedExcerpt {
                        row_range: 20..30,
                        text: "third excerpt\n".into(),
                        order: 0,
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
    fn test_truncation_prioritizes_lower_order_excerpts() {
        // Two files: file_a has a high-order excerpt, file_b has a low-order one.
        // With tight budget, only the lower-order excerpt from file_b should be included.
        let input = make_input(
            "x",
            0..1,
            0,
            vec![],
            vec![
                RelatedFile {
                    path: Path::new("file_a.rs").into(),
                    max_row: 10,
                    in_open_source_repo: false,
                    excerpts: vec![RelatedExcerpt {
                        row_range: 0..10,
                        text: "low priority content\n".into(),
                        order: 5,
                    }],
                },
                RelatedFile {
                    path: Path::new("file_b.rs").into(),
                    max_row: 10,
                    in_open_source_repo: false,
                    excerpts: vec![RelatedExcerpt {
                        row_range: 0..10,
                        text: "high priority content\n".into(),
                        order: 1,
                    }],
                },
            ],
        );

        // With large budget, both files included; rendered in stable lexicographic order.
        assert_eq!(
            format_with_budget(&input, 10000),
            indoc! {r#"
                <|file_sep|>file_a.rs
                low priority content
                <|file_sep|>file_b.rs
                high priority content
                <|file_sep|>test.rs
                <|fim_prefix|>
                <|fim_middle|>current
                <|user_cursor|>x
                <|fim_suffix|>
                <|fim_middle|>updated
            "#}
        );

        // With tight budget, only file_b (lower order) fits.
        // Cursor section is ~37 tokens, so budget 52 leaves ~15 for related files.
        // file_b header (7) + excerpt (7) = 14 tokens, which fits.
        // file_a would need another 14 tokens, which doesn't fit.
        assert_eq!(
            format_with_budget(&input, 52),
            indoc! {r#"
                <|file_sep|>file_b.rs
                high priority content
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
    fn test_truncation_drops_high_order_excerpts_within_file() {
        // A single file has excerpts at order 1 and order 3. With a tight budget,
        // only the order-1 excerpts are included while the order-3 excerpt is
        // dropped — even though they belong to the same file. This also preserves
        // the parent invariant: parent outline items have order ≤ their best
        // child, so they're always included when any child is.
        let input = make_input(
            "x",
            0..1,
            0,
            vec![],
            vec![RelatedFile {
                path: Path::new("mod.rs").into(),
                max_row: 30,
                in_open_source_repo: false,
                excerpts: vec![
                    RelatedExcerpt {
                        row_range: 0..5,
                        text: "mod header\n".into(),
                        order: 1,
                    },
                    RelatedExcerpt {
                        row_range: 5..15,
                        text: "important fn\n".into(),
                        order: 1,
                    },
                    RelatedExcerpt {
                        row_range: 15..30,
                        text: "less important fn\n".into(),
                        order: 3,
                    },
                ],
            }],
        );

        // With large budget, all three excerpts included.
        assert_eq!(
            format_with_budget(&input, 10000),
            indoc! {r#"
                <|file_sep|>mod.rs
                mod header
                ...
                important fn
                ...
                less important fn
                <|file_sep|>test.rs
                <|fim_prefix|>
                <|fim_middle|>current
                <|user_cursor|>x
                <|fim_suffix|>
                <|fim_middle|>updated
            "#}
        );

        // With tight budget, only order<=1 excerpts included (header + important fn).
        assert_eq!(
            format_with_budget(&input, 55),
            indoc! {r#"
                <|file_sep|>mod.rs
                mod header
                ...
                important fn
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
        format_prompt_with_budget_for_format(input, ZetaFormat::V0211SeedCoder, 10000)
    }

    fn format_seed_coder_with_budget(input: &ZetaPromptInput, max_tokens: usize) -> String {
        format_prompt_with_budget_for_format(input, ZetaFormat::V0211SeedCoder, max_tokens)
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
    fn test_seed_coder_truncation_prioritizes_lower_order() {
        let input = make_input(
            "code",
            0..4,
            2,
            vec![],
            vec![
                RelatedFile {
                    path: Path::new("low_prio.rs").into(),
                    max_row: 5,
                    in_open_source_repo: false,
                    excerpts: vec![RelatedExcerpt {
                        row_range: 0..5,
                        text: "low prio\n".into(),
                        order: 10,
                    }],
                },
                RelatedFile {
                    path: Path::new("high_prio.rs").into(),
                    max_row: 5,
                    in_open_source_repo: false,
                    excerpts: vec![RelatedExcerpt {
                        row_range: 0..5,
                        text: "high prio\n".into(),
                        order: 1,
                    }],
                },
            ],
        );

        // With large budget, both included; rendered in stable lexicographic order.
        assert_eq!(
            format_seed_coder(&input),
            indoc! {r#"
                <[fim-suffix]>
                <[fim-prefix]><filename>low_prio.rs
                low prio
                <filename>high_prio.rs
                high prio

                <filename>test.rs
                <<<<<<< CURRENT
                co<|user_cursor|>de
                =======
                <[fim-middle]>"#}
        );

        // With tight budget, only high_prio included.
        // Cursor sections cost 25 tokens, so budget 44 leaves 19 for related files.
        // high_prio header (7) + excerpt (3) = 10, fits. low_prio would add 10 more = 20 > 19.
        assert_eq!(
            format_seed_coder_with_budget(&input, 44),
            indoc! {r#"
                <[fim-suffix]>
                <[fim-prefix]><filename>high_prio.rs
                high prio

                <filename>test.rs
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
            cursor_offset_in_excerpt: 30,
            excerpt_start_row: Some(0),
            events: vec![Arc::new(make_event("other.rs", "-old\n+new\n"))],
            related_files: vec![],
            excerpt_ranges: ExcerptRanges {
                editable_150: 15..41,
                editable_180: 15..41,
                editable_350: 15..41,
                editable_150_context_350: 0..excerpt.len(),
                editable_180_context_350: 0..excerpt.len(),
                editable_350_context_150: 0..excerpt.len(),
                ..Default::default()
            },
            experiment: None,
            in_open_source_repo: false,
            can_collect_data: false,
            repo_url: None,
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
            cursor_offset_in_excerpt: 15,
            excerpt_start_row: Some(10),
            events: vec![],
            related_files: vec![],
            excerpt_ranges: ExcerptRanges {
                editable_150: 0..28,
                editable_180: 0..28,
                editable_350: 0..28,
                editable_150_context_350: 0..28,
                editable_180_context_350: 0..28,
                editable_350_context_150: 0..28,
                ..Default::default()
            },
            experiment: None,
            in_open_source_repo: false,
            can_collect_data: false,
            repo_url: None,
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
            cursor_offset_in_excerpt: 25,
            excerpt_start_row: Some(0),
            events: vec![],
            related_files: vec![],
            excerpt_ranges: ExcerptRanges {
                editable_150: editable_range.clone(),
                editable_180: editable_range.clone(),
                editable_350: editable_range.clone(),
                editable_150_context_350: context_range.clone(),
                editable_180_context_350: context_range.clone(),
                editable_350_context_150: context_range.clone(),
                ..Default::default()
            },
            experiment: None,
            in_open_source_repo: false,
            can_collect_data: false,
            repo_url: None,
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
