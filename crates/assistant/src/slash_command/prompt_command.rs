use super::SlashCommand;
use fuzzy::PathMatch;
use gpui::{AppContext, Model, Task};
use std::sync::{atomic::AtomicBool, Arc};

pub(crate) struct PromptSlashCommand {}

impl PromptSlashCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlashCommand for PromptSlashCommand {
    fn name(&self) -> String {
        "prompt".into()
    }

    fn description(&self) -> String {
        "insert a prompt from the library".into()
    }

    fn complete_argument(
        &self,
        _query: String,
        _cancellation_flag: Arc<AtomicBool>,
        cx: &mut AppContext,
    ) -> Task<http::Result<Vec<String>>> {
        cx.background_executor()
            .spawn(async move { Ok(Vec::new()) })
    }
}
