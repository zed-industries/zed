use crate::{
    PromptFormat,
    example::{Example, ExamplePrompt},
};
use std::fmt::Write;

pub async fn run_format_prompt(example: &mut Example, prompt_format: PromptFormat) {
    match prompt_format {
        PromptFormat::Teacher => {
            // TODO: Format example for teacher
        }
        PromptFormat::Zeta2 => {
            let prompt = format_zeta2_prompt(example);
            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output: example.expected_patch.clone(),
                format: prompt_format,
            });
        }
    }
}

fn format_zeta2_prompt(example: &Example) -> String {
    let mut prompt = String::new();
    write_context_section(&mut prompt, example);
    write_edit_history_section(&mut prompt, example);
    write_cursor_excerpt_section(&mut prompt, example);
    prompt
}

fn write_context_section(prompt: &mut String, example: &Example) {
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

fn write_edit_history_section(prompt: &mut String, example: &Example) {
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

fn write_cursor_excerpt_section(prompt: &mut String, example: &Example) {
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
