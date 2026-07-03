# Stage 3 grid render: current table_data_view grid, ui::Table internals, styling recipes

Companion to `table.md` (generic `ui::Table` API). This file covers what stage 3
(SQL-first table page redesign) actually touches: the existing grid render path in
`crates/database_ui/src/table_data_view.rs`, the `ui::Table` styling internals the
polish work depends on, and concrete snippets for zebra / hover / right-align /
ellipsis / auto column widths. Line numbers verified on branch `database-viewer`,
2026-07-03.

## 1. Current grid mechanism (database_ui)

The data grid is `ui::Table` (i.e. `crates/ui/src/components/data_table.rs`) in
`uniform_list` mode with `ColumnWidthConfig::Resizable` — NOT hand-rolled divs.

Relevant `TableDataView` state (`table_data_view.rs`):

```rust
// table_data_view.rs:45
const COLUMN_WIDTH: f32 = 180.;

// table_data_view.rs:258,261
interaction: Entity<TableInteractionState>,                 // created at :290: cx.new(|cx| TableInteractionState::new(cx))
column_widths: Option<Entity<ResizableColumnsState>>,       // None until first page arrives
```

`set_column_widths(&mut self, cols: usize, cx)` (table_data_view.rs:1094–1113):
recreates the `ResizableColumnsState` entity **only when the column count
changes** (`widths.read(cx).cols() == cols` short-circuits), with
`vec![AbsoluteLength::Pixels(px(COLUMN_WIDTH)); cols]` and
`vec![TableResizeBehavior::Resizable; cols]`. Recreating the entity throws away
the user's manual resizes — that is why it is guarded. Stage 3 auto-width must
respect the same rule: set initial widths once per page shape, don't rebuild on
every render.

Table construction — `render_data(&mut self, cx) -> AnyElement`
(table_data_view.rs:1115–1185):

```rust
Table::new(column_count)
    .interactable(&self.interaction)             // scrollbars + resize handles; without it resize is inert
    .striped()                                   // zebra, see §3
    .width_config(ColumnWidthConfig::Resizable(widths))
    .header(headers)                             // Vec<AnyElement> from render_header(index, column, cx)
    .uniform_list("db-rows", total_row_count, cx.processor(move |this, range: Range<usize>, window, cx| {
        // returns Vec<Vec<AnyElement>>; each inner Vec MUST have exactly column_count items
        // (TableRow::from_vec asserts length — mismatch panics)
        ...this.render_data_cell(row_index, col, value, window, cx)...
        ...this.render_insert_cell(insert_index, col, window, cx)...   // pending inserts appended after page rows
    }))
    .map_row(cx.processor(move |this, (row_index, row), _window, cx| {
        this.map_data_row(row_index, row, page_row_count, created_background, deleted_background, cx)
    }))
    .into_any_element()
```

`cx.processor(...)` adapts an entity-method closure to the `Fn(..., &mut Window,
&mut App)` shape Table callbacks need (`this: &mut TableDataView` inside).

`map_data_row` (table_data_view.rs:1192–1262) is the per-row hook and already
demonstrates the row-hover-group pattern stage 3 needs:

```rust
let group_name = SharedString::from(format!("db-row-{row_index}"));
// hover-revealed delete button:
h_flex().absolute().right_1().top_0().bottom_0().items_center()
    .visible_on_hover(group_name.clone())        // ui::VisibleOnHover, see §4
    .child(IconButton::new(...))
// then:
row.group(group_name)                            // row: gpui::Stateful<Div>, already bg/hover-styled by Table
    .relative()
    .when(is_insert, |row| row.bg(created_background))
    .when(marked_deleted, |row| row.bg(deleted_background).line_through())
    .children(delete_button)
    .into_any_element()
```

Cell render — `render_data_cell` (table_data_view.rs:1268–1347):

```rust
let mut cell = div().w_full();
if modified { cell = cell.bg(modified_cell_background(cx)).rounded_sm().px_1(); }
let cell = match display {
    Some(value) => cell.whitespace_nowrap().text_ellipsis().child(value),
    None => cell.child(Label::new("NULL").color(Color::Muted).italic()),   // NULL rendering already exists
};
// if editable: wrapped in div().id(("db-cell-{col}", row)).cursor_pointer().on_click(double-click → begin_edit_cell)
```

Also: `render_cell_editor` (:1351, inline InputField + "∅ NULL" button),
`render_insert_cell` (:1374–1435, same shape for pending-insert rows),
background tints `modified/created/deleted_cell_background` (:2065–2080, e.g.
`cx.theme().colors().version_control_modified.opacity(0.2)`).

