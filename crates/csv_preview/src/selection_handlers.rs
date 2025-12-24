//! Action handlers for CSV preview navigation
//!
//! Contains streamlined action handlers that delegate to the unified navigation system.

use std::time::Instant;
use ui::{Context, Window};

use crate::{
    ClearSelection, CsvPreviewView, ExtendSelectionDown, ExtendSelectionLeft, ExtendSelectionRight,
    ExtendSelectionToBottomEdge, ExtendSelectionToLeftEdge, ExtendSelectionToRightEdge,
    ExtendSelectionToTopEdge, ExtendSelectionUp, SelectAll, SelectAtBottomEdge, SelectAtLeftEdge,
    SelectAtRightEdge, SelectAtTopEdge, SelectDown, SelectLeft, SelectRight, SelectUp,
    selection::{
        NavigationDirection as ND, NavigationOperation as NO, ScrollOffset, TableSelection,
    },
};

impl CsvPreviewView {
    pub(crate) fn clear_selection(
        &mut self,
        _: &ClearSelection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection = TableSelection::new();
        self.on_selection_changed(window, cx, None);
        cx.notify();
    }

    pub(crate) fn select_all(
        &mut self,
        _: &SelectAll,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let start_time = Instant::now();
        let max_rows = self.contents.rows.len();
        let max_cols = self.contents.number_of_cols;
        self.selection.select_all(max_rows, max_cols);

        self.performance_metrics.last_selection_took = Some(start_time.elapsed());
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
