use gpui::{
    div, DismissEvent, EventEmitter, InteractiveElement, IntoElement, ParentElement, Render,
    SemanticVersion, StatefulInteractiveElement, Styled, ViewContext, WeakView,
};
use menu::Cancel;
use release_channel::ReleaseChannel;
use util::ResultExt;
use workspace::{
    ui::{h_flex, v_flex, Icon, IconName, Label, StyledExt},
    Workspace,
};

pub struct UpdateNotification {
    version: SemanticVersion,
    workspace: WeakView<Workspace>,
}

impl EventEmitter<DismissEvent> for UpdateNotification {}

impl Render for UpdateNotification {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let app_name = ReleaseChannel::global(cx).display_name();

        v_flex()
            .on_action(cx.listener(UpdateNotification::dismiss))
            .elevation_3(cx)
            .p_4()
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new(format!(
                        "Updated to {app_name} {}",
                        self.version
                    )))
                    .child(
                        div()
                            .id("cancel")
                            .child(Icon::new(IconName::Close))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, cx| this.dismiss(&menu::Cancel, cx))),
                    ),
            )
            .child(
                div()
                    .id("notes")
                    .child(Label::new("View the release notes"))
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, cx| {
                        this.workspace
                            .update(cx, |workspace, cx| {
                                crate::view_release_notes_locally(workspace, cx);
                            })
                            .log_err();
                        this.dismiss(&menu::Cancel, cx)
                    })),
            )
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
