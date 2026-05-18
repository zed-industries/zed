# Multi-tenant `Host` bug triage

Concrete bugs in the current branch where code reaches through a `Project` into a
shared host store (`LspStore`, `GitStore`, `WorktreeStore`, `BufferStore`) and
iterates the host's full collection where it should be iterating only what this
`Project` owns. `Project` already carries the filter sets:

- `self.buffers: HashSet<BufferId>`
- `self.repositories: HashSet<RepositoryId>`
- `self.language_servers: HashSet<LanguageServerId>`
- helpers `self.worktrees(cx)`, `self.owns_worktree_id`, `self.owns_abs_path`,
  `self.language_server_belongs_to_us` (`crates/project/src/project.rs:4145`,
  `5123`, `5128`).

Order is roughly by impact. See [`shipping_plan.md`](./shipping_plan.md) §1 for
the canonical task list; each bug below ties back to one of those checkboxes.

---

## 🚨 Ship-blockers (live bugs today)

### 1. `Project::handle_synchronize_buffers` — cross-Project info disclosure

`crates/project/src/project.rs:7182`

```rust
this.shared_buffers.entry(guest_id).or_default().clear();
for buffer in envelope.payload.buffers {
    let buffer_id = BufferId::new(buffer.id)?;
    ...
    if let Some(buffer) = this.buffer_store(cx).read(cx).get(buffer_id) {
        this.shared_buffers.entry(guest_id).or_default()
            .entry(buffer_id).or_insert_with(...);
        // serialize_ops, broadcast UpdateBufferFile / BufferReloaded / UpdateBuffer
    }
}
```

`this.shared_buffers` is per-Project (so the `.clear()` is fine), but the
buffer lookup at `buffer_store(cx).read(cx).get(buffer_id)` reaches into the
host store and returns sibling buffers. There is **no
`self.buffers.contains(&buffer_id)` gate**.

A remote peer can send a `SynchronizeBuffers` request to Project A naming a
buffer id that actually lives in sibling Project B and (a) get B's buffer state
serialized back to them and (b) cause A to re-broadcast B's `UpdateBufferFile`
/ `BufferReloaded` / `UpdateBuffer` over A's collab channel, polluting A's
collaborators with B's buffer state. This is a cross-Project information
disclosure path, not just a clobber.

**Fix:** add `if !this.buffers.contains(&buffer_id) { continue; }` before the
buffer lookup. Same pattern `handle_close_buffer` already does implicitly via
`shared_buffers` membership.

### 2. `RepositoryUpdated(_, _, is_active)` bool now lies

`crates/project/src/git_store.rs:484` (type), `1414` (emission)

```rust
pub enum GitStoreEvent {
    ActiveRepositoryChanged(Option<RepositoryId>),
    /// Bool is true when the repository that's updated is the active repository
    /// todo! remove this bool
    RepositoryUpdated(RepositoryId, RepositoryEvent, bool),
    ...
}

// on_repository_event
cx.emit(GitStoreEvent::RepositoryUpdated(id, event.clone(), false))
```

Now that active-repository state lives on `Project::active_repository_id`,
`GitStore` has no way to compute the `is_active` bool, so it hardcodes `false`.

**Live regression today:** `crates/git_ui/src/git_panel.rs:828-837` pattern
matches `RepositoryUpdated(_, StatusesChanged | HeadChanged, true)` and only
schedules an update when the bool is `true`. With the bool hardwired to
`false`, status/head changes in the active repository **never** trigger a
git-panel refresh through this arm — the panel only updates via the
`RepositoryAdded` / `RepositoryRemoved` / `GlobalConfigurationUpdated` /
`ActiveRepositoryChanged` arms. Routine `git status` updates after a file
change are dropped.

Other subscribers that pattern-match the bool but currently ignore it:

- `crates/project/src/git_store/branch_diff.rs:74-80`
- `crates/editor/src/git/blame.rs:239-246`
- `crates/acp_thread/src/acp_thread.rs:1303-1310`
- `crates/git_ui/src/conflict_view.rs:550-551`
- `crates/git_graph/src/git_graph.rs:1232`
- `crates/git_graph/src/git_graph.rs:4759` — matches `true`, silently observes nothing.

**Fix:** remove the bool from the variant. Have `git_panel.rs` subscribe to
`Project::Event::ActiveRepositoryChanged` for the "active changed" axis, and
treat every `RepositoryUpdated(repo_id, StatusesChanged | HeadChanged, _)` as a
candidate but filter by `repo_id == project.active_repository_id` at the
subscriber. The type cleanup is a touch-all-subscribers change; doing it first
exposes the `git_panel.rs` regression and removes the foot-gun for new
subscribers.

