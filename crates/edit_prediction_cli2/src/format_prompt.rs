use crate::{
    PromptFormat,
    example::{Example, ExampleContextExcerpt, ExamplePrompt},
};
use std::fmt::Write;

pub async fn run_format_prompt(example: &mut Example, prompt_format: PromptFormat) {
    let prompt = match prompt_format {
        PromptFormat::Teacher => TeacherPrompt::format(example),
        PromptFormat::Zeta2 => Zeta2Prompt::format(example),
    };

    example.prompt = Some(ExamplePrompt {
        input: prompt,
        expected_output: example.expected_patch.clone(), // TODO
        format: prompt_format,
    });
}

pub trait PromptFormatter {
    fn format(example: &Example) -> String;
}

pub trait PromptParser {
    /// Return unified diff patch of prediction given raw LLM response
    fn parse(example: &Example, response: &str) -> String;
}

pub struct Zeta2Prompt;
pub struct TeacherPrompt;

impl PromptFormatter for Zeta2Prompt {
    fn format(example: &Example) -> String {
        let mut prompt = String::new();
        Self::write_context_section(&mut prompt, example);
        Self::write_edit_history_section(&mut prompt, example);
        Self::write_cursor_excerpt_section(&mut prompt, example);
        prompt
    }
}

impl Zeta2Prompt {
    pub(crate) fn write_context_section(prompt: &mut String, example: &Example) {
        prompt.push_str("<context>\n");

        if let Some(context) = &example.context {
            for file in &context.files {
                let path_str = file.rel_path.to_string_lossy();
                writeln!(prompt, "<file path=\"{}\">", path_str).unwrap();

                for excerpt in &file.excerpts {
                    writeln!(
                        prompt,
                        "<excerpt lines=\"{}-{}\">",
                        excerpt.row_range.start + 1,
                        excerpt.row_range.end + 1
                    )
                    .unwrap();
                    prompt.push_str(&excerpt.text);
                    if !excerpt.text.ends_with('\n') {
                        prompt.push('\n');
                    }
                    prompt.push_str("</excerpt>\n");
                }

                prompt.push_str("</file>\n");
            }
        }

        prompt.push_str("</context>\n\n");
    }

    pub(crate) fn write_edit_history_section(prompt: &mut String, example: &Example) {
        prompt.push_str("<edit_history>\n");

        if example.edit_history.is_empty() {
            prompt.push_str("(No edit history)\n");
        } else {
            prompt.push_str(&example.edit_history);
            if !example.edit_history.ends_with('\n') {
                prompt.push('\n');
            }
        }

        prompt.push_str("</edit_history>\n\n");
    }

    pub(crate) fn write_cursor_excerpt_section(prompt: &mut String, example: &Example) {
        prompt.push_str("<cursor_excerpt>\n");

        let path_str = example.cursor_path.to_string_lossy();
        writeln!(prompt, "<file path=\"{}\">", path_str).unwrap();

        prompt.push_str("<editable_region>\n");

        prompt.push_str(&example.cursor_position);
        if !example.cursor_position.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("</editable_region>\n");
        prompt.push_str("</file>\n");

        prompt.push_str("</cursor_excerpt>\n");
    }
}

impl PromptFormatter for TeacherPrompt {
    fn format(example: &Example) -> String {
        let edit_history = Self::format_edit_history(&example.edit_history);
        let context = Self::format_context(example);
        let editable_region = Self::format_editable_region(example);

        let prompt = Self::PROMPT
            .replace("{{context}}", &context)
            .replace("{{edit_history}}", &edit_history)
            .replace("{{editable_region}}", &editable_region);

        prompt
    }
}

impl TeacherPrompt {
    const PROMPT: &str = include_str!("teacher.prompt.md");
    pub(crate) const EDITABLE_REGION_START: &str = "<|editable_region_start|>\n";
    pub(crate) const EDITABLE_REGION_END: &str = "<|editable_region_end|>";
    // pub(crate) const USER_CURSOR: &str = "<|user_cursor|>";

