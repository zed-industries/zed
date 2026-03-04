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
    ZetaFormat, excerpt_range_for_format, format_zeta_prompt, resolve_cursor_region,
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
            let excerpt_ranges = prompt_inputs
                .excerpt_ranges
                .as_ref()
                .context("prompt_inputs must have excerpt_ranges")?;
            let (editable_range, context_range) =
                excerpt_range_for_format(zeta_format, excerpt_ranges);

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
            let (expected_patch, expected_cursor_offset) = example
                .spec
                .expected_patches_with_cursor_positions()
                .into_iter()
                .next()
                .context("expected patches is empty")?;
            let expected_output = zeta2_output_for_patch(
                prompt_inputs,
                &expected_patch,
                expected_cursor_offset,
                zeta_format,
            )?;
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
    let (context, editable_range, _) = resolve_cursor_region(input, version);
    let mut old_editable_region = context[editable_range].to_string();

    if !old_editable_region.ends_with_newline() {
        old_editable_region.push('\n');
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

        // Extract updated (new) editable region from the model response.
        let new_editable_region = Self::extract_editable_region(&response)?;
        let cursor_offset = new_editable_region.find(Self::USER_CURSOR_MARKER);
        let mut new_editable_region = new_editable_region.replace(Self::USER_CURSOR_MARKER, "");
        let old_editable_region = Self::extract_editable_region(
            &example
                .prompt
                .as_ref()
                .context("example prompt missing")?
                .input,
        )?
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

        let excerpt = prompt_inputs.cursor_excerpt.as_ref();
        let (editable_region_offset, _) = excerpt
            .match_indices(&old_editable_region)
            .min_by_key(|(index, _)| index.abs_diff(prompt_inputs.cursor_offset_in_excerpt))
            .context("editable region not found in prompt content")?;
        let editable_region_start_line = excerpt[..editable_region_offset].matches('\n').count();

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
        let related_files = example.prompt_inputs.as_ref().map(|pi| &pi.related_files);
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

        let path_str = example.spec.cursor_path.to_string_lossy();
        result.push_str(&format!("`````{path_str}\n"));
        result.push_str(&excerpt[context_range.start..editable_range.start]);
        result.push_str(Self::EDITABLE_REGION_START);
        result.push_str(&excerpt[editable_range.start..cursor_offset]);
        result.push_str(Self::USER_CURSOR_MARKER);
        result.push_str(&excerpt[cursor_offset..editable_range.end]);
        result.push_str(Self::EDITABLE_REGION_END);
        result.push_str(&excerpt[editable_range.end..context_range.end]);
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

    // Simple fallback: just show content around cursor with markers
    let path_str = example.spec.cursor_path.to_string_lossy();
    let mut result = format!("`````{path_str}\n");
    result.push_str(TeacherPrompt::EDITABLE_REGION_START);
    result.push_str(&excerpt[..cursor_offset]);
    result.push_str(TeacherPrompt::USER_CURSOR_MARKER);
    result.push_str(&excerpt[cursor_offset..]);
    result.push_str(TeacherPrompt::EDITABLE_REGION_END);
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
    fn test_extract_editable_region() {
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
