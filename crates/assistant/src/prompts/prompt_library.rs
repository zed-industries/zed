use anyhow::Context;
use collections::HashMap;
use fs::Fs;

use gray_matter::{engine::YAML, Matter};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use smol::stream::StreamExt;
use std::sync::Arc;
use util::paths::PROMPTS_DIR;
use uuid::Uuid;

use super::prompt::StaticPrompt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PromptId(pub Uuid);

#[allow(unused)]
impl PromptId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct PromptLibraryState {
    /// A set of prompts that all assistant contexts will start with
    default_prompt: Vec<PromptId>,
    /// All [Prompt]s loaded into the library
    prompts: HashMap<PromptId, StaticPrompt>,
    /// Prompts that have been changed but haven't been
    /// saved back to the file system
    dirty_prompts: Vec<PromptId>,
    version: usize,
}

pub struct PromptLibrary {
    state: RwLock<PromptLibraryState>,
}

impl Default for PromptLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptLibrary {
    fn new() -> Self {
        Self {
            state: RwLock::new(PromptLibraryState::default()),
        }
    }

    pub fn prompts(&self) -> Vec<(PromptId, StaticPrompt)> {
        let state = self.state.read();
        state
            .prompts
            .iter()
            .map(|(id, prompt)| (*id, prompt.clone()))
            .collect()
    }

    pub fn first_prompt_id(&self) -> Option<PromptId> {
        let state = self.state.read();
        state.prompts.keys().next().cloned()
    }

    pub fn prompt(&self, id: PromptId) -> Option<StaticPrompt> {
        let state = self.state.read();
        state.prompts.get(&id).cloned()
    }

    /// Save the current state of the prompt library to the
    /// file system as a JSON file
    pub async fn save(&self, fs: Arc<dyn Fs>) -> anyhow::Result<()> {
        fs.create_dir(&PROMPTS_DIR).await?;

        let path = PROMPTS_DIR.join("index.json");

        let json = {
            let state = self.state.read();
            serde_json::to_string(&*state)?
        };

        fs.atomic_write(path, json).await?;

        Ok(())
    }

    /// Load the state of the prompt library from the file system
    /// or create a new one if it doesn't exist
    pub async fn load(fs: Arc<dyn Fs>) -> anyhow::Result<Self> {
        let path = PROMPTS_DIR.join("index.json");

        let state = if fs.is_file(&path).await {
            let json = fs.load(&path).await?;
            serde_json::from_str(&json)?
        } else {
            PromptLibraryState::default()
        };

        let mut prompt_library = Self {
            state: RwLock::new(state),
        };

        prompt_library.load_prompts(fs).await?;

        Ok(prompt_library)
    }

    /// Load all prompts from the file system
    /// adding them to the library if they don't already exist
    pub async fn load_prompts(&mut self, fs: Arc<dyn Fs>) -> anyhow::Result<()> {
        // let current_prompts = self.all_prompt_contents().clone();

        // For now, we'll just clear the prompts and reload them all
        self.state.get_mut().prompts.clear();

        let mut prompt_paths = fs.read_dir(&PROMPTS_DIR).await?;

        while let Some(prompt_path) = prompt_paths.next().await {
            let prompt_path = prompt_path.with_context(|| "Failed to read prompt path")?;
            let file_name_lossy = if prompt_path.file_name().is_some() {
                Some(
                    prompt_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                )
            } else {
                None
            };

            if !fs.is_file(&prompt_path).await
                || prompt_path.extension().and_then(|ext| ext.to_str()) != Some("md")
            {
                continue;
            }

            let json = fs
                .load(&prompt_path)
                .await
                .with_context(|| format!("Failed to load prompt {:?}", prompt_path))?;

            // Check that the prompt is valid
            let matter = Matter::<YAML>::new();
            let result = matter.parse(&json);
            if result.data.is_none() {
                log::warn!("Invalid prompt: {:?}", prompt_path);
                continue;
            }

            let static_prompt = StaticPrompt::new(json, file_name_lossy.clone());

            let state = self.state.get_mut();

            let id = Uuid::new_v4();
            state.prompts.insert(PromptId(id), static_prompt);
            state.version += 1;
        }

        // Write any changes back to the file system
        self.save(fs.clone()).await?;

        Ok(())
    }
}
