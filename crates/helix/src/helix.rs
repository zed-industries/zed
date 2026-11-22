//! Helix-style modal editing for the editor.
//!
//! Active when `helix_mode` is enabled together with the `helix-next` feature
//! flag; otherwise Helix mode is provided by the `vim` crate.

mod mode;
mod movement;
mod selection;
#[cfg(test)]
mod test;

use editor::{Editor, EditorEvent, EditorSettings, MultiBufferOffset, SelectionEffects};
use gpui::{
    Action, App, AppContext as _, Context, Entity, KeyContext, Subscription, WeakEntity, Window,
};
use language::{CursorShape, TransactionId};
use settings::{ModeContent, RegisterSetting, Settings, VimInsertModeCursorShape};
use vim_mode_setting::{HelixModeSetting, VimModeSetting, observe_modal_editing_changes_in};

use crate::{mode::InsertSession, selection::HelixRange};

/// Initializes the `helix` crate.
pub fn init(cx: &mut App) {
    cx.observe_new(Helix::register).detach();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    Normal,
    Select,
    Insert,
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

/// Attaches the Helix state to an editor.
pub(crate) struct HelixAddon {
    pub(crate) entity: Entity<Helix>,
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
    insert_session: Option<InsertSession>,
    editor: WeakEntity<Editor>,
    _subscriptions: Vec<Subscription>,
}

impl Helix {
    fn new(window: &mut Window, cx: &mut Context<Editor>) -> Entity<Self> {
        let editor = cx.entity();
        cx.new(|cx| Helix {
            mode: HelixSettings::get_global(cx).default_mode,
            insert_session: None,
            editor: editor.downgrade(),
            _subscriptions: vec![cx.subscribe_in(
                &editor,
                window,
                |helix: &mut Helix, _, event, window, cx| {
                    helix.handle_editor_event(event, window, cx)
                },
            )],
        })
    }

    fn extend_key_context(&self, key_context: &mut KeyContext) {
        if self.mode != Mode::Insert {
            key_context.add("Helix");
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
        });
        helix.update(cx, |_, cx| {
            mode::register(editor, cx);
            movement::register(editor, cx);
        });
        let mode = helix.read(cx).mode;
        Self::sync_editor_settings(mode, editor, window, cx);
        if mode != Mode::Insert {
            Self::ensure_selections_width(editor, window, cx);
        }
    }

    fn deactivate(editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>) {
        editor.unregister_addon::<HelixAddon>();
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

    fn switch_mode(&mut self, mode: Mode, window: &mut Window, cx: &mut Context<Self>) {
        if self.mode == mode {
            return;
        }
        self.mode = mode;
        self.update_editor(cx, |helix, editor, cx| {
            Self::sync_editor_settings(helix.mode, editor, window, cx);
        });
        cx.notify();
    }

    fn sync_editor_settings(
        mode: Mode,
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
        editor.set_expects_character_input(inserting);
        editor.set_cursor_offset_on_selection(!inserting);
        editor.set_edit_predictions_hidden_for_vim_mode(hide_edit_predictions, window, cx);
    }

    fn handle_editor_event(
        &mut self,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::Focused | EditorEvent::FocusedIn | EditorEvent::CursorShapeChanged => {
                self.update_editor(cx, |helix, editor, cx| {
                    Self::sync_editor_settings(helix.mode, editor, window, cx)
                });
            }
            EditorEvent::Blurred => {
                self.update_editor(cx, |helix, editor, cx| {
                    let settings = HelixSettings::get_global(cx);
                    if settings.cursor_shape(helix.mode, cx) == CursorShape::Block {
                        editor.set_cursor_shape(CursorShape::Hollow, cx);
                    }
                });
            }
            EditorEvent::SelectionsChanged { local: true } if self.mode != Mode::Insert => {
                self.update_editor(cx, |_, editor, cx| {
                    Self::ensure_selections_width(editor, window, cx)
                });
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
