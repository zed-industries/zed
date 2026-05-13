# Multi-tenant `Host` context handoff

Companion doc to [`SHIPPING_MULTI_TENANT_PLAN.md`](./SHIPPING_MULTI_TENANT_PLAN.md). The plan is the punchlist. This file is the **agent primer** — the non-obvious context another LLM (or another engineer) needs to pick up the work without re-deriving it from the chat history.

Read this top-to-bottom once, then keep it next to the plan while making changes.

---

## 1. Mental model

`Host` is a per-machine entity holding all I/O-bound state (fs, remote client, LSP/DAP/git/buffer/worktree/bookmark/image stores, etc.). One `Host` per machine, shared across every `Project` targeting that machine (via [`HostRegistry`](./crates/project/src/host.rs) keyed on `HostKey::Local(fs_ptr)` or `HostKey::Remote(remote_entity_id)`). Each collab-joined project gets its own dedicated `Host` (never deduped).

`Project` is per-workspace. It holds a strong `Entity<Host>` plus the per-project view of the host's state (which buffers, worktrees, repositories, language servers, debug sessions, etc. *this Project* claims).

**The leak surface is everywhere a host-shared store emits events or exposes iterators that don't carry per-Project scoping.** The audit pattern is: write a two-`Project` test on the same `FakeFs` (which makes them share a `Host`), exercise the host store, assert one Project's actions don't affect the other.

---

## 2. The two ownership-tracking patterns

Pick based on what the store's state is keyed on.

### Pattern A — id-based ownership set

Use when the host state is keyed on a unique id (`BufferId`, `RepositoryId`, `LanguageServerId`, `SessionId`, `ImageId`).

```rust
// On Project:
buffers: HashSet<BufferId>,
repositories: HashSet<RepositoryId>,
language_servers: HashSet<LanguageServerId>,
dap_sessions: HashSet<SessionId>,
```

- Project claims an id at creation time (e.g. in `Project::claim_buffer`, `Project::claim_dap_session`).
- The `on_<store>_event` handler claims/prunes idempotently on `Added` / `Removed` events.
- Filtered accessors on `Project` (`Project::buffer_for_id`, `Project::dap_sessions`) intersect the host store's view with this set.
- Outbound collab broadcasts in `on_<store>_event` gate on the set: `if !self.owns_X(id) { return; }`.

### Pattern B — path/worktree filter

Use when the host state is keyed on `Arc<Path>` (absolute path), `Arc<RelPath>` + `WorktreeId`, or `WorktreeId` alone.

```rust
// Helpers on Project:
fn owns_abs_path(&self, abs_path: &Path, cx: &App) -> bool;
fn owns_worktree_id(&self, worktree_id: WorktreeId, cx: &App) -> bool;
```

- No HashSet needed — ownership is computed on the fly from `self.worktrees(cx)`.
- Filtered accessors (`Project::serialized_bookmarks`, `Project::serialized_breakpoints`, `Project::images`) iterate the host store and `filter` on the helper.
- For `load_*` operations on host stores, *merge* per-owned-path instead of *replacing all*: clear our owned paths, then add the new ones. The store's `load_X` was changed from clearing-then-inserting to additive (`BookmarkStore::load_serialized_bookmarks`, `BreakpointStore::with_serialized_breakpoints`); the per-path clear method (`clear_bookmarks_for_paths`, `clear_breakpoints_for_paths`) is the new primitive.

### Pattern C — relocate per-project state from store to `Project`

Use when the state on the host store is *fundamentally* per-Project, not host-shared (the canonical case was `Inventory::last_scheduled_tasks` / `last_scheduled_scenarios`).

