mod action_log;
pub mod outline;
mod tool_registry;
mod tool_schema;
mod tool_working_set;

use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Deref;
use std::sync::Arc;

use anyhow::Result;
use gpui::AnyElement;
use gpui::AnyWindowHandle;
use gpui::Context;
use gpui::IntoElement;
use gpui::Window;
use gpui::{App, Entity, SharedString, Task, WeakEntity};
use icons::IconName;
use language_model::LanguageModel;
use language_model::LanguageModelRequestMessage;
use language_model::LanguageModelToolSchemaFormat;
use project::Project;
use workspace::Workspace;

pub use crate::action_log::*;
pub use crate::tool_registry::*;
pub use crate::tool_schema::*;
pub use crate::tool_working_set::*;

pub fn init(cx: &mut App) {
    ToolRegistry::default_global(cx);
}

#[derive(Debug, Clone)]
pub enum ToolUseStatus {
    InputStillStreaming,
    NeedsConfirmation,
    Pending,
    Running,
    Finished(SharedString),
    Error(SharedString),
}

impl ToolUseStatus {
    pub fn text(&self) -> SharedString {
        match self {
            ToolUseStatus::NeedsConfirmation => "".into(),
            ToolUseStatus::InputStillStreaming => "".into(),
            ToolUseStatus::Pending => "".into(),
            ToolUseStatus::Running => "".into(),
            ToolUseStatus::Finished(out) => out.clone(),
            ToolUseStatus::Error(out) => out.clone(),
        }
    }

    pub fn error(&self) -> Option<SharedString> {
        match self {
            ToolUseStatus::Error(out) => Some(out.clone()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ToolResultOutput {
    pub content: String,
    pub output: Option<serde_json::Value>,
}

impl From<String> for ToolResultOutput {
    fn from(value: String) -> Self {
        ToolResultOutput {
            content: value,
            output: None,
        }
    }
}

impl Deref for ToolResultOutput {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

/// The result of running a tool, containing both the asynchronous output
/// and an optional card view that can be rendered immediately.
pub struct ToolResult {
    /// The asynchronous task that will eventually resolve to the tool's output
    pub output: Task<Result<ToolResultOutput>>,
    /// An optional view to present the output of the tool.
    pub card: Option<AnyToolCard>,
}

pub trait ToolCard: 'static + Sized {
    fn render(
        &mut self,
        status: &ToolUseStatus,
        window: &mut Window,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement;
}

#[derive(Clone)]
pub struct AnyToolCard {
    entity: gpui::AnyEntity,
    render: fn(
        entity: gpui::AnyEntity,
        status: &ToolUseStatus,
        window: &mut Window,
        workspace: WeakEntity<Workspace>,
        cx: &mut App,
    ) -> AnyElement,
}

impl<T: ToolCard> From<Entity<T>> for AnyToolCard {
    fn from(entity: Entity<T>) -> Self {
        fn downcast_render<T: ToolCard>(
            entity: gpui::AnyEntity,
            status: &ToolUseStatus,
            window: &mut Window,
            workspace: WeakEntity<Workspace>,
            cx: &mut App,
        ) -> AnyElement {
            let entity = entity.downcast::<T>().unwrap();
            entity.update(cx, |entity, cx| {
                entity
                    .render(status, window, workspace, cx)
                    .into_any_element()
            })
        }

        Self {
            entity: entity.into(),
            render: downcast_render::<T>,
        }
    }
}

impl AnyToolCard {
    pub fn render(
        &self,
        status: &ToolUseStatus,
        window: &mut Window,
        workspace: WeakEntity<Workspace>,
        cx: &mut App,
    ) -> AnyElement {
        (self.render)(self.entity.clone(), status, window, workspace, cx)
    }
}

impl From<Task<Result<ToolResultOutput>>> for ToolResult {
    /// Convert from a task to a ToolResult with no card
    fn from(output: Task<Result<ToolResultOutput>>) -> Self {
        Self { output, card: None }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum ToolSource {
    /// A native tool built-in to Zed.
    Native,
    /// A tool provided by a context server.
    ContextServer { id: SharedString },
}

/// A tool that can be used by a language model.
pub trait Tool: 'static + Send + Sync {
    /// Returns the name of the tool.
    fn name(&self) -> String;

    /// Returns the description of the tool.
    fn description(&self) -> String;

    /// Returns the icon for the tool.
    fn icon(&self) -> IconName;

    /// Returns the source of the tool.
    fn source(&self) -> ToolSource {
        ToolSource::Native
    }

    /// Returns true iff the tool needs the users's confirmation
    /// before having permission to run.
    fn needs_confirmation(&self, input: &serde_json::Value, cx: &App) -> bool;

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(&self, _: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        Ok(serde_json::Value::Object(serde_json::Map::default()))
    }

    /// Returns markdown to be displayed in the UI for this tool.
    fn ui_text(&self, input: &serde_json::Value) -> String;

    /// Returns markdown to be displayed in the UI for this tool, while the input JSON is still streaming
    /// (so information may be missing).
    fn still_streaming_ui_text(&self, input: &serde_json::Value) -> String {
        self.ui_text(input)
    }

    /// Runs the tool with the provided input.
    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        model: Arc<dyn LanguageModel>,
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult;

    fn deserialize_card(
        self: Arc<Self>,
        _output: serde_json::Value,
        _project: Entity<Project>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<AnyToolCard> {
        None
    }
}

impl Debug for dyn Tool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tool").field("name", &self.name()).finish()
    }
}
