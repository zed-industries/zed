use crate::mention_set::Mention;
use gpui::{AppContext as _, Entity, Task};
use language_model::{LanguageModelImage, LanguageModelRequestMessage, MessageContent};
use ui::App;
use util::ResultExt as _;

use crate::mention_set::MentionSet;

#[derive(Debug, Clone, Default)]
pub struct LoadedContext {
    pub text: String,
    pub images: Vec<LanguageModelImage>,
}

impl LoadedContext {
    pub fn add_to_request_message(&self, request_message: &mut LanguageModelRequestMessage) {
        if !self.text.is_empty() {
            request_message
                .content
                .push(MessageContent::Text(self.text.to_string()));
        }

        if !self.images.is_empty() {
            // Some providers only support image parts after an initial text part
            if request_message.content.is_empty() {
                request_message
                    .content
                    .push(MessageContent::Text("Images attached by user:".to_string()));
            }

            for image in &self.images {
                request_message
                    .content
                    .push(MessageContent::Image(image.clone()))
            }
        }
    }
}

/// Loads and formats a collection of contexts.
pub fn load_context(mention_set: &Entity<MentionSet>, cx: &mut App) -> Task<Option<LoadedContext>> {
    let task = mention_set.update(cx, |mention_set, cx| mention_set.contents(true, cx));
    cx.background_spawn(async move {
        let mentions = task.await.log_err()?;
        let mut loaded_context = LoadedContext::default();
        loaded_context
            .text
            .push_str("The following items were attached by the user.\n");
        for (_, (_, mention)) in mentions {
            match mention {
                Mention::Text { content, .. } => {
                    loaded_context.text.push_str(&content);
                }
                Mention::Image(mention_image) => loaded_context.images.push(LanguageModelImage {
                    source: mention_image.data,
                    ..LanguageModelImage::empty()
                }),
                Mention::Link => {}
            }
        }
        Some(loaded_context)
    })
}
