use std::any::TypeId;

use crate::completion_provider::CompletionMessage;
use anyhow::{anyhow, Result};
use collections::HashMap;
use editor::MultiBuffer;
use gpui::{AnyView, AppContext, Model, ModelContext, Render, Subscription, Task, View};
use ui::WindowContext;

// Immutable already done attached sometime ago
pub struct UserAttachment {
    message: Option<CompletionMessage>,
    view: AnyView,
}

pub struct UserAttachmentStore {
    attachments: HashMap<TypeId, DynamicAttachment>,
}

struct DynamicAttachment {
    call: Box<dyn Fn(&mut WindowContext) -> Task<Result<UserAttachment>>>,
}

// ToolRegistry had the constraint of a name, but we don't have that here

impl UserAttachmentStore {
    pub fn register<A: AttachmentTool + 'static>(&mut self, attachment: A) {
        let call = Box::new(move |cx: &mut WindowContext| {
            let result = attachment.run(cx);

            cx.spawn(move |mut cx| async move {
                let result = result.await;
                let message = A::message(&result);
                let view = cx.update(|cx| A::view(result, cx))?;

                Ok(UserAttachment {
                    message,
                    view: view.into(),
                })
            })
        });

        self.attachments
            .insert(TypeId::of::<A>(), DynamicAttachment { call });
    }

    pub fn call<A: AttachmentTool + 'static>(
        &self,
        cx: &mut WindowContext,
    ) -> Task<Result<UserAttachment>> {
        let Some(attachment) = self.attachments.get(&TypeId::of::<A>()) else {
            return Task::ready(Err(anyhow!("no attachment tool")));
        };

        (attachment.call)(cx)
    }
}

pub trait AttachmentTool {
    type Output: 'static;
    type View: Render;

    fn run(&self, cx: &mut WindowContext) -> Task<Result<Self::Output>>;

    fn message(output: &Result<Self::Output>) -> Option<CompletionMessage>;

    fn view(output: Result<Self::Output>, cx: &mut WindowContext) -> View<Self::View>;
}

// I envision this to be similar to the tool calls in that we'll have a registry of scopes
// that we can enable/disable. A "run" of the scope will be an immutable instance of a scope run.
// Gosh, maybe it's exactly like the LanguageModelTool. The difference is that these are user initiated.
//
// They'll include the:
// - active buffer
// - project diagnostics
//

// cx.defer(|this, cx| this.update_active_buffer(workspace, cx));
//
// fn update_active_buffer(
//     &mut self,
//     workspace: View<Workspace>,
//     cx: &mut ViewContext<'_, ConversationEditor>,
// ) {
//     let active_buffer = workspace
//         .read(cx)
//         .active_item(cx)
//         .and_then(|item| Some(item.act_as::<Editor>(cx)?.read(cx).buffer().clone()));

//     self.conversation.update(cx, |conversation, cx| {
//         conversation
//             .embedded_scope
//             .set_active_buffer(active_buffer.clone(), cx);

//         conversation.count_remaining_tokens(cx);
//         cx.notify();
//     });
// }

// #[derive(Default)]
// pub struct ActiveBufferContext {
//     active_buffer: Option<Model<MultiBuffer>>,
//     active_buffer_enabled: bool,
//     active_buffer_subscription: Option<Subscription>,
// }

// impl ActiveBufferContext {
//     pub fn new() -> Self {
//         Self {
//             active_buffer: None,
//             active_buffer_enabled: true,
//             active_buffer_subscription: None,
//         }
//     }

//     pub fn set_active_buffer(
//         &mut self,
//         buffer: Option<Model<MultiBuffer>>,
//         cx: &mut WindowContext,
//         // cx: &mut ModelContext<Conversation>,
//     ) {
//         self.active_buffer_subscription.take();

//         // todo!(): This is where we should set up a subscription to the buffer to count tokens

//         self.active_buffer = buffer;
//     }

//     pub fn active_buffer(&self) -> Option<&Model<MultiBuffer>> {
//         self.active_buffer.as_ref()
//     }

//     pub fn active_buffer_enabled(&self) -> bool {
//         self.active_buffer_enabled
//     }

//     pub fn set_active_buffer_enabled(&mut self, enabled: bool) {
//         self.active_buffer_enabled = enabled;
//     }

//     /// Provide a message for the language model based on the active buffer.
//     pub fn message(&self, cx: &AppContext) -> Option<CompletionMessage> {
//         if !self.active_buffer_enabled {
//             return None;
//         }

//         let active_buffer = self.active_buffer.as_ref()?;
//         let buffer = active_buffer.read(cx);

//         if let Some(singleton) = buffer.as_singleton() {
//             let singleton = singleton.read(cx);

//             let filename = singleton
//                 .file()
//                 .map(|file| file.path().to_string_lossy())
//                 .unwrap_or("Untitled".into());

//             let text = singleton.text();

//             let language = singleton
//                 .language()
//                 .map(|l| {
//                     let name = l.code_fence_block_name();
//                     name.to_string()
//                 })
//                 .unwrap_or_default();

//             let markdown =
//                 format!("User's active file `{filename}`:\n\n```{language}\n{text}```\n\n");

//             return Some(CompletionMessage::System { content: markdown });
//         }

//         None
//     }
// }
