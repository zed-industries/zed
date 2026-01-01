use std::sync::Arc;

use editor::{
    Editor,
    code_context_menus::{CodeActionContents, CodeActionsItem},
};
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, Task,
    WeakEntity, Window,
};
use language::Buffer;
use picker::{Picker, PickerDelegate};
use task::TaskContext;
use ui::{ListItem, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Toast, Workspace, notifications::NotificationId, ui::HighlightedLabel};

pub fn init(cx: &mut App) {
    cx.observe_new(CodeActionsPicker::register).detach();
}

pub fn toggle(
    editor: Entity<Editor>,
    _: &zed_actions::ToggleCodeActionsPicker,
    window: &mut Window,
    cx: &mut App,
) {
    let workspace = window.root::<Workspace>().flatten();
    let Some(workspace) = workspace else {
        log::warn!("Cannot open code actions picker: no workspace found");
        return;
    };

    let editor_weak = editor.downgrade();
    window
        .spawn(cx, async move |cx| {
            let gather_task = editor.update_in(cx, |editor, window, cx| {
                editor.gather_code_actions(window, cx)
            })?;

            let Some((buffer, actions)) = gather_task.await else {
                workspace
                    .update_in(cx, |workspace, _window, cx| {
                        workspace.show_toast(
                            Toast::new(
                                NotificationId::unique::<CodeActionsPicker>(),
                                "No code actions available",
                            ),
                            cx,
                        );
                    })
                    .log_err();
                return anyhow::Ok(());
            };

            workspace.update_in(cx, |workspace, window, cx| {
                let context = actions.context.clone();
                workspace.toggle_modal(window, cx, |window, cx| {
                    let delegate = CodeActionsPickerDelegate::new(
                        cx.entity().downgrade(),
                        editor_weak,
                        buffer,
                        actions,
                        context,
                    );
                    CodeActionsPicker::new(delegate, window, cx)
                });
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
}

pub struct CodeActionsPicker {
    picker: Entity<Picker<CodeActionsPickerDelegate>>,
}

impl CodeActionsPicker {
    fn register(editor: &mut Editor, _: Option<&mut Window>, cx: &mut Context<Editor>) {
        if editor.mode().is_full() {
            let handle = cx.entity().downgrade();
            editor
                .register_action(move |action, window, cx| {
                    if let Some(editor) = handle.upgrade() {
                        toggle(editor, action, window, cx);
                    }
                })
                .detach();
        }
    }

    pub fn new(
        delegate: CodeActionsPickerDelegate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl ModalView for CodeActionsPicker {}

impl EventEmitter<DismissEvent> for CodeActionsPicker {}

impl Focusable for CodeActionsPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for CodeActionsPicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("CodeActionsPicker")
            .w(rems(34.))
            .on_action(cx.listener(
                |_this: &mut CodeActionsPicker,
                 _: &zed_actions::ToggleCodeActionsPicker,
                 _window: &mut Window,
                 cx: &mut Context<CodeActionsPicker>| {
                    cx.emit(DismissEvent);
                },
            ))
            .child(self.picker.clone())
    }
}

pub struct CodeActionsPickerDelegate {
    all_items: Vec<CodeActionsItem>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    buffer: Entity<Buffer>,
    context: TaskContext,
    editor: WeakEntity<Editor>,
    picker: WeakEntity<CodeActionsPicker>,
}

impl CodeActionsPickerDelegate {
    pub fn new(
        picker: WeakEntity<CodeActionsPicker>,
        editor: WeakEntity<Editor>,
        buffer: Entity<Buffer>,
        actions: CodeActionContents,
        context: TaskContext,
    ) -> Self {
        let all_items: Vec<CodeActionsItem> = actions.iter().collect();
        let matches = all_items
            .iter()
            .enumerate()
            .map(|(index, item)| StringMatch {
                candidate_id: index,
                string: format!("{}. {}", index + 1, item.label()),
                positions: Vec::new(),
                score: 0.0,
            })
            .collect();

        Self {
            all_items,
            matches,
            selected_index: 0,
            buffer,
            context,
            editor,
            picker,
        }
    }
}

impl PickerDelegate for CodeActionsPickerDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Filter actions...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates: Vec<_> = self
            .all_items
            .iter()
            .enumerate()
            .map(|(id, item)| {
                let numbered_label = format!("{}. {}", id + 1, item.label());
                StringMatchCandidate::new(id, &numbered_label)
            })
            .collect();

        cx.spawn(async move |this, cx| {
            let matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
                        string: candidate.string,
                        positions: Vec::new(),
                        score: 0.0,
                    })
                    .collect()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(cx, |this, cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
                cx.notify();
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(string_match) = self.matches.get(self.selected_index) else {
            self.picker
                .update(cx, |_, cx| cx.emit(DismissEvent))
                .log_err();
            return;
        };

        let Some(item) = self.all_items.get(string_match.candidate_id).cloned() else {
            log::warn!(
                "Invalid candidate_id {} in code actions picker",
                string_match.candidate_id
            );
            self.picker
                .update(cx, |_, cx| cx.emit(DismissEvent))
                .log_err();
            return;
        };

        let buffer = self.buffer.clone();
        let context = self.context.clone();

        self.editor
            .update(cx, |editor, cx| {
                if let Some(task) = editor.apply_code_action_item(item, buffer, context, window, cx)
                {
                    task.detach_and_log_err(cx);
                }
            })
            .log_err();

        self.picker
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.picker
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let string_match = self.matches.get(ix)?;
        let colors = cx.theme().colors();

        // Action labels may contain newlines which would break the single-line display
        let label = string_match.string.replace("\n", "");

        Some(
            ListItem::new(ix).inset(true).toggle_state(selected).child(
                h_flex()
                    .overflow_hidden()
                    .child(HighlightedLabel::new(label, string_match.positions.clone()))
                    .when(selected, |this| this.text_color(colors.text_accent)),
            ),
        )
    }
}