### 3. `LspStore::restart_all_language_servers` restarts every Project's servers

`crates/project/src/lsp_store.rs:11198`

```rust
pub fn restart_all_language_servers(&mut self, cx: &mut Context<Self>) {
    let buffers = self.buffer_store.read(cx).buffers().collect(); // host-wide
    self.restart_language_servers_for_buffers(buffers, HashSet::default(), cx);
}
```

`restart_language_servers_for_buffers` then derives `language_servers_to_stop`
from `local.language_server_ids_for_buffer(buffer, cx)` for every passed buffer
(L11335-49) — i.e. every server on the host — and after stopping, re-registers
every passed buffer (L11247-53).

The "Restart All Servers" button in `crates/language_tools/src/lsp_button.rs:264`
calls this. **Clicking it in workspace A restarts every server in workspace B
too**, even servers B uses that A doesn't share at all. Same applies to
"Stop All Servers".

**Fix:** add `Project::restart_all_language_servers(&self, cx)` that collects
`self.buffers` (mapped to `Entity<Buffer>` via `buffer_store.get`) and calls
`restart_language_servers_for_buffers` with that set. LSP button calls the
`Project` method. Make `LspStore::restart_all_language_servers` `pub(crate)`.

---

## ⚠️ Sibling state clobbers

### 4. `LspStore::maintain_buffer_languages`

`crates/project/src/lsp_store.rs:4742`

Host-level background task spawned once per `LspStore`. On every
`LanguageRegistry` notification it walks every buffer in the host store:

```rust
for handle in this.buffer_store.read(cx).buffers() { ... } // L4787

.map(|file| file.worktree.read(cx).is_visible()),         // L4807
```

- **Registry-reload bump:** resets the buffer's language, calls
  `local.reset_buffer(&buffer, &f, cx)`, and unregisters it from language
  servers (L4759-78).
- **Subscription tick:** sorts plain-text buffers by visibility of any worktree
  on the host, then runs `detect_language_for_buffer`, `initialize_buffer`,
  and `register_buffer_with_language_servers` for every registered buffer
  (L4803-15). Re-registers every buffer that already has a language (L4831-44).

An extension finishing loading in any window resets and re-registers every
buffer in every Project.

**Fix shape:** move ownership of this loop to `Project` (each `Project`
subscribes to its own languages-changed signal and only iterates its own
`self.buffers`). Matches the pattern used elsewhere (e.g.
`Project::on_inventory_event` for task-template reloads). Alternative: keep on
`LspStore` but route through `local.registered_buffers` and emit an event for
each `Project` to handle.

### 5. `LspStore::stop_local_language_server` clears sibling diagnostics

`crates/project/src/lsp_store.rs:11047`

```rust
// L11062-69
self.buffer_store.update(cx, |buffer_store, cx| {
    for buffer in buffer_store.buffers() { // host-wide
        buffer.update(cx, |buffer, cx| {
            buffer.update_diagnostics(server_id, DiagnosticSet::new([], buffer), cx);
            buffer.set_completion_triggers(server_id, Default::default(), cx);
        });
    }
});

// L11072-108
for (worktree_id, summaries) in self.diagnostic_summaries.iter_mut() { ... }
// emits DiagnosticsSummariesUpdated / DiagnosticsUpdated for every worktree on the host
```

Stopping `server_id` in Project A clears any matching diagnostics on Project
B's buffers (rare, but possible if A and B share a server-id) — and more
importantly, emits host-wide `DiagnosticsSummariesUpdated` /
`DiagnosticsUpdated` events for sibling worktrees that B will re-broadcast
over collab.

`language_server_statuses.remove(&server_id)` (L11125), the
`local.language_servers.remove`, and `cleanup_lsp_data` are intrinsically
host-wide and fine. The bleed is the buffer scan and the diagnostic-summary
scan.

**Fix shape:** drive the buffer-clearing scan from `local.registered_buffers`
(buffers that actually had `did_open` for this server), or gate the iteration
on the worktree-set this server was attached to (`local.lsp_tree.instances`).
Restrict the diagnostic-summaries scan to worktrees in the server's instance
set.

### 6. `LspStore::insert_newly_running_language_server` opens sibling buffers

`crates/project/src/lsp_store.rs:11567`

