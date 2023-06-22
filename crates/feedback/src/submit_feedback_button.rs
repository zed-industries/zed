use crate::feedback_editor::{FeedbackEditor, SubmitFeedback};
use anyhow::Result;
use gpui::{
    elements::{Label, MouseEventHandler},
    platform::{CursorStyle, MouseButton},
    AnyElement, AppContext, Element, Entity, Task, View, ViewContext, ViewHandle,
};
use workspace::{item::ItemHandle, ToolbarItemLocation, ToolbarItemView};

pub fn init(cx: &mut AppContext) {
    cx.add_async_action(SubmitFeedbackButton::submit);
}

pub struct SubmitFeedbackButton {
    pub(crate) active_item: Option<ViewHandle<FeedbackEditor>>,
}

impl SubmitFeedbackButton {
    pub fn new() -> Self {
        Self {
            active_item: Default::default(),
        }
    }

    pub fn submit(
        &mut self,
        _: &SubmitFeedback,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if let Some(active_item) = self.active_item.as_ref() {
            Some(active_item.update(cx, |feedback_editor, cx| feedback_editor.submit(cx)))
        } else {
            None
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

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = theme::current(cx).clone();
        enum SubmitFeedbackButton {}
        MouseEventHandler::<SubmitFeedbackButton, Self>::new(0, cx, |state, _| {
            let style = theme.feedback.submit_button.style_for(state);
            Label::new("Submit as Markdown", style.text.clone())
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, this, cx| {
            this.submit(&Default::default(), cx);
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
        .into_any()
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
