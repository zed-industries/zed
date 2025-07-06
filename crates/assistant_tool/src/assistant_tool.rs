mod action_log;
pub mod outline;
mod tool_registry;
mod tool_schema;
mod tool_working_set;

use std::{fmt, fmt::Debug, fmt::Formatter, ops::Deref, sync::Arc};

use anyhow::Result;
use gpui::{
    AnyElement, AnyWindowHandle, App, Context, Entity, IntoElement, SharedString, Task, WeakEntity,
    Window,
};
use icons::IconName;
use language_model::{
    LanguageModel, LanguageModelImage, LanguageModelRequest, LanguageModelToolSchemaFormat,
};
use project::Project;
use serde::de::DeserializeOwned;
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
    pub content: ToolResultContent,
    pub output: Option<serde_json::Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ToolResultContent {
    Text(String),
    Image(LanguageModelImage),
}

impl ToolResultContent {
    pub fn len(&self) -> usize {
        match self {
            ToolResultContent::Text(str) => str.len(),
            ToolResultContent::Image(image) => image.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ToolResultContent::Text(str) => str.is_empty(),
            ToolResultContent::Image(image) => image.is_empty(),
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            ToolResultContent::Text(str) => Some(str),
            ToolResultContent::Image(_) => None,
        }
    }
}

impl From<String> for ToolResultOutput {
    fn from(value: String) -> Self {
        ToolResultOutput {
            content: ToolResultContent::Text(value),
            output: None,
        }
    }
}

impl Deref for ToolResultOutput {
    type Target = ToolResultContent;

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
pub trait Tool: Send + Sync + 'static {
    /// The input type that is accepted by the tool.
    type Input: DeserializeOwned;

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

    /// Returns true if the tool needs the users's confirmation
    /// before having permission to run.
    fn needs_confirmation(&self, input: &Self::Input, cx: &App) -> bool;

    /// Returns true if the tool may perform edits.
    fn may_perform_edits(&self) -> bool;

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(&self, _: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        Ok(serde_json::Value::Object(serde_json::Map::default()))
    }

    /// Returns markdown to be displayed in the UI for this tool.
    fn ui_text(&self, input: &Self::Input) -> String;

    /// Returns markdown to be displayed in the UI for this tool, while the input JSON is still streaming
    /// (so information may be missing).
    fn still_streaming_ui_text(&self, input: &Self::Input) -> String {
        self.ui_text(input)
    }

    /// Runs the tool with the provided input.
    fn run(
        self: Arc<Self>,
        input: Self::Input,
        request: Arc<LanguageModelRequest>,
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

#[derive(Clone)]
pub struct AnyTool {
    inner: Arc<dyn ErasedTool>,
}

/// Copy of `Tool` where the Input type is erased.
trait ErasedTool: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn icon(&self) -> IconName;
    fn source(&self) -> ToolSource;
    fn may_perform_edits(&self) -> bool;
    fn needs_confirmation(&self, input: &serde_json::Value, cx: &App) -> bool;
    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value>;
    fn ui_text(&self, input: &serde_json::Value) -> String;
    fn still_streaming_ui_text(&self, input: &serde_json::Value) -> String;
    fn run(
        &self,
        input: serde_json::Value,
        request: Arc<LanguageModelRequest>,
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        model: Arc<dyn LanguageModel>,
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult;
    fn deserialize_card(
        &self,
        output: serde_json::Value,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyToolCard>;
}

struct ErasedToolWrapper<T: Tool> {
    tool: Arc<T>,
}

impl<T: Tool> ErasedTool for ErasedToolWrapper<T> {
    fn name(&self) -> String {
        self.tool.name()
    }

    fn description(&self) -> String {
        self.tool.description()
    }

    fn icon(&self) -> IconName {
        self.tool.icon()
    }

    fn source(&self) -> ToolSource {
        self.tool.source()
    }

    fn may_perform_edits(&self) -> bool {
        self.tool.may_perform_edits()
    }

    fn needs_confirmation(&self, input: &serde_json::Value, cx: &App) -> bool {
        match serde_json::from_value::<T::Input>(input.clone()) {
            Ok(parsed_input) => self.tool.needs_confirmation(&parsed_input, cx),
            Err(_) => true,
        }
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        self.tool.input_schema(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<T::Input>(input.clone()) {
            Ok(parsed_input) => self.tool.ui_text(&parsed_input),
            Err(_) => "Invalid input".to_string(),
        }
    }

    fn still_streaming_ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<T::Input>(input.clone()) {
            Ok(parsed_input) => self.tool.still_streaming_ui_text(&parsed_input),
            Err(_) => "Invalid input".to_string(),
        }
    }

    fn run(
        &self,
        input: serde_json::Value,
        request: Arc<LanguageModelRequest>,
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        model: Arc<dyn LanguageModel>,
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        match serde_json::from_value::<T::Input>(input) {
            Ok(parsed_input) => self.tool.clone().run(
                parsed_input,
                request,
                project,
                action_log,
                model,
                window,
                cx,
            ),
            Err(err) => ToolResult::from(Task::ready(Err(err.into()))),
        }
    }

    fn deserialize_card(
        &self,
        output: serde_json::Value,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyToolCard> {
        self.tool
            .clone()
            .deserialize_card(output, project, window, cx)
    }
}

impl<T: Tool> From<Arc<T>> for AnyTool {
    fn from(tool: Arc<T>) -> Self {
        Self {
            inner: Arc::new(ErasedToolWrapper { tool }),
        }
    }
}

impl AnyTool {
    pub fn name(&self) -> String {
        self.inner.name()
    }

    pub fn description(&self) -> String {
        self.inner.description()
    }

    pub fn icon(&self) -> IconName {
        self.inner.icon()
    }

    pub fn source(&self) -> ToolSource {
        self.inner.source()
    }

    pub fn may_perform_edits(&self) -> bool {
        self.inner.may_perform_edits()
    }

    pub fn needs_confirmation(&self, input: &serde_json::Value, cx: &App) -> bool {
        self.inner.needs_confirmation(input, cx)
    }

    pub fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        self.inner.input_schema(format)
    }

    pub fn ui_text(&self, input: &serde_json::Value) -> String {
        self.inner.ui_text(input)
    }

    pub fn still_streaming_ui_text(&self, input: &serde_json::Value) -> String {
        self.inner.still_streaming_ui_text(input)
    }

    pub fn run(
        &self,
        input: serde_json::Value,
        request: Arc<LanguageModelRequest>,
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        model: Arc<dyn LanguageModel>,
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        self.inner
            .run(input, request, project, action_log, model, window, cx)
    }

    pub fn deserialize_card(
        &self,
        output: serde_json::Value,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyToolCard> {
        self.inner.deserialize_card(output, project, window, cx)
    }
}

impl Debug for AnyTool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tool").field("name", &self.name()).finish()
    }
}
