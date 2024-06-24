use fuzzy::StringMatch;
use gpui::{
    rems, DismissEvent, EventEmitter, FocusableView, Model, Render, Subscription,
    Task as AsyncTask, View, ViewContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use project::Project;
use std::sync::Arc;
use ui::{prelude::*, ListItem, ListItemSpacing};
use workspace::{ModalView, Workspace};

use crate::debugger_panel::DebugPanel;

pub struct DebuggerSelectModal {
    picker: View<Picker<DebuggerModelDelegate>>,
    _subscription: Subscription,
}

impl DebuggerSelectModal {
    pub fn new(
        _project: Model<Project>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let picker =
            cx.new_view(|cx| Picker::uniform_list(DebuggerModelDelegate::new(workspace), cx));

        let _subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            _subscription,
        }
    }
}

impl ModalView for DebuggerSelectModal {}

impl EventEmitter<DismissEvent> for DebuggerSelectModal {}

impl FocusableView for DebuggerSelectModal {
    fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.picker.read(cx).focus_handle(cx)
    }
}

impl Render for DebuggerSelectModal {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        v_flex()
            .id("DebuggerSelectModel")
            .key_context("DebuggerSelectModel")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

struct DebuggerModelDelegate {
    matches: Vec<StringMatch>,
    selected_index: usize,
    workspace: WeakView<Workspace>,
    placeholder_text: Arc<str>,
}

impl DebuggerModelDelegate {
    fn new(workspace: WeakView<Workspace>) -> Self {
        Self {
            workspace,
            matches: vec![StringMatch {
                candidate_id: 0,
                score: 1.0,
                positions: vec![0],
                string: String::from("Mock debugger config"),
            }],
            selected_index: 0,
            placeholder_text: Arc::from("Select & Start a debugger config"),
        }
    }
}

impl PickerDelegate for DebuggerModelDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _cx: &mut ui::WindowContext) -> std::sync::Arc<str> {
        self.placeholder_text.clone()
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        AsyncTask::Ready(Some(()))
    }

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        self.workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.start_debug_adapter_client(cx);
            });

            workspace.focus_panel::<DebugPanel>(cx);
        });

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        Some(
            ListItem::new(SharedString::from("ajklsdf"))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .child(self.matches[ix].string.clone()),
        )
    }
}
