mod slash_command_registry;

use anyhow::Result;
use gpui::{AnyElement, AppContext, ElementId, SharedString, Task, WeakView, WindowContext};
use language::{CodeLabel, LspAdapterDelegate};
use serde::{Deserialize, Serialize};
pub use slash_command_registry::*;
use std::{
    ops::Range,
    sync::{atomic::AtomicBool, Arc},
};
use workspace::{ui::IconName, Workspace};

pub fn init(cx: &mut AppContext) {
    SlashCommandRegistry::default_global(cx);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AfterCompletion {
    /// Run the command
    Run,
    /// Continue composing the current argument, doesn't add a space
    Compose,
    /// Continue the command composition, adds a space
    Continue,
}

impl From<bool> for AfterCompletion {
    fn from(value: bool) -> Self {
        if value {
            AfterCompletion::Run
        } else {
            AfterCompletion::Continue
        }
    }
}

impl AfterCompletion {
    pub fn run(&self) -> bool {
        match self {
            AfterCompletion::Run => true,
            AfterCompletion::Compose | AfterCompletion::Continue => false,
        }
    }
}

#[derive(Debug)]
pub struct ArgumentCompletion {
    /// The label to display for this completion.
    pub label: CodeLabel,
    /// The new text that should be inserted into the command when this completion is accepted.
    pub new_text: String,
    /// Whether the command should be run when accepting this completion.
    pub after_completion: AfterCompletion,
    /// Whether to replace the all arguments, or whether to treat this as an independent argument.
    pub replace_previous_arguments: bool,
}

pub trait SlashCommand: 'static + Send + Sync {
    fn name(&self) -> String;
    fn label(&self, _cx: &AppContext) -> CodeLabel {
        CodeLabel::plain(self.name(), None)
    }
    fn description(&self) -> String;
    fn menu_text(&self) -> String;
    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        cancel: Arc<AtomicBool>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>>;
    fn requires_argument(&self) -> bool;
    fn accepts_arguments(&self) -> bool {
        self.requires_argument()
    }
    fn run(
        self: Arc<Self>,
        arguments: &[String],
        workspace: WeakView<Workspace>,
        // TODO: We're just using the `LspAdapterDelegate` here because that is
        // what the extension API is already expecting.
        //
        // It may be that `LspAdapterDelegate` needs a more general name, or
        // perhaps another kind of delegate is needed here.
        delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>>;
}

pub type RenderFoldPlaceholder = Arc<
    dyn Send
        + Sync
        + Fn(ElementId, Arc<dyn Fn(&mut WindowContext)>, &mut WindowContext) -> AnyElement,
>;

#[derive(Debug, Default)]
pub struct SlashCommandOutput {
    pub text: String,
    pub sections: Vec<SlashCommandOutputSection<usize>>,
    pub run_commands_in_text: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlashCommandOutputSection<T> {
    pub range: Range<T>,
    pub icon: IconName,
    pub label: SharedString,
}
