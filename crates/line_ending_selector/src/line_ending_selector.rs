use editor::Editor;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity, actions};
use language::{Buffer, LineEnding};
use picker::{Picker, PickerDelegate};
use project::Project;
use std::sync::Arc;
use ui::{ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::ModalView;

actions!(
    line_ending,
    [
        /// Toggles the line ending selector modal.
        Toggle
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(LineEndingSelector::register).detach();
}

pub struct LineEndingSelector {
    picker: Entity<Picker<LineEndingSelectorDelegate>>,
}

impl LineEndingSelector {
    fn register(editor: &mut Editor, _window: Option<&mut Window>, cx: &mut Context<Editor>) {
        let editor_handle = cx.weak_entity();
        editor
            .register_action(move |_: &Toggle, window, cx| {
                Self::toggle(&editor_handle, window, cx);
            })
            .detach();
    }

    fn toggle(editor: &WeakEntity<Editor>, window: &mut Window, cx: &mut App) {
        let Some((workspace, buffer)) = editor
            .update(cx, |editor, cx| {
                Some((editor.workspace()?, editor.active_excerpt(cx)?.1))
            })
            .ok()
            .flatten()
        else {
            return;
        };

        workspace.update(cx, |workspace, cx| {
            let project = workspace.project().clone();
            workspace.toggle_modal(window, cx, move |window, cx| {
                LineEndingSelector::new(buffer, project, window, cx)
            });
        })
    }

    fn new(
        buffer: Entity<Buffer>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let line_ending = buffer.read(cx).line_ending();
        let delegate =
            LineEndingSelectorDelegate::new(cx.entity().downgrade(), buffer, project, line_ending);
        let picker = cx.new(|cx| Picker::nonsearchable_uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl Render for LineEndingSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl Focusable for LineEndingSelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for LineEndingSelector {}
impl ModalView for LineEndingSelector {}

struct LineEndingSelectorDelegate {
    line_ending_selector: WeakEntity<LineEndingSelector>,
    buffer: Entity<Buffer>,
    project: Entity<Project>,
    line_ending: LineEnding,
    matches: Vec<LineEnding>,
    selected_index: usize,
}

impl LineEndingSelectorDelegate {
    fn new(
        line_ending_selector: WeakEntity<LineEndingSelector>,
        buffer: Entity<Buffer>,
        project: Entity<Project>,
        line_ending: LineEnding,
    ) -> Self {
        Self {
            line_ending_selector,
            buffer,
            project,
            line_ending,
            matches: vec![LineEnding::Unix, LineEnding::Windows],
            selected_index: 0,
        }
    }
}

impl PickerDelegate for LineEndingSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a line ending…".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(line_ending) = self.matches.get(self.selected_index) {
            self.buffer.update(cx, |this, cx| {
                this.set_line_ending(*line_ending, cx);
            });
            let buffer = self.buffer.clone();
            let project = self.project.clone();
            cx.defer(move |cx| {
                project.update(cx, |this, cx| {
                    this.save_buffer(buffer, cx).detach();
                });
            });
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.line_ending_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        _query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        return Task::ready(());
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let line_ending = self.matches[ix];
        let label = match line_ending {
            LineEnding::Unix => "LF",
            LineEnding::Windows => "CRLF",
        };

        let mut list_item = ListItem::new(ix)
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .child(Label::new(label));

        if self.line_ending == line_ending {
            list_item = list_item.end_slot(Icon::new(IconName::Check).color(Color::Muted));
        }

        Some(list_item)
    }
}
