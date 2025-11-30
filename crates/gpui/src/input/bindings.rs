use crate::{App, KeyBinding, actions};

actions!(
    input,
    [
        /// Delete the character before the cursor.
        Backspace,
        /// Delete the character after the cursor.
        Delete,
        /// Insert a tab character at the cursor position.
        Tab,
        /// Move the cursor one character to the left.
        Left,
        /// Move the cursor one character to the right.
        Right,
        /// Move the cursor up one visual line.
        Up,
        /// Move the cursor down one visual line.
        Down,
        /// Extend selection one character to the left.
        SelectLeft,
        /// Extend selection one character to the right.
        SelectRight,
        /// Extend selection up one visual line.
        SelectUp,
        /// Extend selection down one visual line.
        SelectDown,
        /// Select all text content.
        SelectAll,
        /// Move cursor to the start of the current line.
        Home,
        /// Move cursor to the end of the current line.
        End,
        /// Extend selection to the beginning of the content.
        SelectToBeginning,
        /// Extend selection to the end of the content.
        SelectToEnd,
        /// Move cursor to the beginning of the content.
        MoveToBeginning,
        /// Move cursor to the end of the content.
        MoveToEnd,
        /// Paste from clipboard at the cursor position.
        Paste,
        /// Cut selected text to clipboard.
        Cut,
        /// Copy selected text to clipboard.
        Copy,
        /// Insert a newline at the cursor position.
        Enter,
        /// Move cursor one word to the left.
        WordLeft,
        /// Move cursor one word to the right.
        WordRight,
        /// Extend selection one word to the left.
        SelectWordLeft,
        /// Extend selection one word to the right.
        SelectWordRight,
        /// Undo the last edit.
        Undo,
        /// Redo the last undone edit.
        Redo,
    ]
);

/// The key context used for input element keybindings.
pub const INPUT_CONTEXT: &str = "Input";

/// Keybindings configuration for input elements.
///
/// Each field is an `Option<KeyBinding>` to allow:
/// - Using defaults (via `Default::default()`)
/// - Overriding with custom bindings
/// - Unbinding keys by setting fields to `None`
///
/// The `Default` implementation returns platform-specific keybindings.
#[derive(Clone)]
pub struct InputBindings {
    /// Binding for deleting the character before the cursor.
    /// Default: `backspace`
    pub backspace: Option<KeyBinding>,

    /// Binding for deleting the character after the cursor.
    /// Default: `delete`
    pub delete: Option<KeyBinding>,

    /// Binding for inserting a tab character.
    /// Default: `tab`
    pub tab: Option<KeyBinding>,

    /// Binding for inserting a newline (multi-line) or confirming input (single-line).
    /// Default: `enter`
    pub enter: Option<KeyBinding>,

    /// Binding for moving the cursor one character to the left.
    /// Default: `left`
    pub left: Option<KeyBinding>,

    /// Binding for moving the cursor one character to the right.
    /// Default: `right`
    pub right: Option<KeyBinding>,

    /// Binding for moving the cursor up one line (multi-line) or to the start of the line (single-line).
    /// Default: `up`
    pub up: Option<KeyBinding>,

    /// Binding for moving the cursor down one line (multi-line) or to the end of the line (single-line).
    /// Default: `down`
    pub down: Option<KeyBinding>,

    /// Binding for extending selection one character to the left.
    /// Default: `shift-left`
    pub select_left: Option<KeyBinding>,

    /// Binding for extending selection one character to the right.
    /// Default: `shift-right`
    pub select_right: Option<KeyBinding>,

    /// Binding for extending selection up one line (multi-line) or to the start (single-line).
    /// Default: `shift-up`
    pub select_up: Option<KeyBinding>,

    /// Binding for extending selection down one line (multi-line) or to the end (single-line).
    /// Default: `shift-down`
    pub select_down: Option<KeyBinding>,

    /// Binding for selecting all text content.
    /// Default: `cmd-a` (macOS) / `ctrl-a` (other platforms)
    pub select_all: Option<KeyBinding>,

    /// Binding for moving cursor to the start of the current line.
    /// Default: `home`
    pub home: Option<KeyBinding>,