    /// Number of lines to include before the cursor position
    // pub(crate) const LEFT_CONTEXT_SIZE: usize = 5;

    /// Number of lines to include after the cursor position
    // pub(crate) const RIGHT_CONTEXT_SIZE: usize = 5;

    /// Truncate edit history to this number of last lines
    const MAX_HISTORY_LINES: usize = 128;

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
        if example.context.is_none() {
            panic!("Missing context retriever step");
        }

        let mut prompt = String::new();
        Zeta2Prompt::write_context_section(&mut prompt, example);

        prompt
    }

    fn format_editable_region(example: &Example) -> String {
        let mut result = String::new();

        let path_str = example.cursor_path.to_string_lossy();
        result.push_str(&format!("`````path=\"{path_str}\"\n"));
        result.push_str(&format!("{}", Self::EDITABLE_REGION_START));

        // TODO: control number of lines around cursor
        result.push_str(&example.cursor_position);
        if !example.cursor_position.ends_with('\n') {
            result.push('\n');
        }

        result.push_str(&format!("{}\n", Self::EDITABLE_REGION_END));
        result.push_str("`````");

        result
    }

    fn extract_editable_region(text: &str) -> String {
        let start = text
            .find(Self::EDITABLE_REGION_START)
            .map_or(0, |pos| pos + Self::EDITABLE_REGION_START.len());
        let end = text.find(Self::EDITABLE_REGION_END).unwrap_or(text.len());

        let region = &text[start..end];

        region.replace("<|user_cursor|>", "")
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

impl PromptParser for TeacherPrompt {
    fn parse(example: &Example, response: &str) -> String {
        // Ideally, we should always be able to find cursor position in the retrieved context.
        // In reality, sometimes we don't find it for these reasons:
        // 1. `example.cursor_position` contains _more_ context than included in the retrieved context
        //    (can be fixed by getting cursor coordinates at the load_example stage)
        // 2. Context retriever just didn't include cursor line.
        //
        // In that case, fallback to using `cursor_position` as excerpt.
        let cursor_excerpt = find_context_excerpt_under_cursor(example)
            .map_or(&example.cursor_position, |e| &e.text)
            .replace("<|user_cursor|>", "");

        // Extract updated (new) editable region from the model response
        let new_editable_region = extract_last_codeblock(response);

        // Reconstruct old editable region we sent to the model
        let old_editable_region = Self::format_editable_region(example);
        let old_editable_region = Self::extract_editable_region(&old_editable_region);

        // Apply editable region to a larger context and compute diff.
        // This is needed to get a better context lines around the editable region
        // TODO: Report an error when old_editable_region not found in cursor_excerpt
        let edited_cursor_excerpt =
            cursor_excerpt.replace(&old_editable_region, &new_editable_region);
        let diff = language::unified_diff(&cursor_excerpt, &edited_cursor_excerpt);

        diff
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
        let closing_backticks = "`".repeat(backtick_count);

        while backtick_end < bytes.len() && bytes[backtick_end] != b'\n' {
            backtick_end += 1;
        }

        if let Some(end_pos) = text[backtick_end..].find(&closing_backticks) {
            let code_block = &text[backtick_end + 1..backtick_end + end_pos - 1];
            last_block = Some(code_block.to_string());
            search_start = backtick_end + end_pos + backtick_count;
        } else {
            break;
        }
    }

    last_block.unwrap_or_else(|| text.to_string())
}

fn find_context_excerpt_under_cursor(example: &Example) -> Option<&ExampleContextExcerpt> {
    let context = example.context.as_ref().expect("Context must be provided");
    let cursor_position = example.cursor_position.replace("<|user_cursor|>", "");
    let cursor_file = context
        .files
        .iter()
        .filter(|f| f.rel_path == example.cursor_path)
        .next()?;
    let cursor_excerpt = cursor_file
        .excerpts
        .iter()
        .filter(|e| e.text.contains(&cursor_position))
        .next()?;

    Some(cursor_excerpt)
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
        assert_eq!(last_block, "last block");
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
            two three

            "}
        );
    }
}
