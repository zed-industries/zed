use crate::templates::base::{PromptArguments, PromptFileType, PromptTemplate};
use std::fmt::Write;

struct EngineerPreamble {}

impl PromptTemplate for EngineerPreamble {
    fn generate(&self, args: &PromptArguments, max_token_length: Option<usize>) -> String {
        let mut prompt = String::new();

        match args.get_file_type() {
            PromptFileType::Code => {
                writeln!(
                    prompt,
                    "You are an expert {} engineer.",
                    args.language_name.clone().unwrap_or("".to_string())
                )
                .unwrap();
            }
            PromptFileType::Text => {
                writeln!(prompt, "You are an expert engineer.").unwrap();
            }
        }

        if let Some(project_name) = args.project_name.clone() {
            writeln!(
                prompt,
                "You are currently working inside the '{project_name}' in Zed the code editor."
            )
            .unwrap();
        }

        prompt
    }
}
