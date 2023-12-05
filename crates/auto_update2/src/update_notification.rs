use gpui::{
    div, listener, DismissEvent, Div, EventEmitter, InteractiveElement, ParentElement, Render,
    SemanticVersion, StatefulInteractiveElement, Styled, ViewContext,
};
use util::channel::ReleaseChannel;
use workspace::ui::{h_stack, v_stack, Icon, IconElement, Label, StyledExt};

pub struct UpdateNotification {
    version: SemanticVersion,
}

impl EventEmitter<DismissEvent> for UpdateNotification {}

impl Render for UpdateNotification {
    type Element = Div;

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
        let app_name = cx.global::<ReleaseChannel>().display_name();

        v_stack()
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
                            .child(IconElement::new(Icon::Close))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, cx| this.dismiss(cx))),
                    ),
            )
            .child(
                div()
                    .id("notes")
                    .child(Label::new("View the release notes"))
                    .cursor_pointer()
                    .on_click(listener(|_, cx| {
                        crate::view_release_notes(&Default::default(), cx)
                    })),
            )
    }
}

impl UpdateNotification {
    pub fn new(version: SemanticVersion) -> Self {
        Self { version }
    }

    pub fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }
}
