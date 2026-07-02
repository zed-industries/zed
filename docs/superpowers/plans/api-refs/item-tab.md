# Opening a custom view as an editor tab in Zed (workspace `Item` trait)

# Opening a custom view as an editor tab (workspace `Item` trait)

## 1. The `Item` trait — `crates/workspace/src/item.rs`

### Trait declaration (line 170)

```rust
pub trait Item: Focusable + EventEmitter<Self::Event> + Render + Sized {
    type Event;
    ...
}
```

Your view type must therefore implement **four** things: `Render`, `Focusable`, `EventEmitter<YourEvent>`, and `Item` (with `type Event = YourEvent`). If you have no events, `impl EventEmitter<()> for MyView {}` and `type Event = ();` works (csv_preview does exactly this).

### REQUIRED members (no default body)

- `type Event;` — the event type your view emits (line 171).
- `fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString;` (line 186) — the only required method. Everything else has a default.

### `ItemEvent` (line 122) and `to_item_events`

```rust
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ItemEvent {
    CloseItem,          // pane closes the tab
    UpdateTab,          // tab title/icon re-rendered
    UpdateBreadcrumbs,
    Edit,               // marks dirty-tracking / autosave behavior
}
```

The pane doesn't understand your `Self::Event` directly; it subscribes through `fn to_item_events(_event: &Self::Event, _f: &mut dyn FnMut(ItemEvent)) {}` (line 213, default no-op). Override it to translate your events, e.g. call `f(ItemEvent::UpdateTab)` when the title changes.

### Commonly overridden methods (exact signatures, with line numbers and defaults)

```rust
// line 177 — full custom tab element; default renders a Label from tab_content_text
fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
    let text = self.tab_content_text(params.detail.unwrap_or_default(), cx);
    Label::new(text).color(params.text_color()).into_any_element()
}

// line 194 — default None
fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> { None }

// line 201 — default None
fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> { None }

// line 209 — default wraps tab_tooltip_text
fn tab_tooltip_content(&self, cx: &App) -> Option<TabTooltipContent> {
    self.tab_tooltip_text(cx).map(TabTooltipContent::Text)
}

// line 229 — default None; if Some, pane.add_item fires telemetry::event!(text)
fn telemetry_event_text(&self) -> Option<&'static str> { None }

// line 263 — default false; gates clone_on_split
fn can_split(&self) -> bool { false }

// line 266 — default unimplemented!(); MUST implement if can_split() returns true
fn clone_on_split(
    &self,
    workspace_id: Option<WorkspaceId>,
    window: &mut Window,
    cx: &mut Context<Self>,
) -> Task<Option<Entity<Self>>> where Self: Sized { ... }

// line 240 — default ItemBufferKind::None. (There is no `is_singleton` method on the
// trait anymore; the equivalent is returning ItemBufferKind::Singleton here.)
fn buffer_kind(&self, _cx: &App) -> ItemBufferKind { ItemBufferKind::None }
// enum ItemBufferKind { Multibuffer, Singleton, None }  (line 163)

// Dirty/save family (defaults shown; save/save_as/reload default to unimplemented!())
fn is_dirty(&self, _: &App) -> bool { false }                     // line 278
fn has_conflict(&self, _: &App) -> bool { false }                 // line 290
fn can_save(&self, _cx: &App) -> bool { false }                   // line 293
fn can_save_as(&self, _: &App) -> bool { false }                  // line 296
fn save(&mut self, _options: SaveOptions, _project: Entity<Project>,
        _window: &mut Window, _cx: &mut Context<Self>) -> Task<Result<()>>      // line 299
fn save_as(&mut self, _project: Entity<Project>, _path: ProjectPath,
        _window: &mut Window, _cx: &mut Context<Self>) -> Task<Result<()>>      // line 308
fn reload(&mut self, _project: Entity<Project>,
        _window: &mut Window, _cx: &mut Context<Self>) -> Task<Result<()>>      // line 317

// line 326 — lets item.act_as::<T>() find an inner entity (e.g. an Editor)
fn act_as_type<'a>(&'a self, type_id: TypeId, self_handle: &'a Entity<Self>, _: &'a App)
    -> Option<AnyEntity>

// Lifecycle hooks (all default no-ops)
fn deactivated(&mut self, _window: &mut Window, _: &mut Context<Self>) {}        // line 215
fn discarded(&self, _project: Entity<Project>, _window: &mut Window, _cx: &mut Context<Self>) {} // 216
fn on_removed(&self, _cx: &mut Context<Self>) {}                                 // line 217
fn added_to_workspace(&mut self, _workspace: &mut Workspace, _window: &mut Window,
                      _cx: &mut Context<Self>) {}                                // line 360
fn set_nav_history(&mut self, _: ItemNavHistory, _window: &mut Window, _: &mut Context<Self>) {} // 261
fn navigate(&mut self, _: Arc<dyn Any + Send>, _window: &mut Window, _: &mut Context<Self>) -> bool { false } // 220

// Misc
fn show_toolbar(&self) -> bool { true }                                          // line 368
fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation { ToolbarItemLocation::Hidden } // 343
fn breadcrumbs(&self, _cx: &App) -> Option<(Vec<HighlightedText>, Option<Font>)> { None } // 347
fn as_searchable(&self, _: &Entity<Self>, _: &App) -> Option<Box<dyn SearchableItemHandle>> { None } // 339
fn include_in_nav_history() -> bool { true }                                     // line 380
fn suggested_filename(&self, cx: &App) -> SharedString { self.tab_content_text(0, cx) } // 190
```

