use gpui::{
    elements::*,
    platform::{CursorStyle, MouseButton},
    Entity, View, ViewContext,
};
use settings::Settings;
use workspace::{item::ItemHandle, StatusItemView};

use crate::feedback_editor::{FeedbackEditor, GiveFeedback};

pub struct DeployFeedbackButton {
    active: bool,
}

impl Entity for DeployFeedbackButton {
    type Event = ();
}

impl DeployFeedbackButton {
    pub fn new() -> Self {
        DeployFeedbackButton { active: false }
    }
}

impl View for DeployFeedbackButton {
    fn ui_name() -> &'static str {
        "DeployFeedbackButton"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let active = self.active;
        let theme = cx.global::<Settings>().theme.clone();
        Stack::new()
            .with_child(
                MouseEventHandler::<Self, Self>::new(0, cx, |state, _| {
                    let style = &theme
                        .workspace
                        .status_bar
                        .sidebar_buttons
                        .item
                        .style_for(state, active);

                    Svg::new("icons/feedback_16.svg")
                        .with_color(style.icon_color)
                        .constrained()
                        .with_width(style.icon_size)
                        .aligned()
                        .constrained()
                        .with_width(style.icon_size)
                        .with_height(style.icon_size)
                        .contained()
                        .with_style(style.container)
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, _, cx| {
                    if !active {
                        cx.dispatch_action(GiveFeedback)
                    }
                })
                .with_tooltip::<Self>(
                    0,
                    "Send Feedback".into(),
                    Some(Box::new(GiveFeedback)),
                    theme.tooltip.clone(),
                    cx,
                ),
            )
            .into_any()
    }
}

impl StatusItemView for DeployFeedbackButton {
    fn set_active_pane_item(&mut self, item: Option<&dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        if let Some(item) = item {
            if let Some(_) = item.downcast::<FeedbackEditor>() {
                self.active = true;
                cx.notify();
                return;
            }
        }
        self.active = false;
        cx.notify();
    }
}
