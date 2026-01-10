use crate::{
    PromptFormat,
    example::{Example, ExamplePrompt},
    headless::EpAppState,
    load_project::run_load_project,
    progress::{Progress, Step},
    retrieve_context::run_context_retrieval,
};
use anyhow::{Context as _, Result};
use edit_prediction::{
    EditPredictionStore,
    zeta2::{zeta2_output_for_patch, zeta2_prompt_input},
};
use gpui::{AsyncApp, Entity};
use std::fmt::Write as _;
use std::sync::Arc;
use zeta_prompt::format_zeta_prompt;

pub async fn run_format_prompt(
    example: &mut Example,
    prompt_format: PromptFormat,
    app_state: Arc<EpAppState>,
    mut cx: AsyncApp,
) -> Result<()> {
    run_context_retrieval(example, app_state.clone(), cx.clone()).await?;

    let step_progress = Progress::global().start(Step::FormatPrompt, &example.spec.name);

    match prompt_format {
        PromptFormat::Teacher => {
            step_progress.set_substatus("formatting teacher prompt");
            let prompt = TeacherPrompt::format_prompt(example);
            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output: example
                    .spec
                    .expected_patches
                    .first()
                    .cloned()
                    .unwrap_or_default(),
                format: prompt_format,
            });
        }
        PromptFormat::Zeta2 => {
            step_progress.set_substatus("loading project");
            run_load_project(example, app_state, cx.clone()).await?;

            step_progress.set_substatus("formatting zeta2 prompt");

            let ep_store: Entity<EditPredictionStore> = cx.update(|cx| {
                EditPredictionStore::try_global(cx).context("EditPredictionStore not initialized")
            })?;

            let state = example.state.as_ref().context("state must be set")?;
            let snapshot = state.buffer.read_with(&cx, |buffer, _| buffer.snapshot());
            let project = state.project.clone();
            let (_, input) =
                ep_store.update(&mut cx, |ep_store: &mut EditPredictionStore, cx| {
                    let events = ep_store
                        .edit_history_for_project(&project, cx)
                        .into_iter()
                        .map(|e| e.event)
                        .collect();
                    anyhow::Ok(zeta2_prompt_input(
                        &snapshot,
                        example
                            .context
                            .as_ref()
                            .context("context must be set")?
                            .files
                            .clone(),
                        events,
                        example.spec.cursor_path.clone(),
                        example
                            .buffer
                            .as_ref()
                            .context("buffer must be set")?
                            .cursor_offset,
                    ))
                })?;
            let prompt = format_zeta_prompt(&input);
            let expected_output = zeta2_output_for_patch(
                &input,
                &example
                    .spec
                    .expected_patches
                    .first()
                    .context("expected patches is empty")?
                    .clone(),
            )?;
            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output,
                format: prompt_format,
            });
        }
    };
    Ok(())
}

pub struct TeacherPrompt;

impl TeacherPrompt {
    const PROMPT: &str = include_str!("teacher.prompt.md");
    pub(crate) const EDITABLE_REGION_START: &str = "<|editable_region_start|>\n";
    pub(crate) const EDITABLE_REGION_END: &str = "\n<|editable_region_end|>";
    pub(crate) const USER_CURSOR_MARKER: &str = "<|user_cursor|>";

    /// Truncate edit history to this number of last lines
    const MAX_HISTORY_LINES: usize = 128;

    pub fn format_prompt(example: &Example) -> String {
        let edit_history = Self::format_edit_history(&example.spec.edit_history);
        let context = Self::format_context(example);
        let cursor_excerpt = Self::format_cursor_excerpt(example);

        let prompt = Self::PROMPT
            .replace("{{context}}", &context)
            .replace("{{edit_history}}", &edit_history)
            .replace("{{cursor_excerpt}}", &cursor_excerpt);

        prompt
    }

