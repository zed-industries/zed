use anyhow::Context;
use collections::HashMap;
use fs::Fs;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use smol::stream::StreamExt;
use std::sync::Arc;
use util::{paths::PROMPTS_DIR, ResultExt};
use uuid::Uuid;

use super::{prompt_library::PromptId, prompts2::StaticPrompt2};

#[derive(Serialize, Deserialize)]
pub struct PromptLibraryState2 {
    /// A set of prompts that all assistant contexts will start with
    default_prompt: Vec<PromptId>,
    /// All [Prompt]s loaded into the library
    prompts: HashMap<PromptId, StaticPrompt2>,
    /// Prompts that have been changed but haven't been
    /// saved back to the file system
    dirty_prompts: Vec<PromptId>,
    version: usize,
}

impl Default for PromptLibraryState2 {
    fn default() -> Self {
        Self {
            default_prompt: Vec::new(),
            prompts: HashMap::default(),
            dirty_prompts: Vec::new(),
            version: 0,
        }
    }
}

pub struct PromptLibrary2 {
    state: RwLock<PromptLibraryState2>,
}

impl Default for PromptLibrary2 {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptLibrary2 {
    pub fn init(fs: Arc<dyn Fs>) -> anyhow::Result<Self> {
        let prompt_library = futures::executor::block_on(Self::load(fs))?;
        Ok(prompt_library)
    }

    fn new() -> Self {
        Self {
            state: RwLock::new(PromptLibraryState2::default()),
        }
    }

    fn all_prompt_contents(&self) -> Vec<String> {
        let state = self.state.read();
        state
            .prompts
            .values()
            .map(|prompt| prompt.content().clone())
            .collect()
    }

    /// Save the current state of the prompt library to the
    /// file system as a JSON file
    pub async fn save(&self, fs: Arc<dyn Fs>) -> anyhow::Result<()> {
        fs.create_dir(&PROMPTS_DIR).await?;

        let path = PROMPTS_DIR.join("index.json");

        let state = self.state.read();
        let json = serde_json::to_string(&*state)?;

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
            PromptLibraryState2::default()
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
        let current_prompts = self.all_prompt_contents().clone();
        let mut prompt_paths = fs.read_dir(&PROMPTS_DIR).await?;

        while let Some(prompt_path) = prompt_paths.next().await {
            let prompt_path = prompt_path.with_context(|| "Failed to read prompt path")?;

            if !fs.is_file(&prompt_path).await {
                continue;
            }

            let json = fs
                .load(&prompt_path)
                .await
                .with_context(|| format!("Failed to load prompt {:?}", prompt_path))?;
            let mut static_prompt = StaticPrompt2::new(json);

            if let Some(file_name) = prompt_path.file_name() {
                let file_name = file_name.to_string_lossy().into_owned();
                static_prompt.file_name(file_name);
            }

            let state = self.state.get_mut();

            if !current_prompts.contains(&static_prompt.content()) {
                let id = Uuid::new_v4();
                state.prompts.insert(PromptId(id), static_prompt);
            }
        }

        // Write any changes back to the file system
        self.save(fs.clone()).await?;

        Ok(())
    }
}
