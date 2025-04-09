use gpui::{AnyElement, App, SharedString, Window};
use serde::{Deserialize, Serialize};

/// An enum that represents different types of tool outputs that can be provided to the language model
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolOutput {
    /// A simple string output
    String(StringToolOutput),
    // Add other tool output types here as variants
}

impl ToolOutput {
    /// Returns a string that will be given to the model
    /// as the tool output.
    pub fn response_for_model(&self) -> SharedString {
        match self {
            ToolOutput::String(output) => output.0.clone(),
            // Handle other variants here
        }
    }

    /// Returns a custom UI element to render the tool's output.
    /// Returns None by default to indicate that rendering has not yet been
    /// implemented for this tool, and the caller should do some default rendering.
    pub fn render(&self, _window: &mut Window, _cx: &App) -> Option<AnyElement> {
        None
    }
}

/// A simple implementation of ToolOutput that wraps a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringToolOutput(pub SharedString);

impl StringToolOutput {
    /// Create a new StringToolOutput from a string-like value
    pub fn new(value: impl Into<SharedString>) -> ToolOutput {
        ToolOutput::String(Self(value.into()))
    }
}