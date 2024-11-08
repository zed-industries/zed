use crate::{prelude::*, Indicator};
use gpui::{div, AnyElement, FontWeight, IntoElement};

/// A table component
#[derive(IntoElement)]
pub struct Table {
    column_headers: Vec<SharedString>,
    rows: Vec<Vec<TableCell>>,
    column_count: usize,
    striped: bool,
}

impl Table {
    /// Create a new table with a column count equal to the
    /// number of headers provided.
    pub fn new(headers: Vec<impl Into<SharedString>>) -> Self {
        let column_count = headers.len();

        Table {
            column_headers: headers.into_iter().map(Into::into).collect(),
            column_count,
            rows: Vec::new(),
            striped: false,
        }
    }

    /// Adds a row to the table.
    ///
    /// The row must have the same number of columns as the table.
    pub fn row(mut self, items: Vec<impl Into<TableCell>>) -> Self {
        if items.len() == self.column_count {
            self.rows.push(items.into_iter().map(Into::into).collect());
        } else {
            // TODO: Log error: Row length mismatch
        }
        self
    }

    /// Adds multiple rows to the table.
    ///
    /// Each row must have the same number of columns as the table.
    /// Rows that don't match the column count are ignored.
    pub fn rows(mut self, rows: Vec<Vec<impl Into<TableCell>>>) -> Self {
        for row in rows {
            self = self.row(row);
        }
        self
    }

    fn base_cell_style(cx: &WindowContext) -> Div {
        div()
            .px_1p5()
            .flex_1()
            .justify_start()
            .text_ui(cx)
            .whitespace_nowrap()
            .text_ellipsis()
            .overflow_hidden()
    }

    /// Enables row striping.
    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }
}

impl RenderOnce for Table {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .p_2()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .children(self.column_headers.into_iter().map(|h| {
                Self::base_cell_style(cx)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(h)
            }));

        let row_count = self.rows.len();
        let rows = self.rows.into_iter().enumerate().map(|(ix, row)| {
            let is_last = ix == row_count - 1;
            let bg = if ix % 2 == 1 && self.striped {
                Some(cx.theme().colors().text.opacity(0.05))
            } else {
                None
            };
            div()
                .w_full()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_1p5()
                .py_1()
                .when_some(bg, |row, bg| row.bg(bg))
                .when(!is_last, |row| {
                    row.border_b_1().border_color(cx.theme().colors().border)
                })
                .children(row.into_iter().map(|cell| match cell {
                    TableCell::String(s) => Self::base_cell_style(cx).child(s),
                    TableCell::Element(e) => Self::base_cell_style(cx).child(e),
                }))
        });

        div()
            .w_full()
            .overflow_hidden()
            .child(header)
            .children(rows)
    }
}

/// Represents a cell in a table.
pub enum TableCell {
    /// A cell containing a string value.
    String(SharedString),
    /// A cell containing a UI element.
    Element(AnyElement),
}

/// Creates a `TableCell` containing a string value.
pub fn string_cell(s: impl Into<SharedString>) -> TableCell {
    TableCell::String(s.into())
}

/// Creates a `TableCell` containing an element.
pub fn element_cell(e: impl Into<AnyElement>) -> TableCell {
    TableCell::Element(e.into())
}

impl<E> From<E> for TableCell
where
    E: Into<SharedString>,
{
    fn from(e: E) -> Self {
        TableCell::String(e.into())
    }
}

impl ComponentPreview for Table {
    fn description() -> impl Into<Option<&'static str>> {
        "Used for showing tabular data. Tables may show both text and elements in their cells."
    }

    fn examples() -> Vec<ComponentExampleGroup<Self>> {
        vec![
            example_group(
                "Basic Table",
                vec![single_example(
                    "Simple Table",
                    Table::new(vec!["Name", "Age", "City"])
                        .row(vec!["Alice", "28", "New York"])
                        .row(vec!["Bob", "32", "San Francisco"])
                        .row(vec!["Charlie", "25", "London"]),
                )],
            ),
            example_group(
                "Striped Table",
                vec![single_example(
                    "Striped Table",
                    Table::new(vec!["Product", "Price", "Stock"])
                        .striped()
                        .row(vec!["Laptop", "$999", "In Stock"])
                        .row(vec!["Phone", "$599", "Low Stock"])
                        .row(vec!["Tablet", "$399", "Out of Stock"])
                        .row(vec!["Headphones", "$199", "In Stock"]),
                )],
            ),
            example_group(
                "Mixed Content Table",
                vec![single_example(
                    "Table with Elements",
                    Table::new(vec!["Status", "Name", "Action"])
                        .row(vec![
                            element_cell(Indicator::dot().color(Color::Success).into_any_element()),
                            string_cell("Project A"),
                            element_cell(Button::new("view_a", "View").into_any_element()),
                        ])
                        .row(vec![
                            element_cell(Indicator::dot().color(Color::Warning).into_any_element()),
                            string_cell("Project B"),
                            element_cell(Button::new("view_b", "View").into_any_element()),
                        ])
                        .row(vec![
                            element_cell(Indicator::dot().color(Color::Error).into_any_element()),
                            string_cell("Project C"),
                            element_cell(Button::new("view_c", "View").into_any_element()),
                        ]),
                )],
            ),
        ]
    }
}
