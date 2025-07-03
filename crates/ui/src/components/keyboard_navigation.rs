use gpui::{Focusable, actions};

actions!(
    keyboard_navigation,
    [NextRow, PreviousRow, NextColumn, PreviousColumn]
);

/// Implement this trait to enable grid-like keyboard navigation for a layout.
///
/// This trait allows you to navigate through a layout of rows with mixed column
/// lengths. In example, a layout might have rows with 5, 1 and 3 columns.
///
/// Moving up or down between rows will focus the first element in the next or previous row.
/// Moving left or right between columns will focus the next or previous element in the same row.
///
/// Wrapping can be enabled via `vertical_wrapping` and `horizontal_wrapping` respectively.
pub trait KeyboardNavigation: Focusable {
    fn has_focus(&self) -> bool;
    /// The focused row. Always has a value to allow for "focused inactive" states.
    fn focused_row(&self) -> usize;
    /// The focused column. Always has a value to allow for "focused inactive" states.
    fn focused_column(&self) -> usize;
    /// Focus the first focusable element in the layout.
    fn focus_first(&self);
    /// Focus the next row, wrapping back to the first row if necessary.
    ///
    /// Is a no-op if wrapping is not enabled.
    fn focus_next_row(&self);
    /// Focus the previous row, wrapping back to the last row if necessary.
    ///
    /// Is a no-op if wrapping is not enabled.
    fn focus_previous_row(&self);
    /// Focus the next column, wrapping back to the first column if necessary.
    ///
    /// Is a no-op if wrapping is not enabled.
    fn focus_next_column(&self);
    /// Focus the previous column, wrapping back to the last column if necessary.
    ///
    /// Is a no-op if wrapping is not enabled.
    fn focus_previous_column(&self);
    /// Focus the row at the given index.
    fn focus_row_index(&self, index: usize);
    /// Focus the column at the given index.
    fn focus_column_index(&self, ix: usize);
    /// When reaching the last row, should moving down wrap
    /// back to the first row, and vice versa?
    fn vertical_wrap(&self) -> bool {
        false
    }
    /// When reaching the last column, should moving right wrap
    /// back to the first column, and vice versa?
    fn horizontal_wrap(&self) -> bool {
        false
    }
}

pub struct NavigationRow {}

pub struct NavigationColumn {}