`TabContentParams` (line 129): `{ detail: Option<usize>, selected: bool, preview: bool, deemphasized: bool, max_title_len: Option<usize>, truncate_title_middle: bool }` with helper `params.text_color() -> Color`.

**`item_id`**: not on `Item`. It lives on the object-safe wrapper trait `ItemHandle` (`fn item_id(&self) -> EntityId;`, line 527). `ItemHandle` is blanket-implemented for `Entity<T: Item>`, so once your type implements `Item`, `Box::new(entity)` coerces to `Box<dyn ItemHandle>` automatically — that box is what you hand to `pane.add_item`. There is also `SerializableItem: Item` (line 407) for workspace-restore persistence — entirely optional.

## 2. Full working example: `CsvPreviewView` — `crates/csv_preview/src/csv_preview.rs`

### Trait implementations (lines 242–274)

```rust
impl Focusable for CsvPreviewView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()      // stored field, created via cx.focus_handle()
    }
}

impl EventEmitter<()> for CsvPreviewView {}

impl Item for CsvPreviewView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::FileDoc))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        // derives "Preview foo.csv" from the source editor's file, else fallback
        ...
        .unwrap_or_else(|| SharedString::from("CSV Preview"))
    }
}
```

`Render` is implemented in `crates/csv_preview/src/renderer/preview_view.rs:7`:

```rust
impl Render for CsvPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().p_4()
            .bg(cx.theme().colors().editor_background)
            .track_focus(&self.focus_handle)   // important: ties element to focus_handle
            .child(...)
    }
}
```

### Wiring: init → register action → create entity → add to pane

Registered at app startup in `crates/zed/src/main.rs:776` (`csv_preview::init(cx);`).

`crates/csv_preview/src/csv_preview.rs:26,52–57`:

```rust
actions!(csv, [OpenPreview, OpenPreviewToTheSide]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        CsvPreviewView::register(workspace);
    })
    .detach()
}
```

The action handler that creates the view and adds it to the active pane (`csv_preview.rs:84–106`; it uses `register_action_renderer` only because it is feature-flag gated — see markdown_preview below for plain `register_action`):

```rust
pub fn register(workspace: &mut Workspace) {
    workspace.register_action_renderer(|div, _, _, cx| {
        div.when(cx.has_flag::<TabularDataPreviewFeatureFlag>(), |div| {
            div.on_action(cx.listener(|workspace, _: &OpenPreview, window, cx| {
                if let Some(editor) = workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
                    .filter(|editor| Self::is_csv_file(editor, cx))
                {
                    let csv_preview = Self::new(&editor, cx);
                    workspace.active_pane().update(cx, |pane, cx| {
                        let existing = pane
                            .items_of_type::<CsvPreviewView>()
                            .find(|view| view.read(cx).active_editor_state.editor == editor);
                        if let Some(idx) = existing.and_then(|e| pane.index_for_item(&e)) {
                            pane.activate_item(idx, true, true, window, cx);
                        } else {
                            pane.add_item(Box::new(csv_preview), true, true, None, window, cx);
                        }
                    });
                    cx.notify();
                }
            }))
            ...
```

Entity construction (`csv_preview.rs:151–192`, abbreviated): `fn new(editor: &Entity<Editor>, cx: &mut Context<Workspace>) -> Entity<Self>` calls `cx.new(|cx| { ... CsvPreviewView { focus_handle: cx.focus_handle(), ... } })`, subscribing to the source editor with `cx.subscribe(editor, |this, _editor, event: &EditorEvent, cx| ...)` and keeping the `Subscription` in a field.

Open-to-the-side variant (`csv_preview.rs:108–144`) finds/creates a split first:

```rust
let pane = workspace
    .find_pane_in_direction(SplitDirection::Right, cx)
    .unwrap_or_else(|| {
        workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, window, cx)
    });
pane.update(cx, |pane, cx| {
    ...
    pane.add_item(Box::new(csv_preview), false, false, None, window, cx);
});
```

