use collections::HashMap;
use editor_state::{EditorState, OverlayState};
use serde::Deserialize;
use settings::Settings;
use std::cmp::Ordering;
use theme::ThemeSettings;

use editor::scroll::Autoscroll;
use editor::{DisplayPoint, Editor};
use gpui::{
    actions, impl_actions, AppContext, EntityId, FocusableView, Global, HighlightStyle,
    KeystrokeEvent, Subscription, View, ViewContext, WeakView,
};
use perm::{Trie, TrieBuilder, TrimResult};
use text::{Bias, SelectionGoal};
use ui::{BorrowAppContext, WindowContext};
use workspace::Workspace;

use crate::util::{manh_distance, window_bottom, window_top};

mod editor_events;
mod editor_state;
mod perm;
mod util;

#[derive(Eq, PartialEq, Copy, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Direction {
    BiDirectional,
    Forwards,
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
    workspace.register_action(|_: &mut Workspace, action: &Word, cx| {
        EasyMotion::update(cx, |easy, cx| {
            easy.easy_motion_word(action, cx);
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
                // disable easy if a sub-editor (inline assist, rename, etc.) is focused
            } else if editor.focus_handle(cx).contains_focused(cx) {
                editor.remove_keymap_context_layer::<Self>(cx);
            }
        });
    }

    #[allow(dead_code)]
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

    fn easy_motion_word(&mut self, action: &Word, cx: &mut WindowContext) {
        self.update_easy_and_active_editor(cx, |easy, editor, cx| {
            let selections = editor.selections.newest_display(cx);
            let snapshot = editor.snapshot(cx);
            let map = &snapshot.display_snapshot;
            let text_layout_details = editor.text_layout_details(cx);

            let direction = action.0;
            let start = match direction {
                Direction::BiDirectional | Direction::Backwards => {
                    window_top(map, &text_layout_details)
                }
                Direction::Forwards => selections.end,
            };
            let end = match direction {
                Direction::BiDirectional | Direction::Forwards => {
                    window_bottom(map, &text_layout_details)
                }
                Direction::Backwards => selections.start,
            };

            let mut word_starts = util::word_starts_in_range(&map, start, end, true);
            word_starts.sort_unstable_by(|a, b| {
                let a_distance = manh_distance(a, &selections.start, 2.0);
                let b_distance = manh_distance(b, &selections.start, 2.0);
                if a_distance == b_distance {
                    Ordering::Equal
                } else if a_distance < b_distance {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            let settings = ThemeSettings::get_global(cx);
            let players = &settings.active_theme.players().0;
            let color_0 = players[0].cursor;
            let color_1 = players[2].cursor;
            let color_2 = players[3].cursor;
            let trie =
                TrieBuilder::new("asdghklqwertyuiopzxcvbnmfj".to_string(), word_starts.len())
                    .populate_with(true, word_starts.into_iter(), |seq, point| {
                        let color = match seq.len() {
                            0 | 1 => color_0,
                            2 => color_1,
                            3.. => color_2,
                        };
                        OverlayState { color, point }
                    });
            EasyMotion::add_overlays(editor, &trie, cx);

            easy.insert_state(EditorState {
                control: true,
                current_trie: Some(trie),
            })
            .unwrap();

            let anchor_start = map.display_point_to_anchor(start, Bias::Left);
            let anchor_end = map.display_point_to_anchor(end, Bias::Left);
            // let settings = ThemeSettings::get_global(cx);
            // let muted = settings.active_theme.colors().text_muted;
            let highlight = HighlightStyle {
                fade_out: Some(0.7),
                ..Default::default()
            };
            editor.highlight_text::<EasyMotion>(vec![anchor_start..anchor_end], highlight, cx);
        });
    }
    fn easy_motion_pattern(&mut self, _cx: &mut WindowContext) {}
    fn easy_motion_sub_word(&mut self, _cx: &mut WindowContext) {}
    fn easy_motion_full_word(&mut self, _cx: &mut WindowContext) {}
    fn easy_motion_cancel(&mut self, cx: &mut WindowContext) {
        self.clear_state();
        self.update_active_editor(cx, |_, editor, cx| {
            editor.clear_overlays(cx);
            editor.clear_highlights::<EasyMotion>(cx);
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
                    editor.clear_highlights::<EasyMotion>(cx);
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
                    if let Some(trie) = trie {
                        EasyMotion::add_overlays(editor, trie, cx);
                    } else {
                        editor.clear_highlights::<EasyMotion>(cx);
                    };
                });
            }
            TrimResult::Err => {
                self.clear_state();
                self.update_active_editor(cx, |_, editor, cx| {
                    editor.clear_overlays(cx);
                    editor.clear_highlights::<EasyMotion>(cx);
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

    fn add_overlays(editor: &mut Editor, trie: &Trie<OverlayState>, cx: &mut ViewContext<Editor>) {
        let settings = ThemeSettings::get_global(cx);
        let background = settings.active_theme.colors().background;
        for (seq, overlay) in trie.iter() {
            let mut highlights = vec![(
                0..1,
                HighlightStyle {
                    color: Some(overlay.color),
                    background_color: Some(background),
                    ..Default::default()
                },
            )];
            if seq.len() > 1 {
                highlights.push((
                    1..seq.len(),
                    HighlightStyle {
                        color: Some(overlay.color),
                        background_color: Some(background),
                        fade_out: Some(0.3),
                        ..Default::default()
                    },
                ));
            }
            editor.add_overlay(seq.to_string(), overlay.point.clone(), highlights, cx);
        }
    }
}
