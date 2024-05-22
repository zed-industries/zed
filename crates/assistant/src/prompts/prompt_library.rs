use collections::HashMap;
use fs::Fs;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use super::prompts::StaticPrompt;

// actions!(prompt_manager, [NewPrompt, EditPrompt, SavePrompt]);

pub struct PromptLibraryState {
    /// The default prompt all assistant contexts will start with
    _system_prompt: String,
    /// All [UserPrompt]s loaded into the library
    prompts: HashMap<PromptId, StaticPrompt>,
    /// Prompts included in the default prompt
    default_prompts: Vec<PromptId>,
    /// Prompts that have been changed but haven't been saved back to the file system
    _dirty_prompts: Vec<PromptId>,
    /// Prompts that have a pending update that hasn't been applied yet
    _updateable_prompts: Vec<PromptId>,
    /// Prompts that have been changed since they were loaded
    /// and can be reverted to their original state
    _revertable_prompts: Vec<PromptId>,
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
            state: RwLock::new(PromptLibraryState {
                _system_prompt: String::new(),
                prompts: HashMap::default(),
                default_prompts: Vec::new(),
                _dirty_prompts: Vec::new(),
                _updateable_prompts: Vec::new(),
                _revertable_prompts: Vec::new(),
                version: 0,
            }),
        }
    }

    pub async fn init(fs: Arc<dyn Fs>) -> anyhow::Result<Self> {
        let prompt_library = PromptLibrary::new();
        prompt_library.load_prompts(fs)?;
        Ok(prompt_library)
    }

    fn load_prompts(&self, fs: Arc<dyn Fs>) -> anyhow::Result<()> {
        let prompts = futures::executor::block_on(StaticPrompt::list(fs))?;
        let prompts_with_ids = prompts
            .clone()
            .into_iter()
            .map(|prompt| {
                let id = PromptId(uuid::Uuid::new_v4());
                (id, prompt)
            })
            .collect::<Vec<_>>();
        let mut state = self.state.write();
        state.prompts.extend(prompts_with_ids);
        state.version += 1;

        Ok(())
    }

    pub fn default_prompt(&self) -> Option<String> {
        let state = self.state.read();

        if state.default_prompts.is_empty() {
            None
        } else {
            Some(self.join_default_prompts())
        }
    }

    fn join_default_prompts(&self) -> String {
        let state = self.state.read();
        let active_prompt_ids = state.default_prompts.to_vec();

        active_prompt_ids
            .iter()
            .filter_map(|id| state.prompts.get(id).map(|p| p.body.clone()))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }

    #[allow(unused)]
    pub fn prompts(&self) -> Vec<StaticPrompt> {
        let state = self.state.read();
        state.prompts.values().cloned().collect()
    }

    pub fn prompts_with_ids(&self) -> Vec<(PromptId, StaticPrompt)> {
        let state = self.state.read();
        state
            .prompts
            .iter()
            .map(|(id, prompt)| (id.clone(), prompt.clone()))
            .collect()
    }

    pub fn prompt_str_for_id(&self, prompt_id: PromptId) -> Option<String> {
        self.state
            .read()
            .prompts
            .get(&prompt_id)
            .and_then(|prompt| Some(prompt.to_str()))
    }

    pub fn update_prompt_raw_for_id(
        &self,
        prompt_id: PromptId,
        new_raw: String,
    ) -> anyhow::Result<()> {
        let mut state = self.state.write();
        let prompt = state
            .prompts
            .get_mut(&prompt_id)
            .ok_or_else(|| anyhow::anyhow!("Prompt not found"))?;
        prompt._raw = new_raw;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PromptId(pub Uuid);