    /// Binding for moving cursor to the end of the current line.
    /// Default: `end`
    pub end: Option<KeyBinding>,

    /// Binding for moving cursor to the beginning of all content.
    /// Default: `cmd-up` (macOS) / `ctrl-home` (other platforms)
    pub move_to_beginning: Option<KeyBinding>,

    /// Binding for moving cursor to the end of all content.
    /// Default: `cmd-down` (macOS) / `ctrl-end` (other platforms)
    pub move_to_end: Option<KeyBinding>,

    /// Binding for extending selection to the beginning of all content.
    /// Default: `cmd-shift-up` (macOS) / `ctrl-shift-home` (other platforms)
    pub select_to_beginning: Option<KeyBinding>,

    /// Binding for extending selection to the end of all content.
    /// Default: `cmd-shift-down` (macOS) / `ctrl-shift-end` (other platforms)
    pub select_to_end: Option<KeyBinding>,

    /// Binding for moving cursor one word to the left.
    /// Default: `alt-left` (macOS) / `ctrl-left` (other platforms)
    pub word_left: Option<KeyBinding>,

    /// Binding for moving cursor one word to the right.
    /// Default: `alt-right` (macOS) / `ctrl-right` (other platforms)
    pub word_right: Option<KeyBinding>,

    /// Binding for extending selection one word to the left.
    /// Default: `alt-shift-left` (macOS) / `ctrl-shift-left` (other platforms)
    pub select_word_left: Option<KeyBinding>,

    /// Binding for extending selection one word to the right.
    /// Default: `alt-shift-right` (macOS) / `ctrl-shift-right` (other platforms)
    pub select_word_right: Option<KeyBinding>,

    /// Binding for copying selected text to clipboard.
    /// Default: `cmd-c` (macOS) / `ctrl-c` (other platforms)
    pub copy: Option<KeyBinding>,

    /// Binding for cutting selected text to clipboard.
    /// Default: `cmd-x` (macOS) / `ctrl-x` (other platforms)
    pub cut: Option<KeyBinding>,

    /// Binding for pasting from clipboard.
    /// Default: `cmd-v` (macOS) / `ctrl-v` (other platforms)
    pub paste: Option<KeyBinding>,

    /// Binding for undoing the last edit.
    /// Default: `cmd-z` (macOS) / `ctrl-z` (other platforms)
    pub undo: Option<KeyBinding>,

    /// Binding for redoing the last undone edit.
    /// Default: `cmd-shift-z` (macOS) / `ctrl-shift-z` (other platforms)
    pub redo: Option<KeyBinding>,
}

