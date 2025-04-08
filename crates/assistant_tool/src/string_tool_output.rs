use std::sync::Arc;

use gpui::SharedString;

use crate::ToolOutput;

/// A simple implementation of ToolOutput that wraps a string.
#[derive(Debug)]
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