use gpui::{AnyElement, Render, ViewContext, WeakView};
use ui::{prelude::*, ButtonCommon, Icon, IconButton, Tooltip};
use workspace::{StatusItemView, Workspace};

use crate::feedback_modal::FeedbackModal;

pub struct DeployFeedbackButton {
    _active: bool,
    workspace: WeakView<Workspace>,
}

impl DeployFeedbackButton {
    pub fn new(workspace: &Workspace) -> Self {
        DeployFeedbackButton {
            _active: false,
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
            .selected(is_open)
            .tooltip(|cx| Tooltip::text("Give Feedback", cx))
            .on_click(cx.listener(|this, _, cx| {
                let Some(workspace) = this.workspace.upgrade() else {
                    return;
                };
                workspace.update(cx, |workspace, cx| {
                    workspace.toggle_modal(cx, |cx| FeedbackModal::new(cx))
                })
            }))
            .into_any_element()
    }
}
impl StatusItemView for DeployFeedbackButton {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn workspace::item::ItemHandle>,
        _cx: &mut ViewContext<Self>,
    ) {
        // no-op
    }
}
