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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SortOrder {
    Alphabetical,
}

#[allow(unused)]
impl PromptId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_str(id: &str) -> anyhow::Result<Self> {
        Ok(Self(Uuid::parse_str(id)?))
    }
}

impl Default for PromptId {
    fn default() -> Self {
        Self::new()
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

    pub fn new_prompt(&self) -> StaticPrompt {
        StaticPrompt::default()
    }

    pub fn add_prompt(&self, prompt: StaticPrompt) {
        let mut state = self.state.write();
        let id = *prompt.id();
        state.prompts.insert(id, prompt);
        state.version += 1;
    }

    pub fn prompts(&self) -> HashMap<PromptId, StaticPrompt> {
        let state = self.state.read();
        state.prompts.clone()
    }

    pub fn sorted_prompts(&self, sort_order: SortOrder) -> Vec<(PromptId, StaticPrompt)> {
        let state = self.state.read();

        let mut prompts = state
            .prompts
            .iter()
            .map(|(id, prompt)| (*id, prompt.clone()))
            .collect::<Vec<_>>();

        match sort_order {
            SortOrder::Alphabetical => prompts.sort_by(|(_, a), (_, b)| a.title().cmp(&b.title())),
        };

        prompts
    }

    pub fn prompt_by_id(&self, id: PromptId) -> Option<StaticPrompt> {
        let state = self.state.read();
        state.prompts.get(&id).cloned()
    }

    pub fn first_prompt_id(&self) -> Option<PromptId> {
        let state = self.state.read();
        state.prompts.keys().next().cloned()
    }

    pub fn is_dirty(&self, id: &PromptId) -> bool {
        let state = self.state.read();
        state.dirty_prompts.contains(&id)
    }

    pub fn set_dirty(&self, id: PromptId, dirty: bool) {
        let mut state = self.state.write();
        if dirty {
            if !state.dirty_prompts.contains(&id) {
                state.dirty_prompts.push(id);
            }
            state.version += 1;
        } else {
            state.dirty_prompts.retain(|&i| i != id);
            state.version += 1;
        }
    }

    /// Load the state of the prompt library from the file system
    /// or create a new one if it doesn't exist
    pub async fn load_index(fs: Arc<dyn Fs>) -> anyhow::Result<Self> {
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
        self.save_index(fs.clone()).await?;

        Ok(())
    }

    /// Save the current state of the prompt library to the
    /// file system as a JSON file
    pub async fn save_index(&self, fs: Arc<dyn Fs>) -> anyhow::Result<()> {
        fs.create_dir(&PROMPTS_DIR).await?;

        let path = PROMPTS_DIR.join("index.json");

        let json = {
            let state = self.state.read();
            serde_json::to_string(&*state)?
        };

        fs.atomic_write(path, json).await?;

        Ok(())
    }

    pub async fn save_prompt(
        &self,
        prompt_id: PromptId,
        updated_content: Option<String>,
        fs: Arc<dyn Fs>,
    ) -> anyhow::Result<()> {
        if let Some(updated_content) = updated_content {
            let mut state = self.state.write();
            if let Some(prompt) = state.prompts.get_mut(&prompt_id) {
                prompt.update(prompt_id, updated_content);
                state.version += 1;
            }
        }

        if let Some(prompt) = self.prompt_by_id(prompt_id) {
            prompt.save(fs).await?;
            self.set_dirty(prompt_id, false);
        } else {
            log::warn!("Failed to save prompt: {:?}", prompt_id);
        }

        Ok(())
    }
}
