use collections::HashMap;
use project::search::SearchQuery;
use serde::Deserialize;
use std::cmp::Ordering;
use std::{fmt, mem};

use editor::scroll::Autoscroll;
use editor::{DisplayPoint, Editor};
use gpui::{
    actions, impl_actions, AppContext, EntityId, FocusableView, Global, HighlightStyle,
    KeystrokeEvent, Model, ModelContext, Subscription, View, ViewContext, WeakView,
};
use perm::{Trie, TrieBuilder, TrimResult};
use settings::Settings;
use text::{Bias, SelectionGoal};
use theme::ThemeSettings;
use ui::{BorrowAppContext, Context, VisualContext, WindowContext};
use workspace::searchable::SearchableItem;
use workspace::Workspace;

use editor_state::{EditorState, InputResult, NCharInput, OverlayState, Selection};
use util::{end_of_document, manh_distance, window_bottom, window_top};

mod editor_events;
mod editor_state;
mod perm;
mod search;
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

enum WordType {
    Word,
    SubWord,
    FullWord,
}

#[derive(Default)]
pub struct EasyMotion {
    active_editor: Option<WeakView<Editor>>,
    editor_subscription: Option<Subscription>,
    dimming: bool,
    enabled: bool,
    editor_states: HashMap<EntityId, EditorState>,
}

impl fmt::Debug for EasyMotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EasyMotion")
            .field("active_editor", &self.active_editor)
            .field("dimming", &self.dimming)
            .field("enabled", &self.enabled)
            .field("editor_states(members)", &self.editor_states)
            .finish()
    }
}

struct GlobalEasyMotion(Model<EasyMotion>);

impl Global for GlobalEasyMotion {}

pub fn init(cx: &mut AppContext) {
    let easy = cx.new_model({
        move |cx| {
            let mut easy = EasyMotion::default();
            easy.enabled = true;
            easy.dimming = true;
            easy
        }
    });
    EasyMotion::set_global(easy.clone(), cx);
    cx.observe_keystrokes(EasyMotion::observe_keystrokes)
        .detach();
    cx.observe_new_views(|workspace: &mut Workspace, cx| register(workspace, cx))
        .detach();

    editor_events::init(cx);
}

fn register(workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>) {
    workspace.register_action(|_workspace: &mut Workspace, action: &Word, cx| {
        let direction = action.0;
        EasyMotion::word(WordType::Word, direction, cx);
    });
    workspace.register_action(|_: &mut Workspace, action: &SubWord, cx| {
        let direction = action.0;
        EasyMotion::word(WordType::SubWord, direction, cx);
    });
    workspace.register_action(|_: &mut Workspace, action: &FullWord, cx| {
        let direction = action.0;
        EasyMotion::word(WordType::FullWord, direction, cx);
    });

    workspace.register_action(|_: &mut Workspace, _action: &NChar, cx| {
        // EasyMotion::update(cx, |easy, cx| {
        //     easy.easy_motion_n_char(cx);
        // });
    });

    workspace.register_action(|_: &mut Workspace, _action: &Pattern, cx| {
        // EasyMotion::update(cx, |easy, cx| {
        //     easy.easy_motion_pattern(cx);
        // });
    });

    workspace.register_action(|_: &mut Workspace, _action: &Cancel, cx| {
        // EasyMotion::update(cx, |easy, cx| {
        //     easy.easy_motion_cancel(cx);
        //     easy.sync(cx);
        // });
    });
}

impl EasyMotion {
    pub fn update<F, S>(cx: &mut AppContext, f: F) -> Option<S>
    where
        F: FnOnce(&mut EasyMotion, &mut ModelContext<EasyMotion>) -> S,
    {
        EasyMotion::global(cx).map(|easy| easy.update(cx, f))
    }

    pub fn global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<GlobalEasyMotion>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(easy: Model<Self>, cx: &mut AppContext) {
        cx.set_global(GlobalEasyMotion(easy));
    }

    pub fn read_with<S>(
        cx: &mut ViewContext<Workspace>,
        f: impl FnOnce(&EasyMotion, &AppContext) -> S,
    ) -> Option<S> {
        EasyMotion::global(cx).map(|easy| easy.read_with(cx, f))
    }

