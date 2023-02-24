use gpui::{
    elements::{Flex, Label, MouseEventHandler, ParentElement, Text},
    CursorStyle, Element, ElementBox, Entity, MouseButton, RenderContext, View, ViewContext,
    ViewHandle,
};
use settings::Settings;
use workspace::{item::ItemHandle, ToolbarItemLocation, ToolbarItemView};

use crate::{feedback_editor::FeedbackEditor, OpenZedCommunityRepo};

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

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();

        Flex::row()
            .with_child(
                Text::new(
                    "We read whatever you submit here. For issues and discussions, visit the ",
                    theme.feedback.info_text_default.text.clone(),
                )
                .with_soft_wrap(false)
                .aligned()
                .boxed(),
            )
            .with_child(
                MouseEventHandler::<OpenZedCommunityRepo>::new(0, cx, |state, _| {
                    let text = if state.hovered() {
                        theme.feedback.link_text_hover.clone()
                    } else {
                        theme.feedback.link_text_default.clone()
                    };

                    Label::new("community repo", text.text)
                        .contained()
                        .aligned()
                        .left()
                        .clipped()
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, cx| {
                    cx.dispatch_action(OpenZedCommunityRepo)
                })
                .boxed(),
            )
            .with_child(
                Text::new(" on GitHub.", theme.feedback.info_text_default.text.clone())
                    .with_soft_wrap(false)
                    .aligned()
                    .boxed(),
            )
            .aligned()
            .left()
            .clipped()
            .boxed()
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
