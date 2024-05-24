mod slash_command_registry;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::Result;
use futures::channel::oneshot;
use gpui::{AppContext, Task};

pub use slash_command_registry::*;

pub fn init(cx: &mut AppContext) {
    SlashCommandRegistry::default_global(cx);
}

pub trait SlashCommand: 'static + Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn complete_argument(
        &self,
        query: String,
        cancel: Arc<AtomicBool>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>>;
    fn requires_argument(&self) -> bool;
    fn run(&self, argument: Option<&str>, cx: &mut AppContext) -> SlashCommandInvocation;
}

pub struct SlashCommandInvocation {
    pub output: Task<Result<String>>,
    pub invalidated: oneshot::Receiver<()>,
    pub cleanup: SlashCommandCleanup,
}

#[derive(Default)]
pub struct SlashCommandCleanup(Option<Box<dyn FnOnce()>>);

impl SlashCommandCleanup {
    pub fn new(cleanup: impl FnOnce() + 'static) -> Self {
        Self(Some(Box::new(cleanup)))
    }
}

impl Drop for SlashCommandCleanup {
    fn drop(&mut self) {
        if let Some(cleanup) = self.0.take() {
            cleanup();
        }
    }
}
