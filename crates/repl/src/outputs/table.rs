//! # Table Output for REPL
//!
//! This module provides functionality to render tabular data in Zed's REPL output.
//!
//! It supports the [Frictionless Data Table Schema](https://specs.frictionlessdata.io/table-schema/)
//! for data interchange, implemented by Pandas in Python and Polars for Deno.
//!
//! # Python Example
//!
//! Tables can be created and displayed in two main ways:
//!
//! 1. Using raw JSON data conforming to the Tabular Data Resource specification.
//! 2. Using Pandas DataFrames (in Python kernels).
//!
//! ## Raw JSON Method
//!
//! To create a table using raw JSON, you need to provide a JSON object that conforms
//! to the Tabular Data Resource specification. Here's an example:
//!
//! ```json
//! {
//!     "schema": {
//!         "fields": [
//!             {"name": "id", "type": "integer"},
//!             {"name": "name", "type": "string"},
//!             {"name": "age", "type": "integer"}
//!         ]
//!     },
//!     "data": [
//!         {"id": 1, "name": "Alice", "age": 30},
//!         {"id": 2, "name": "Bob", "age": 28},
//!         {"id": 3, "name": "Charlie", "age": 35}
//!     ]
//! }
//! ```
//!
//! ## Pandas Method
//!
//! To create a table using Pandas in a Python kernel, you can use the following steps:
//!
//! ```python
//! import pandas as pd
//!
//! # Enable table schema output
//! pd.set_option('display.html.table_schema', True)
//!
//! # Create a DataFrame
//! df = pd.DataFrame({
//!     'id': [1, 2, 3],
//!     'name': ['Alice', 'Bob', 'Charlie'],
//!     'age': [30, 28, 35]
//! })
//!
//! # Display the DataFrame
//! display(df)
//! ```
use gpui::{AnyElement, ClipboardItem, TextRun};
use runtimelib::datatable::TableSchema;
use runtimelib::media::datatable::TabularDataResource;
use serde_json::Value;
use settings::Settings;
use theme::ThemeSettings;
use ui::{IntoElement, Styled, div, prelude::*, v_flex};
use util::markdown::MarkdownEscaped;

use crate::outputs::OutputContent;

/// TableView renders a static table inline in a buffer.
///
/// It uses the <https://specs.frictionlessdata.io/tabular-data-resource/>
/// specification for data interchange.
pub struct TableView {
    pub table: TabularDataResource,
    pub widths: Vec<Pixels>,
    cached_clipboard_content: ClipboardItem,
}

fn cell_content(row: &Value, field: &str) -> String {
    match row.get(field) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Array(arr)) => format!("{:?}", arr),
        Some(Value::Object(obj)) => format!("{:?}", obj),
        Some(Value::Null) | None => String::new(),
    }
}

// Declare constant for the padding multiple on the line height
const TABLE_Y_PADDING_MULTIPLE: f32 = 0.5;

impl TableView {
    pub fn new(table: &TabularDataResource, window: &mut Window, cx: &mut App) -> Self {
        let mut widths = Vec::with_capacity(table.schema.fields.len());

        let text_system = window.text_system();
        let text_style = window.text_style();
        let text_font = ThemeSettings::get_global(cx).buffer_font.clone();
        let font_size = ThemeSettings::get_global(cx).buffer_font_size(cx);
        let mut runs = [TextRun {
            len: 0,
            font: text_font,
            color: text_style.color,
            ..Default::default()
        }];

        for field in table.schema.fields.iter() {
            runs[0].len = field.name.len();
            let mut width = text_system
                .layout_line(&field.name, font_size, &runs, None)
                .width;

            let Some(data) = table.data.as_ref() else {
                widths.push(width);
                continue;
            };

            for row in data {
                let content = cell_content(row, &field.name);
                runs[0].len = content.len();
                let cell_width = window
                    .text_system()
                    .layout_line(&content, font_size, &runs, None)
                    .width;

                width = width.max(cell_width)
            }

            widths.push(width)
        }

        let cached_clipboard_content = Self::create_clipboard_content(table);

        Self {
            table: table.clone(),
            widths,
            cached_clipboard_content: ClipboardItem::new_string(cached_clipboard_content),
        }
    }

