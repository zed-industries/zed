use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
use std::time::Duration;

use crate::Editor;
use db::kvp::KEY_VALUE_STORE;
use futures::channel::oneshot;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render,
    Styled, TextAlign, TextStyleRefinement,
    Subscription, Task, WeakEntity, Window, rems,
};
use picker::{Picker, PickerDelegate};
use parking_lot::Mutex;
use ui::{Button, KeyBinding, ListItem, ListItemSpacing, Toggleable, h_flex, prelude::*, v_flex};
use util::ResultExt;
use workspace::ModalView;

const ONESHOT_HISTORY_NAMESPACE: &str = "editor";
const ONESHOT_HISTORY_KEY: &str = "unix_command_history";
const MAX_HISTORY_ITEMS: usize = 500;
const DEFAULT_TIMEOUT_SECONDS: u64 = 10;
const MIN_TIMEOUT_SECONDS: u64 = 1;

pub(crate) struct UnixCommandExecution {
    pub(crate) cancel_requested: Arc<AtomicBool>,
    pub(crate) completion: oneshot::Receiver<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnixCommandOutputMode {
    CopyOutput,
    NewDocument,
    ReplaceSelection,
}

pub struct UnixCommandModal {
    picker: Entity<Picker<UnixCommandModalDelegate>>,
    _subscription: Subscription,
}

struct SharedCommandState {
    timeout_seconds: AtomicU64,
    is_running: AtomicBool,
    cancel_requested: Mutex<Option<Arc<AtomicBool>>>,
}

impl SharedCommandState {
    fn new() -> Self {
        Self {
            timeout_seconds: AtomicU64::new(DEFAULT_TIMEOUT_SECONDS),
            is_running: AtomicBool::new(false),
            cancel_requested: Mutex::new(None),
        }
    }

    fn timeout_seconds(&self) -> u64 {
        self.timeout_seconds.load(Ordering::Relaxed)
    }
}

impl UnixCommandModal {
    pub fn new(
        editor: WeakEntity<Editor>,
        mode: UnixCommandOutputMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let modal = cx.entity().downgrade();
        let shared_state = Arc::new(SharedCommandState::new());
        let picker = cx.new(|cx| {
            Picker::uniform_list(
                UnixCommandModalDelegate::new(
                    modal.clone(),
                    editor.clone(),
                    mode,
                    shared_state.clone(),
                    window,
                    cx,
                ),
                window,
                cx,
            )
            .modal(false)
        });
        let weak_picker = picker.downgrade();
        picker.update(cx, |picker, _| {
            picker.delegate.set_picker(weak_picker);
        });

        let _subscription = cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            _subscription,
        }
    }
}

impl Focusable for UnixCommandModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

#[cfg(test)]
impl UnixCommandModal {
    pub(crate) fn is_running_for_test(&self, cx: &App) -> bool {
        self.picker.read(cx).delegate.is_running()
    }

    pub(crate) fn seed_running_state_for_test(&self, cx: &mut App) -> Arc<AtomicBool> {
        let cancel_requested = Arc::new(AtomicBool::new(false));
        let cancel_requested_for_delegate = cancel_requested.clone();

        self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .shared_state
                .is_running
                .store(true, Ordering::Relaxed);
            picker
                .delegate
                .shared_state
                .cancel_requested
                .lock()
                .replace(cancel_requested_for_delegate);
            cx.notify();
        });

        cancel_requested
    }

    pub(crate) fn stop_for_test(&self, cx: &mut App) {
        self.picker.update(cx, |picker, cx| {
            if let Some(cancel_requested) = picker
                .delegate
                .shared_state
                .cancel_requested
                .lock()
                .as_ref()
                .cloned()
            {
                cancel_requested.store(true, Ordering::Relaxed);
            }
            cx.notify();
        });
    }
}

impl EventEmitter<DismissEvent> for UnixCommandModal {}
impl ModalView for UnixCommandModal {}

impl Render for UnixCommandModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("UnixCommandModal")
            .w(rems(40.))
            .elevation_3(cx)
            .bg(cx.theme().colors().elevated_surface_background)
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .rounded_lg()
            .overflow_hidden()
            .child(self.picker.clone())
    }
}

