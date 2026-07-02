# ui::Table component API and csv_preview data-grid rendering

# ui::Table and the csv_preview data grid

## 1. `ui::Table` public API — `/Users/user/zed/crates/ui/src/components/data_table.rs`

`Table` is a `RenderOnce` builder element (`#[derive(RegisterComponent, IntoElement)]`, line 355). Rows are `TableContents`: either an eager `Vec<TableRow<AnyElement>>`, a virtualized `UniformList`, or a `VariableRowHeightList` (lines 155–159).

### Row type helpers
```rust
// data_table.rs:27
pub type UncheckedTableRow<T> = Vec<T>;
```
`TableRow<T>` (`data_table/table_row.rs:15`) is a length-checked `Vec<T>` wrapper: `from_vec(data, expected_length)`, `try_from_vec`, `get(col)`, `expect_get(col)`, `as_slice()`, `into_vec()`, `map`, `map_cloned`, `map_ref`, `cols()`. `trait IntoTableRow<T> { fn into_table_row(self, cols: usize) -> TableRow<T> }` is implemented for `Vec<T>` (table_row.rs:131–134). The builder methods take `UncheckedTableRow` (plain `Vec`) and convert internally, panicking/asserting on column-count mismatch.

### Constructor and builder methods (all on `impl Table`, lines 373–544)
```rust
pub fn new(cols: usize) -> Self                                  // line 375; defaults: not striped, row borders+hover on, ColumnWidthConfig::auto(), pinned_cols: 0, use_ui_font: true

pub fn header(mut self, headers: UncheckedTableRow<impl IntoElement>) -> Self   // line 486
pub fn row(mut self, items: UncheckedTableRow<impl IntoElement>) -> Self        // line 495; ignored in uniform_list / variable_row_height_list modes

pub fn uniform_list(                                             // line 406
    mut self,
    id: impl Into<ElementId>,
    row_count: usize,
    render_item_fn: impl Fn(Range<usize>, &mut Window, &mut App) -> Vec<UncheckedTableRow<AnyElement>> + 'static,
) -> Self

pub fn variable_row_height_list(                                 // line 434
    mut self,
    row_count: usize,
    list_state: ListState,   // gpui::ListState, caller-owned, must be kept in sync with row_count
    render_row_fn: impl Fn(usize, &mut Window, &mut App) -> UncheckedTableRow<AnyElement> + 'static,
) -> Self

pub fn striped(mut self) -> Self                                 // line 449; alternating row bg = theme text @ 0.05 opacity
pub fn hide_row_borders(mut self) -> Self                        // line 455
pub fn width(mut self, width: impl Into<DefiniteLength>) -> Self // line 464; shorthand for width_config(ColumnWidthConfig::auto_with_table_width(width))
pub fn width_config(mut self, config: ColumnWidthConfig) -> Self // line 470
pub fn interactable(mut self, interaction_state: &Entity<TableInteractionState>) -> Self  // line 481; stores a WeakEntity; enables scrolling, scrollbars, resize handles
pub fn no_ui_font(mut self) -> Self                              // line 506
pub fn pin_cols(mut self, n: usize) -> Self                      // line 516; only supported with ColumnWidthConfig::Resizable
pub fn map_row(mut self, callback: impl Fn((usize, Stateful<Div>), &mut Window, &mut App) -> AnyElement + 'static) -> Self  // line 521
pub fn hide_row_hover(mut self) -> Self                          // line 531
pub fn empty_table_callback(mut self, callback: impl Fn(&mut Window, &mut App) -> AnyElement + 'static) -> Self  // line 537
pub fn disable_base_style(mut self) -> Self                      // line 397; drops per-cell padding/ellipsis/nowrap (keeps widths + overflow-hidden); does not affect header cells
```

