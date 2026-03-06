use crate::{
    FormatPromptArgs, PredictionProvider,
    example::{ActualCursor, Example, ExamplePrompt},
    headless::EpAppState,
    progress::{ExampleProgress, Step},
    retrieve_context::run_context_retrieval,
};
use anyhow::{Context as _, Result, anyhow};
use edit_prediction::udiff;
use gpui::AsyncApp;
use similar::DiffableStr;
use std::ops::Range;
use std::sync::Arc;
use zeta_prompt::{
    ZetaFormat, encode_patch_as_output_for_format, excerpt_range_for_format, format_zeta_prompt,
    output_end_marker_for_format, resolve_cursor_region,
};

pub async fn run_format_prompt(
    example: &mut Example,
    args: &FormatPromptArgs,
    app_state: Arc<EpAppState>,
    example_progress: &ExampleProgress,
    cx: AsyncApp,
) -> Result<()> {
    run_context_retrieval(example, app_state.clone(), example_progress, cx.clone()).await?;

    let step_progress = example_progress.start(Step::FormatPrompt);

    let prompt_inputs = example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs must be set after context retrieval")?;

    match args.provider {
        PredictionProvider::Teacher(_) | PredictionProvider::TeacherNonBatching(_) => {
            step_progress.set_substatus("formatting teacher prompt");

            let zeta_format = ZetaFormat::default();
            let (editable_range, context_range) =
                excerpt_range_for_format(zeta_format, &prompt_inputs.excerpt_ranges);

            let prompt = TeacherPrompt::format_prompt(example, editable_range, context_range);
            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output: String::new(),
                rejected_output: None,
                prefill: None,
                provider: args.provider,
            });
        }
        PredictionProvider::Zeta2(zeta_format) => {
            step_progress.set_substatus("formatting zeta2 prompt");

            let prompt = format_zeta_prompt(prompt_inputs, zeta_format);
            let prefill = zeta_prompt::get_prefill(prompt_inputs, zeta_format);
            let expected_output = example
                .spec
                .expected_patches_with_cursor_positions()
                .into_iter()
                .next()
                .and_then(|(expected_patch, expected_cursor_offset)| {
                    zeta2_output_for_patch(
                        prompt_inputs,
                        &expected_patch,
                        expected_cursor_offset,
                        zeta_format,
                    )
                    .ok()
                })
                .unwrap_or_default();

            let rejected_output = example.spec.rejected_patch.as_ref().and_then(|patch| {
                zeta2_output_for_patch(prompt_inputs, patch, None, zeta_format).ok()
            });

            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output,
                rejected_output,
                provider: args.provider,
                prefill: Some(prefill),
            });
        }
        _ => {
            panic!("Cannot format prompt for {:?}", args.provider);
        }
    };
    Ok(())
}

pub fn zeta2_output_for_patch(
    input: &zeta_prompt::ZetaPromptInput,
    patch: &str,
    cursor_offset: Option<usize>,
    version: ZetaFormat,
) -> Result<String> {
    let (context, editable_range, _, _) = resolve_cursor_region(input, version);
    let mut old_editable_region = context[editable_range].to_string();

    if !old_editable_region.ends_with_newline() {
        old_editable_region.push('\n');
    }

    if let Some(encoded_output) =
        encode_patch_as_output_for_format(version, &old_editable_region, patch, cursor_offset)?
    {
        return Ok(encoded_output);
    }

    let (mut result, first_hunk_offset) =
        udiff::apply_diff_to_string_with_hunk_offset(patch, &old_editable_region).with_context(
            || {
                format!(
                    "Patch:\n```\n{}```\n\nEditable region:\n```\n{}```",
                    patch, old_editable_region
                )
            },
        )?;

    if let Some(cursor_offset) = cursor_offset {
        // The cursor_offset is relative to the start of the hunk's new text (context + additions).
        // We need to add where the hunk context matched in the editable region to compute
        // the actual cursor position in the result.
        let hunk_start = first_hunk_offset.unwrap_or(0);
        let offset = result.floor_char_boundary((hunk_start + cursor_offset).min(result.len()));
        result.insert_str(offset, zeta_prompt::CURSOR_MARKER);
    }

    if let Some(end_marker) = output_end_marker_for_format(version) {
        if !result.ends_with('\n') {
            result.push('\n');
        }
        result.push_str(end_marker);
    }

    Ok(result)
}

