# Context menus and element-anchored popovers in Zed (GPUI) — API reference

Scope: stage 3 of the table page redesign (`docs/superpowers/specs/2026-07-03-table-page-redesign-design.md`) needs
(a) right-click context menus on grid cells/headers, (b) a filter-edit popover with an operator picker + text input +
Apply button anchored to a header icon or chip, (c) a "View value" popup with long read-only text. This file covers all
three patterns with exact signatures and in-repo references. All paths relative to `/Users/user/zed`.

## 1. Right-click → `ContextMenu` at cursor position (canonical pattern)

This crate already ships one working instance: `crates/database_ui/src/database_panel.rs` (connection/database tree
rows). The upstream reference is `crates/project_panel/src/project_panel.rs`. Four pieces:

### 1.1 State field on the view

`project_panel.rs:150`, `database_panel.rs:96`:
```rust
context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
```
Initialize to `None`. The `Subscription` keeps the `DismissEvent` handler alive; dropping the tuple drops both.

### 1.2 Catching the right click

Right click arrives as "secondary mouse down". On any interactive element (`div()`, `ListItem`, …):
`database_panel.rs:633-643` (also `project_panel.rs:6171-6185`):
```rust
.on_secondary_mouse_down(cx.listener(
    move |this, event: &MouseDownEvent, window, cx| {
        cx.stop_propagation(); // required: otherwise outer catch-all handlers also deploy
        this.deploy_cell_context_menu(event.position, row, column.clone(), window, cx);
    },
))
```
`event.position: Point<Pixels>` is window-relative — pass it straight to `anchored().position(...)`.

### 1.3 Building, focusing, subscribing

`ContextMenu::build` — `crates/ui/src/components/context_menu.rs:329`:
```rust
pub fn build(
    window: &mut Window,
    cx: &mut App,
    f: impl FnOnce(Self, &mut Window, &mut Context<Self>) -> Self,
) -> Entity<Self>
```
Deploy method, verbatim shape from `database_panel.rs:371-395`:
```rust
fn deploy_cell_context_menu(
    &mut self,
    position: Point<Pixels>,
    row_key: RowKey,
    column: String,
    window: &mut Window,
    cx: &mut Context<Self>,
) {
    let context_menu = ContextMenu::build(window, cx, |menu, _, _| {
        menu.context(self.focus_handle.clone()) // see trap 7.1
            .action("Filter by This Value", Box::new(FilterByCellValue))
            .action("View Value", Box::new(ViewCellValue))
            .separator()
            .action("Set to NULL", Box::new(SetCellNull))
    });
    window.focus(&context_menu.focus_handle(cx), cx);
    let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
        this.context_menu.take();
        cx.notify();
    });
    self.context_menu = Some((context_menu, position, subscription));
    // Stash WHAT the menu targets in fields the action handlers read later,
    // like database_panel.rs:392 (`self.menu_target = Some(connection_name)`).
    self.menu_target_cell = Some((row_key, column));
    cx.notify();
}
```
Actions fired by menu entries are plain GPUI actions; the view handles them with `.on_action(cx.listener(...))` on its
root element (see `database_panel.rs:420-430` `refresh_connection` reading `self.menu_target`). Alternative if you
don't want actions: `menu.entry(label, None, move |window, cx| ...)` (`context_menu.rs:522`) with a
`WeakEntity` captured closure — exactly what `table_data_view.rs:1710-1723` already does for the filter dropdowns.

### 1.4 Rendering — `deferred(anchored())` at the end of the root element

`database_panel.rs:851-859` (identical to `project_panel.rs:7394-7402`):
```rust
.children(self.context_menu.as_ref().map(|(menu, position, _)| {
    deferred(
        anchored()
            .position(*position)
            .anchor(gpui::Anchor::TopLeft)
            .child(menu.clone()),
    )
    .with_priority(3)
}))
```
- `deferred(child)` — `crates/gpui/src/elements/deferred.rs:7`; `.with_priority(usize)` — `deferred.rs:25` (higher
  paints later/on top; project/database panels use 3, `PopoverMenu` internally uses 1).
