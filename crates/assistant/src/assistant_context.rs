use editor::{Editor, MultiBuffer};
use gpui::{AnyElement, AppContext, FontWeight, Model, View};
use language::Buffer;
use workspace::Workspace;

use std::path::PathBuf;
use ui::{prelude::*, Checkbox};

use crate::{LanguageModelRequestMessage, Role};

/// This trait allows the assistant panel (or any LLM utility) to
/// * create a text based representation for large language model consumption
/// * render a mini view of the item for humans

pub trait AssistantContext {
    fn text_for_llm(&self, cx: &AppContext) -> String;
}

#[derive(Default)]
pub struct InlineContext {
    active_buffer: Option<Model<MultiBuffer>>,
    active_buffer_enabled: bool,
}

// In the future we'll need some way to group on the file context, project diagnostics, etc.
// For now, we'll just render the file context as is.

impl InlineContext {
    pub fn new() -> Self {
        Self {
            active_buffer: None,
            active_buffer_enabled: true,
        }
    }

    pub fn set_active_buffer(&mut self, buffer: Option<Model<MultiBuffer>>) {
        self.active_buffer = buffer;
    }

    pub fn active_buffer(&self) -> Option<&Model<MultiBuffer>> {
        self.active_buffer.as_ref()
    }

    pub fn active_buffer_enabled(&self) -> bool {
        self.active_buffer_enabled
    }

    pub fn set_active_buffer_enabled(&mut self, enabled: bool) {
        self.active_buffer_enabled = enabled;
    }

    pub fn message(&self, cx: &AppContext) -> Option<LanguageModelRequestMessage> {
        if !self.active_buffer_enabled {
            return None;
        }

        let active_buffer = self.active_buffer.as_ref()?;
        let buffer = active_buffer.read(cx);

        if let Some(singleton) = buffer.as_singleton() {
            let singleton = singleton.read(cx);

            let filename = singleton
                .file()
                .map(|file| file.path().to_string_lossy())
                .unwrap_or("Untitled".into());

            let text = singleton.text();

            let language = singleton
                .language()
                .map(|l| {
                    // TODO: Find out the markdown code fence block name. In some cases the name
                    // doesn't match the code fence block name, which the model will later copy.
                    // For example, "Shell Script" is a language name whereas the code fence block
                    // name is "shell", "bash", or "sh".
                    let name = l.name();
                    name.to_string()
                })
                .unwrap_or_default();

            let markdown =
                format!("User's active file `{filename}`:\n\n```{language}\n{text}```\n\n");

            return Some(LanguageModelRequestMessage {
                role: Role::System,
                content: markdown,
            });
        }

        return None;
    }
}