    #[allow(dead_code)]
    fn update_active_editor<S>(
        &self,
        cx: &mut WindowContext,
        update: impl FnOnce(&EasyMotion, &mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        let editor = self.active_editor.clone()?.upgrade()?;
        Some(editor.update(cx, |editor, cx| update(self, editor, cx)))
    }

    fn activate_editor(&mut self, editor: View<Editor>) {
        self.active_editor = Some(editor.downgrade());
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

    #[allow(dead_code)]
    fn clear_state(&mut self) {
        self.insert_state(EditorState::None);
    }

    #[allow(dead_code)]
    fn state(&self) -> Option<&EditorState> {
        self.active_editor
            .as_ref()
            .map(|active_editor| self.editor_states.get(&active_editor.entity_id()))
            .flatten()
    }

    #[allow(dead_code)]
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

    fn take_state(&mut self) -> Option<EditorState> {
        self.active_editor.as_ref().map(|active_editor| {
            self.editor_states
                .get_mut(&active_editor.entity_id())
                .map(|state| mem::take(state))
                .unwrap_or_default()
        })
    }

    fn word(word_type: WordType, direction: Direction, cx: &mut ViewContext<Workspace>) {
        let weak_editor =
            EasyMotion::read_with(cx, |easy, _cx| easy.active_editor.clone()).flatten();
        let Some(weak_editor) = weak_editor else {
            return;
        };
        let entity_id = weak_editor.entity_id();

        let editor = weak_editor.upgrade();
        let Some(editor) = editor else {
            return;
        };

        let new_state = editor.update(cx, |editor, cx| {
            let new_state = EasyMotion::word_impl(true, word_type, direction, editor, cx);
            let ctx = new_state.keymap_context_layer();
            editor.set_keymap_context_layer::<EasyMotion>(ctx, cx);
            new_state
        });

        EasyMotion::update(cx, move |easy, _cx| {
            easy.editor_states.insert(entity_id, new_state);
        });
    }

    fn word_impl(
        dimming: bool,
        word_type: WordType,
        direction: Direction,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> EditorState {
        let selections = editor.selections.newest_display(cx);
        let snapshot = editor.snapshot(cx);
        let map = &snapshot.display_snapshot;
        let text_layout_details = editor.text_layout_details(cx);

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

        let full_word = match word_type {
            WordType::Word => false,
            WordType::FullWord => true,
            _ => false,
        };
        let mut word_starts = util::word_starts_in_range(&map, start, end, full_word);
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

        if word_starts.is_empty() {
            return EditorState::None;
        }

        let settings = ThemeSettings::get_global(cx);
        let players = &settings.active_theme.players().0;
        let style_0 = HighlightStyle {
            color: Some(players[0].cursor),
            ..HighlightStyle::default()
        };
        let style_1 = HighlightStyle {
            color: Some(players[2].cursor),
            ..HighlightStyle::default()
        };
        let style_2 = HighlightStyle {
            color: Some(players[3].cursor),
            ..HighlightStyle::default()
        };
        let trie = TrieBuilder::new("asdghklqwertyuiopzxcvbnmfj".to_string(), word_starts.len())
            .populate_with(true, word_starts.into_iter(), |seq, point| {
                let style = match seq.len() {
                    0 | 1 => style_0,
                    2 => style_1,
                    3.. => style_2,
                };
                OverlayState { style, point }
            });
        EasyMotion::add_overlays(editor, &trie, cx);

        if dimming {
            let start = match direction {
                Direction::BiDirectional | Direction::Backwards => DisplayPoint::zero(),
                Direction::Forwards => selections.start,
            };
            let end = match direction {
                Direction::BiDirectional | Direction::Forwards => end_of_document(map),
                Direction::Backwards => selections.end,
            };
            let anchor_start = map.display_point_to_anchor(start, Bias::Left);
            let anchor_end = map.display_point_to_anchor(end, Bias::Left);
            let highlight = HighlightStyle {
                fade_out: Some(0.7),
                ..Default::default()
            };
            editor.highlight_text::<EasyMotion>(vec![anchor_start..anchor_end], highlight, cx);
        }

        EditorState::Selection(Selection::new(trie))
    }

    #[allow(dead_code)]
    fn easy_motion_pattern(&mut self, _cx: &mut WindowContext) {}

    // fn active_editor_input_ignored(text: Arc<str>, cx: &mut WindowContext) {
    #[allow(dead_code)]
    fn easy_motion_n_char(&mut self, _cx: &mut WindowContext) {}

    #[allow(dead_code)]
    fn easy_motion_cancel(&mut self, cx: &mut WindowContext) {
        self.clear_state();
        self.update_active_editor(cx, |_, editor, cx| {
            editor.clear_overlays(cx);
            editor.clear_highlights::<EasyMotion>(cx);
        });
    }

    fn observe_keystrokes(keystroke_event: &KeystrokeEvent, cx: &mut WindowContext) {
        if keystroke_event.action.is_some() {
            return;
        } else if cx.has_pending_keystrokes() {
            return;
        }

        let Some((state, weak_editor)) = Self::update(cx, |easy, _cx| {
            let state = easy.take_state();
            let weak_editor = easy.active_editor.clone();
            state.zip(weak_editor)
        })
        .flatten() else {
            return;
        };

        if !state.easy_motion_controlled() {
            return;
        }

        let entity_id = weak_editor.entity_id();
        let editor = weak_editor.upgrade();
        let Some(editor) = editor else {
            return;
        };

        let keys = keystroke_event.keystroke.key.as_str();
        let new_state = editor.update(cx, |editor, cx| match state {
            EditorState::NCharInput(char_input) => Self::handle_record_char(char_input, keys, cx),
            EditorState::Selection(selection) => Self::handle_trim(selection, keys, editor, cx),
            EditorState::None => EditorState::None,
        });

        Self::update(cx, move |easy, _cx| {
            easy.editor_states.insert(entity_id, new_state);
        });
    }

    fn handle_trim(
        selection: Selection,
        keys: &str,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> EditorState {
        let (selection, res) = selection.record_str(keys);
        match res {
            TrimResult::Found(point) => {
                editor.change_selections(Some(Autoscroll::fit()), cx, |selection| {
                    selection.move_cursors_with(|_, _, _| (point, SelectionGoal::None))
                });
                editor.clear_overlays(cx);
                editor.clear_highlights::<EasyMotion>(cx);
                editor.remove_keymap_context_layer::<Self>(cx);
                EditorState::None
            }
            TrimResult::Changed => {
                let trie = selection.trie();
                editor.clear_overlays(cx);
                EasyMotion::add_overlays(editor, trie, cx);
                EditorState::Selection(selection)
            }
            TrimResult::Err => {
                editor.clear_overlays(cx);
                editor.clear_highlights::<EasyMotion>(cx);
                editor.remove_keymap_context_layer::<Self>(cx);
                EditorState::None
            }
            TrimResult::NoChange => EditorState::Selection(selection),
        }
    }

    fn handle_record_char(n_char: NCharInput, keys: &str, cx: &mut WindowContext) -> EditorState {
        let res = n_char.record_str(keys);
        match res {
            InputResult::ShowTrie(query) => {
                // maybe just have a whole separate struct for the search version?
                // model? view?
                // let chan = self.update_easy_and_active_editor(cx, |easy, editor, cx| {
                //     cx.spawn(|view, cx| async move {
                //         EasyMotion::update(cx, |easy, window| {});
                //     });
                // });

                // search
                // create trie
                // show overlays
                // etc
                EditorState::None
            }
            // do nothing
            InputResult::Recording(n_char) => EditorState::NCharInput(n_char),
        }
    }

    #[allow(dead_code)]
    fn handle_query(query: String) {}

    fn add_overlays(editor: &mut Editor, trie: &Trie<OverlayState>, cx: &mut ViewContext<Editor>) {
        let settings = ThemeSettings::get_global(cx);
        // if not doing direct overlays
        // let background = None;
        let background = settings.active_theme.colors().background;
        for (seq, overlay) in trie.iter() {
            let mut highlights = vec![(
                0..1,
                HighlightStyle {
                    background_color: Some(background),
                    ..overlay.style.clone()
                },
            )];
            if seq.len() > 1 {
                highlights.push((
                    1..seq.len(),
                    HighlightStyle {
                        background_color: Some(background),
                        fade_out: Some(0.3),
                        ..overlay.style.clone()
                    },
                ));
            }
            editor.add_overlay(seq, overlay.point.clone(), 0.0, highlights, cx);
        }
    }
}
