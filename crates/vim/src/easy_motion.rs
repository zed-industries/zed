use anyhow::Result;
use schemars::JsonSchema;
use search::word_starts;
use serde::{Deserialize, Serialize};
use std::fmt;

use editor::{
    display_map::DisplaySnapshot, overlay::Overlay, scroll::Autoscroll, DisplayPoint, Editor,
    EditorEvent, MultiBufferSnapshot, ToPoint,
};
use gpui::{
    actions, impl_actions, saturate, Action, AppContext, HighlightStyle, KeystrokeEvent, View,
    ViewContext, WeakView,
};
use settings::{Settings, SettingsSources};
use text::{Bias, SelectionGoal};
use theme::ThemeSettings;
use ui::{IntoElement, Render, VisualContext};

use crate::easy_motion::{
    editor_state::{EasyMotionState, OverlayState},
    search::{row_starts, sort_matches_display},
    trie::{Trie, TrimResult},
};
use crate::{state::Mode, Vim};

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

#[derive(Clone)]
pub(crate) struct EasyMotionAddon {
    pub(crate) _view: View<EasyMotion>,
}

impl editor::Addon for EasyMotionAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct EasyMotion {
    state: Option<EasyMotionState>,
    editor: WeakView<Editor>,
    vim: WeakView<Vim>,
}

impl fmt::Debug for EasyMotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EasyMotion")
            .field("state", &self.state)
            .finish()
    }
}

pub fn init(cx: &mut AppContext) {
    EasyMotionSettings::register(cx);
}

pub(crate) fn register(editor: &mut Editor, cx: &mut ViewContext<EasyMotion>) {
    EasyMotion::action(editor, cx, EasyMotion::word);
    EasyMotion::action(editor, cx, EasyMotion::full_word);
    EasyMotion::action(editor, cx, EasyMotion::row);
    EasyMotion::action(editor, cx, EasyMotion::cancel);
}

// Hack: Vim intercepts events dispatched to a window and updates the view in response.
// This means it needs a VisualContext. The easiest way to satisfy that constraint is
// to make Vim a "View" that is just never actually rendered.
impl Render for EasyMotion {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

impl EasyMotion {
    pub(crate) fn new(cx: &mut ViewContext<Editor>, vim: WeakView<Vim>) -> View<Self> {
        let editor = cx.view().clone();

        cx.new_view(|cx: &mut ViewContext<EasyMotion>| {
            cx.subscribe(&editor, EasyMotion::update_overlays).detach();

            cx.observe_keystrokes(EasyMotion::observe_keystrokes)
                .detach();
            Self {
                editor: editor.downgrade(),
                vim,
                state: None,
            }
        })
    }

    pub fn action<A: Action>(
        editor: &mut Editor,
        cx: &mut ViewContext<EasyMotion>,
        f: impl Fn(&mut EasyMotion, &A, &mut ViewContext<EasyMotion>) + 'static,
    ) {
        let subscription = editor.register_action(cx.listener(f));
        cx.on_release(|_, _, _| drop(subscription)).detach();
    }