- `anchored()` — `crates/gpui/src/elements/anchored.rs:27`; `.anchor(Anchor)` `:40`, `.position(Point<Pixels>)` `:47`,
  `.offset(Point<Pixels>)` `:54`, `.snap_to_window_with_margin(impl Into<Edges<Pixels>>)` `:74` (add this so menus near
  the window edge get pushed inside; the panel examples omit it, `PopoverMenu` uses `px(8.)`).
- No manual `.occlude()` needed: `ContextMenu`'s own render occludes itself (`context_menu.rs:2152,2178`).

Dismissal is built into `ContextMenu`: blur → cancel (`context_menu.rs:273-296`), `on_mouse_down_out` → cancel
(`context_menu.rs:2218-2239`), Escape → `menu::Cancel` (its `key_context` is `"menu"`, `context_menu.rs:309`). Each of
these `cx.emit(DismissEvent)` which your subscription turns into `self.context_menu.take()`.

### 1.5 ContextMenu builder cheat sheet (`crates/ui/src/components/context_menu.rs`)

| method | line | signature (abridged) |
|---|---|---|
| `context` | :479 | `fn context(self, focus: FocusHandle) -> Self` — dispatch actions on this handle |
| `header` | :484 | `fn header(self, title: impl Into<SharedString>) -> Self` |
| `separator` | :503 | `fn separator(self) -> Self` |
| `entry` | :522 | `fn entry(self, label, Option<Box<dyn Action>>, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self` |
| `toggleable_entry` | :612 | checkbox-style entry |
| `custom_row` | :642 | `fn custom_row(self, renderer: impl Fn(&mut Window, &mut App) -> AnyElement + 'static) -> Self` (non-clickable) |
| `custom_entry` | :655 | same renderer + click handler |
| `action` | :700 | `fn action(self, label, Box<dyn Action>) -> Self` |
| `action_disabled_when` | :751 | `fn action_disabled_when(self, disabled: bool, label, Box<dyn Action>) -> Self` |
| `fixed_width` | :882 | `fn fixed_width(self, DefiniteLength) -> Self` |
| `build_persistent` | :341 | menu that stays open across confirms (`keep_open_on_confirm`) |

`impl EventEmitter<DismissEvent> for ContextMenu` — `context_menu.rs:262`.

### 1.6 Alternative: `RightClickMenu` element (no state field needed)

`crates/ui/src/components/right_click_menu.rs:75`:
```rust
pub fn right_click_menu<M: ManagedView>(id: impl Into<ElementId>) -> RightClickMenu<M>
// .trigger(|is_menu_active: bool, window, cx| element)   :32
// .menu(|window, cx| Entity<M>)                          :19
// .anchor(Anchor) :45   .attach(Anchor) :51
```
Wrap the cell element as `.trigger(...)`; it captures `MouseButton::Right` itself, shows the menu at
`window.mouse_position()`, does the focus/restore dance and clears on `DismissEvent` (`right_click_menu.rs:246-296`).
Used by tab bars (`crates/workspace/src/pane.rs`). Downside: menu construction lives in render closures, awkward when
the menu needs per-cell data and the handler needs `&mut self` — for a data grid the § 1.1-1.4 pattern is the better
fit (one menu for N×M cells instead of N×M element states).

## 2. `PopoverMenu` attached to a trigger button (already used in this crate)

`crates/ui/src/components/popover_menu.rs:150`:
```rust
PopoverMenu::<M>::new(id)              // M: ManagedView, usually ContextMenu
    .trigger(button)                   // :182 — any Clickable + Toggleable; click toggles menu
    .trigger_with_tooltip(btn, tt)     // :198 — hides tooltip while open
    .menu(|window, cx| Option<Entity<M>>)  // :169 — called on every open
    .anchor(Anchor::TopLeft)           // :222 — menu corner anchored to trigger
    .attach(Anchor::BottomLeft)        // :228 — trigger corner to attach to (auto-derived otherwise, :245)
    .offset(point(px(0.), px(-2.)))    // :234
    .with_handle(handle)               // :177 — programmatic control
    .full_width(true)                  // :164
```
In-crate example: `table_data_view.rs:1700-1725` (filter column dropdown = `PopoverMenu` + `ContextMenu::build` with
`menu.entry(...)` closures updating the view via `cx.weak_entity()`).

