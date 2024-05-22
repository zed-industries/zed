use std::cmp::Ordering;

use collections::HashMap;
use editor::Editor;
use gpui::{
    impl_actions, AppContext, EntityId, FocusableView, Global, HighlightStyle, KeyContext,
    KeystrokeEvent, Subscription, ViewContext, WeakView,
};
use serde::Deserialize;
use text::Anchor;
use ui::{BorrowAppContext, WindowContext};

use crate::util::manh_distance;

mod perm;
mod util;

#[derive(Eq, PartialEq, Copy, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    /// Only include the conversation.
    BiDirectional,
    /// Send the current file as context.
    Forwards,
    /// Search the codebase and send relevant excerpts.
    Backwards,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct EasyMotionNChar {
    direction: Direction,
    n: u32,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct EasyMotionPattern(Direction);

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct EasyMotionWord(Direction);

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct EasyMotionSubWord(Direction);

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct EasyMotionFullWord(Direction);

impl_actions!(
    vim,
    [
        EasyMotionNChar,
        EasyMotionPattern,
        EasyMotionWord,
        EasyMotionSubWord,
        EasyMotionFullWord
    ]
);

pub struct EditorState {
    control: bool,
}

#[derive(Default)]
struct EasyMotion {
    active_editor: Option<WeakView<Editor>>,
    editor_subscription: Option<Subscription>,
    enabled: bool,
    editor_states: HashMap<EntityId, EditorState>,
}

struct Motion {
    anchor: Anchor,
    text: String,
}

impl Global for EasyMotion {}

pub fn init(cx: &mut AppContext) {
    cx.set_global(EasyMotion::default());
    cx.observe_keystrokes(observe_keystrokes).detach();
    cx.observe_new_views(|editor: &mut Editor, cx| register(editor, cx))
        .detach();
}

pub(crate) fn register(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
    if !editor.use_modal_editing() {
        return;
    }

    editor.register_action(cx.listener(easy_motion_n_char));
    editor.register_action(cx.listener(easy_motion_pattern));
    editor.register_action(cx.listener(easy_motion_word));
    editor.register_action(cx.listener(easy_motion_sub_word));
    editor.register_action(cx.listener(easy_motion_full_word));
}

fn easy_motion_n_char(
    editor: &mut Editor,
    _action: &EasyMotionNChar,
    cx: &mut ViewContext<Editor>,
) {
    let selections = editor.selections.newest_display(cx);
    let map = &editor.snapshot(cx).display_snapshot;
    let start = selections.start;

    let highlight = HighlightStyle {
        background_color: Some(gpui::red()),
        ..Default::default()
    };
    let mut word_starts = util::word_starts(&map, start, true, 30);
    word_starts.sort_unstable_by(|a, b| {
        let a_distance = manh_distance(a, &start, 1.0);
        let b_distance = manh_distance(b, &start, 1.0);
        if a_distance == b_distance {
            Ordering::Equal
        } else if a_distance < b_distance {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let trie = perm::Trie::new("asdghklqwertyuiopzxcvbnmfj".to_string(), word_starts.len());
    let perms = trie.trie_to_perms_rev();
    for (word_start, perm) in word_starts.iter().zip(perms.iter()) {
        editor.add_overlay(perm.clone(), *word_start, highlight, cx);
    }

    // editor.highlight_text::<EasyMotionHighlight>(
    //     ranges,
    //     HighlightStyle {
    //         background_color: Some(gpui::red()),
    //         ..Default::default()
    //     },
    //     cx,
    // );
    println!("easy_motion_n_chars");
}

fn easy_motion_pattern(
    _editor: &mut Editor,
    _action: &EasyMotionPattern,
    _cx: &mut ViewContext<Editor>,
) {
    println!("easy_motion_pattern");
}

fn easy_motion_word(_editor: &mut Editor, _action: &EasyMotionWord, _cx: &mut ViewContext<Editor>) {
    println!("easy_motion_word");
}

fn easy_motion_sub_word(
    _editor: &mut Editor,
    _action: &EasyMotionSubWord,
    _cx: &mut ViewContext<Editor>,
) {
    println!("easy_motion_sub_word");
}

fn easy_motion_full_word(
    _editor: &mut Editor,
    _action: &EasyMotionFullWord,
    _cx: &mut ViewContext<Editor>,
) {
    println!("easy_motion_full_word");
}

fn observe_keystrokes(keystroke_event: &KeystrokeEvent, cx: &mut WindowContext) {
    println!("easy_motion::observe_keystrokes");
    dbg!(&keystroke_event);
    if let Some(action) = keystroke_event
        .action
        .as_ref()
        .map(|action| action.boxed_clone())
    {
    } else if cx.has_pending_keystrokes() {
        return;
    }
}

impl EasyMotion {
    fn update_global<F, S>(cx: &mut WindowContext, update: F) -> S
    where
        F: FnOnce(&mut Self, &mut WindowContext) -> S,
    {
        cx.update_global(update)
    }

    fn read(cx: &mut AppContext) -> &Self {
        cx.global::<Self>()
    }

    fn update_active_editor<S>(
        &mut self,
        cx: &mut WindowContext,
        update: impl FnOnce(&mut EasyMotion, &mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        let editor = self.active_editor.clone()?.upgrade()?;
        Some(editor.update(cx, |editor, cx| update(self, editor, cx)))
    }

    fn sync_easy_motion_settings(&mut self, cx: &mut WindowContext) {
        self.update_active_editor(cx, |easy, editor, cx| {
            let state = easy.state();
            editor.set_input_enabled(
                !state
                    .map(|state| state.easy_motion_controlled())
                    .unwrap_or_default(),
            );
            if editor.is_focused(cx) {
                // editor.set_keymap_context_layer::<Self>(state.keymap_context_layer(), cx);
                // disable vim mode if a sub-editor (inline assist, rename, etc.) is focused
            } else if editor.focus_handle(cx).contains_focused(cx) {
                editor.remove_keymap_context_layer::<Self>(cx);
            }
        });
    }

    pub fn state(&self) -> Option<&EditorState> {
        self.active_editor
            .as_ref()
            .map(|active_editor| self.editor_states.get(&active_editor.entity_id()))
            .flatten()
    }

    // fn active_editor_input_ignored(text: Arc<str>, cx: &mut WindowContext) {
}

// impl Default for EditorState {
//     fn default() -> Self {
//         Self {
//             control: false,
//             original_buffer_state: MultiBuffer::default(),
//         }
//     }
// }

impl EditorState {
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
}
