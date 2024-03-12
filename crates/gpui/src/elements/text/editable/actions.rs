use crate::{self as gpui, AppContext, KeyBinding, KeyContext};

crate::actions!(
    text::editable,
    [
        Backspace,
        Copy,
        Cut,
        Delete,
        MoveDown,
        MoveLeft,
        MoveRight,
        MoveUp,
        MoveToBeginning,
        MoveToEnd,
        MoveToNextWordEnd,
        MoveToPreviousWordStart,
        Newline,
        Paste,
        Redo,
        SelectAll,
        SelectLeft,
        SelectRight,
        SelectToBeginning,
        SelectToEnd,
        SelectToNextWordEnd,
        SelectToPreviousWordStart,
        Tab,
        TabPrev,
        Undo,
    ]
);

const KEY_CONTEXT: &str = "EditableText";

pub fn key_context() -> KeyContext {
    let mut key_context = KeyContext::default();
    key_context.add(KEY_CONTEXT);
    key_context
}

pub fn bind_keys(cx: &mut AppContext) {
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, Some(KEY_CONTEXT)),
        KeyBinding::new("cmd-c", Copy, Some(KEY_CONTEXT)),
        KeyBinding::new("cmd-x", Cut, Some(KEY_CONTEXT)),
        KeyBinding::new("down", MoveDown, Some(KEY_CONTEXT)),
        KeyBinding::new("right", MoveRight, Some(KEY_CONTEXT)),
        KeyBinding::new("left", MoveLeft, Some(KEY_CONTEXT)),
        KeyBinding::new("up", MoveUp, Some(KEY_CONTEXT)),
        KeyBinding::new("cmd-left", MoveToBeginning, Some(KEY_CONTEXT)),
        KeyBinding::new("cmd-right", MoveToEnd, Some(KEY_CONTEXT)),
        KeyBinding::new("alt-right", MoveToNextWordEnd, Some(KEY_CONTEXT)),
        KeyBinding::new("alt-left", MoveToPreviousWordStart, Some(KEY_CONTEXT)),
        KeyBinding::new("enter", Newline, Some(KEY_CONTEXT)),
        KeyBinding::new("cmd-v", Paste, Some(KEY_CONTEXT)),
        KeyBinding::new("cmd-shift-z", Redo, Some(KEY_CONTEXT)),
        KeyBinding::new("cmd-a", SelectAll, Some(KEY_CONTEXT)),
        KeyBinding::new("shift-left", SelectLeft, Some(KEY_CONTEXT)),
        KeyBinding::new("shift-right", SelectRight, Some(KEY_CONTEXT)),
        KeyBinding::new("cmd-shift-left", SelectToBeginning, Some(KEY_CONTEXT)),
        KeyBinding::new("cmd-shift-right", SelectToEnd, Some(KEY_CONTEXT)),
        KeyBinding::new("alt-shift-right", SelectToNextWordEnd, Some(KEY_CONTEXT)),
        KeyBinding::new(
            "alt-shift-left",
            SelectToPreviousWordStart,
            Some(KEY_CONTEXT),
        ),
        KeyBinding::new("tab", Tab, Some(KEY_CONTEXT)),
        KeyBinding::new("shift-tab", TabPrev, Some(KEY_CONTEXT)),
        KeyBinding::new("cmd-z", Undo, Some(KEY_CONTEXT)),
    ]);
}