struct UnixCommandModalDelegate {
    modal: WeakEntity<UnixCommandModal>,
    picker: Option<WeakEntity<Picker<UnixCommandModalDelegate>>>,
    editor: WeakEntity<Editor>,
    mode: UnixCommandOutputMode,
    timeout_input: Entity<Editor>,
    history: Vec<String>,
    prompt: String,
    matches: Vec<StringMatch>,
    selected_index: usize,
    shared_state: Arc<SharedCommandState>,
}

impl UnixCommandModalDelegate {
    fn new(
        modal: WeakEntity<UnixCommandModal>,
        editor: WeakEntity<Editor>,
        mode: UnixCommandOutputMode,
        shared_state: Arc<SharedCommandState>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Self {
        let timeout_input = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_text_style_refinement(TextStyleRefinement {
                text_align: Some(TextAlign::Center),
                ..Default::default()
            });
            input.set_text(DEFAULT_TIMEOUT_SECONDS.to_string(), window, cx);
            input
        });

        Self {
            modal,
            picker: None,
            editor,
            mode,
            timeout_input,
            history: load_history(),
            prompt: String::new(),
            matches: Vec::new(),
            selected_index: 0,
            shared_state,
        }
    }

    fn set_picker(&mut self, picker: WeakEntity<Picker<UnixCommandModalDelegate>>) {
        self.picker = Some(picker);
    }

    fn is_running(&self) -> bool {
        self.shared_state.is_running.load(Ordering::Relaxed)
    }

    fn timeout_seconds(&self) -> u64 {
        self.shared_state.timeout_seconds()
    }

    fn update_timeout(
        &self,
        decrease: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if self.is_running() {
            return;
        }

        let current = self.sync_timeout_input(window, cx);

        let timeout_seconds = if decrease {
            decrement_timeout_seconds(current)
        } else {
            increment_timeout_seconds(current)
        };

        self.set_timeout_seconds(timeout_seconds, window, cx);
    }

    fn set_timeout_seconds(
        &self,
        timeout_seconds: u64,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let timeout_seconds = timeout_seconds.max(MIN_TIMEOUT_SECONDS);
        self.shared_state
            .timeout_seconds
            .store(timeout_seconds, Ordering::Relaxed);
        let timeout_text = timeout_seconds.to_string();

        self.timeout_input.update(cx, |editor, cx| {
            if editor.text(cx) != timeout_text {
                editor.set_text(timeout_text.clone(), window, cx);
            }
        });

        cx.notify();
    }

    fn sync_timeout_input(&self, window: &mut Window, cx: &mut Context<Picker<Self>>) -> u64 {
        let input_text = self.timeout_input.read(cx).text(cx);
        if let Some(timeout_seconds) = parse_timeout_seconds(&input_text) {
            self.set_timeout_seconds(timeout_seconds, window, cx);
            timeout_seconds
        } else {
            let current = self.timeout_seconds();
            self.set_timeout_seconds(current, window, cx);
            current
        }
    }

    fn execute_command(
        &mut self,
        command: String,
        omit_history_entry: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if self.is_running() {
            return;
        }

        if command.trim().is_empty() {
            return;
        }

        if !omit_history_entry {
            persist_history(&mut self.history, command.as_str(), cx);
        }

        let timeout_seconds = self.sync_timeout_input(window, cx);
        let timeout = Duration::from_secs(timeout_seconds);
        let mode = self.mode;
        let editor = self.editor.clone();
        let execution = editor
            .update(cx, |editor, cx| {
                editor.execute_selection_unix_command(command, mode, timeout, window, cx)
            })
            .ok()
            .flatten();

        let Some(execution) = execution else {
            return;
        };

        self.shared_state.is_running.store(true, Ordering::Relaxed);
        self.shared_state
            .cancel_requested
            .lock()
            .replace(execution.cancel_requested.clone());
        cx.notify();

        let shared_state = self.shared_state.clone();
        let picker = self.picker.clone();
        let modal = self.modal.clone();
        cx.spawn_in(window, async move |_, cx| {
            let completed_successfully = execution.completion.await.unwrap_or(false);
            shared_state.is_running.store(false, Ordering::Relaxed);
            shared_state.cancel_requested.lock().take();

            if completed_successfully {
                modal
                    .update(cx, |_, cx| {
                        cx.emit(DismissEvent);
                    })
                    .ok();
            }

            if let Some(picker) = picker {
                picker
                    .update(cx, |_, cx| {
                        cx.notify();
                    })
                    .ok();
            }
        })
        .detach();
    }
}

