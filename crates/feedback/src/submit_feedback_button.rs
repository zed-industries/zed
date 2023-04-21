use gpui::{
    elements::{Label, MouseEventHandler},
    platform::{CursorStyle, MouseButton},
    Drawable, Element, Entity, View, ViewContext, ViewHandle,
};
use settings::Settings;
use workspace::{item::ItemHandle, ToolbarItemLocation, ToolbarItemView};

use crate::feedback_editor::{FeedbackEditor, SubmitFeedback};

pub struct SubmitFeedbackButton {
    pub(crate) active_item: Option<ViewHandle<FeedbackEditor>>,
}

impl SubmitFeedbackButton {
    pub fn new() -> Self {
        Self {
            active_item: Default::default(),
        }
    }
}

impl Entity for SubmitFeedbackButton {
    type Event = ();
}

impl View for SubmitFeedbackButton {
    fn ui_name() -> &'static str {
        "SubmitFeedbackButton"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Element<Self> {
        let theme = cx.global::<Settings>().theme.clone();
        enum SubmitFeedbackButton {}
        MouseEventHandler::<SubmitFeedbackButton, Self>::new(0, cx, |state, _| {
            let style = theme.feedback.submit_button.style_for(state, false);
            Label::new("Submit as Markdown", style.text.clone())
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, _, cx| {
            cx.dispatch_action(SubmitFeedback)
        })
        .aligned()
        .contained()
        .with_margin_left(theme.feedback.button_margin)
        .with_tooltip::<Self>(
            0,
            "cmd-s".into(),
            Some(Box::new(SubmitFeedback)),
            theme.tooltip.clone(),
            cx,
        )
        .boxed()
    }
}

impl ToolbarItemView for SubmitFeedbackButton {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> workspace::ToolbarItemLocation {
        cx.notify();
        if let Some(feedback_editor) = active_pane_item.and_then(|i| i.downcast::<FeedbackEditor>())
        {
            self.active_item = Some(feedback_editor);
            ToolbarItemLocation::PrimaryRight { flex: None }
        } else {
            self.active_item = None;
            ToolbarItemLocation::Hidden
        }
    }
}