The `worktrees_using_server` set (L11675-91) is reconstructed from
`lsp_tree.instances`, which contains entries for every Project's worktrees
that ever caused this server to be reused. The subsequent buffer scan (L11694- 731) iterates `buffer_store.buffers()` host-wide:

```rust
self.buffer_store.clone().update(cx, |buffer_store, cx| {
    ...
    for buffer_handle in buffer_store.buffers() {
        ...
        if !worktrees_using_server.contains(&file.worktree.read(cx).id()) || ... {
            continue;
        }
        // didOpen for this buffer, register it in local.buffers_opened_in_servers, etc.
    }
});
```

As soon as the host shares an `lsp_tree` across two `Project`s, "language
server starts in A's worktree, also reused by B's worktree" results in a
`didOpen` to that server for B's buffers — and B's buffers get inserted into
`local.buffers_opened_in_servers` and `local.buffer_snapshots` under A's
`server_id`. It also emits per-buffer `LanguageServerUpdate ...
RegisteredForBuffer { buffer_id }` (L11788-98); per-`Project` event handlers
gate the wire broadcast, but the local registration and snapshot state is
wrong.

**Fix shape:** when a `Project` claims a server in `on_lsp_store_event`
(`project.rs:4213`), it should register the buffers it owns with that server,
and `insert_newly_running_language_server` should not eagerly iterate buffers.
Smaller patch: skip buffers whose `file.worktree_id` is not in any `Project`'s
worktrees that claim this server, or only register buffers already in
`local.registered_buffers`.

### 7. `LspStore::clear_unregistered_diagnostics` collects sibling abs_paths

`crates/project/src/lsp_store.rs:13082`

```rust
let mut affected_abs_paths: HashSet<PathBuf> = HashSet::default();

self.buffer_store.update(cx, |buffer_store, cx| {
    for buffer_handle in buffer_store.buffers() { // host-wide
        ...
        affected_abs_paths.insert(abs_path);
    }
});

let local = self.as_local()...;
for (worktree_id, diagnostics_for_tree) in local.diagnostics.iter() { // host-wide
    ...
    for (rel_path, diagnostics_by_server_id) in diagnostics_for_tree.iter() {
        if let Ok(ix) = diagnostics_by_server_id.binary_search_by_key(&server_id, |e| e.0) {
            ...
            affected_abs_paths.insert(abs_path);
        }
    }
}
```

`affected_abs_paths` is fed to `merge_diagnostic_entries`, which publishes
empty-diagnostic updates for every path — including sibling paths — and
per-`Project` event handlers forward them.

**Fix shape:** the function already knows `server_id`. Restrict the
path-collection passes to buffers in `local.registered_buffers` for this
`server_id` and/or `(worktree_id, rel_path)` entries whose worktree is in
`local.lsp_tree.instances[server_id]`.

### 8. LSP status-bar indicator iterates host-wide servers

`crates/language_tools/src/lsp_button.rs`

Three call sites iterate `lsp_store.read(cx).language_server_statuses()`
(host-wide):

- L228 (menu fill)
- L804 (construction)
- L1010 (refresh)

The `on_lsp_store_event` at L848 already has a `// TODO` comment noting the
problem:

```rust
// TODO `LspStore` is global and reports status from all language servers, even from the other windows.
// Also, we do not get "LSP removed" events so LSPs are never removed.
match e {
    LspStoreEvent::LanguageServerUpdate { language_server_id, name, message: ... } => { ... }
```

The "Restart All / Stop All" buttons (`fill_menu` L258-68) call
`lsp_store.restart_all_language_servers(cx)` / `stop_all_language_servers(cx)`
directly on the host store — see bug 3.

`Project` already maintains the per-tenant set
(`crates/project/src/project.rs:302`):

```rust
language_servers: HashSet<LanguageServerId>,
```

populated on `LspStoreEvent::LanguageServerAdded` (gated by
`self.language_server_belongs_to_us`, `project.rs:4212`) and cleared on
`LanguageServerRemoved` (`project.rs:4217`).

**Fix shape:**

1. Add `Project::language_server_statuses(&self, cx) -> impl Iterator<Item = (LanguageServerId, &LanguageServerStatus)>`
   that filters `lsp_store.language_server_statuses()` by
   `self.language_servers.contains(server_id)`.
2. Rewrite the three call sites in `lsp_button.rs` to use it.
3. Gate `on_lsp_store_event` (L848) on
   `project.read(cx).language_servers.contains(language_server_id)`. The
   button currently only holds a `WeakEntity<LspStore>` — needs a
   `WeakEntity<Project>` (or use the `Workspace` handle it already has).
