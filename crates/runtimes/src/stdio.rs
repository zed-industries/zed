use gpui::AnyElement;
use ui::{IntoElement, SharedString};

#[derive(Clone, Debug)]
pub struct TerminalOutput {
    buffer: String,
}

impl TerminalOutput {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn append_text(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    pub fn num_lines(&self) -> u8 {
        self.buffer.lines().count() as u8
    }

    pub fn render(&self) -> AnyElement {
        SharedString::from(self.buffer.trim_end().to_string()).into_any_element()
    }
}
