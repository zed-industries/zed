use crate::OpenBrowser;
use gpui::{
    elements::{MouseEventHandler, Text},
    platform::CursorStyle,
    Element, Entity, MouseButton, RenderContext, View,
};
use settings::Settings;
use workspace::{item::ItemHandle, StatusItemView};

pub const NEW_ISSUE_URL: &str = "https://github.com/zed-industries/feedback/issues/new/choose";

pub struct FeedbackLink;

impl Entity for FeedbackLink {
    type Event = ();
}

impl View for FeedbackLink {
    fn ui_name() -> &'static str {
        "FeedbackLink"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> gpui::ElementBox {
        MouseEventHandler::<Self>::new(0, cx, |state, cx| {
            let theme = &cx.global::<Settings>().theme;
            let theme = &theme.workspace.status_bar.feedback;
            Text::new(
                "Give Feedback".to_string(),
                theme.style_for(state, false).clone(),
            )
            .boxed()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, cx| {
            cx.dispatch_action(OpenBrowser {
                url: NEW_ISSUE_URL.into(),
            })
        })
        .boxed()
    }
}

impl StatusItemView for FeedbackLink {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn ItemHandle>,
        _: &mut gpui::ViewContext<Self>,
    ) {
    }
}
