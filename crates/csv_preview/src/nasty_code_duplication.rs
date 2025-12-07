//! # Const Generics Workaround: Minimal Duplication for Dynamic Tables
//!
//! This module bridges the gap between CSV files' dynamic column counts and Zed's const-generic
//! `Table<COLS>` API which requires compile-time column knowledge for type safety.
//!
//! ## The Problem
//!
//! - **Zed's Table**: `Table::<3>` and `TableColumnWidths<3>` require compile-time constants
//! - **CSV files**: Have 1-30+ columns determined at runtime
//! - **Rust constraint**: Can't convert runtime values to const generics
//!
//! ## Why This Approach
//!
//! Alternative approaches fail due to Rust's type system:
//! - Can't return different `TableColumnWidths<N>` types from match arms
//! - Can't use trait objects with const generics
//! - Can't store different generic types in collections
//!
//! ## Solution
//!
//! Pre-allocate one `Entity<TableColumnWidths<N>>` for each column count (1-30), then use
//! a match statement to bridge runtime values to compile-time types. All table logic remains
//! in a single generic `create_table<COLS>()` method.
use gpui::{AppContext as _, Entity};
use ui::{
    ActiveTheme as _, AnyElement, Context, IntoElement as _, ParentElement as _, SharedString,
    Styled as _, TableColumnWidths, div,
};

use crate::CsvPreviewView;

/// Pre-allocated `TableColumnWidths` entities for columns 1-30.
///
/// Each field has a concrete type known at compile time, enabling the match statement
/// in `render_table_with_cols` to select the correct entity type. Collections like
/// `Vec` or `HashMap` can't store different `TableColumnWidths<N>` generic types.
pub(crate) struct ColumnWidths {
    widths_1: Entity<TableColumnWidths<1>>,
    widths_2: Entity<TableColumnWidths<2>>,
    widths_3: Entity<TableColumnWidths<3>>,
    widths_4: Entity<TableColumnWidths<4>>,
    widths_5: Entity<TableColumnWidths<5>>,
    widths_6: Entity<TableColumnWidths<6>>,
    widths_7: Entity<TableColumnWidths<7>>,
    widths_8: Entity<TableColumnWidths<8>>,
    widths_9: Entity<TableColumnWidths<9>>,
    widths_10: Entity<TableColumnWidths<10>>,
    widths_11: Entity<TableColumnWidths<11>>,
    widths_12: Entity<TableColumnWidths<12>>,
    widths_13: Entity<TableColumnWidths<13>>,
    widths_14: Entity<TableColumnWidths<14>>,
    widths_15: Entity<TableColumnWidths<15>>,
    widths_16: Entity<TableColumnWidths<16>>,
    widths_17: Entity<TableColumnWidths<17>>,
    widths_18: Entity<TableColumnWidths<18>>,
    widths_19: Entity<TableColumnWidths<19>>,
    widths_20: Entity<TableColumnWidths<20>>,
    widths_21: Entity<TableColumnWidths<21>>,
    widths_22: Entity<TableColumnWidths<22>>,
    widths_23: Entity<TableColumnWidths<23>>,
    widths_24: Entity<TableColumnWidths<24>>,
    widths_25: Entity<TableColumnWidths<25>>,
    widths_26: Entity<TableColumnWidths<26>>,
    widths_27: Entity<TableColumnWidths<27>>,
    widths_28: Entity<TableColumnWidths<28>>,
    widths_29: Entity<TableColumnWidths<29>>,
    widths_30: Entity<TableColumnWidths<30>>,
}

impl ColumnWidths {
    pub(crate) fn new(cx: &mut Context<CsvPreviewView>) -> Self {
        Self {
            widths_1: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_2: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_3: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_4: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_5: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_6: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_7: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_8: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_9: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_10: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_11: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_12: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_13: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_14: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_15: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_16: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_17: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_18: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_19: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_20: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_21: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_22: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_23: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_24: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_25: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_26: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_27: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_28: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_29: cx.new(|cx| TableColumnWidths::new(cx)),
            widths_30: cx.new(|cx| TableColumnWidths::new(cx)),
        }
    }
}

