pub mod active_file;

use anyhow::{anyhow, Result};
use assistant_tooling::{LanguageModelAttachment, ProjectContext, ToolOutput};
use editor::Editor;
use gpui::{Render, Task, View, WeakModel, WeakView};
use language::Buffer;
use project::ProjectPath;
use ui::{prelude::*, ButtonLike, Tooltip, WindowContext};
use util::maybe;
use workspace::Workspace;

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
    fn generate(&self, project: &mut ProjectContext, cx: &mut WindowContext) -> String {
        if let Ok(result) = &self.output {
            if let Some(path) = &result.path {
                project.add_file(path.clone());
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

impl LanguageModelAttachment for ActiveEditorAttachmentTool {
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