- Move the field from the store onto `Project`.
- Change the store's compute methods to take the state as a parameter (`Inventory::used_and_current_resolved_tasks(last_scheduled_tasks: VecDeque<...>, ...)`).
- For cache invalidation (when the host store's settings reload), have the host store emit an event (`InventoryEvent::TaskTemplatesReloaded`); `Project` subscribes in `Project::on_inventory_event` and prunes its own state.

### Pattern D — enrich events with a discriminator

Use when an event is host-emitted but lacks the id/path needed to route it back to the originating Project.

Examples done:
- `DapStoreEvent::Notification(String)` → `Notification { session_id: Option<SessionId>, message: String }`. `None` means host-wide (every Project surfaces it); `Some(id)` is filterable.
- `InvalidSettingsError::LocalSettings { path, message }` → `LocalSettings { worktree_id, path, message }`. Plumbed through `SettingsStore::set_local_settings` and `apply_local_settings`.

The pattern is: find the one (usually one) emit site, look at what's in scope there, attach it to the event.

---

## 3. Test harness conventions

All Phase 2 audit tests live in `crates/project/tests/integration/multi_tenant.rs`. The module is registered in `project_tests.rs`'s `mod multi_tenant;` declaration.

### The two-Project setup

```rust
let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;
```

Helper at the top of the file. Builds two `Project::test(...)` instances on the same `Arc<FakeFs>`, which makes them share one `Host` (verified via the bookmark_store entity-id sanity check).

### Triggering events that aren't easy to produce naturally

For events that normally come from filesystem watchers / DAP adapters / etc., fake them directly:

```rust
let observer = project_a.read_with(cx, |p, cx| p.settings_observer(cx));
observer.update(cx, |_, cx| {
    cx.emit(SettingsObserverEvent::LocalSettingsUpdated(Err(...)));
});
cx.run_until_parked();
```

The handler runs synchronously after `run_until_parked`. Assert against subscriber-side accumulators (typically a `Rc<RefCell<Vec<String>>>` populated by a `cx.subscribe(&project_a, ...)` callback).

### Testing LRU-style invariants without a real Project

When testing an `Inventory`-style host store whose LRU has been relocated to `Project`, use the `TestLru` helper in `crates/project/tests/integration/task_inventory.rs`. It subscribes to the host store's invalidation event and mirrors `Project::on_inventory_event`'s pruning logic against a local `Rc<RefCell<VecDeque<_>>>`. Tests pass `lru.task_snapshot()` to `Inventory::used_and_current_resolved_tasks(...)` and `lru.scenario_snapshot()` to `Inventory::list_debug_scenarios(...)`.

---

## 4. Traps (read these before editing)

These bit during the work. Each is one line away from a panic or a regression.

### Async-context entity update semantics

- `Entity<T>::update(cx, f)` — returns the closure's `R` directly. Panics if entity is dead.
- `WeakEntity<T>::update(cx, f)` — returns `Result<R>`. Use `?` to propagate.
- `cx.update(|cx| ...)` on `&mut AsyncApp` — returns `Result<R>`. Use `?`.

Don't reflexively add `?` after `entity.update(...)` in async contexts. The compiler will tell you, but the error is confusing ("the `?` operator cannot be applied to type `(Entity<X>, Task<...>)`"). The same applies to `Entity::read_with` — strong entities return `R`, weak ones return `Result<R>`.

### `cx.observe_new::<T>` racing with entity construction

The callback fires *during* entity construction. Reading the entity via `cx.entity().read(cx)` panics:

```
cannot read project::debugger::session::Session while it is already being updated
```

Use the `&mut T` parameter the callback already gives you. The pattern:

```rust
cx.observe_new::<Session>(move |session: &mut Session, _, cx| {
    let session_id = session.session_id();   // ← from the parameter, not the entity
    if !owns_dap_session(session_id) { return; }
    let session_entity = cx.entity();        // safe AFTER the no-op gate
    // ... subscribe / read state
});
```

### `WeakEntity::read` from inside a render or update

If a UI delegate holds `WeakEntity<Workspace>` and reads it during render (e.g. to check `workspace.project().read(cx).last_scheduled_task(None)`), it can race with an in-flight workspace update and panic the same way.

**Fix:** hold an `Entity<Project>` (or whatever inner state you need) directly on the delegate. `TasksModalDelegate` does this — its `project: Entity<Project>` is set in `new`, alongside the existing `workspace: WeakEntity<Workspace>`.

### `collections::HashMap` vs `std::collections::HashMap`

`collections::HashMap` (`use collections::HashMap`) is FxHash-keyed. `std::collections::HashMap` is RandomState-keyed. They're different types. When wrapping a store accessor in a Project-side helper, match the store's HashMap variant exactly or you'll get a confusing type-mismatch error at the call site.

Hit this in `Project::all_bookmark_locations` which had to return `std::collections::HashMap<Entity<Buffer>, Vec<Range<Point>>>` to satisfy `Editor::open_locations_in_multibuffer`.

### `Arc<RelPath>` vs `PathBuf` in settings errors

`InvalidSettingsError::LocalSettings.path` is `Arc<RelPath>` (worktree-relative). `Tasks` and `Debug` variants use `PathBuf` (absolute). The Project filter `toast_path_visible_to_us` only handles `Path` (absolute); the `LocalSettings` variant has a separate `worktree_id` field for filtering. Don't try to unify them.

### Test "leaky" warnings

Nextest occasionally reports `LEAK [ ... ]` for a passing test. It means a background task didn't terminate within the test's lifetime. Not a real failure. Confirm by re-running the specific test in isolation.

---

## 5. File map

The bulk of the work lives in these files. Skim them in this order if you're picking up:

| File | What it has |
|---|---|
| `crates/project/src/project.rs` | The `Project` struct, all ownership sets, every `on_<store>_event` handler, every filtered accessor. ~7000 lines. Use grep to find specific methods. |
| `crates/project/src/host.rs` | `Host`, `HostKey`, `HostRegistry`. ~800 lines. Read once for the dedup mechanics. |
| `crates/project/HOST_PROJECT_SPLIT.md` | Original design doc. **Stale per the plan** — Phase 2 is described as future work but is mostly done. Rewriting it is on the §3 follow-up list. |
| `crates/project/tests/integration/multi_tenant.rs` | The audit harness. Every audit test lives here. |
| `SHIPPING_MULTI_TENANT_PLAN.md` | The rolling punchlist. Source of truth for what's done / what's left. |
| `SHIPPING_MULTI_TENANT_CONTEXT.md` | This file. |

The host stores themselves (`crates/project/src/{bookmark_store,buffer_store,git_store,lsp_store,worktree_store}.rs`, `crates/project/src/debugger/{dap_store,breakpoint_store}.rs`, `crates/project/src/{task_inventory,project_settings,image_store}.rs`) generally don't need editing for new audits — the per-Project filter lives on `Project`. Edit a store only when:
- You need to add a per-path / per-id clear method (e.g. `clear_bookmarks_for_paths`).
- You're enriching an event with a discriminator (Pattern D).
- You're moving state off the store (Pattern C).
- Or you're flipping a `load_X` from "clear-then-insert" to "merge".

---

## 6. What's done, what's not

Source of truth is the plan. Quick summary at the time of writing:

### §3 stores — **all checked off**
BookmarkStore, BreakpointStore, ImageStore, TaskStore, SettingsObserver, DapStore. Each has 1+ audit test in `multi_tenant.rs`. Each fix is described in the plan with the pattern used + test names.

### Bystander UI consumers — **all migrated**
- `debugger_ui::BreakpointList::render` → `project.serialized_breakpoints(cx)`
- `debugger_ui::DebugPanel::render` (emptiness check) → `project.serialized_breakpoints(cx)`
- `debugger_ui::DebugPanel::start_session` / `handle_restart_request` / `handle_start_debugging_request` → `project.claim_dap_session(session_id)` after `dap_store.new_session(...)`
- `debugger_ui::DebugPanel::rerun_last_session` → `project.last_scheduled_scenario()`
- `tasks_ui::Rerun` action → `project.last_scheduled_task(...)`
- `tasks_ui::modal::TasksModalDelegate` — now holds `Entity<Project>` directly; all reads go through it
- `editor::bookmarks::Editor::view_bookmarks` → `Project::all_bookmark_locations(project, cx)`
- `editor::Editor::new_internal` (DAP session subscriptions) → `project.dap_sessions(cx)` + `owns_dap_session` gate in `observe_new`
- `editor::editor_tests::BookmarkTestContext` → `project.serialized_bookmarks(cx)`
- `collab::tests::integration::editor_tests::test_add_breakpoints` (8 sites) → `editor.project().unwrap().read(cx).serialized_breakpoints(cx)`

### What's left

1. **§3 loose ends** (next natural chunk — small, contained):
   - `pending_worktree_paths` never removed after successful claim
   - `HostRegistry` accumulates dead `WeakEntity<Host>` entries
   - No `observe_release` cleanup for `HostRegistry`

2. **§3 missing direct unit tests** for established patterns:
   - `pending_worktree_paths` race
   - `claim_found_worktree` repo back-claim
   - `RepositoryAdded` re-emit on new worktree association
   - `claim_buffer` idempotency
   - Distinct `FakeFs` → distinct `Host` (the `HostKey::Local(fs_ptr)` discriminator)
   - Sibling visible-worktree skip vs pending claim
   - LSP restart in A doesn't disturb B
   - Active repository is per-Project

3. **§2 UI-layer audit / lockdown sweep** — bigger:
   - ~375 grep hits for `git_store.read` / `lsp_store.read` / `worktree_store.read` / `buffer_store.read` outside `crates/project/`
   - The plan recommends a compiler-driven sweep: rename a store accessor on `Project`, watch build errors enumerate every external call site, mechanically rewrite each to use the `Project` facade. Add a lockdown lint as the final pass.
   - Crates with the largest external surface area (per the plan): `git_ui`, `language_tools`, `editor`, `workspace`, `project_panel`, `agent_servers`, `collab`, `search`.

4. **§1 old-semantics-to-fix items** — assigned to teammates per the plan's "disjoint write scopes" section. Check what they've landed before duplicating work:
   - LspStore: `maintain_buffer_languages`, `stop_local_language_server`, `restart_all_language_servers`, `insert_newly_running_language_server`, `clear_unregistered_diagnostics`
   - GitStore: `active_repo_id`, `forget_shared_diffs_for`, `ActiveRepositoryChanged`, `RepositoryUpdated`
   - `Project::status_for_buffer_id`, `project_path_git_status`, `wait_for_initial_scan`, `handle_synchronize_buffers`
   - `Project::default_visible_worktree_paths` (helper used by `agent_server_store`, `environment`)
   - `context_server_store::create_context_server` worktree fallback
   - `buffer_store::non_searchable_buffers`
   - `language_tools` status-bar LSP indicator

5. **§2 product decisions** that haven't been made (no code change to do — just need a call):
   - LSP restart-from-A semantics: restart both, or per-Project instances?
   - LSP stop-server-for-buffers-from-A: refcount, or scope to A?
   - Invisible worktrees auto-claim across Projects: keep, or scope to initiating Project?
   - Conflicting `.zed/settings.json` for the same language server: first-write, last-write, or per-Project servers?
   - Two workspaces editing/saving the same buffer: feature or bug?
   - Peer joined to multiple of my shared Projects: what happens to `shared_diffs` / `shared_buffers`?
   - LSP/git crashes/toasts in A surfacing in B: should they?

6. **Documentation**:
   - Rewrite `crates/project/HOST_PROJECT_SPLIT.md` (still says Phase 2 is future)
   - CHANGELOG / release notes entry for the user-visible change (shared LSP/git/fs across workspaces on the same machine)

---

## 7. Workflow for the next agent

1. **Read** `SHIPPING_MULTI_TENANT_PLAN.md` for current state. Read this file for the traps.
2. **Pick** a task. Smallest contained chunks first; the loose-ends cluster is a good warm-up.
3. **For an audit** (new store / new leak surface): write a failing two-Project test in `multi_tenant.rs` *before* fixing. The test makes the leak concrete and prevents regressions.
4. **Apply** the matching pattern (A/B/C/D from §2). Don't invent a new shape — every leak so far has fit one of the four.
5. **Run** the affected crate's tests (`cargo nextest run -p <crate>`), then a broader sweep if you touched cross-crate code.
6. **Run** `./script/clippy -p <crates>` before declaring done. The script wraps clippy + machete + typos; CI runs the same.
7. **Update** the SHIPPING plan checkbox with a one-paragraph description: what moved, where, what test pins it. Patterns documented in the plan let the next agent skim instead of re-deriving.
8. **Don't** edit this context doc inline during a normal fix. If you discover a new trap worth recording, put it in your PR description under "Suggested context-doc additions" so a human can decide whether it generalizes.

---

## 8. Build / test reference

```sh
# Per-crate compile check (fastest signal)
cargo check -p project --tests

# All multi-tenant audit tests
cargo nextest run -p project --test integration 'multi_tenant::'

# Per-crate test run
cargo nextest run -p project --test integration

# Workspace-wide (~5 min, ~5800 tests). Run before declaring big-PR ready.
cargo nextest run --workspace --no-fail-fast

# Lint pass (clippy + machete + typos)
./script/clippy -p project -p workspace -p debugger_ui -p editor -p tasks_ui -p git_ui -p settings
```

The full workspace currently passes 5853/5853 with 23 skipped (pre-existing) at the time of writing this doc.
