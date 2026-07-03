# Stage 3: Embedding the SQL editor — everything reusable from `sql_query_view.rs`

Source: `crates/database_ui/src/sql_query_view.rs` (617 lines, the existing SQL tab).
Goal: reuse its patterns to embed the same editor as the SQL bar in the table page
(`table_data_view.rs`). Companion doc with generic editor-embedding details:
`docs/superpowers/plans/api-refs/editor-embed.md`.

## 1. View anatomy (what SqlQueryView holds)

`sql_query_view.rs:50-70`:
```rust
pub struct SqlQueryView {
    focus_handle: FocusHandle,               // own handle, tracked in render
    client: Arc<dyn DatabaseClient>,
    connection: String,
    database: String,
    editor: Entity<Editor>,
    run_state: RunState,                     // Idle | Running | Error(String)  (:38-43)
    result: Option<Arc<QueryResult>>,        // Arc: cheap clone into uniform_list closure
    elapsed: Option<Duration>,               // wall-clock of last run
    status_message: Option<String>,          // e.g. "Cancelled"
    interaction: Entity<TableInteractionState>,
    column_widths: Option<Entity<ResizableColumnsState>>,
    _run_task: Option<Task<()>>,             // dropping aborts the in-flight run
}
```

## 2. Creating the multi-line SQL editor

`sql_query_view.rs:82-92` — buffer chain + `EditorMode::full()`:
```rust
let editor = cx.new(|cx| {
    let buffer = cx.new(|cx| {
        let buffer = Buffer::local("", cx);
        buffer.set_language_registry(language_registry.clone()); // needed for highlight queries
        buffer
    });
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
    editor.set_placeholder_text("Enter SQL…", window, cx);
    editor
});
```
- `Editor::new(mode: EditorMode, buffer: Entity<MultiBuffer>, project: Option<Entity<Project>>, window: &mut Window, cx: &mut Context<Self>) -> Self` — `crates/editor/src/editor.rs:1836`.
- The SQL tab uses `EditorMode::full()` (gutter with line numbers visible, fixed-height
  container). **For the stage-3 SQL bar (auto-height 1–5 lines, no gutter) use instead**:
  - `Editor::auto_height(min_lines: usize, max_lines: usize, window, cx)` — editor.rs:1743
    (i.e. `Editor::auto_height(1, 5, window, cx)`), or `EditorMode::AutoHeight { min_lines, max_lines }`.
  - `editor.set_show_gutter(false, cx)` — `crates/editor/src/config.rs:109`.
  - Optional: `editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx)` — config.rs:14.
    SqlQueryView sets neither soft wrap nor gutter options (Full mode defaults).
- The `AutoHeight` constructor builds its own empty buffer; to attach a language registry
  you must build the `Buffer`/`MultiBuffer` chain yourself and pass `EditorMode::AutoHeight`
  to `Editor::new` (same shape as above, just a different first argument).

Where the language registry comes from (caller side, `database_panel.rs:507`):
```rust
let language_registry = project.read(cx).languages().clone();
```

## 3. Setting the SQL language (async, extension-provided)

`sql_query_view.rs:96-111` — SQL is NOT built into Zed; it ships as the external `sql`
extension. `language_for_name("SQL")` fails if not installed → keep plain text:
```rust
cx.spawn(async move |this, cx| {
    let sql = language_registry.language_for_name("SQL").await.ok();
    if sql.is_none() {
        log::debug!("SQL language unavailable; query editor stays plain text");
    }
    // Closing the tab before the language resolves releases the entity; expected
    // race — .ok() instead of logging an error on every quick close.
    this.update(cx, |this: &mut Self, cx| {
        if let Some(buffer) = this.editor.read(cx).buffer().read(cx).as_singleton() {
            buffer.update(cx, |buffer, cx| buffer.set_language(sql, cx));
        }
    })
    .ok();
})
.detach();
```
- `LanguageRegistry::language_for_name(self: &Arc<Self>, name: &str) -> impl Future<Output = Result<Arc<Language>>>` — `crates/language/src/language_registry.rs:538`, case-insensitive.
- `Buffer::set_language(&mut self, language: Option<Arc<Language>>, cx)` — `crates/language/src/buffer.rs:1489`; `None` = plain text.

## 4. Reading / setting text programmatically

- Read: `let sql = self.editor.read(cx).text(cx);` (`sql_query_view.rs:156`;
  `Editor::text(&self, cx: &App) -> String` — editor.rs:8469).
- Set (`sql_query_view.rs:147-151`):
```rust
pub fn set_query_text(&mut self, text: impl Into<String>, window: &mut Window, cx: &mut App) {
    self.editor.update(cx, |editor, cx| {
        editor.set_text(text.into(), window, cx);
    });
}
```
  `Editor::set_text` (editor.rs:8488) **requires `&mut Window`** — plan for a `window`
  parameter on every QueryState→text regeneration path (stage 3 rewrites editor text on
  each sort/filter/page action). In tests use `view.update_in(cx, |view, window, cx| ...)`.