Header cell — `render_header(&self, index, column, cx)` (table_data_view.rs:1437–1478):

```rust
h_flex().justify_between().items_center().w_full()
    .child(Label::new(column.clone()))
    .child(Button::new(("db-sort", index), indicator)     // "↑"/"↓"/"↕" unicode label today
        .size(ButtonSize::Compact)
        .style(if sorted.is_some() { ButtonStyle::Filled } else { ButtonStyle::Subtle })
        .on_click(cx.listener(move |this, _, _, cx| this.toggle_sort(&column, cx))))
```

Sorting is entirely view-owned (`self.spec.sort`, `toggle_sort`); `ui::Table` has
no sort support. `render_structure` (:1480) uses a second, eager `Table::new(6)
.striped().header(...)` with `.row(...)` calls — untouched by stage 3.

Page layout: `impl Render for TableDataView::render` (table_data_view.rs:1966–2029)
stacks header-bar / edit_toolbar / filter_bar / body(`render_data`) /
footer(`render_footer` :1780) in a `v_flex()` with
`.bg(cx.theme().colors().editor_background)`.

## 2. ui::Table internals that matter (crates/ui/src/components/data_table.rs)

Full builder API is in `table.md`. What stage 3 styling relies on:

Cell chrome (every non-header cell goes through this):

```rust
// data_table.rs:552-560
fn base_cell_style(width: Option<Length>) -> Div {
    div().px_1p5()
        .when_some(width, |this, width| this.w(width))
        .when(width.is_none(), |this| this.flex_1())
        .whitespace_nowrap().text_ellipsis().overflow_hidden()
}
// data_table.rs:566-579  render_cell(): base_cell_style_text(width, use_ui_font, cx).px_1().py_0p5().child(cell)
//   (.px_1() overrides px_1p5 — effective horizontal cell padding is px_1 = 4px per side)
//   use_ui_font (default true) applies .text_ui(cx); Table::no_ui_font() (:506) disables it
//   Table::disable_base_style() (:397) drops padding/ellipsis/nowrap entirely (csv_preview does this and re-implements)
```

So **truncation-with-… already happens at the Table level** for plain text
cells; `render_data_cell`'s own `.whitespace_nowrap().text_ellipsis()` is
belt-and-braces on the inner div.

Row chrome — `render_table_row` (data_table.rs:611–705):

```rust
let bg = if row_index % 2 == 1 && is_striped { Some(cx.theme().colors().text.opacity(0.05)) } else { None };  // :620-624
row = div().flex().flex_row().id(("table_row", row_index)).size_full()
    .when_some(bg, |row, bg| row.bg(bg))
    .when(table_context.show_row_hover, |row| {                                   // :639-641, default ON
        row.hover(|s| s.bg(cx.theme().colors().element_hover.opacity(0.6)))
    })
    .when(!is_striped && table_context.show_row_borders, |row| { row.border_b_1()... });  // :642-646
// :698-702 — map_row runs LAST, receiving (row_index, Stateful<Div>) with all of the above applied
```

Traps:
- Stripe color is hard-coded (`text @ 0.05` on odd rows); not configurable.
  Custom zebra ⇒ skip `.striped()` and set bg per-row in `map_row`.
- `.striped()` disables row bottom borders (the `!is_striped` guard).
- A `.bg(x)` applied in `map_row` replaces the stripe bg for that row (this is
  how insert/delete tints win today) but the `.hover()` refinement still
  overlays on hover.
- `hide_row_hover()` (:531) exists for taking over hover styling manually.

