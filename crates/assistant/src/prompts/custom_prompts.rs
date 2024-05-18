use fs::Fs;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use util::paths::PROMPTS_DIR;

use super::prompt_library::PromptLibrary;

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
            version,
            title,
            author,
            languages,
            dependencies,
            body,
        }
    }

    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        let parsed: StaticPrompt = serde_yml::from_str(s)?;
        Ok(parsed)
    }

    pub fn to_str(&self) -> anyhow::Result<String> {
        let s = serde_yml::to_string(self)?;
        Ok(s)
    }
}

impl PromptLibrary {
    pub async fn load_prompts(fs: Arc<dyn Fs>) -> anyhow::Result<Vec<StaticPrompt>> {
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
                    Ok(content) => {
                        let user_prompt: StaticPrompt = StaticPrompt::from_str(&content)?;
                        prompts.push(user_prompt);
                    }
                    Err(e) => eprintln!("Failed to load file {}: {}", path.display(), e),
                }
            }
        }

        Ok(prompts)
    }
}

/// A custom prompt that can be loaded into the prompt library
///
/// Example:
///
/// ```json
/// {
///   "title": "Foo",
///   "version": "1.0",
///   "author": "Jane Kim <jane@kim.com>",
///   "languages": ["*"], // or ["rust", "python", "javascript"] etc...
///   "prompt": "bar"
/// }
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPrompt {
    pub version: String,
    pub title: String,
    pub author: String,
    pub languages: Vec<String>,
    pub body: String,
}

impl CustomPrompt {
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

            if path.extension() == Some(std::ffi::OsStr::new("json")) {
                match fs.load(&path).await {
                    Ok(content) => {
                        let user_prompt: CustomPrompt =
                            serde_json::from_str(&content).map_err(|e| {
                                anyhow::anyhow!("Failed to deserialize UserPrompt: {}", e)
                            })?;

                        prompts.push(user_prompt);
                    }
                    Err(e) => eprintln!("Failed to load file {}: {}", path.display(), e),
                }
            }
        }

        Ok(prompts)
    }
}