## 5. Subscribing to text changes (dirty detection)

SqlQueryView does NOT subscribe to its editor — stage 3 needs this for the
"text ≠ render(state) → dirty" gate. The in-tree pattern (`crates/sidebar/src/sidebar.rs:869`):
```rust
cx.subscribe(&editor, |this: &mut Self, _, event, cx| {
    if let editor::EditorEvent::BufferEdited = event {
        let text = this.editor.read(cx).text(cx);
        // compare against render(&query_state), set dirty flag, cx.notify()
    }
})
.detach(); // or store the Subscription in _subscriptions
```
- `EditorEvent::BufferEdited` — editor.rs:11795 ("emitted when an underlying buffer
  changes, including edits made through another editor").
- `EditorEvent::Edited { transaction_id }` — editor.rs:11797 (per edit transaction).
- Trap: `BufferEdited` also fires for programmatic `set_text` — when regenerating the
  text from QueryState, either set a guard flag around the `set_text` call or re-compare
  text to `render(state)` instead of latching a boolean.

## 6. Actions and keymap

Declared at `sql_query_view.rs:20-28` (doc comments are user-visible descriptions):
```rust
actions!(
    database,
    [
        /// Runs the SQL in the query editor and shows the results below.
        RunQuery,
        /// Cancels the currently-running query.
        CancelQuery,
    ]
);
```
No registration in `database_ui::init` is needed — the `actions!` macro registers them;
handlers are element-scoped in render (`sql_query_view.rs:372-376`):
```rust
v_flex()
    .key_context("SqlQueryEditor")
    .track_focus(&self.focus_handle)
    .on_action(cx.listener(|this, _: &RunQuery, _, cx| this.run_query(cx)))
    .on_action(cx.listener(|this, _: &CancelQuery, _, cx| this.cancel_query(cx)))
```
Keymap — `assets/keymaps/default-macos.json:1138-1145` (verbatim):
```json
{
  "context": "SqlQueryEditor > Editor",
  "use_key_equivalents": true,
  "bindings": {
    "cmd-enter": "database::RunQuery",
    "enter": "editor::Newline",
  },
},
```
Linux equivalent — `assets/keymaps/default-linux.json:1106-1112`: same block, `"ctrl-enter": "database::RunQuery"`, no `use_key_equivalents`.

Traps:
- The context is `"SqlQueryEditor > Editor"` (child selector) because keyboard focus sits
  in the **inner Editor**; the action bubbles up the element tree to the wrapping div's
  `on_action`. A binding on plain `"SqlQueryEditor"` would not fire while typing.
- `"enter": "editor::Newline"` is re-asserted explicitly so Enter keeps inserting a
  newline in this context. Keep it if you add the same block for the table page.
- If the stage-3 table view puts `.key_context("SqlQueryEditor")` (or wraps just the SQL
  bar in a div with that context) around its editor, the existing keymap blocks work
  as-is; a new context name (e.g. `"TableDataView"`) requires new keymap blocks in
  **both** default-macos.json and default-linux.json.
- Tooltip showing the binding (`sql_query_view.rs:280-282`):
  `Tooltip::for_action("Run Query", &RunQuery, cx)`.
- `cx.processor(...)` (used for uniform_list at :354) and `cx.listener(...)` both need
  `Context<Self>`.

## 7. Running a query

Trait — `crates/database_client/src/database_client.rs:165-184`, `#[async_trait::async_trait]`:
```rust
async fn run_query(&self, database: &str, sql: &str, max_rows: usize) -> Result<QueryResult>; // :173
async fn cancel_running(&self) -> Result<()>;                                                 // :183
```
`QueryResult` — database_client.rs:111-116:
```rust
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,   // all values as text; None = NULL
    pub truncated: bool,                  // rows dropped to respect max_rows
    pub command_tag: Option<String>,      // e.g. "SELECT 42"
}
```
Row cap: `const UI_MAX_QUERY_ROWS: usize = 1000;` (`sql_query_view.rs:32`), passed as
`max_rows` at :169. Spec says the table page must use the same cap — consider moving the
const somewhere shared in `database_ui`.

The run pattern (`sql_query_view.rs:155-191`) — two tasks, tokio for I/O + foreground for state:
```rust
pub fn run_query(&mut self, cx: &mut Context<Self>) {
    let sql = self.editor.read(cx).text(cx);
    if sql.trim().is_empty() { return; }                      // empty query = no-op

    self.run_state = RunState::Running;
    self.status_message = None;
    cx.notify();

    let client = self.client.clone();
    let database = self.database.clone();
    let task = gpui_tokio::Tokio::spawn_result(cx, async move {
        let started = Instant::now();
        let result = client.run_query(&database, &sql, UI_MAX_QUERY_ROWS).await;
        result.map(|result| (result, started.elapsed()))       // timing measured here
    });

    self._run_task = Some(cx.spawn(async move |this, cx| {
        let outcome = task.await;
        this.update(cx, |this, cx| {
            match outcome {
                Ok((result, elapsed)) => {
                    this.set_column_widths(result.columns.len(), cx);
                    this.result = Some(Arc::new(result));
                    this.elapsed = Some(elapsed);
                    this.run_state = RunState::Idle;
                }
                Err(error) => {
                    this.run_state = RunState::Error(format!("{error:#}")); // {:#} = full anyhow chain
                }
            }
            cx.notify();
        })
        .log_err();
    }));
}
```
- `gpui_tokio::Tokio::spawn_result<C, Fut, R>(cx: &C, f: Fut) -> Task<anyhow::Result<R>>`
  — `crates/gpui_tokio/src/gpui_tokio.rs:77`. Dropping the returned Task aborts the tokio
  task (abort guard inside). Requires `gpui_tokio::init(cx)` (done in `zed` main and in tests).
- Storing the foreground task in `_run_task` means starting a new run (overwriting the
  field) aborts the previous in-flight request automatically.
- Error display: `RunState::Error(message)` renders a red label instead of the grid
  (`render_results`, :303-309: `Label::new(message.clone()).color(Color::Error)` inside
  `v_flex().size_full().p_4()`). The previous `result` is retained in the struct but not
  shown while in Error state.

## 8. Cancelling

`sql_query_view.rs:195-209`:
```rust
pub fn cancel_query(&mut self, cx: &mut Context<Self>) {
    if self.run_state != RunState::Running { return; }
    self._run_task = None;                      // abort guard fires, local task dies

    let client = self.client.clone();
    gpui_tokio::Tokio::spawn_result(cx, async move { client.cancel_running().await })
        .detach_and_log_err(cx);                // server-side pg_cancel_backend

    self.run_state = RunState::Idle;
    self.status_message = Some("Cancelled".to_string());
    cx.notify();
}
```

## 9. Status line format

`status_text()` (:234-259) joins with `" · "`: `command_tag` (if any), `"{n} rows"`,
`"(truncated)"` when `result.truncated`, `"{n} ms"` from `elapsed.as_millis()`. While
running it shows `"Running…"`; the stage-3 footer's `N rows · M ms` can reuse this.

## 10. Rendering the editor in the element tree

`sql_query_view.rs:372-396` (Root is `v_flex().size_full().bg(cx.theme().colors().editor_background)`):
```rust
.child(
    div()
        .h(rems(12.))                 // fixed height — Full-mode editor fills it
        .w_full()
        .border_b_1()
        .border_color(cx.theme().colors().border)
        .overflow_hidden()
        .child(self.editor.clone()),  // Editor implements Render: entity as child directly
)
```
For the auto-height SQL bar drop the fixed `.h(...)` — an `AutoHeight` editor sizes
itself between min/max lines; keep a bordered wrapper div for the bar chrome. Default
in-tree styling via `.child(editor_entity)` picks up theme + buffer font automatically;
only use `EditorElement::new(&entity, EditorStyle { .. })` if custom fonts are needed
(see editor-embed.md §Rendering).

## 11. Focus traps

- `impl Focusable for SqlQueryView` delegates to the editor (`sql_query_view.rs:399-403`:
  `self.editor.focus_handle(cx)`) — focusing the tab focuses the editor, which is what
  makes the `"SqlQueryEditor > Editor"` keymap context resolve.
- The render root still tracks the view's **own** `focus_handle` field
  (`.track_focus(&self.focus_handle)`, :374). Both exist: `track_focus` anchors the
  `key_context` div in the focus path; `Focusable` decides where focus lands.
- For the table page, focus should stay on the grid by default and only enter the
  editor on click — do NOT delegate the whole view's `focus_handle()` to the SQL-bar
  editor, or every tab activation will drop the user into the editor.

## 12. Test scaffolding (real tokio + deterministic scheduler)

`sql_query_view.rs:460-617`. Essentials:
- `init_test`: `settings::SettingsStore::test`, `theme_settings::init(theme::LoadThemes::JustBase, cx)`,
  `editor::init(cx)`, `gpui_tokio::init(cx)`, `crate::init(cx)` (:471-480).
- `cx.executor().allow_parking()` is required because query I/O runs on real tokio.
- `wait_until` helper (:489-507): loop of `cx.run_until_parked()` +
  `cx.background_executor.timer(Duration::from_millis(5)).await`, bounded at 200 iterations —
  copy this for any test that awaits a tokio-side result.
- `FakeDatabaseClient` (`database_client::fake`): `::new()`, `::with_error("msg")`,
  `.calls()` returns strings like `"run_query app"` / `"cancel_running"` for assertions.
- Seed text + run: `view.update_in(cx, |view, window, cx| { view.set_query_text("select 1", window, cx); view.run_query(cx); })`.
