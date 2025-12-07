use gpui::{AnyElement, Entity};
use ui::{
    DefiniteLength, SharedString, Table, TableColumnWidths, TableResizeBehavior, div, prelude::*,
};

use crate::{CsvPreviewView, Ordering, OrderingDirection};

impl Render for CsvPreviewView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .w_full()
            .h_full()
            .p_4()
            .bg(theme.colors().editor_background)
            .child(
                Button::new(
                    "order-first-column",
                    if let Some(order) = self.ordering {
                        let col_name = self
                            .contents
                            .headers
                            .get(0)
                            .map(|h| h.as_ref())
                            .unwrap_or("Column 1");
                        if order.direction == OrderingDirection::Asc {
                            format!("Sort {} desc", col_name)
                        } else {
                            "Clear sorting".to_string()
                        }
                    } else {
                        let col_name = self
                            .contents
                            .headers
                            .get(0)
                            .map(|h| h.as_ref())
                            .unwrap_or("Column 1");
                        format!("Sort {} asc", col_name)
                    },
                )
                .on_click(cx.listener(|this, _event, _window, cx| {
                    let new_dir = match this.ordering {
                        Some(ordering) => match ordering.direction {
                            OrderingDirection::Asc => Some(OrderingDirection::Desc),
                            OrderingDirection::Desc => None,
                        },
                        None => Some(OrderingDirection::Asc),
                    };

                    this.ordering = new_dir.map(|d| Ordering {
                        col_idx: 0, // For poc purposes always sorting 0th column
                        direction: d,
                    });
                    cx.notify();
                })),
            )
            .child({
                if self.contents.headers.is_empty() {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h_32()
                        .text_ui(cx)
                        .text_color(cx.theme().colors().text_muted)
                        .child("No CSV content to display")
                        .into_any_element()
                } else {
                    let column_count = self.contents.headers.len();

                    self.render_table_with_cols(column_count, cx)
                }
            })
    }
}

impl CsvPreviewView {
    /// Generate ordered row indices based on current ordering settings
    fn generate_ordered_indices(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.contents.rows.len()).collect();

        if let Some(ordering) = self.ordering {
            indices.sort_by(|&a, &b| {
                let row_a = &self.contents.rows[a];
                let row_b = &self.contents.rows[b];

                let val_a = row_a
                    .get(ordering.col_idx)
                    .map(|s| s.as_ref())
                    .unwrap_or("");
                let val_b = row_b
                    .get(ordering.col_idx)
                    .map(|s| s.as_ref())
                    .unwrap_or("");

                // Try numeric comparison first, fall back to string comparison
                let cmp = match (val_a.parse::<f64>(), val_b.parse::<f64>()) {
                    (Ok(num_a), Ok(num_b)) => num_a
                        .partial_cmp(&num_b)
                        .unwrap_or(std::cmp::Ordering::Equal),
                    _ => val_a.cmp(val_b),
                };

                match ordering.direction {
                    OrderingDirection::Asc => cmp,
                    OrderingDirection::Desc => cmp.reverse(),
                }
            });
        }

        indices
    }

    pub(crate) fn create_table<const COLS: usize>(
        &self,
        current_widths: &Entity<TableColumnWidths<COLS>>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let widths = [DefiniteLength::Fraction(1. / COLS as f32); COLS];
        let resize_behaviors = [TableResizeBehavior::Resizable; COLS];

        self.create_table_inner(
            self.contents.rows.len(),
            widths,
            resize_behaviors,
            current_widths,
            cx,
        )
    }

    fn create_table_inner<const COLS: usize>(
        &self,
        row_count: usize,
        widths: [DefiniteLength; COLS],
        resize_behaviors: [TableResizeBehavior; COLS],
        current_widths: &Entity<TableColumnWidths<COLS>>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // Create headers array
        let mut headers = Vec::with_capacity(COLS);
        for i in 0..COLS {
            headers.push(
                self.contents
                    .headers
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("Col {}", i + 1).into()),
            );
        }
        let headers_array: [SharedString; COLS] = headers.try_into().unwrap();

        Table::new()
            .interactable(&self.table_interaction_state)
            .striped()
            .column_widths(widths)
            .resizable_columns(resize_behaviors, current_widths, cx)
            .header(headers_array)
            .uniform_list(
                "csv-table",
                row_count,
                cx.processor(move |this, range: std::ops::Range<usize>, _window, _cx| {
                    let ordered_indices = this.generate_ordered_indices();

                    range
                        .filter_map(|display_index| {
                            // Get the actual row index from our ordered indices
                            let row_index = *ordered_indices.get(display_index)?;
                            let row = this.contents.rows.get(row_index)?;

                            let mut elements = Vec::with_capacity(COLS);
                            for col in 0..COLS {
                                let cell_content: SharedString =
                                    row.get(col).cloned().unwrap_or_else(|| "".into());
                                elements.push(div().child(cell_content).into_any_element());
                            }

                            let elements_array: [gpui::AnyElement; COLS] =
                                elements.try_into().ok()?;
                            Some(elements_array)
                        })
                        .collect()
                }),
            )
            .into_any_element()
    }
}