impl CsvPreviewView {
    /// Maps runtime column count to compile-time const generic.
    ///
    /// Takes number of headers (+1 for line numbers col) and dispatches to `create_table<COLS>()`
    /// with the matching `TableColumnWidths<COLS>` entity. Falls back to ASCII
    /// table for unsupported column counts.
    pub(crate) fn render_table_with_cols(&self, cx: &mut Context<Self>) -> AnyElement {
        let w = &self.column_widths;
        // Add 1 for the line number column
        let column_count = self.contents.headers.len() + 1;

        match column_count {
            1 => self.create_table::<1>(&w.widths_1, cx),
            2 => self.create_table::<2>(&w.widths_2, cx),
            3 => self.create_table::<3>(&w.widths_3, cx),
            4 => self.create_table::<4>(&w.widths_4, cx),
            5 => self.create_table::<5>(&w.widths_5, cx),
            6 => self.create_table::<6>(&w.widths_6, cx),
            7 => self.create_table::<7>(&w.widths_7, cx),
            8 => self.create_table::<8>(&w.widths_8, cx),
            9 => self.create_table::<9>(&w.widths_9, cx),
            10 => self.create_table::<10>(&w.widths_10, cx),
            11 => self.create_table::<11>(&w.widths_11, cx),
            12 => self.create_table::<12>(&w.widths_12, cx),
            13 => self.create_table::<13>(&w.widths_13, cx),
            14 => self.create_table::<14>(&w.widths_14, cx),
            15 => self.create_table::<15>(&w.widths_15, cx),
            16 => self.create_table::<16>(&w.widths_16, cx),
            17 => self.create_table::<17>(&w.widths_17, cx),
            18 => self.create_table::<18>(&w.widths_18, cx),
            19 => self.create_table::<19>(&w.widths_19, cx),
            20 => self.create_table::<20>(&w.widths_20, cx),
            21 => self.create_table::<21>(&w.widths_21, cx),
            22 => self.create_table::<22>(&w.widths_22, cx),
            23 => self.create_table::<23>(&w.widths_23, cx),
            24 => self.create_table::<24>(&w.widths_24, cx),
            25 => self.create_table::<25>(&w.widths_25, cx),
            26 => self.create_table::<26>(&w.widths_26, cx),
            27 => self.create_table::<27>(&w.widths_27, cx),
            28 => self.create_table::<28>(&w.widths_28, cx),
            29 => self.create_table::<29>(&w.widths_29, cx),
            30 => self.create_table::<30>(&w.widths_30, cx),
            _ => self.render_fallback_table(cx),
        }
    }

    /// Renders a fallback ASCII table for unsupported column counts (>30).
    ///
    /// Creates a monospace text table with proper column alignment and borders.
    /// Used when the column count exceeds our pre-allocated `TableColumnWidths`
    /// entities or when the resizable table fails to render.
    ///
    /// The table format includes:
    /// - Header row with column names
    /// - Separator line with dashes
    /// - Data rows with consistent spacing
    /// - Monospace font for alignment
    fn render_fallback_table(&self, cx: &mut Context<Self>) -> AnyElement {
        let max_widths = self.calculate_column_widths_pixels();
        let header_row = self.format_row(&self.contents.headers, &max_widths);

        let separator = max_widths
            .iter()
            .map(|&width| "-".repeat(width))
            .collect::<Vec<_>>()
            .join("-+-");

        let data_rows: Vec<String> = self
            .contents
            .rows
            .iter()
            .map(|row| self.format_row(row, &max_widths))
            .collect();

        let all_content = format!("{}\n{}\n{}", header_row, separator, data_rows.join("\n"));

        div()
            .font_family("monospace")
            .w_full()
            .h_full()
            .p_2()
            .bg(cx.theme().colors().editor_subheader_background)
            .child(all_content)
            .into_any_element()
    }

    fn calculate_column_widths_pixels(&self) -> Vec<usize> {
        if self.contents.headers.is_empty() {
            return vec![];
        }

        let num_cols = self.contents.headers.len();
        let mut max_widths = vec![0; num_cols];

        for (i, header) in self.contents.headers.iter().enumerate() {
            max_widths[i] = max_widths[i].max(header.len());
        }

        for row in &self.contents.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < max_widths.len() {
                    max_widths[i] = max_widths[i].max(cell.len());
                }
            }
        }

        max_widths.into_iter().map(|w| w.max(3) + 2).collect()
    }

    fn format_row(&self, row: &[SharedString], widths: &[usize]) -> String {
        row.iter()
            .enumerate()
            .map(|(i, cell)| {
                let width = widths.get(i).copied().unwrap_or(10);
                format!("{:width$}", cell.as_ref(), width = width)
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }
}
