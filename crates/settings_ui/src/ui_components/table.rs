use std::ops::Range;

use db::smol::stream::iter;
use gpui::{Entity, FontWeight, Length, uniform_list};
use ui::{
    ActiveTheme as _, AnyElement, App, Button, ButtonCommon as _, ButtonStyle, Color, Component,
    ComponentScope, Div, ElementId, FixedWidth as _, FluentBuilder as _, Indicator, IntoElement,
    ParentElement, RegisterComponent, RenderOnce, Styled, StyledTypography, Window, div,
    example_group_with_title, px, single_example, v_flex,
};

struct UniformListData {
    render_item_fn: Box<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<AnyElement>>,
    element_id: ElementId,
    row_count: usize,
}

enum TableContents<const COLS: usize> {
    Vec(Vec<[AnyElement; COLS]>),
    UniformList(UniformListData),
}

impl<const COLS: usize> TableContents<COLS> {
    fn rows_mut(&mut self) -> Option<&mut Vec<[AnyElement; COLS]>> {
        match self {
            TableContents::Vec(rows) => Some(rows),
            TableContents::UniformList(_) => None,
        }
    }

    fn len(&self) -> usize {
        match self {
            TableContents::Vec(rows) => rows.len(),
            TableContents::UniformList(data) => data.row_count,
        }
    }
}

/// A table component
#[derive(RegisterComponent, IntoElement)]
pub struct Table<const COLS: usize = 3> {
    striped: bool,
    width: Length,
    headers: Option<[AnyElement; COLS]>,
    rows: TableContents<COLS>,
}

impl<const COLS: usize> Table<COLS> {
    pub fn uniform_list(
        id: impl Into<ElementId>,
        row_count: usize,
        render_item_fn: impl Fn(Range<usize>, &mut Window, &mut App) -> Vec<AnyElement> + 'static,
    ) -> Self {
        Table {
            striped: false,
            width: Length::Auto,
            headers: None,
            rows: TableContents::UniformList(UniformListData {
                element_id: id.into(),
                row_count: row_count,
                render_item_fn: Box::new(render_item_fn),
            }),
        }
    }

    /// Create a new table with a column count equal to the
    /// number of headers provided.
    pub fn new() -> Self {
        Table {
            striped: false,
            width: Length::Auto,
            headers: None,
            rows: TableContents::Vec(Vec::new()),
        }
    }

    /// Enables row striping.
    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }

    /// Sets the width of the table.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn header(mut self, headers: [impl IntoElement; COLS]) -> Self {
        self.headers = Some(headers.map(IntoElement::into_any_element));
        self
    }

    pub fn row(mut self, items: [impl IntoElement; COLS]) -> Self {
        if let Some(rows) = self.rows.rows_mut() {
            rows.push(items.map(IntoElement::into_any_element));
        }
        self
    }

    pub fn render_row(&self, items: [impl IntoElement; COLS], cx: &mut App) -> AnyElement {
        return render_row(0, items, self.rows.len(), self.striped, cx);
    }

    pub fn render_header(
        &self,
        headers: [impl IntoElement; COLS],
        cx: &mut App,
    ) -> impl IntoElement {
        render_header(headers, cx)
    }
}

fn base_cell_style(cx: &App) -> Div {
    div()
        .px_1p5()
        .flex_1()
        .justify_start()
        .text_ui(cx)
        .whitespace_nowrap()
        .text_ellipsis()
        .overflow_hidden()
}