### `ColumnWidthConfig` (lines 224–352)
```rust
pub enum ColumnWidthConfig {
    Static { widths: StaticColumnWidths, table_width: Option<DefiniteLength> },
    Redistributable { columns_state: Entity<RedistributableColumnsState>, table_width: Option<DefiniteLength> },  // drag redistributes fixed space, table width unchanged
    Resizable(Entity<ResizableColumnsState>),   // spreadsheet-style: drag changes absolute col width and total table width
}
pub enum StaticColumnWidths { Auto, Explicit(TableRow<DefiniteLength>) }

impl ColumnWidthConfig {
    pub fn auto() -> Self                                                        // line 251
    pub fn redistributable(columns_state: Entity<RedistributableColumnsState>) -> Self  // line 259
    pub fn auto_with_table_width(width: impl Into<DefiniteLength>) -> Self       // line 267
    pub fn explicit<T: Into<DefiniteLength>>(widths: Vec<T>) -> Self             // line 275
    pub fn widths_to_render(&self, cx: &App) -> Option<TableRow<Length>>         // line 290
    pub fn table_width(&self, window: &Window, cx: &App) -> Option<Length>       // line 316; Resizable sums column widths
    pub fn list_horizontal_sizing(&self, window: &Window, cx: &App) -> ListHorizontalSizingBehavior  // line 339
}
```

### `ResizableColumnsState` (lines 33–139) — held in an `Entity`, created by caller with `cx.new(...)`
```rust
pub struct ResizableColumnsState { initial_widths: TableRow<AbsoluteLength>, widths: TableRow<AbsoluteLength>, resize_behavior: TableRow<TableResizeBehavior> }

impl ResizableColumnsState {
    pub fn new(cols: usize, initial_widths: Vec<impl Into<AbsoluteLength>>, resize_behavior: Vec<TableResizeBehavior>) -> Self  // line 40
    pub fn cols(&self) -> usize                                                                  // line 57
    pub fn resize_behavior(&self) -> &TableRow<TableResizeBehavior>                              // line 61
    pub fn set_column_configuration(&mut self, col_idx: usize, width: impl Into<AbsoluteLength>, resize_behavior: TableResizeBehavior)  // line 95; sets both initial and current width
    pub fn reset_column_to_initial_width(&mut self, col_idx: usize)                              // line 107
    pub fn pinned_width(&self, pinned_cols: usize, rem_size: Pixels) -> Pixels                   // line 111
    pub fn scrollable_width(&self, pinned_cols: usize, rem_size: Pixels) -> Pixels               // line 118
    // pub(crate): on_drag_move / drag_to — Table wires these itself via on_drag_move::<DraggedColumn>
}
```
`TableResizeBehavior` (`/Users/user/zed/crates/ui/src/components/redistributable_columns.rs:32`):
```rust
pub enum TableResizeBehavior { None, Resizable, MinSize(f32) }   // min_size(): None → None, Resizable → Some(0.05 rems*?), MinSize(m) → Some(m); values are in rems
```
`RedistributableColumnsState::new(cols, initial_widths: Vec<impl Into<DefiniteLength>>, resize_behavior: Vec<TableResizeBehavior>)` (redistributable_columns.rs:113) is the analogous state for the `Redistributable` variant.

Double-clicking a resizable header cell resets that column to its initial width (`render_header_cell`, data_table.rs:598–608, via `HeaderResizeInfo::reset_column`, redistributable_columns.rs:84). Drag handles are rendered as absolute-positioned dividers over the table (`render_resize_handles_resizable`, data_table.rs:926); the table root registers `.on_drag_move::<DraggedColumn>` and forwards to `ResizableColumnsState::on_drag_move`, compensating for horizontal scroll offset (data_table.rs:1132–1146).

### `TableInteractionState` (lines 183–222) — an `Entity` created by caller, passed to `.interactable(&entity)`
```rust
pub struct TableInteractionState {
    pub focus_handle: FocusHandle,
    pub scroll_handle: UniformListScrollHandle,      // vertical scroll (uniform_list mode)
    pub horizontal_scroll_handle: ScrollHandle,
    pub custom_scrollbar: Option<Scrollbars>,
}
impl TableInteractionState {
    pub fn new(cx: &mut App) -> Self                                                    // line 191
    pub fn with_custom_scrollbar(mut self, custom_scrollbar: Scrollbars) -> Self        // line 200
    pub fn scroll_offset(&self) -> Point<Pixels>                                        // line 205
    pub fn set_scroll_offset(&self, offset: Point<Pixels>)                              // line 209
    pub fn listener<E: ?Sized>(this: &Entity<Self>, f: impl Fn(&mut Self, &E, &mut Window, &mut Context<Self>) + 'static) -> impl Fn(&E, &mut Window, &mut App) + 'static  // line 213
}
```
Without `.interactable(...)` the table renders no scrollbars, no resize handles, and header resize/reset is inert (render(), data_table.rs:1052–1112, 1245–1310). In variable-row-height mode the vertical scrollbar tracks the `ListState` instead of `scroll_handle` (lines 1266–1274).