## 3. markdown_preview (plain `register_action`, no feature flag) — `crates/markdown_preview/src/markdown_preview_view.rs`

Register (lines 95–111), called from `markdown_preview::init` in `crates/zed/src/main.rs:775`:

```rust
impl MarkdownPreviewView {
    pub fn register(workspace: &mut Workspace, _window: &mut Window, _cx: &mut Context<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_markdown_editor(workspace, cx) {
                let view = Self::create_markdown_view(workspace, editor.clone(), window, cx);
                workspace.active_pane().update(cx, |pane, cx| {
                    if let Some(existing_view_idx) =
                        Self::find_existing_independent_preview_item_idx(pane, &editor, cx)
                    {
                        pane.activate_item(existing_view_idx, true, true, window, cx);
                    } else {
                        pane.add_item(Box::new(view.clone()), true, true, None, window, cx)
                    }
                });
                cx.notify();
            }
        });
```

Its `Item` impl (lines 1083–1140) additionally overrides `act_as_type` (to expose the inner `Editor`), `telemetry_event_text` (`Some("Markdown Preview Opened")`), `added_to_workspace`, `can_save`, and uses a real event type: `type Event = MarkdownPreviewEvent;` with `impl EventEmitter<MarkdownPreviewEvent> for MarkdownPreviewView {}` (line 1080).

## 4. Pane / Workspace API signatures

`crates/workspace/src/pane.rs`:

```rust
// line 1346
pub fn add_item(
    &mut self,
    item: Box<dyn ItemHandle>,
    activate_pane: bool,
    focus_item: bool,
    destination_index: Option<usize>,   // None = insert per settings (after active item)
    window: &mut Window,
    cx: &mut Context<Self>,
)

// line 1474
pub fn activate_item(&mut self, index: usize, activate_pane: bool, focus_item: bool,
                     window: &mut Window, cx: &mut Context<Self>)

// line 1378
pub fn items_of_type<T: Render>(&self) -> impl '_ + Iterator<Item = Entity<T>>

// line 1430
pub fn index_for_item(&self, item: &dyn ItemHandle) -> Option<usize>
```

`crates/workspace/src/workspace.rs`:

```rust
// line 4540 — convenience wrapper over active pane
pub fn add_item_to_active_pane(
    &mut self,
    item: Box<dyn ItemHandle>,
    destination_index: Option<usize>,
    focus_item: bool,
    window: &mut Window,
    cx: &mut App,
)

// line 4559
pub fn add_item(&mut self, pane: Entity<Pane>, item: Box<dyn ItemHandle>,
    destination_index: Option<usize>, activate_pane: bool, focus_item: bool,
    window: &mut Window, cx: &mut App)

// line 4520
pub fn add_item_to_center(&mut self, item: Box<dyn ItemHandle>,
    window: &mut Window, cx: &mut Context<Self>) -> bool

// line 4581
pub fn split_item(&mut self, split_direction: SplitDirection, item: Box<dyn ItemHandle>,
    window: &mut Window, cx: &mut Context<Self>)

// line 7858
pub fn register_action<A: Action>(
    &mut self,
    callback: impl Fn(&mut Self, &A, &mut Window, &mut Context<Self>) + 'static,
) -> &mut Self

// line 7872 — variant that lets you conditionally attach handlers (feature flags)
pub fn register_action_renderer(
    &mut self,
    callback: impl Fn(Div, &Workspace, &mut Window, &mut Context<Self>) -> Div + 'static,
) -> &mut Self
```

## Minimal recipe

1. Struct with `focus_handle: FocusHandle` (from `cx.focus_handle()`).
2. `impl Render` (call `.track_focus(&self.focus_handle)` on the root element), `impl Focusable`, `impl EventEmitter<()>`.
3. `impl Item { type Event = (); fn tab_content_text(...) -> SharedString { "My Tab".into() } }` — optionally `tab_icon`, `telemetry_event_text`.
4. In crate `init(cx: &mut App)`: `cx.observe_new(|workspace: &mut Workspace, _, _| { ... }).detach()`, inside call `workspace.register_action(|workspace, _: &MyAction, window, cx| { ... })`.
5. In the handler: `let view = cx.new(|cx| MyView { focus_handle: cx.focus_handle(), ... });` then `workspace.active_pane().update(cx, |pane, cx| pane.add_item(Box::new(view), true, true, None, window, cx));` (or `workspace.add_item_to_active_pane(Box::new(view), None, true, window, cx)`).
6. Define the action with `actions!(my_namespace, [MyAction]);` and call your crate's `init` from `crates/zed/src/main.rs` (see lines 775–776).
