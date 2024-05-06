use std::{
    any::TypeId,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};

use anyhow::{anyhow, Result};
use assistant_tooling::{AssistantContext, ToolOutput};
use collections::HashMap;
use editor::Editor;
use futures::future::join_all;
use gpui::{AnyView, Render, Task, View, WeakModel, WeakView};
use language::Buffer;
use project::ProjectPath;
use ui::{prelude::*, ButtonLike, Tooltip, WindowContext};
use util::{maybe, ResultExt};
use workspace::Workspace;

/// A collected attachment from running an attachment tool
pub struct UserAttachment {
    pub view: AnyView,
    generate_fn: fn(AnyView, &mut AssistantContext, cx: &mut WindowContext) -> String,
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
                let view = cx.update(|cx| A::view(result, cx))?;

                Ok(UserAttachment {
                    view: view.into(),
                    generate_fn: generate::<A>,
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
        return;

        fn generate<T: AttachmentTool>(
            view: AnyView,
            output: &mut AssistantContext,
            cx: &mut WindowContext,
        ) -> String {
            view.downcast::<T::View>()
                .unwrap()
                .update(cx, |view, cx| T::View::generate(view, output, cx))
        }
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

pub trait AttachmentTool {
    type Output: 'static;
    type View: Render + ToolOutput;

    fn run(&self, cx: &mut WindowContext) -> Task<Result<Self::Output>>;

    fn view(output: Result<Self::Output>, cx: &mut WindowContext) -> View<Self::View>;
}

impl UserAttachment {
    pub fn generate(
        &self,
        output: &mut AssistantContext,
        cx: &mut WindowContext,
    ) -> Option<String> {
        let result = (self.generate_fn)(self.view.clone(), output, cx);
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

pub struct ActiveEditorAttachment {
    buffer: WeakModel<Buffer>,
    path: Option<ProjectPath>,
}

pub struct FileAttachmentView {
    output: Result<ActiveEditorAttachment>,
}

impl Render for FileAttachmentView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        match &self.output {
            Ok(attachment) => {
                let filename: SharedString = attachment
                    .path
                    .as_ref()
                    .and_then(|p| p.path.file_name()?.to_str())
                    .unwrap_or("Untitled")
                    .to_string()
                    .into();

                // todo!(): make the button link to the actual file to open
                ButtonLike::new("file-attachment")
                    .child(
                        h_flex()
                            .gap_1()
                            .bg(cx.theme().colors().editor_background)
                            .rounded_md()
                            .child(ui::Icon::new(IconName::File))
                            .child(filename.clone()),
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

impl ToolOutput for FileAttachmentView {
    fn generate(
        &self,
        output: &mut assistant_tooling::AssistantContext,
        cx: &mut WindowContext,
    ) -> String {
        if let Ok(result) = &self.output {
            if let Some(path) = &result.path {
                output.add_file(path.clone());
                return format!("current file: {}", path.path.display());
            } else if let Some(buffer) = result.buffer.upgrade() {
                return format!("current untitled buffer text:\n{}", buffer.read(cx).text());
            }
        }
        String::new()
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

            if let Some(buffer) = buffer.as_singleton() {
                let path =
                    project::File::from_dyn(buffer.read(cx).file()).map(|file| ProjectPath {
                        worktree_id: file.worktree_id(cx),
                        path: file.path.clone(),
                    });
                return Ok(ActiveEditorAttachment {
                    buffer: buffer.downgrade(),
                    path,
                });
            } else {
                Err(anyhow!("no active buffer"))
            }
        }))
    }

    fn view(output: Result<Self::Output>, cx: &mut WindowContext) -> View<Self::View> {
        cx.new_view(|_cx| FileAttachmentView { output })
    }
}