impl PickerDelegate for UnixCommandModalDelegate {
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
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        if self.is_running() {
            return;
        }
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::from("Filter selection through command")
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if self.is_running() {
            return Task::ready(());
        }

        self.prompt = query.clone();

        let history = self.history.clone();
        let candidates = history
            .iter()
            .enumerate()
            .map(|(id, command)| StringMatchCandidate::new(id, command))
            .collect::<Vec<_>>();

        let background = cx.background_executor().clone();
        cx.spawn_in(window, async move |picker, cx| {
            let mut matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .rev()
                    .map(|candidate| StringMatch {
                        candidate_id: candidate.id,
                        string: candidate.string,
                        positions: Vec::new(),
                        score: 0.0,
                    })
                    .collect::<Vec<_>>()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    true,
                    true,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            if !query.trim().is_empty()
                && !matches.iter().any(|candidate| candidate.string == query)
            {
                matches.insert(
                    0,
                    StringMatch {
                        candidate_id: usize::MAX,
                        string: query,
                        positions: Vec::new(),
                        score: 1.0,
                    },
                );
            }

            picker
                .update(cx, |picker, _| {
                    picker.delegate.matches = matches;
                    picker.delegate.selected_index = 0;
                })
                .log_err();
        })
    }

    fn confirm(
        &mut self,
        omit_history_entry: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if self.is_running() {
            return;
        }

        let Some(command) = self
            .matches
            .get(self.selected_index)
            .map(|candidate| candidate.string.clone())
        else {
            return;
        };

        self.execute_command(command, omit_history_entry, window, cx);
    }

    fn confirm_input(
        &mut self,
        omit_history_entry: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if self.is_running() {
            return;
        }

        let command = self.prompt.clone();
        self.execute_command(command, omit_history_entry, window, cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.modal
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let command = self.matches.get(ix)?.string.clone();
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(command)),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
        let action = picker::ConfirmInput { secondary: false }.boxed_clone();
        let is_running = self.is_running();

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .justify_between()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .when(!is_running, |this| {
                    this.child(
                        h_flex()
                            .gap_1()
                            .items_center()
                            .child(Label::new("Timeout (s):"))
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap_0()
                                    .child(
                                        Button::new("decrease-timeout", "-")
                                            .on_click({
                                                let picker = self.picker.clone();
                                                move |_, window, cx| {
                                                    if let Some(picker) = &picker {
                                                        picker
                                                            .update(cx, |picker, cx| {
                                                                picker.delegate.update_timeout(
                                                                    true, window, cx,
                                                                );
                                                            })
                                                            .ok();
                                                    }
                                                }
                                            }),
                                    )
                                    .child(h_flex().w(rems(6.)).child(self.timeout_input.clone()))
                                    .child(
                                        Button::new("increase-timeout", "+")
                                            .on_click({
                                                let picker = self.picker.clone();
                                                move |_, window, cx| {
                                                    if let Some(picker) = &picker {
                                                        picker
                                                            .update(cx, |picker, cx| {
                                                                picker.delegate.update_timeout(
                                                                    false, window, cx,
                                                                );
                                                            })
                                                            .ok();
                                                    }
                                                }
                                            }),
                                    ),
                            ),
                    )
                })
                .child(
                    h_flex()
                        .gap_1()
                        .when(is_running, |this| {
                            this.child(
                                Button::new("stop", "Stop")
                                    .on_click({
                                        let shared_state = self.shared_state.clone();
                                        move |_, _, _cx| {
                                            if let Some(cancel_requested) = shared_state
                                                .cancel_requested
                                                .lock()
                                                .as_ref()
                                                .cloned()
                                            {
                                                cancel_requested.store(true, Ordering::Relaxed);
                                            }
                                        }
                                    }),
                            )
                            .child(
                                Button::new("hide", "Hide")
                                    .on_click({
                                        let modal = self.modal.clone();
                                        move |_, _, cx| {
                                            modal
                                                .update(cx, |_, cx| {
                                                    cx.emit(DismissEvent);
                                                })
                                                .ok();
                                        }
                                    }),
                            )
                        })
                        .when(!is_running, |this| {
                            this.child(
                                Button::new("run", "Run")
                                    .key_binding(KeyBinding::for_action(&*action, cx))
                                    .on_click(move |_, window, cx| {
                                        window.dispatch_action(action.boxed_clone(), cx);
                                    }),
                            )
                        }),
                )
                .into_any_element(),
        )
    }
}

