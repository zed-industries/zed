use std::sync::Arc;

use gpui::{div, prelude::*, px, App, CursorStyle, IntoElement, MouseButton, MouseDownEvent, Pixels, Point, SharedString, UniformListScrollHandle};
use serde::{Deserialize, Serialize};
use ui::{prelude::*, Icon, IconName, Label};

use database_core::QueryResult;

pub const ROWS_PER_PAGE_OPTIONS: &[usize] = &[25, 50, 100, 250];

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TableConfig {
    pub default_column_width: f32,
    pub column_widths: Vec<f32>,
    pub max_cell_chars: usize,
    pub row_height: f32,
    pub header_height: f32,
    pub fk_columns: HashSet<usize>,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            default_column_width: 200.0,
            column_widths: Vec::new(),
            max_cell_chars: 500,
            row_height: 26.0,
            header_height: 28.0,
            fk_columns: HashSet::new(),
        }
    }
}

impl TableConfig {
    fn column_width(&self, index: usize) -> f32 {
        self.column_widths
            .get(index)
            .copied()
            .unwrap_or(self.default_column_width)
    }

    fn total_width(&self, column_count: usize) -> Pixels {
        let width: f32 = (0..column_count)
            .map(|i| self.column_width(i))
            .sum();
        px(width)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

fn truncate_cell(value: &str, max_chars: usize) -> SharedString {
    let needs_replace = value.contains(['\n', '\r']);
    let single_line = if needs_replace {
        std::borrow::Cow::Owned(value.replace(['\n', '\r'], " "))
    } else {
        std::borrow::Cow::Borrowed(value)
    };

    if single_line.chars().count() > max_chars {
        let truncated: String = single_line.chars().take(max_chars).collect();
        SharedString::from(format!("{}...", truncated))
    } else {
        SharedString::from(single_line.into_owned())
    }
}

pub fn render_results_table(
    result: &QueryResult,
    page_offset: usize,
    scroll_handle: &UniformListScrollHandle,
    sort_columns: &[(usize, SortDirection)],
    selected_cell: Option<(usize, usize)>,
    on_header_click: Arc<dyn Fn(usize, &mut gpui::Window, &mut App) + Send + Sync>,
    on_cell_click: Arc<dyn Fn(usize, usize, usize, &mut gpui::Window, &mut App) + Send + Sync>,
    on_cell_secondary_click: Arc<dyn Fn(usize, usize, Point<Pixels>, &mut gpui::Window, &mut App) + Send + Sync>,
    on_resize_start: Arc<dyn Fn(usize, Pixels, &mut gpui::Window, &mut App) + Send + Sync>,
    config: &TableConfig,
    cx: &App,
) -> impl IntoElement {
    let max_cell_chars = config.max_cell_chars;
    let row_height = config.row_height;
    let header_height = config.header_height;

    let columns: Vec<SharedString> = result
        .columns
        .iter()
        .map(|c| SharedString::from(c.clone()))
        .collect();

    let fk_columns = Arc::new(config.fk_columns.clone());

    let rows: Vec<Vec<SharedString>> = result
        .rows
        .iter()
        .map(|row| row.iter().map(|cell| truncate_cell(&cell.to_string(), max_cell_chars)).collect())
        .collect();

    let row_count = rows.len();
    let column_count = columns.len();
    let total_width = config.total_width(column_count);

    let col_widths: Arc<Vec<f32>> = Arc::new(
        (0..column_count).map(|i| config.column_width(i)).collect(),
    );

    div()
        .id("results-scroll-container")
        .flex()
        .flex_col()
        .flex_grow()
        .w_full()
        .overflow_x_scroll()
        .overflow_y_hidden()
        .child(render_header_row(&columns, total_width, header_height, &col_widths, sort_columns, on_header_click, on_resize_start, cx))
        .child(
            gpui::uniform_list("results-table", row_count, {
                let col_widths = col_widths.clone();
                move |range, _window, cx| {
                    range
                        .map(|index| {
                            let row = &rows[index];
                            let absolute_row = page_offset + index;
                            render_data_row(
                                absolute_row,
                                index,
                                row,
                                total_width,
                                row_height,
                                &col_widths,
                                selected_cell,
                                &fk_columns,
                                on_cell_click.clone(),
                                on_cell_secondary_click.clone(),
                                cx,
                            )
                        })
                        .collect()
                }
            })
            .track_scroll(scroll_handle)
            .min_w(total_width)
            .flex_grow(),
        )
}

fn render_header_row(
    columns: &[SharedString],
    total_width: Pixels,
    header_height: f32,
    col_widths: &[f32],
    sort_columns: &[(usize, SortDirection)],
    on_header_click: Arc<dyn Fn(usize, &mut gpui::Window, &mut App) + Send + Sync>,
    on_resize_start: Arc<dyn Fn(usize, Pixels, &mut gpui::Window, &mut App) + Send + Sync>,
    cx: &App,
) -> impl IntoElement {
    let mut header = div()
        .flex()
        .flex_none()
        .h(px(header_height))
        .min_w(total_width)
        .border_b_1()
        .border_color(cx.theme().colors().border);

    for (col_index, column) in columns.iter().enumerate() {
        let col_width = col_widths.get(col_index).copied().unwrap_or(200.0);
        let sort_info = sort_columns
            .iter()
            .enumerate()
            .find(|(_, (idx, _))| *idx == col_index)
            .map(|(priority, (_, dir))| (priority, *dir));
        let is_sorted = sort_info.is_some();

        header = header.child(
            div()
                .id(gpui::ElementId::named_usize("header-col", col_index))
                .flex_none()
                .w(px(col_width))
                .h_full()
                .flex()
                .items_center()
                .overflow_hidden()
                .border_r_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    div()
                        .id(gpui::ElementId::named_usize("header-label", col_index))
                        .flex_grow()
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px_2()
                        .overflow_hidden()
                        .cursor_pointer()
                        .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                        .when(is_sorted, |this| {
                            this.bg(cx.theme().colors().ghost_element_selected)
                        })
                        .on_click({
                            let on_header_click = on_header_click.clone();
                            move |_, window, cx| {
                                on_header_click(col_index, window, cx);
                            }
                        })
                        .child(
                            Label::new(column.clone())
                                .size(LabelSize::Small)
                                .weight(gpui::FontWeight::BOLD)
                                .color(ui::Color::Default)
                                .single_line(),
                        )
                        .when_some(sort_info, |this, (priority, dir)| {
                            let icon = match dir {
                                SortDirection::Ascending => IconName::ArrowUp,
                                SortDirection::Descending => IconName::ArrowDown,
                            };
                            this.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_px()
                                    .child(
                                        Icon::new(icon)
                                            .size(IconSize::XSmall)
                                            .color(ui::Color::Accent),
                                    )
                                    .when(sort_columns.len() > 1, |this| {
                                        this.child(
                                            Label::new(SharedString::from(format!("{}", priority + 1)))
                                                .size(LabelSize::XSmall)
                                                .color(ui::Color::Accent),
                                        )
                                    }),
                            )
                        }),
                )
                .child(render_resize_handle(col_index, on_resize_start.clone())),
        );
    }