    fn handle_new_matches(
        mut matches: Vec<DisplayPoint>,
        direction: Direction,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> Option<EasyMotionState> {
        editor.blink_manager.update(cx, |blink, cx| {
            blink.disable(cx);
            blink.hide_cursor(cx);
        });

        let selections = editor.selections.newest_display(cx);
        let snapshot = editor.snapshot(cx);
        let map = &snapshot.display_snapshot;

        if matches.is_empty() {
            return None;
        }
        sort_matches_display(&mut matches, &selections.start);

        let settings = EasyMotionSettings::get_global(cx);
        let keys = settings.keys.clone();

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
        Self::add_overlays(
            editor,
            trie.iter(),
            trie.len(),
            &snapshot.buffer_snapshot,
            &snapshot.display_snapshot,
            cx,
        );

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
        Some(new_state)
    }

    fn word(&mut self, action: &Word, cx: &mut ViewContext<EasyMotion>) {
        let Word(direction) = *action;
        self.word_impl(WordType::Word, direction, cx);
    }

    fn full_word(&mut self, action: &FullWord, cx: &mut ViewContext<EasyMotion>) {
        let FullWord(direction) = *action;
        self.word_impl(WordType::FullWord, direction, cx);
    }

    fn clear_editor(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        editor.blink_manager.update(cx, |blink, cx| {
            blink.enable(cx);
        });
        editor.clear_overlays::<Self>(cx);
        editor.clear_highlights::<Self>(cx);
    }

    fn word_impl(
        &mut self,
        word_type: WordType,
        direction: Direction,
        cx: &mut ViewContext<EasyMotion>,
    ) {
        let Some((vim, editor)) = self.vim.upgrade().zip(self.editor.upgrade()) else {
            return;
        };
        let mode = vim.update(cx, |vim, cx| {
            let mode = vim.mode;
            assert_ne!(mode, Mode::EasyMotion);
            vim.switch_mode(Mode::EasyMotion, false, cx);
            mode
        });

        let new_state = editor.update(cx, |editor, cx| {
            let word_starts = word_starts(word_type, direction, editor, cx);
            Self::handle_new_matches(word_starts, direction, editor, cx)
        });
        let Some(new_state) = new_state else {
            vim.update(cx, move |vim, cx| {
                vim.switch_mode(mode, false, cx);
            });
            return;
        };

        self.state = Some(new_state);
    }

    fn row(&mut self, action: &Row, cx: &mut ViewContext<EasyMotion>) {
        let Some((vim, editor)) = self.vim.upgrade().zip(self.editor.upgrade()) else {
            return;
        };
        vim.update(cx, |vim, cx| {
            let mode = vim.mode;
            assert_ne!(mode, Mode::EasyMotion);
            vim.switch_mode(Mode::EasyMotion, false, cx);
        });

        let Row(direction) = *action;

        let new_state = editor.update(cx, |editor, cx| {
            let matches = row_starts(direction, editor, cx);
            Self::handle_new_matches(matches, direction, editor, cx)
        });
        let Some(new_state) = new_state else {
            return;
        };

        self.state = Some(new_state);
    }

    fn cancel(&mut self, _action: &Cancel, cx: &mut ViewContext<EasyMotion>) {
        let Some((vim, editor)) = self.vim.upgrade().zip(self.editor.upgrade()) else {
            return;
        };
        vim.update(cx, |vim, cx| {
            let mode = vim.mode;
            assert_eq!(mode, Mode::EasyMotion);
            vim.switch_mode(Mode::Normal, false, cx);
        });

        self.state = None;
        editor.update(cx, |editor, cx| Self::clear_editor(editor, cx));
    }

    fn observe_keystrokes(&mut self, keystroke_event: &KeystrokeEvent, cx: &mut ViewContext<Self>) {
        if keystroke_event.action.is_some() {
            return;
        } else if cx.has_pending_keystrokes() {
            return;
        }

        let Some((state, editor)) = self.state.take().zip(self.editor.upgrade()) else {
            return;
        };

        let keys = keystroke_event.keystroke.key.as_str();
        let new_state = editor.update(cx, |editor, cx| Self::handle_trim(state, keys, editor, cx));
        let Some(new_state) = new_state else {
            let Some(vim) = self.vim.upgrade() else {
                return;
            };
            vim.update(cx, |vim, cx| vim.switch_mode(Mode::Normal, false, cx));
            return;
        };

        self.state = Some(new_state);
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
                Self::clear_editor(editor, cx);
                None
            }
            TrimResult::Changed => {
                let trie = selection.trie();
                let len = trie.len();
                editor.clear_overlays::<Self>(cx);
                let snapshot = editor.snapshot(cx);
                Self::add_overlays(
                    editor,
                    trie.iter(),
                    len,
                    &snapshot.buffer_snapshot,
                    &snapshot.display_snapshot,
                    cx,
                );
                Some(selection)
            }
            TrimResult::Err => {
                Self::clear_editor(editor, cx);
                None
            }
            TrimResult::NoChange => Some(selection),
        }
    }

    fn add_overlays<'a>(
        editor: &mut Editor,
        trie_iter: impl Iterator<Item = (String, &'a OverlayState)>,
        len: usize,
        buffer_snapshot: &MultiBufferSnapshot,
        display_snapshot: &DisplaySnapshot,
        cx: &mut ViewContext<Editor>,
    ) {
        let overlays = trie_iter.filter_map(|(seq, overlay)| {
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
            let point = buffer_snapshot.offset_to_point(overlay.offset);
            if display_snapshot.is_point_folded(point) {
                None
            } else {
                Some(Overlay {
                    text: seq,
                    highlights,
                    point: display_snapshot.point_to_display_point(point, text::Bias::Left),
                })
            }
        });
        editor.add_overlays_with_reserve::<Self>(overlays, len, cx);
    }

    fn update_overlays(
        &mut self,
        view: View<Editor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if !matches!(event, EditorEvent::Fold | EditorEvent::UnFold) {
            return;
        }
        let Some(state) = self.state.as_ref() else {
            return;
        };

        view.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            editor.clear_overlays::<Self>(cx);
            Self::add_overlays(
                editor,
                state.trie().iter(),
                state.trie().len(),
                &snapshot.buffer_snapshot,
                &snapshot.display_snapshot,
                cx,
            );
        });
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
