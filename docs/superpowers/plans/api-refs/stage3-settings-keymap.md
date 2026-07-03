# Stage 3: database settings + keymap/actions reference (page_size, TableSqlEditor context, database::RunQuery reuse)

Scope: everything an engineer needs to (a) read `database.*` settings for the new footer
page-size dropdown, (b) add a keymap context for the SQL bar editor on the table page
("TableSqlEditor > Editor"), and (c) declare/reuse `database::*` actions. Companion refs:
`settings.md` (general settings machinery), `editor-embed.md` (embedding an Editor + key
bindings in general).

---

## 1. How database_ui reads `database.*` settings today

### Resolved struct — `/Users/user/zed/crates/database_ui/src/database_settings.rs` (entire file)

```rust
use database_client::ConnectionConfig;
use settings::{RegisterSetting, Settings};

#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct DatabaseSettings {
    pub page_size: u32,
    pub query_timeout_seconds: u64,
    pub mcp_max_rows: u32,
    pub connections: Vec<ConnectionConfig>,
}

impl Settings for DatabaseSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let database = content.database.clone().unwrap();
        Self {
            page_size: database.page_size.unwrap(),
            query_timeout_seconds: database.query_timeout_seconds.unwrap(),
            mcp_max_rows: database.mcp_max_rows.unwrap(),
            connections: database
                .connections
                .unwrap_or_default()
                .into_iter()
                .map(|connection| ConnectionConfig {
                    name: connection.name,
                    host: connection.host,
                    port: connection.port,
                    database: connection.database,
                    user: connection.user,
                })
                .collect(),
        }
    }
}
```

- `#[derive(RegisterSetting)]` auto-registers via `inventory` — **no manual `register()` call
  anywhere in database_ui**. Re-exported at `crates/database_ui/src/database_ui.rs:11`
  (`pub use database_settings::DatabaseSettings;`).
- Access method: `DatabaseSettings::get_global(cx)` returns `&DatabaseSettings`
  (needs `use settings::Settings as _;` in the calling file — see `table_data_view.rs:13`).

### Current read sites

| Site | Code |
|---|---|
| `crates/database_ui/src/table_data_view.rs:288` | `let limit = DatabaseSettings::get_global(cx).page_size.max(1) as usize;` (in `TableDataView::new`, used to seed `SelectSpec { limit, .. }`) |
| `crates/database_ui/src/connection_store.rs:306` | `let timeout = Duration::from_secs(DatabaseSettings::get_global(cx).query_timeout_seconds);` |
| `crates/database_ui/src/connection_store.rs:101,130` | reads `.connections` |
| `crates/database_ui/src/database_ui.rs:51` | test asserting defaults resolve (page_size 100, timeout 30, mcp_max_rows 200) |

