use crate::prompts::base::{PromptArguments, PromptTemplate};
use std::fmt::Write;
use std::{ops::Range, path::PathBuf};

use gpui::{AsyncAppContext, Model};
use language::{Anchor, Buffer};

#[derive(Clone)]
pub struct PromptCodeSnippet {
    path: Option<PathBuf>,
    language_name: Option<String>,
    content: String,
}

impl PromptCodeSnippet {
    pub fn new(
        buffer: Model<Buffer>,
        range: Range<Anchor>,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<Self> {
        let (content, language_name, file_path) = buffer.update(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            let content = snapshot.text_for_range(range.clone()).collect::<String>();

            let language_name = buffer
                .language()
                .map(|language| language.name().to_string().to_lowercase());

            let file_path = buffer.file().map(|file| file.path().to_path_buf());

            (content, language_name, file_path)
        })?;

        anyhow::Ok(PromptCodeSnippet {
            path: file_path,
            language_name,
            content,
        })
    }
}

impl ToString for PromptCodeSnippet {
    fn to_string(&self) -> String {
        let path = self
            .path
            .as_ref()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or("".to_string());
        let language_name = self.language_name.clone().unwrap_or("".to_string());
        let content = self.content.clone();

        format!("The below code snippet may be relevant from file: {path}\n```{language_name}\n{content}\n```")
    }
}

pub struct RepositoryContext {}

impl PromptTemplate for RepositoryContext {
    fn generate(
        &self,
        args: &PromptArguments,
        max_token_length: Option<usize>,
    ) -> anyhow::Result<(String, usize)> {
        const MAXIMUM_SNIPPET_TOKEN_COUNT: usize = 500;
        let template = "You are working inside a large repository, here are a few code snippets that may be useful.";
        let mut prompt = String::new();

        let mut remaining_tokens = max_token_length;
        let separator_token_length = args.model.count_tokens("\n")?;
        for snippet in &args.snippets {
            let mut snippet_prompt = template.to_string();
            let content = snippet.to_string();
            writeln!(snippet_prompt, "{content}").unwrap();

            let token_count = args.model.count_tokens(&snippet_prompt)?;
            if token_count <= MAXIMUM_SNIPPET_TOKEN_COUNT {
                if let Some(tokens_left) = remaining_tokens {
                    if tokens_left >= token_count {
                        writeln!(prompt, "{snippet_prompt}").unwrap();
                        remaining_tokens = if tokens_left >= (token_count + separator_token_length)
                        {
                            Some(tokens_left - token_count - separator_token_length)
                        } else {
                            Some(0)
                        };
                    }
                } else {
                    writeln!(prompt, "{snippet_prompt}").unwrap();
                }
            }
        }

        let total_token_count = args.model.count_tokens(&prompt)?;
        anyhow::Ok((prompt, total_token_count))
    }
}
