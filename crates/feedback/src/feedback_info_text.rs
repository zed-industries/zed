use gpui::{
    elements::{Flex, Label, MouseEventHandler, ParentElement, Text},
    platform::{CursorStyle, MouseButton},
    AnyElement, Element, Entity, View, ViewContext, ViewHandle,
};
use workspace::{item::ItemHandle, ToolbarItemLocation, ToolbarItemView};

use crate::{feedback_editor::FeedbackEditor, open_zed_community_repo, OpenZedCommunityRepo};

pub struct FeedbackInfoText {
    active_item: Option<ViewHandle<FeedbackEditor>>,
}

impl FeedbackInfoText {
    pub fn new() -> Self {
        Self {
            active_item: Default::default(),
        }
    }
}

impl Entity for FeedbackInfoText {
    type Event = ();
}

impl View for FeedbackInfoText {
    fn ui_name() -> &'static str {
        "FeedbackInfoText"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = theme::current(cx).clone();

        Flex::row()
            .with_child(
                Text::new(
                    "Share your feedback. Include your email for replies. For issues and discussions, visit the ",
                    theme.feedback.info_text_default.text.clone(),
                )
                .with_soft_wrap(false)
                .aligned(),
            )
            .with_child(
                MouseEventHandler::<OpenZedCommunityRepo, Self>::new(0, cx, |state, _| {
                    let contained_text = if state.hovered() {
                        &theme.feedback.link_text_hover
                    } else {
                        &theme.feedback.link_text_default
                    };

                    Label::new("community repo", contained_text.text.clone())
                        .contained()
                        .aligned()
                        .left()
                        .clipped()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, _, cx| {
                    open_zed_community_repo(&Default::default(), cx)
                }),
            )
            .with_child(
                Text::new(".", theme.feedback.info_text_default.text.clone())
                    .with_soft_wrap(false)
                    .aligned(),
            )
            .aligned()
            .left()
            .clipped()
            .into_any()
    }
}

impl ToolbarItemView for FeedbackInfoText {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> workspace::ToolbarItemLocation {
        cx.notify();
        if let Some(feedback_editor) = active_pane_item.and_then(|i| i.downcast::<FeedbackEditor>())
        {
            self.active_item = Some(feedback_editor);
            ToolbarItemLocation::PrimaryLeft {
                flex: Some((1., false)),
            }
        } else {
            self.active_item = None;
            ToolbarItemLocation::Hidden
        }
    }
}