What `PopoverMenu` does for you (so you don't re-implement it):
- wraps the menu in `deferred(anchored().snap_to_window_with_margin(px(8.)).anchor(..).offset(..).child(div().occlude().child(menu))).with_priority(1)` — `popover_menu.rs:376-386`;
- on open, remembers `window.focused(cx)` and focuses the menu's `focus_handle` **two frames later** (`show_menu`,
  `popover_menu.rs:274-317`) — restore-on-dismiss included;
- clicking the trigger while open dismisses instead of re-opening (`popover_menu.rs:483-497`).

`PopoverMenuHandle<M>` (`popover_menu.rs:41`): `show(window, cx)` :62, `hide(cx)` :74, `toggle(window, cx)` :82,
`is_deployed()` :92, `is_focused(window, cx)` :99. Store it as a field and pass via `.with_handle()` — this is how a
context-menu action ("Filter by this value…") can open the header filter popover programmatically.

## 3. Popover with arbitrary content (filter editor, "View value")

**Recommendation:** implement a small custom view (a `ManagedView`) and hand it to `PopoverMenu` — the same thing
git_panel does with non-menu content: `git_panel.rs:7587-7593` (`RepositorySelector`), `:7623-7637`
(`branch_picker::popover`, `crates/git_ui/src/branch_picker.rs:81`). Everything in § 2 (anchoring, occlusion, focus,
outside-click-on-trigger, focus restore) comes for free. Do **not** hand-roll `deferred(anchored())` + own state field
for these; that is only needed for cursor-positioned menus (§ 1) or when there is no clickable trigger.

`ManagedView` is a blanket trait — `crates/gpui/src/window.rs:570-572`:
```rust
pub trait ManagedView: Focusable + EventEmitter<DismissEvent> + Render {}
```
There is also `ui::Popover` (`crates/ui/src/components/popover.rs:38`) — it is **only a styled container**
(`elevation_2` + padding), no behavior; use it (or plain `v_flex().elevation_2(cx)`) inside your view's render.
`StyledExt::elevation_2` — `crates/ui/src/traits/styled_ext.rs:62`.

### 3.1 Full snippet: filter popover (operator + input + Apply) on a column-header icon

```rust
use gpui::{App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, WeakEntity, Window};
use ui::{prelude::*, PopoverMenu, IconButton};
use ui_input::InputField;

pub struct FilterPopover {
    column: String,
    op: FilterOp,
    value_field: Entity<InputField>,
    table_view: WeakEntity<TableDataView>,
}

impl FilterPopover {
    pub fn new(
        column: String,
        existing: Option<&Filter>, // pre-fill when editing an existing chip
        table_view: WeakEntity<TableDataView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let value_field = cx.new(|cx| {
            let field = InputField::new(window, cx, "Value");
            if let Some(filter) = existing {
                field.set_text(&filter.value, window, cx);
            }
            field
        });
        Self { column, op: existing.map_or(FilterOp::Eq, |f| f.op), value_field, table_view }
    }

    fn apply(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let value = self.value_field.read(cx).text(cx);
        self.table_view
            .update(cx, |table, cx| table.set_filter(self.column.clone(), self.op, value, cx))
            .log_err();
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for FilterPopover {}

impl Focusable for FilterPopover {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        // PopoverMenu focuses this handle when the popover opens (popover_menu.rs:305-310).
        // Delegating to the input == "focus lands in the text field", no manual focus call.
        self.value_field.focus_handle(cx)
    }
}

impl Render for FilterPopover {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DatabaseFilterPopover")
            .occlude() // PopoverMenu also occludes its wrapper; keep this for safety in tests/reuse
            .elevation_2(cx)
            .w_72()
            .p_2()
            .gap_2()
            // Enter / Escape arrive as menu::Confirm / menu::Cancel (see § 5)
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| this.apply(window, cx)))
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
            .on_mouse_down_out(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))) // click-outside closes
            .child(
                // Operator row: prefer toggle buttons over a nested PopoverMenu — see trap 7.4
                h_flex().gap_1().children(all_filter_ops().map(|op| {
                    Button::new(("filter-op", op as usize), filter_op_label(op))
                        .size(ButtonSize::Compact)
                        .toggle_state(self.op == op)
                        .on_click(cx.listener(move |this, _, _, cx| { this.op = op; cx.notify(); }))
                })),
            )
            .child(self.value_field.clone())
            .child(
                h_flex().justify_end().child(
                    Button::new("filter-apply", "Apply")
                        .style(ButtonStyle::Filled)
                        .size(ButtonSize::Compact)
                        .on_click(cx.listener(|this, _, window, cx| this.apply(window, cx))),
                ),
            )
    }
}

// In the column-header render (ids must be per-column unique — tuple ElementIds work):
PopoverMenu::new(("db-col-filter", column_index))
    .trigger(
        IconButton::new(("db-col-filter-icon", column_index), IconName::Filter)
            .icon_size(IconSize::XSmall),
    )
    .anchor(Anchor::TopLeft) // menu's top-left attaches below the icon (attach auto-derived)
    .menu({
        let table_view = cx.weak_entity();
        let column = column.clone();
        move |window, cx| {
            let existing = /* look up current Filter for `column` via table_view */ None;
            Some(cx.new(|cx| FilterPopover::new(column.clone(), existing, table_view.clone(), window, cx)))
        }
    })
```
Editing an existing chip: same `FilterPopover`, trigger is the chip itself (chips are `Button`s → they satisfy
`PopoverTrigger`, `popover_menu.rs:12-14`).

