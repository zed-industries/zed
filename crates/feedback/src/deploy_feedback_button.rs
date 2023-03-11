use gpui::{elements::*, CursorStyle, Entity, MouseButton, RenderContext, View, ViewContext};
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

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        let active = self.active;
        Stack::new()
            .with_child(
                MouseEventHandler::<Self>::new(0, cx, |state, cx| {
                    let theme = &cx.global::<Settings>().theme;
                    let style = &theme
                        .workspace
                        .status_bar
                        .sidebar_buttons
                        .item
                        .style_for(state, active);

                    Svg::new("icons/speech_bubble_12.svg")
                        .with_color(style.icon_color)
                        .constrained()
                        .with_width(style.icon_size)
                        .aligned()
                        .constrained()
                        .with_width(style.icon_size)
                        .with_height(style.icon_size)
                        .contained()
                        .with_style(style.container)
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    if !active {
                        cx.dispatch_action(GiveFeedback)
                    }
                })
                .boxed(),
            )
            .boxed()
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
