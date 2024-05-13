use std::{path::PathBuf, sync::Arc};

use anyhow::{anyhow, Result};
use assistant_tooling::{AttachmentOutput, LanguageModelAttachment, ProjectContext};
use editor::Editor;
use gpui::{Render, Task, View, WeakModel, WeakView};
use language::Buffer;
use project::ProjectPath;
use serde::{Deserialize, Serialize};
use ui::{prelude::*, ButtonLike, Tooltip, WindowContext};
use util::maybe;
use workspace::Workspace;

#[derive(Serialize, Deserialize)]
pub struct ActiveEditorAttachment {
    #[serde(skip)]
    buffer: Option<WeakModel<Buffer>>,
    path: Option<PathBuf>,
}

pub struct FileAttachmentView {
    project_path: Option<ProjectPath>,
    buffer: Option<WeakModel<Buffer>>,
    error: Option<anyhow::Error>,
}

impl Render for FileAttachmentView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if let Some(error) = &self.error {
            return div().child(error.to_string()).into_any_element();
        }

        let filename: SharedString = self
            .project_path
            .as_ref()
            .and_then(|p| p.path.file_name()?.to_str())
            .unwrap_or("Untitled")
            .to_string()
            .into();

        ButtonLike::new("file-attachment")
            .child(
                h_flex()
                    .gap_1()
                    .bg(cx.theme().colors().editor_background)
                    .rounded_md()
                    .child(ui::Icon::new(IconName::File))
                    .child(filename.clone()),
            )
            .tooltip(move |cx| Tooltip::with_meta("File Attached", None, filename.clone(), cx))
            .into_any_element()
    }
}

impl AttachmentOutput for FileAttachmentView {
    fn generate(&self, project: &mut ProjectContext, cx: &mut WindowContext) -> String {
        if let Some(path) = &self.project_path {
            project.add_file(path.clone());
            return format!("current file: {}", path.path.display());
        }

        if let Some(buffer) = self.buffer.as_ref().and_then(|buffer| buffer.upgrade()) {
            return format!("current untitled buffer text:\n{}", buffer.read(cx).text());
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

    fn name(&self) -> Arc<str> {
        "active-editor-attachment".into()
    }

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
                let path = project::File::from_dyn(buffer.read(cx).file())
                    .and_then(|file| file.worktree.read(cx).absolutize(&file.path).ok());
                return Ok(ActiveEditorAttachment {
                    buffer: Some(buffer.downgrade()),
                    path,
                });
            } else {
                Err(anyhow!("no active buffer"))
            }
        }))
    }

    fn view(
        &self,
        output: Result<ActiveEditorAttachment>,
        cx: &mut WindowContext,
    ) -> View<Self::View> {
        let error;
        let project_path;
        let buffer;
        match output {
            Ok(output) => {
                error = None;
                let workspace = self.workspace.upgrade().unwrap();
                let project = workspace.read(cx).project();
                project_path = output
                    .path
                    .and_then(|path| project.read(cx).project_path_for_absolute_path(&path, cx));
                buffer = output.buffer;
            }
            Err(err) => {
                error = Some(err);
                buffer = None;
                project_path = None;
            }
        }
        cx.new_view(|_cx| FileAttachmentView {
            project_path,
            buffer,
            error,
        })
    }
}
