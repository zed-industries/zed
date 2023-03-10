use gpui::{elements::*, CursorStyle, Entity, MouseButton, RenderContext, View, ViewContext};
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
                    let style = &theme.workspace.status_bar.feedback.style_for(state, false);
                    Svg::new("icons/speech_bubble_12.svg")
                        .with_color(style.color)
                        .constrained()
                        .with_width(style.icon_width)
                        .aligned()
                        .constrained()
                        .with_width(style.button_width)
                        .with_height(style.button_width)
                        .contained()
                        .with_style(style.container)
                        .boxed()
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