Stage 3 note: the footer page-size dropdown seeds its default from
`DatabaseSettings::get_global(cx).page_size` exactly like `table_data_view.rs:288` does today
(keep the `.max(1)` guard — a user can set `"page_size": 0`). Settings are read once at view
construction; there is no `observe_global::<SettingsStore>` in database_ui today, and stage 3
does not require one (the spec's dropdown only uses the setting as the *initial* limit).

### Content struct — `/Users/user/zed/crates/settings_content/src/database.rs:5-24`

```rust
#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct DatabaseSettingsContent {
    /// Number of rows per page in the database table data view.
    ///
    /// Default: 100
    pub page_size: Option<u32>,
    /// Statement timeout for database queries, in seconds.
    ///
    /// Default: 30
    pub query_timeout_seconds: Option<u64>,
    /// Maximum number of rows the MCP run_query tool returns.
    ///
    /// Default: 200
    pub mcp_max_rows: Option<u32>,
    /// Configured database connections. Passwords are stored in the system keychain.
    ///
    /// Default: []
    pub connections: Option<Vec<DatabaseConnectionContent>>,
}
```

Wired into the root at `/Users/user/zed/crates/settings_content/src/settings_content.rs:170`:
`pub database: Option<DatabaseSettingsContent>`. There is **no** settings_ui `page_data.rs`
entry for the database section (optional, not needed for stage 3).

### Defaults — `/Users/user/zed/assets/settings/default.json:2723-2732` (exact quote)

```jsonc
"database": {
    // Number of rows per page in the database table data view.
    "page_size": 100,
    // Statement timeout for database queries, in seconds.
    "query_timeout_seconds": 30,
    // Maximum number of rows the MCP run_query tool returns.
    "mcp_max_rows": 200,
    // Database connections (passwords live in the system keychain).
    "connections": []
},
```

Trap: `from_settings` `.unwrap()`s every field, so any **new** `database.*` field must get a
default here or Zed panics at startup. Stage 3 as spec'd adds no new settings fields.

---

## 2. Keymap: existing "SqlQueryEditor > Editor" blocks (all three platforms, exact quotes)

`/Users/user/zed/assets/keymaps/default-macos.json:1138-1145`:
```jsonc
{
    "context": "SqlQueryEditor > Editor",
    "use_key_equivalents": true,
    "bindings": {
      "cmd-enter": "database::RunQuery",
      "enter": "editor::Newline",
    },
},
```

`/Users/user/zed/assets/keymaps/default-linux.json:1106-1112` (note: **no**
`use_key_equivalents` on Linux — replicate per-platform style exactly):
```jsonc
{
    "context": "SqlQueryEditor > Editor",
    "bindings": {
      "ctrl-enter": "database::RunQuery",
      "enter": "editor::Newline",
    },
},
```

`/Users/user/zed/assets/keymaps/default-windows.json:1103-1110`:
```jsonc
{
    "context": "SqlQueryEditor > Editor",
    "use_key_equivalents": true,
    "bindings": {
      "ctrl-enter": "database::RunQuery",
      "enter": "editor::Newline",
    },
},
```

### How "SqlQueryEditor" gets into the context stack (exact code)

`SqlQueryView::render` — `/Users/user/zed/crates/database_ui/src/sql_query_view.rs:370-377`:
```rust
impl Render for SqlQueryView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("SqlQueryEditor")          // line 373
            .track_focus(&self.focus_handle)        // line 374
            .on_action(cx.listener(|this, _: &RunQuery, _, cx| this.run_query(cx)))
            .on_action(cx.listener(|this, _: &CancelQuery, _, cx| this.cancel_query(cx)))
            // ... .child(self.editor.clone()) deeper in the tree (line 386)
```

That's the whole mechanism: `.key_context("SqlQueryEditor")` (a plain `&str` — the
`InteractiveElement::key_context` builder accepts `impl TryInto<KeyContext>`) on a **wrapper
element that contains the editor as a child**. The inner `Editor` contributes its own
`"Editor"` context automatically — `Editor::key_context`,
`/Users/user/zed/crates/editor/src/editor.rs:2649-2673`, does
`KeyContext::new_with_defaults(); key_context.add("Editor"); key_context.set("mode", mode)`
where mode is `"single_line" | "auto_height" | "full" | "minimap"`. So when focus is inside
the editor, the dispatch-path contexts are `... > SqlQueryEditor > Editor` and the keymap
selector `"SqlQueryEditor > Editor"` matches.

### Adding "TableSqlEditor > Editor" for the table-page SQL bar

1. In the table page render, wrap the SQL-bar editor child:
   ```rust
   div()
       .key_context("TableSqlEditor")
       .child(self.sql_editor.clone())
   ```
   The wrapper only needs `key_context`; it does **not** need its own `track_focus` — the
   context stack is built from the element tree above the focused (inner Editor) element.
   Put the `on_action(RunQuery)` handler on this wrapper or on the view root (see §4).
2. Add a block to **all three** keymap files, next to the existing SqlQueryEditor blocks
   (macos ~line 1145, linux ~1112, windows ~1110), copying each platform's exact style:
   `cmd-enter` on macOS, `ctrl-enter` on Linux/Windows, `use_key_equivalents: true` on
   macOS/Windows only.

**Trap (auto-height editors):** the spec's SQL bar is auto-height (1–5 lines →
`Editor::auto_height(1, 5, window, cx)`), and the default keymaps bind
`"Editor && mode == auto_height"` → `"ctrl-enter": "editor::Newline"` (macos.json:206,
linux.json:171, windows.json:168). Your `"TableSqlEditor > Editor"` block overrides it because
it appears later in the file and matches a deeper context — this is exactly the precedent used
by `"GitCommit > Editor && mode == auto_height"` (linux.json:1049-1057), which rebinds
`ctrl-enter` to `git::Commit`. Keep `"enter": "editor::Newline"` in the new block (same as the
SqlQueryEditor block) so plain Enter inserts a newline instead of anything context-inherited.

---

## 3. `actions!` macro in database_ui

Two namespaces are in use:

**Namespace `database`** — `/Users/user/zed/crates/database_ui/src/sql_query_view.rs:20-28`:
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
and `/Users/user/zed/crates/database_ui/src/table_data_view.rs:24-42`: `NextPage`, `PrevPage`,
`ToggleStructure`, `RefreshData`, `CommitCellEdit`, `CancelCellEdit`, `SetCellNull` (each with
a `///` doc comment).