pub struct TeacherPrompt;

impl TeacherPrompt {
    pub(crate) const EDITABLE_REGION_START: &str = "<|editable_region_start|>\n";
    pub(crate) const EDITABLE_REGION_END: &str = "\n<|editable_region_end|>";
    pub(crate) const USER_CURSOR_MARKER: &str = "<|user_cursor|>";
    pub(crate) const NO_EDITS: &str = "NO_EDITS";

    const MARKER_TAG_PREFIX: &str = "<|marker_";
    const MARKER_TAG_SUFFIX: &str = "|>";
    const MIN_BLOCK_LINES: usize = 3;
    const MAX_BLOCK_LINES: usize = 8;

    /// Truncate edit history to this number of last lines
    const MAX_HISTORY_LINES: usize = 128;

    pub fn format_prompt(
        example: &Example,
        editable_range: Range<usize>,
        context_range: Range<usize>,
    ) -> String {
        let edit_history = Self::format_edit_history(&example.spec.edit_history);
        let context = Self::format_context(example);
        let cursor_excerpt = Self::format_cursor_excerpt(example, editable_range, context_range);

        let prompt_template = crate::prompt_assets::get_prompt("teacher.md");
        let prompt = prompt_template
            .replace("{{context}}", &context)
            .replace("{{edit_history}}", &edit_history)
            .replace("{{cursor_excerpt}}", &cursor_excerpt);

        prompt
    }

    pub fn parse(example: &Example, response: &str) -> Result<(String, Option<ActualCursor>)> {
        // Check if the model indicated no edits are needed
        let no_edits = (String::new(), None);
        if let Some(last_codeblock) = extract_last_codeblock(&response) {
            if last_codeblock.trim() == Self::NO_EDITS {
                return Ok(no_edits);
            }
        }

        if response.trim().ends_with(Self::NO_EDITS) {
            return Ok(no_edits);
        }

        let prompt_inputs = example
            .prompt_inputs
            .as_ref()
            .context("example is missing prompt inputs")?;

        let zeta_format = ZetaFormat::default();
        let (editable_range, _) =
            excerpt_range_for_format(zeta_format, &prompt_inputs.excerpt_ranges);
        let excerpt = prompt_inputs.cursor_excerpt.as_ref();
        let old_editable_region = &excerpt[editable_range.clone()];
        let marker_offsets = Self::compute_marker_offsets(old_editable_region);

        // Extract the model's response from the last codeblock
        let codeblock =
            extract_last_codeblock(&response).context("no codeblock found in model response")?;
        let (start_num, end_num, raw_new_span) = Self::extract_marker_span(&codeblock)?;

        let start_idx = start_num
            .checked_sub(1)
            .context("marker numbers are 1-indexed")?;
        let end_idx = end_num
            .checked_sub(1)
            .context("marker numbers are 1-indexed")?;
        let start_byte = *marker_offsets
            .get(start_idx)
            .context("start marker number out of range")?;
        let end_byte = *marker_offsets
            .get(end_idx)
            .context("end marker number out of range")?;

        if start_byte > end_byte {
            return Err(anyhow!("start marker must come before end marker"));
        }

        // Handle cursor marker in the new span
        let cursor_in_span = raw_new_span.find(Self::USER_CURSOR_MARKER);
        let new_span = raw_new_span.replace(Self::USER_CURSOR_MARKER, "");

        // Match trailing-newline convention: the old span for non-last blocks ends
        // with '\n' (line boundary). The model might drop or add a trailing newline
        // due to the formatting around the end marker. Normalize to match the old span.
        let old_span = &old_editable_region[start_byte..end_byte];
        let mut new_span = new_span;
        if old_span.ends_with('\n') && !new_span.ends_with('\n') && !new_span.is_empty() {
            new_span.push('\n');
        }
        if !old_span.ends_with('\n') && new_span.ends_with('\n') {
            new_span.pop();
        }

        // Build the full new editable region
        let mut new_editable_region = String::new();
        new_editable_region.push_str(&old_editable_region[..start_byte]);
        new_editable_region.push_str(&new_span);
        new_editable_region.push_str(&old_editable_region[end_byte..]);

        // Compute cursor offset relative to the full new editable region
        let cursor_offset = cursor_in_span.map(|pos| start_byte + pos);

        // Normalize leading newlines: if old starts with newline but new doesn't,
        // prepend newline to new to preserve whitespace structure.
        if old_editable_region.starts_with('\n') && !new_editable_region.starts_with('\n') {
            new_editable_region.insert(0, '\n');
        }

        let editable_region_offset = editable_range.start;
        let editable_region_start_line = excerpt[..editable_region_offset].matches('\n').count();

        let editable_region_lines = old_editable_region.lines().count() as u32;
        let diff = language::unified_diff_with_context(
            old_editable_region,
            &new_editable_region,
            editable_region_start_line as u32,
            editable_region_start_line as u32,
            editable_region_lines,
        );

        let diff = indoc::formatdoc! {"
            --- a/{path}
            +++ b/{path}
            {diff}",
            path = example.spec.cursor_path.to_string_lossy(),
            diff = diff,
        };

        let actual_cursor = cursor_offset.map(|editable_region_cursor_offset| {
            ActualCursor::from_editable_region(
                &example.spec.cursor_path,
                editable_region_cursor_offset,
                &new_editable_region,
                excerpt,
                editable_region_offset,
                editable_region_start_line,
            )
        });

        Ok((diff, actual_cursor))
    }

