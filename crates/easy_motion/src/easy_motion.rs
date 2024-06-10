use collections::HashMap;
use serde::Deserialize;
use std::{fmt, sync::Arc};

use editor::{overlay::Overlay, scroll::Autoscroll, DisplayPoint, Editor};
use gpui::{
    actions, impl_actions, saturate, AppContext, Entity, EntityId, Global, HighlightStyle,
    KeystrokeEvent, Model, ModelContext, View, ViewContext, WeakView,
};
use settings::Settings;
use text::{Bias, SelectionGoal};
use theme::ThemeSettings;
use ui::{Context, WindowContext};
use workspace::Workspace;

use crate::{
    editor_state::{EditorState, OverlayState},
    search::{row_starts, sort_matches_display, word_starts},
    trie::{Trie, TrimResult},
};

mod editor_events;
mod editor_state;
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
    active_editor: Option<WeakView<Editor>>,
    dimming: bool,
    keys: Arc<str>,
    enabled: bool,
    editor_states: HashMap<EntityId, EditorState>,
}

impl fmt::Debug for EasyMotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EasyMotion")
            .field(
                "active_editor",
                &self.active_editor.as_ref().map(|editor| editor.entity_id()),
            )
            .field("dimming", &self.dimming)
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
            active_editor: None,
            dimming: true,
            editor_states: HashMap::default(),
            enabled: true,
            keys: DEFAULT_KEYS.into(),
        }
    });
    EasyMotion::set_global(easy.clone(), cx);
    cx.observe_keystrokes(EasyMotion::observe_keystrokes)
        .detach();
    cx.observe_new_views(|workspace: &mut Workspace, cx| register(workspace, cx))
        .detach();

    editor_events::init(cx);
}

