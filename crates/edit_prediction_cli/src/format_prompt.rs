use crate::{
    FormatPromptArgs, PredictionProvider,
    example::{Example, ExamplePrompt},
    headless::EpAppState,
    progress::{ExampleProgress, Step},
    retrieve_context::run_context_retrieval,
};
use anyhow::{Context as _, Result, anyhow};
use edit_prediction::{cursor_excerpt::editable_and_context_ranges_for_cursor_position, udiff};
use gpui::{AppContext, AsyncApp};
use language::{Buffer, OffsetRangeExt, Point};
use similar::DiffableStr;
use std::sync::Arc;
use std::{fmt::Write as _, ops::Range};
use zeta_prompt::ZetaFormat;
use zeta_prompt::format_zeta_prompt;

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

    let language = app_state
        .languages
        .load_language_for_file_path(&example.spec.cursor_path)
        .await
        .ok();
    let snapshot_fut = cx.update(|cx| {
        Buffer::build_snapshot(
            prompt_inputs.content.as_str().into(),
            language,
            Some(app_state.languages.clone()),
            cx,
        )
    });
    let cursor_point = Point::new(prompt_inputs.cursor_row, prompt_inputs.cursor_column);
    let snapshot = cx.background_spawn(snapshot_fut).await;

    match args.provider {
        PredictionProvider::Teacher(_) | PredictionProvider::TeacherNonBatching(_) => {
            step_progress.set_substatus("formatting teacher prompt");

            let (editable_range, context_range) = editable_and_context_ranges_for_cursor_position(
                cursor_point,
                &snapshot,
                edit_prediction::zeta2::max_editable_tokens(ZetaFormat::default()),
                edit_prediction::zeta2::MAX_CONTEXT_TOKENS,
            );
            let editable_range = editable_range.to_offset(&snapshot);
            let context_range = context_range.to_offset(&snapshot);

            let prompt = TeacherPrompt::format_prompt(example, editable_range, context_range);
            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output: String::new(),
                rejected_output: None,
                prefill: None,
                provider: args.provider,
            });
        }
        PredictionProvider::Zeta2(version) => {
            step_progress.set_substatus("formatting zeta2 prompt");

            let (editable_range, context_range) = editable_and_context_ranges_for_cursor_position(
                cursor_point,
                &snapshot,
                edit_prediction::zeta2::max_editable_tokens(version),
                edit_prediction::zeta2::MAX_CONTEXT_TOKENS,
            );
            let editable_range = editable_range.to_offset(&snapshot);
            let context_range = context_range.to_offset(&snapshot);

            let context_start = context_range.start;
            let cursor_offset_in_excerpt = prompt_inputs.cursor_offset - context_start;
            let editable_range_in_excerpt =
                (editable_range.start - context_start)..(editable_range.end - context_start);
            let input = zeta_prompt::ZetaPromptInput {
                cursor_path: example.spec.cursor_path.clone(),
                cursor_excerpt: prompt_inputs.content[context_range].to_string().into(),
                editable_range_in_excerpt,
                cursor_offset_in_excerpt,
                excerpt_start_row: prompt_inputs.excerpt_start_row,
                events: prompt_inputs.edit_history.clone(),
                related_files: prompt_inputs.related_files.clone().unwrap_or_default(),
                excerpt_ranges: None,
                preferred_model: None,
                in_open_source_repo: example
                    .spec
                    .captured_prompt_input
                    .as_ref()
                    .map_or(false, |input| input.in_open_source_repo),
            };
            let prompt = format_zeta_prompt(&input, version);
            let prefill = zeta_prompt::get_prefill(&input, version);
            let (expected_patch, expected_selections) = example
                .spec
                .expected_patches_with_selections()
                .into_iter()
                .next()
                .context("expected patches is empty")?;
            let expected_output =
                zeta2_output_for_patch(&input, &expected_patch, &expected_selections, version)?;
            let rejected_output = example
                .spec
                .rejected_patch
                .as_ref()
                .and_then(|patch| zeta2_output_for_patch(&input, patch, &[], version).ok());

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
    selections: &[Range<usize>],
    version: ZetaFormat,
) -> Result<String> {
    let mut old_editable_region =
        input.cursor_excerpt[input.editable_range_in_excerpt.clone()].to_string();

    if !old_editable_region.ends_with_newline() {
        old_editable_region.push('\n');
    }

    let (mut result, hunk_start) =
        udiff::apply_diff_to_string_with_hunk_offset(patch, &old_editable_region).with_context(
            || {
                format!(
                    "Patch:\n```\n{}```\n\nEditable region:\n```\n{}```",
                    patch, old_editable_region
                )
            },
        )?;

    // Each selection is relative to the start of the hunk's new text (context + additions).
    // We need to add where the hunk context matched in the editable region to compute
    // the actual positions in the result. Insert in reverse order so earlier
    // insertions don't shift later offsets. For each selection, insert the cursor
    // marker at the end first, then the selection start marker (if non-empty).
    for selection in selections.iter().rev() {
        let end = (hunk_start + selection.end).min(result.len());
        result.insert_str(end, TeacherPrompt::USER_CURSOR_MARKER);
        if selection.start != selection.end {
            let start = (hunk_start + selection.start).min(result.len());
            result.insert_str(start, TeacherPrompt::SELECTION_START_MARKER);
        }
    }

    match version {
        ZetaFormat::V0120GitMergeMarkers
        | ZetaFormat::V0131GitMergeMarkersPrefix
        | ZetaFormat::V0211SeedCoder => {
            if !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(zeta_prompt::v0120_git_merge_markers::END_MARKER);
        }
        _ => (),
    }

    Ok(result)
}

