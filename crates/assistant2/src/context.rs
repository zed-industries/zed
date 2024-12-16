use gpui::SharedString;
use language_model::{LanguageModelRequestMessage, MessageContent};
use serde::{Deserialize, Serialize};
use util::post_inc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct ContextId(pub(crate) usize);

impl ContextId {
    pub fn post_inc(&mut self) -> Self {
        Self(post_inc(&mut self.0))
    }
}

/// Some context attached to a message in a thread.
#[derive(Debug, Clone)]
pub struct Context {
    pub id: ContextId,
    pub name: SharedString,
    pub kind: ContextKind,
    pub text: SharedString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextKind {
    File,
    FetchedUrl,
    Thread,
}

pub fn attach_context_to_message(
    message: &mut LanguageModelRequestMessage,
    context: impl IntoIterator<Item = Context>,
) {
    let mut file_context = String::new();
    let mut fetch_context = String::new();
    let mut thread_context = String::new();

    for context in context.into_iter() {
        match context.kind {
            ContextKind::File => {
                file_context.push_str(&context.text);
                file_context.push('\n');
            }
            ContextKind::FetchedUrl => {
                fetch_context.push_str(&context.name);
                fetch_context.push('\n');
                fetch_context.push_str(&context.text);
                fetch_context.push('\n');
            }
            ContextKind::Thread => {
                thread_context.push_str(&context.name);
                thread_context.push('\n');
                thread_context.push_str(&context.text);
                thread_context.push('\n');
            }
        }
    }

    let mut context_text = String::new();
    if !file_context.is_empty() {
        context_text.push_str("The following files are available:\n");
        context_text.push_str(&file_context);
    }

    if !fetch_context.is_empty() {
        context_text.push_str("The following fetched results are available\n");
        context_text.push_str(&fetch_context);
    }

    if !thread_context.is_empty() {
        context_text.push_str("The following previous conversation threads are available\n");
        context_text.push_str(&thread_context);
    }

    message.content.push(MessageContent::Text(context_text));
}
