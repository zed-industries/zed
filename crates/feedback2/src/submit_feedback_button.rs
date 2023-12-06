use crate::feedback_editor::{FeedbackEditor, SubmitFeedback};
use anyhow::Result;
use gpui::{AppContext, Div, EventEmitter, Render, Task, View, ViewContext};
use ui::prelude::*;
use workspace::{item::ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView};

pub fn init(cx: &mut AppContext) {
    // cx.add_action(SubmitFeedbackButton::submit);
}

pub struct SubmitFeedbackButton {
    pub(crate) active_item: Option<View<FeedbackEditor>>,
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

// TODO
impl Render for SubmitFeedbackButton {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let allow_submission = self
            .active_item
            .as_ref()
            .map_or(true, |i| i.read(cx).allow_submission);

        div()
    }
}

// TODO - delete
// impl View for SubmitFeedbackButton {

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
//         let theme = theme::current(cx).clone();
//         let allow_submission = self
//             .active_item
//             .as_ref()
//             .map_or(true, |i| i.read(cx).allow_submission);

//         enum SubmitFeedbackButton {}
//         MouseEventHandler::new::<SubmitFeedbackButton, _>(0, cx, |state, _| {
//             let text;
//             let style = if allow_submission {
//                 text = "Submit as Markdown";
//                 theme.feedback.submit_button.style_for(state)
//             } else {
//                 text = "Submitting...";
//                 theme
//                     .feedback
//                     .submit_button
//                     .disabled
//                     .as_ref()
//                     .unwrap_or(&theme.feedback.submit_button.default)
//             };

//             Label::new(text, style.text.clone())
//                 .contained()
//                 .with_style(style.container)
//         })
//         .with_cursor_style(CursorStyle::PointingHand)
//         .on_click(MouseButton::Left, |_, this, cx| {
//             this.submit(&Default::default(), cx);
//         })
//         .aligned()
//         .contained()
//         .with_margin_left(theme.feedback.button_margin)
//         .with_tooltip::<Self>(
//             0,
//             "cmd-s",
//             Some(Box::new(SubmitFeedback)),
//             theme.tooltip.clone(),
//             cx,
//         )
//         .into_any()
//     }
// }

impl EventEmitter<ToolbarItemEvent> for SubmitFeedbackButton {}

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
            ToolbarItemLocation::PrimaryRight
        } else {
            self.active_item = None;
            ToolbarItemLocation::Hidden
        }
    }
}
