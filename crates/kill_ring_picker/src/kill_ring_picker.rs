use editor::{Editor, KillRingEntrySnapshot, KillRingState, KillRingYankPreview};
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, AppContext as _, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Render, Subscription, Task, WeakEntity, Window,
};
use picker::{Picker, PickerDelegate};
use std::{sync::Arc, sync::atomic::AtomicBool};
use ui::prelude::*;
use ui::{HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{DismissDecision, ModalView};

#[cfg(test)]
mod kill_ring_picker_tests;

pub fn init(cx: &mut App) {
    cx.observe_new(KillRingPicker::register).detach();
}

pub struct KillRingPicker {
    picker: Entity<Picker<KillRingPickerDelegate>>,
    previous_focus_handle: Option<FocusHandle>,
    _subscriptions: Vec<Subscription>,
}

impl ModalView for KillRingPicker {
    fn on_before_dismiss(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> DismissDecision {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.revert_preview(window, cx);
        });
        DismissDecision::Dismiss(true)
    }
}

impl EventEmitter<DismissEvent> for KillRingPicker {}

impl Focusable for KillRingPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for KillRingPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("KillRingPicker")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

impl KillRingPicker {
    fn register(editor: &mut Editor, _window: Option<&mut Window>, cx: &mut Context<Editor>) {
        let handle = cx.entity().downgrade();
        editor
            .register_action(
                move |_: &editor::actions::KillRingPickAndYank, window, cx| {
                    let entries = cx.default_global::<KillRingState>().snapshot();
                    if entries.is_empty() {
                        return;
                    }

                    let Some(editor_handle) = handle.upgrade() else {
                        return;
                    };
                    let Some(workspace) = editor_handle.read(cx).workspace() else {
                        return;
                    };
                    let previous_focus_handle = window.focused(cx);

                    workspace.update(cx, |workspace, cx| {
                        workspace.toggle_modal(window, cx, move |window, cx| {
                            Self::new(editor_handle, entries, previous_focus_handle, window, cx)
                        });
                    });
                },
            )
            .detach();
    }

    fn new(
        active_editor: Entity<Editor>,
        entries: Vec<KillRingEntrySnapshot>,
        previous_focus_handle: Option<FocusHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let selector = cx.weak_entity();
        let delegate =
            KillRingPickerDelegate::new(active_editor.downgrade(), entries, Some(selector));
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        picker.update(cx, |picker, cx| {
            picker.delegate.set_selected_index(0, window, cx);
        });

        let release_subscription =
            cx.observe_release_in(&active_editor, window, |this, _, _window, cx| {
                this.picker.update(cx, |picker, _| {
                    picker.delegate.preview.take();
                });
                cx.emit(DismissEvent);
            });

        Self {
            picker,
            previous_focus_handle,
            _subscriptions: vec![release_subscription],
        }
    }
}

struct KillRingPickerDelegate {
    active_editor: WeakEntity<Editor>,
    entries: Vec<KillRingEntrySnapshot>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    preview: Option<KillRingYankPreview>,
    selector: Option<WeakEntity<KillRingPicker>>,
}

impl KillRingPickerDelegate {
    fn new(
        active_editor: WeakEntity<Editor>,
        entries: Vec<KillRingEntrySnapshot>,
        selector: Option<WeakEntity<KillRingPicker>>,
    ) -> Self {
        let matches = Self::matches_for_entries(&entries);
        Self {
            active_editor,
            entries,
            matches,
            selected_index: 0,
            preview: None,
            selector,
        }
    }

    fn matches_for_entries(entries: &[KillRingEntrySnapshot]) -> Vec<StringMatch> {
        entries
            .iter()
            .enumerate()
            .map(|(index, entry)| StringMatch {
                candidate_id: index,
                score: 0.,
                positions: Vec::new(),
                string: single_line_preview(entry.text()),
            })
            .collect()
    }

    fn candidates(&self) -> Vec<StringMatchCandidate> {
        self.entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                StringMatchCandidate::new(index, &single_line_preview(entry.text()))
            })
            .collect()
    }

    fn selected_entry_index(&self) -> Option<usize> {
        self.matches
            .get(self.selected_index)
            .map(|entry_match| entry_match.candidate_id)
    }

    fn selected_entry_text(&self) -> Option<String> {
        self.selected_entry_index()
            .and_then(|entry_index| self.entries.get(entry_index))
            .map(|entry| entry.text().to_string())
    }

    fn apply_preview(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(text) = self.selected_entry_text() else {
            return;
        };

        match self.active_editor.update(cx, |editor, cx| {
            editor.preview_kill_ring_yank(&text, window, cx)
        }) {
            Ok(preview) => {
                self.preview = preview;
            }
            Err(error) => {
                log::debug!("could not preview kill-ring entry: {error}");
                self.dismiss_modal(false, window, cx);
            }
        }
    }

    fn revert_preview(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if self.preview.take().is_none() {
            return;
        }

        self.active_editor
            .update(cx, |editor, cx| {
                editor.undo(&editor::actions::Undo, window, cx);
            })
            .log_err();
    }

    fn dismiss_modal(
        &self,
        refocus_previous: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if let Some(selector) = &self.selector {
            selector
                .update(cx, |selector, cx| {
                    if refocus_previous
                        && let Some(previous_focus_handle) = &selector.previous_focus_handle
                    {
                        previous_focus_handle.focus(window, cx);
                    }
                    cx.emit(DismissEvent);
                })
                .log_err();
        }
    }
}

impl PickerDelegate for KillRingPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if self.matches.is_empty() {
            self.selected_index = 0;
            self.revert_preview(window, cx);
            return;
        }

        let ix = ix.min(self.matches.len() - 1);
        if self.selected_index == ix && self.preview.is_some() {
            return;
        }

        self.revert_preview(window, cx);
        self.selected_index = ix;
        self.apply_preview(window, cx);
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Yank from Kill Ring...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let candidates = self.candidates();
        let background = cx.background_executor().clone();

        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                candidates
                    .iter()
                    .map(|candidate| StringMatch {
                        candidate_id: candidate.id,
                        score: 0.,
                        positions: Vec::new(),
                        string: candidate.string.clone(),
                    })
                    .collect()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    candidates.len(),
                    &AtomicBool::new(false),
                    background,
                )
                .await
            };

            this.update_in(cx, |picker, window, cx| {
                picker.delegate.matches = matches;
                picker.delegate.selected_index = picker
                    .delegate
                    .selected_index
                    .min(picker.delegate.matches.len().saturating_sub(1));
                picker.delegate.revert_preview(window, cx);
                if !picker.delegate.matches.is_empty() {
                    picker.delegate.apply_preview(window, cx);
                }
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry_index) = self.selected_entry_index() else {
            self.dismiss_modal(true, window, cx);
            return;
        };
        let Some(preview) = self.preview.take() else {
            self.dismiss_modal(true, window, cx);
            return;
        };

        self.active_editor
            .update(cx, |editor, cx| {
                editor.commit_kill_ring_yank_preview(entry_index, preview, cx);
            })
            .log_err();
        self.dismiss_modal(true, window, cx);
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.revert_preview(window, cx);
        self.dismiss_modal(false, window, cx);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry_match = self.matches.get(ix)?;
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    entry_match.string.clone(),
                    entry_match.positions.clone(),
                )),
        )
    }
}

fn single_line_preview(text: &str) -> String {
    text.chars()
        .map(|character| match character {
            '\n' | '\r' => '⏎',
            _ => character,
        })
        .collect()
}
