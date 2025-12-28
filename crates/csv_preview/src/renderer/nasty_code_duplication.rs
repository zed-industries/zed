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
use crate::data_table::TableColumnWidths;
use gpui::{AppContext as _, Entity};
use ui::{AnyElement, Context};

use crate::CsvPreviewView;

pub(crate) struct ColumnWidths {
    widths: Entity<TableColumnWidths>,
}

impl ColumnWidths {
    pub(crate) fn new(cx: &mut Context<CsvPreviewView>, cols: usize) -> Self {
        Self {
            widths: cx.new(|cx| TableColumnWidths::new(cx, cols)),
        }
    }
    /// Replace the current `TableColumnWidths` entity with a new one for the given column count.
    pub(crate) fn replace(&self, cx: &mut Context<CsvPreviewView>, cols: usize) {
        self.widths
            .update(cx, |entity, cx| *entity = TableColumnWidths::new(cx, cols));
    }
}

impl CsvPreviewView {
    /// Maps runtime column count to compile-time const generic.
    ///
    /// Takes number of headers (+1 for line numbers col) and dispatches to `create_table<COLS>()`
    /// with the matching `TableColumnWidths<COLS>` entity.
    pub(crate) fn render_table_with_cols(&self, cx: &mut Context<Self>, cols: usize) -> AnyElement {
        // Add 1 for the line number column
        self.column_widths.replace(cx, cols + 1);
        self.create_table(&self.column_widths.widths, cx)
    }
}