pub struct TeacherPrompt;

impl TeacherPrompt {
    pub(crate) const EDITABLE_REGION_START: &str = "<|editable_region_start|>\n";
    pub(crate) const EDITABLE_REGION_END: &str = "\n<|editable_region_end|>";
    pub(crate) const USER_CURSOR_MARKER: &str = "<|user_cursor|>";
    pub(crate) const SELECTION_START_MARKER: &str = "<|selection_start|>";
    pub(crate) const NO_EDITS: &str = "NO_EDITS";

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

    pub fn parse(example: &Example, response: &str) -> Result<(String, Vec<Range<usize>>)> {
        // Extract updated (new) editable region from the model response.
        // The model may include editable region markers in its output, so we need to strip them.
        let new_editable_region = extract_last_codeblock(response);

        // Check if the model indicated no edits are needed
        if new_editable_region.trim() == Self::NO_EDITS {
            return Ok((String::new(), Vec::new()));
        }

        let new_editable_region = Self::extract_editable_region(&new_editable_region)?;

        let selection_ranges = Self::extract_selections(&new_editable_region);

        let mut new_editable_region = new_editable_region
            .replace(Self::SELECTION_START_MARKER, "")
            .replace(Self::USER_CURSOR_MARKER, "");

        let old_editable_region = Self::extract_editable_region(
            &example
                .prompt
                .as_ref()
                .context("example prompt missing")?
                .input,
        )?
        .replace(Self::SELECTION_START_MARKER, "")
        .replace(Self::USER_CURSOR_MARKER, "");

        let prompt_inputs = example
            .prompt_inputs
            .as_ref()
            .context("example is missing prompt inputs")?;

        // Normalize leading newlines: if old starts with newline but new doesn't,
        // prepend newline to new to preserve whitespace structure.
        // This handles the case where the model drops the leading blank line.
        if old_editable_region.starts_with('\n') && !new_editable_region.starts_with('\n') {
            new_editable_region.insert(0, '\n');
        }

        let (editable_region_offset, _) = prompt_inputs
            .content
            .match_indices(&old_editable_region)
            .min_by_key(|(index, _)| index.abs_diff(prompt_inputs.cursor_offset))
            .context("editable region not found in prompt content")?;
        let editable_region_start_line = prompt_inputs.content[..editable_region_offset]
            .matches('\n')
            .count();

        // Use full context so cursor offset (relative to editable region start) aligns with diff content
        let editable_region_lines = old_editable_region.lines().count() as u32;
        let diff = language::unified_diff_with_context(
            &old_editable_region,
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

        Ok((diff, selection_ranges))
    }

    /// Extract all selection/cursor ranges from an editable region that still
    /// contains `SELECTION_START_MARKER` and `USER_CURSOR_MARKER` markers.
    ///
    /// Returns byte-offset ranges in the *marker-stripped* text. Each range
    /// represents one selection: `start..end` where `start == end` means an
    /// empty cursor and `start < end` means selected text.
    pub(crate) fn extract_selections(text_with_markers: &str) -> Vec<Range<usize>> {
        #[derive(Clone, Copy, PartialEq)]
        enum Kind {
            SelectionStart,
            UserCursor,
        }

        let sel_marker = Self::SELECTION_START_MARKER;
        let cur_marker = Self::USER_CURSOR_MARKER;

        // Collect every marker occurrence in document order.
        let mut markers: Vec<(usize, Kind)> = Vec::new();
        let mut pos = 0;
        while pos < text_with_markers.len() {
            if text_with_markers[pos..].starts_with(sel_marker) {
                markers.push((pos, Kind::SelectionStart));
                pos += sel_marker.len();
            } else if text_with_markers[pos..].starts_with(cur_marker) {
                markers.push((pos, Kind::UserCursor));
                pos += cur_marker.len();
            } else {
                pos += text_with_markers[pos..]
                    .chars()
                    .next()
                    .map_or(1, |c| c.len_utf8());
            }
        }

        // Compute the clean (marker-stripped) offset for each marker.
        let mut clean_offsets = Vec::with_capacity(markers.len());
        let mut removed_bytes = 0usize;
        for &(raw_pos, kind) in &markers {
            clean_offsets.push(raw_pos - removed_bytes);
            removed_bytes += match kind {
                Kind::SelectionStart => sel_marker.len(),
                Kind::UserCursor => cur_marker.len(),
            };
        }

        // Pair markers into selection ranges.
        let mut selections = Vec::new();
        let mut i = 0;
        while i < markers.len() {
            match markers[i].1 {
                Kind::SelectionStart => {
                    if i + 1 < markers.len() && markers[i + 1].1 == Kind::UserCursor {
                        selections.push(clean_offsets[i]..clean_offsets[i + 1]);
                        i += 2;
                    } else {
                        // Orphaned selection_start – skip it.
                        i += 1;
                    }
                }
                Kind::UserCursor => {
                    if i + 1 < markers.len() && markers[i + 1].1 == Kind::SelectionStart {
                        // Backwards pair: UserCursor then SelectionStart.
                        // clean_offsets are in document order so this still
                        // produces a valid forward range.
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

    fn format_edit_history(edit_history: &str) -> String {
        // Strip comments ("garbage lines") from edit history
        let lines = edit_history
            .lines()
            .filter(|&s| Self::is_udiff_content_line(s))
            .collect::<Vec<_>>();

        let history_lines = if lines.len() > Self::MAX_HISTORY_LINES {
            &lines[lines.len() - Self::MAX_HISTORY_LINES..]
        } else {
            &lines
        };

        if history_lines.is_empty() {
            return "(No edit history)".to_string();
        }

        history_lines.join("\n")
    }

    pub fn format_context(example: &Example) -> String {
        let related_files = example
            .prompt_inputs
            .as_ref()
            .and_then(|pi| pi.related_files.as_ref());

        let Some(related_files) = related_files else {
            return "(No context)".to_string();
        };

        if related_files.is_empty() {
            return "(No context)".to_string();
        }

        let mut prompt = String::new();
        for file in related_files {
            let path_str = file.path.to_string_lossy();
            writeln!(&mut prompt, "`````{path_str}").ok();

            let mut prev_row = 0;
            for excerpt in &file.excerpts {
                if excerpt.row_range.start > prev_row {
                    prompt.push_str("…\n");
                }
                prompt.push_str(&excerpt.text);
                prompt.push('\n');
                prev_row = excerpt.row_range.end;
            }
            if prev_row < file.max_row {
                prompt.push_str("…\n");
            }
            prompt.push_str("\n`````\n");
        }

        prompt
    }

    fn format_cursor_excerpt(
        example: &Example,
        editable_range: Range<usize>,
        context_range: Range<usize>,
    ) -> String {
        let mut result = String::new();

        let prompt_inputs = example.prompt_inputs.as_ref().unwrap();
        let selection_range = prompt_inputs.selection_range();
        let selection_start = selection_range.start;
        let cursor_offset = selection_range.end;
        let is_empty_selection = selection_start == cursor_offset;

        let path_str = example.spec.cursor_path.to_string_lossy();
        result.push_str(&format!("`````{path_str}\n"));
        result.push_str(&prompt_inputs.content[context_range.start..editable_range.start]);
        result.push_str(Self::EDITABLE_REGION_START);

        if is_empty_selection {
            // Just cursor, no selection
            result.push_str(&prompt_inputs.content[editable_range.start..cursor_offset]);
            result.push_str(Self::USER_CURSOR_MARKER);
            result.push_str(&prompt_inputs.content[cursor_offset..editable_range.end]);
        } else {
            // Non-empty selection: place SELECTION_START_MARKER at start, USER_CURSOR_MARKER at end
            // Clamp positions to editable region
            let start_pos = selection_start
                .max(editable_range.start)
                .min(editable_range.end);
            let end_pos = cursor_offset
                .max(editable_range.start)
                .min(editable_range.end);

            result.push_str(&prompt_inputs.content[editable_range.start..start_pos]);
            result.push_str(Self::SELECTION_START_MARKER);
            result.push_str(&prompt_inputs.content[start_pos..end_pos]);
            result.push_str(Self::USER_CURSOR_MARKER);
            result.push_str(&prompt_inputs.content[end_pos..editable_range.end]);
        }

        result.push_str(Self::EDITABLE_REGION_END);
        result.push_str(&prompt_inputs.content[editable_range.end..context_range.end]);
        result.push_str("\n`````");

        result
    }

    pub fn extract_editable_region(text: &str) -> Result<String> {
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

    fn is_udiff_content_line(s: &str) -> bool {
        s.starts_with("-")
            || s.starts_with("+")
            || s.starts_with(" ")
            || s.starts_with("---")
            || s.starts_with("+++")
            || s.starts_with("@@")
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
    let content = &prompt_inputs.content;
    let cursor_offset = prompt_inputs.cursor_offset;

    // Simple fallback: just show content around cursor with markers
    let path_str = example.spec.cursor_path.to_string_lossy();
    let mut result = format!("`````{path_str}\n");
    result.push_str(TeacherPrompt::EDITABLE_REGION_START);
    result.push_str(&content[..cursor_offset]);
    result.push_str(TeacherPrompt::USER_CURSOR_MARKER);
    result.push_str(&content[cursor_offset..]);
    result.push_str(TeacherPrompt::EDITABLE_REGION_END);
    result.push_str("\n`````");

    Some(result)
}

pub(crate) fn extract_last_codeblock(text: &str) -> String {
    let mut last_block = None;
    let mut search_start = 0;

    while let Some(start) = text[search_start..].find("```") {
        let start = start + search_start;
        let bytes = text.as_bytes();
        let mut backtick_end = start;

        while backtick_end < bytes.len() && bytes[backtick_end] == b'`' {
            backtick_end += 1;
        }

        let backtick_count = backtick_end - start;
        let closing_pattern = format!("\n{}", "`".repeat(backtick_count));

        while backtick_end < bytes.len() && bytes[backtick_end] != b'\n' {
            backtick_end += 1;
        }

        if let Some(end_pos) = text[backtick_end..].find(&closing_pattern) {
            let code_block = &text[backtick_end + 1..backtick_end + end_pos + 1];
            last_block = Some(code_block.to_string());
            search_start = backtick_end + end_pos + closing_pattern.len();
        } else {
            break;
        }
    }

    last_block.unwrap_or_else(|| text.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::example::{Example, ExamplePrompt, ExamplePromptInputs};
    use edit_prediction::example_spec::ExampleSpec;
    use std::path::Path;
    use std::sync::Arc;
    use util::test::marked_text_ranges;

    /// Convert marked text to prompt format and extract cursor/selection info.
    /// Uses `ˇ` for cursor position and `«»` for selection ranges (with `ˇ` inside for direction).
    fn parse_marked_text(marked_text: &str) -> (String, Option<usize>, Option<usize>) {
        let (text, ranges) = marked_text_ranges(marked_text, true);
        let range = ranges.first();

        let cursor_offset = range.map(|r| r.end);
        let selection_start = range.and_then(|r| {
            if r.start != r.end {
                Some(r.start)
            } else {
                None
            }
        });

        (text, cursor_offset, selection_start)
    }

    /// Convert marked text to the prompt input format with actual markers.
    fn to_prompt_format(marked_text: &str) -> String {
        let (text, cursor_offset, selection_start) = parse_marked_text(marked_text);

        let mut result = String::new();
        for (i, c) in text.chars().enumerate() {
            if Some(i) == selection_start {
                result.push_str(TeacherPrompt::SELECTION_START_MARKER);
            }
            if Some(i) == cursor_offset {
                result.push_str(TeacherPrompt::USER_CURSOR_MARKER);
            }
            result.push(c);
        }
        // Handle cursor at end of text
        if Some(text.len()) == cursor_offset {
            if Some(text.len()) == selection_start {
                result.push_str(TeacherPrompt::SELECTION_START_MARKER);
            }
            result.push_str(TeacherPrompt::USER_CURSOR_MARKER);
        }

        result
    }

    fn make_example(file_content: &str, prompt_input_with_markers: &str) -> Example {
        let cursor_path: Arc<Path> = Path::new("test.rs").into();

        let prompt_input = to_prompt_format(prompt_input_with_markers);
        let (content, cursor_offset, selection_start_offset) =
            parse_marked_text(prompt_input_with_markers);
        let _ = content; // Used only for extracting cursor info

        let cursor_offset = cursor_offset.unwrap_or(0);

        Example {
            spec: ExampleSpec {
                name: String::new(),
                repository_url: String::new(),
                revision: String::new(),
                tags: Vec::new(),
                reasoning: None,
                uncommitted_diff: String::new(),
                cursor_path,
                cursor_position: String::new(),
                edit_history: String::new(),
                expected_patches: Vec::new(),
                rejected_patch: None,
                captured_prompt_input: None,
                telemetry: None,
                human_feedback: Vec::new(),
                rating: None,
            },
            prompt_inputs: Some(ExamplePromptInputs {
                content: file_content.to_string(),
                cursor_row: 0,
                cursor_column: 0,
                cursor_offset,
                selection_start_offset,
                excerpt_start_row: Some(0),
                edit_history: vec![],
                related_files: None,
            }),
            prompt: Some(ExamplePrompt {
                input: prompt_input,
                expected_output: String::new(),
                rejected_output: None,
                prefill: None,
                provider: crate::PredictionProvider::Teacher(crate::TeacherBackend::Sonnet45),
            }),
            predictions: Vec::new(),
            score: Vec::new(),
            qa: Vec::new(),
            state: None,
        }
    }

    struct ParseTestCase {
        name: &'static str,
        file_content: &'static str,
        prompt_editable_region: &'static str,
        response: &'static str,
        expected_new_region: &'static str,
    }

    #[test]
    fn test_parse_teacher_output() {
        let test_cases = [
            ParseTestCase {
                name: "cursor only - no change",
                file_content: "let x = 42;\n",
                prompt_editable_region: "let x = ˇ42;\n",
                response: indoc::indoc! {"
                    The code looks good.

                    `````
                    <|editable_region_start|>
                    let x = <|user_cursor|>42;
                    <|editable_region_end|>
                    `````
                "},
                expected_new_region: "let x = ˇ42;\n",
            },
            ParseTestCase {
                name: "cursor only - with edit",
                file_content: "let x = 42;\n",
                prompt_editable_region: "let x = ˇ42;\n",
                response: indoc::indoc! {"
                    Changing the value.

                    `````
                    <|editable_region_start|>
                    let x = <|user_cursor|>100;
                    <|editable_region_end|>
                    `````
                "},
                expected_new_region: "let x = ˇ100;\n",
            },
            ParseTestCase {
                name: "cursor moves after edit",
                file_content: "let x = 42;\n",
                prompt_editable_region: "let x = ˇ42;\n",
                response: indoc::indoc! {"
                    Updating value and moving cursor to end.

                    `````
                    <|editable_region_start|>
                    let x = 100;<|user_cursor|>
                    <|editable_region_end|>
                    `````
                "},
                expected_new_region: "let x = 100;ˇ\n",
            },
            ParseTestCase {
                name: "selection in response - text selected",
                file_content: "let x = 42;\n",
                prompt_editable_region: "let x = ˇ42;\n",
                response: indoc::indoc! {"
                    Selecting the number.

                    `````
                    <|editable_region_start|>
                    let x = <|selection_start|>42<|user_cursor|>;
                    <|editable_region_end|>
                    `````
                "},
                expected_new_region: "let x = «42ˇ»;\n",
            },
            ParseTestCase {
                name: "selection markers backwards - cursor before selection_start",
                file_content: "let x = 42;\n",
                prompt_editable_region: "let x = ˇ42;\n",
                response: indoc::indoc! {"
                    Selecting the number (markers reversed).

                    `````
                    <|editable_region_start|>
                    let x = <|user_cursor|>42<|selection_start|>;
                    <|editable_region_end|>
                    `````
                "},
                expected_new_region: "let x = «42ˇ»;\n",
            },
            ParseTestCase {
                name: "multiline edit with cursor",
                file_content: indoc::indoc! {"
                    fn main() {
                        let x = 42;
                    }
                "},
                prompt_editable_region: indoc::indoc! {"
                    fn main() {
                        let x = ˇ42;
                    }
                "},
                response: indoc::indoc! {"
                    Adding a new variable.

                    `````
                    <|editable_region_start|>
                    fn main() {
                        let x = 100;
                        let y = <|user_cursor|>200;
                    }
                    <|editable_region_end|>
                    `````
                "},
                expected_new_region: indoc::indoc! {"
                    fn main() {
                        let x = 100;
                        let y = ˇ200;
                    }
                "},
            },
            ParseTestCase {
                name: "no cursor in response",
                file_content: "let x = 42;\n",
                prompt_editable_region: "let x = ˇ42;\n",
                response: indoc::indoc! {"
                    Simple edit without cursor.

                    `````
                    <|editable_region_start|>
                    let x = 100;
                    <|editable_region_end|>
                    `````
                "},
                expected_new_region: "let x = 100;\n",
            },
            ParseTestCase {
                name: "NO_EDITS response",
                file_content: "let x = 42;\n",
                prompt_editable_region: "let x = ˇ42;\n",
                response: indoc::indoc! {"
                    The code is already complete.

                    `````
                    NO_EDITS
                    `````
                "},
                expected_new_region: "",
            },
            ParseTestCase {
                name: "nested codeblock in response",
                file_content: indoc::indoc! {r#"
                    ## Citation

                    ```bibtex
                    @misc{foo}
                    ```
                "#},
                prompt_editable_region: indoc::indoc! {r#"
                    ## Citation

                    ˇ```bibtex
                    @misc{foo}
                    ```
                "#},
                response: indoc::indoc! {r#"
                    Adding a citation.

                    `````
                    <|editable_region_start|>
                    ## Citation

                    ```bibtex
                    @misc{foo,
                        title={Bar}<|user_cursor|>
                    }
                    ```
                    <|editable_region_end|>
                    `````
                "#},
                expected_new_region: indoc::indoc! {r#"
                    ## Citation

                    ```bibtex
                    @misc{foo,
                        title={Bar}ˇ
                    }
                    ```
                "#},
            },
        ];

        for test_case in test_cases {
            let example = make_example(test_case.file_content, test_case.prompt_editable_region);

            let result = TeacherPrompt::parse(&example, test_case.response);
            assert!(
                result.is_ok(),
                "Test '{}' failed to parse: {:?}",
                test_case.name,
                result.err()
            );

            let (diff, actual_selections) = result.unwrap();

            // Handle NO_EDITS case specially.
            if test_case.expected_new_region.is_empty() {
                assert!(
                    diff.is_empty(),
                    "Test '{}': expected empty diff for NO_EDITS",
                    test_case.name
                );
                assert!(
                    actual_selections.is_empty(),
                    "Test '{}': expected no selections for NO_EDITS",
                    test_case.name
                );
                continue;
            }

            // Apply the diff to the file content to get the new text.
            let actual_text =
                edit_prediction::udiff::apply_diff_to_string(&diff, test_case.file_content)
                    .unwrap_or_else(|e| {
                        panic!(
                            "Test '{}': failed to apply diff: {:?}\nDiff:\n{}",
                            test_case.name, e, diff
                        )
                    });

            let actual_with_markers =
                util::test::generate_marked_text(&actual_text, &actual_selections, true);

            // Compare with expected.
            assert_eq!(
                actual_with_markers, test_case.expected_new_region,
                "Test '{}': mismatch after applying diff and inserting cursor markers",
                test_case.name
            );
        }
    }

    #[test]
    fn test_parse_teacher_output_multiple_selections() {
        let file_content = indoc::indoc! {"
            fn process(data: &DataSet) -> Vec<String> {
                let mut results = Vec::new();
                for
            }
        "};
        let prompt_editable_region = indoc::indoc! {"
            fn process(data: &DataSet) -> Vec<String> {
                let mut results = Vec::new();
                forˇ
            }
        "};
        let response = indoc::indoc! {"
            Completing the for loop.

            `````
            <|editable_region_start|>
            fn process(data: &DataSet) -> Vec<String> {
                let mut results = Vec::new();
                for <|selection_start|>item<|user_cursor|> in <|selection_start|>data.items()<|user_cursor|> {
                    <|user_cursor|>
                }
            }
            <|editable_region_end|>
            `````
        "};

        let example = make_example(file_content, prompt_editable_region);
        let (_, actual_selections) = TeacherPrompt::parse(&example, response).unwrap();

        assert_eq!(
            actual_selections.len(),
            3,
            "Expected 3 selections (two non-empty + one empty cursor in body)"
        );

        // First: selection over "item"
        let first = &actual_selections[0];
        assert_ne!(
            first.start, first.end,
            "first selection should be non-empty"
        );

        // Second: selection over "data.items()"
        let second = &actual_selections[1];
        assert_ne!(
            second.start, second.end,
            "second selection should be non-empty"
        );

        // Third: empty cursor in the loop body
        let third = &actual_selections[2];
        assert_eq!(
            third.start, third.end,
            "third selection should be empty (cursor only)"
        );

        // Verify the selections don't overlap and are in order
        assert!(first.end <= second.start);
        assert!(second.end <= third.start);
    }
}