**Namespace `database_panel`** — `/Users/user/zed/crates/database_ui/src/database_panel.rs:27-45`:
`Toggle`, `ToggleFocus`, `AddConnection`, `RefreshConnection`, `EditConnection`,
`RemoveConnection`, `NewSqlQuery`.

Facts:
- Full action name = `namespace::Struct`, e.g. `"database::RunQuery"` — that string is what
  keymaps and the command palette use. The `///` doc comment is the user-visible description
  (command palette / keymap editor), so every new action **must** have one.
- Adding a new action = append a variant with a doc comment to the existing `actions!` block
  in the file that owns the behavior (e.g. add `ResetToTableQuery` to the
  `table_data_view.rs:24` block). The macro generates `pub struct X;` implementing `Action`
  + `inventory` registration — nothing else to wire.
- Handler registration is per-element via `.on_action(cx.listener(...))` on a rendered element
  that is on the focus dispatch path:
  - `sql_query_view.rs:375-376` (RunQuery/CancelQuery on the view root),
  - `table_data_view.rs:1977-2008` (view root: `v_flex().key_context("TableDataView")
    .track_focus(&self.focus_handle).on_action(...)*7` plus conditional `menu::Confirm`/
    `menu::Cancel` handlers that call `cx.propagate()` when the cell editor is not focused),
  - `database_panel.rs:818-824` (panel root),
  - workspace-level: `database_ui.rs:18-30` uses `workspace.register_action` for
    `Toggle`/`ToggleFocus` inside `cx.observe_new(|workspace: &mut Workspace, ...|)`.

**Trap — duplicate action names panic at startup.** GPUI's `ActionRegistry::insert_action`
(`/Users/user/zed/crates/gpui/src/action.rs:293-300`) panics with
``Action with name `database::RunQuery` already registered`` if two `actions!` blocks declare
the same namespaced name. So to use `RunQuery` from the table page, **import the existing
type** — `use crate::sql_query_view::RunQuery;` — never redeclare it in another `actions!`.

---

## 4. Reusing `database::RunQuery` in the table-page view

`RunQuery` is a unit struct; the existing handler is view-local
(`sql_query_view.rs:375` dispatches to `SqlQueryView::run_query(&mut self, cx)`,
defined at sql_query_view.rs:155 — a method, not shareable). Reuse = register your own
handler for the same action type in the other view:

```rust
use crate::sql_query_view::RunQuery;   // do NOT redeclare via actions!

// in TableDataView::render, on the root element (alongside NextPage etc., ~line 1980):
.on_action(cx.listener(|this, _: &RunQuery, window, cx| this.run_page_query(window, cx)))
```

Why the root works: action dispatch bubbles from the focused element up through its
ancestors, so with focus inside the embedded SQL-bar `Editor`, the event passes through the
`TableSqlEditor` wrapper and then the `TableDataView` root — any of those can handle it.
Handlers on the root also make the action fire when the grid (view `focus_handle`) is
focused rather than the editor. No extra focus scope is required beyond the existing
`.track_focus(&self.focus_handle)` at table_data_view.rs:1979; the keybinding itself only
fires inside the editor because the keymap context is `"TableSqlEditor > Editor"`.

Two views handling the same action type never conflict: only handlers on the focused
element's ancestor path run, so `SqlQueryView` tabs and `TableDataView` tabs stay independent.

For the Run button, mirror the SQL tab's tooltip so the shortcut shows up
(sql_query_view.rs:277-283):
```rust
IconButton::new("table-run-query", IconName::PlayFilled)
    .tooltip(move |_window, cx| Tooltip::for_action("Run Query", &RunQuery, cx))
    .on_click(cx.listener(|this, _, window, cx| this.run_page_query(window, cx)))
```

---

## Checklist for stage 3

1. Footer page-size default: `DatabaseSettings::get_global(cx).page_size.max(1)` (pattern at
   table_data_view.rs:288); dropdown options 100/500/1000 + the setting value if it differs.
2. New action(s) (e.g. `ResetToTableQuery`): add to the `actions!(database, [...])` block in
   `table_data_view.rs:24` with a doc comment; handle via `.on_action` on the view root.
3. Run in SQL bar: import `crate::sql_query_view::RunQuery`, `.on_action` on TableDataView
   root, `.key_context("TableSqlEditor")` on the wrapper around the editor child.
4. Keymap: add `"TableSqlEditor > Editor"` blocks to all three
   `assets/keymaps/default-{macos,linux,windows}.json` (cmd-enter / ctrl-enter / ctrl-enter;
   `use_key_equivalents` on macOS+Windows only; include `"enter": "editor::Newline"`).
5. No new settings fields, no default.json change, no settings_ui change.
