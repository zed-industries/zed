use super::SlashCommand;
use anyhow::Result;
use gpui::{AppContext, Task};
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

    fn requires_argument(&self) -> bool {
        true
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

    fn run(&self, argument: Option<&str>, _cx: &mut AppContext) -> Task<Result<String>> {
        Task::ready(Ok(format!("inserting prompt {:?}", argument)))
    }
}