Minimal static usage (from the component preview, data_table.rs:1333–1339):
```rust
Table::new(3)
    .width(px(400.))
    .header(vec!["Name", "Age", "City"])
    .row(vec!["Alice", "28", "New York"])
    .row(vec!["Bob", "32", "San Francisco"])
    .into_any_element()
```

Lower-level pieces are also public if you need custom composition: `render_table_row(row_index, items: TableRow<impl IntoElement>, table_context: TableRenderContext, window, cx) -> AnyElement` (line 611), `render_table_header(...)` (line 707), and `TableRenderContext` (line 825, with `TableRenderContext::for_column_widths(column_widths, use_ui_font)` at line 857).

## 2. How csv_preview builds the grid

### State creation — `/Users/user/zed/crates/csv_preview/src/csv_preview.rs`
Fields on `CsvPreviewView` (lines 36–50): `table_interaction_state: Entity<TableInteractionState>`, `column_widths: ColumnWidths`, `list_state: gpui::ListState`.

```rust
// csv_preview.rs:153-157 — interaction state with editor-settings-driven scrollbars
let table_interaction_state = cx.new(|cx| {
    TableInteractionState::new(cx).with_custom_scrollbar(ui::Scrollbars::for_settings::<
        editor::EditorSettingsScrollbarProxy,
    >())
});

// csv_preview.rs:316-333 — wrapper holding the resizable-columns entity
pub(crate) struct ColumnWidths { pub widths: Entity<ResizableColumnsState> }
impl ColumnWidths {
    pub(crate) fn new(cx: &mut Context<CsvPreviewView>, cols: usize) -> Self {
        Self { widths: cx.new(|_cx| ResizableColumnsState::new(
            cols,
            vec![AbsoluteLength::Pixels(px(150.)); cols],
            vec![ui::TableResizeBehavior::Resizable; cols],
        )) }
    }
}

// csv_preview.rs:182-183 — variable-height list state; measure_all() so total height is known
list_state: gpui::ListState::new(contents.rows.len(), ListAlignment::Top, px(1.)).measure_all(),
```
When the CSV column count changes, `sync_column_widths` (csv_preview.rs:60–82) rebuilds the state in place: cols = data cols + 1 (row-identifier column), width 150px each, column 0 gets the computed line-number width with `TableResizeBehavior::None`; if the count is unchanged it only calls `state.set_column_configuration(0, ...)`. After filtering/sorting changes row count, `apply_filter_sort` (lines 204–213) replaces `self.list_state` with a fresh `ListState::new(visible_rows, ListAlignment::Top, px(100.)).measure_all()`.

### Render entry — `/Users/user/zed/crates/csv_preview/src/renderer/preview_view.rs:31`
`impl Render for CsvPreviewView` calls `self.create_table(&self.column_widths.widths, cx)` inside a `v_flex()`.

