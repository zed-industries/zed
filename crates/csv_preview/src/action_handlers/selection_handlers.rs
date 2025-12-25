//! Action handlers for CSV preview navigation
//!
//! Contains streamlined action handlers that delegate to the unified navigation system.

use gpui::ScrollStrategy;
use ui::{Context, Window};

use crate::{
    ClearSelection, CsvPreviewView, ExtendSelectionDown, ExtendSelectionLeft, ExtendSelectionRight,
    ExtendSelectionToBottomEdge, ExtendSelectionToLeftEdge, ExtendSelectionToRightEdge,
    ExtendSelectionToTopEdge, ExtendSelectionUp, SelectAll, SelectAtBottomEdge, SelectAtLeftEdge,
    SelectAtRightEdge, SelectAtTopEdge, SelectDown, SelectLeft, SelectRight, SelectUp,
    settings::RowRenderMechanism,
    table_data_engine::selection::{
        NavigationDirection as ND, NavigationOperation as NO, ScrollOffset, TableSelection,
    },
};

///// Selection related CsvPreviewView methods /////
impl CsvPreviewView {
    /// Unified navigation handler - eliminates code duplication
    pub(crate) fn handle_navigation(
        &mut self,
        direction: ND,
        operation: NO,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.performance_metrics.record("selection", || {
            self.engine.change_selection(direction, operation);
        });

        let scroll = match direction {
            ND::Up => Some(ScrollOffset::Negative),
            ND::Down => Some(ScrollOffset::Positive),
            ND::Left => None,
            ND::Right => None,
        };

        // Update cell editor to show focused cell content
        self.on_selection_changed(window, cx, scroll);
        cx.notify();
    }

    /// Performs actions triggered by selection change
    pub(crate) fn on_selection_changed(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
        apply_scroll: Option<ScrollOffset>,
    ) {
        self.clear_cell_editor();
        self.scroll_to_focused_cell(cx, apply_scroll);
    }

    fn scroll_to_focused_cell(
        &mut self,
        cx: &mut Context<'_, CsvPreviewView>,
        apply_scroll: Option<ScrollOffset>,
    ) {
        if let Some(focused_cell) = self.engine.selection.get_focused_cell()
            && let Some(scroll) = apply_scroll
        {
            let display_row_index = focused_cell.row;
            let ix = display_row_index.0;

            match self.settings.rendering_with {
                RowRenderMechanism::VariableList => {
                    // Variable height list uses ListState::scroll_to_reveal_item
                    let ix_with_offset = match scroll {
                        ScrollOffset::NoOffset => ix,
                        ScrollOffset::Negative => ix.saturating_sub(2), // Avoid overflowing
                        ScrollOffset::Positive => ix + 2,
                    };
                    self.list_state.scroll_to_reveal_item(ix_with_offset);
                }
                RowRenderMechanism::UniformList => {
                    // Uniform list uses UniformListScrollHandle
                    let table_interaction_state = &self.table_interaction_state;
                    table_interaction_state.update(cx, |state, _| {
                        let ix_with_offset = match scroll {
                            ScrollOffset::NoOffset => ix,
                            ScrollOffset::Negative => ix.saturating_sub(2),
                            ScrollOffset::Positive => ix + 2,
                        };
                        // Use ScrollStrategy::Nearest for minimal scrolling (like scroll_to_reveal_item)
                        state.scroll_handle.scroll_to_item_with_offset(
                            ix_with_offset,
                            ScrollStrategy::Nearest,
                            0, // No additional offset since we already calculate it above
                        );
                    });
                }
            }
        }
    }
}

impl CsvPreviewView {
    pub(crate) fn clear_selection(
        &mut self,
        _: &ClearSelection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.engine.selection = TableSelection::new();
        self.on_selection_changed(window, cx, None);
        cx.notify();
    }

    pub(crate) fn select_all(
        &mut self,
        _: &SelectAll,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let max_rows = self.engine.contents.rows.len();
        let max_cols = self.engine.contents.number_of_cols;

        self.performance_metrics.record("select_all", || {
            self.engine.selection.select_all(max_rows, max_cols);
        });
        self.on_selection_changed(window, cx, Some(ScrollOffset::NoOffset));
        cx.notify();
    }

    // Single cell selection actions
    pub(crate) fn select_up(&mut self, _: &SelectUp, window: &mut Window, cx: &mut Context<Self>) {
        self.handle_navigation(ND::Up, NO::MoveFocus, window, cx);
    }

    pub(crate) fn select_down(
        &mut self,
        _: &SelectDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Down, NO::MoveFocus, window, cx);
    }

    pub(crate) fn select_left(
        &mut self,
        _: &SelectLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Left, NO::MoveFocus, window, cx);
    }

    pub(crate) fn select_right(
        &mut self,
        _: &SelectRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::MoveFocus, window, cx);
    }

    // Selection extension actions
    pub(crate) fn extend_selection_up(
        &mut self,
        _: &ExtendSelectionUp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Up, NO::ExtendSelection, window, cx);
    }

    pub(crate) fn extend_selection_down(
        &mut self,
        _: &ExtendSelectionDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Down, NO::ExtendSelection, window, cx);
    }

    pub(crate) fn extend_selection_left(
        &mut self,
        _: &ExtendSelectionLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Left, NO::ExtendSelection, window, cx);
    }

    pub(crate) fn extend_selection_right(
        &mut self,
        _: &ExtendSelectionRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::ExtendSelection, window, cx);
    }

    // Select at edge actions
    pub(crate) fn select_at_top_edge(
        &mut self,
        _: &SelectAtTopEdge,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Up, NO::JumpToEdge, window, cx);
    }

    pub(crate) fn select_at_bottom_edge(
        &mut self,
        _: &SelectAtBottomEdge,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Down, NO::JumpToEdge, window, cx);
    }

    pub(crate) fn select_at_left_edge(
        &mut self,
        _: &SelectAtLeftEdge,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Left, NO::JumpToEdge, window, cx);
    }

    pub(crate) fn select_at_right_edge(
        &mut self,
        _: &SelectAtRightEdge,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::JumpToEdge, window, cx);
    }

    // Extend selection to edge actions
    pub(crate) fn extend_selection_to_top_edge(
        &mut self,
        _: &ExtendSelectionToTopEdge,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Up, NO::ExtendToEdge, window, cx);
    }

    pub(crate) fn extend_selection_to_bottom_edge(
        &mut self,
        _: &ExtendSelectionToBottomEdge,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Down, NO::ExtendToEdge, window, cx);
    }

    pub(crate) fn extend_selection_to_left_edge(
        &mut self,
        _: &ExtendSelectionToLeftEdge,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Left, NO::ExtendToEdge, window, cx);
    }

    pub(crate) fn extend_selection_to_right_edge(
        &mut self,
        _: &ExtendSelectionToRightEdge,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::ExtendToEdge, window, cx);
    }
}
