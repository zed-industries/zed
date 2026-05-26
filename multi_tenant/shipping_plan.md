# Shipping multi-tenant `Host` plan

Sections:

1. **Old semantics to fix** — named regressions caused by the refactor; pure correctness.
2. **Ownership-blind call-site audit** — hundreds of sites reach through `Project` into a shared store. Systematic sweep, not a named list.
3. **New semantics to decide** — multi-tenant raises product questions that didn't exist before.
4. **Incomplete refactors** — structural work started but not finished; decide ship-with vs follow-up.
5. **Legwork / meta** — coordination, review, manual testing, rollout.

---

## 1. Old semantics to fix

Bugs introduced or exposed by moving to shared `Host`. Each is a real multi-tenant
correctness bug that the automated test suite can't catch without an explicit
two-Project test.

### `crates/language_tools/`

- [x] Status-bar LSP indicator iterates shared `LspStore` — fixed: `Project::language_server_statuses` now filters by Project-owned server ids, and `LspButton` reads/gates status state through its `Project` instead of the shared host `LspStore`. Regression test: `test_language_server_statuses_are_scoped_to_project` in `crates/project/tests/integration/multi_tenant.rs` verifies two Projects sharing one host `LspStore` each see only their own server status.

### `crates/project/src/lsp_store.rs`

- [ ] `maintain_buffer_languages` (~L4751) touches sibling buffers.
- [ ] `stop_local_language_server` (~L11049) clears sibling diagnostics.
- [x] `restart_all_language_servers` (~L11184) restarts sibling servers — fixed: status-bar “Restart All Servers” / “Stop All Servers” now route through `Project::{restart,stop}_all_language_servers`; restart derives servers from Project-owned open buffers, while stop uses Project-owned language server ids. Regression test: `test_restart_all_language_servers_is_scoped_to_project` in `crates/project/tests/integration/multi_tenant.rs` verifies Project A does not shut down Project B's server.
- [ ] `insert_newly_running_language_server` (~L11679) attaches to sibling buffers.
- [ ] `clear_unregistered_diagnostics` (~L13076) collects sibling abs_paths.

### `crates/project/src/git_store.rs`

- [x] `active_repo_id` is a single field shared across Projects — moved to `Project::active_repository_id` on this branch. `GitStore` no longer owns active repository state.
- [ ] `forget_shared_diffs_for(peer)` clears every Project's diffs.
- [ ] `forget_all_shared_diffs` (called from `Project::unshare_internal`) `.clear()`s `shared_diffs` for every Project's peers. Bug 20. Same fix shape as bug 9 — move `shared_diffs` onto `Project`.
- [ ] `GitStore::checkpoint` / `restore_checkpoint` / `compare_checkpoints` iterate `self.repositories.values()` host-wide. Used by agent "rewind". Bug 19.
- [x] `RepositoryUpdated(_, _, is_active)` still carries host-level `is_active` shape — fixed: `GitStoreEvent::RepositoryUpdated` now carries only `(RepositoryId, RepositoryEvent)`. `Project::on_git_store_event` computes project-scoped `is_active` and re-emits `GitEvent::RepositoryUpdated { repo_id, event, is_active }`; UI/project consumers now subscribe to `Project` git events.
- [x] `ActiveRepositoryChanged` as a `GitStoreEvent` is no longer emitted by `GitStore` — fixed: the dead host-level variant was removed. Active repository changes now emit `Project::Event::ActiveRepositoryChanged` and `Project::GitEvent::ActiveRepositoryChanged`.

### `crates/project/src/project.rs`

- [ ] `status_for_buffer_id` can return sibling repo's status.
- [ ] `project_path_git_status` can return sibling repo's status.
- [ ] `wait_for_initial_scan` waits on every Project's worktrees.
- [x] `handle_synchronize_buffers` doesn't gate on `self.buffers` — fixed: `project.rs:7222` now skips buffer ids not in `self.buffers` before the shared `BufferStore` lookup. Regression test: `test_synchronize_buffers_does_not_disclose_sibling_project` in `crates/collab/tests/integration/integration_tests.rs` exercises the attack end-to-end through the collab server. Bug 1.
- [ ] `Project::shared` / `Project::reshared` iterate `git_store.repositories()` host-wide — collab peer joining Project A receives `UpdateRepository`s for every host repo, including Project B's. Bug 16.
- [ ] `Project::handle_reload_buffers` no ownership gate on `buffer_ids`. Bug 21.
- [ ] `Project::handle_register_buffer_with_language_servers` no ownership gate; peer can pin sibling buffer into A's LSP state. Bug 22.
- [ ] `Project::on_breakpoint_store_event` broadcasts `BreakpointsForFile` for any toggled path on the shared store without `self.owns_abs_path` gate. Bug 18.
- [x] `git_panel.rs:837` pattern-matches `GitStoreEvent::ActiveRepositoryChanged(_)` but `GitStore` no longer emits it — fixed: `GitPanel` now listens only to project-scoped `GitEvent`s for git updates/errors, and the host-level `GitStoreEvent::ActiveRepositoryChanged` variant was removed.

