use crate::prompts::base::{PromptArguments, PromptFileType, PromptTemplate};
use anyhow::anyhow;
use std::fmt::Write;

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub struct GenerateInlineContent {}

impl PromptTemplate for GenerateInlineContent {
    fn generate(
        &self,
        args: &PromptArguments,
        max_token_length: Option<usize>,
    ) -> anyhow::Result<(String, usize)> {
        let Some(user_prompt) = &args.user_prompt else {
            return Err(anyhow!("user prompt not provided"));
        };

        let file_type = args.get_file_type();
        let content_type = match &file_type {
            PromptFileType::Code => "code",
            PromptFileType::Text => "text",
        };

        let mut prompt = String::new();

        if let Some(selected_range) = &args.selected_range {
            if selected_range.start == selected_range.end {
                writeln!(
                    prompt,
                    "Assume the cursor is located where the `<|START|>` span is."
                )
                .unwrap();
                writeln!(
                    prompt,
                    "{} can't be replaced, so assume your answer will be inserted at the cursor.",
                    capitalize(content_type)
                )
                .unwrap();
                writeln!(
                    prompt,
                    "Generate {content_type} based on the users prompt: {user_prompt}",
                )
                .unwrap();
            } else {
                writeln!(prompt, "Modify the user's selected {content_type} based upon the users prompt: '{user_prompt}'").unwrap();
                writeln!(prompt, "You must reply with only the adjusted {content_type} (within the '<|START|' and '|END|>' spans) not the entire file.").unwrap();
                writeln!(prompt, "Double check that you only return code and not the '<|START|' and '|END|'> spans").unwrap();
            }
        } else {
            writeln!(
                prompt,
                "Generate {content_type} based on the users prompt: {user_prompt}"
            )
            .unwrap();
        }

        if let Some(language_name) = &args.language_name {
            writeln!(
                prompt,
                "Your answer MUST always and only be valid {}.",
                language_name
            )
            .unwrap();
        }
        writeln!(prompt, "Never make remarks about the output.").unwrap();
        writeln!(
            prompt,
            "Do not return anything else, except the generated {content_type}."
        )
        .unwrap();

        match file_type {
            PromptFileType::Code => {
                // writeln!(prompt, "Always wrap your code in a Markdown block.").unwrap();
            }
            _ => {}
        }

        // Really dumb truncation strategy
        if let Some(max_tokens) = max_token_length {
            prompt = args.model.truncate(
                &prompt,
                max_tokens,
                crate::models::TruncationDirection::End,
            )?;
        }

        let token_count = args.model.count_tokens(&prompt)?;

        anyhow::Ok((prompt, token_count))
    }
}