fn register(workspace: &mut Workspace, _: &ViewContext<Workspace>) {
    workspace.register_action(|workspace: &mut Workspace, action: &Word, cx| {
        EasyMotion::word(action, workspace, cx);
    });
    workspace.register_action(|workspace: &mut Workspace, action: &SubWord, cx| {
        EasyMotion::sub_word(action, workspace, cx);
    });
    workspace.register_action(|workspace: &mut Workspace, action: &FullWord, cx| {
        EasyMotion::full_word(action, workspace, cx);
    });

    workspace.register_action(|workspace: &mut Workspace, action: &Row, cx| {
        EasyMotion::row(action, workspace, cx);
    });

    workspace.register_action(|workspace: &mut Workspace, _: &Cancel, cx| {
        EasyMotion::cancel(workspace, cx);
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
        cx: &WindowContext,
        f: impl FnOnce(&EasyMotion, &AppContext) -> S,
    ) -> Option<S> {
        EasyMotion::global(cx).map(|easy| easy.read_with(cx, f))
    }

    #[allow(dead_code)]
    fn update_active_editor<S>(
        cx: &mut ViewContext<Workspace>,
        update: impl FnOnce(&Editor, &ViewContext<Editor>) -> S,
    ) -> Option<S> {
        let editor = EasyMotion::read_with(cx, |easy, _cx| easy.active_editor.clone())
            .flatten()?
            .upgrade()?;
        Some(editor.update(cx, |editor, cx| update(editor, cx)))
    }

    fn activate_editor(&mut self, editor: View<Editor>) {
        self.active_editor = Some(editor.downgrade());
    }

    fn active_editor(cx: &WindowContext) -> Option<View<Editor>> {
        Self::read_with(cx, |easy, _| {
            easy.active_editor.as_ref().and_then(|weak| weak.upgrade())
        })
        .flatten()
    }

    #[allow(dead_code)]
    fn clear_state(&mut self) -> Option<()> {
        let active_editor = self.active_editor.as_ref()?;
        self.editor_states.remove(&active_editor.entity_id());
        Some(())
    }

    fn take_state(&mut self) -> Option<EditorState> {
        self.active_editor
            .as_ref()
            .and_then(|active_editor| self.editor_states.remove(&active_editor.entity_id()))
    }

    fn handle_new_matches(
        mut matches: Vec<DisplayPoint>,
        direction: Direction,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> Option<EditorState> {
        let selections = editor.selections.newest_display(cx);
        let snapshot = editor.snapshot(cx);
        let map = &snapshot.display_snapshot;

        if matches.is_empty() {
            return None;
        }
        sort_matches_display(&mut matches, &selections.start);

        let (keys, dimming) = Self::read_with(cx, |easy, _| (easy.keys.clone(), easy.dimming))
            .unwrap_or((DEFAULT_KEYS.into(), true));

        let (style_0, style_1, style_2) = Self::get_highlights(cx);
        let trie = Trie::new_from_vec(keys, matches, |depth, point| {
            let style = match depth {
                0 | 1 => style_0,
                2 => style_1,
                3.. => style_2,
            };
            OverlayState { style, point }
        });
        Self::add_overlays(editor, trie.iter(), trie.len(), cx);

        if dimming {
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
        }

        let new_state = EditorState::new(trie);
        let ctx = new_state.keymap_context_layer();
        editor.set_keymap_context_layer::<Self>(ctx, cx);
        Some(new_state)
    }

    fn word(action: &Word, _workspace: &Workspace, cx: &mut WindowContext) {
        let Word(direction) = *action;
        // TODO other directions?
        // not sure if check for multiple editors is totally necessary
        EasyMotion::word_single_pane(WordType::Word, direction, cx);
    }

    fn sub_word(action: &SubWord, _workspace: &Workspace, cx: &mut WindowContext) {
        let SubWord(direction) = *action;
        // TODO other directions?
        // not sure if check for multiple editors is totally necessary
        EasyMotion::word_single_pane(WordType::SubWord, direction, cx);
    }

    fn full_word(action: &FullWord, _workspace: &Workspace, cx: &mut WindowContext) {
        let FullWord(direction) = *action;
        // TODO other directions?
        // not sure if check for multiple editors is totally necessary
        EasyMotion::word_single_pane(WordType::FullWord, direction, cx);
    }

    fn word_single_pane(word_type: WordType, direction: Direction, cx: &mut WindowContext) {
        let Some(active_editor) = Self::active_editor(cx) else {
            return;
        };
        let entity_id = active_editor.entity_id();

        let new_state = active_editor.update(cx, |editor, cx| {
            let word_starts = word_starts(word_type, direction, editor, cx);

            Self::handle_new_matches(word_starts, direction, editor, cx)
        });

        Self::update(cx, move |easy, cx| {
            if let Some(new_state) = new_state {
                easy.editor_states.insert(entity_id, new_state);
            }
            cx.notify();
        });
    }

    fn row(action: &Row, _: &Workspace, cx: &mut WindowContext) {
        let Row(direction) = *action;
        let Some(active_editor) = Self::active_editor(cx) else {
            return;
        };
        let entity_id = active_editor.entity_id();

        let new_state = active_editor.update(cx, |editor, cx| {
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

    fn cancel(_workspace: &Workspace, cx: &mut WindowContext) {
        if let Some(editor) = Self::active_editor(cx) {
            editor.update(cx, |editor, cx| {
                editor.clear_overlays::<Self>(cx);
                editor.clear_highlights::<Self>(cx);
                editor.remove_keymap_context_layer::<Self>(cx);
            });
        }
    }

    fn observe_keystrokes(keystroke_event: &KeystrokeEvent, cx: &mut WindowContext) {
        if keystroke_event.action.is_some() {
            return;
        } else if cx.has_pending_keystrokes() {
            return;
        }

        Self::observe_keystrokes_impl(keystroke_event, cx);
    }

    fn observe_keystrokes_impl(keystroke_event: &KeystrokeEvent, cx: &mut WindowContext) {
        let Some((state, weak_editor)) = Self::update(cx, |easy, _| {
            let state = easy.take_state()?;
            let weak_editor = easy.active_editor.clone()?;
            Some((state, weak_editor))
        })
        .flatten() else {
            return;
        };

        let editor = weak_editor.upgrade();
        let Some(editor) = editor else {
            return;
        };
        let entity_id = editor.entity_id();

        let keys = keystroke_event.keystroke.key.as_str();
        let new_state = editor.update(cx, |editor, cx| Self::handle_trim(state, keys, editor, cx));

        Self::update(cx, move |easy, cx| {
            if let Some(new_state) = new_state {
                easy.editor_states.insert(entity_id, new_state);
            }
            cx.notify();
        });
    }

    fn handle_trim(
        selection: EditorState,
        keys: &str,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> Option<EditorState> {
        let (selection, res) = selection.record_str(keys);
        match res {
            TrimResult::Found(overlay) => {
                editor.change_selections(Some(Autoscroll::fit()), cx, |selection| {
                    selection.move_cursors_with(|_, _, _| (overlay.point, SelectionGoal::None))
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
        let iter = trie_iter.map(|(seq, overlay)| {
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
                point: overlay.point,
                offset: 0.0,
            }
        });
        editor.add_overlays::<Self>(iter, len, cx);
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
