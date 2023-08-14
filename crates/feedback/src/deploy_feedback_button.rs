use gpui::{
    elements::*,
    platform::{CursorStyle, MouseButton},
    Entity, View, ViewContext, WeakViewHandle,
};
use workspace::{item::ItemHandle, StatusItemView, Workspace};

use crate::feedback_editor::{FeedbackEditor, GiveFeedback};

pub struct DeployFeedbackButton {
    active: bool,
    workspace: WeakViewHandle<Workspace>,
}

impl Entity for DeployFeedbackButton {
    type Event = ();
}

impl DeployFeedbackButton {
    pub fn new(workspace: &Workspace) -> Self {
        DeployFeedbackButton {
            active: false,
            workspace: workspace.weak_handle(),
        }
    }
}

impl View for DeployFeedbackButton {
    fn ui_name() -> &'static str {
        "DeployFeedbackButton"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let active = self.active;
        let theme = theme::current(cx).clone();
        Stack::new()
            .with_child(
                MouseEventHandler::<Self, Self>::new(0, cx, |state, _| {
                    let style = &theme
                        .workspace
                        .status_bar
                        .panel_buttons
                        .button
                        .in_state(active)
                        .style_for(state);

                    Svg::new("icons/feedback.svg")
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
                .on_click(MouseButton::Left, move |_, this, cx| {
                    if !active {
                        if let Some(workspace) = this.workspace.upgrade(cx) {
                            workspace
                                .update(cx, |workspace, cx| FeedbackEditor::deploy(workspace, cx))
                        }
                    }
                })
                .with_tooltip::<Self>(
                    0,
                    "Send Feedback",
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
