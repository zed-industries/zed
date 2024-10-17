use editor::{actions::ToggleGoToRecentDirectories, Editor};
use fuzzy::StringMatch;
use gpui::*;
use picker::{
    highlighted_match_with_paths::{HighlightedMatchWithPaths, HighlightedText},
    Picker, PickerDelegate,
};
use std::{path::PathBuf, sync::Arc};
use terminal_view::TerminalView;
use ui::{
    h_flex, v_flex, vh, ActiveTheme, Color, Icon, IconName, Label, LabelCommon, ListItem, Selectable, Tooltip
};
use workspace::{DismissDecision, ModalView, Workspace};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(DirectoryHistoryPicker::register)
        .detach();
}

pub fn toggle(editor: View<Editor>, _: &ToggleGoToRecentDirectories, cx: &mut WindowContext) {
    println!("Toggling directory history picker");
    let outline = editor
        .read(cx)
        .buffer()
        .read(cx)
        .snapshot(cx)
        .outline(Some(cx.theme().syntax()));

    if let Some((workspace, outline)) = editor.read(cx).workspace().zip(outline) {
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(cx, |cx| DirectoryHistoryPicker::new(cx));
        })
    }
}

pub struct DirectoryHistoryPicker {
    picker: View<Picker<DirectoryHistoryDelegate>>,
}

impl FocusableView for DirectoryHistoryPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for DirectoryHistoryPicker {}

impl ModalView for DirectoryHistoryPicker {
    fn on_before_dismiss(&mut self, cx: &mut ViewContext<Self>) -> DismissDecision {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.restore_terminal(cx);
        });
        DismissDecision::Dismiss(true)
    }
}

impl Render for DirectoryHistoryPicker {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl DirectoryHistoryPicker {
    fn register(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        let handle = cx.view().downgrade();
        editor
            .register_action(move |action: &ToggleGoToRecentDirectories, cx| {
                if let Some(editor) = handle.upgrade() {
                    toggle(editor, action, cx);
                }
            })
            .detach();
    }

    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        // let delegate = DirectoryHistoryDelegate::new(terminal, cx.view().downgrade());
        let delegate = DirectoryHistoryDelegate::new(cx.view().downgrade());
        let picker =
            cx.new_view(|cx| Picker::uniform_list(delegate, cx).max_height(Some(vh(0.75, cx))));
        Self { picker }
    }
}

pub struct DirectoryHistoryDelegate {
    history: Vec<PathBuf>,
    // terminal: WeakView<TerminalView>,
    picker_view: WeakView<DirectoryHistoryPicker>,
    matches: Vec<StringMatch>,
    selected_match_index: usize,
    last_query: String,
}

impl DirectoryHistoryDelegate {
    pub fn new(
        // terminal: WeakView<TerminalView>,
        picker_view: WeakView<DirectoryHistoryPicker>,
    ) -> Self {
        Self {
            history: Vec::new(),
            // terminal,
            picker_view,
            matches: Vec::new(),
            selected_match_index: 0,
            last_query: String::new(),
        }
    }

    fn restore_terminal(&mut self, cx: &mut WindowContext) {
        todo!();
        // if let Some(terminal) = self.terminal.upgrade() {
        //     terminal.update(cx, |terminal, cx| {
        //         // Restore terminal state if needed
        //     });
        // }
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        navigate: bool,
        cx: &mut ViewContext<Picker<DirectoryHistoryDelegate>>,
    ) {
        self.selected_match_index = ix;

        if navigate && !self.matches.is_empty() {
            let selected_match = &self.matches[self.selected_match_index];
            let selected_path = &self.history[selected_match.candidate_id];
            todo!();
            // if let Some(terminal) = self.terminal.upgrade() {
            //     terminal.update(cx, |terminal, cx| {
            //         // Update terminal preview based on selected path
            //     });
            // }
        }
    }
}

impl PickerDelegate for DirectoryHistoryDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search directory history...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        cx: &mut ViewContext<Picker<DirectoryHistoryDelegate>>,
    ) {
        self.set_selected_index(ix, true, cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<DirectoryHistoryDelegate>>,
    ) -> Task<()> {
        if query.is_empty() {
            self.restore_terminal(cx);
            self.matches = self
                .history
                .iter()
                .enumerate()
                .map(|(index, _)| StringMatch {
                    candidate_id: index,
                    score: Default::default(),
                    positions: Default::default(),
                    string: Default::default(),
                })
                .collect();
            self.set_selected_index(0, false, cx);
        } else {
            // Implement fuzzy search on self.history
            // Update self.matches based on the search results
            // Set selected_index to the best match
        }
        self.last_query = query;
        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        cx: &mut ViewContext<Picker<DirectoryHistoryDelegate>>,
    ) {
        if let Some(selected_path) = self
            .matches
            .get(self.selected_match_index)
            .map(|m| &self.history[m.candidate_id])
        {
            todo!();
            // if let Some(terminal) = self.terminal.upgrade() {
            //     terminal.update(cx, |terminal, cx| {
            //         // Change directory in the terminal
            //     });
            // }
        }
        self.dismissed(cx);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<DirectoryHistoryDelegate>>) {
        if let Some(picker_view) = self.picker_view.upgrade() {
            picker_view.update(cx, |_, cx| cx.emit(DismissEvent));
        }
        self.restore_terminal(cx);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = self.matches.get(ix)?;
        let path = self.history.get(mat.candidate_id)?;

        Some(
            ListItem::new(ix).inset(true).selected(selected).child(
                h_flex()
                    .gap_x(rems(0.5))
                    .child(Icon::new(IconName::Folder))
                    .child(Label::new(path.to_string_lossy().to_string())),
            ),
        )
    }
}