impl Default for InputBindings {
    /// Returns platform-specific default keybindings for input elements.
    fn default() -> Self {
        let context = Some(INPUT_CONTEXT);

        #[cfg(target_os = "macos")]
        {
            Self {
                backspace: Some(KeyBinding::new("backspace", Backspace, context)),
                delete: Some(KeyBinding::new("delete", Delete, context)),
                tab: Some(KeyBinding::new("tab", Tab, context)),
                enter: Some(KeyBinding::new("enter", Enter, context)),
                left: Some(KeyBinding::new("left", Left, context)),
                right: Some(KeyBinding::new("right", Right, context)),
                up: Some(KeyBinding::new("up", Up, context)),
                down: Some(KeyBinding::new("down", Down, context)),
                select_left: Some(KeyBinding::new("shift-left", SelectLeft, context)),
                select_right: Some(KeyBinding::new("shift-right", SelectRight, context)),
                select_up: Some(KeyBinding::new("shift-up", SelectUp, context)),
                select_down: Some(KeyBinding::new("shift-down", SelectDown, context)),
                select_all: Some(KeyBinding::new("cmd-a", SelectAll, context)),
                home: Some(KeyBinding::new("home", Home, context)),
                end: Some(KeyBinding::new("end", End, context)),
                move_to_beginning: Some(KeyBinding::new("cmd-up", MoveToBeginning, context)),
                move_to_end: Some(KeyBinding::new("cmd-down", MoveToEnd, context)),
                select_to_beginning: Some(KeyBinding::new(
                    "cmd-shift-up",
                    SelectToBeginning,
                    context,
                )),
                select_to_end: Some(KeyBinding::new("cmd-shift-down", SelectToEnd, context)),
                word_left: Some(KeyBinding::new("alt-left", WordLeft, context)),
                word_right: Some(KeyBinding::new("alt-right", WordRight, context)),
                select_word_left: Some(KeyBinding::new("alt-shift-left", SelectWordLeft, context)),
                select_word_right: Some(KeyBinding::new(
                    "alt-shift-right",
                    SelectWordRight,
                    context,
                )),
                copy: Some(KeyBinding::new("cmd-c", Copy, context)),
                cut: Some(KeyBinding::new("cmd-x", Cut, context)),
                paste: Some(KeyBinding::new("cmd-v", Paste, context)),
                undo: Some(KeyBinding::new("cmd-z", Undo, context)),
                redo: Some(KeyBinding::new("cmd-shift-z", Redo, context)),
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            Self {
                backspace: Some(KeyBinding::new("backspace", Backspace, context)),
                delete: Some(KeyBinding::new("delete", Delete, context)),
                tab: Some(KeyBinding::new("tab", Tab, context)),
                enter: Some(KeyBinding::new("enter", Enter, context)),
                left: Some(KeyBinding::new("left", Left, context)),
                right: Some(KeyBinding::new("right", Right, context)),
                up: Some(KeyBinding::new("up", Up, context)),
                down: Some(KeyBinding::new("down", Down, context)),
                select_left: Some(KeyBinding::new("shift-left", SelectLeft, context)),
                select_right: Some(KeyBinding::new("shift-right", SelectRight, context)),
                select_up: Some(KeyBinding::new("shift-up", SelectUp, context)),
                select_down: Some(KeyBinding::new("shift-down", SelectDown, context)),
                select_all: Some(KeyBinding::new("ctrl-a", SelectAll, context)),
                home: Some(KeyBinding::new("home", Home, context)),
                end: Some(KeyBinding::new("end", End, context)),
                move_to_beginning: Some(KeyBinding::new("ctrl-home", MoveToBeginning, context)),
                move_to_end: Some(KeyBinding::new("ctrl-end", MoveToEnd, context)),
                select_to_beginning: Some(KeyBinding::new(
                    "ctrl-shift-home",
                    SelectToBeginning,
                    context,
                )),
                select_to_end: Some(KeyBinding::new("ctrl-shift-end", SelectToEnd, context)),
                word_left: Some(KeyBinding::new("ctrl-left", WordLeft, context)),
                word_right: Some(KeyBinding::new("ctrl-right", WordRight, context)),
                select_word_left: Some(KeyBinding::new("ctrl-shift-left", SelectWordLeft, context)),
                select_word_right: Some(KeyBinding::new(
                    "ctrl-shift-right",
                    SelectWordRight,
                    context,
                )),
                copy: Some(KeyBinding::new("ctrl-c", Copy, context)),
                cut: Some(KeyBinding::new("ctrl-x", Cut, context)),
                paste: Some(KeyBinding::new("ctrl-v", Paste, context)),
                undo: Some(KeyBinding::new("ctrl-z", Undo, context)),
                redo: Some(KeyBinding::new("ctrl-shift-z", Redo, context)),
            }
        }
    }
}

impl InputBindings {
    /// Creates an empty `InputBindings` with all fields set to `None`.
    ///
    /// Use this as a starting point when you want to override only specific bindings:
    ///
    /// ```ignore
    /// let bindings = InputBindings::empty();
    /// bindings.select_all = Some(KeyBinding::new("ctrl-shift-a", SelectAll, Some(INPUT_CONTEXT)));
    /// ```
    pub fn empty() -> Self {
        Self {
            backspace: None,
            delete: None,
            tab: None,
            enter: None,
            left: None,
            right: None,
            up: None,
            down: None,
            select_left: None,
            select_right: None,
            select_up: None,
            select_down: None,
            select_all: None,
            home: None,
            end: None,
            move_to_beginning: None,
            move_to_end: None,
            select_to_beginning: None,
            select_to_end: None,
            word_left: None,
            word_right: None,
            select_word_left: None,
            select_word_right: None,
            copy: None,
            cut: None,
            paste: None,
            undo: None,
            redo: None,
        }
    }

