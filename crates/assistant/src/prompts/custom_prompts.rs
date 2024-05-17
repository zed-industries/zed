use fs::Fs;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use util::paths::PROMPTS_DIR;

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
