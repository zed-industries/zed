use fs::Fs;
use futures::StreamExt;
use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use util::paths::PROMPTS_DIR;

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
}

impl StaticPrompt {
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

    // /// Load all static prompts from the prompts directory
    // pub async fn list(fs: Arc<dyn Fs>) -> anyhow::Result<Vec<StaticPrompt>> {
    //     fs.create_dir(&PROMPTS_DIR).await?;

    //     let mut paths = fs.read_dir(&PROMPTS_DIR).await?;
    //     let mut prompts = Vec::new();

    //     while let Some(path_result) = paths.next().await {
    //         let path = match path_result {
    //             Ok(p) => p,
    //             Err(e) => {
    //                 eprintln!("Error reading path: {:?}", e);
    //                 continue;
    //             }
    //         };

    //         if path.extension() == Some(std::ffi::OsStr::new("md")) {
    //             match fs.load(&path).await {
    //                 Ok(content) => {
    //                     let user_prompt: StaticPrompt = StaticPrompt::from_str(&content)?;
    //                     println!("Loaded prompt: {}", user_prompt.clone().title.clone());
    //                     prompts.push(user_prompt);
    //                 }
    //                 Err(e) => eprintln!("Failed to load file {}: {}", path.display(), e),
    //             }
    //         }
    //     }

    //     Ok(prompts)
    // }
}
