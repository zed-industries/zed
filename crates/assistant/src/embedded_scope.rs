use editor::MultiBuffer;
use gpui::{AppContext, Model, ModelContext, Subscription};

use crate::{assistant_panel::Conversation, LanguageModelRequestMessage, Role};

#[derive(Default)]
pub struct EmbeddedScope {
    active_buffer: Option<Model<MultiBuffer>>,
    active_buffer_enabled: bool,
    active_buffer_subscription: Option<Subscription>,
}

impl EmbeddedScope {
    pub fn new() -> Self {
        Self {
            active_buffer: None,
            active_buffer_enabled: true,
            active_buffer_subscription: None,
        }
    }

    pub fn set_active_buffer(
        &mut self,
        buffer: Option<Model<MultiBuffer>>,
        cx: &mut ModelContext<Conversation>,
    ) {
        self.active_buffer_subscription.take();

        if let Some(active_buffer) = buffer.clone() {
            self.active_buffer_subscription =
                Some(cx.subscribe(&active_buffer, |conversation, _, e, cx| {
                    if let multi_buffer::Event::Edited { .. } = e {
                        conversation.count_remaining_tokens(cx)
                    }
                }));
        }

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

    /// Provide a message for the language model based on the active buffer.
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
                    let name = l.code_fence_block_name();
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

        None
    }
}
