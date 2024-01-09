use gpui::{
    div, DismissEvent, EventEmitter, InteractiveElement, IntoElement, ParentElement, Render,
    SemanticVersion, StatefulInteractiveElement, Styled, ViewContext,
};
use menu::Cancel;
use util::channel::ReleaseChannel;
use workspace::ui::{h_stack, v_stack, Icon, IconPath, Label, StyledExt};

pub struct UpdateNotification {
    version: SemanticVersion,
}

impl EventEmitter<DismissEvent> for UpdateNotification {}

impl Render for UpdateNotification {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let app_name = cx.global::<ReleaseChannel>().display_name();

        v_stack()
            .on_action(cx.listener(UpdateNotification::dismiss))
            .elevation_3(cx)
            .p_4()
            .child(
                h_stack()
                    .justify_between()
                    .child(Label::new(format!(
                        "Updated to {app_name} {}",
                        self.version
                    )))
                    .child(
                        div()
                            .id("cancel")
                            .child(Icon::new(IconPath::Close))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, cx| this.dismiss(&menu::Cancel, cx))),
                    ),
            )
            .child(
                div()
                    .id("notes")
                    .child(Label::new("View the release notes"))
                    .cursor_pointer()
                    .on_click(|_, cx| crate::view_release_notes(&Default::default(), cx)),
            )
    }
}

impl UpdateNotification {
    pub fn new(version: SemanticVersion) -> Self {
        Self { version }
    }

    pub fn dismiss(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }
}