4. Replace the "Restart All" / "Stop All" handlers with `Project`-scoped
   variants (see fix for bug 3).

### 9. `GitStore::forget_shared_diffs_for(peer)` wipes every Project's diffs

`crates/project/src/git_store.rs:705`

```rust
pub(crate) fn forget_shared_diffs_for(&mut self, peer_id: &proto::PeerId) {
    self.shared_diffs.remove(peer_id);
}
```

`shared_diffs: HashMap<proto::PeerId, HashMap<BufferId, SharedDiffs>>`
(struct decl at L108) lives on the shared `GitStore`. Entries are inserted in
`handle_open_unstaged_diff` / `handle_open_uncommitted_diff` (L3246, L3270)
keyed by peer.

Caller: `crates/project/src/project.rs:6871` (`handle_remove_collaborator`):

```rust
this.git_store(cx).update(cx, |git_store, _| {
    git_store.forget_shared_diffs_for(&peer_id);
});
```

When peer P leaves Project A, A wipes every Project's per-peer diff entry for
P — including Project B's, even though P is still on B. The next time P
scrolls a buffer in B that had an open uncommitted diff, the diff state is
gone host-side.

**Fix shape:** move `shared_diffs` off `GitStore` and onto `Project` (mirrors
the pattern already used for `shared_buffers`). Insertion happens via a
`Project::handle_open_unstaged_diff` wrapper that delegates the actual diff
loading to `GitStore`.

### 10. `Project::status_for_buffer_id` and `project_path_git_status`

`crates/project/src/project.rs:8061`, `2697`

```rust
pub fn status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus> {
    self.git_store(cx).read(cx).status_for_buffer_id(buffer_id, cx)
}
```

Underlying (`crates/project/src/git_store.rs:1768`):

```rust
pub fn status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus> {
    let (repo, path) = self.repository_and_path_for_buffer_id(buffer_id, cx)?;
    let status = repo.read(cx).snapshot.status_for_path(&path)?;
    Some(status.status)
}
```

`repository_and_path_for_buffer_id` (L1774) gets the buffer from the shared
`buffer_store`, then calls `repository_and_path_for_project_path` (L1784)
which iterates `self.repositories.values()` and picks the deepest match. No
filter by `Project::repositories`. Sibling parent monorepo can win the
`max_by_key(work_directory_abs_path)` for A's path.

**Fix shape:** in `Project::status_for_buffer_id`, gate on
`self.buffers.contains(&buffer_id)` (early return `None`) and pass
`self.repositories` (or the `self.repository_belongs_to_us` predicate,
`project.rs:4164`) into a filtered variant. Add
`GitStore::status_for_buffer_id_filtered(&self, buffer_id,
&HashSet<RepositoryId>, cx)` and have the `Project` method call it with
`&self.repositories`. Same shape for `project_path_git_status`.

### 11. `Project::wait_for_initial_scan` waits on every Project's worktrees

`crates/project/src/project.rs:2108`

```rust
pub fn wait_for_initial_scan(&self, cx: &App) -> impl Future<Output = ()> + use<> {
    self.worktree_store(cx).read(cx).wait_for_initial_scan()
}
```

`WorktreeStore::wait_for_initial_scan` (`crates/project/src/worktree_store.rs:307`)
returns a future that resolves when `initial_scan_complete.1` reads `true`.
The channel is updated in `update_initial_scan_state` (L327):

```rust
let complete = self.loading_worktrees.is_empty()
    && self.visible_worktrees(cx).all(|wt| wt.read(cx).completed_scan_id() >= 1);
```

Goes `true` only when every visible worktree on the host has finished its
initial scan and no host-wide `loading_worktrees` is pending. Project A's
`wait_for_initial_scan` is gated on Project B's slow remote worktree, and can
flip back to `false` if B adds a new worktree.

**Fix shape:** drop the watch channel from `WorktreeStore`. Have
`Project::wait_for_initial_scan` build a future that awaits
`worktree.wait_for_snapshot(1)` for each entity in
`self.worktrees(cx).filter(is_visible)`. The existing private
`WorktreeStore::observe_worktree_scan_completion` and host-wide
`initial_scan_complete` watch can both go away.

### 12. `default_visible_worktree_paths` reads host's full worktree set

`crates/project/src/project.rs:2588` (definition)