    /// Merges these bindings with defaults, using `self` values where `Some`,
    /// falling back to defaults for `None` values.
    pub fn merged_with_defaults(self) -> Self {
        let defaults = Self::default();
        Self {
            backspace: self.backspace.or(defaults.backspace),
            delete: self.delete.or(defaults.delete),
            tab: self.tab.or(defaults.tab),
            enter: self.enter.or(defaults.enter),
            left: self.left.or(defaults.left),
            right: self.right.or(defaults.right),
            up: self.up.or(defaults.up),
            down: self.down.or(defaults.down),
            select_left: self.select_left.or(defaults.select_left),
            select_right: self.select_right.or(defaults.select_right),
            select_up: self.select_up.or(defaults.select_up),
            select_down: self.select_down.or(defaults.select_down),
            select_all: self.select_all.or(defaults.select_all),
            home: self.home.or(defaults.home),
            end: self.end.or(defaults.end),
            move_to_beginning: self.move_to_beginning.or(defaults.move_to_beginning),
            move_to_end: self.move_to_end.or(defaults.move_to_end),
            select_to_beginning: self.select_to_beginning.or(defaults.select_to_beginning),
            select_to_end: self.select_to_end.or(defaults.select_to_end),
            word_left: self.word_left.or(defaults.word_left),
            word_right: self.word_right.or(defaults.word_right),
            select_word_left: self.select_word_left.or(defaults.select_word_left),
            select_word_right: self.select_word_right.or(defaults.select_word_right),
            copy: self.copy.or(defaults.copy),
            cut: self.cut.or(defaults.cut),
            paste: self.paste.or(defaults.paste),
            undo: self.undo.or(defaults.undo),
            redo: self.redo.or(defaults.redo),
        }
    }

    /// Collects all `Some` bindings into a `Vec<KeyBinding>`.
    pub fn into_bindings(self) -> Vec<KeyBinding> {
        [
            self.backspace,
            self.delete,
            self.tab,
            self.enter,
            self.left,
            self.right,
            self.up,
            self.down,
            self.select_left,
            self.select_right,
            self.select_up,
            self.select_down,
            self.select_all,
            self.home,
            self.end,
            self.move_to_beginning,
            self.move_to_end,
            self.select_to_beginning,
            self.select_to_end,
            self.word_left,
            self.word_right,
            self.select_word_left,
            self.select_word_right,
            self.copy,
            self.cut,
            self.paste,
            self.undo,
            self.redo,
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

/// Binds input keybindings to the application.
///
/// If no bindings are provided, platform defaults are used. When bindings are
/// provided, they are used exactly as-is - fields set to `None` will not have
/// any keybinding registered for that action.
///
/// # Examples
///
/// Use all platform defaults:
///
/// ```ignore
/// bind_input_keys(cx, None);
/// ```
///
/// Unbind a specific key while keeping other defaults:
///
/// ```ignore
/// bind_input_keys(cx, InputBindings {
///     up: None, // Unbind up arrow
///     ..Default::default()
/// });
/// ```
///
/// Override a specific binding while keeping other defaults:
///
/// ```ignore
/// bind_input_keys(cx, InputBindings {
///     select_all: Some(KeyBinding::new("ctrl-shift-a", SelectAll, Some(INPUT_CONTEXT))),
///     ..Default::default()
/// });
/// ```
///
/// Use [`InputBindings::empty()`] with [`merged_with_defaults()`](InputBindings::merged_with_defaults)
/// if you only want to specify a few custom bindings and fill in the rest with defaults:
///
/// ```ignore
/// let mut bindings = InputBindings::empty();
/// bindings.select_all = Some(KeyBinding::new("ctrl-shift-a", SelectAll, Some(INPUT_CONTEXT)));
/// bind_input_keys(cx, bindings.merged_with_defaults());
/// ```
pub fn bind_input_keys(cx: &mut App, bindings: impl Into<Option<InputBindings>>) {
    let bindings = bindings.into().unwrap_or_default();
    cx.bind_keys(bindings.into_bindings());
}
