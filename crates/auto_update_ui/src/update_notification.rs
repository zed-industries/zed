use gpui::{
    rems, DismissEvent, EventEmitter, InteractiveElement, IntoElement, ParentElement, Render,
    SemanticVersion, Styled, ViewContext, WeakView,
};
use menu::Cancel;
use release_channel::ReleaseChannel;
use util::ResultExt;
use workspace::{
    ui::{h_flex, v_flex, Button, Clickable, IconButton, IconName, IconSize, Label, StyledExt},
    Workspace,
};

pub struct UpdateNotification {
    version: SemanticVersion,
    workspace: WeakView<Workspace>,
}

impl EventEmitter<DismissEvent> for UpdateNotification {}

impl Render for UpdateNotification {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let app_name = ReleaseChannel::global(cx).display_name();

        v_flex()
            .occlude()
            .on_action(cx.listener(UpdateNotification::dismiss))
            .elevation_3(cx)
            .p_4()
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        Label::new(format!("Updated to {app_name} {}", self.version)).ml(rems(0.2)),
                    )
                    .child(
                        IconButton::new("cancel", IconName::Close)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener(|this, _, cx| this.dismiss(&menu::Cancel, cx))),
                    ),
            )
            .child(h_flex().pt(rems(0.3)).child(
                Button::new("notes", "View the release notes").on_click(cx.listener(
                    |this, _, cx| {
                        this.workspace
                            .update(cx, |workspace, cx| {
                                crate::view_release_notes_locally(workspace, cx);
                            })
                            .log_err();
                        this.dismiss(&menu::Cancel, cx)
                    },
                )),
            ))
    }
}

impl UpdateNotification {
    pub fn new(version: SemanticVersion, workspace: WeakView<Workspace>) -> Self {
        Self { version, workspace }
    }

    pub fn dismiss(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }
}