pub fn render_row<const COLS: usize>(
    row_index: usize,
    items: [impl IntoElement; COLS],
    row_count: usize,
    striped: bool,
    cx: &App,
) -> AnyElement {
    let is_last = row_index == row_count - 1;
    let bg = if row_index % 2 == 1 && striped {
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
        .children(
            items
                .map(IntoElement::into_any_element)
                .map(|cell| base_cell_style(cx).child(cell)),
        )
        .into_any_element()
}

pub fn render_header<const COLS: usize>(
    headers: [impl IntoElement; COLS],
    cx: &mut App,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .w_full()
        .p_2()
        .border_b_1()
        .border_color(cx.theme().colors().border)
        .children(headers.into_iter().map(|h| {
            base_cell_style(cx)
                .font_weight(FontWeight::SEMIBOLD)
                .child(h)
        }))
}

impl<const COLS: usize> RenderOnce for Table<COLS> {
    fn render(mut self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        // match self.ro
        let row_count = self.rows.len();
        div()
            .w(self.width)
            .overflow_hidden()
            .when_some(self.headers.take(), |this, headers| {
                this.child(render_header(headers, cx))
            })
            .map(|div| match self.rows {
                TableContents::Vec(items) => div.children(
                    items
                        .into_iter()
                        .enumerate()
                        .map(|(index, row)| render_row(index, row, row_count, self.striped, cx)),
                ),
                TableContents::UniformList(uniform_list_data) => div.child(uniform_list(
                    uniform_list_data.element_id,
                    uniform_list_data.row_count,
                    uniform_list_data.render_item_fn,
                )),
            })
    }
}

impl Component for Table<3> {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn description() -> Option<&'static str> {
        Some("A table component for displaying data in rows and columns with optional styling.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic Tables",
                        vec![
                            single_example(
                                "Simple Table",
                                Table::new()
                                    .width(px(400.))
                                    .header(["Name", "Age", "City"])
                                    .row(["Alice", "28", "New York"])
                                    .row(["Bob", "32", "San Francisco"])
                                    .row(["Charlie", "25", "London"])
                                    .into_any_element(),
                            ),
                            single_example(
                                "Two Column Table",
                                Table::new()
                                    .header(["Category", "Value"])
                                    .width(px(300.))
                                    .row(["Revenue", "$100,000"])
                                    .row(["Expenses", "$75,000"])
                                    .row(["Profit", "$25,000"])
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Styled Tables",
                        vec![
                            single_example(
                                "Default",
                                Table::new()
                                    .width(px(400.))
                                    .header(["Product", "Price", "Stock"])
                                    .row(["Laptop", "$999", "In Stock"])
                                    .row(["Phone", "$599", "Low Stock"])
                                    .row(["Tablet", "$399", "Out of Stock"])
                                    .into_any_element(),
                            ),
                            single_example(
                                "Striped",
                                Table::new()
                                    .width(px(400.))
                                    .striped()
                                    .header(["Product", "Price", "Stock"])
                                    .row(["Laptop", "$999", "In Stock"])
                                    .row(["Phone", "$599", "Low Stock"])
                                    .row(["Tablet", "$399", "Out of Stock"])
                                    .row(["Headphones", "$199", "In Stock"])
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Mixed Content Table",
                        vec![single_example(
                            "Table with Elements",
                            Table::new()
                                .width(px(840.))
                                .header(["Status", "Name", "Priority", "Deadline", "Action"])
                                .row([
                                    Indicator::dot().color(Color::Success).into_any_element(),
                                    "Project A".into_any_element(),
                                    "High".into_any_element(),
                                    "2023-12-31".into_any_element(),
                                    Button::new("view_a", "View")
                                        .style(ButtonStyle::Filled)
                                        .full_width()
                                        .into_any_element(),
                                ])
                                .row([
                                    Indicator::dot().color(Color::Warning).into_any_element(),
                                    "Project B".into_any_element(),
                                    "Medium".into_any_element(),
                                    "2024-03-15".into_any_element(),
                                    Button::new("view_b", "View")
                                        .style(ButtonStyle::Filled)
                                        .full_width()
                                        .into_any_element(),
                                ])
                                .row([
                                    Indicator::dot().color(Color::Error).into_any_element(),
                                    "Project C".into_any_element(),
                                    "Low".into_any_element(),
                                    "2024-06-30".into_any_element(),
                                    Button::new("view_c", "View")
                                        .style(ButtonStyle::Filled)
                                        .full_width()
                                        .into_any_element(),
                                ])
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}