    fn format_edit_history(edit_history: &str) -> String {
        let lines: Vec<&str> = edit_history.lines().collect();

        if lines.is_empty() {
            return "(No edit history)".to_string();
        }

        if lines.len() > Self::MAX_HISTORY_LINES {
            let truncated = lines[lines.len() - Self::MAX_HISTORY_LINES..].join("\n");
            format!("{truncated}\n[...truncated...]")
        } else {
            lines.join("\n")
        }
    }

    pub fn format_context(example: &Example) -> String {
        let related_files = example
            .prompt_inputs
            .as_ref()
            .and_then(|pi| pi.related_files.as_deref());
        let Some(related_files) = related_files else {
            return "(No context)".to_string();
        };

        if related_files.is_empty() {
            return "(No context)".to_string();
        }

        let prefix = "`````";
        let suffix = "`````\n\n";
        let max_tokens = 1024;
        zeta_prompt::format_related_files_within_budget(related_files, &prefix, &suffix, max_tokens)
    }

    fn format_cursor_excerpt(
        example: &Example,
        editable_range: Range<usize>,
        context_range: Range<usize>,
    ) -> String {
        let mut result = String::new();

        let prompt_inputs = example.prompt_inputs.as_ref().unwrap();
        let excerpt = prompt_inputs.cursor_excerpt.as_ref();
        let cursor_offset = prompt_inputs.cursor_offset_in_excerpt;

        let editable_text = &excerpt[editable_range.clone()];
        let marker_offsets = Self::compute_marker_offsets(editable_text);
        let cursor_in_editable = cursor_offset - editable_range.start;

        let path_str = example.spec.cursor_path.to_string_lossy();
        result.push_str(&format!("`````{path_str}\n"));

        // Read-only prefix context
        result.push_str(&excerpt[context_range.start..editable_range.start]);

        // Emit markers and block content
        for (i, &offset) in marker_offsets.iter().enumerate() {
            let marker_num = i + 1;

            // Ensure the marker tag starts on its own line
            if !result.is_empty() && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(&Self::marker_tag(marker_num));

            // Emit block content after every marker except the last
            if let Some(&next_offset) = marker_offsets.get(i + 1) {
                result.push('\n');
                let block = &editable_text[offset..next_offset];

                if cursor_in_editable >= offset && cursor_in_editable <= next_offset {
                    let cursor_in_block = cursor_in_editable - offset;
                    result.push_str(&block[..cursor_in_block]);
                    result.push_str(Self::USER_CURSOR_MARKER);
                    result.push_str(&block[cursor_in_block..]);
                } else {
                    result.push_str(block);
                }
            }
        }

        // Read-only suffix context
        result.push_str(&excerpt[editable_range.end..context_range.end]);
        result.push_str("\n`````");

        result
    }

