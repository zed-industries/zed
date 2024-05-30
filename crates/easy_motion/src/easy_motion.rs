use collections::HashMap;
use editor::display_map::DisplaySnapshot;
use serde::Deserialize;
use std::cmp::Ordering;
use std::{fmt, mem};

use editor::scroll::Autoscroll;
use editor::{DisplayPoint, Editor};
use gpui::{
    actions, impl_actions, saturate, AppContext, Bounds, Entity, EntityId, Global, HighlightStyle,
    KeystrokeEvent, Model, ModelContext, Point, Subscription, View, ViewContext, WeakView,
};
use perm::{TrieBuilder, TrimResult};
use settings::Settings;
use text::{Bias, Selection as TextSelection, SelectionGoal};
use theme::ThemeSettings;
use ui::{Context, Pixels, WindowContext};
use workspace::{Pane, Workspace};

use editor_state::{EditorState, InputResult, NCharInput, OverlayState, Selection};
use util::{end_of_document, manh_distance, start_of_document, window_bottom, window_top};

use crate::util::manh_distance_pixels;

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

#[derive(Clone, Copy, Debug)]
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
    multipane_state: Option<EditorState>,
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
        move |_cx| {
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

fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(|workspace: &mut Workspace, action: &Word, cx| {
        let Word(direction) = *action;
        // TODO other directions
        if workspace.is_split() && matches!(direction, Direction::BiDirectional) {
            EasyMotion::word_multipane(WordType::Word, workspace, cx);
        } else {
            EasyMotion::word(WordType::Word, direction, cx);
        }
    });
    workspace.register_action(|_: &mut Workspace, action: &SubWord, cx| {
        let direction = action.0;
        // TODO multipane
        EasyMotion::word(WordType::SubWord, direction, cx);
    });
    workspace.register_action(|_: &mut Workspace, action: &FullWord, cx| {
        let direction = action.0;
        // TODO multipane
        EasyMotion::word(WordType::FullWord, direction, cx);
    });

    workspace.register_action(|_: &mut Workspace, action: &NChar, cx| {
        EasyMotion::n_char(action, cx);
    });

    workspace.register_action(|_: &mut Workspace, action: &Pattern, cx| {
        EasyMotion::pattern(action, cx);
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
        cx: &ViewContext<Workspace>,
        f: impl FnOnce(&EasyMotion, &AppContext) -> S,
    ) -> Option<S> {
        EasyMotion::global(cx).map(|easy| easy.read_with(cx, f))
    }

    #[allow(dead_code)]
    fn update_active_editor<S>(
        cx: &mut ViewContext<Workspace>,
        update: impl FnOnce(&mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        let weak_editor =
            EasyMotion::read_with(cx, |easy, _cx| easy.active_editor.clone()).flatten()?;
        let editor = weak_editor.upgrade()?;
        Some(editor.update(cx, |editor, cx| update(editor, cx)))
    }

    fn activate_editor(&mut self, editor: View<Editor>) {
        self.active_editor = Some(editor.downgrade());
    }

    fn active_editor(cx: &mut ViewContext<Workspace>) -> Option<View<Editor>> {
        Self::read_with(cx, |easy, _| {
            easy.active_editor.as_ref().and_then(|weak| weak.upgrade())
        })
        .flatten()
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
        self.insert_state(EditorState::None);
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
        let Some(active_editor) = Self::active_editor(cx) else {
            return;
        };
        let entity_id = active_editor.entity_id();

        let new_state = active_editor.update(cx, |editor, cx| {
            let new_state = Self::word_impl(true, word_type, direction, editor, cx);
            let ctx = new_state.keymap_context_layer();
            editor.set_keymap_context_layer::<Self>(ctx, cx);
            new_state
        });

        Self::update(cx, move |easy, _cx| {
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
        let map = &editor.snapshot(cx).display_snapshot;

        let mut word_starts = Self::word_starts(word_type, direction, map, &selections, editor, cx);
        word_starts.sort_unstable_by(|a, b| {
            let a_distance = manh_distance(a, &selections.start, 2.5);
            let b_distance = manh_distance(b, &selections.start, 2.5);
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

        let (style_0, style_1, style_2) = get_highlights(cx);
        let trie = TrieBuilder::new("asdghklqwertyuiopzxcvbnmfj".to_string(), word_starts.len())
            .populate_with(true, word_starts.into_iter(), |seq, point| {
                let style = match seq.len() {
                    0 | 1 => style_0,
                    2 => style_1,
                    3.. => style_2,
                };
                OverlayState {
                    style,
                    point,
                    editor_id: cx.entity_id(),
                }
            });
        Self::add_overlays(editor, trie.iter(), cx);

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
            editor.highlight_text::<Self>(vec![anchor_start..anchor_end], highlight, cx);
        }

        EditorState::Selection(Selection::new(trie))
    }

    fn word_starts(
        word_type: WordType,
        direction: Direction,
        map: &DisplaySnapshot,
        selections: &TextSelection<DisplayPoint>,
        editor: &Editor,
        cx: &WindowContext,
    ) -> Vec<DisplayPoint> {
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
        util::word_starts_in_range(&map, start, end, full_word)
    }

    fn word_multipane(
        word_type: WordType,
        workspace: &mut Workspace,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(active_editor) = Self::active_editor(cx) else {
            return;
        };
        let active_editor_id = active_editor.entity_id();

        let panes = workspace.panes();

        let editors = panes
            .iter()
            .filter_map(|pane| {
                pane.update(cx, |pane, _cx| {
                    let active_item = pane.active_item();
                    active_item.map(|item| item.downcast::<Editor>())
                })
                .flatten()
                .map(|editor| {
                    let bounding_box = workspace.center().bounding_box_for_pane(pane).unwrap();
                    (editor, pane.clone(), bounding_box)
                })
            })
            .collect::<Vec<_>>();

        let new_state = Self::word_multipane_impl(word_type, true, active_editor_id, &editors, cx);

        for (editor, _, _) in editors {
            editor.update(cx, |editor, cx| {
                let ctx = new_state.keymap_context_layer();
                editor.set_keymap_context_layer::<Self>(ctx, cx);
            });
        }

        Self::update(cx, move |easy, _cx| {
            easy.multipane_state = Some(new_state);
        });
    }

    fn word_multipane_impl(
        word_type: WordType,
        dimming: bool,
        active_editor_id: EntityId,
        editors: &[(View<Editor>, View<Pane>, Bounds<Pixels>)],
        cx: &mut ViewContext<Workspace>,
    ) -> EditorState {
        // TODO do this in parallel?
        // get words along with their display points within their editors
        // as well as a rough absolute position for sorting purposes
        let cursor = editors
            .iter()
            .find(|(editor_view, _, _)| editor_view.entity_id() == active_editor_id)
            .map(|(editor_view, _, bounding_box)| {
                editor_view.update(cx, |editor, cx| {
                    let style = cx.text_style();
                    let line_height = style
                        .line_height
                        .to_pixels(style.font_size, cx.rem_size())
                        .0;
                    let selections = editor.selections.newest_display(cx);
                    let start = selections.start;
                    let text_layout_details = editor.text_layout_details(cx);
                    let map = editor.snapshot(cx).display_snapshot;
                    let window_top = window_top(&map, &text_layout_details);
                    let x = bounding_box.origin.x.0 + start.column() as f32 * line_height * 0.5;
                    let y = bounding_box.origin.y.0
                        + (start.row().0 as f32 - window_top.row().0 as f32) * line_height;
                    let x = Pixels(x);
                    let y = Pixels(y);
                    Point::new(x, y)
                })
            })
            .unwrap();

        let mut word_starts = editors
            .iter()
            .map(|(editor_view, _pane_view, bounding_box)| {
                editor_view.update(cx, |editor, cx| {
                    let selections = editor.selections.newest_display(cx);

                    let map = editor.snapshot(cx).display_snapshot;

                    let style = cx.text_style();
                    let line_height = style
                        .line_height
                        .to_pixels(style.font_size, cx.rem_size())
                        .0;

                    let text_layout_details = editor.text_layout_details(cx);
                    let window_top = window_top(&map, &text_layout_details);

                    let words = Self::word_starts(
                        word_type,
                        Direction::BiDirectional,
                        &map,
                        &selections,
                        editor,
                        cx,
                    );
                    words.into_iter().map(move |word| {
                        let x = bounding_box.origin.x.0 + word.column() as f32 * line_height * 0.5;
                        let y = bounding_box.origin.y.0 - window_top.row().0 as f32 * line_height
                            + word.row().0 as f32 * line_height;
                        let x = Pixels(x);
                        let y = Pixels(y);
                        (word, editor_view.entity_id(), Point::new(x, y))
                    })
                })
            })
            .flatten()
            .collect::<Vec<_>>();

        word_starts.sort_unstable_by(|a, b| {
            let a_distance = manh_distance_pixels(&a.2, &cursor, 2.5);
            let b_distance = manh_distance_pixels(&b.2, &cursor, 2.5);
            if a_distance == b_distance {
                Ordering::Equal
            } else if a_distance < b_distance {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let (style_0, style_1, style_2) = get_highlights(cx);
        let word_starts = word_starts.into_iter().map(|(point, id, _)| (point, id));
        let trie = TrieBuilder::new("asdghklqwertyuiopzxcvbnmfj".to_string(), word_starts.len())
            .populate_with(true, word_starts, |seq, point| {
                let style = match seq.len() {
                    0 | 1 => style_0,
                    2 => style_1,
                    3.. => style_2,
                };
                OverlayState {
                    style,
                    point: point.0,
                    editor_id: point.1,
                }
            });

        for (editor, _, _) in editors {
            let trie_iter = trie
                .iter()
                .filter(|(_seq, overlay)| overlay.editor_id == editor.entity_id());

            editor.update(cx, |editor, cx| {
                Self::add_overlays(editor, trie_iter, cx);
                if dimming {
                    let map = &editor.snapshot(cx).display_snapshot;
                    let start = start_of_document(map);
                    let end = end_of_document(map);
                    let anchor_start = map.display_point_to_anchor(start, Bias::Left);
                    let anchor_end = map.display_point_to_anchor(end, Bias::Left);
                    let highlight = HighlightStyle {
                        fade_out: Some(0.7),
                        ..Default::default()
                    };
                    editor.highlight_text::<Self>(vec![anchor_start..anchor_end], highlight, cx);
                }
            });
        }

        EditorState::Selection(Selection::new(trie))
    }

    fn pattern(_action: &Pattern, _cx: &mut WindowContext) {
        todo!()
    }

    fn n_char(_action: &NChar, _cx: &mut WindowContext) {
        todo!()
    }

    fn cancel(workspace: &mut Workspace, cx: &mut WindowContext) {
        let editor = Self::update(cx, |easy, _| {
            if let Some(state) = easy.multipane_state.as_mut() {
                state.clear();
                None
            } else {
                easy.clear_state();
                easy.active_editor.clone()
            }
        })
        .flatten();

        if workspace.panes().len() > 1 {
            let editors = active_editor_views(workspace, cx);
            for editor in editors {
                editor.update(cx, |editor, cx| {
                    editor.clear_overlays(cx);
                    editor.clear_highlights::<Self>(cx);
                    editor.remove_keymap_context_layer::<Self>(cx);
                });
            }
        } else if let Some(editor) = editor.map(|editor| editor.upgrade()).flatten() {
            editor.update(cx, |editor, cx| {
                editor.clear_overlays(cx);
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

        if let Some(state) = Self::update(cx, |easy, _| easy.multipane_state.take()).flatten() {
            Self::observe_keystrokes_impl_multipane(keystroke_event, state, cx)
        } else {
            Self::observe_keystrokes_impl(keystroke_event, cx);
        };
    }

    fn observe_keystrokes_impl(keystroke_event: &KeystrokeEvent, cx: &mut WindowContext) {
        let Some((state, weak_editor)) = Self::update(cx, |easy, _| {
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

    fn observe_keystrokes_impl_multipane(
        keystroke_event: &KeystrokeEvent,
        state: EditorState,
        cx: &mut WindowContext,
    ) {
        if !state.easy_motion_controlled() {
            return;
        }

        let keys = keystroke_event.keystroke.key.as_str();
        let new_state = match state {
            EditorState::NCharInput(char_input) => Self::handle_record_char(char_input, keys, cx),
            EditorState::Selection(selection) => Self::handle_trim_multipane(selection, keys, cx),
            EditorState::None => EditorState::None,
        };

        Self::update(cx, move |easy, _| {
            easy.multipane_state = Some(new_state);
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
            TrimResult::Found(overlay) => {
                editor.change_selections(Some(Autoscroll::fit()), cx, |selection| {
                    selection.move_cursors_with(|_, _, _| (overlay.point, SelectionGoal::None))
                });
                editor.clear_overlays(cx);
                editor.clear_highlights::<Self>(cx);
                editor.remove_keymap_context_layer::<Self>(cx);
                EditorState::None
            }
            TrimResult::Changed => {
                let trie = selection.trie();
                editor.clear_overlays(cx);
                Self::add_overlays(editor, trie.iter(), cx);
                EditorState::Selection(selection)
            }
            TrimResult::Err => {
                editor.clear_overlays(cx);
                editor.clear_highlights::<Self>(cx);
                editor.remove_keymap_context_layer::<Self>(cx);
                EditorState::None
            }
            TrimResult::NoChange => EditorState::Selection(selection),
        }
    }

    fn handle_trim_multipane(
        selection: Selection,
        keys: &str,
        cx: &mut WindowContext,
    ) -> EditorState {
        let handle = cx
            .window_handle()
            .downcast::<Workspace>()
            .and_then(|handle| handle.root(cx).ok());
        let Some(workspace_view) = handle else {
            return EditorState::None;
        };
        workspace_view.update(cx, |workspace, cx| {
            let editors = active_editor_views(workspace, cx);
            let (selection, res) = selection.record_str(keys);
            match res {
                TrimResult::Found(overlay) => {
                    let Some(editor) = editors
                        .iter()
                        .find(|editor| editor.entity_id() == overlay.editor_id)
                    else {
                        return EditorState::None;
                    };
                    workspace.activate_item(editor, cx);
                    editor.update(cx, |editor, cx| {
                        editor.change_selections(Some(Autoscroll::fit()), cx, |selection| {
                            selection
                                .move_cursors_with(|_, _, _| (overlay.point, SelectionGoal::None))
                        });
                    });
                    for editor in editors {
                        editor.update(cx, |editor, cx| {
                            editor.clear_overlays(cx);
                            editor.clear_highlights::<Self>(cx);
                            editor.remove_keymap_context_layer::<Self>(cx);
                        });
                    }
                    EditorState::None
                }
                TrimResult::Changed => {
                    let trie = selection.trie();
                    for editor in editors {
                        let iter = trie
                            .iter()
                            .filter(|(_, overlay)| overlay.editor_id == editor.entity_id());
                        editor.update(cx, |editor, cx| {
                            editor.clear_overlays(cx);
                            Self::add_overlays(editor, iter, cx);
                        });
                    }
                    EditorState::Selection(selection)
                }
                TrimResult::Err => {
                    for editor in editors {
                        editor.update(cx, |editor, cx| {
                            editor.clear_overlays(cx);
                            editor.clear_highlights::<Self>(cx);
                            editor.remove_keymap_context_layer::<Self>(cx);
                        });
                    }
                    EditorState::None
                }
                TrimResult::NoChange => EditorState::Selection(selection),
            }
        })
    }

    fn handle_record_char(n_char: NCharInput, keys: &str, _: &mut WindowContext) -> EditorState {
        let res = n_char.record_str(keys);
        match res {
            InputResult::ShowTrie(_query) => {
                // maybe just have a whole separate struct for the search version?
                // model? view?
                // let chan = self.update_easy_and_active_editor(cx, |easy, editor, cx| {
                //     cx.spawn(|view, cx| async move {
                //         Self::update(cx, |easy, window| {});
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
    fn handle_query(_query: String) {
        todo!()
    }

    fn add_overlays<'a>(
        editor: &mut Editor,
        trie_iter: impl Iterator<Item = (String, &'a OverlayState)>,
        cx: &mut ViewContext<Editor>,
    ) {
        for (seq, overlay) in trie_iter {
            let mut highlights = vec![(0..1, overlay.style.clone())];
            if seq.len() > 1 {
                highlights.push((
                    1..seq.len(),
                    HighlightStyle {
                        fade_out: Some(0.3),
                        ..overlay.style.clone()
                    },
                ));
            }
            editor.add_overlay(seq, overlay.point.clone(), 0.0, highlights, cx);
        }
    }
}

fn active_editor_views(workspace: &Workspace, cx: &mut WindowContext) -> Vec<View<Editor>> {
    let panes = workspace.panes();
    panes
        .iter()
        .filter_map(|pane| {
            pane.update(cx, |pane, _cx| {
                let active_item = pane.active_item();
                active_item.map(|item| item.downcast::<Editor>())
            })
            .flatten()
        })
        .collect()
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
