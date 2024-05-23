use std::cmp::Ordering;

use collections::HashMap;
use editor::scroll::Autoscroll;
use editor::{DisplayPoint, Editor};
use gpui::{
    actions, impl_actions, AppContext, EntityId, FocusableView, Global, HighlightStyle, KeyContext,
    KeystrokeEvent, Subscription, View, ViewContext, WeakView,
};
use perm::{Trie, TrimResult};
use serde::Deserialize;
use text::SelectionGoal;
use ui::{BorrowAppContext, WindowContext};
use workspace::Workspace;

use crate::util::manh_distance;

mod editor_events;
mod perm;
mod util;

#[derive(Eq, PartialEq, Copy, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Direction {
    /// Only include the conversation.
    BiDirectional,
    /// Send the current file as context.
    Forwards,
    /// Search the codebase and send relevant excerpts.
    Backwards,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct NChar {
    direction: Direction,
    n: u32,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Pattern(Direction);

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Word(Direction);

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct SubWord(Direction);

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct FullWord(Direction);

impl_actions!(easy_motion, [NChar, Pattern, Word, SubWord, FullWord]);

actions!(easy_motion, [Cancel]);

#[derive(Debug, Default)]
pub(crate) struct EditorState {
    pub control: bool,
    current_trie: Option<Trie>,
}

#[derive(Default)]
pub struct EasyMotion {
    active_editor: Option<WeakView<Editor>>,
    editor_subscription: Option<Subscription>,
    enabled: bool,
    editor_states: HashMap<EntityId, EditorState>,
}

impl Global for EasyMotion {}

pub fn init(cx: &mut AppContext) {
    let mut easy = EasyMotion::default();
    easy.enabled = true;
    cx.set_global(easy);
    cx.observe_keystrokes(observe_keystrokes).detach();
    cx.observe_new_views(|workspace: &mut Workspace, cx| register(workspace, cx))
        .detach();

    editor_events::init(cx);
}

fn register(workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>) {
    workspace.register_action(|_: &mut Workspace, _action: &NChar, cx| {
        EasyMotion::update(cx, |easy, cx| {
            easy.easy_motion_n_char(cx);
        });
    });
    workspace.register_action(|_: &mut Workspace, _action: &Word, cx| {
        EasyMotion::update(cx, |easy, cx| {
            easy.easy_motion_word(cx);
            easy.sync(cx);
        });
    });
    workspace.register_action(|_: &mut Workspace, _action: &Pattern, cx| {
        EasyMotion::update(cx, |easy, cx| {
            easy.easy_motion_pattern(cx);
        });
    });
    workspace.register_action(|_: &mut Workspace, _action: &SubWord, cx| {
        EasyMotion::update(cx, |easy, cx| {
            easy.easy_motion_sub_word(cx);
        });
    });
    workspace.register_action(|_: &mut Workspace, _action: &FullWord, cx| {
        EasyMotion::update(cx, |easy, cx| {
            easy.easy_motion_full_word(cx);
        });
    });
    workspace.register_action(|_: &mut Workspace, _action: &Cancel, cx| {
        EasyMotion::update(cx, |easy, cx| {
            easy.easy_motion_cancel(cx);
            easy.sync(cx);
        });
    });
}

fn observe_keystrokes(keystroke_event: &KeystrokeEvent, cx: &mut WindowContext) {
    if keystroke_event.action.is_some() {
        return;
    } else if cx.has_pending_keystrokes() {
        return;
    }

    let keys = keystroke_event.keystroke.key.as_str();
    EasyMotion::update(cx, |easy, cx| {
        easy.keystroke(keys, cx);
    });
}

impl EasyMotion {
    fn update<F, S>(cx: &mut WindowContext, update: F) -> S
    where
        F: FnOnce(&mut Self, &mut WindowContext) -> S,
    {
        cx.update_global(update)
    }

    #[allow(dead_code)]
    fn read(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }

    fn update_easy_and_active_editor<S>(
        &mut self,
        cx: &mut WindowContext,
        update: impl FnOnce(&mut EasyMotion, &mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        let editor = self.active_editor.clone()?.upgrade()?;
        Some(editor.update(cx, |editor, cx| update(self, editor, cx)))
    }

    fn update_active_editor<S>(
        &self,
        cx: &mut WindowContext,
        update: impl FnOnce(&EasyMotion, &mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        let editor = self.active_editor.clone()?.upgrade()?;
        Some(editor.update(cx, |editor, cx| update(self, editor, cx)))
    }

    fn activate_editor(&mut self, editor: View<Editor>, _cx: &mut WindowContext) {
        self.active_editor = Some(editor.downgrade());
    }

    // fn read_active_editor<'a>(&mut self, cx: &'a mut WindowContext) -> Option<&'a Editor> {
    //     let editor = self.active_editor.clone()?.upgrade()?;
    //     Some(editor.read(cx))
    // }

    fn sync(&mut self, cx: &mut WindowContext) {
        self.update_easy_and_active_editor(cx, |easy, editor, cx| {
            let state = easy.state();
            if editor.is_focused(cx) && state.is_some() {
                let ctx = state.unwrap().keymap_context_layer();
                editor.set_keymap_context_layer::<Self>(ctx, cx);
                // disable vim mode if a sub-editor (inline assist, rename, etc.) is focused
            } else if editor.focus_handle(cx).contains_focused(cx) {
                editor.remove_keymap_context_layer::<Self>(cx);
            }
        });
    }

    fn new_state(&mut self) -> &EditorState {
        self.active_editor
            .as_ref()
            .map(|active_editor| {
                self.editor_states
                    .insert(active_editor.entity_id(), EditorState::default());
                self.editor_states.get(&active_editor.entity_id()).unwrap()
            })
            .unwrap()
    }

    fn clear_state(&mut self) {
        self.update_state(|state| *state = EditorState::default());
    }

    fn state(&self) -> Option<&EditorState> {
        self.active_editor
            .as_ref()
            .map(|active_editor| self.editor_states.get(&active_editor.entity_id()))
            .flatten()
    }

    fn update_state<T>(&mut self, func: impl FnOnce(&mut EditorState) -> T) -> Option<T> {
        let state = self
            .active_editor
            .as_ref()
            .map(|active_editor| self.editor_states.get_mut(&active_editor.entity_id()))
            .flatten()?;
        let ret = func(state);
        Some(ret)
    }

    fn insert_state(&mut self, state: EditorState) -> Option<()> {
        let active_editor = self.active_editor.as_ref()?;
        self.editor_states.insert(active_editor.entity_id(), state);
        Some(())
    }

    // fn active_editor_input_ignored(text: Arc<str>, cx: &mut WindowContext) {
    fn easy_motion_n_char(&mut self, _cx: &mut WindowContext) {}

    fn easy_motion_word(&mut self, cx: &mut WindowContext) {
        self.update_easy_and_active_editor(cx, |easy, editor, view_cx| {
            let selections = editor.selections.newest_display(view_cx);
            let map = &editor.snapshot(view_cx).display_snapshot;

            let start = selections.start;

            let highlight = HighlightStyle {
                background_color: Some(gpui::red()),
                ..Default::default()
            };
            let mut word_starts = util::word_starts(&map, start, true, 30);
            word_starts.sort_unstable_by(|a, b| {
                let a_distance = manh_distance(a, &start, 2.0);
                let b_distance = manh_distance(b, &start, 2.0);
                if a_distance == b_distance {
                    Ordering::Equal
                } else if a_distance < b_distance {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            let trie = Trie::new("asdghklqwertyuiopzxcvbnmfj".to_string(), word_starts, true);
            let perms = trie.trie_to_perms();
            for (seq, point) in perms {
                editor.add_overlay(seq.to_string(), point.to_owned(), highlight, view_cx);
            }
            easy.insert_state(EditorState {
                control: true,
                current_trie: Some(trie),
            })
            .unwrap();
        });
    }
    fn easy_motion_pattern(&mut self, _cx: &mut WindowContext) {}
    fn easy_motion_sub_word(&mut self, _cx: &mut WindowContext) {}
    fn easy_motion_full_word(&mut self, _cx: &mut WindowContext) {}
    fn easy_motion_cancel(&mut self, cx: &mut WindowContext) {
        self.clear_state();
        self.update_active_editor(cx, |_, editor, cx| {
            editor.clear_overlays(cx);
        });
    }

    fn keystroke(&mut self, keys: &str, cx: &mut WindowContext) {
        let Some(state) = self.state() else {
            return;
        };
        if !state.control {
            return;
        }

        let res = self.update_state(|state| state.record_str(keys)).unwrap();
        match res {
            TrimResult::Found(point) => {
                self.move_cursor(point, cx);
                self.clear_state();
                self.update_active_editor(cx, |_, editor, cx| {
                    editor.clear_overlays(cx);
                });
                self.sync(cx);
            }
            TrimResult::Changed => {
                let trie = self
                    .state()
                    .map(|state| state.current_trie.as_ref())
                    .flatten();
                self.update_active_editor(cx, |_, editor, cx| {
                    editor.clear_overlays(cx);
                    let Some(trie) = trie else {
                        return;
                    };
                    let perms = trie.trie_to_perms();
                    let highlight = HighlightStyle {
                        background_color: Some(gpui::red()),
                        ..Default::default()
                    };
                    for (seq, point) in perms {
                        editor.add_overlay(seq.to_string(), point.to_owned(), highlight, cx);
                    }
                });
            }
            TrimResult::Err => {
                self.clear_state();
                self.update_active_editor(cx, |_, editor, cx| {
                    editor.clear_overlays(cx);
                });
                self.sync(cx);
            }
            TrimResult::NoChange => {}
        }
    }

    fn move_cursor(&mut self, point: DisplayPoint, cx: &mut WindowContext) {
        self.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |selection| {
                selection.move_cursors_with(|_, _, _| (point, SelectionGoal::None))
            });
        });
    }
}

impl EditorState {
    #[allow(dead_code)]
    fn trie(&self) -> Option<&Trie> {
        self.current_trie.as_ref()
    }

    fn easy_motion_controlled(&self) -> bool {
        return self.control;
    }

    fn keymap_context_layer(&self) -> KeyContext {
        let mut context = KeyContext::new_with_defaults();
        if self.easy_motion_controlled() {
            context.add("EasyMotionControlled");
            context.add("menu");
        }
        return context;
    }

    fn record_char(&mut self, character: char) -> TrimResult {
        let Some(trie) = &mut self.current_trie else {
            return TrimResult::Err;
        };
        trie.trim(character)
    }

    fn record_str(&mut self, characters: &str) -> TrimResult {
        let mut changed = false;
        for character in characters.chars() {
            let ret = self.record_char(character);
            match ret {
                TrimResult::NoChange => {}
                TrimResult::Changed => changed = true,
                TrimResult::Found(_) => return ret,
                TrimResult::Err => return ret,
            };
        }
        if changed {
            TrimResult::Changed
        } else {
            TrimResult::NoChange
        }
    }
}
