use crate::attachments::{ActiveEditorAttachmentTool, UserAttachmentStore};
use editor::Editor;
use gpui::{prelude::*, Subscription, View};
use std::sync::Arc;
use ui::{prelude::*, ButtonLike, Color, Icon, IconName, Tooltip};
use workspace::Workspace;

#[derive(Clone)]
enum Status {
    ActiveFile(String),
    #[allow(dead_code)]
    NoFile,
}

pub struct ActiveFileButton {
    attachment_store: Arc<UserAttachmentStore>,
    status: Status,
    #[allow(dead_code)]
    workspace_subscription: Subscription,
}

impl ActiveFileButton {
    pub fn new(
        attachment_store: Arc<UserAttachmentStore>,
        workspace: View<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let workspace_subscription = cx.subscribe(&workspace, Self::handle_workspace_event);

        cx.defer(move |this, cx| this.update_active_buffer(workspace.clone(), cx));

        Self {
            attachment_store,
            status: Status::NoFile,
            workspace_subscription,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.attachment_store
            .set_attachment_tool_enabled::<ActiveEditorAttachmentTool>(enabled);
    }

    pub fn update_active_buffer(&mut self, workspace: View<Workspace>, cx: &mut ViewContext<Self>) {
        let active_buffer = workspace
            .read(cx)
            .active_item(cx)
            .and_then(|item| Some(item.act_as::<Editor>(cx)?.read(cx).buffer().clone()));

        if let Some(buffer) = active_buffer {
            let buffer = buffer.read(cx);

            if let Some(singleton) = buffer.as_singleton() {
                let singleton = singleton.read(cx);

                let filename: String = singleton
                    .file()
                    .map(|file| file.path().to_string_lossy())
                    .unwrap_or("Untitled".into())
                    .into();

                self.status = Status::ActiveFile(filename);
            }
        }
    }

    fn handle_workspace_event(
        &mut self,
        workspace: View<Workspace>,
        event: &workspace::Event,
        cx: &mut ViewContext<Self>,
    ) {
        if let workspace::Event::ActiveItemChanged = event {
            self.update_active_buffer(workspace, cx);
        }
    }
}

impl Render for ActiveFileButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_enabled = self
            .attachment_store
            .is_attachment_tool_enabled::<ActiveEditorAttachmentTool>();

        let icon = if is_enabled {
            Icon::new(IconName::File)
                .size(IconSize::XSmall)
                .color(Color::Default)
        } else {
            Icon::new(IconName::File)
                .size(IconSize::XSmall)
                .color(Color::Disabled)
        };

        let indicator = None;

        let status = self.status.clone();

        ButtonLike::new("active-file-button")
            .child(
                ui::IconWithIndicator::new(icon, indicator)
                    .indicator_border_color(Some(gpui::transparent_black())),
            )
            .tooltip({
                move |cx| {
                    let status = status.clone();
                    let (tooltip, meta) = match (is_enabled, status) {
                        (false, _) => (
                            "Active file disabled".to_string(),
                            Some("Click to enable".to_string()),
                        ),
                        (true, Status::ActiveFile(filename)) => (
                            format!("Active file {filename} enabled"),
                            Some("Click to disable".to_string()),
                        ),
                        (true, Status::NoFile) => {
                            ("No file active for conversation".to_string(), None)
                        }
                    };

                    if let Some(meta) = meta {
                        Tooltip::with_meta(tooltip, None, meta, cx)
                    } else {
                        Tooltip::text(tooltip, cx)
                    }
                }
            })
            .on_click(cx.listener(move |this, _, cx| {
                this.set_enabled(!is_enabled);
                cx.notify();
            }))
    }
}