### `crates/project/src/agent_server_store.rs`, `environment.rs`

- [ ] `default_visible_worktree_paths` reads host's full worktree set — thread Project through; delete the helper.

### `crates/project/src/context_server_store.rs`

- [ ] `create_context_server` worktree fallback picks first sibling worktree when `root_path_override` is `None`.
- [ ] `Project::wire_context_server_triggers` doesn't refresh on `WorktreeAdded` / `WorktreeRemoved` — the in-store subscription that origin/main had was deliberately removed (correctly), but the Project-side replacement was never wired. Add `self.available_context_servers_changed(cx)` to the `WorktreeAdded` / `WorktreeRemoved` arms of `Project::on_worktree_store_event`, after the ownership filter. See `bugs.md` bug 14.

### `crates/project/src/buffer_store.rs`

- [ ] `non_searchable_buffers` lives on the shared store — Project A marking bleeds into B's search.
- [ ] `BufferStore::handle_save_buffer`, `handle_update_buffer`, `handle_update_buffer_file` stay registered on the shared `BufferStore` entity and look up buffers without project-ownership gates — collab peer of Project A can save/update sibling Project B's buffer via A's `project_id`. Bug 17. Fix: move handlers onto `Project` (mirroring Phase 1's move of `handle_close_buffer` / `handle_synchronize_buffers`).

---

## 2. Ownership-blind call-site audit

The structural refactor is done, but ~375 call sites of `project.<store>(cx)` were left as-is. Each is potentially reading or writing shared-store state without an ownership filter. Concrete example:

```rust
// crates/git_ui/src/project_diff.rs:301
let Some(repo) = project.read(cx).git_store(cx).read(cx).active_repository() else { ... };
//                                                       ^^^^^^^^^^^^^^^^^
//                                  reads the shared field; in multi-tenant this is whoever-set-it-last
```

Should be `project.read(cx).active_repository(cx)`.

### Scale

| Pattern                      | Sites    |
| ---------------------------- | -------- |
| `project.git_store(cx)`      | 107      |
| `project.lsp_store(cx)`      | 144      |
| `project.worktree_store(cx)` | 72       |
| `project.buffer_store(cx)`   | 52       |
| **Total**                    | **~375** |

### Approach: let the compiler do the audit

Don't grep-and-read 375 sites. For each obviously-wrong accessor:

1. Add a `Project::<m>` method that does the ownership filter (if it doesn't exist).
2. Make the underlying store method `pub(crate)` (or rename to `<m>_unfiltered` so it's grep-able).
3. `cargo check --workspace --all-targets` — build errors enumerate every external call site.
4. Mechanically rewrite each site to use the `Project` facade.
5. Re-run the test suite; commit.

Each accessor is one self-contained commit. Very agent-amenable — the build is the spec.

### Known-bad accessors (one commit each)

Order roughly by user-visible impact:

- [x] `GitStore::active_repository` — removed; external call sites touched in this branch now use `Project::active_repository` / project-scoped setters.
- [ ] `GitStore::repositories` — ownership-blind iteration; add/use `Project::repositories`.
- [ ] `GitStore::repository_for_*` family — needs ownership filter.
- [ ] `LspStore::language_server_statuses` — source of the status-bar indicator bug (§1).
- [ ] `LspStore::language_servers_for_*` family.
- [ ] `WorktreeStore::visible_worktrees` — bypasses `Project::worktrees`.
- [ ] `WorktreeStore::worktrees`.
- [ ] `WorktreeStore::worktree_for_id`.
- [ ] `BufferStore::get(id)` and `BufferStore::buffers()`.

_Expand as Pass 3 surfaces more._

### Legitimate raw-store access

Some callers genuinely need the store handle (subscribing to events, threading the entity into a child component, single-tenant-by-construction paths in `crates/project/` itself). When a build error turns out to be one of these, the fix is to keep `project.<store>(cx)` exactly as-is — don't invent a `Project` facade just to satisfy the lockdown.

### Lockdown (final pass, separate PR)

Once the known-bad accessors are gone:

- [ ] Move `Project::git_store`, `lsp_store`, etc., to `pub(crate)` if feasible, with a `pub` escape hatch like `Project::raw_git_store_for_testing` or `Project::store_handles()` so legitimate consumers stay grep-able.
- [ ] Or: add a clippy lint that flags `project.<store>(cx)` outside `crates/project/` and `tests/`.

Without lockdown, the next refactor regrows the same pattern.

---

## 3. New semantics to decide

Each needs a product call before the code change.

### LSP lifecycle

- [ ] Restart from workspace A while B uses the same server — restart both, or per-Project instances?
- [ ] Stop-server-for-buffers from A — refcount across Projects, or scope to A?

### Invisible worktrees

- [ ] Go-to-definition into a file outside any worktree currently auto-claims for every Project (Phase 1 holdover). Keep, or scope to the initiating Project?

### Settings precedence

- [ ] Two workspaces with different `.zed/settings.json` configuring the same language server — first-write wins, last-write wins, or force per-Project servers?

### Save / edit conflicts

- [ ] Two workspaces editing the same buffer share state today. Feature or bug? Document either way.
- [ ] Two workspaces saving the same file concurrently.

### Snippets

- [ ] `Host::snippets: Entity<SnippetProvider>` is shared across every Project on a machine (`crates/project/src/host.rs:268`; the field carries `// todo! Are there host local snippets?`). Per-workspace snippet config would leak across Projects on the same host. Decide: host-shared (current), or per-Project?

### Collab + multi-tenant

- [ ] Peer joined to multiple of my shared Projects — what happens to `shared_diffs` / `shared_buffers` when they leave one?

### Cross-workspace observability

- [ ] Should LSP/git crashes and toasts in Project A surface in Project B's window?

---

## 4. Incomplete refactors

### Stores not yet audited for Phase 2

Audit recipe: write the two-Project test first; if it passes, document as trivially multi-tenant; if not, apply the established pattern (per-project HashSet on `Project`, filtered accessors, ownership-gated handlers).

Audit scaffold lives in `crates/project/tests/integration/multi_tenant.rs` — add the failing two-Project test there before applying a fix.

- [x] `DapStore` — fixed: per-Project `dap_sessions: HashSet<SessionId>` on `Project`, populated by `Project::claim_dap_session` (called from each of the three `DapStore::new_session` sites in `debugger_ui::DebugPanel` — `start_session`, `handle_restart_request`, `handle_start_debugging_request`), pruned by `Project::on_dap_store_event` on `DebugClientShutdown`. New filtered accessors `Project::owns_dap_session`, `dap_session_by_id`, `dap_sessions`. The handler now gates the collab `LogToDebugConsole` broadcast on ownership, and the new `Notification { session_id, message }` variant (was `Notification(String)`) is filtered on session ownership for session-scoped toasts — host-wide ones (`session_id: None`) still toast everywhere. `Project::active_debug_session` filters via `owns_dap_session` so a sibling's host-wide `BreakpointStore::active_position` doesn't show in our editors. `Editor::new_internal` now subscribes only to sessions in `project.dap_sessions(cx)` and gates the `observe_new::<Session>` callback on `owns_dap_session`. Tests: `test_dap_store_notification_scoped_by_session_id`, `test_dap_store_session_ownership_set`.
- [x] `BreakpointStore` — fixed: `Project::serialized_breakpoints`, `Project::restore_serialized_breakpoints`, `Project::clear_breakpoints` filter by `Project::owns_abs_path`. Store gained `clear_breakpoints_for_paths`; `with_serialized_breakpoints` now merges instead of replacing. Workspace serialize/load and the `ClearAllBreakpoints` action route through `Project`. Tests: `test_breakpoint_store_per_project_serialization`, `test_breakpoint_store_load_preserves_other_project`, `test_breakpoint_store_clear_per_project`.
- [x] `TaskStore` — fixed: `last_scheduled_tasks` and `last_scheduled_scenarios` moved off `Inventory` onto `Project`. `Inventory::used_and_current_resolved_tasks` and `list_debug_scenarios` now take the LRU as a parameter. New `Project::task_scheduled` / `scenario_scheduled` / `last_scheduled_task` / `last_scheduled_scenario` / `delete_previously_used_task` / `last_scheduled_tasks` / `last_scheduled_scenarios` accessors. Cache invalidation rides a new `InventoryEvent::{TaskTemplatesReloaded, DebugScenariosReloaded}` event that each `Project` subscribes to in `Project::on_inventory_event`. Call sites updated in `workspace::tasks::schedule_resolved_task`, `tasks_ui::Rerun` action, `tasks_ui::modal::TasksModalDelegate` (now holds an `Entity<Project>` directly to avoid in-render workspace-update collisions), `debugger_ui::DebugPanel::{start_session, rerun_last_session}`, and `debugger_ui::new_process_modal::{NewProcessModal::show, DebugDelegate::tasks_loaded}`. Test: `test_task_inventory_last_scheduled_per_project`.
- [x] `SettingsObserver` — fixed: `on_settings_observer_event` now gates the collab proto broadcast (`UpdateWorktreeSettings`) on `Project::owns_worktree_id`, gates `Event::Toast` emissions for `Tasks` / `Debug` / `LocalSettingsUpdated::Ok` paths on `Project::toast_path_visible_to_us` (which lets _global_ config paths through so they toast in every workspace), and gates `LocalSettingsUpdated::Err(LocalSettings { worktree_id, .. })` on `Project::owns_worktree_id`. `InvalidSettingsError::LocalSettings` was enriched with a `worktree_id: WorktreeId` field plumbed through `SettingsStore::set_local_settings` and `apply_local_settings`. Tests: `test_settings_observer_toast_scoped_to_owning_project`, `test_settings_observer_global_toasts_in_every_project`, `test_settings_observer_local_settings_scoped_by_worktree_id`.
- [x] `BookmarkStore` — fixed: `Project::serialized_bookmarks` and `Project::restore_serialized_bookmarks` filter by `Project::owns_abs_path`. Store gained `clear_bookmarks_for_paths`; `load_serialized_bookmarks` now merges instead of replacing. Workspace serialize/load routes through `Project`. Tests: `test_bookmark_store_per_project_serialization`, `test_bookmark_store_load_preserves_other_project`.
- [x] `ImageStore` — fixed: `Project::images(cx)` filters by `Project::owns_worktree_id` (checking the image's `file.worktree_id`). UI surfaces that want per-Project visibility should go through `Project::images` rather than `ImageStore::images()`. Test: `test_image_store_images_per_project`.

### Bystander accessors migrated

The stores still expose host-wide accessors (`all_serialized_bookmarks`, `all_source_breakpoints`, `BookmarkStore::all_bookmark_locations`, `ImageStore::images`) because some test/collab paths still need them. Production UI surfaces should go through `Project`. Known direct callers were migrated:

- [x] `debugger_ui::session::running::breakpoint_list::BreakpointList::render` — now reads `workspace.project().serialized_breakpoints(cx)` (with a fallback to the unfiltered store accessor if the workspace handle has been dropped).
- [x] `debugger_ui::debugger_panel::DebugPanel::render` (emptiness check) — now reads `project.serialized_breakpoints(cx)`.
- [x] `editor::bookmarks::Editor::view_bookmarks` — now calls `Project::all_bookmark_locations(project, cx)` which wraps the store accessor and filters by `owns_worktree_id`.

All known bystander accessors have been migrated:

- [x] `editor::editor_tests::BookmarkTestContext::all_bookmarks` — now goes through `project.serialized_bookmarks(cx)`.
- [x] `collab::integration::editor_tests::test_add_breakpoints` (8 sites) — now go through `editor.project().unwrap().read(cx).serialized_breakpoints(cx)`.

### Loose ends in the existing pattern

- [ ] `pending_worktree_paths` never removed after successful claim.
- [ ] `HostRegistry` accumulates dead `WeakEntity<Host>` entries.
- [ ] No `observe_release` cleanup for `HostRegistry`.

### Missing direct unit tests for the patterns

- [ ] `pending_worktree_paths` race.
- [x] `claim_found_worktree` repo back-claim — covered by `test_project_claiming_existing_repository_sets_active_repository`.
- [ ] `RepositoryAdded` re-emit on new worktree association.
- [ ] `claim_buffer` idempotency.
- [ ] Distinct `FakeFs` → distinct `Host` (the `HostKey::Local(fs_ptr)` discriminator).
- [ ] Sibling visible-worktree skip vs pending claim.
- [ ] LSP restart in A doesn't disturb B's buffers (would fail today).
- [x] Active repository is per-Project — `Project::active_repository_id` now owns the state and the GitStore property test asserts active ids are project-owned.
- [ ] Active repository fallback is directly tested — add a targeted scenario/property operation for removing the active repo while another project-owned repo remains.

### UI-layer audit (outside `crates/project/`)

- [ ] Sweep `lsp_store.read`, `worktree_store.read`, `git_store.read`, etc. outside `project/` for unfiltered iterations. LSP indicator was one; almost certainly more.

### Project re-emits `LspStoreEvent` migration follow-ups

`Project::on_lsp_store_event` now re-emits `LspStoreEvent` (with ownership filtering via `owns_lsp_event`) instead of translating to `Project::Event::*` LSP variants. Open items beyond the consumer migration itself:

- [x] `Project::handle_language_server_prompt_request` (RPC handler at `project.rs:~7083`) now emits `LspStoreEvent::LanguageServerPrompt` instead of `Event::LanguageServerPrompt`. Landed together with the `Workspace` migration of the LSP prompt subscription.
- [x] Dead `Project::Event::*` LSP variants removed from the `Event` enum: `DiagnosticsUpdated`, `LanguageServerAdded`, `LanguageServerRemoved`, `LanguageServerLog`, `LanguageServerBufferRegistered`, `LanguageNotFound`, `RefreshInlayHints`, `RefreshSemanticTokens`, `RefreshCodeLens`, `LanguageServerPrompt`, `DiskBasedDiagnosticsStarted`, `DiskBasedDiagnosticsFinished`, `SnippetEdit`, `WorkspaceEditApplied`.
- [ ] `LspStoreEvent::LanguageServerUpdate { message: proto::update_language_server::Variant::*, .. }` is awkward to match on at consumer sites — see `editor.rs:~1964` matching `RegisteredForBuffer`, and the equivalent in `language_tools/lsp_button.rs` matching `StatusUpdate { Binary(_) | Health(_) }`. The variant is a multiplexed proto envelope (`RegisteredForBuffer`, `StatusUpdate { Binary(_) | Health(_) }`, `WorkStart/Progress/End`, `MetadataUpdated`), so every consumer has to know proto-shaped details. Consider splitting these into first-class `LspStoreEvent` variants once the migration is otherwise stable. The removed `Event::LanguageServerBufferRegistered` is a good shape to revive as `LspStoreEvent::LanguageServerBufferRegistered { server_id, buffer_id, buffer_abs_path, name }` — typed `BufferId` (not `u64`), typed `PathBuf` (not `String`), no proto envelope. Likely also worth: `LanguageServerBinaryStatus`, `LanguageServerHealth`, `LanguageServerWorkProgress`. Mechanical change but big diff; defer until after Group B.

---

## 5. Legwork / meta

### Coordination

- [ ] Merge `main` into branch, fix conflicts, restabilize on `cargo nextest run --workspace`.
- [ ] Open draft PR; paste rewritten `HOST_PROJECT_SPLIT.md` as the body.
- [x] 60-min live onboarding with both teammates: doc walkthrough → `Host`/`HostRegistry`/`HostKey` → walk one leak fix end-to-end → patterns against prior commits.
  - [@anthony @cameron]

### Manual testing

3 people × ≥3 working days daily-driver on the branch. Two workspaces against the same machine, at least one window remote/SSH.

- [ ] LSP server starts only once per (language, project) combo — check process list, not just UI.
- [ ] Status-bar LSP indicator reflects active workspace's servers only.
- [ ] Restart-LSP from A: confirm B's behavior matches the §2 decision.
- [ ] Stop-LSP-for-buffers from A doesn't kill servers B needs.
- [ ] Diagnostics appear only in the owning workspace.
- [ ] Go-to-definition into a file outside any worktree — confirm behavior matches the §2 decision.
- [ ] Active git repository is per-workspace.
- [ ] Git status indicators in project panel are per-workspace.
- [ ] Search in A only searches A's worktrees.
- [ ] Close A doesn't break B (servers, file watchers).
- [ ] SSH disconnect — both workspaces behave correctly.
- [ ] Two workspaces with conflicting `.zed/settings.json` — confirm §2 decision.
- [ ] Bug-bash doc with `blocker` / `ship-with` / `follow-up` triage tags.

### Rollout

- [ ] Land to `main`, bump nightly immediately.
- [ ] ≥1 week nightly soak with team before stable.
