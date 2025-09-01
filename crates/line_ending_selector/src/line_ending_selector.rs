use editor::Editor;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, WeakEntity, actions};
use language::{Buffer, LineEnding};
use picker::{Picker, PickerDelegate};
use project::Project;
use std::sync::Arc;
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace};

actions!(
    line_ending_selector,
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
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(move |workspace, _: &Toggle, window, cx| {
            Self::toggle(workspace, window, cx);
        });
    }

    fn toggle(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<()> {
        let (_, buffer, _) = workspace
            .active_item(cx)?
            .act_as::<Editor>(cx)?
            .read(cx)
            .active_excerpt(cx)?;
        let project = workspace.project().clone();

        workspace.toggle_modal(window, cx, move |window, cx| {
            LineEndingSelector::new(buffer, project, window, cx)
        });
        Some(())
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
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
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
    matches: Vec<StringMatch>,
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
            matches: vec![],
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
        if let Some(mat) = self.matches.get(self.selected_index) {
            let line_ending = match mat.candidate_id {
                0 => LineEnding::Unix,
                1 => LineEnding::Windows,
                _ => unreachable!(),
            };
            self.buffer.update(cx, |this, cx| {
                this.set_line_ending(line_ending, cx);
            });
            let buffer = self.buffer.clone();
            self.project.update(cx, |this, cx| {
                this.save_buffer(buffer, cx).detach();
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
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                vec![
                    StringMatch {
                        candidate_id: 0,
                        string: "LF".to_string(),
                        positions: Vec::new(),
                        score: 0.0,
                    },
                    StringMatch {
                        candidate_id: 1,
                        string: "CRLF".to_string(),
                        positions: Vec::new(),
                        score: 0.0,
                    },
                ]
            } else {
                match_strings(
                    &[
                        StringMatchCandidate::new(0, "LF"),
                        StringMatchCandidate::new(1, "CRLF"),
                    ],
                    &query,
                    false,
                    false,
                    2,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(cx, |this, cx| {
                let delegate = &mut this.delegate;
                delegate.matches = matches;
                delegate.selected_index = delegate
                    .selected_index
                    .min(delegate.matches.len().saturating_sub(1));
                cx.notify();
            })
            .log_err();
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];
        let line_ending = match mat.candidate_id {
            0 => LineEnding::Unix,
            1 => LineEnding::Windows,
            _ => unreachable!(),
        };
        let label = match line_ending {
            LineEnding::Unix => "LF",
            LineEnding::Windows => "CRLF",
        };

        let mut list_item = ListItem::new(ix)
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .child(HighlightedLabel::new(label, mat.positions.clone()));

        if self.line_ending == line_ending {
            list_item = list_item.end_slot(Icon::new(IconName::Check).color(Color::Muted));
        }

        Some(list_item)
    }
}