    header
}

const RESIZE_HANDLE_WIDTH: f32 = 4.0;
pub const MIN_COLUMN_WIDTH: f32 = 50.0;

fn render_resize_handle(
    col_index: usize,
    on_resize_start: Arc<dyn Fn(usize, Pixels, &mut gpui::Window, &mut App) + Send + Sync>,
) -> impl IntoElement {
    div()
        .id(gpui::ElementId::named_usize("resize-handle", col_index))
        .flex_none()
        .w(px(RESIZE_HANDLE_WIDTH))
        .h_full()
        .cursor(CursorStyle::ResizeLeftRight)
        .on_mouse_down(MouseButton::Left, {
            let on_resize_start = on_resize_start.clone();
            move |event: &MouseDownEvent, window, cx| {
                on_resize_start(col_index, event.position.x, window, cx);
            }
        })
}

fn render_data_row(
    absolute_row: usize,
    relative_row: usize,
    row: &[SharedString],
    total_width: Pixels,
    row_height: f32,
    col_widths: &[f32],
    selected_cell: Option<(usize, usize)>,
    fk_columns: &HashSet<usize>,
    on_cell_click: Arc<dyn Fn(usize, usize, usize, &mut gpui::Window, &mut App) + Send + Sync>,
    on_cell_secondary_click: Arc<dyn Fn(usize, usize, Point<Pixels>, &mut gpui::Window, &mut App) + Send + Sync>,
    cx: &App,
) -> gpui::AnyElement {
    let is_even = absolute_row.is_multiple_of(2);

    let mut row_element = div()
        .id(gpui::ElementId::named_usize("result-row", absolute_row))
        .flex()
        .flex_none()
        .h(px(row_height))
        .min_w(total_width)
        .when(is_even, |this| {
            this.bg(cx.theme().colors().surface_background)
        });

    for (col_index, cell) in row.iter().enumerate() {
        let col_width = col_widths.get(col_index).copied().unwrap_or(200.0);
        let is_selected = selected_cell
            .map(|(r, c)| r == relative_row && c == col_index)
            .unwrap_or(false);
        let is_fk = fk_columns.contains(&col_index);
        let is_null_cell = cell.as_ref() == "NULL";

        let cell_color = if is_null_cell {
            ui::Color::Muted
        } else if is_fk {
            ui::Color::Accent
        } else {
            ui::Color::Default
        };

        row_element = row_element.child(
            div()
                .id(gpui::ElementId::Name(SharedString::from(format!(
                    "cell-{}-{}",
                    absolute_row, col_index
                ))))
                .flex_none()
                .w(px(col_width))
                .h_full()
                .flex()
                .items_center()
                .px_2()
                .overflow_hidden()
                .border_r_1()
                .border_color(if is_selected {
                    cx.theme().colors().border_focused
                } else {
                    cx.theme().colors().border_transparent
                })
                .when(is_selected, |this| {
                    this.border_1()
                        .bg(cx.theme().colors().ghost_element_selected)
                })
                .cursor_pointer()
                .on_click({
                    let on_cell_click = on_cell_click.clone();
                    move |event, window, cx| {
                        on_cell_click(relative_row, col_index, event.click_count(), window, cx);
                    }
                })
                .on_mouse_down(MouseButton::Right, {
                    let on_cell_secondary_click = on_cell_secondary_click.clone();
                    move |event: &MouseDownEvent, window, cx| {
                        on_cell_secondary_click(relative_row, col_index, event.position, window, cx);
                    }
                })
                .child(
                    Label::new(cell.clone())
                        .size(LabelSize::Small)
                        .single_line()
                        .color(cell_color)
                        .when(is_fk && !is_null_cell, |label| {
                            label.underline()
                        }),
                ),
        );
    }

    row_element.into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_cell_short() {
        let result = truncate_cell("hello", 500);
        assert_eq!(result.as_ref(), "hello");
    }

    #[test]
    fn test_truncate_cell_newlines() {
        let result = truncate_cell("line1\nline2\rline3", 500);
        assert_eq!(result.as_ref(), "line1 line2 line3");
    }

    #[test]
    fn test_truncate_cell_long() {
        let max_chars = 500;
        let long_string: String = "a".repeat(max_chars + 100);
        let result = truncate_cell(&long_string, max_chars);
        let expected_len = max_chars + 3; // +3 for "..."
        assert_eq!(result.len(), expected_len);
        assert!(result.ends_with("..."));
    }
}

pub fn render_status_bar(
    result: &QueryResult,
    total_row_count: usize,
    cx: &App,
) -> impl IntoElement {
    let execution_ms = result.execution_time.as_millis();

    let mut status_parts = vec![format!(
        "{} row{}",
        total_row_count,
        if total_row_count != 1 { "s" } else { "" }
    )];

    if let Some(affected) = result.affected_rows {
        status_parts.push(format!("{} affected", affected));
    }

    status_parts.push(format!("{}ms", execution_ms));

    let status_text = status_parts.join(" \u{00B7} ");

    div()
        .flex()
        .flex_none()
        .h(px(24.0))
        .w_full()
        .px_2()
        .items_center()
        .border_t_1()
        .border_color(cx.theme().colors().border)
        .child(
            Label::new(status_text)
                .size(LabelSize::XSmall)
                .color(ui::Color::Muted),
        )
}