    pub fn extract_editable_region(text: &str) -> Result<String> {
        // Try marker format first: extract all content between first and last markers,
        // stripping intermediate marker tags.
        if let Some(region) = Self::extract_editable_region_from_markers(text) {
            return Ok(region);
        }

        // Fall back to old editable region format
        let start = text
            .rfind(Self::EDITABLE_REGION_START)
            .map_or(0, |pos| pos + Self::EDITABLE_REGION_START.len());
        let end = text.rfind(Self::EDITABLE_REGION_END).unwrap_or(text.len());

        if start >= end {
            return Err(anyhow!("Invalid editable region markers"));
        }

        let region = &text[start..end];
        Ok(region.strip_suffix('\n').unwrap_or(region).to_string())
    }

    fn marker_tag(number: usize) -> String {
        format!(
            "{}{}{}",
            Self::MARKER_TAG_PREFIX,
            number,
            Self::MARKER_TAG_SUFFIX
        )
    }

    /// Compute byte offsets within `editable_text` where marker boundaries should be placed.
    ///
    /// Returns a sorted `Vec<usize>` that always starts with `0` and ends with
    /// `editable_text.len()`. Interior offsets are placed at line boundaries (right
    /// after a `\n`), preferring blank-line boundaries when available and respecting
    /// `MIN_BLOCK_LINES` / `MAX_BLOCK_LINES` constraints.
    fn compute_marker_offsets(editable_text: &str) -> Vec<usize> {
        if editable_text.is_empty() {
            return vec![0, 0];
        }

        let mut offsets = vec![0usize];
        let mut lines_since_last_marker = 0usize;
        let mut byte_offset = 0usize;

        for line in editable_text.split('\n') {
            let line_end = byte_offset + line.len() + 1; // +1 for the '\n'
            let is_past_end = line_end > editable_text.len();
            let actual_line_end = line_end.min(editable_text.len());
            lines_since_last_marker += 1;

            let is_blank = line.trim().is_empty();

            if !is_past_end && lines_since_last_marker >= Self::MIN_BLOCK_LINES {
                if is_blank {
                    // Found a blank-line boundary. Skip any consecutive blank
                    // lines so the next block starts with a non-blank line.
                    // We'll place the marker when we find that non-blank line
                    // (handled below by checking the *start* of each iteration).
                } else if lines_since_last_marker >= Self::MAX_BLOCK_LINES {
                    // Force a split at this line boundary.
                    offsets.push(actual_line_end);
                    lines_since_last_marker = 0;
                }
            }

            // Check if this is a non-blank line that immediately follows blank
            // line(s) — if so, and the preceding block is long enough, place a
            // marker here so the new block starts with this non-blank line.
            if !is_blank && byte_offset > 0 && lines_since_last_marker >= Self::MIN_BLOCK_LINES {
                let before = &editable_text[..byte_offset];
                // `before` ends with '\n' (we're at a line start). Strip it to
                // reach the end of the preceding line, then check whether that
                // line was blank (empty or whitespace-only).
                let has_preceding_blank_line = before
                    .strip_suffix('\n')
                    .and_then(|stripped| {
                        let last_line = match stripped.rfind('\n') {
                            Some(pos) => &stripped[pos + 1..],
                            None => stripped,
                        };
                        Some(last_line.trim().is_empty())
                    })
                    .unwrap_or(false);

                if has_preceding_blank_line {
                    offsets.push(byte_offset);
                    lines_since_last_marker = 1; // current line counts toward the new block
                }
            }

            byte_offset = actual_line_end;

            // Handle forced max-line split (re-check after the blank-line logic
            // since lines_since_last_marker may have been reset).
            if !is_past_end && lines_since_last_marker >= Self::MAX_BLOCK_LINES {
                if *offsets.last().unwrap_or(&0) != actual_line_end {
                    offsets.push(actual_line_end);
                    lines_since_last_marker = 0;
                }
            }
        }

        // Always end with editable_text.len()
        let end = editable_text.len();
        if *offsets.last().unwrap_or(&0) != end {
            offsets.push(end);
        }

        offsets
    }