```rust
pub(crate) fn default_visible_worktree_paths(
    worktree_store: &WorktreeStore,
    cx: &App,
) -> Vec<PathBuf> {
    worktree_store
        .visible_worktrees(cx) // host-wide
        .sorted_by(|left, right| left.read(cx).is_single_file().cmp(&right.read(cx).is_single_file()))
        .filter_map(|worktree| { ... })
        .collect()
}
```

Note: the sibling helper `Project::default_path_list` (L2611) was already
converted to use `self.visible_worktrees(cx)` — exactly the pattern this
helper is missing. Phase-1 holdover.

**Callers:**

- `crates/project/src/agent_server_store.rs:1000` —
  `RemoteExternalAgentServer::get_command` uses the first result as `root_dir`
  in the `GetAgentServerCommand` proto request. Remote agent commands resolve
  relative paths against the wrong project root.
- `crates/project/src/environment.rs:179` —
  `ProjectEnvironment::default_environment` uses the first result as the
  directory for `.envrc` / shell env loading. **Project A's spawned-task
  environment can inherit Project B's direnv variables.**

**Fix shape:** delete the helper. Route through `Project`:

1. `RemoteExternalAgentServer` holds a `WeakEntity<Project>` and calls
   `project.read(cx).visible_worktrees(cx).sorted_by(...).next()`.
2. `ProjectEnvironment` holds a `WeakEntity<Project>`; `default_environment`
   uses `project.read(cx).visible_worktrees(cx)`. Alternative: push
   path-selection up to callers and pass `abs_path` into
   `ProjectEnvironment::directory_environment(abs_path, cx)`.

### 13. `ContextServerStore::create_context_server` worktree fallback

`crates/project/src/context_server_store.rs:711`

```rust
let root_path: Option<Arc<Path>> = if let Some(path) = root_path_override {
    Some(path)
} else {
    this.update(cx, |this, cx| {
        this.worktree_store.read_with(cx, |store, cx| {
            store.visible_worktrees(cx).fold(None, |acc, item| { // host-wide
                if acc.is_none() {
                    item.read(cx).root_dir()
                } else {
                    acc
                }
            })
        })
    })?
};
```

When `root_path_override` is `None`, the fallback picks the first
`visible_worktrees(cx)` entry's `root_dir()` — whichever worktree happens to
sort first across all Projects. The stdio context-server's working directory
and the remote `GetContextServerCommand`'s `root_dir` end up pointing at a
sibling Project's worktree.

The doc-comment immediately above (L677-83) actually says callers _should_
always pass `root_path_override`, but `maintain_servers` (called at L1242) is
invoked with whatever override the caller threaded in, and at least one path
fires the fallback.

**Fix shape:** `ContextServerStore` is per-`Project` (one per `Project`), so
hold a `WeakEntity<Project>` and resolve the fallback against
`project.read(cx).visible_worktrees(cx)`. Or remove the fallback and make
`root_path_override` non-optional, forcing every call site to compute it via
`Project::active_project_directory` (already referenced in the doc-comment).

### 14. Project doesn't refresh context servers on worktree add/remove

`crates/project/src/project.rs:2137` (`Project::wire_context_server_triggers`),
`4651` / `4713` (`on_worktree_store_event::WorktreeAdded` / `WorktreeRemoved`)

On `origin/main`, `ContextServerStore::new_internal` directly subscribed to
the host `WorktreeStore` and called `available_context_servers_changed` on
`WorktreeAdded` / `WorktreeRemoved`, so adding a worktree with a
`.zed/settings.json` declaring a context server would pick the server up.

The multi-tenant refactor moved the maintain loop onto `Project` (good — see
bug 13's rationale), but `Project::wire_context_server_triggers` only
subscribes to `ContextServersChanged` from the store and `observe(&registry)`.
Worktree add/remove no longer triggers a refresh.

**Surfacing scenario:** open Project A → add a new worktree to A that
contains a `.zed/settings.json` declaring an MCP server → the server
doesn't start until something else (settings file edit, AI toggle, extension
load) triggers `ContextServersChanged`.

**Fix shape:** in `Project::on_worktree_store_event`, after the existing
ownership filter has decided the worktree belongs to this Project, call
`self.available_context_servers_changed(cx)`:

- `WorktreeAdded` arm: after the `if !already_owned { ... }` block claims
  the worktree and we reach `self.on_worktree_added(...)`.
- `WorktreeRemoved` arm: after `self.worktrees.retain(...)` if any entry
  was actually removed for this Project.

