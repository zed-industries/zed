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
use zeta_prompt::ZetaVersion;
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
                edit_prediction::zeta2::max_editable_tokens(ZetaVersion::default()),
                edit_prediction::zeta2::MAX_CONTEXT_TOKENS,
            );
            let editable_range = editable_range.to_offset(&snapshot);
            let context_range = context_range.to_offset(&snapshot);

            let prompt = TeacherPrompt::format_prompt(example, editable_range, context_range);
            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output: String::new(),
                rejected_output: None,
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
            };
            let prompt = format_zeta_prompt(&input, version);
            let (expected_patch, expected_cursor_offset) = example
                .spec
                .expected_patches_with_cursor_positions()
                .into_iter()
                .next()
                .context("expected patches is empty")?;
            let expected_output =
                zeta2_output_for_patch(&input, &expected_patch, expected_cursor_offset, version)?;
            let rejected_output = example
                .spec
                .rejected_patch
                .as_ref()
                .and_then(|patch| zeta2_output_for_patch(&input, patch, None, version).ok());

            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output,
                rejected_output,
                provider: args.provider,
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
    version: ZetaVersion,
) -> Result<String> {
    let mut old_editable_region =
        input.cursor_excerpt[input.editable_range_in_excerpt.clone()].to_string();

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
        let offset = (hunk_start + cursor_offset).min(result.len());
        result.insert_str(offset, zeta_prompt::CURSOR_MARKER);
    }

    match version {
        ZetaVersion::V0120GitMergeMarkers | ZetaVersion::V0131GitMergeMarkersPrefix => {
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

    pub fn parse(example: &Example, response: &str) -> Result<(String, Option<usize>)> {
        // Extract updated (new) editable region from the model response.
        // The model may include editable region markers in its output, so we need to strip them.
        let new_editable_region = extract_last_codeblock(response);

        // Check if the model indicated no edits are needed
        if new_editable_region.trim() == Self::NO_EDITS {
            return Ok((String::new(), None));
        }

        let new_editable_region = Self::extract_editable_region(&new_editable_region)?;
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

        Ok((diff, cursor_offset))
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

        let path_str = example.spec.cursor_path.to_string_lossy();
        result.push_str(&format!("`````{path_str}\n"));
        result.push_str(&prompt_inputs.content[context_range.start..editable_range.start]);
        result.push_str(Self::EDITABLE_REGION_START);
        result.push_str(&prompt_inputs.content[editable_range.start..prompt_inputs.cursor_offset]);
        result.push_str(Self::USER_CURSOR_MARKER);
        result.push_str(&prompt_inputs.content[prompt_inputs.cursor_offset..editable_range.end]);
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

fn extract_last_codeblock(text: &str) -> String {
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
        let last_block = extract_last_codeblock(text);
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
        let last_block = extract_last_codeblock(text);
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
        let last_block = extract_last_codeblock(text);
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
        let last_block = extract_last_codeblock(text);
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
        let codeblock = extract_last_codeblock(response);
        assert_eq!(codeblock.trim(), TeacherPrompt::NO_EDITS);
    }
}
