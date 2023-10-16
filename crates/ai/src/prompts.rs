use gpui::{AsyncAppContext, ModelHandle};
use language::{Anchor, Buffer};
use std::{fmt::Write, ops::Range, path::PathBuf};

pub struct PromptCodeSnippet {
    path: Option<PathBuf>,
    language_name: Option<String>,
    content: String,
}

impl PromptCodeSnippet {
    pub fn new(buffer: ModelHandle<Buffer>, range: Range<Anchor>, cx: &AsyncAppContext) -> Self {
        let (content, language_name, file_path) = buffer.read_with(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            let content = snapshot.text_for_range(range.clone()).collect::<String>();

            let language_name = buffer
                .language()
                .and_then(|language| Some(language.name().to_string()));

            let file_path = buffer
                .file()
                .and_then(|file| Some(file.path().to_path_buf()));

            (content, language_name, file_path)
        });

        PromptCodeSnippet {
            path: file_path,
            language_name,
            content,
        }
    }
}

impl ToString for PromptCodeSnippet {
    fn to_string(&self) -> String {
        let path = self
            .path
            .as_ref()
            .and_then(|path| Some(path.to_string_lossy().to_string()))
            .unwrap_or("".to_string());
        let language_name = self.language_name.clone().unwrap_or("".to_string());
        let content = self.content.clone();

        format!("The below code snippet may be relevant from file: {path}\n```{language_name}\n{content}\n```")
    }
}

enum PromptFileType {
    Text,
    Code,
}

#[derive(Default)]
struct PromptArguments {
    pub language_name: Option<String>,
    pub project_name: Option<String>,
    pub snippets: Vec<PromptCodeSnippet>,
    pub model_name: String,
}

impl PromptArguments {
    pub fn get_file_type(&self) -> PromptFileType {
        if self
            .language_name
            .as_ref()
            .and_then(|name| Some(!["Markdown", "Plain Text"].contains(&name.as_str())))
            .unwrap_or(true)
        {
            PromptFileType::Code
        } else {
            PromptFileType::Text
        }
    }
}

trait PromptTemplate {
    fn generate(args: PromptArguments, max_token_length: Option<usize>) -> String;
}

struct EngineerPreamble {}

impl PromptTemplate for EngineerPreamble {
    fn generate(args: PromptArguments, max_token_length: Option<usize>) -> String {
        let mut prompt = String::new();

        match args.get_file_type() {
            PromptFileType::Code => {
                writeln!(
                    prompt,
                    "You are an expert {} engineer.",
                    args.language_name.unwrap_or("".to_string())
                )
                .unwrap();
            }
            PromptFileType::Text => {
                writeln!(prompt, "You are an expert engineer.").unwrap();
            }
        }

        if let Some(project_name) = args.project_name {
            writeln!(
                prompt,
                "You are currently working inside the '{project_name}' in Zed the code editor."
            )
            .unwrap();
        }

        prompt
    }
}

struct RepositorySnippets {}

impl PromptTemplate for RepositorySnippets {
    fn generate(args: PromptArguments, max_token_length: Option<usize>) -> String {
        const MAXIMUM_SNIPPET_TOKEN_COUNT: usize = 500;
        let mut template = "You are working inside a large repository, here are a few code snippets that may be useful";
        let mut prompt = String::new();

        if let Ok(encoding) = tiktoken_rs::get_bpe_from_model(args.model_name.as_str()) {
            let default_token_count =
                tiktoken_rs::model::get_context_size(args.model_name.as_str());
            let mut remaining_token_count = max_token_length.unwrap_or(default_token_count);

            for snippet in args.snippets {
                let mut snippet_prompt = template.to_string();
                let content = snippet.to_string();
                writeln!(snippet_prompt, "{content}").unwrap();

                let token_count = encoding
                    .encode_with_special_tokens(snippet_prompt.as_str())
                    .len();
                if token_count <= remaining_token_count {
                    if token_count < MAXIMUM_SNIPPET_TOKEN_COUNT {
                        writeln!(prompt, "{snippet_prompt}").unwrap();
                        remaining_token_count -= token_count;
                        template = "";
                    }
                } else {
                    break;
                }
            }
        }

        prompt
    }
}
