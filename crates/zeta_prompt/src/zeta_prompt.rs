use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Write;
use std::ops::Range;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZetaPromptInput<'a> {
    #[serde(borrow)]
    pub cursor_path: Cow<'a, Path>,
    #[serde(borrow)]
    pub cursor_position: Cow<'a, str>,
    #[serde(borrow)]
    pub edit_history: Cow<'a, str>,
    #[serde(borrow)]
    pub context: Option<ZetaPromptContext<'a>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZetaPromptContext<'a> {
    #[serde(borrow)]
    pub files: Vec<ZetaPromptContextFile<'a>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZetaPromptContextFile<'a> {
    #[serde(borrow)]
    pub rel_path: Cow<'a, Path>,
    #[serde(borrow)]
    pub excerpts: Vec<ZetaPromptExcerpt<'a>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZetaPromptExcerpt<'a> {
    pub row_range: Range<u32>,
    #[serde(borrow)]
    pub text: Cow<'a, str>,
}

pub struct Zeta2Prompt;

impl Zeta2Prompt {
    pub fn format(input: &ZetaPromptInput) -> String {
        let mut prompt = String::new();
        Self::write_context_section(&mut prompt, input);
        Self::write_edit_history_section(&mut prompt, input);
        Self::write_cursor_excerpt_section(&mut prompt, input);
        prompt
    }

    pub fn write_context_section(prompt: &mut String, input: &ZetaPromptInput) {
        prompt.push_str("<context>\n");

        if let Some(context) = &input.context {
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

    pub fn write_edit_history_section(prompt: &mut String, input: &ZetaPromptInput) {
        prompt.push_str("<edit_history>\n");

        if input.edit_history.is_empty() {
            prompt.push_str("(No edit history)\n");
        } else {
            prompt.push_str(&input.edit_history);
            if !input.edit_history.ends_with('\n') {
                prompt.push('\n');
            }
        }

        prompt.push_str("</edit_history>\n\n");
    }

    pub fn write_cursor_excerpt_section(prompt: &mut String, input: &ZetaPromptInput) {
        prompt.push_str("<cursor_excerpt>\n");

        let path_str = input.cursor_path.to_string_lossy();
        writeln!(prompt, "<file path=\"{}\">", path_str).unwrap();

        prompt.push_str("<editable_region>\n");

        prompt.push_str(&input.cursor_position);
        if !input.cursor_position.ends_with('\n') {
            prompt.push('\n');
        }

        prompt.push_str("</editable_region>\n");
        prompt.push_str("</file>\n");

        prompt.push_str("</cursor_excerpt>\n");
    }
}
