use ui::{
    Button, ButtonCommon as _, ButtonSize, ButtonStyle, Clickable as _, Context, ElementId,
    IntoElement, Tooltip,
};

use crate::{
    CsvPreviewView,
    table_like_content::TableLikeContent,
    types::{AnyColumn, DataRow, DisplayRow},
};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OrderingDirection {
    Asc,
    Desc,
}

#[derive(Clone, Copy)]
pub struct Ordering {
    /// 0-based column index
    pub col_idx: AnyColumn,
    /// Direction of ordering
    pub direction: OrderingDirection,
}

/// Ordered indices mapping display positions to data positions
#[derive(Debug, Clone)]
pub struct OrderedIndices {
    mapping: HashMap<DisplayRow, DataRow>,
}

impl OrderedIndices {
    /// Get the data row for a given display row
    pub fn get_data_row(&self, display_row: DisplayRow) -> Option<DataRow> {
        self.mapping.get(&display_row).copied()
    }

    /// Get the display row for a given data row (reverse lookup)
    pub fn get_display_row(&self, data_row: DataRow) -> Option<DisplayRow> {
        self.mapping
            .iter()
            .find(|(_, mapped_data_row)| **mapped_data_row == data_row)
            .map(|(display_row, _)| *display_row)
    }
}

/// Generate ordered row indices based on current ordering settings.
/// Returns a mapping from DisplayRow to DataRow.
/// Note: ordering.col_idx refers to CSV data columns (0-based), not display columns
/// (display columns include the line number column at index 0)
pub fn generate_ordered_indices(
    ordering: Option<Ordering>,
    contents: &TableLikeContent,
) -> OrderedIndices {
    let indices: Vec<usize> = (0..contents.rows.len()).collect();

    let ordered_indices = if let Some(ordering) = ordering {
        order_indices(contents, indices, ordering)
    } else {
        indices
    };

    // Create mapping from display position to data row
    let mapping: HashMap<DisplayRow, DataRow> = ordered_indices
        .iter()
        .enumerate()
        .map(|(display_idx, &data_idx)| (DisplayRow::from(display_idx), DataRow::from(data_idx)))
        .collect();

    OrderedIndices { mapping }
}

fn order_indices(
    contents: &TableLikeContent,
    mut indices: Vec<usize>,
    ordering: Ordering,
) -> Vec<usize> {
    indices.sort_by(|&a, &b| {
        let row_a = &contents.rows[a];
        let row_b = &contents.rows[b];

        let val_a = row_a
            .get(ordering.col_idx.get())
            .map(|s| s.display_value().as_ref())
            .unwrap_or("");
        let val_b = row_b
            .get(ordering.col_idx.get())
            .map(|s| s.display_value().as_ref())
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

    indices
}

impl CsvPreviewView {
    pub(crate) fn create_sort_button(
        &self,
        cx: &mut Context<'_, CsvPreviewView>,
        col_idx: AnyColumn,
    ) -> Button {
        let sort_btn = Button::new(
            ElementId::NamedInteger("sort-button".into(), col_idx.get() as u64),
            match self.ordering {
                Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                    OrderingDirection::Asc => "↓",
                    OrderingDirection::Desc => "↑",
                },
                _ => "↕", // Unsorted/available for sorting
            },
        )
        .size(ButtonSize::Compact)
        .style(if self.ordering.is_some_and(|o| o.col_idx == col_idx) {
            ButtonStyle::Filled
        } else {
            ButtonStyle::Subtle
        })
        .tooltip(Tooltip::text(match self.ordering {
            Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                OrderingDirection::Asc => "Sorted A-Z. Click to sort Z-A",
                OrderingDirection::Desc => "Sorted Z-A. Click to disable sorting",
            },
            _ => "Not sorted. Click to sort A-Z",
        }))
        .on_click(cx.listener(move |this, _event, _window, cx| {
            let new_ordering = match this.ordering {
                Some(ordering) if ordering.col_idx == col_idx => {
                    // Same column clicked - cycle through states
                    match ordering.direction {
                        OrderingDirection::Asc => Some(Ordering {
                            col_idx,
                            direction: OrderingDirection::Desc,
                        }),
                        OrderingDirection::Desc => None, // Clear sorting
                    }
                }
                _ => {
                    // Different column or no sorting - start with ascending
                    Some(Ordering {
                        col_idx,
                        direction: OrderingDirection::Asc,
                    })
                }
            };

            this.ordering = new_ordering;
            this.update_ordered_indices();
            cx.notify();
        }));
        sort_btn
    }
}
