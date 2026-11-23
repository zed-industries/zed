//! Helix-style modal editing for the editor.
//!
//! Active when `helix_mode` is enabled together with the `helix-next` feature
//! flag; otherwise Helix mode is provided by the `vim` crate.

mod mode;
mod movement;
mod registers;
mod selection;
#[cfg(test)]
mod test;

use editor::{
    Editor, EditorEvent, EditorSettings, HighlightKey, MultiBufferOffset, SelectionEffects,
};
use gpui::{
    Action, App, AppContext as _, Context, Entity, EventEmitter, KeyContext, KeystrokeEvent,
    Subscription, WeakEntity, Window, actions,
};
use language::{CursorShape, TransactionId};
use modal_editor::{ModeIndicatorFocused, ModeIndicatorSource, ModeStatus};
use schemars::JsonSchema;
use serde::Deserialize;
use settings::{
    ModeContent, RegisterSetting, Settings, UseSystemClipboard, VimInsertModeCursorShape,
};
use theme::ActiveTheme as _;
use vim_mode_setting::{HelixModeSetting, VimModeSetting, observe_modal_editing_changes_in};

use crate::{mode::InsertSession, selection::HelixRange};

pub use registers::{DEFAULT_REGISTER, HelixRegisters};

actions!(
    helix,
    [
        /// Selects the register the next command uses, from the next typed character.
        SelectRegister,
    ]
);

/// Appends a digit to the count of the next command.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = helix)]
struct PushDigit(usize);

/// Helix ignores digits that would make the count larger than this.
const MAX_COUNT: usize = 100_000_000;

/// Initializes the `helix` crate.
pub fn init(cx: &mut App) {
    modal_editor::register_mode_indicator_source::<Helix>(cx);
    cx.observe_new(Helix::register).detach();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    Normal,
    Select,
    Insert,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Mode::Normal => "NORMAL",
            Mode::Select => "SELECT",
            Mode::Insert => "INSERT",
        })
    }
}

impl Mode {
    /// The value of the `helix_mode` key context.
    fn context_id(self) -> &'static str {
        match self {
            Mode::Normal => "normal",
            Mode::Select => "select",
            Mode::Insert => "insert",
        }
    }
}

/// A command waiting for the next typed character.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PendingInput {
    Register,
}

impl PendingInput {
    /// The value of the `helix_pending` key context.
    fn context_id(self) -> &'static str {
        match self {
            PendingInput::Register => "register",
        }
    }
}

/// Attaches the Helix state to an editor.
pub(crate) struct HelixAddon {
    pub(crate) entity: Entity<Helix>,
    /// The editor's key context depends on the Helix state, and the window
    /// only rebuilds it when the editor itself is notified.
    _observe_state: Subscription,
}

