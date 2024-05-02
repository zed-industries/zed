use std::{
    any::TypeId,
    sync::{mpsc, Arc},
};

use crate::completion_provider::CompletionMessage;
use anyhow::{anyhow, Result};
use collections::HashMap;
use editor::{Editor, MultiBuffer};
use gpui::{
    AnyView, AppContext, EventEmitter, Model, ModelContext, Render, Subscription, Task, View,
    WeakView,
};
use ui::{prelude::*, WindowContext};
use util::maybe;
use workspace::Workspace;

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

struct ActiveEditorAttachment {
    filename: Arc<str>,
    language: Arc<str>,
    text: Arc<str>,
}

struct FileAttachmentView {
    output: Result<ActiveEditorAttachment>,
}

impl Render for FileAttachmentView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        match &self.output {
            Ok(attachment) => {
                let filename = attachment.filename.clone();
                div().child(SharedString::from(filename))
            }
            Err(err) => div().child(err.to_string()),
        }
    }
}

struct ActiveEditorAttachmentTool {
    workspace: WeakView<Workspace>,
}

impl ActiveEditorAttachmentTool {
    pub fn new(workspace: WeakView<Workspace>, _cx: &mut WindowContext) -> Self {
        Self { workspace }
    }
}

impl AttachmentTool for ActiveEditorAttachmentTool {
    type Output = ActiveEditorAttachment;
    type View = FileAttachmentView;

    fn run(&self, cx: &mut WindowContext) -> Task<Result<ActiveEditorAttachment>> {
        Task::ready(maybe!({
            let active_buffer = self
                .workspace
                .update(cx, |workspace, cx| {
                    workspace
                        .active_item(cx)
                        .and_then(|item| Some(item.act_as::<Editor>(cx)?.read(cx).buffer().clone()))
                })?
                .ok_or_else(|| anyhow!("no active buffer"))?;

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

                return Ok(ActiveEditorAttachment {
                    filename: filename.into(),
                    language: language.into(),
                    text: text.into(),
                });
            }

            Err(anyhow!("no active buffer"))
        }))
    }

    fn message(output: &Result<Self::Output>) -> Option<CompletionMessage> {
        let output = output.as_ref().ok()?;

        let filename = &output.filename;
        let language = &output.language;
        let text = &output.text;

        let markdown_content =
            format!("User's active file `{filename}`:\n\n```{language}\n{text}```\n\n");

        return Some(CompletionMessage::System {
            content: markdown_content,
        });
    }

    fn view(output: Result<Self::Output>, cx: &mut WindowContext) -> View<Self::View> {
        cx.new_view(|_cx| FileAttachmentView { output })
    }
}
