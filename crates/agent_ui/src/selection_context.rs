use std::ops::Range;

use editor::{Anchor, Editor, MultiBuffer};
use gpui::{App, Entity, Window};
use terminal_view::TerminalView;
use workspace::{Workspace, item::ItemHandle};
use zed_actions::agent::AddSelectionToThread;

use crate::text_thread_editor::AgentPanelDelegate;

/// Initializes the selection context module.
/// Note: The actual action registration happens in TextThreadEditor::init
/// to ensure it's registered at the same time as other workspace actions.
pub fn init(_cx: &mut App) {
    log::info!("[selection_context] init called");
}

/// Represents selected content that can be added to an agent thread.
pub enum SelectionContent {
    /// Selection from an editor with buffer anchors (for rich context like file paths, line numbers)
    Editor {
        selections: Vec<Range<Anchor>>,
        buffer: Entity<MultiBuffer>,
    },
    /// Terminal selection with shell name for proper formatting
    Terminal { text: String, shell_name: String },
}

/// Handles the `AddSelectionToThread` action by extracting selection from the active item
/// and delegating to the agent panel.
pub fn quote_selection(
    workspace: &mut Workspace,
    _: &AddSelectionToThread,
    window: &mut Window,
    cx: &mut gpui::Context<Workspace>,
) {
    log::info!("[selection_context] quote_selection called");

    let Some(delegate) = <dyn AgentPanelDelegate>::try_global(cx) else {
        log::warn!("[selection_context] No AgentPanelDelegate found, returning early");
        return;
    };
    log::info!("[selection_context] AgentPanelDelegate found");

    let Some(content) = get_selection_from_focused_item(workspace, window, cx) else {
        log::warn!("[selection_context] get_selection_from_focused_item returned None");
        return;
    };

    match content {
        SelectionContent::Editor { selections, buffer } => {
            log::info!(
                "[selection_context] Got editor selection with {} ranges",
                selections.len()
            );
            delegate.quote_editor_selection(workspace, selections, buffer, window, cx);
        }
        SelectionContent::Terminal { text, shell_name } => {
            log::info!(
                "[selection_context] Got terminal selection: {} chars, shell: {}",
                text.len(),
                shell_name
            );
            delegate.quote_terminal_selection(workspace, text, shell_name, window, cx);
        }
    }
}

/// Extracts selection content from the currently focused workspace item.
/// This correctly handles items in docks (like terminal in the bottom dock)
/// by using `focused_pane` instead of `active_pane`.
fn get_selection_from_focused_item(
    workspace: &mut Workspace,
    window: &Window,
    cx: &mut gpui::Context<Workspace>,
) -> Option<SelectionContent> {
    let focused_pane = workspace.focused_pane(window, cx);
    log::info!(
        "[selection_context] focused_pane entity_id: {:?}",
        focused_pane.entity_id()
    );

    let active_item = focused_pane.read(cx).active_item();
    if active_item.is_none() {
        log::warn!("[selection_context] No active_item in focused_pane");
        return None;
    }
    let active_item = active_item.unwrap();
    log::info!(
        "[selection_context] active_item tab_content_text: {:?}",
        active_item.tab_content_text(0, cx)
    );

    // Try editor first (most common case)
    if let Some(content) = get_editor_selection(&*active_item, cx) {
        log::info!("[selection_context] Got content from editor");
        return Some(content);
    }
    log::info!("[selection_context] No editor selection, trying terminal");

    // Try terminal
    if let Some(content) = get_terminal_selection(&*active_item, cx) {
        log::info!("[selection_context] Got content from terminal");
        return Some(content);
    }
    log::warn!("[selection_context] No terminal selection either");

    None
}

/// Extracts selection from an Editor item.
fn get_editor_selection(
    item: &dyn ItemHandle,
    cx: &mut gpui::Context<Workspace>,
) -> Option<SelectionContent> {
    let editor = item.act_as::<Editor>(cx)?;

    let buffer = editor.read(cx).buffer().clone();
    let snapshot = buffer.read(cx).snapshot(cx);

    let selections = editor.update(cx, |editor, cx| {
        editor
            .selections
            .all_adjusted(&editor.display_snapshot(cx))
            .into_iter()
            .filter_map(|selection| {
                (!selection.is_empty()).then(|| {
                    snapshot.anchor_after(selection.start)..snapshot.anchor_before(selection.end)
                })
            })
            .collect::<Vec<_>>()
    });

    if selections.is_empty() {
        return None;
    }

    Some(SelectionContent::Editor { selections, buffer })
}

/// Extracts selection from a TerminalView item.
fn get_terminal_selection(item: &dyn ItemHandle, cx: &App) -> Option<SelectionContent> {
    log::info!("[selection_context] get_terminal_selection: attempting downcast to TerminalView");

    let terminal_view = item.downcast::<TerminalView>()?;
    log::info!("[selection_context] downcast to TerminalView succeeded");

    let terminal = terminal_view.read(cx).terminal();
    let terminal_read = terminal.read(cx);
    let shell_name = terminal_read.shell_name();
    let selection_text = terminal_read.last_content.selection_text.clone()?;

    log::info!(
        "[selection_context] terminal selection_text: {:?}, shell: {}",
        selection_text,
        shell_name
    );

    if selection_text.is_empty() {
        log::warn!("[selection_context] selection_text is empty");
        return None;
    }

    log::info!(
        "[selection_context] returning Terminal selection with {} chars, shell: {}",
        selection_text.len(),
        shell_name
    );
    Some(SelectionContent::Terminal {
        text: selection_text,
        shell_name,
    })
}