fn load_history() -> Vec<String> {
    KEY_VALUE_STORE
        .scoped(ONESHOT_HISTORY_NAMESPACE)
        .read(ONESHOT_HISTORY_KEY)
        .ok()
        .flatten()
        .and_then(|serialized| serde_json::from_str::<Vec<String>>(&serialized).ok())
        .unwrap_or_default()
}

fn persist_history(
    history: &mut Vec<String>,
    command: &str,
    cx: &mut Context<Picker<UnixCommandModalDelegate>>,
) {
    if !remember_history_entry(history, command) {
        return;
    }

    let Ok(serialized) = serde_json::to_string(history) else {
        return;
    };

    cx.spawn(async move |_picker, _cx| {
        KEY_VALUE_STORE
            .scoped(ONESHOT_HISTORY_NAMESPACE)
            .write(ONESHOT_HISTORY_KEY.to_string(), serialized)
            .await
    })
    .detach_and_log_err(cx);
}

fn remember_history_entry(history: &mut Vec<String>, command: &str) -> bool {
    let trimmed_command = command.trim();
    if trimmed_command.is_empty() {
        return false;
    }

    history.retain(|entry| entry != trimmed_command);
    history.push(trimmed_command.to_string());
    if history.len() > MAX_HISTORY_ITEMS {
        let overflow = history.len() - MAX_HISTORY_ITEMS;
        history.drain(..overflow);
    }

    true
}

fn decrement_timeout_seconds(current: u64) -> u64 {
    current.saturating_sub(1).max(MIN_TIMEOUT_SECONDS)
}

fn increment_timeout_seconds(current: u64) -> u64 {
    current.saturating_add(1).max(MIN_TIMEOUT_SECONDS)
}

fn parse_timeout_seconds(input: &str) -> Option<u64> {
    input
        .trim()
        .parse::<u64>()
        .ok()
        .map(|timeout_seconds| timeout_seconds.max(MIN_TIMEOUT_SECONDS))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remember_history_entry_dedupes_and_trims() {
        let mut history = vec!["echo a".to_string(), "echo b".to_string()];

        assert!(remember_history_entry(&mut history, "  echo a  "));
        assert_eq!(history, vec!["echo b".to_string(), "echo a".to_string()]);

        assert!(!remember_history_entry(&mut history, "   "));
        assert_eq!(history, vec!["echo b".to_string(), "echo a".to_string()]);
    }

    #[test]
    fn test_remember_history_entry_respects_max_size() {
        let mut history = (0..MAX_HISTORY_ITEMS)
            .map(|index| format!("command-{index}"))
            .collect::<Vec<_>>();

        assert!(remember_history_entry(&mut history, "new-command"));
        assert_eq!(history.len(), MAX_HISTORY_ITEMS);
        assert_eq!(history.first().unwrap(), "command-1");
        assert_eq!(history.last().unwrap(), "new-command");
    }

    #[test]
    fn test_timeout_adjustment_functions() {
        assert_eq!(decrement_timeout_seconds(10), 9);
        assert_eq!(decrement_timeout_seconds(1), 1);
        assert_eq!(decrement_timeout_seconds(0), 1);

        assert_eq!(increment_timeout_seconds(10), 11);
        assert_eq!(increment_timeout_seconds(0), 1);
    }

    #[test]
    fn test_parse_timeout_seconds() {
        assert_eq!(parse_timeout_seconds("10"), Some(10));
        assert_eq!(parse_timeout_seconds(" 42 "), Some(42));
        assert_eq!(parse_timeout_seconds("0"), Some(1));
        assert_eq!(parse_timeout_seconds("not-a-number"), None);
    }
}
