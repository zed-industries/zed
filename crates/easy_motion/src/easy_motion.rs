use collections::HashMap;
use editor::display_map::DisplaySnapshot;
use futures::future::join_all;
use search::{search, search_multipane};
use serde::Deserialize;
use std::cmp::Ordering;
use std::{fmt, mem};

use editor::scroll::Autoscroll;
use editor::{DisplayPoint, Editor};
use gpui::{
    actions, impl_actions, saturate, AppContext, AsyncAppContext, Bounds, Entity, EntityId, Global,
    HighlightStyle, Hsla, KeystrokeEvent, Model, ModelContext, Point, Subscription, View,
    ViewContext, WeakView,
};
use perm::{TrieBuilder, TrimResult};
use settings::Settings;
use text::{Bias, Selection as TextSelection, SelectionGoal};
use theme::ThemeSettings;
use ui::{Context, Pixels, VisualContext, WindowContext};
use workspace::Workspace;

use editor_state::{EditorState, InputResult, OverlayState, Selection};
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

    workspace.register_action(|workspace: &mut Workspace, action: &NChar, cx| {
        EasyMotion::n_char(action, workspace, cx);
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

    pub fn update_async<F, S>(cx: &mut AsyncAppContext, f: F) -> Option<S>
    where
        F: FnOnce(&mut EasyMotion, &mut ModelContext<EasyMotion>) -> S,
    {
        EasyMotion::global_async(cx).and_then(|easy| easy.update(cx, f).ok())
    }

    pub fn global_async(cx: &AsyncAppContext) -> Option<Model<Self>> {
        cx.try_read_global::<GlobalEasyMotion, _>(|global_easy, _cx| global_easy.0.clone())
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
        let weak_editor =
            EasyMotion::read_with(cx, |easy, _cx| easy.active_editor.clone()).flatten()?;
        let editor = weak_editor.upgrade()?;
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

    fn word(action: &Word, workspace: &Workspace, cx: &mut WindowContext) {
        let Word(direction) = *action;
        // TODO other directions?
        // not sure if check for multiple editors is totally necessary
        if matches!(direction, Direction::BiDirectional)
            && workspace.is_split()
            && workspace_has_multiple_editors(workspace, cx)
        {
            EasyMotion::word_multipane(WordType::Word, workspace, cx);
        } else {
            EasyMotion::word_single_pane(WordType::Word, direction, cx);
        }
    }

    fn sub_word(action: &SubWord, workspace: &Workspace, cx: &mut WindowContext) {
        let SubWord(direction) = *action;
        // TODO other directions?
        // not sure if check for multiple editors is totally necessary
        if matches!(direction, Direction::BiDirectional)
            && workspace.is_split()
            && workspace_has_multiple_editors(workspace, cx)
        {
            // todo?
        } else {
            EasyMotion::word_single_pane(WordType::SubWord, direction, cx);
        }
    }

    fn full_word(action: &FullWord, workspace: &Workspace, cx: &mut WindowContext) {
        let FullWord(direction) = *action;
        // TODO other directions?
        // not sure if check for multiple editors is totally necessary
        if matches!(direction, Direction::BiDirectional)
            && workspace.is_split()
            && workspace_has_multiple_editors(workspace, cx)
        {
            // todo?
        } else {
            EasyMotion::word_single_pane(WordType::FullWord, direction, cx);
        }
    }

    fn word_single_pane(word_type: WordType, direction: Direction, cx: &mut WindowContext) {
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
        if word_starts.is_empty() {
            return EditorState::None;
        }

        sort_matches_display(&mut word_starts, &selections.start);

        let (style_0, style_1, style_2) = get_highlights(cx);
        let trie = TrieBuilder::new("asdghklqwertyuiopzxcvbnmfj".to_string(), word_starts.len())
            .populate_with(true, word_starts, |seq, point| {
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

    fn word_multipane(word_type: WordType, workspace: &Workspace, cx: &mut WindowContext) {
        let Some(active_editor) = Self::active_editor(cx) else {
            return;
        };
        let active_editor_id = active_editor.entity_id();

        let panes = workspace.panes();

        let editors = panes
            .iter()
            .filter_map(|pane| {
                pane.update(cx, |pane, _cx| {
                    pane.active_item()
                        .and_then(|item| item.downcast::<Editor>())
                })
                .map(|editor| {
                    let bounding_box = workspace.center().bounding_box_for_pane(pane).unwrap();
                    (editor, bounding_box)
                })
            })
            .collect::<Vec<_>>();

        // get words along with their display points within their editors
        // as well as a rough absolute position for sorting purposes
        let mut matches = editors
            .iter()
            .flat_map(|(editor_view, bounding_box)| {
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
            .collect::<Vec<_>>();

        let cursor = editors
            .iter()
            .find(|(editor_view, _)| editor_view.entity_id() == active_editor_id)
            .map(|(editor_view, bounding_box)| {
                editor_view.update(cx, |editor, cx| {
                    Self::get_cursor_pixels(bounding_box, editor, cx)
                })
            })
            .unwrap();

        sort_matches_pixel(&mut matches, &cursor);

        let len = matches.len();
        let matches = matches.into_iter().map(|(point, id, _)| (point, id));
        let (style_0, style_1, style_2) = get_highlights(cx);
        let trie = TrieBuilder::new("asdghklqwertyuiopzxcvbnmfj".to_string(), len).populate_with(
            true,
            matches,
            |seq, point| {
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
            },
        );

        let new_state = EditorState::new_selection(trie);
        let just_editors = editors.into_iter().map(|(editor, _)| editor);
        Self::update_editors(&new_state, true, just_editors, cx);

        Self::update(cx, move |easy, _cx| {
            easy.multipane_state = Some(new_state);
        });
    }

    fn get_cursor_pixels(
        bounding_box: &Bounds<Pixels>,
        editor: &Editor,
        cx: &mut ViewContext<Editor>,
    ) -> Point<Pixels> {
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
    }

    fn pattern(_action: &Pattern, _cx: &WindowContext) {
        todo!()
    }

    fn n_char(action: &NChar, workspace: &Workspace, cx: &mut WindowContext) {
        let n = action.n;
        if workspace.is_split() && workspace_has_multiple_editors(workspace, cx) {
            let panes = workspace.panes();

            let editors = panes
                .into_iter()
                .filter_map(|pane| {
                    pane.update(cx, |pane, _cx| {
                        pane.active_item()
                            .and_then(|item| item.downcast::<Editor>())
                    })
                })
                .collect::<Vec<_>>();

            let new_state = EditorState::new_n_char(n as usize);
            Self::update_editors(&new_state, true, editors.into_iter(), cx);

            Self::update(cx, move |easy, _cx| {
                easy.multipane_state = Some(new_state);
            });
        } else {
            let Some(active_editor) = Self::active_editor(cx) else {
                return;
            };
            let entity_id = active_editor.entity_id();

            let new_state = active_editor.update(cx, |editor, cx| {
                let new_state = EditorState::new_n_char(action.n as usize);
                let ctx = new_state.keymap_context_layer();
                editor.set_keymap_context_layer::<Self>(ctx, cx);
                new_state
            });

            Self::update(cx, move |easy, _cx| {
                easy.editor_states.insert(entity_id, new_state);
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
            EditorState::NCharInput(char_input) => {
                let res = char_input.record_str(keys);
                match res {
                    InputResult::ShowTrie(query) => Self::show_trie(query, editor, cx),
                    // do nothing
                    InputResult::Recording(n_char) => EditorState::NCharInput(n_char),
                }
            }
            EditorState::Selection(selection) => Self::handle_trim(selection, keys, editor, cx),
            EditorState::PendingSearch => EditorState::PendingSearch,
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
            EditorState::NCharInput(char_input) => {
                let res = char_input.record_str(keys);
                match res {
                    InputResult::ShowTrie(query) => Self::show_trie_multipane(query, cx),
                    // do nothing
                    InputResult::Recording(n_char) => EditorState::NCharInput(n_char),
                }
            }
            EditorState::Selection(selection) => Self::handle_trim_multipane(selection, keys, cx),
            EditorState::PendingSearch => EditorState::PendingSearch,
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

    fn handle_record_char_impl(
        mut matches: Vec<DisplayPoint>,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> EditorState {
        let selections = editor.selections.newest_display(cx);
        let snapshot = editor.snapshot(cx);
        let map = &snapshot.display_snapshot;

        if matches.is_empty() {
            return EditorState::None;
        }
        matches.sort_unstable_by(|a, b| {
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

        let (style_0, style_1, style_2) = get_highlights(cx);
        let trie = TrieBuilder::new("asdghklqwertyuiopzxcvbnmfj".to_string(), matches.len())
            .populate_with(true, matches, |seq, point| {
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

        // todo dimming
        if true {
            let start = DisplayPoint::zero();
            let end = end_of_document(map);
            let anchor_start = map.display_point_to_anchor(start, Bias::Left);
            let anchor_end = map.display_point_to_anchor(end, Bias::Left);
            let highlight = HighlightStyle {
                fade_out: Some(0.7),
                ..Default::default()
            };
            editor.highlight_text::<Self>(vec![anchor_start..anchor_end], highlight, cx);
        }
        EditorState::new_selection(trie)
    }

    fn show_trie(query: String, editor: &mut Editor, cx: &mut ViewContext<Editor>) -> EditorState {
        let task = search(query.as_str(), editor, cx);
        let Some(task) = task else {
            return EditorState::None;
        };
        cx.spawn(|editor, mut cx| async move {
            let entity_id = editor.entity_id();
            let Some(editor) = editor.upgrade() else {
                return;
            };
            let matches = task.await;
            let res = editor.update(&mut cx, move |editor, cx| {
                editor.clear_search_within_ranges(cx);
                let new_state = Self::handle_record_char_impl(matches, editor, cx);
                // should already be set
                // let ctx = new_state.keymap_context_layer();
                // editor.set_keymap_context_layer::<Self>(ctx, cx);
                new_state
            });
            match res {
                Ok(state) => {
                    Self::update_async(&mut cx, move |easy, _cx| {
                        easy.editor_states.insert(entity_id, state);
                    });
                }
                Err(err) => {
                    dbg!(err);
                }
            }
        })
        .detach();

        EditorState::None
    }

    fn show_trie_multipane(query: String, cx: &mut WindowContext) -> EditorState {
        let Some(active_editor_id) = Self::active_editor(cx).map(|editor| editor.entity_id())
        else {
            return EditorState::None;
        };

        let handle = cx
            .window_handle()
            .downcast::<Workspace>()
            .and_then(|handle| handle.root(cx).ok());
        let Some(workspace_view) = handle else {
            return EditorState::None;
        };

        let state = workspace_view.update(cx, |workspace, cx| {
            let panes = workspace.panes();

            let editors = panes
                .iter()
                .filter_map(|pane| {
                    pane.update(cx, |pane, _cx| {
                        pane.active_item()
                            .and_then(|item| item.downcast::<Editor>())
                    })
                    .map(|editor| {
                        let bounding_box = workspace.center().bounding_box_for_pane(pane).unwrap();
                        (editor, bounding_box)
                    })
                })
                .collect::<Vec<_>>();

            let tasks = editors
                .iter()
                .map(|(editor, bounding_box)| {
                    let entity_id = editor.entity_id();
                    editor.update(cx, |editor, cx| {
                        search_multipane(query.as_str(), *bounding_box, entity_id, editor, cx)
                    })
                })
                .collect::<Option<Vec<_>>>();
            let Some(tasks) = tasks else {
                // there was an issue with at least one of the searches
                // todo need to remove search highlights if there's an issue
                return EditorState::None;
            };

            // downgrade before spawn?
            let cursor = editors
                .iter()
                .find(|(editor_view, _)| editor_view.entity_id() == active_editor_id)
                .map(|(editor_view, bounding_box)| {
                    editor_view.update(cx, |editor, cx| {
                        Self::get_cursor_pixels(bounding_box, editor, cx)
                    })
                })
                .unwrap();

            cx.spawn(move |_view, mut cx| async move {
                // let matches = tasks.
                let cursor = cursor;
                let tasks = join_all(tasks).await;

                let mut matches = tasks.into_iter().flatten().collect::<Vec<_>>();
                sort_matches_pixel(&mut matches, &cursor);

                let len = matches.len();
                let matches = matches.into_iter().map(|(point, id, _)| (point, id));
                let (style_0, style_1, style_2) = get_highlights_async(&cx);
                let trie = TrieBuilder::new("asdghklqwertyuiopzxcvbnmfj".to_string(), len)
                    .populate_with(true, matches, |seq, point| {
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

                let new_state = EditorState::new_selection(trie);
                let just_editors = editors.into_iter().map(|(editor, _)| editor);
                Self::update_editors(&new_state, true, just_editors, &mut cx);

                Self::update_async(&mut cx, move |easy, _cx| {
                    easy.multipane_state = Some(new_state);
                });
            })
            .detach();
            EditorState::PendingSearch
        });
        state
    }

    fn update_editors(
        state: &EditorState,
        dimming: bool,
        editors: impl Iterator<Item = View<Editor>>,
        cx: &mut impl VisualContext,
    ) {
        // filter trie entries by editor and add overlays
        let ctx = state.keymap_context_layer();
        match state {
            EditorState::None => {
                for editor in editors {
                    editor.update(cx, |editor, cx| {
                        editor.clear_highlights::<Self>(cx);
                        editor.set_keymap_context_layer::<Self>(ctx.clone(), cx);
                    });
                }
            }
            EditorState::Selection(selection) => {
                for editor in editors {
                    let trie = selection.trie();
                    let trie_iter = trie
                        .iter()
                        .filter(|(_seq, overlay)| overlay.editor_id == editor.entity_id());

                    editor.update(cx, |editor, cx| {
                        editor.clear_search_within_ranges(cx);
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
                            editor.highlight_text::<Self>(
                                vec![anchor_start..anchor_end],
                                highlight,
                                cx,
                            );
                        }

                        editor.set_keymap_context_layer::<Self>(ctx.clone(), cx);
                    });
                }
            }
            EditorState::NCharInput(_) => {
                for editor in editors {
                    editor.update(cx, |editor, cx| {
                        editor.clear_highlights::<Self>(cx);
                        editor.set_keymap_context_layer::<Self>(ctx.clone(), cx);
                    });
                }
            }
            EditorState::PendingSearch => {}
        }
    }

    fn cancel(workspace: &Workspace, cx: &mut WindowContext) {
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
        } else if let Some(editor) = editor.and_then(|editor| editor.upgrade()) {
            editor.update(cx, |editor, cx| {
                editor.clear_overlays(cx);
                editor.clear_highlights::<Self>(cx);
                editor.remove_keymap_context_layer::<Self>(cx);
            });
        }
    }

    fn add_overlays<'a>(
        editor: &mut Editor,
        trie_iter: impl Iterator<Item = (String, &'a OverlayState)>,
        cx: &mut ViewContext<Editor>,
    ) {
        for (seq, overlay) in trie_iter {
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
            editor.add_overlay(seq, overlay.point, 0.0, highlights, cx);
        }
    }
}

fn workspace_has_multiple_editors(workspace: &Workspace, cx: &WindowContext) -> bool {
    let panes = workspace.panes();
    panes
        .iter()
        .filter(|pane| {
            pane.read(cx)
                .active_item()
                .and_then(|item| item.downcast::<Editor>())
                .is_some()
        })
        .take(2)
        .count()
        == 2
}

fn active_editor_views(workspace: &Workspace, cx: &WindowContext) -> Vec<View<Editor>> {
    let panes = workspace.panes();
    panes
        .iter()
        .filter_map(|pane| {
            pane.read(cx)
                .active_item()
                .and_then(|item| item.downcast::<Editor>())
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

fn get_highlights_async(cx: &AsyncAppContext) -> (HighlightStyle, HighlightStyle, HighlightStyle) {
    let (bg, color_0, color_1, color_2) = ThemeSettings::try_read_global(cx, |theme| {
        let theme = theme.active_theme.clone();
        let players = &theme.players().0;
        let bg = theme.colors().background;
        let color_0 = saturate(players[0].cursor, 1.0);
        let color_1 = saturate(players[2].cursor, 1.0);
        let color_2 = saturate(players[3].cursor, 1.0);
        (bg, color_0, color_1, color_2)
    })
    .unwrap_or((Hsla::white(), Hsla::red(), Hsla::green(), Hsla::blue()));

    let style_0 = HighlightStyle {
        color: Some(color_0),
        background_color: Some(bg),
        ..HighlightStyle::default()
    };
    let style_1 = HighlightStyle {
        color: Some(color_1),
        background_color: Some(bg),
        ..HighlightStyle::default()
    };
    let style_2 = HighlightStyle {
        color: Some(color_2),
        background_color: Some(bg),
        ..HighlightStyle::default()
    };
    (style_0, style_1, style_2)
}

fn sort_matches_pixel(
    matches: &mut Vec<(DisplayPoint, EntityId, Point<Pixels>)>,
    cursor: &Point<Pixels>,
) {
    matches.sort_unstable_by(|a, b| {
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
}

fn sort_matches_display(matches: &mut Vec<DisplayPoint>, cursor: &DisplayPoint) {
    matches.sort_unstable_by(|a, b| {
        let a_distance = manh_distance(a, cursor, 2.5);
        let b_distance = manh_distance(b, cursor, 2.5);
        if a_distance == b_distance {
            Ordering::Equal
        } else if a_distance < b_distance {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}
