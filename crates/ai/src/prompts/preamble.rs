use crate::prompts::base::{PromptArguments, PromptFileType, PromptTemplate};
use std::fmt::Write;

pub struct EngineerPreamble {}

impl PromptTemplate for EngineerPreamble {
    fn generate(
        &self,
        args: &PromptArguments,
        max_token_length: Option<usize>,
    ) -> anyhow::Result<(String, usize)> {
        let mut prompts = Vec::new();

        match args.get_file_type() {
            PromptFileType::Code => {
                prompts.push(format!(
                    "You are an expert {}engineer.",
                    args.language_name.clone().unwrap_or("".to_string()) + " "
                ));
            }
            PromptFileType::Text => {
                prompts.push("You are an expert engineer.".to_string());
            }
        }

        if let Some(project_name) = args.project_name.clone() {
            prompts.push(format!(
                "You are currently working inside the '{project_name}' project in code editor Zed."
            ));
        }

        if let Some(mut remaining_tokens) = max_token_length {
            let mut prompt = String::new();
            let mut total_count = 0;
            for prompt_piece in prompts {
                let prompt_token_count =
                    args.model.count_tokens(&prompt_piece)? + args.model.count_tokens("\n")?;
                if remaining_tokens > prompt_token_count {
                    writeln!(prompt, "{prompt_piece}").unwrap();
                    remaining_tokens -= prompt_token_count;
                    total_count += prompt_token_count;
                }
            }

            anyhow::Ok((prompt, total_count))
        } else {
            let prompt = prompts.join("\n");
            let token_count = args.model.count_tokens(&prompt)?;
            anyhow::Ok((prompt, token_count))
        }
    }
}
