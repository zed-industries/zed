use gpui::{AnyElement, App, EntityId, SharedString, Window};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Trait for tool output objects that can be provided to the language model
pub trait ToolOutput: Send + Sync {
    /// Returns a string that will be given to the model
    /// as the tool output.
    fn response_for_model(&self) -> SharedString;

    /// Returns a custom UI element to render the tool's output.
    /// Returns None by default to indicate that rendering has not yet been
    /// implemented for this tool, and the caller should do some default rendering.
    fn render(&self, _window: &mut Window, _cx: &App) -> Option<AnyElement> {
        None
    }
}

/// Implementation of ToolOutput for SharedString
impl ToolOutput for SharedString {
    fn response_for_model(&self) -> SharedString {
        self.clone()
    }
}

/// A simple implementation of ToolOutput that wraps a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringToolOutput(SharedString);

impl StringToolOutput {
    /// Create a new StringToolOutput from a string-like value
    pub fn new(value: impl Into<SharedString>) -> Arc<dyn ToolOutput> {
        Arc::new(Self(value.into()))
    }
}

impl ToolOutput for StringToolOutput {
    fn response_for_model(&self) -> SharedString {
        self.0.clone()
    }
}
