mod tool_registry;
mod tool_working_set;

use std::sync::Arc;

use anyhow::Result;
use gpui::AnyElement;
use gpui::IntoElement;
use gpui::{App, Task, WeakEntity, Window};
use language::Language;
use ui::div;
use ui::Label;
use ui::LabelCommon;
use ui::LabelSize;
use ui::ParentElement;
use ui::SharedString;
use workspace::Workspace;

pub use crate::tool_registry::*;
pub use crate::tool_working_set::*;

pub fn init(cx: &mut App) {
    ToolRegistry::default_global(cx);
}

/// A tool that can be used by a language model.
pub trait Tool: 'static + Send + Sync {
    /// Returns the name of the tool.
    fn name(&self) -> String;

    /// Returns the description of the tool.
    fn description(&self) -> String;

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(&self) -> serde_json::Value {
        serde_json::Value::Object(serde_json::Map::default())
    }

    /// Runs the tool with the provided input.
    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        thread_id: Arc<str>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<String>>;

    /// Renders the tool's input when the user expands it.
    fn render_input(
        self: Arc<Self>,
        input: serde_json::Value,
        _lua_language: Option<Arc<Language>>,
        _cx: &mut App,
    ) -> AnyElement {
        default_render_input(input)
    }

    /// Renders the tool's output when the user expands it.
    fn render_output(self: Arc<Self>, output: SharedString, _cx: &mut App) -> AnyElement {
        default_render_output(output)
    }

    /// Renders the tool's error message when the user expands it.
    fn render_error(self: Arc<Self>, err: SharedString, _cx: &mut App) -> AnyElement {
        default_render_error(err)
    }
}

pub fn default_render_input(input: serde_json::Value) -> AnyElement {
    div()
        .child(Label::new("Input:").size(LabelSize::Small))
        .child(Label::new(
            serde_json::to_string_pretty(&input).unwrap_or_default(),
        ))
        .into_any_element()
}

pub fn default_render_output(output: SharedString) -> AnyElement {
    div()
        .child(Label::new("Result:").size(LabelSize::Small))
        .child(Label::new(output))
        .into_any_element()
}

pub fn default_render_error(err: SharedString) -> AnyElement {
    div()
        .child(Label::new("Error:").size(LabelSize::Small))
        .child(Label::new(err))
        .into_any_element()
}
