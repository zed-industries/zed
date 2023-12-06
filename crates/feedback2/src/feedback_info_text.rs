use gpui::{Div, EventEmitter, View, ViewContext};
use ui::{prelude::*, Label};
use workspace::{item::ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView};

use crate::feedback_editor::FeedbackEditor;

pub struct FeedbackInfoText {
    active_item: Option<View<FeedbackEditor>>,
}

impl FeedbackInfoText {
    pub fn new() -> Self {
        Self {
            active_item: Default::default(),
        }
    }
}

// TODO
impl Render for FeedbackInfoText {
    type Element = Div;

    fn render(&mut self, _: &mut ViewContext<Self>) -> Self::Element {
        // TODO - get this into the toolbar area like before - ensure things work the same when horizontally shrinking app
        div()
            .size_full()
            .child(Label::new("Share your feedback. Include your email for replies. For issues and discussions, visit the ").color(Color::Muted))
            .child(Label::new("community repo").color(Color::Muted)) // TODO - this needs to be a link
            .child(Label::new(".").color(Color::Muted))
    }
}

// TODO - delete
// impl View for FeedbackInfoText {
//     fn ui_name() -> &'static str {
//         "FeedbackInfoText"
//     }

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
//         let theme = theme::current(cx).clone();

//         Flex::row()
//             .with_child(
//                 Text::new(
//                     "Share your feedback. Include your email for replies. For issues and discussions, visit the ",
//                     theme.feedback.info_text_default.text.clone(),
//                 )
//                 .with_soft_wrap(false)
//                 .aligned(),
//             )
//             .with_child(
//                 MouseEventHandler::new::<OpenZedCommunityRepo, _>(0, cx, |state, _| {
//                     let style = if state.hovered() {
//                         &theme.feedback.link_text_hover
//                     } else {
//                         &theme.feedback.link_text_default
//                     };
//                     Label::new("community repo", style.text.clone())
//                         .contained()
//                         .with_style(style.container)
//                         .aligned()
//                         .left()
//                         .clipped()
//                 })
//                 .with_cursor_style(CursorStyle::PointingHand)
//                 .on_click(MouseButton::Left, |_, _, cx| {
//                     open_zed_community_repo(&Default::default(), cx)
//                 }),
//             )
//             .with_child(
//                 Text::new(".", theme.feedback.info_text_default.text.clone())
//                     .with_soft_wrap(false)
//                     .aligned(),
//             )
//             .contained()
//             .with_style(theme.feedback.info_text_default.container)
//             .aligned()
//             .left()
//             .clipped()
//             .into_any()
//     }
// }

impl EventEmitter<ToolbarItemEvent> for FeedbackInfoText {}

impl ToolbarItemView for FeedbackInfoText {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> workspace::ToolbarItemLocation {
        cx.notify();

        if let Some(feedback_editor) = active_pane_item.and_then(|i| i.downcast::<FeedbackEditor>())
        {
            dbg!("Editor");
            self.active_item = Some(feedback_editor);
            ToolbarItemLocation::PrimaryLeft
        } else {
            dbg!("no editor");
            self.active_item = None;
            ToolbarItemLocation::Hidden
        }
    }
}
