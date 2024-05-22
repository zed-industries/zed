use language::BufferSnapshot;
use std::{fmt::Write, ops::Range};

use fs::{CreateOptions, Fs};
use futures::StreamExt;
use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use util::{paths::PROMPTS_DIR, ResultExt};

fn slugify(input: String) -> String {
    input
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "_")
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct StaticPromptFrontmatter {
    pub title: String,
    pub version: String,
    pub author: String,
    pub languages: Vec<String>,
    pub dependencies: Vec<String>,
}

/// A statuc prompt that can be loaded into the prompt library
/// from Markdown with a frontmatter header
///
/// Examples:
///
/// ### Globally available prompt
///
/// ```markdown
/// ---
/// title: Foo
/// version: 1.0
/// author: Jane Kim <jane@kim.com
/// languages: ["*"]
/// dependencies: []
/// ---
///
/// Foo and bar are terms used in programming to describe generic concepts.
/// ```
///
/// ### Language-specific prompt
///
/// ```markdown
/// ---
/// title: UI with GPUI
/// version: 1.0
/// author: Nate Butler <iamnbutler@gmail.com>
/// languages: ["rust"]
/// dependencies: ["gpui"]
/// ---
///
/// When building a UI with GPUI, ensure you...
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StaticPrompt {
    pub _raw: String,
    pub _file_name: Option<String>,
    pub version: String,
    pub title: String,
    pub author: String,
    pub languages: Vec<String>,
    pub dependencies: Vec<String>,
    pub body: String,
}

impl Default for StaticPrompt {
    fn default() -> Self {
        StaticPrompt::new(None)
    }
}

impl StaticPrompt {
    pub fn new(author: Option<String>) -> Self {
        let title = "New Prompt".to_string();
        let version = "1.0".to_string();
        let author = if let Some(author) = author {
            author
        } else {
            "Jane Kim <jane@kim.com>".to_string()
        };
        let languages = vec!["*".to_string()];
        let dependencies = vec![];

        let body = "Write a new prompt here".to_string();

        StaticPrompt {
            _raw: "".to_string(),
            _file_name: None,
            version,
            title,
            author,
            languages,
            dependencies,
            body,
        }
    }

    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        let matter = Matter::<YAML>::new();
        let result = matter.parse(s);

        #[derive(Deserialize, Debug)]
        struct FrontMatter {
            title: String,
            version: String,
            author: String,
            languages: Vec<String>,
            dependencies: Vec<String>,
        }

        impl Default for FrontMatter {
            fn default() -> Self {
                FrontMatter {
                    title: "New Prompt".to_string(),
                    version: "1.0".to_string(),
                    author: "No Author".to_string(),
                    languages: vec!["*".to_string()],
                    dependencies: vec![],
                }
            }
        }

        let front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap_or_default();

        let prompt = StaticPrompt {
            _raw: result.orig,
            _file_name: None,
            title: front_matter.title,
            version: front_matter.version,
            author: front_matter.author,
            languages: front_matter.languages,
            dependencies: front_matter.dependencies,
            body: result.content,
        };

        Ok(prompt)
    }

    pub fn to_str(&self) -> String {
        self._raw.clone()
    }

    pub fn file_name(&mut self, file_name: String) -> &mut Self {
        self._file_name = Some(file_name);
        self
    }

    pub async fn list(fs: Arc<dyn Fs>) -> anyhow::Result<Vec<Self>> {
        fs.create_dir(&PROMPTS_DIR).await?;

        let mut paths = fs.read_dir(&PROMPTS_DIR).await?;
        let mut prompts = Vec::new();

        while let Some(path_result) = paths.next().await {
            let path = match path_result {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error reading path: {:?}", e);
                    continue;
                }
            };

            if path.extension() == Some(std::ffi::OsStr::new("md")) {
                match fs.load(&path).await {
                    Ok(content) => match StaticPrompt::from_str(&content) {
                        Ok(user_prompt) => {
                            prompts.push(user_prompt);
                        }
                        Err(e) => eprintln!("{}", e),
                    },
                    Err(e) => eprintln!("Failed to load file {}: {}", path.display(), e),
                }
            }
        }

        Ok(prompts)
    }

    pub async fn save(&self, fs: Arc<dyn Fs>) -> anyhow::Result<()> {
        fs.create_dir(&PROMPTS_DIR).await?;

        let mut paths = fs.read_dir(&PROMPTS_DIR).await?;

        let file_name = if let Some(file_name) = &self._file_name {
            file_name.clone()
        } else {
            slugify(self.title.clone())
        };

        while let Some(path_result) = paths.next().await {
            let path = match path_result {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error reading path: {:?}", e);
                    continue;
                }
            };

            let save_path = path.join(&file_name).with_extension("md");
            let options = CreateOptions {
                overwrite: true,
                ignore_if_exists: false,
            };

            fs.create_file(&save_path.as_path(), options)
                .await
                .log_err();
            fs.save(
                &save_path.as_path(),
                &self._raw.as_str().into(),
                Default::default(),
            )
            .await
            .log_err();

            println!("Saved prompt to {}", save_path.display());
        }

        Ok(())
    }
}

pub fn generate_content_prompt(
    user_prompt: String,
    language_name: Option<&str>,
    buffer: BufferSnapshot,
    range: Range<usize>,
    project_name: Option<String>,
) -> anyhow::Result<String> {
    let mut prompt = String::new();

    let content_type = match language_name {
        None | Some("Markdown" | "Plain Text") => {
            writeln!(prompt, "You are an expert engineer.")?;
            "Text"
        }
        Some(language_name) => {
            writeln!(prompt, "You are an expert {language_name} engineer.")?;
            writeln!(
                prompt,
                "Your answer MUST always and only be valid {}.",
                language_name
            )?;
            "Code"
        }
    };

    if let Some(project_name) = project_name {
        writeln!(
            prompt,
            "You are currently working inside the '{project_name}' project in code editor Zed."
        )?;
    }

    // Include file content.
    for chunk in buffer.text_for_range(0..range.start) {
        prompt.push_str(chunk);
    }

    if range.is_empty() {
        prompt.push_str("<|START|>");
    } else {
        prompt.push_str("<|START|");
    }

    for chunk in buffer.text_for_range(range.clone()) {
        prompt.push_str(chunk);
    }

    if !range.is_empty() {
        prompt.push_str("|END|>");
    }

    for chunk in buffer.text_for_range(range.end..buffer.len()) {
        prompt.push_str(chunk);
    }

    prompt.push('\n');

    if range.is_empty() {
        writeln!(
            prompt,
            "Assume the cursor is located where the `<|START|>` span is."
        )
        .unwrap();
        writeln!(
            prompt,
            "{content_type} can't be replaced, so assume your answer will be inserted at the cursor.",
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
        writeln!(
            prompt,
            "Double check that you only return code and not the '<|START|' and '|END|'> spans"
        )
        .unwrap();
    }

    writeln!(prompt, "Never make remarks about the output.").unwrap();
    writeln!(
        prompt,
        "Do not return anything else, except the generated {content_type}."
    )
    .unwrap();

    Ok(prompt)
}