impl editor::Addon for HelixAddon {
    fn extend_key_context(&self, key_context: &mut KeyContext, cx: &App) {
        self.entity.read(cx).extend_key_context(key_context);
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// The Helix state of one editor.
pub(crate) struct Helix {
    pub(crate) mode: Mode,
    count: Option<usize>,
    pub(crate) selected_register: Option<char>,
    pending_input: Option<PendingInput>,
    insert_session: Option<InsertSession>,
    editor: WeakEntity<Editor>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<ModeIndicatorFocused> for Helix {}

impl ModeIndicatorSource for Helix {
    fn mode_status(&self, cx: &App) -> ModeStatus {
        let colors = cx.theme().colors();
        let (foreground, background) = match self.mode {
            Mode::Normal => (
                colors.vim_helix_normal_foreground,
                colors.vim_helix_normal_background,
            ),
            Mode::Select => (
                colors.vim_helix_select_foreground,
                colors.vim_helix_select_background,
            ),
            Mode::Insert => (colors.vim_insert_foreground, colors.vim_insert_background),
        };
        let mut pending = String::new();
        if let Some(register) = self.selected_register {
            pending.push('"');
            pending.push(register);
        }
        if self.pending_input == Some(PendingInput::Register) {
            pending.push('"');
        }
        if let Some(count) = self.count {
            pending.push_str(&count.to_string());
        }
        ModeStatus {
            label: None,
            mode: self.mode.to_string(),
            pending,
            foreground,
            background,
        }
    }
}

impl Helix {
    fn new(window: &mut Window, cx: &mut Context<Editor>) -> Entity<Self> {
        let editor = cx.entity();
        cx.new(|cx| Helix {
            mode: HelixSettings::get_global(cx).default_mode,
            count: None,
            selected_register: None,
            pending_input: None,
            insert_session: None,
            editor: editor.downgrade(),
            _subscriptions: vec![
                cx.observe_keystrokes(Self::observe_keystrokes),
                cx.subscribe_in(
                    &editor,
                    window,
                    |helix: &mut Helix, _, event, window, cx| {
                        helix.handle_editor_event(event, window, cx)
                    },
                ),
            ],
        })
    }

    fn extend_key_context(&self, key_context: &mut KeyContext) {
        // While a command waits for a character, keys must reach the editor
        // as text instead of triggering the usual bindings.
        if let Some(pending_input) = self.pending_input {
            key_context.set("helix_pending", pending_input.context_id());
        } else if self.mode != Mode::Insert {
            key_context.add("Helix");
        }
        if self.count.is_some() {
            key_context.add("helix_count");
        }
        key_context.set("helix_mode", self.mode.context_id());
    }

    fn register(editor: &mut Editor, window: Option<&mut Window>, cx: &mut Context<Editor>) {
        let Some(window) = window else {
            return;
        };
        if !editor.use_modal_editing() {
            return;
        }

        let mut was_enabled = HelixModeSetting::uses_helix_crate(cx);
        observe_modal_editing_changes_in(window, cx, move |editor, window, cx| {
            let enabled = HelixModeSetting::uses_helix_crate(cx);
            if enabled == was_enabled {
                return;
            }
            was_enabled = enabled;
            if enabled {
                Self::activate(editor, window, cx);
            } else {
                Self::deactivate(editor, window, cx);
            }
        })
        .detach();
        if was_enabled {
            Self::activate(editor, window, cx);
        }
    }

    fn activate(editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>) {
        let helix = Helix::new(window, cx);
        editor.register_addon(HelixAddon {
            entity: helix.clone(),
            _observe_state: cx.observe(&helix, |_, _, cx| cx.notify()),
        });
        helix.update(cx, |_, cx| {
            Self::action(editor, cx, |helix, &PushDigit(digit), _, cx| {
                helix.push_digit(digit, cx)
            });
            Self::action(editor, cx, |helix, _: &SelectRegister, window, cx| {
                helix.await_input(PendingInput::Register, window, cx)
            });
            mode::register(editor, cx);
            movement::register(editor, cx);
        });
        let mode = helix.read(cx).mode;
        Self::sync_editor_settings(mode, false, editor, window, cx);
        if mode != Mode::Insert {
            Self::ensure_selections_width(editor, window, cx);
        }
        if editor.is_focused(window) {
            // Let the mode indicator subscribe to the new state first.
            cx.defer_in(window, move |_, _, cx| {
                helix.update(cx, |_, cx| cx.emit(ModeIndicatorFocused));
            });
        }
    }

    fn deactivate(editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>) {
        editor.unregister_addon::<HelixAddon>();
        editor.clear_background_highlights(HighlightKey::HelixInsertSelections, cx);
        // When the `vim` crate takes over it configures the editor itself, and
        // its settings observer runs before ours.
        if VimModeSetting::is_enabled(cx) || HelixModeSetting::is_enabled(cx) {
            return;
        }
        editor.set_cursor_shape(
            EditorSettings::get_global(cx)
                .cursor_shape
                .unwrap_or_default(),
            cx,
        );
        editor.set_input_enabled(true);
        editor.set_expects_character_input(true);
        editor.set_cursor_offset_on_selection(false);
        editor.set_single_grapheme_selections_are_cursors(false, window, cx);
        editor.set_edit_predictions_hidden_for_vim_mode(false, window, cx);
    }

    /// Registers an action handler on the editor that runs on this state.
    fn action<A: Action>(
        editor: &mut Editor,
        cx: &mut Context<Self>,
        handler: impl Fn(&mut Self, &A, &mut Window, &mut Context<Self>) + 'static,
    ) {
        let subscription = editor.register_action(cx.listener(handler));
        cx.on_release(|_, _| drop(subscription)).detach();
    }

    fn update_editor<R>(
        &mut self,
        cx: &mut Context<Self>,
        update: impl FnOnce(&mut Self, &mut Editor, &mut Context<Editor>) -> R,
    ) -> Option<R> {
        let editor = self.editor.upgrade()?;
        Some(editor.update(cx, |editor, cx| update(self, editor, cx)))
    }

    fn push_digit(&mut self, digit: usize, cx: &mut Context<Self>) {
        let count = self
            .count
            .unwrap_or(0)
            .saturating_mul(10)
            .saturating_add(digit);
        if count <= MAX_COUNT {
            self.count = Some(count).filter(|count| *count > 0);
            cx.notify();
        }
    }

    /// Takes the count typed before the current command, defaulting to 1.
    fn take_count(&mut self, cx: &mut Context<Self>) -> usize {
        match self.count.take() {
            Some(count) => {
                cx.notify();
                count
            }
            None => 1,
        }
    }

    fn await_input(
        &mut self,
        pending_input: PendingInput,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pending_input = Some(pending_input);
        self.sync_editor_settings_for_state(window, cx);
        cx.notify();
    }

    /// Receives the character a pending command waits for. Keys without a
    /// binding reach the editor as ignored input outside of insert mode.
    fn input_ignored(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(pending_input) = self.pending_input.take() else {
            return;
        };
        match pending_input {
            PendingInput::Register => self.selected_register = text.chars().next(),
        }
        self.sync_editor_settings_for_state(window, cx);
        cx.notify();
    }

    /// Like Helix, the count and the selected register only apply to the
    /// command that follows them, whichever command that is.
    fn observe_keystrokes(
        &mut self,
        keystroke_event: &KeystrokeEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut keep_register = false;
        if let Some(action) = keystroke_event.action.as_ref() {
            if action.as_any().is::<PushDigit>() {
                return;
            }
            keep_register = action.as_any().is::<SelectRegister>();
        } else if self.pending_input.is_some()
            || window.has_pending_keystrokes()
            || keystroke_event.keystroke.is_ime_in_progress()
        {
            // A character for the pending command arrives as ignored input
            // after this observer runs.
            return;
        }

        let mut changed = self.count.take().is_some();
        if !keep_register {
            changed |= self.selected_register.take().is_some();
            if self.pending_input.take().is_some() {
                self.sync_editor_settings_for_state(window, cx);
                changed = true;
            }
        }
        if changed {
            cx.notify();
        }
    }

    fn switch_mode(&mut self, mode: Mode, window: &mut Window, cx: &mut Context<Self>) {
        if self.mode == mode {
            return;
        }
        self.mode = mode;
        self.sync_editor_settings_for_state(window, cx);
        cx.notify();
    }

    fn sync_editor_settings_for_state(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(cx, |helix, editor, cx| {
            Self::sync_editor_settings(
                helix.mode,
                helix.pending_input.is_some(),
                editor,
                window,
                cx,
            );
        });
    }

    fn sync_editor_settings(
        mode: Mode,
        awaiting_input: bool,
        editor: &mut Editor,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let settings = HelixSettings::get_global(cx);
        let inserting = mode == Mode::Insert;
        let hide_edit_predictions =
            !inserting && !(mode == Mode::Normal && settings.show_edit_predictions_in_normal_mode);
        editor.set_cursor_shape(settings.cursor_shape(mode, cx), cx);
        editor.set_clip_at_line_ends(false, cx);
        editor.set_collapse_matches(false);
        editor.set_input_enabled(inserting);
        editor.set_expects_character_input(inserting || awaiting_input);
        editor.set_cursor_offset_on_selection(!inserting);
        editor.set_single_grapheme_selections_are_cursors(mode == Mode::Normal, window, cx);
        editor.set_edit_predictions_hidden_for_vim_mode(hide_edit_predictions, window, cx);
    }

    fn handle_editor_event(
        &mut self,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::Focused => {
                cx.emit(ModeIndicatorFocused);
                self.sync_editor_settings_for_state(window, cx);
            }
            EditorEvent::FocusedIn | EditorEvent::CursorShapeChanged => {
                self.sync_editor_settings_for_state(window, cx);
            }
            EditorEvent::InputIgnored { text } => self.input_ignored(text, window, cx),
            EditorEvent::Blurred => {
                self.update_editor(cx, |helix, editor, cx| {
                    let settings = HelixSettings::get_global(cx);
                    if settings.cursor_shape(helix.mode, cx) == CursorShape::Block {
                        editor.set_cursor_shape(CursorShape::Hollow, cx);
                    }
                });
            }
            EditorEvent::SelectionsChanged { local: true } => {
                if self.mode == Mode::Insert {
                    self.highlight_insert_mode_ranges(cx);
                } else {
                    self.update_editor(cx, |_, editor, cx| {
                        Self::ensure_selections_width(editor, window, cx)
                    });
                }
            }
            EditorEvent::TransactionBegun { transaction_id } => {
                self.transaction_begun(*transaction_id);
            }
            _ => {}
        }
    }

    fn transaction_begun(&mut self, transaction_id: TransactionId) {
        if let Some(session) = self.insert_session.as_mut() {
            session.first_transaction.get_or_insert(transaction_id);
        }
    }

    /// Keeps every selection at least one grapheme wide outside of insert
    /// mode, since the editor itself (clicks, navigation) makes empty ones.
    fn ensure_selections_width(editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>) {
        if editor.leader_id().is_some() || editor.selections.pending_anchor().is_some() {
            return;
        }
        let display_map = editor.display_snapshot(cx);
        let snapshot = display_map.buffer_snapshot();
        let needs_widening = editor
            .selections
            .all::<MultiBufferOffset>(&display_map)
            .iter()
            .any(|selection| {
                let range = HelixRange::from_selection(selection);
                range.min_width_1(snapshot) != range
            });
        if !needs_widening {
            return;
        }
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
            selections.move_offsets_with(&mut |snapshot, selection| {
                HelixRange::from_selection(selection)
                    .min_width_1(snapshot)
                    .apply_to(selection);
            });
        });
    }
}

