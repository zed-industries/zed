use fs::Fs;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use util::paths::PROMPTS_DIR;

pub struct PromptLibrary {
    prompts: HashMap<String, UserPrompt>,
    active_prompts: Vec<String>,
}

impl PromptLibrary {
    pub fn new() -> Self {
        Self {
            prompts: HashMap::new(),
            active_prompts: Vec::new(),
        }
    }

    pub fn load_prompts(&mut self, fs: Arc<dyn Fs>) -> anyhow::Result<()> {
        let prompts = futures::executor::block_on(UserPrompt::list(fs))?;
        for prompt in prompts {
            let id = uuid::Uuid::new_v4().to_string();
            self.prompts.insert(id.clone(), prompt);
            // temp for testing, activate all prompts as they are loaded
            self.active_prompts.push(id);
        }
        Ok(())
    }

    pub fn active_prompt(&self) -> Option<String> {
        if self.active_prompts.is_empty() {
            None
        } else {
            Some(self.join_active_prompts())
        }
    }

    // pub fn activate_prompt(&mut self, prompt_id: String) -> anyhow::Result<()> {
    //     if !self.active_prompts.contains(&prompt_id) {
    //         self.active_prompts.push(prompt_id);
    //     }
    //     Ok(())
    // }

    // pub fn activate_prompts(&mut self, prompt_ids: Vec<String>) -> anyhow::Result<()> {
    //     for id in prompt_ids {
    //         self.activate_prompt(id)?;
    //     }
    //     Ok(())
    // }

    // pub fn deactivate_prompt(&mut self, prompt_id: String) -> anyhow::Result<()> {
    //     self.active_prompts.retain(|id| id != &prompt_id);
    //     Ok(())
    // }

    pub fn join_active_prompts(&self) -> String {
        let active_prompt_ids = &self.active_prompts;

        active_prompt_ids
            .iter()
            .map(|id| self.prompts.get(id).map(|p| p.content.clone()))
            .flatten()
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptMetadata {
    title: String,
    author: String,
    #[serde(default)]
    languages: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPrompt {
    metadata: PromptMetadata,
    content: String,
}

impl UserPrompt {
    fn parse_metadata(content: &str) -> anyhow::Result<(PromptMetadata, String)> {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() >= 3 {
            let frontmatter_str = parts[1].trim();
            let metadata: PromptMetadata = serde_yml::from_str(frontmatter_str)
                .map_err(|e| anyhow::anyhow!("Failed to parse front matter: {}", e))?;

            let content_body = parts.get(2).map_or("", |s| *s).trim();

            Ok((metadata, content_body.to_string()))
        } else {
            Err(anyhow::anyhow!("Invalid or missing front matter"))
        }
    }

    async fn list(fs: Arc<dyn Fs>) -> anyhow::Result<Vec<Self>> {
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
                    Ok(content) => match Self::parse_metadata(&content) {
                        Ok((metadata, content_body)) => prompts.push(UserPrompt {
                            metadata,
                            content: content_body,
                        }),
                        Err(e) => eprintln!("{}", e),
                    },
                    Err(e) => eprintln!("Failed to load file {}: {}", path.display(), e),
                }
            }
        }

        Ok(prompts)
    }
}
