use gpui::{
    Action, Context, EventEmitter, IntoElement, ParentElement, Render, Subscription, WeakEntity,
    Window,
};
use ui::{
    prelude::*, Button, ButtonCommon, ButtonStyle, IconName, IconSize, Label, LabelSize, Tooltip,
};
use workspace::{item::ItemHandle, StatusItemView};

use crate::{HexEditorView, OpenHexEditor};

// ============================================================================
// HexEditorButton - Button to open hex editor
// ============================================================================

pub struct HexEditorButton {
    active_item_is_hex_viewable: bool,
}

impl HexEditorButton {
    pub fn new() -> Self {
        Self {
            active_item_is_hex_viewable: false,
        }
    }
}

impl Default for HexEditorButton {
    fn default() -> Self {
        Self::new()
    }
}

impl EventEmitter<()> for HexEditorButton {}

impl Render for HexEditorButton {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_enabled = self.active_item_is_hex_viewable;

        div().when(is_enabled, |el| {
            el.child(
                Button::new("hex-editor-button", "")
                    .icon(IconName::Binary)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .tooltip(Tooltip::text("Open in Hex Editor"))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(OpenHexEditor.boxed_clone(), cx);
                    }),
            )
        })
    }
}

impl StatusItemView for HexEditorButton {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Check if the active item has a project path (i.e., it's a file we can open in hex)
        // But hide the button if we're already viewing a hex editor
        let is_hex_editor = active_pane_item
            .map(|item| item.downcast::<HexEditorView>().is_some())
            .unwrap_or(false);

        self.active_item_is_hex_viewable = !is_hex_editor
            && active_pane_item
                .map(|item| item.project_path(cx).is_some())
                .unwrap_or(false);

        cx.notify();
    }
}

// ============================================================================
// HexEditorStatus - Status bar item showing hex editor info
// ============================================================================

pub struct HexEditorStatus {
    hex_editor: Option<WeakEntity<HexEditorView>>,
    _observe_active_editor: Option<Subscription>,
}

impl HexEditorStatus {
    pub fn new() -> Self {
        Self {
            hex_editor: None,
            _observe_active_editor: None,
        }
    }
}

impl Default for HexEditorStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventEmitter<()> for HexEditorStatus {}

impl Render for HexEditorStatus {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(hex_editor) = self.hex_editor.as_ref().and_then(|e| e.upgrade()) else {
            return div().into_any_element();
        };

        let hex_editor = hex_editor.read(cx);
        let cursor_offset = hex_editor.cursor_offset();
        let file_size = hex_editor.file_size();
        let selection = hex_editor.selection();
        let edit_mode = hex_editor.edit_mode();

        let selection_info = selection
            .as_ref()
            .map(|s| format!(" ({} selected)", s.len()))
            .unwrap_or_default();

        let mode_str = match edit_mode {
            crate::EditMode::Hex => "HEX",
            crate::EditMode::Ascii => "ASCII",
        };

        div()
            .flex()
            .flex_row()
            .gap_3()
            .child(
                Label::new(format!(
                    "0x{:08X} / {} bytes{}",
                    cursor_offset, file_size, selection_info
                ))
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .child(
                Label::new(mode_str)
                    .size(LabelSize::Small)
                    .color(Color::Default),
            )
            .into_any_element()
    }
}

impl StatusItemView for HexEditorStatus {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(hex_editor) = active_pane_item.and_then(|item| item.downcast::<HexEditorView>())
        {
            self._observe_active_editor =
                Some(cx.observe_in(&hex_editor, window, |this, _, _, cx| {
                    cx.notify();
                    this.hex_editor.as_ref().map(|_| ());
                }));
            self.hex_editor = Some(hex_editor.downgrade());
        } else {
            self.hex_editor = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}
