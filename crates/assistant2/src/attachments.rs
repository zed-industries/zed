use std::{
    any::TypeId,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};

use anyhow::{anyhow, Result};
use collections::HashMap;
use editor::Editor;
use futures::future::join_all;
use gpui::{AnyView, Render, Task, View, WeakView};
use ui::{prelude::*, ButtonLike, Tooltip, WindowContext};
use util::{maybe, ResultExt};
use workspace::Workspace;

/// A collected attachment from running an attachment tool
pub struct UserAttachment {
    pub message: Option<String>,
    pub view: AnyView,
}

pub struct UserAttachmentStore {
    attachment_tools: HashMap<TypeId, DynamicAttachment>,
}

/// Internal representation of an attachment tool to allow us to treat them dynamically
struct DynamicAttachment {
    enabled: AtomicBool,
    call: Box<dyn Fn(&mut WindowContext) -> Task<Result<UserAttachment>>>,
}

impl UserAttachmentStore {
    pub fn new() -> Self {
        Self {
            attachment_tools: HashMap::default(),
        }
    }

    pub fn register<A: AttachmentTool + 'static>(&mut self, attachment: A) {
        let call = Box::new(move |cx: &mut WindowContext| {
            let result = attachment.run(cx);

            cx.spawn(move |mut cx| async move {
                let result: Result<A::Output> = result.await;
                let message = A::format(&result);
                let view = cx.update(|cx| A::view(result, cx))?;

                Ok(UserAttachment {
                    message,
                    view: view.into(),
                })
            })
        });

        self.attachment_tools.insert(
            TypeId::of::<A>(),
            DynamicAttachment {
                call,
                enabled: AtomicBool::new(true),
            },
        );
    }

    pub fn set_attachment_tool_enabled<A: AttachmentTool + 'static>(&self, is_enabled: bool) {
        if let Some(attachment) = self.attachment_tools.get(&TypeId::of::<A>()) {
            attachment.enabled.store(is_enabled, SeqCst);
        }
    }

    pub fn is_attachment_tool_enabled<A: AttachmentTool + 'static>(&self) -> bool {
        if let Some(attachment) = self.attachment_tools.get(&TypeId::of::<A>()) {
            attachment.enabled.load(SeqCst)
        } else {
            false
        }
    }

    pub fn call<A: AttachmentTool + 'static>(
        &self,
        cx: &mut WindowContext,
    ) -> Task<Result<UserAttachment>> {
        let Some(attachment) = self.attachment_tools.get(&TypeId::of::<A>()) else {
            return Task::ready(Err(anyhow!("no attachment tool")));
        };

        (attachment.call)(cx)
    }

    pub fn call_all_attachment_tools(
        self: Arc<Self>,
        cx: &mut WindowContext<'_>,
    ) -> Task<Result<Vec<UserAttachment>>> {
        let this = self.clone();
        cx.spawn(|mut cx| async move {
            let attachment_tasks = cx.update(|cx| {
                let mut tasks = Vec::new();
                for attachment in this
                    .attachment_tools
                    .values()
                    .filter(|attachment| attachment.enabled.load(SeqCst))
                {
                    tasks.push((attachment.call)(cx))
                }

                tasks
            })?;

            let attachments = join_all(attachment_tasks.into_iter()).await;

            Ok(attachments
                .into_iter()
                .filter_map(|attachment| attachment.log_err())
                .collect())
        })
    }
}

///
pub trait AttachmentTool {
    type Output: 'static;
    type View: Render;

    fn run(&self, cx: &mut WindowContext) -> Task<Result<Self::Output>>;

    fn format(output: &Result<Self::Output>) -> Option<String>;

    fn view(output: Result<Self::Output>, cx: &mut WindowContext) -> View<Self::View>;
}

pub struct ActiveEditorAttachment {
    filename: Arc<str>,
    language: Arc<str>,
    text: Arc<str>,
}

pub struct FileAttachmentView {
    output: Result<ActiveEditorAttachment>,
}

impl Render for FileAttachmentView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        match &self.output {
            Ok(attachment) => {
                let filename = attachment.filename.clone();

                // todo!(): make the button link to the actual file to open
                ButtonLike::new("file-attachment")
                    .child(
                        h_flex()
                            .gap_1()
                            .bg(cx.theme().colors().editor_background)
                            .rounded_md()
                            .child(ui::Icon::new(IconName::File))
                            .child(filename.to_string()),
                    )
                    .tooltip({
                        move |cx| Tooltip::with_meta("File Attached", None, filename.clone(), cx)
                    })
                    .into_any_element()
            }
            Err(err) => div().child(err.to_string()).into_any_element(),
        }
    }
}

pub struct ActiveEditorAttachmentTool {
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

    fn format(output: &Result<Self::Output>) -> Option<String> {
        let output = output.as_ref().ok()?;

        let filename = &output.filename;
        let language = &output.language;
        let text = &output.text;

        Some(format!(
            "User's active file `{filename}`:\n\n```{language}\n{text}```\n\n"
        ))
    }

    fn view(output: Result<Self::Output>, cx: &mut WindowContext) -> View<Self::View> {
        cx.new_view(|_cx| FileAttachmentView { output })
    }
}
