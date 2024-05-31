mod slash_command_registry;

use anyhow::Result;
use gpui::{AnyElement, AppContext, ElementId, Task, WeakView, WindowContext};
use language::{CodeLabel, LspAdapterDelegate};
pub use slash_command_registry::*;
use std::{
    ops::Range,
    sync::{atomic::AtomicBool, Arc},
};
use workspace::Workspace;

pub fn init(cx: &mut AppContext) {
    SlashCommandRegistry::default_global(cx);
}

pub trait SlashCommand: 'static + Send + Sync {
    fn name(&self) -> String;
    fn label(&self, _cx: &AppContext) -> CodeLabel {
        CodeLabel::plain(self.name(), None)
    }
    fn description(&self) -> String;
    fn menu_text(&self) -> String;
    fn complete_argument(
        &self,
        query: String,
        cancel: Arc<AtomicBool>,
        workspace: WeakView<Workspace>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>>;
    fn requires_argument(&self) -> bool;
    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        // TODO: We're just using the `LspAdapterDelegate` here because that is
        // what the extension API is already expecting.
        //
        // It may be that `LspAdapterDelegate` needs a more general name, or
        // perhaps another kind of delegate is needed here.
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>>;
}

pub type RenderFoldPlaceholder = Arc<
    dyn Send
        + Sync
        + Fn(ElementId, Arc<dyn Fn(&mut WindowContext)>, &mut WindowContext) -> AnyElement,
>;

pub struct SlashCommandOutput {
    pub text: String,
    pub sections: Vec<SlashCommandOutputSection<usize>>,
}

#[derive(Clone)]
pub struct SlashCommandOutputSection<T> {
    pub range: Range<T>,
    pub render_placeholder: RenderFoldPlaceholder,
}
