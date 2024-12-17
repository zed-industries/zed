// TODO: Remove this once we've implemented the functionality.
#![allow(unused)]

use std::sync::Arc;

use fuzzy::PathMatch;
use gpui::{AppContext, DismissEvent, FocusHandle, FocusableView, Task, View, WeakModel, WeakView};
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, WorktreeId};
use ui::{prelude::*, ListItem};
use util::ResultExt as _;
use workspace::Workspace;

use crate::context_picker::ContextPicker;
use crate::context_store::ContextStore;

pub struct DirectoryContextPicker {
    picker: View<Picker<DirectoryContextPickerDelegate>>,
}

impl DirectoryContextPicker {
    pub fn new(
        context_picker: WeakView<ContextPicker>,
        workspace: WeakView<Workspace>,
        context_store: WeakModel<ContextStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate =
            DirectoryContextPickerDelegate::new(context_picker, workspace, context_store);
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));

        Self { picker }
    }
}

impl FocusableView for DirectoryContextPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for DirectoryContextPicker {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

pub struct DirectoryContextPickerDelegate {
    context_picker: WeakView<ContextPicker>,
    workspace: WeakView<Workspace>,
    context_store: WeakModel<ContextStore>,
    matches: Vec<PathMatch>,
    selected_index: usize,
}

impl DirectoryContextPickerDelegate {
    pub fn new(
        context_picker: WeakView<ContextPicker>,
        workspace: WeakView<Workspace>,
        context_store: WeakModel<ContextStore>,
    ) -> Self {
        Self {
            context_picker,
            workspace,
            context_store,
            matches: Vec::new(),
            selected_index: 0,
        }
    }
}

impl PickerDelegate for DirectoryContextPickerDelegate {
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

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search folders…".into()
    }

    fn update_matches(&mut self, _query: String, _cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        // TODO: Implement this once we fix the issues with the file context picker.
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, _cx: &mut ViewContext<Picker<Self>>) {
        // TODO: Implement this once we fix the issues with the file context picker.
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.context_picker
            .update(cx, |this, cx| {
                this.reset_mode();
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn render_match(
        &self,
        _ix: usize,
        _selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        None
    }
}