### 3.2 "View value" popup (long read-only text)

Same mechanism, trivially simpler view — no input, no Confirm:
```rust
impl Render for ValuePopover {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .occlude()
            .elevation_2(cx)
            .max_w_96()
            .max_h_80()
            .p_2()
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
            .on_mouse_down_out(cx.listener(|_, _, _, cx| cx.emit(DismissEvent)))
            .child(
                div().id("db-value-scroll").overflow_y_scroll().max_h_72()
                    .child(Label::new(self.value.clone()).size(LabelSize::Small)),
            )
    }
}
```
Give it `focus_handle: FocusHandle` (from `cx.focus_handle()`) + `impl Focusable` returning it. If opened from a
context-menu action (no trigger element), fall back to the § 1 manual pattern: store
`Option<(Entity<ValuePopover>, Point<Pixels>, Subscription)>` and render with `deferred(anchored().position(p)…)` —
identical plumbing to the context menu, just a different entity type.

## 4. `InputField` (crates/ui_input/src/input_field.rs) — as already used in this crate

Struct `:21`, `impl Focusable` `:49`, `impl Render` `:146`. API:
```rust
pub fn new(window: &mut Window, cx: &mut App, placeholder_text: &str) -> Self   // :56
pub fn start_icon(mut self, icon: IconName) -> Self                             // :77
pub fn label(mut self, label: impl Into<SharedString>) -> Self                  // :82
pub fn tab_index(mut self, index: isize) -> Self                                // :97
pub fn masked(mut self, masked: bool) -> Self                                   // :108
pub fn set_error(&mut self, error: Option<impl Into<SharedString>>, cx: &mut Context<Self>) // :116
pub fn is_empty(&self, cx: &App) -> bool                                        // :121
pub fn editor(&self) -> &Arc<dyn ErasedEditor>                                  // :125
pub fn text(&self, cx: &App) -> String                                          // :129
pub fn clear(&self, window: &mut Window, cx: &mut App)                          // :133
pub fn set_text(&self, text: &str, window: &mut Window, cx: &mut App)           // :137
```
Existing usage in `connection_modal.rs`:
- create + pre-fill (`:167-178`):
```rust
let name_field = cx.new(|cx| {
    let field = InputField::new(window, cx, "Connection name").label("Name").tab_index(1);
    if let Some(existing) = existing.as_ref() {
        field.set_text(&existing.name, window, cx);
        field.editor().set_read_only(true, cx);
    }
    field
});
```
- read (`:239-247`): `self.name_field.read(cx).text(cx)`;
- clear errors loop (`:263`): `field.update(cx, |field, cx| field.set_error(None::<SharedString>, cx))` style.
Cell editor usage (focus after create) — `table_data_view.rs:643-650`:
```rust
let field = cx.new(|cx| { let field = InputField::new(window, cx, ""); ...; field });
field.focus_handle(cx).focus(window, cx);
```

## 5. Enter / Escape plumbing

