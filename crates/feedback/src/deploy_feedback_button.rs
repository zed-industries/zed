use gpui::{
    elements::{MouseEventHandler, ParentElement, Stack, Text},
    CursorStyle, Element, ElementBox, Entity, MouseButton, RenderContext, View, ViewContext,
};
use settings::Settings;
use workspace::{item::ItemHandle, StatusItemView};

use crate::feedback_editor::GiveFeedback;

pub struct DeployFeedbackButton;

impl Entity for DeployFeedbackButton {
    type Event = ();
}

impl View for DeployFeedbackButton {
    fn ui_name() -> &'static str {
        "DeployFeedbackButton"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        Stack::new()
            .with_child(
                MouseEventHandler::<Self>::new(0, cx, |state, cx| {
                    let theme = &cx.global::<Settings>().theme;
                    let theme = &theme.workspace.status_bar.feedback;

                    Text::new("Give Feedback", theme.style_for(state, true).clone()).boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, cx| cx.dispatch_action(GiveFeedback))
                .boxed(),
            )
            .boxed()
    }
}

impl StatusItemView for DeployFeedbackButton {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemHandle>, _: &mut ViewContext<Self>) {}
}
