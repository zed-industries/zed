use gpui::{AnyElement, Render, ViewContext, WeakView};
use ui::{prelude::*, ButtonCommon, Icon, IconButton, Tooltip};
use workspace::{item::ItemHandle, StatusItemView, Workspace};

use crate::{feedback_modal::FeedbackModal, GiveFeedback};

pub struct DeployFeedbackButton {
    workspace: WeakView<Workspace>,
}

impl DeployFeedbackButton {
    pub fn new(workspace: &Workspace) -> Self {
        DeployFeedbackButton {
            workspace: workspace.weak_handle(),
        }
    }
}

impl Render for DeployFeedbackButton {
    type Element = AnyElement;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let is_open = self
            .workspace
            .upgrade()
            .and_then(|workspace| {
                workspace.update(cx, |workspace, cx| {
                    workspace.active_modal::<FeedbackModal>(cx)
                })
            })
            .is_some();
        IconButton::new("give-feedback", Icon::Envelope)
            .style(ui::ButtonStyle::Subtle)
            .icon_size(IconSize::Small)
            .selected(is_open)
            .tooltip(|cx| Tooltip::text("Share Feedback", cx))
            .on_click(|_, cx| {
                cx.dispatch_action(Box::new(GiveFeedback));
            })
            .into_any_element()
    }
}

impl StatusItemView for DeployFeedbackButton {
    fn set_active_pane_item(
        &mut self,
        _item: Option<&dyn ItemHandle>,
        _cx: &mut ViewContext<Self>,
    ) {
    }
}
