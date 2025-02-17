use gpui::{
    Context, DismissEvent, EventEmitter, IntoElement, Render, SemanticVersion, WeakEntity, Window,
};
use release_channel::ReleaseChannel;
use ui::prelude::*;
use workspace::{notifications::simple_message_notification::MessageNotification, Workspace};

pub struct UpdateNotification {
    version: SemanticVersion,
    workspace: WeakEntity<Workspace>,
}

impl EventEmitter<DismissEvent> for UpdateNotification {}

impl Render for UpdateNotification {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app_name = ReleaseChannel::global(cx).display_name();
        let version = self.version.clone();
        let workspace = self.workspace.clone();

        cx.new(|_cx| {
            MessageNotification::new(format!("Updated to {app_name} {}", version))
                .primary_message("View Release Notes")
                .primary_icon(IconName::ArrowUpRight)
                .primary_on_click(move |window, cx| {
                    if let Some(workspace) = workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            crate::view_release_notes_locally(workspace, window, cx);
                        })
                    }
                    cx.emit(DismissEvent);
                })
        })
    }
}

impl UpdateNotification {
    pub fn new(version: SemanticVersion, workspace: WeakEntity<Workspace>) -> Self {
        Self { version, workspace }
    }

    pub fn show(version: SemanticVersion, workspace: &mut Workspace, cx: &mut Context<Workspace>) {
        let notification = Self::new(version, workspace.weak_handle());
        workspace.show_notification(
            workspace::notifications::NotificationId::unique::<Self>(),
            cx,
            |cx| cx.new(|_| notification),
        );
    }
}
