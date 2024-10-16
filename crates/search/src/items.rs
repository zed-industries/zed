use editor::Editor;
use gpui::{IntoElement, Render, ViewContext, WeakView};
use ui::{prelude::*, Color, IconName, Tooltip};
use workspace::{item::ItemHandle, DeploySearch, StatusItemView, Workspace};

use crate::ProjectSearchView;

pub struct ProjectSearchIndicator {
    active_editor: Option<WeakView<Editor>>,
    workspace: WeakView<Workspace>,
}

impl Render for ProjectSearchIndicator {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        IconButton::new("project-search-indicator", IconName::MagnifyingGlass)
            .icon_size(IconSize::Small)
            .icon_color(Color::Default)
            .tooltip(|cx| Tooltip::for_action("Project Search", &DeploySearch::default(), cx))
            .on_click(cx.listener(|this, _, cx| {
                if let Some(workspace) = this.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        ProjectSearchView::deploy_search(workspace, &DeploySearch::default(), cx)
                    })
                }
            }))
    }
}

impl ProjectSearchIndicator {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            active_editor: None,
            workspace: workspace.weak_handle(),
        }
    }
}

impl StatusItemView for ProjectSearchIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self.active_editor = Some(editor.downgrade());
        } else {
            self.active_editor = None;
        }
        cx.notify();
    }
}
