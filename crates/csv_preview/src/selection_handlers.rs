//! Action handlers for CSV preview navigation
//!
//! Contains streamlined action handlers that delegate to the unified navigation system.

use ui::{Context, Window};

use crate::{
    ClearSelection, CsvPreviewView, JumpToBottomEdge, JumpToLeftEdge, JumpToRightEdge,
    JumpToTopEdge, MoveFocusDown, MoveFocusLeft, MoveFocusRight, MoveFocusUp, SelectAll,
    SelectDown, SelectLeft, SelectRight, SelectUp, SelectionToBottomEdge, SelectionToLeftEdge,
    SelectionToRightEdge, SelectionToTopEdge,
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

    // Movement actions
    pub(crate) fn move_focus_up(
        &mut self,
        _: &MoveFocusUp,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Up, NO::MoveFocus, cx);
    }

    pub(crate) fn move_focus_down(
        &mut self,
        _: &MoveFocusDown,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Down, NO::MoveFocus, cx);
    }

    pub(crate) fn move_focus_left(
        &mut self,
        _: &MoveFocusLeft,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Left, NO::MoveFocus, cx);
    }

    pub(crate) fn move_focus_right(
        &mut self,
        _: &MoveFocusRight,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::MoveFocus, cx);
    }

    // Selection extension actions
    pub(crate) fn select_up(&mut self, _: &SelectUp, _w: &mut Window, cx: &mut Context<Self>) {
        self.handle_navigation(ND::Up, NO::ExtendSelection, cx);
    }

    pub(crate) fn select_down(&mut self, _: &SelectDown, _w: &mut Window, cx: &mut Context<Self>) {
        self.handle_navigation(ND::Down, NO::ExtendSelection, cx);
    }

    pub(crate) fn select_left(&mut self, _: &SelectLeft, _w: &mut Window, cx: &mut Context<Self>) {
        self.handle_navigation(ND::Left, NO::ExtendSelection, cx);
    }

    pub(crate) fn select_right(
        &mut self,
        _: &SelectRight,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::ExtendSelection, cx);
    }

    pub(crate) fn select_all(&mut self, _: &SelectAll, _w: &mut Window, cx: &mut Context<Self>) {
        let max_rows = self.contents.rows.len();
        let max_cols = self.contents.headers.len();
        self.selection
            .select_all(&self.ordered_indices, max_rows, max_cols);
        cx.notify();
    }

    // Jump to edge actions
    pub(crate) fn jump_to_top_edge(
        &mut self,
        _: &JumpToTopEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Up, NO::JumpToEdge, cx);
    }

    pub(crate) fn jump_to_bottom_edge(
        &mut self,
        _: &JumpToBottomEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Down, NO::JumpToEdge, cx);
    }

    pub(crate) fn jump_to_left_edge(
        &mut self,
        _: &JumpToLeftEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Left, NO::JumpToEdge, cx);
    }

    pub(crate) fn jump_to_right_edge(
        &mut self,
        _: &JumpToRightEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::JumpToEdge, cx);
    }

    // Extend selection to edge actions
    pub(crate) fn extend_selection_to_top_edge(
        &mut self,
        _: &SelectionToTopEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Up, NO::ExtendToEdge, cx);
    }

    pub(crate) fn extend_selection_to_bottom_edge(
        &mut self,
        _: &SelectionToBottomEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Down, NO::ExtendToEdge, cx);
    }

    pub(crate) fn extend_selection_to_left_edge(
        &mut self,
        _: &SelectionToLeftEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Left, NO::ExtendToEdge, cx);
    }

    pub(crate) fn extend_selection_to_right_edge(
        &mut self,
        _: &SelectionToRightEdge,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_navigation(ND::Right, NO::ExtendToEdge, cx);
    }
}
