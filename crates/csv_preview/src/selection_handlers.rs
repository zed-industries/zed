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
    selection::{NavigationDirection as ND, NavigationOperation as NO, TableSelection},
};

impl CsvPreviewView {
    pub(crate) fn clear_selection(
        &mut self,
        _: &ClearSelection,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection = TableSelection::new();
        cx.notify();
    }

    pub(crate) fn select_all(&mut self, _: &SelectAll, _w: &mut Window, cx: &mut Context<Self>) {
        let start_time = Instant::now();
        let max_rows = self.contents.rows.len();
        let max_cols = self.contents.headers.len();
        self.selection
            .select_all(&self.ordered_indices, max_rows, max_cols);

        let selection_duration = start_time.elapsed();
        self.performance_metrics.last_selection_took = Some(selection_duration);
        cx.notify();
    }

    // Single cell selection actions
    pub(crate) fn select_up(&mut self, _: &SelectUp, _w: &mut Window, cx: &mut Context<Self>) {
        self.handle_navigation(ND::Up, NO::MoveFocus, cx);
    }

    pub(crate) fn select_down(&mut self, _: &SelectDown, _w: &mut Window, cx: &mut Context<Self>) {
        self.handle_navigation(ND::Down, NO::MoveFocus, cx);
    }

    pub(crate) fn select_left(&mut self, _: &SelectLeft, _w: &mut Window, cx: &mut Context<Self>) {
        self.handle_navigation(ND::Left, NO::MoveFocus, cx);
    }

    pub(crate) fn select_right(
        &mut self,
        _: &SelectRight,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::MoveFocus, cx);
    }

    // Selection extension actions
    pub(crate) fn extend_selection_up(
        &mut self,
        _: &ExtendSelectionUp,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Up, NO::ExtendSelection, cx);
    }

    pub(crate) fn extend_selection_down(
        &mut self,
        _: &ExtendSelectionDown,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Down, NO::ExtendSelection, cx);
    }

    pub(crate) fn extend_selection_left(
        &mut self,
        _: &ExtendSelectionLeft,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Left, NO::ExtendSelection, cx);
    }

    pub(crate) fn extend_selection_right(
        &mut self,
        _: &ExtendSelectionRight,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::ExtendSelection, cx);
    }

    // Select at edge actions
    pub(crate) fn select_at_top_edge(
        &mut self,
        _: &SelectAtTopEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Up, NO::JumpToEdge, cx);
    }

    pub(crate) fn select_at_bottom_edge(
        &mut self,
        _: &SelectAtBottomEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Down, NO::JumpToEdge, cx);
    }

    pub(crate) fn select_at_left_edge(
        &mut self,
        _: &SelectAtLeftEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Left, NO::JumpToEdge, cx);
    }

    pub(crate) fn select_at_right_edge(
        &mut self,
        _: &SelectAtRightEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::JumpToEdge, cx);
    }

    // Extend selection to edge actions
    pub(crate) fn extend_selection_to_top_edge(
        &mut self,
        _: &ExtendSelectionToTopEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Up, NO::ExtendToEdge, cx);
    }

    pub(crate) fn extend_selection_to_bottom_edge(
        &mut self,
        _: &ExtendSelectionToBottomEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Down, NO::ExtendToEdge, cx);
    }

    pub(crate) fn extend_selection_to_left_edge(
        &mut self,
        _: &ExtendSelectionToLeftEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Left, NO::ExtendToEdge, cx);
    }

    pub(crate) fn extend_selection_to_right_edge(
        &mut self,
        _: &ExtendSelectionToRightEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::ExtendToEdge, cx);
    }
}