Header — `render_table_header` (data_table.rs:707–822): a flex row with
`.border_b_1().border_color(cx.theme().colors().border)` (:733) and **no
background**. `render_header_cell` (:581–609) = `base_cell_style_text(...)
.px_1().py_0p5().child(header).id(...)`, plus an `on_click` that resets the
column width on double-click when resizable (:598–608). For the spec's
"distinct header bg": either give the header child element
`.size_full().bg(...)` (leaves the 4px cell padding unpainted), or add a bg to
the header row upstream in `data_table.rs` — this is a fork, a one-line
`ui::Table` tweak (e.g. a `.header_bg(hsla)` builder or unconditional
`cx.theme().colors().title_bar_background`) is acceptable and cleaner.
Child elements inside a header cell handle their own clicks fine (today's sort
Button, csv_preview's sort button) — single clicks on children don't trigger
the header's double-click reset.

## 3. ResizableColumnsState — full public API (data_table.rs:33–139)

```rust
pub struct ResizableColumnsState {          // all fields PRIVATE
    initial_widths: TableRow<AbsoluteLength>,
    widths: TableRow<AbsoluteLength>,
    resize_behavior: TableRow<TableResizeBehavior>,
}
pub fn new(cols: usize, initial_widths: Vec<impl Into<AbsoluteLength>>,
           resize_behavior: Vec<TableResizeBehavior>) -> Self          // :40  — the ONLY way to set all widths at once
pub fn cols(&self) -> usize                                            // :57
pub fn resize_behavior(&self) -> &TableRow<TableResizeBehavior>        // :61
pub fn set_column_configuration(&mut self, col_idx: usize,
    width: impl Into<AbsoluteLength>, resize_behavior: TableResizeBehavior)  // :95 — sets BOTH initial and current width (clobbers a user resize of that column)
pub fn reset_column_to_initial_width(&mut self, col_idx: usize)        // :107
pub fn pinned_width(&self, pinned_cols: usize, rem_size: Pixels) -> Pixels      // :111 — sum only
pub fn scrollable_width(&self, pinned_cols: usize, rem_size: Pixels) -> Pixels  // :118 — sum only
```

- Programmatic initial widths: pass them to `new(...)` (what stage 3 auto-width
  wants: measure first page → `cx.new(|_| ResizableColumnsState::new(cols, measured, behaviors))`).
  Double-click on a header then resets to *these* measured widths — nice.
- **Trap: there is no public getter for current per-column widths** (only the
  sums). If stage 3 needs to preserve user resizes across a column-count-stable
  reload with new measured widths, `set_column_configuration` per column is the
  tool (csv_preview.rs:60–82 uses it for column 0) — but it overwrites the
  user's current width for that column. Preserving user resizes exactly would
  need a small upstream accessor (fork — fine to add, e.g. `pub fn widths(&self)
  -> &TableRow<AbsoluteLength>`).
- `TableResizeBehavior` (`redistributable_columns.rs:32`): `None | Resizable |
  MinSize(f32 /* rems */)`.
- Drag plumbing is internal: `Table::render` wires
  `on_drag_move::<DraggedColumn>` itself when `.interactable(...)` is set.

## 4. Styling recipes (verified APIs)

**Zebra** — already on: `.striped()` (odd rows `text @ 0.05`). Custom colors:

```rust
// omit .striped(); in map_row:
row.when(row_index % 2 == 1, |row| row.bg(cx.theme().colors().element_background))
```

**Row hover** — already on by default (`element_hover.opacity(0.6)`); to
customize: `.hide_row_hover()` on the Table, then in `map_row`:
`row.hover(|s| s.bg(my_color))`. Hover-revealed controls use the group pattern
(exact API, `crates/ui/src/traits/visible_on_hover.rs:5–17`):

```rust
trait VisibleOnHover { fn visible_on_hover(self, group_name: impl Into<SharedString>) -> Self; }
// impl for all InteractiveElement + Styled: self.invisible().group_hover(group_name, |style| style.visible())
// container: .group(name)  →  child: .visible_on_hover(name)
// Implemented specifically on IconButton (icon_button.rs:215), ButtonLike (:701), Disclosure (:86) too.
```

Header funnel (per-column group so only the hovered header shows its funnel):

```rust
let group = SharedString::from(format!("db-header-{index}"));
h_flex().group(group.clone()).justify_between().items_center().w_full()
    .child(Label::new(column))
    .child(Icon::new(IconName::ArrowUp))                       // sort indicator when sorted; ArrowDown for desc
    .child(IconButton::new(("db-filter", index as u64), IconName::Filter)   // IconName::Filter / ListFilter both exist (icons.rs:139/:181)
        .visible_on_hover(group)
        .when(has_filter, |b| b.toggle_state(true)))           // keep permanently visible when filtered: skip visible_on_hover instead
```

**Right-align** (numeric columns) — `gpui::Styled` (styled.rs:123/128/133):

```rust
fn text_left(self) / text_center(self) / text_right(self)     // sets TextAlign on the element's text style, inherited by child text
// in render_data_cell: let mut cell = div().w_full(); if numeric { cell = cell.text_right(); }
```

**Ellipsis** — `gpui::Styled`: `.text_ellipsis()` (styled.rs:89, sets
`TextOverflow::Truncate("…")`; needs `overflow_hidden` + `whitespace_nowrap` on
the same element), or the one-shot `.truncate()` (styled.rs:139–141 ≡
`overflow_hidden().whitespace_nowrap().text_ellipsis()`). Variants
`text_ellipsis_start/middle` exist (:97/:105). Already applied by both the
Table cell chrome and render_data_cell.

**Monospace values** — grid cells currently render in UI font
(`use_ui_font: true` → `.text_ui(cx)`). For buffer font: call `.no_ui_font()`
on the Table and style cells with `crates/ui/src/styles/typography.rs`:
`.font_buffer(cx)` (:13, family only) + `.text_buffer(cx)` (:83, size only) —
or keep ui font size and only swap family.

## 5. Measuring text width for auto column widths

GPUI `TextSystem` (`crates/gpui/src/text_system.rs`) — available as
`window.text_system()` (also `cx.text_system()` on `App`):

```rust
pub fn resolve_font(&self, font: &Font) -> FontId
pub fn em_width(&self, font_id: FontId, font_size: Pixels) -> Result<Pixels>     // :226, width of 'm'
pub fn em_advance(&self, font_id: FontId, font_size: Pixels) -> Result<Pixels>   // :233, advance of 'm'
pub fn ch_width(&self, font_id: FontId, font_size: Pixels) -> Result<Pixels>     // :239, width of '0'
pub fn shape_line(&self, text: SharedString, font_size: Pixels,
                  runs: &[TextRun], force_width: Option<Pixels>) -> ShapedLine   // :397; panics on '\n' in text
// ShapedLine derefs to LineLayout → `.width: Pixels` (text_system/line.rs:43, line_layout.rs:20)
// TextRun { len /* utf8 bytes */, font, color, background_color: None, underline: None, strikethrough: None }  // :987-1000
```

Recommended cheap approach (exact for a monospace buffer font, since every
ASCII glyph advance equals `em_advance`; the editor uses the same trick,
`crates/editor/src/element.rs:7961–7966`):

```rust
let font = theme::theme_settings(cx).buffer_font(cx).clone();   // theme_settings_provider.rs:14; ui font: .ui_font(cx)
let font_size = TextSize::default().rems(cx).to_pixels(window.rem_size());  // match the cell's actual text size
let font_id = window.text_system().resolve_font(&font);
let advance = window.text_system().em_advance(font_id, font_size)?;   // Result — don't unwrap, fall back to px(8.)
let width = px(max_char_count as f32) * f32::from(advance) /* i.e. advance * max_char_count */;
let column_width = (f32::from(width) + 2. * 4. /* px_1 cell padding */ + 12. /* slack */).clamp(60., 480.);
```

`max_char_count` = max of `chars().count()` over the header name and the first
page's values in that column (`NULL` counts as 4). For exact non-monospace
measurement use `shape_line(value.into(), font_size, &[TextRun{ len: value.len(), font, color: Hsla::default(), background_color: None, underline: None, strikethrough: None }], None).width` — costlier;
avoid per-cell shaping of 1000 rows × N cols unless capped (e.g. sample first
100 rows). csv_preview never solved this (hard-coded `char_width_px = 9.0`,
`csv_preview/src/renderer/row_identifiers.rs:86`).

Measurement needs a `Window` (for `rem_size` and text system): do it in the
render/update path that has `window: &mut Window` — e.g. where the page result
lands (`update_in`) or lazily in the first `render_data` after a page swap,
before creating the `ResizableColumnsState`.

## 6. Known traps, condensed

1. `uniform_list` inner Vec length must equal `Table::new(cols)` — asserted, panics.
2. Recreating the `ResizableColumnsState` entity resets user resizes; current widths are not publicly readable (add a fork accessor if needed).
3. `set_column_configuration` clobbers both current and initial width of that column.
4. `.striped()` ⇒ no row borders and non-configurable stripe color; custom zebra goes in `map_row` without `.striped()`.
5. `map_row` bg overrides stripe/deleted tint order matters: last `.bg()` wins; hover refinement still applies on top.
6. Header row has no background; header cell padding (px_1) is outside the child element you control.
7. Double-click on a resizable header cell resets its width — coexists with single-click child buttons, but a sort-cycling single-click handler on the header cell itself will also receive the two clicks of a double-click (check `event.click_count()`).
8. `render_data_cell`/`render_insert_cell` return editor UI for the cell being edited — auto-width/right-align changes must not disturb the `h_flex` editor row (`render_cell_editor`).
9. Table paddings: effective cell padding is `px_1().py_0p5()` (render_cell overrides base `px_1p5`).
10. `em_advance`/`em_width` return `Result` — propagate or fall back, don't `unwrap()` (repo rule).