This fires exactly once per (Project, worktree) pair instead of the host-
wide N-per-worktree the old in-store subscription did.

Context: discovered while resolving a merge conflict in
`crates/project/src/context_server_store.rs:442`. The merge resolution
(dropping origin's `if maintain_server_loop { ... }` block from
`new_internal`) is correct — the loop belongs on `Project` — but the
worktree trigger needs to be ported to the Project side.

---

### 15. `BufferStore::non_searchable_buffers` shared across Projects

`crates/project/src/buffer_store.rs:37`

```rust
pub struct BufferStore {
    ...
    non_searchable_buffers: HashSet<BufferId>,
    ...
}
```

Writers: L389, L538, L770, L1404. Reader (L1049):

```rust
pub(crate) fn is_searchable(&self, id: &BufferId) -> bool {
    !self.non_searchable_buffers.contains(&id)
}
```

Consumed by `crates/project/src/project_search.rs:168`:

```rust
for handle in buffers.buffers() {
    let buffer = handle.read(cx);
    if !buffers.is_searchable(&buffer.remote_id()) {
        continue;
    } ...
}
```

Two failure modes layered:

1. `non_searchable_buffers` is host-wide. Project A creating an internal
   buffer (e.g. markdown preview, notebook scratch) and marking it
   non-searchable will hide a buffer with that id from Project B's search
   results. `BufferId`s are host-allocated so collisions don't occur today,
   but as soon as a sibling Project's buffer is registered on the host with a
   marked id, B's search silently skips it.
2. The iteration `buffers.buffers()` is itself host-wide — Project A's search
   currently sees Project B's open buffers as candidates for the "already-
   open" fast-path. The `non_searchable_buffers` field is the smaller of the
   two bugs.

**Fix shape:**

- Move `non_searchable_buffers` off `BufferStore` and onto `Project` as
  `Project::non_searchable_buffers: HashSet<BufferId>`. Each writer is
  currently inside a `LocalBufferStore::create_buffer` /
  `RemoteBufferStore::create_buffer` flow invoked through a `Project` API; set
  the flag via the per-`Project` field. `BufferStoreEvent::BufferAdded` can
  carry the `project_searchable` bit and update the project field in
  `Project::on_buffer_store_event`.
- In `project_search.rs:166`, iterate over `self.buffers` (search routine has
  a `Project` in scope — verify in surrounding function). The `is_searchable`
  check becomes `!project.non_searchable_buffers.contains(...)`.

---

## Cross-cutting observations

- The recurring fix shape is: every host store needs a
  `pub(crate) fn ..._filtered(&self, allowed: &HashSet<Id>, ...)` and a
  corresponding `Project::...` facade. The shipping plan's §2 "ownership-blind
  call-site audit" already proposes this for `LspStore::language_servers_for_*`,
  `WorktreeStore::visible_worktrees`, `BufferStore::get`/`buffers`, etc. —
  every §1 bug above benefits from the same primitives, so building those
  facades first will make the §1 fixes one-liners.
- Bugs 3, 4, 6, and 7 all iterate `buffer_store.buffers()` — the single most
  clobber-prone call in `lsp_store.rs`. A
  `BufferStore::buffers_in(&HashSet<BufferId>)` helper (or consistent use of
  `self.buffers.iter().filter_map(|id| buffer_store.get(*id))`) would let
  each fix be a 1-3 line change.
- Bug 1 (`handle_synchronize_buffers`) is the only item that's an
  information-disclosure bug rather than a UX/clobber bug. The others corrupt
  or miscount sibling state; this one serves sibling state to a peer who
  shouldn't see it.
- Bug 2's `git_panel.rs` regression is live on the branch today (status-only
  changes never refresh the panel) — worth elevating from "follow-up" to
  "ship blocker".

---

## Suggested fix order

1. Bug 1 (info disclosure) — one-line gate.
2. Bug 2 (live git panel regression) — touch-all-subscribers, do it before
   adding more `RepositoryUpdated` consumers.
3. Bug 3 (Restart-All clobber) — visible to anyone clicking the LSP button.
4. Bugs 11, 12 (env / scan races) — affect remote/SSH multi-project users
   most.
5. Bug 14 (worktree-add context-server refresh) — one-line addition in
   `Project::on_worktree_store_event`; pair with the fix for bug 13.
6. Bugs 4-10, 13, 15 — sibling clobbers, fix as a batch via the
   `BufferStore::buffers_in` / `Project::..._filtered` primitives once they
   exist.