    /// Parse a model output codeblock that uses the marker format.
    ///
    /// Returns `(start_marker_num, end_marker_num, content_between_markers)`.
    /// The content has the format-level newlines (after start marker, before end
    /// marker) stripped so it corresponds to the raw editable region text.
    fn extract_marker_span(text: &str) -> Result<(usize, usize, String)> {
        let first_tag_start = text
            .find(Self::MARKER_TAG_PREFIX)
            .context("no start marker found in output")?;
        let first_num_start = first_tag_start + Self::MARKER_TAG_PREFIX.len();
        let first_num_end = text[first_num_start..]
            .find(Self::MARKER_TAG_SUFFIX)
            .map(|i| i + first_num_start)
            .context("malformed start marker tag")?;
        let start_num: usize = text[first_num_start..first_num_end]
            .parse()
            .context("start marker number is not a valid integer")?;
        let first_tag_end = first_num_end + Self::MARKER_TAG_SUFFIX.len();

        let last_tag_start = text
            .rfind(Self::MARKER_TAG_PREFIX)
            .context("no end marker found in output")?;
        let last_num_start = last_tag_start + Self::MARKER_TAG_PREFIX.len();
        let last_num_end = text[last_num_start..]
            .find(Self::MARKER_TAG_SUFFIX)
            .map(|i| i + last_num_start)
            .context("malformed end marker tag")?;
        let end_num: usize = text[last_num_start..last_num_end]
            .parse()
            .context("end marker number is not a valid integer")?;

        if start_num == end_num {
            return Err(anyhow!(
                "start and end markers are the same (marker {})",
                start_num
            ));
        }

        // Content sits between the end of the first marker tag and the start of
        // the last marker tag. Strip the format-level '\n' separators.
        let mut content_start = first_tag_end;
        if text.as_bytes().get(content_start) == Some(&b'\n') {
            content_start += 1;
        }
        let mut content_end = last_tag_start;
        if content_end > content_start && text.as_bytes().get(content_end - 1) == Some(&b'\n') {
            content_end -= 1;
        }

        let content = &text[content_start..content_end.max(content_start)];
        Ok((start_num, end_num, content.to_string()))
    }