/// The `vim` settings that Helix mode follows too.
#[derive(Clone, RegisterSetting)]
pub(crate) struct HelixSettings {
    pub default_mode: Mode,
    pub show_edit_predictions_in_normal_mode: bool,
    pub use_system_clipboard: UseSystemClipboard,
    normal_cursor_shape: CursorShape,
    select_cursor_shape: CursorShape,
    insert_cursor_shape: Option<CursorShape>,
}

impl HelixSettings {
    pub fn cursor_shape(&self, mode: Mode, cx: &App) -> CursorShape {
        match mode {
            Mode::Normal => self.normal_cursor_shape,
            Mode::Select => self.select_cursor_shape,
            Mode::Insert => self.insert_cursor_shape.unwrap_or_else(|| {
                EditorSettings::get_global(cx)
                    .cursor_shape
                    .unwrap_or_default()
            }),
        }
    }
}

impl Settings for HelixSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let vim = content.vim.clone().unwrap_or_default();
        let cursor_shape = vim.cursor_shape.unwrap_or_default();
        Self {
            default_mode: match vim.default_mode.unwrap_or_default() {
                ModeContent::Normal => Mode::Normal,
                ModeContent::Insert => Mode::Insert,
            },
            show_edit_predictions_in_normal_mode: vim
                .show_edit_predictions_in_normal_mode
                .unwrap_or_default(),
            use_system_clipboard: vim
                .use_system_clipboard
                .unwrap_or(UseSystemClipboard::Always),
            normal_cursor_shape: cursor_shape.normal.unwrap_or_default().into(),
            select_cursor_shape: cursor_shape.visual.unwrap_or_default().into(),
            insert_cursor_shape: match cursor_shape.insert {
                None | Some(VimInsertModeCursorShape::Inherit) => None,
                Some(VimInsertModeCursorShape::Bar) => Some(CursorShape::Bar),
                Some(VimInsertModeCursorShape::Block) => Some(CursorShape::Block),
                Some(VimInsertModeCursorShape::Underline) => Some(CursorShape::Underline),
                Some(VimInsertModeCursorShape::Hollow) => Some(CursorShape::Hollow),
            },
        }
    }
}
