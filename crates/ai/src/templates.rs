use std::fmt::Write;

pub struct PromptCodeSnippet {
    path: Option<PathBuf>,
    language_name: Option<String>,
    content: String,
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
    fn generate(args: PromptArguments) -> String;
}

struct EngineerPreamble {}

impl PromptTemplate for EngineerPreamble {
    fn generate(args: PromptArguments) -> String {
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
    fn generate(args: PromptArguments) -> String {}
}