    /// Extract the full editable region text from a prompt that uses marker tags.
    ///
    /// Returns the concatenation of all block contents between the first and last
    /// markers, with intermediate marker tags stripped.
    fn extract_editable_region_from_markers(text: &str) -> Option<String> {
        let first_marker_start = text.find(Self::MARKER_TAG_PREFIX)?;

        // Find all marker tags and collect their positions
        let mut markers: Vec<(usize, usize)> = Vec::new(); // (tag_start, tag_end)
        let mut search_start = first_marker_start;
        while let Some(rel_pos) = text[search_start..].find(Self::MARKER_TAG_PREFIX) {
            let tag_start = search_start + rel_pos;
            let num_start = tag_start + Self::MARKER_TAG_PREFIX.len();
            let num_end = text[num_start..].find(Self::MARKER_TAG_SUFFIX)?;
            let tag_end = num_start + num_end + Self::MARKER_TAG_SUFFIX.len();
            markers.push((tag_start, tag_end));
            search_start = tag_end;
        }

        if markers.len() < 2 {
            return None;
        }

        // Content spans from after the first marker tag to before the last marker tag.
        let (_, first_tag_end) = markers[0];
        let (last_tag_start, _) = markers[markers.len() - 1];

        let mut content_start = first_tag_end;
        if text.as_bytes().get(content_start) == Some(&b'\n') {
            content_start += 1;
        }
        let mut content_end = last_tag_start;
        if content_end > content_start && text.as_bytes().get(content_end - 1) == Some(&b'\n') {
            content_end -= 1;
        }

        let raw = &text[content_start..content_end];

        // Strip intermediate marker tags (and their trailing '\n')
        let mut result = String::with_capacity(raw.len());
        let mut pos = 0;
        let raw_bytes = raw.as_bytes();
        while let Some(rel) = raw[pos..].find(Self::MARKER_TAG_PREFIX) {
            result.push_str(&raw[pos..pos + rel]);
            let tag_num_start = pos + rel + Self::MARKER_TAG_PREFIX.len();
            let tag_suffix_pos = raw[tag_num_start..]
                .find(Self::MARKER_TAG_SUFFIX)
                .map(|i| i + tag_num_start)?;
            let mut tag_end = tag_suffix_pos + Self::MARKER_TAG_SUFFIX.len();
            // Also strip the '\n' that follows the marker tag
            if raw_bytes.get(tag_end) == Some(&b'\n') {
                tag_end += 1;
            }
            pos = tag_end;
        }
        result.push_str(&raw[pos..]);

        let result = result.strip_suffix('\n').unwrap_or(&result).to_string();
        Some(result)
    }
}

/// Extract the cursor excerpt from an example.
/// First tries to extract from an existing prompt, then falls back to constructing from prompt_inputs.
pub fn extract_cursor_excerpt_from_example(example: &Example) -> Option<String> {
    // If we have the original prompt, extract the cursor excerpt from it
    if let Some(prompt) = &example.prompt {
        // Find "# 3. Current File" section and extract the content
        if let Some(start) = prompt.input.find("# 3. Current File") {
            let content_start = prompt.input[start..].find('`').map(|i| start + i)?;
            let backtick_count = prompt.input[content_start..]
                .chars()
                .take_while(|&c| c == '`')
                .count();
            let content_start = content_start + backtick_count;

            // Find the path line and skip it
            let newline_pos = prompt.input[content_start..].find('\n')?;
            let text_start = content_start + newline_pos + 1;

            // Find the closing backticks
            let closing_pattern = "`".repeat(backtick_count);
            let text_end = prompt.input[text_start..].find(&closing_pattern)?;
            let cursor_excerpt = &prompt.input[text_start..text_start + text_end];

            let path_str = example.spec.cursor_path.to_string_lossy();
            return Some(format!("`````{path_str}\n{cursor_excerpt}`````"));
        }
    }

    // Fallback: construct from prompt_inputs if available
    let prompt_inputs = example.prompt_inputs.as_ref()?;
    let excerpt = prompt_inputs.cursor_excerpt.as_ref();
    let cursor_offset = prompt_inputs.cursor_offset_in_excerpt;

    // Simple fallback: wrap entire excerpt in two markers with cursor
    let path_str = example.spec.cursor_path.to_string_lossy();
    let mut result = format!("`````{path_str}\n");
    result.push_str(&TeacherPrompt::marker_tag(1));
    result.push('\n');
    result.push_str(&excerpt[..cursor_offset]);
    result.push_str(TeacherPrompt::USER_CURSOR_MARKER);
    result.push_str(&excerpt[cursor_offset..]);
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result.push_str(&TeacherPrompt::marker_tag(2));
    result.push_str("\n`````");

    Some(result)
}

