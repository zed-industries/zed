use ui::{
    Button, ButtonCommon as _, ButtonSize, ButtonStyle, Clickable as _, Context, ElementId, Tooltip,
};

use crate::{
    CsvPreviewView,
    table_like_content::TableLikeContent,
    types::{AnyColumn, DataRow, DisplayRow},
};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Clone, Copy)]
pub struct SortingConfig {
    /// 0-based column index
    pub col_idx: AnyColumn,
    /// Direction of sorting (asc/desc)
    pub direction: SortDirection,
}

/// Sorted indices mapping display positions to data positions
#[derive(Debug, Clone)]
pub struct SortedIndices {
    mapping: HashMap<DisplayRow, DataRow>,
}

impl SortedIndices {
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

/// Generate sorted row indices based on current sorting settings.
/// Returns a mapping from DisplayRow to DataRow.
/// Note: sorting.col_idx refers to CSV data columns (0-based), not display columns
/// (display columns include the line number column at index 0)
pub fn generate_sorted_indices(
    sorting: Option<SortingConfig>,
    contents: &TableLikeContent,
) -> SortedIndices {
    let indices: Vec<usize> = (0..contents.rows.len()).collect();

    let sorted_indices = if let Some(sorting) = sorting {
        sort_indices(contents, indices, sorting)
    } else {
        indices
    };

    // Create mapping from display position to data row
    let mapping: HashMap<DisplayRow, DataRow> = sorted_indices
        .iter()
        .enumerate()
        .map(|(display_idx, &data_idx)| (DisplayRow::from(display_idx), DataRow::from(data_idx)))
        .collect();

    SortedIndices { mapping }
}

fn sort_indices(
    contents: &TableLikeContent,
    mut indices: Vec<usize>,
    sorting: SortingConfig,
) -> Vec<usize> {
    indices.sort_by(|&a, &b| {
        let row_a = &contents.rows[a];
        let row_b = &contents.rows[b];

        let val_a = row_a
            .get(sorting.col_idx)
            .map(|s| s.display_value().as_ref())
            .unwrap_or("");
        let val_b = row_b
            .get(sorting.col_idx)
            .map(|s| s.display_value().as_ref())
            .unwrap_or("");

        // Try numeric comparison first, fall back to string comparison
        let cmp = match (val_a.parse::<f64>(), val_b.parse::<f64>()) {
            (Ok(num_a), Ok(num_b)) => num_a
                .partial_cmp(&num_b)
                .unwrap_or(std::cmp::Ordering::Equal),
            _ => val_a.cmp(val_b),
        };

        match sorting.direction {
            SortDirection::Asc => cmp,
            SortDirection::Desc => cmp.reverse(),
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
            match self.sorting_cfg {
                Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                    SortDirection::Asc => "↓",
                    SortDirection::Desc => "↑",
                },
                _ => "↕", // Unsorted/available for sorting
            },
        )
        .size(ButtonSize::Compact)
        .style(if self.sorting_cfg.is_some_and(|o| o.col_idx == col_idx) {
            ButtonStyle::Filled
        } else {
            ButtonStyle::Subtle
        })
        .tooltip(Tooltip::text(match self.sorting_cfg {
            Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                SortDirection::Asc => "Sorted A-Z. Click to sort Z-A",
                SortDirection::Desc => "Sorted Z-A. Click to disable sorting",
            },
            _ => "Not sorted. Click to sort A-Z",
        }))
        .on_click(cx.listener(move |this, _event, _window, cx| {
            let new_ordering = match this.sorting_cfg {
                Some(ordering) if ordering.col_idx == col_idx => {
                    // Same column clicked - cycle through states
                    match ordering.direction {
                        SortDirection::Asc => Some(SortingConfig {
                            col_idx,
                            direction: SortDirection::Desc,
                        }),
                        SortDirection::Desc => None, // Clear sorting
                    }
                }
                _ => {
                    // Different column or no sorting - start with ascending
                    Some(SortingConfig {
                        col_idx,
                        direction: SortDirection::Asc,
                    })
                }
            };

            this.sorting_cfg = new_ordering;
            this.re_sort_indices();
            cx.notify();
        }));
        sort_btn
    }
}