### Table construction — `/Users/user/zed/crates/csv_preview/src/renderer/render_table.rs`
```rust
// render_table.rs:52-103 (inside create_table_inner; cols = current_widths.read(cx).cols())
Table::new(cols)
    .interactable(&self.table_interaction_state)
    .striped()
    .width_config(ColumnWidthConfig::Resizable(current_widths.clone()))
    .header(headers)                 // Vec<AnyElement>: row-id header + per-column header-with-sort-button
    .disable_base_style()
    .pin_cols(1)                     // pins the row-identifier column during horizontal scroll
    .map(|table| {
        let row_identifier_text_color = cx.theme().colors().editor_line_number;
        match self.settings.rendering_with {
            RowRenderMechanism::VariableList => {
                table.variable_row_height_list(row_count, self.list_state.clone(), {
                    cx.processor(move |this, display_row: usize, _window, cx| {
                        let display_row = DisplayRow(display_row);
                        Self::render_single_table_row(this, cols, display_row, row_identifier_text_color, cx)
                            .unwrap_or_else(|| panic!("Expected to render a table row"))
                    })
                })
            }
            RowRenderMechanism::UniformList => {
                table.uniform_list("csv-table", row_count, {
                    cx.processor(move |this, range: Range<usize>, _window, cx| {
                        range.filter_map(|display_index| {
                            Self::render_single_table_row(this, cols, DisplayRow(display_index), row_identifier_text_color, cx)
                        }).collect()
                    })
                })
            }
        }
    })
    .into_any_element()
```
Note `cx.processor(...)`: adapts an entity-method closure to the `Fn(..., &mut Window, &mut App)` shape the Table callbacks require, giving `this: &mut CsvPreviewView` inside.

Cell rendering (`render_single_table_row`, render_table.rs:109–165) returns `Option<UncheckedTableRow<AnyElement>>` (i.e. `Vec<AnyElement>`, one per column): first `create_row_identifier_cell(...)`, then per data column a wrapper `div().size_full().text_ui(cx)` containing `div().size_full().whitespace_nowrap().text_ellipsis().child(CsvPreviewView::create_selectable_cell(display_cell_id, cell_content, this.settings.vertical_alignment, cx))`. Rows are looked up through the sort/filter mapping: `this.engine.d2d_mapping().get_data_row(display_row)?` then `this.engine.contents.get_row(data_row)?`; returning `None` skips the row (hence `filter_map` in uniform mode).

## 3. Sorting indicators and header clicks

The Table component itself has no sort support — header cell clicks are only used for double-click column-width reset (data_table.rs:598–608). csv_preview implements sorting entirely with its own header widgets.

### `/Users/user/zed/crates/csv_preview/src/renderer/table_header.rs`
`create_header_element_with_sort_button(header_text, cx, col_idx: AnyColumn) -> AnyElement` (line 12) builds `h_flex().justify_between()...child(header_text).child(self.create_sort_button(cx, col_idx))`.

`create_sort_button` (lines 29–89): a `ui::Button` whose label is a unicode indicator based on `self.engine.applied_sorting: Option<AppliedSorting>` — `"↓"` for `SortDirection::Asc` on the sorted column, `"↑"` for `Desc`, `"↕"` otherwise; style `ButtonStyle::Filled` when this column is sorted, else `Subtle`; `ButtonSize::Compact`; tooltip describes the next state. Click handler cycles None → Asc → Desc → None:
```rust
.on_click(cx.listener(move |this, _event, _window, cx| {
    let new_sorting = match this.engine.applied_sorting {
        Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
            SortDirection::Asc => Some(AppliedSorting { col_idx, direction: SortDirection::Desc }),
            SortDirection::Desc => None,
        },
        _ => Some(AppliedSorting { col_idx, direction: SortDirection::Asc }),
    };
    this.engine.applied_sorting = new_sorting;
    this.apply_sort();
    cx.notify();
}))
```

### `/Users/user/zed/crates/csv_preview/src/table_data_engine/sorting_by_column.rs`
```rust
pub enum SortDirection { Asc, Desc }                                     // line 6
pub struct AppliedSorting { pub col_idx: AnyColumn, pub direction: SortDirection }  // line 13 (Copy)
pub fn sort_data_rows(content_rows: &[TableRow<TableCell>], mut data_row_ids: Vec<DataRow>, sorting: AppliedSorting) -> Vec<DataRow>  // line 20; string compare on display values, reversed for Desc
```
Sorting never mutates the parsed rows — it reorders a display→data row-index mapping. `CsvPreviewView::apply_sort` (csv_preview.rs:197–201) calls `self.engine.apply_sort()`; `apply_filter_sort` (csv_preview.rs:204–213) recomputes `engine.calculate_d2d_mapping()` and rebuilds `list_state` with the new visible row count. The render path then resolves each `DisplayRow` through `engine.d2d_mapping().get_data_row(...)` (render_table.rs:117).