    fn create_clipboard_content(table: &TabularDataResource) -> String {
        let data = match table.data.as_ref() {
            Some(data) => data,
            None => &Vec::new(),
        };
        let schema = table.schema.clone();

        let mut markdown = format!(
            "| {} |\n",
            table
                .schema
                .fields
                .iter()
                .map(|field| field.name.clone())
                .collect::<Vec<_>>()
                .join(" | ")
        );

        markdown.push_str("|---");
        for _ in 1..table.schema.fields.len() {
            markdown.push_str("|---");
        }
        markdown.push_str("|\n");

        let body = data
            .iter()
            .map(|record: &Value| {
                let row_content = schema
                    .fields
                    .iter()
                    .map(|field| MarkdownEscaped(&cell_content(record, &field.name)).to_string())
                    .collect::<Vec<_>>();

                row_content.join(" | ")
            })
            .collect::<Vec<String>>();

        for row in body {
            markdown.push_str(&format!("| {} |\n", row));
        }

        markdown
    }

    pub fn render_row(
        &self,
        schema: &TableSchema,
        is_header: bool,
        row: &Value,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let theme = cx.theme();

        let line_height = window.line_height();

        let row_cells = schema
            .fields
            .iter()
            .zip(self.widths.iter())
            .map(|(field, width)| {
                let container = match field.field_type {
                    runtimelib::datatable::FieldType::String => div(),

                    runtimelib::datatable::FieldType::Number
                    | runtimelib::datatable::FieldType::Integer
                    | runtimelib::datatable::FieldType::Date
                    | runtimelib::datatable::FieldType::Time
                    | runtimelib::datatable::FieldType::Datetime
                    | runtimelib::datatable::FieldType::Year
                    | runtimelib::datatable::FieldType::Duration
                    | runtimelib::datatable::FieldType::Yearmonth => v_flex().items_end(),

                    _ => div(),
                };

                let value = cell_content(row, &field.name);

                let mut cell = container
                    .min_w(*width + px(22.))
                    .w(*width + px(22.))
                    .child(value)
                    .px_2()
                    .py((TABLE_Y_PADDING_MULTIPLE / 2.0) * line_height)
                    .border_color(theme.colors().border);

                if is_header {
                    cell = cell.border_1().bg(theme.colors().border_focused)
                } else {
                    cell = cell.border_1()
                }
                cell
            })
            .collect::<Vec<_>>();

        let mut total_width = px(0.);
        for width in self.widths.iter() {
            // Width fudge factor: border + 2 (heading), padding
            total_width += *width + px(22.);
        }

        h_flex()
            .w(total_width)
            .children(row_cells)
            .into_any_element()
    }
}

impl Render for TableView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let data = match &self.table.data {
            Some(data) => data,
            None => return div().into_any_element(),
        };

        let mut headings = serde_json::Map::new();
        for field in &self.table.schema.fields {
            headings.insert(field.name.clone(), Value::String(field.name.clone()));
        }
        let header = self.render_row(
            &self.table.schema,
            true,
            &Value::Object(headings),
            window,
            cx,
        );

        let body = data
            .iter()
            .map(|row| self.render_row(&self.table.schema, false, row, window, cx));

        v_flex()
            .id("table")
            .overflow_x_scroll()
            .w_full()
            .child(header)
            .children(body)
            .into_any_element()
    }
}

impl OutputContent for TableView {
    fn clipboard_content(&self, _window: &Window, _cx: &App) -> Option<ClipboardItem> {
        Some(self.cached_clipboard_content.clone())
    }

    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }
}
