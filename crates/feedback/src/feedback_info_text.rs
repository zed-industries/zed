use gpui::{
    elements::Label, Element, ElementBox, Entity, RenderContext, View, ViewContext, ViewHandle,
};
use settings::Settings;
use workspace::{item::ItemHandle, ToolbarItemLocation, ToolbarItemView};

use crate::feedback_editor::FeedbackEditor;

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
        let text = "We read whatever you submit here. For issues and discussions, visit the community repo on GitHub.";
        Label::new(text.to_string(), theme.feedback.info_text.text.clone())
            .contained()
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
