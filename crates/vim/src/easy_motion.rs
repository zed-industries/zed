use anyhow::Result;
use collections::HashMap;
use itertools::Itertools;
use schemars::JsonSchema;
use search::word_starts_fold;
use serde::{Deserialize, Serialize};
use std::fmt;
use workspace::Workspace;

use editor::{overlay::Overlay, scroll::Autoscroll, DisplayPoint, Editor, ToPoint};
use gpui::{
    actions, impl_actions, saturate, AppContext, Entity, EntityId, Global, HighlightStyle,
    KeystrokeEvent, Model, ModelContext, Subscription, View, ViewContext,
};
use settings::{Settings, SettingsSources, SettingsStore};
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

impl_actions!(easy_motion, [NChar, Pattern, Word, FullWord, Row]);

actions!(easy_motion, [Cancel, PatternSubmit]);

#[derive(Clone, Copy, Debug)]
enum WordType {
    Word,
    FullWord,
}

pub struct EasyMotion {
    keys: String,
    editor_states: HashMap<EntityId, EasyMotionState>,
    subscriptions: Vec<Subscription>,
}

impl fmt::Debug for EasyMotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EasyMotion")
            .field("keys", &self.keys)
            .field("editor_states", &self.editor_states)
            .field("subscriptions_count", &self.subscriptions.len())
            .finish()
    }
}

struct GlobalEasyMotion(Model<EasyMotion>);

impl Global for GlobalEasyMotion {}

pub fn init(cx: &mut AppContext) {
    EasyMotionSettings::register(cx);
    sync(true, cx);
    cx.observe_global::<SettingsStore>(|cx| {
        sync(false, cx);
    })
    .detach();
}

fn active_editor_views(workspace: &Workspace, cx: &AppContext) -> Vec<View<Editor>> {
    let panes = workspace.panes();
    panes
        .iter()
        .flat_map(|pane| {
            pane.read(cx)
                .items()
                .filter_map(|item| item.downcast::<Editor>())
        })
        .collect()
}

fn sync(init: bool, cx: &mut AppContext) {
    let settings = EasyMotionSettings::get_global(cx);
    let was_enabled = cx.has_global::<GlobalEasyMotion>();

    if !settings.enabled {
        if was_enabled {
            // TODO: there also must be a better way to do this
            let _ = cx.active_window().map(|window| {
                window.update(cx, |_, cx| {
                    Vim::update(cx, |vim, cx| {
                        if vim.mode() == Mode::EasyMotion {
                            vim.switch_mode(Mode::Normal, false, cx);
                        }
                    });
                })
            });
            cx.remove_global::<GlobalEasyMotion>();
        }
        return;
    }

    if was_enabled {
        let keys = settings.keys.clone();
        EasyMotion::update(cx, |easy, _cx| easy.keys = keys);
    } else {
        let keys = settings.keys.clone();
        let mut subs = if init {
            Vec::new()
        } else {
            // if the application is already open then we need to add listeners to all the open editors
            // TODO: there must be a better way to do this
            cx.windows()[0]
                .downcast::<Workspace>()
                .unwrap()
                .update(cx, |workspace, cx| {
                    active_editor_views(workspace, cx)
                        .into_iter()
                        .flat_map(|editor| {
                            editor
                                .update(cx, |editor, cx| register(editor, cx))
                                .into_iter()
                        })
                        .collect_vec()
                })
                .unwrap()
        };
        subs.push(cx.observe_new_views(|editor: &mut Editor, cx| {
            let mut hi = register(editor, cx);
            EasyMotion::update(cx, |easy, _cx| easy.subscriptions.append(&mut hi));
        }));

        let easy = cx.new_model(move |_| EasyMotion::new(keys, subs));
        EasyMotion::set_global(easy.clone(), cx);
    }
}

fn register(editor: &mut Editor, cx: &mut ViewContext<Editor>) -> Vec<Subscription> {
    let view = cx.view().downgrade();
    let mut subs = Vec::new();
    subs.push(editor.register_action(move |action: &Word, cx| {
        let Some(editor) = view.upgrade() else {
            return;
        };
        EasyMotion::word(editor, action, cx);
    }));

    let view = cx.view().downgrade();
    subs.push(editor.register_action(move |action: &FullWord, cx| {
        let Some(editor) = view.upgrade() else {
            return;
        };
        EasyMotion::full_word(editor, action, cx);
    }));

    let view = cx.view().downgrade();
    subs.push(editor.register_action(move |action: &Row, cx| {
        let Some(editor) = view.upgrade() else {
            return;
        };
        EasyMotion::row(editor, action, cx);
    }));

    let view = cx.view().downgrade();
    subs.push(editor.register_action(move |_: &Cancel, cx| {
        let Some(editor) = view.upgrade() else {
            return;
        };
        EasyMotion::cancel(editor, cx);
    }));

    let view = cx.view().downgrade();
    subs.push(cx.observe_keystrokes(move |event, cx| {
        let Some(editor) = view.clone().upgrade() else {
            return;
        };
        EasyMotion::observe_keystrokes(editor, event, cx);
    }));

    let entity_id = cx.view().entity_id();
    subs.push(cx.on_release(move |_, _, cx| {
        EasyMotion::update(cx, |easy, _cx| easy.editor_states.remove(&entity_id));
    }));

    subs
}

impl EasyMotion {
    fn new(keys: String, subscriptions: Vec<Subscription>) -> Self {
        Self {
            editor_states: HashMap::default(),
            keys,
            subscriptions,
        }
    }

    fn update<F, S>(cx: &mut AppContext, f: F) -> Option<S>
    where
        F: FnOnce(&mut EasyMotion, &mut ModelContext<EasyMotion>) -> S,
    {
        EasyMotion::global(cx).map(|easy| easy.update(cx, f))
    }

    fn global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<GlobalEasyMotion>()
            .map(|model| model.0.clone())
    }

    fn set_global(easy: Model<Self>, cx: &mut AppContext) {
        cx.set_global(GlobalEasyMotion(easy));
    }

    fn read_with<S>(
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

        let keys = Self::read_with(cx, |easy, _| easy.keys.clone()).unwrap();

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
        Vim::update(cx, |vim, cx| vim.switch_mode(Mode::Normal, false, cx));

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
        let id = editor.entity_id();
        Self::update(cx, |easy, _| easy.editor_states.remove(&id));
        Vim::update(cx, |vim, cx| vim.switch_mode(Mode::Normal, false, cx));
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

#[derive(Deserialize)]
struct EasyMotionSettings {
    pub enabled: bool,
    pub keys: String,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
struct EasyMotionSettingsContent {
    pub enabled: Option<bool>,
    pub keys: Option<String>,
}

impl Settings for EasyMotionSettings {
    const KEY: Option<&'static str> = Some("easy_motion");

    type FileContent = EasyMotionSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