    pub fn parse(example: &Example, response: &str) -> Result<String> {
        // Ideally, we should always be able to find cursor position in the retrieved context.
        // In reality, sometimes we don't find it for these reasons:
        // 1. `example.cursor_position` contains _more_ context than included in the retrieved context
        //    (can be fixed by getting cursor coordinates at the load_example stage)
        // 2. Context retriever just didn't include cursor line.
        //
        // In that case, fallback to using `cursor_position` as excerpt.
        let example_buffer = example
            .buffer
            .as_ref()
            .context("`buffer` should be filled in in the context collection step")?;

        // Extract updated (new) editable region from the model response.
        // The model may include editable region markers in its output, so we need to strip them.
        let new_editable_region = extract_last_codeblock(response);
        let mut new_editable_region = Self::extract_editable_region(&new_editable_region);

        let old_editable_region =
            example_buffer.content[example_buffer.editable_range.clone()].to_string();

        // Normalize leading newlines: if old starts with newline but new doesn't,
        // prepend newline to new to preserve whitespace structure.
        // This handles the case where the model drops the leading blank line.
        if old_editable_region.starts_with('\n') && !new_editable_region.starts_with('\n') {
            new_editable_region.insert(0, '\n');
        }

        let editable_region_start_line = example_buffer.content
            [..example_buffer.editable_range.start]
            .matches('\n')
            .count();

        let diff = language::unified_diff_with_offsets(
            &old_editable_region,
            &new_editable_region,
            editable_region_start_line as u32,
            editable_region_start_line as u32,
        );

        let diff = indoc::formatdoc! {"
            --- a/{path}
            +++ b/{path}
            {diff}",
            path = example.spec.cursor_path.to_string_lossy(),
            diff = diff,
        };

        Ok(diff)
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

    fn format_context(example: &Example) -> String {
        let context = example
            .context
            .as_ref()
            .expect("Missing context retriever step");

        if context.files.is_empty() {
            return "(No context)".to_string();
        }

        let mut prompt = String::new();
        for file in context.files.as_ref() {
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
            prompt.push_str("\n`````");
        }

        prompt
    }

    fn format_cursor_excerpt(example: &Example) -> String {
        let mut result = String::new();

        let example_buffer = example.buffer.as_ref().unwrap();

        let path_str = example.spec.cursor_path.to_string_lossy();
        result.push_str(&format!("`````{path_str}\n"));
        result.push_str(
            &example_buffer.content
                [example_buffer.context_range.start..example_buffer.editable_range.start],
        );
        result.push_str(Self::EDITABLE_REGION_START);
        result.push_str(
            &example_buffer.content
                [example_buffer.editable_range.start..example_buffer.cursor_offset],
        );
        result.push_str(Self::USER_CURSOR_MARKER);
        result.push_str(
            &example_buffer.content
                [example_buffer.cursor_offset..example_buffer.editable_range.end],
        );
        result.push_str(Self::EDITABLE_REGION_END);
        result.push_str(
            &example_buffer.content
                [example_buffer.editable_range.end..example_buffer.context_range.end],
        );
        result.push_str("\n`````");

        result
    }

    fn extract_editable_region(text: &str) -> String {
        let start = text
            .find(Self::EDITABLE_REGION_START)
            .map_or(0, |pos| pos + Self::EDITABLE_REGION_START.len());
        let end = text.find(Self::EDITABLE_REGION_END).unwrap_or(text.len());

        let region = &text[start..end];
        let region = region.strip_suffix('\n').unwrap_or(region);

        region.replace(Self::USER_CURSOR_MARKER, "")
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
        let parsed = TeacherPrompt::extract_editable_region(text);
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
        let parsed = TeacherPrompt::extract_editable_region(text);
        assert_eq!(
            parsed,
            indoc::indoc! {"
            one
            two three"}
        );
    }

    #[test]
    fn test_extract_editable_region_strips_cursor_marker() {
        let text = indoc::indoc! {"
            <|editable_region_start|>
            one
            <|user_cursor|>two three

            <|editable_region_end|>
            "};
        let parsed = TeacherPrompt::extract_editable_region(text);
        assert_eq!(
            parsed,
            indoc::indoc! {"
            one
            two three"}
        );
    }
}
