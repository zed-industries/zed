use crate::{
    FormatPromptArgs, PredictionProvider,
    example::{ActualCursor, Example, ExamplePrompt},
    headless::EpAppState,
    progress::{ExampleProgress, Step},
    retrieve_context::run_context_retrieval,
};
use anyhow::{Context as _, Result, anyhow};
use gpui::AsyncApp;
use std::ops::Range;
use std::sync::Arc;
use zeta_prompt::{
    ZetaFormat, format_expected_output, format_zeta_prompt, multi_region, resolve_cursor_region,
};

fn resolved_excerpt_ranges_for_format(
    input: &zeta_prompt::ZetaPromptInput,
    format: ZetaFormat,
) -> (Range<usize>, Range<usize>) {
    let (_, editable_range_in_context, context_range, _) = resolve_cursor_region(input, format);
    let editable_range = (context_range.start + editable_range_in_context.start)
        ..(context_range.start + editable_range_in_context.end);
    (editable_range, context_range)
}

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
        PredictionProvider::Teacher(_, zeta_format)
        | PredictionProvider::TeacherNonBatching(_, zeta_format) => {
            step_progress.set_substatus("formatting teacher prompt");

            let (editable_range, context_range) =
                resolved_excerpt_ranges_for_format(prompt_inputs, zeta_format);

            let include_diagnostics = matches!(zeta_format, ZetaFormat::V0420Diagnostics);

            let prompt = TeacherPrompt::format_prompt(
                example,
                editable_range,
                context_range,
                include_diagnostics,
            );
            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output: None,
                rejected_output: None,
                prefill: None,
                provider: args.provider,
            });
        }
        PredictionProvider::TeacherMultiRegion(_)
        | PredictionProvider::TeacherMultiRegionNonBatching(_) => {
            step_progress.set_substatus("formatting teacher multi-region prompt");

            let zeta_format = ZetaFormat::default();
            let (editable_range, context_range) =
                resolved_excerpt_ranges_for_format(prompt_inputs, zeta_format);

            let include_diagnostics = matches!(zeta_format, ZetaFormat::V0420Diagnostics);

            let prompt = TeacherMultiRegionPrompt::format_prompt(
                example,
                editable_range,
                context_range,
                include_diagnostics,
            );
            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output: None,
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
                    format_expected_output(
                        prompt_inputs,
                        zeta_format,
                        &expected_patch,
                        expected_cursor_offset,
                    )
                    .ok()
                });

            let rejected_output = example.spec.rejected_patch.as_ref().and_then(|patch| {
                format_expected_output(prompt_inputs, zeta_format, patch, None).ok()
            });

            example.prompt = prompt.map(|prompt| ExamplePrompt {
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
        include_diagnostics: bool,
    ) -> String {
        let edit_history = Self::format_edit_history(&example.spec.edit_history);
        let context = Self::format_context(example);
        let cursor_excerpt = Self::format_cursor_excerpt(example, editable_range, context_range);
        let diagnostics = include_diagnostics
            .then(|| Self::format_diagnostics(example))
            .map(|diagnostics| format!("# 4. Diagnostics\n\n{diagnostics}"));

        let prompt_template = crate::prompt_assets::get_prompt("teacher.md");
        let prompt = prompt_template
            .replace("{{context}}", &context)
            .replace("{{edit_history}}", &edit_history)
            .replace("{{diagnostics}}", diagnostics.as_deref().unwrap_or(""))
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

        if response
            .trim_end_matches(&[' ', '\n', '`'])
            .ends_with(Self::NO_EDITS)
        {
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

    fn format_diagnostics(example: &Example) -> String {
        example
            .prompt_inputs
            .as_ref()
            .map(|prompt_inputs| {
                prompt_inputs
                    .active_buffer_diagnostics
                    .iter()
                    .map(|diagnostic| {
                        format!(
                            "*{}*:\n```\n{}\n```\n",
                            &diagnostic.message, &diagnostic.snippet
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .filter(|m| !m.is_empty())
            .unwrap_or("No Diagnostics".to_string())
    }
}

pub struct TeacherMultiRegionPrompt;

impl TeacherMultiRegionPrompt {
    pub(crate) const USER_CURSOR_MARKER: &str = "<|user_cursor|>";
    pub(crate) const NO_EDITS: &str = "NO_EDITS";

    /// Truncate edit history to this number of last lines
    const MAX_HISTORY_LINES: usize = 128;

    pub fn format_prompt(
        example: &Example,
        editable_range: Range<usize>,
        context_range: Range<usize>,
        include_diagnostics: bool,
    ) -> String {
        let edit_history = Self::format_edit_history(&example.spec.edit_history);
        let context = Self::format_context(example);
        let cursor_excerpt = Self::format_cursor_excerpt(example, editable_range, context_range);
        let diagnostics = include_diagnostics
            .then(|| TeacherPrompt::format_diagnostics(example))
            .map(|diagnostics| format!("# 4. Diagnostics\n\n{diagnostics}"));

        let prompt_template = crate::prompt_assets::get_prompt("teacher_multi_region.md");
        let prompt = prompt_template
            .replace("{{context}}", &context)
            .replace("{{edit_history}}", &edit_history)
            .replace("{{diagnostics}}", diagnostics.as_deref().unwrap_or(""))
            .replace("{{cursor_excerpt}}", &cursor_excerpt);

        prompt
    }

    pub fn parse(example: &Example, response: &str) -> Result<(String, Option<ActualCursor>)> {
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
        let (editable_range, _) = resolved_excerpt_ranges_for_format(prompt_inputs, zeta_format);
        let excerpt = prompt_inputs.cursor_excerpt.as_ref();
        let old_editable_region = &excerpt[editable_range.clone()];
        let marker_offsets = multi_region::compute_marker_offsets(old_editable_region);

        let codeblock =
            extract_last_codeblock(&response).context("no codeblock found in model response")?;
        let (start_num, end_num, raw_new_span) = multi_region::extract_marker_span(&codeblock)?;

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

        let cursor_in_span = raw_new_span.find(Self::USER_CURSOR_MARKER);
        let new_span = raw_new_span.replace(Self::USER_CURSOR_MARKER, "");

        let old_span = &old_editable_region[start_byte..end_byte];
        let mut new_span = new_span;
        if old_span.ends_with('\n') && !new_span.ends_with('\n') && !new_span.is_empty() {
            new_span.push('\n');
        }
        if !old_span.ends_with('\n') && new_span.ends_with('\n') {
            new_span.pop();
        }

        let mut new_editable_region = String::new();
        new_editable_region.push_str(&old_editable_region[..start_byte]);
        new_editable_region.push_str(&new_span);
        new_editable_region.push_str(&old_editable_region[end_byte..]);

        let cursor_offset = cursor_in_span.map(|pos| start_byte + pos);

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
        let cursor_in_editable = cursor_offset - editable_range.start;

        let path_str = example.spec.cursor_path.to_string_lossy();
        result.push_str(&format!("`````{path_str}\n"));

        result.push_str(&excerpt[context_range.start..editable_range.start]);

        multi_region::write_editable_with_markers(
            &mut result,
            editable_text,
            cursor_in_editable,
            Self::USER_CURSOR_MARKER,
        );

        result.push_str(&excerpt[editable_range.end..context_range.end]);
        result.push_str("\n`````");

        result
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
        let parsed = multi_region::extract_editable_region_from_markers(text).unwrap();
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
        let parsed = multi_region::extract_editable_region_from_markers(text).unwrap();
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

    #[test]
    fn test_parse_no_edits_response_with_trailing_backticks() {
        let response = "NO_EDITS```";

        let parsed = TeacherPrompt::parse(
            &Example {
                spec: edit_prediction::example_spec::ExampleSpec {
                    name: "test".to_string(),
                    repository_url: "https://github.com/zed-industries/zed.git".to_string(),
                    revision: "HEAD".to_string(),
                    tags: Vec::new(),
                    reasoning: None,
                    uncommitted_diff: String::new(),
                    cursor_path: std::sync::Arc::from(std::path::Path::new("src/main.rs")),
                    cursor_position: "0:0".to_string(),
                    edit_history: String::new(),
                    expected_patches: Vec::new(),
                    rejected_patch: None,
                    telemetry: None,
                    human_feedback: Vec::new(),
                    rating: None,
                },
                prompt_inputs: None,
                prompt: None,
                predictions: Vec::new(),
                score: Vec::new(),
                qa: Vec::new(),
                zed_version: None,
                state: None,
            },
            response,
        )
        .unwrap();

        assert!(parsed.0.is_empty());
        assert!(parsed.1.is_none());
    }

    #[test]
    fn test_v0327_teacher_prompt_uses_resolved_ranges() {
        let excerpt = (0..80)
            .map(|index| format!("line{index:02}\n"))
            .collect::<String>();
        let cursor_offset = excerpt.find("line40").expect("cursor line exists");
        let prompt_inputs = zeta_prompt::ZetaPromptInput {
            cursor_path: std::path::Path::new("src/main.rs").into(),
            cursor_excerpt: excerpt.clone().into(),
            cursor_offset_in_excerpt: cursor_offset,
            excerpt_start_row: None,
            events: Vec::new(),
            related_files: Some(Vec::new()),
            active_buffer_diagnostics: Vec::new(),
            excerpt_ranges: zeta_prompt::ExcerptRanges {
                editable_150: 0..32,
                editable_180: 0..32,
                editable_350: 0..32,
                editable_512: None,
                editable_150_context_350: 0..48,
                editable_180_context_350: 0..48,
                editable_350_context_150: 20..50,
                editable_350_context_512: None,
                editable_350_context_1024: None,
                context_4096: None,
                context_8192: Some(30..excerpt.len()),
            },
            syntax_ranges: None,
            in_open_source_repo: false,
            can_collect_data: false,
            repo_url: None,
        };

        let (stored_editable_range, stored_context_range) = zeta_prompt::excerpt_range_for_format(
            ZetaFormat::V0327SingleFile,
            &prompt_inputs.excerpt_ranges,
        );
        assert!(stored_context_range.start > stored_editable_range.start);

        let (editable_range, context_range) =
            resolved_excerpt_ranges_for_format(&prompt_inputs, ZetaFormat::V0327SingleFile);
        assert_eq!(context_range, 0..excerpt.len());
        assert!(editable_range.start < cursor_offset);
        assert!(editable_range.end > cursor_offset);

        let prompt = TeacherPrompt::format_prompt(
            &Example {
                spec: edit_prediction::example_spec::ExampleSpec {
                    name: "test".to_string(),
                    repository_url: "https://github.com/zed-industries/zed.git".to_string(),
                    revision: "HEAD".to_string(),
                    tags: Vec::new(),
                    reasoning: None,
                    uncommitted_diff: String::new(),
                    cursor_path: std::sync::Arc::from(std::path::Path::new("src/main.rs")),
                    cursor_position: "0:0".to_string(),
                    edit_history: String::new(),
                    expected_patches: Vec::new(),
                    rejected_patch: None,
                    telemetry: None,
                    human_feedback: Vec::new(),
                    rating: None,
                },
                prompt_inputs: Some(prompt_inputs),
                prompt: None,
                predictions: Vec::new(),
                score: Vec::new(),
                qa: Vec::new(),
                zed_version: None,
                state: None,
            },
            editable_range,
            context_range,
            false,
        );

        assert!(prompt.contains(TeacherPrompt::EDITABLE_REGION_START));
        assert!(prompt.contains(TeacherPrompt::USER_CURSOR_MARKER));
        assert!(prompt.contains("line40"));
    }
}
