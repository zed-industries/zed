use crate::{
    PromptFormat,
    example::{Example, ExamplePrompt},
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

trait PromptFormatter {
    fn format(example: &Example) -> String;
}

struct Zeta2Prompt;
struct TeacherPrompt;

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
        result.push_str(&format!("{}\n", Self::EDITABLE_REGION_START));

        // TODO: control number of lines around cursor
        result.push_str(&example.cursor_position);
        if !example.cursor_position.ends_with('\n') {
            result.push('\n');
        }

        result.push_str(&format!("{}\n", Self::EDITABLE_REGION_END));
        result.push_str("`````");

        result
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