pub(crate) fn extract_last_codeblock(text: &str) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();

    // Search from the end for a closing fence (line containing only backticks, 3+)
    let mut closing_line_idx = None;
    let mut backtick_count = 0;

    for i in (0..lines.len()).rev() {
        let line = lines[i].trim();
        if line.len() >= 3 && line.chars().all(|c| c == '`') {
            closing_line_idx = Some(i);
            backtick_count = line.len();
            break;
        }
    }

    let closing_idx = closing_line_idx?;

    // Search backwards for matching opening fence
    // Opening fence starts with same backtick count, possibly followed by language/metadata
    let opening_pattern = "`".repeat(backtick_count);

    for i in (0..closing_idx).rev() {
        let line = lines[i];
        if line.starts_with(&opening_pattern) {
            // Ensure it's exactly the right number of backticks (not more)
            let rest = &line[backtick_count..];
            if rest.is_empty() || !rest.starts_with('`') {
                // Found matching opening fence
                // Extract content between opening and closing (exclusive)
                if closing_idx > i + 1 {
                    let content = lines[i + 1..closing_idx].join("\n");
                    // Preserve trailing newline to match previous behavior
                    return Some(format!("{}\n", content));
                } else {
                    // Empty block
                    return Some(String::new());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_last_code_block() {
        let text = indoc::indoc! {"
            Some thinking

            ```
            first block
            ```

            `````path='something' lines=1:2
            last block
            `````
            "};
        let last_block = extract_last_codeblock(text).unwrap();
        assert_eq!(last_block, "last block\n");
    }

    #[test]
    fn test_extract_codeblock_with_nested_fences() {
        let text = indoc::indoc! {"
            `````
            content with ``` inline
            and ```python nested
            more content
            `````
            "};
        let last_block = extract_last_codeblock(text).unwrap();
        assert_eq!(
            last_block,
            "content with ``` inline\nand ```python nested\nmore content\n"
        );
    }

    #[test]
    fn test_extract_codeblock_ignores_inline_backticks() {
        let text = indoc::indoc! {"
            `````
            here is some `code` with inline backticks
            and here```more```stuff
            `````
            "};
        let last_block = extract_last_codeblock(text).unwrap();
        assert_eq!(
            last_block,
            "here is some `code` with inline backticks\nand here```more```stuff\n"
        );
    }

    #[test]
    fn test_extract_editable_region_old_format() {
        let text = indoc::indoc! {"
            some lines
            are
            here
            <|editable_region_start|>
            one
            two three

            <|editable_region_end|>
            more
            lines here
            "};
        let parsed = TeacherPrompt::extract_editable_region(text).unwrap();
        assert_eq!(
            parsed,
            indoc::indoc! {"
            one
            two three"}
        );
    }

    #[test]
    fn test_extract_editable_region_marker_format() {
        let text = indoc::indoc! {"
            some context
            <|marker_1|>
            one
            two three
            <|marker_2|>
            more context
            "};
        let parsed = TeacherPrompt::extract_editable_region(text).unwrap();
        assert_eq!(parsed, "one\ntwo three");
    }

    #[test]
    fn test_extract_editable_region_multi_markers() {
        let text = indoc::indoc! {"
            prefix
            <|marker_1|>
            aaa
            bbb
            <|marker_2|>
            ccc
            ddd
            <|marker_3|>
            suffix
            "};
        let parsed = TeacherPrompt::extract_editable_region(text).unwrap();
        // Intermediate marker and its trailing \n are stripped
        assert_eq!(parsed, "aaa\nbbb\nccc\nddd");
    }

    #[test]
    fn test_extract_last_codeblock_nested_bibtex() {
        let text = indoc::indoc! {r#"
            Looking at the edit history, I can see that a Citation section was just added.

            `````
            ## Collaborations
            Our mission is to create a 4D generative model.

            ## Citation

            If you found Unique3D helpful, please cite our report:
            ```bibtex
            @misc{wu2024unique3d,
                  title={Unique3D},
            }
            ```
            `````
            "#};
        let last_block = extract_last_codeblock(text).unwrap();
        assert_eq!(
            last_block,
            indoc::indoc! {r#"
            ## Collaborations
            Our mission is to create a 4D generative model.

            ## Citation

            If you found Unique3D helpful, please cite our report:
            ```bibtex
            @misc{wu2024unique3d,
                  title={Unique3D},
            }
            ```
            "#}
        );
    }

    #[test]
    fn test_extract_editable_region_no_markers() {
        let text = indoc::indoc! {"
            one
            two three"};
        let parsed = TeacherPrompt::extract_editable_region(text).unwrap();
        assert_eq!(
            parsed,
            indoc::indoc! {"
            one
            two three"}
        );
    }

    #[test]
    fn test_compute_marker_offsets_small_block() {
        // A small block (< MAX_BLOCK_LINES, no blank lines) should get just start+end
        let text = "aaa\nbbb\nccc\n";
        let offsets = TeacherPrompt::compute_marker_offsets(text);
        assert_eq!(offsets, vec![0, text.len()]);
    }

    #[test]
    fn test_compute_marker_offsets_blank_line_split() {
        // After the blank line, the remaining block needs >= MIN_BLOCK_LINES too
        let text = "aaa\nbbb\nccc\n\nddd\neee\nfff\n";
        let offsets = TeacherPrompt::compute_marker_offsets(text);
        // The blank line comes after 3 non-blank lines (aaa, bbb, ccc).
        // Including the blank line itself, that's 4 lines in block 1.
        // Remaining block (ddd, eee, fff) has 3 lines, also enough.
        // Marker goes at byte offset 13: right before "ddd".
        //   a(0)a(1)a(2)\n(3)b(4)b(5)b(6)\n(7)c(8)c(9)c(10)\n(11)\n(12)d(13)...
        assert_eq!(offsets[0], 0);
        assert!(offsets.contains(&13), "offsets: {:?}", offsets);
        assert_eq!(*offsets.last().unwrap(), text.len());
    }

    #[test]
    fn test_compute_marker_offsets_max_lines_split() {
        // A long block with no blank lines should be force-split at MAX_BLOCK_LINES
        let text = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n";
        let offsets = TeacherPrompt::compute_marker_offsets(text);
        // Should have a split somewhere due to MAX_BLOCK_LINES=8
        assert!(offsets.len() >= 3, "offsets: {:?}", offsets);
    }

    #[test]
    fn test_compute_marker_offsets_empty() {
        let offsets = TeacherPrompt::compute_marker_offsets("");
        assert_eq!(offsets, vec![0, 0]);
    }

    #[test]
    fn test_extract_marker_span() {
        let text = "<|marker_2|>\n    new content\n<|marker_3|>\n";
        let (start, end, content) = TeacherPrompt::extract_marker_span(text).unwrap();
        assert_eq!(start, 2);
        assert_eq!(end, 3);
        assert_eq!(content, "    new content");
    }

    #[test]
    fn test_extract_marker_span_multi_line() {
        let text = "<|marker_1|>\nline1\nline2\nline3\n<|marker_4|>";
        let (start, end, content) = TeacherPrompt::extract_marker_span(text).unwrap();
        assert_eq!(start, 1);
        assert_eq!(end, 4);
        assert_eq!(content, "line1\nline2\nline3");
    }

    #[test]
    fn test_parse_no_edits_response() {
        let response = indoc::indoc! {"
            The code is already complete. There is no clear next edit to make.

            `````
            NO_EDITS
            `````
        "};
        let codeblock = extract_last_codeblock(response).unwrap();
        assert_eq!(codeblock.trim(), TeacherPrompt::NO_EDITS);
    }

    #[test]
    fn test_extract_codeblock_no_valid_block() {
        // Text with no code blocks should return None
        let text = "Just some plain text without any code blocks";
        assert!(extract_last_codeblock(text).is_none());

        // Unclosed code block should return None
        let text = indoc::indoc! {"
            ```
            unclosed block
        "};
        assert!(extract_last_codeblock(text).is_none());

        // Analysis text with nested markdown but no proper outer block
        let text = indoc::indoc! {"
            # Analysis
            Looking at this:
            ```
            some code
            ```
            But then more analysis without wrapping block
        "};
        // This should find the inner block
        let result = extract_last_codeblock(text).unwrap();
        assert_eq!(result, "some code\n");
    }

    #[test]
    fn test_extract_codeblock_no_trailing_newline() {
        // Text ending without trailing newline after closing fence
        let text = "`````\ncontent here\n`````";
        let result = extract_last_codeblock(text).unwrap();
        assert_eq!(result, "content here\n");
    }
}
