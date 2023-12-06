use gpui::{AnyElement, Render, ViewContext, WeakView};
use ui::{prelude::*, ButtonCommon, Icon, IconButton, Tooltip};
use workspace::{item::ItemHandle, StatusItemView, Workspace};

use crate::feedback_editor::FeedbackEditor;

pub struct DeployFeedbackButton {
    active: bool,
    workspace: WeakView<Workspace>,
}

impl DeployFeedbackButton {
    pub fn new(workspace: &Workspace) -> Self {
        DeployFeedbackButton {
            active: false,
            workspace: workspace.weak_handle(),
        }
    }
}

impl Render for DeployFeedbackButton {
    type Element = AnyElement;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let active = self.active;

        IconButton::new("give-feedback", Icon::Envelope)
            .style(ui::ButtonStyle::Subtle)
            .tooltip(|cx| Tooltip::text("Give Feedback", cx))
            .on_click(cx.listener(move |this, _, cx| {
                let Some(workspace) = this.workspace.upgrade() else {
                    return;
                };

                if !active {
                    workspace.update(cx, |workspace, cx| FeedbackEditor::deploy(workspace, cx))
                }
            }))
            .into_any_element()
    }
}

impl StatusItemView for DeployFeedbackButton {
    fn set_active_pane_item(&mut self, item: Option<&dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        if let Some(item) = item {
            if let Some(_) = item.downcast::<FeedbackEditor>() {
                self.active = true;
                cx.notify();
                return;
            }
        }
        self.active = false;
        cx.notify();
    }
}
