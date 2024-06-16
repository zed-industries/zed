use collections::HashMap;
use search::word_starts_fold;
use serde::Deserialize;
use std::fmt;

use editor::{overlay::Overlay, scroll::Autoscroll, DisplayPoint, Editor, ToPoint};
use gpui::{
    actions, impl_actions, saturate, AppContext, Entity, EntityId, Global, HighlightStyle,
    KeystrokeEvent, Model, ModelContext, View, ViewContext,
};
use settings::Settings;
use text::{Bias, SelectionGoal};
use theme::ThemeSettings;
use ui::{Context, WindowContext};

use crate::{
    easy_motion::{
        editor_state::{EasyMotionState, OverlayState},
        search::{row_starts, sort_matches_display},
        trie::{Trie, TrimResult},
    },
    state::Mode,
    Vim,
};

pub mod editor_state;
mod search;
mod trie;

#[derive(Eq, PartialEq, Copy, Clone, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
enum Direction {
    #[default]
    Both,
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

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Row(Direction);

impl_actions!(easy_motion, [NChar, Pattern, Word, SubWord, FullWord, Row]);

actions!(easy_motion, [Cancel, PatternSubmit]);

#[derive(Clone, Copy, Debug)]
enum WordType {
    Word,
    SubWord,
    FullWord,
}

pub struct EasyMotion {
    keys: String,
    enabled: bool,
    editor_states: HashMap<EntityId, EasyMotionState>,
}

impl fmt::Debug for EasyMotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EasyMotion")
            .field("enabled", &self.enabled)
            .field("editor_states(members)", &self.editor_states)
            .finish()
    }
}

struct GlobalEasyMotion(Model<EasyMotion>);

impl Global for GlobalEasyMotion {}

const DEFAULT_KEYS: &'static str = "asdghklqwertyuiopzxcvbnmfj";

pub fn init(cx: &mut AppContext) {
    let easy = cx.new_model({
        |_| EasyMotion {
            editor_states: HashMap::default(),
            enabled: true,
            keys: DEFAULT_KEYS.into(),
        }
    });
    EasyMotion::set_global(easy.clone(), cx);
    cx.observe_new_views(|editor: &mut Editor, cx| register(editor, cx))
        .detach();
}