The default keymap binds these **contextlessly** (`assets/keymaps/default-macos.json:27,33`, top block has no
`"context"`): `enter → menu::Confirm`, `escape → menu::Cancel`. So with focus inside an `InputField`, a parent
element's `.on_action(cx.listener(|_, _: &menu::Confirm, ...|))` fires — this is exactly how the existing inline cell
editor works (`table_data_view.rs:1991-2008`, gated by `cell_editor_focused`, `:792-796`). Inside the popover view no
gating is needed (the handler subtree only contains the popover). `ContextMenu` handles these itself via
`key_context("menu")` (`context_menu.rs:309`).

## 6. Focus & dismissal semantics summary

| concern | ContextMenu (§1) | PopoverMenu + custom view (§3) |
|---|---|---|
| focus on open | you call `window.focus(&menu.focus_handle(cx), cx)` | automatic, 2 frames delayed (`popover_menu.rs:299-310`) — delegate `Focusable::focus_handle` to the input |
| Esc | built in (`menu::Cancel`) | add `.on_action` for `menu::Cancel` → `cx.emit(DismissEvent)` |
| click outside | built in (blur `:273` + `on_mouse_down_out` `:2218`) | add `.on_mouse_down_out(... cx.emit(DismissEvent))` (pattern: `git_ui/src/branch_picker.rs:452-459`) |
| focus restore on close | `RightClickMenu`/`PopoverMenu` restore; manual § 1 pattern does not (usually fine — panel keeps focus) | automatic (`popover_menu.rs:288-293`) |
| cleanup of state | your `cx.subscribe(..., DismissEvent)` takes the field | `PopoverMenu` clears its internal `Rc<RefCell<Option<Entity<M>>>>` |

## 7. Traps

1. **`menu.context(focus_handle)` is mandatory for `.action(...)` entries.** The menu is drawn deferred at the window
   root, outside your view's dispatch subtree; without `.context(self.focus_handle.clone())` the dispatched action
   never reaches your `.on_action` handlers (see `project_panel.rs:1117`, `database_panel.rs:379`).
2. **`cx.stop_propagation()` in `on_secondary_mouse_down`** — otherwise ancestor right-click handlers (or a future
   pane-level catch-all) also fire (`project_panel.rs:6173-6175`).
3. **Don't focus the popover's input manually at build time.** `PopoverMenu` focuses the view's `focus_handle` two
   frames after opening and would stomp it; instead make `Focusable::focus_handle` return the input's handle (§ 3.1).
4. **Nested `PopoverMenu`/dropdown inside a popover fights outside-click dismissal.** The nested menu is deferred to
   the window root, so a mousedown on it is spatially outside the popover bounds → the popover's `on_mouse_down_out`
   fires and everything closes. `Picker` works around this with explicit handle checks
   (`crates/picker/src/picker.rs:997-1005`: `actions_menu_handle.is_focused(..) || is_deployed()`). For the filter
   popover, prefer flat toggle buttons for the operator; if you must nest, keep a `PopoverMenuHandle` and check
   `is_deployed()` before emitting `DismissEvent`.
5. **`.occlude()`** blocks mouse events from leaking through to the grid beneath (hover, scroll, cell clicks).
   `ContextMenu` and `PopoverMenu` already occlude; a hand-rolled `deferred(anchored())` popup must add it itself
   (`project_panel.rs:6189-6191` shows the manual pattern).
6. **`anchored()` without `.snap_to_window_with_margin(..)` clips at window edges.** The existing panel context menus
   omit it (menus are small); the filter popover is bigger — add it.
7. **Unique `ElementId`s per column.** `PopoverMenu::new(("db-col-filter", column_index))` — duplicate ids make GPUI
   element state collide (wrong popover opens / state leaks between columns).
8. **Deferred priority:** if the popover must layer above an open context menu, give the popover's `deferred` a higher
   priority than 3 (the panels' menus use 3, `PopoverMenu` content uses 1 — fine, they're never open simultaneously
   with the same parent).
9. **Focus checks with `window.focused(cx)`:** returns `Option<FocusHandle>`. For "is my input focused" prefer
   `handle.is_focused(window)` / `handle.contains_focused(window, cx)` (`table_data_view.rs:792-796`); after opening a
   deferred popover the dispatch-tree link appears one frame late (see comment at `popover_menu.rs:299-304`), so
   same-frame `contains_focused` on the parent returns `false`.