fn register(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
    let view = cx.view().downgrade();
    editor
        .register_action(move |action: &Word, cx| {
            let Some(editor) = view.upgrade() else {
                return;
            };
            EasyMotion::word(editor, action, cx);
        })
        .detach();

    let view = cx.view().downgrade();
    editor
        .register_action(move |action: &SubWord, cx| {
            let Some(editor) = view.upgrade() else {
                return;
            };
            EasyMotion::sub_word(editor, action, cx);
        })
        .detach();

    let view = cx.view().downgrade();
    editor
        .register_action(move |action: &FullWord, cx| {
            let Some(editor) = view.upgrade() else {
                return;
            };
            EasyMotion::full_word(editor, action, cx);
        })
        .detach();

    let view = cx.view().downgrade();
    editor
        .register_action(move |action: &Row, cx| {
            let Some(editor) = view.upgrade() else {
                return;
            };
            EasyMotion::row(editor, action, cx);
        })
        .detach();

    let view = cx.view().downgrade();
    editor
        .register_action(move |_: &Cancel, cx| {
            let Some(editor) = view.upgrade() else {
                return;
            };
            EasyMotion::cancel(editor, cx);
        })
        .detach();

    let view = cx.view().downgrade();
    cx.observe_keystrokes(move |event, cx| {
        let Some(editor) = view.clone().upgrade() else {
            return;
        };
        EasyMotion::observe_keystrokes(editor, event, cx);
    })
    .detach();

    let entity_id = cx.view().entity_id();
    cx.on_release(move |_, _, cx| {
        EasyMotion::update(cx, |easy, _cx| easy.editor_states.remove(&entity_id));
    })
    .detach();
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
        cx: &WindowContext,
        f: impl FnOnce(&EasyMotion, &AppContext) -> S,
    ) -> Option<S> {
        EasyMotion::global(cx).map(|easy| easy.read_with(cx, f))
    }

    fn handle_new_matches(
        mut matches: Vec<DisplayPoint>,
        direction: Direction,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> Option<EasyMotionState> {
        let selections = editor.selections.newest_display(cx);
        let snapshot = editor.snapshot(cx);
        let map = &snapshot.display_snapshot;

        if matches.is_empty() {
            return None;
        }
        sort_matches_display(&mut matches, &selections.start);

        let keys = Self::read_with(cx, |easy, _| easy.keys.clone()).unwrap_or(DEFAULT_KEYS.into());

        let (style_0, style_1, style_2) = Self::get_highlights(cx);
        let trie = Trie::new_from_vec(keys, matches, |depth, point| {
            let style = match depth {
                0 | 1 => style_0,
                2 => style_1,
                3.. => style_2,
            };
            OverlayState {
                style,
                offset: point.to_offset(map, Bias::Right),
            }
        });
        Self::add_overlays(editor, trie.iter(), trie.len(), cx);

        let start = match direction {
            Direction::Both | Direction::Backwards => DisplayPoint::zero(),
            Direction::Forwards => selections.start,
        };
        let end = match direction {
            Direction::Both | Direction::Forwards => map.max_point(),
            Direction::Backwards => selections.end,
        };
        let anchor_start = map.display_point_to_anchor(start, Bias::Left);
        let anchor_end = map.display_point_to_anchor(end, Bias::Left);
        let highlight = HighlightStyle {
            fade_out: Some(0.7),
            ..Default::default()
        };
        editor.highlight_text::<Self>(vec![anchor_start..anchor_end], highlight, cx);

        let new_state = EasyMotionState::new(trie);
        let ctx = new_state.keymap_context_layer();
        editor.set_keymap_context_layer::<Self>(ctx, cx);
        Some(new_state)
    }

    fn word(editor: View<Editor>, action: &Word, cx: &mut WindowContext) {
        let Word(direction) = *action;
        EasyMotion::word_single_pane(editor, WordType::Word, direction, cx);
    }

    fn sub_word(editor: View<Editor>, action: &SubWord, cx: &mut WindowContext) {
        let SubWord(direction) = *action;
        EasyMotion::word_single_pane(editor, WordType::SubWord, direction, cx);
    }

    fn full_word(editor: View<Editor>, action: &FullWord, cx: &mut WindowContext) {
        let FullWord(direction) = *action;
        EasyMotion::word_single_pane(editor, WordType::FullWord, direction, cx);
    }

    fn word_single_pane(
        editor: View<Editor>,
        word_type: WordType,
        direction: Direction,
        cx: &mut WindowContext,
    ) {
        Vim::update(cx, |vim, cx| vim.switch_mode(Mode::EasyMotion, false, cx));

        let entity_id = editor.entity_id();

        let new_state = editor.update(cx, |editor, cx| {
            let word_starts = word_starts_fold(word_type, direction, editor, cx);
            Self::handle_new_matches(word_starts, direction, editor, cx)
        });

        Self::update(cx, move |easy, cx| {
            if let Some(new_state) = new_state {
                easy.editor_states.insert(entity_id, new_state);
            }
            cx.notify();
        });
    }

    fn row(editor: View<Editor>, action: &Row, cx: &mut WindowContext) {
        let Row(direction) = *action;
        let entity_id = editor.entity_id();

        let new_state = editor.update(cx, |editor, cx| {
            let matches = row_starts(direction, editor, cx);
            Self::handle_new_matches(matches, direction, editor, cx)
        });

        Self::update(cx, move |easy, cx| {
            if let Some(new_state) = new_state {
                easy.editor_states.insert(entity_id, new_state);
            }
            cx.notify();
        });
    }

    fn cancel(editor: View<Editor>, cx: &mut WindowContext) {
        editor.update(cx, |editor, cx| {
            editor.clear_overlays::<Self>(cx);
            editor.clear_highlights::<Self>(cx);
            editor.remove_keymap_context_layer::<Self>(cx);
        });
    }

    fn observe_keystrokes(
        editor: View<Editor>,
        keystroke_event: &KeystrokeEvent,
        cx: &mut WindowContext,
    ) {
        if keystroke_event.action.is_some() {
            return;
        } else if cx.has_pending_keystrokes() {
            return;
        }

        let entity_id = editor.entity_id();
        let Some(state) =
            Self::update(cx, |easy, _| easy.editor_states.remove(&entity_id)).flatten()
        else {
            return;
        };

        let keys = keystroke_event.keystroke.key.as_str();
        let new_state = editor.update(cx, |editor, cx| Self::handle_trim(state, keys, editor, cx));

        Vim::update(cx, |vim, cx| vim.switch_mode(Mode::Normal, false, cx));
        Self::update(cx, move |easy, cx| {
            if let Some(new_state) = new_state {
                easy.editor_states.insert(entity_id, new_state);
            }
            cx.notify();
        });
    }

    fn handle_trim(
        selection: EasyMotionState,
        keys: &str,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> Option<EasyMotionState> {
        let (selection, res) = selection.record_str(keys);
        match res {
            TrimResult::Found(overlay) => {
                let snapshot = editor.snapshot(cx);
                let point = overlay.offset.to_point(&snapshot.buffer_snapshot);
                let point = snapshot
                    .display_snapshot
                    .point_to_display_point(point, Bias::Right);
                editor.change_selections(Some(Autoscroll::fit()), cx, |selection| {
                    selection.move_cursors_with(|_, _, _| (point, SelectionGoal::None))
                });
                editor.clear_overlays::<Self>(cx);
                editor.clear_highlights::<Self>(cx);
                editor.remove_keymap_context_layer::<Self>(cx);
                None
            }
            TrimResult::Changed => {
                let trie = selection.trie();
                let len = trie.len();
                editor.clear_overlays::<Self>(cx);
                Self::add_overlays(editor, trie.iter(), len, cx);
                Some(selection)
            }
            TrimResult::Err => {
                editor.clear_overlays::<Self>(cx);
                editor.clear_highlights::<Self>(cx);
                editor.remove_keymap_context_layer::<Self>(cx);
                None
            }
            TrimResult::NoChange => Some(selection),
        }
    }

    fn add_overlays<'a>(
        editor: &mut Editor,
        trie_iter: impl Iterator<Item = (String, &'a OverlayState)>,
        len: usize,
        cx: &mut ViewContext<Editor>,
    ) {
        let overlays = trie_iter.map(|(seq, overlay)| {
            let mut highlights = vec![(0..1, overlay.style)];
            if seq.len() > 1 {
                highlights.push((
                    1..seq.len(),
                    HighlightStyle {
                        fade_out: Some(0.3),
                        ..overlay.style
                    },
                ));
            }
            Overlay {
                text: seq,
                highlights,
                buffer_offset: overlay.offset,
            }
        });
        editor.add_overlays_with_reserve::<Self>(overlays, len, cx);
    }

    fn get_highlights(cx: &AppContext) -> (HighlightStyle, HighlightStyle, HighlightStyle) {
        let theme = &ThemeSettings::get_global(cx).active_theme;
        let players = &theme.players().0;
        let bg = theme.colors().background;
        let style_0 = HighlightStyle {
            color: Some(saturate(players[0].cursor, 1.0)),
            background_color: Some(bg),
            ..HighlightStyle::default()
        };
        let style_1 = HighlightStyle {
            color: Some(saturate(players[2].cursor, 1.0)),
            background_color: Some(bg),
            ..HighlightStyle::default()
        };
        let style_2 = HighlightStyle {
            color: Some(saturate(players[3].cursor, 1.0)),
            background_color: Some(bg),
            ..HighlightStyle::default()
        };
        (style_0, style_1, style_2)
    }
}
