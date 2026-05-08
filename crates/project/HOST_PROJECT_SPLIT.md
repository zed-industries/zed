# Host / Project split

> Status: **in progress**. This document describes the target
> architecture and the staged migration plan for splitting
> `crates/project/src/project.rs` into a `Host` (machine-bound services)
> and `Project` (per-workspace state). Read top to bottom before making
> changes.
>
> **Phase 1 strategy update**: We chose Flavor 1 (per-project state
> lives directly on `Project`) over Flavor 2 (`ProjectXxxStore`
> wrappers). `Project` already has the per-project methods
> (`project.worktrees()`, `project.open_buffer()`, etc.) and is the
> per-project entity. A `ProjectImageStore` wrapper was introduced and
> reverted because the wrapper layer added boilerplate without holding
> any state that wasn't already on `Project`. Per-project state moves
> directly to `Project`'s fields; the host-side store shrinks to the
> registry/scanner/inbound-rpc role.
>
> Progress checkpoints (commits on `host-project-refactor`):
> - `BreakpointStore`, `TaskStore`, `SettingsObserver`, `DapStore` —
>   Phase 0 done.
> - `BookmarkStore` — already dumb, no work needed.
> - `BufferStore` — Phase 0 partial (inbound rpc forwards moved out;
>   `create_buffer_for_peer` and `LocalBufferStore::save_local_buffer` /
>   `local_worktree_entry_changed` still use `downstream_client`).
> - `WorktreeStore` — Phase 0 *and* Phase 1 done. `downstream_client`,
>   `shared`/`unshared`, `send_project_updates`, `retain_worktrees`
>   flag, and the `WorktreeHandle::Strong/Weak` storage all moved out.
>   The host's `WorktreeStore` now holds only weak references. Project
>   and HeadlessProject hold the strong handles per the per-project
>   retention policy.
> - `BufferStore` Phase 1, `GitStore`, `LspStore` — not started.
> - `ImageStore` — already dumb; per-project state will live on
>   `Project` when we have a host registry to point at.

## Why

`Project` today mixes two concerns:

1. **Machine-bound services**: LSP, DAP, fs, remote_client, environment,
   languages, toolchains, agent/MCP servers, git, etc.
2. **Workspace-bound state**: active entry, search histories, terminals,
   agent location, collab sharing state, the worktrees this workspace
   sees.

Each `Workspace` has its own `Project`, so two `Workspace`s in a
`MultiWorkspace` against the same machine duplicate the entire
machine-bound stack (two rust-analyzers, two DAP registries, etc.).

`ProjectGroupKey = (Option<RemoteConnectionOptions>, PathList)` already
encodes host identity at the `MultiWorkspace` layer; this refactor
gives that identity a runtime entity.

## Target shape

```rust
pub struct Host {
    // machine identity & I/O
    fs, remote_client, node, environment, languages,
    snippets, user_store, collab_client,

    // host-level stores: data + events only, no collab awareness
    worktree_store,        // weak handles + scanner
    buffer_store,          // registry by (worktree, path)
    image_store,
    lsp_store,
    dap_store, breakpoint_store,
    git_store,
    task_store,
    settings_observer,
    agent_server_store, context_server_store,
    toolchain_store,
    bookmark_store,
    downloading_files,
}

pub struct Project {
    host: Entity<Host>,

    // per-project view stores: strong handles to this project's slice
    worktree_store: Entity<ProjectWorktreeStore>,
    buffer_store:   Entity<ProjectBufferStore>,
    image_store:    Entity<ProjectImageStore>,

    // per-workspace UI state
    active_entry, agent_location,
    search_history, search_included_history, search_excluded_history,
    last_worktree_paths, terminals,

    // collab personality (per-project sharing)
    client_state, collaborators, client_subscriptions,
    join_project_response_message_id, remotely_created_models,
    buffer_ordered_messages_tx /* + processor task */,
    buffers_needing_diff, git_diff_debouncer,
}
```

### Key invariants

- **One `Host` per machine, shared by all `Project`s targeting it.**
  Local is one host. Each remote target (keyed by
  `RemoteConnectionOptions`) is its own host.
- **Each collab joined project gets its own dedicated `Host`.** Never
  deduped — single-use, scoped to that join, runs in remote-mode façade
  against the collab `Client`.
- **Host stores are passive.** No `downstream_client` field. No
  `shared` / `unshared` methods. They handle inbound rpc (rpc routes
  to entities; they own the data) but they do not initiate outbound
  broadcasts.
- **Collab sharing is a `Project` concern.** When shared, `Project`
  subscribes to host stores' events, filters them to its
  worktrees/buffers/servers, and forwards to its peer. Unsharing drops
  the subscriptions. `pub fn shared`/`unshared` on stores are deleted.
- **Worktree retention is implicit.** Host's `WorktreeStore` holds
  weak handles only. Strong handles live in each project's
  `ProjectWorktreeStore`. A worktree is alive iff some project holds
  it. The `retain_worktrees: bool` flag and `WorktreeHandle::Strong/Weak`
  enum go away.
- **Buffers are de-duplicated per host.** Two `Project`s on the same
  host opening the same file see the same `Entity<Buffer>`.

### Per-project state lives on `Project`, not in wrapper entities

`Project` itself is the per-project entity. It already has the
per-project method surface (`project.worktrees()`,
`project.open_buffer()`, `project.find_or_create_worktree()`, etc.) and
these methods continue to live on `Project`. Per-project state moves
from inside the host-shaped stores up to direct fields on `Project`.

```rust
pub struct Project {
    host: Entity<Host>,

    // Per-project worktree state (was inside WorktreeStore):
    worktrees: Vec<Entity<Worktree>>,           // strong handles
    retain_worktrees: bool,                     // collab share retention

    // Per-project buffer state (was inside BufferStore):
    shared_buffers: HashMap<PeerId, HashMap<BufferId, SharedBuffer>>,
    non_searchable_buffers: HashSet<BufferId>,

    // ... other per-project state (active_entry, search histories, etc.)
}
```

The host-level stores keep only the host-shaped pieces:

```rust
pub struct WorktreeStore {  // owned by Host
    next_entry_id, next_worktree_id,
    worktrees: Vec<WeakEntity<Worktree>>,       // weak registry, no retain flag
    loading_worktrees,
    state: WorktreeStoreState { Local | Remote },
    scanning_enabled, initial_scan_complete,
}
```

A `ProjectXxxStore` wrapper entity is *not* introduced. The
`ProjectImageStore` wrapper was tried and reverted: it held only an
`Entity<ImageStore>` and re-emitted events, providing no value beyond
the Project entity that already exists.

Note: methods that used to be on the host-shaped store but operate on
per-project state move to `Project`. Methods that operate on the
host-shaped state stay on the store. Some methods (e.g.
`find_or_create_worktree`) bridge — they call into the host registry to
load, then attach the result to `Project`'s strong handle list.

### Phase 1 GC notes (worktrees / buffers)

The Strong/Weak distinction on the host registry (and the analogous
`opened_buffers` weak-handle behavior on `BufferStore`) encodes a real
GC policy:

- A *visible* worktree is retained unconditionally.
- An *invisible* worktree (e.g. a single file opened outside any
  visible root) is retained only when the project is collab-shared.
- When the project becomes shared, all worktrees are promoted to
  strong. When unshared, invisible worktrees are demoted to weak so
  they can be garbage-collected once their last external strong holder
  drops them.

This policy now lives on `Project` (option 1 from the original
discussion: pure relocation). `Project` holds a
`Vec<WorktreeHandle>` with `Strong`/`Weak` variants and a
`retain_worktrees: bool` mirroring the previous logic. The host's
`WorktreeStore` holds only weak references and has no opinion about
retention.

`BufferStore` will follow the same pattern when its Phase 1 happens.
`shared_buffers: HashMap<PeerId, ...>` is per-project per-peer state
that will move to `Project` (or perhaps `Project`'s collab-state
struct) at that point.

## What we're deleting

The "flip" — per-store collab-shared mode — is what gets removed. It
has three layers:

1. **The flag.** `downstream_client: Option<(AnyProtoClient, u64)>` on
   each store. Set by `shared`, cleared by `unshared`.
2. **The orchestrator.** `Project::shared` calls each store's
   `shared(project_id, client)` and registers rpc subscriptions.
3. **Outbound calls.** Dozens of sites per store guard a peer-broadcast
   on `if let Some((client, project_id)) = self.downstream_client`.

Two non-trivial flip side effects (not just "remember the client"):

- **`WorktreeStore::shared`** sets `retain_worktrees = true` and
  promotes all weak handles to strong → replaced by per-project strong
  handles in `ProjectWorktreeStore`.
- **`LspStore::shared`** walks all servers and announces them to the
  peer → replaced by `Project::shared` walking *its* servers (filtered
  by worktree) and announcing them.

Also gone: `active_entry: Option<ProjectEntryId>` on `LspStore` (used
in `deserialize_workspace_edit`); `TaskStore::Noop` (a project just
doesn't subscribe to task events when it doesn't run tasks).

## Migration plan

Three phases. Each is independently shippable. Phase 0 and Phase 1
produce no behavioral change. Phase 2 delivers the user-visible win.

Phases 0 and 1 can interleave per store. The sweet spot established
so far: do Phase 0 (dumb store) first, then Phase 1 (wrapper) — but
for stores like `ImageStore` that are already dumb, Phase 1 can
proceed immediately. Phase 0 partial is also acceptable for stores
with pervasive `downstream_client` use; Phase 1's wrapper just inherits
the inner store's still-existing collab logic.

The wrapper-pattern works against any store. Wrapping a store with a
wide public API (e.g. `BufferStore`) requires more delegation methods,
but it's mechanical.

### Phase 0 — Dumb stores (no `Host` yet)

Per host-shaped store, in this order:

1. `BreakpointStore` — small, isolated; proof of pattern
2. `BookmarkStore`
3. `TaskStore`
4. `SettingsObserver`
5. `DapStore`
6. `GitStore`
7. `BufferStore`
8. `ImageStore`
9. `WorktreeStore`
10. `LspStore` — last; most entangled

Each store's PR does:

- For every outbound `if let Some((client, project_id)) =
  self.downstream_client { client.send(...) }` site:
  - Confirm a corresponding event is emitted. If not, add one.
  - Move the `client.send(...)` into `Project::on_<store>_event`,
    gated on `self.is_shared()`.
- For rpc handlers reading `project_id` from `self.downstream_client`,
  read it from the rpc envelope instead.
- Delete `pub fn shared`, `pub fn unshared`, and the `downstream_client`
  field.
- Update `Project::shared` / `Project::unshare_internal` to stop
  calling the deleted methods. Keep the rpc subscription registration
  in `Project::shared`.
- For `WorktreeStore`: drop `retain_worktrees`. Hold strong handles on
  `Project` directly as an interim step before Phase 1 introduces
  `ProjectWorktreeStore`.
- For `LspStore`: move "announce all servers" from `LspStore::shared`
  into `Project::shared`, filtered to this project's worktrees.

After Phase 0: no store has any collab awareness. All collab logic
lives on `Project`.

### Phase 1 — Introduce `Host`

1. Define `pub struct Host { ... }` with the fields listed above.
   Constructors:
   - `Host::local(client, node, user_store, languages, fs, env, ...)`
   - `Host::remote(remote, client, node, user_store, languages, fs, ...)`
   - `Host::collab(client, fs, languages, user_store, ...)`
2. Define `ProjectWorktreeStore`, `ProjectBufferStore`,
   `ProjectImageStore`. Move per-project methods off the host stores
   onto these.
3. `Project` gets `host: Entity<Host>`. Replace direct ownership of
   host-shaped stores with `host.read(cx).<store>()`. Add convenience
   accessors on `Project` that delegate to host so existing call sites
   keep working (`project.lsp_store(cx)` → `project.host().read(cx).lsp_store()`).
4. The three `Project` constructors (`local`, `remote`,
   `from_join_project_response`) each construct a fresh `Host` per
   project. **No host de-duplication yet.** This keeps Phase 1 a no-op.

### Phase 2 — Share `Host` across `Project`s

1. Introduce a host registry (probably on `MultiWorkspace`; could be
   app-global) keyed by host identity:
   - Local: singleton.
   - Remote: keyed by `RemoteConnectionOptions`.
   - Collab: never deduped.
2. `Project` constructors consult the registry and reuse an existing
   `Entity<Host>` if one matches; otherwise create and register.
3. Audit places that assume "this project's host is mine alone." The
   per-project view stores from Phase 1 should cover most of this; the
   audit catches stragglers (e.g. anything that used to live on
   `LspStore` but is actually per-project).
4. Verify drop ordering: `Host` drops when its last referencing
   `Project` drops. Move `cx.on_app_quit` shutdown logic from
   `Project::remote` to `Host`.

## Risks / things to handle in the migration

- **Drop ordering at shutdown.** `Project::release` and the
  `cx.on_app_quit` handler currently send `LeaveProject` and shut
  down remote processes per-project. With shared `Host`, this becomes
  reference-counted: only shut down host-level remote processes when
  the last project drops the host. Move `cx.on_app_quit` from
  `Project::remote` to `Host` in Phase 1 or 2.
- **Broadcast vs. response ordering.** A few rpc handlers do work,
  send a response, *and* broadcast the change downstream, in that
  order. Moving the broadcast to a separate event listener may
  interleave differently. Collab protocol is causally consistent so
  this should be fine, but flag and review on a per-handler basis
  during Phase 0.
- **Per-peer state on `BufferStore`.**
  `shared_buffers: HashMap<proto::PeerId, HashMap<BufferId, SharedBuffer>>`
  is per-project-collab-session state. Move it to `Project` in the
  `BufferStore` step of Phase 0.
- **`active_entry` on `LspStore`.** Used in
  `deserialize_workspace_edit` to gate snippet emission. The calling
  project must pass its `active_entry` into the LSP call site, or the
  workspace-edit deserialization moves project-side. Resolve in the
  `LspStore` step of Phase 0.
- **Initial-state announcements on share.** `Project::shared` must
  walk host stores' state (language servers, diagnostic summaries,
  worktree metadata) filtered to its own worktrees. This requires
  `Project` to know its worktree set — what `ProjectWorktreeStore`
  provides in Phase 1. In Phase 0, the project's own
  worktree/buffer/etc. fields supply the filter.
- **Lazy `Host` construction.** Phase 2 host de-duplication requires
  that constructing a `Host` does not eagerly start scanners or
  watchers. Phase 1's `Host::local` etc. should be side-effect-free
  until something subscribes.

## Cross-references

- `crates/project/src/project.rs` — `Project` struct and the three
  constructors (`local`, `remote`, `from_join_project_response`).
- `crates/project/src/project.rs` — `Project::shared`,
  `unshare_internal`, `set_role`, `disconnected_from_host_internal`:
  the orchestrator that Phase 0 thins out.
- `crates/project/src/worktree_store.rs` — `WorktreeStore::shared`
  with `retain_worktrees` and handle promotion: deleted in Phase 0.
- `crates/project/src/lsp_store.rs` — `LspStore::shared` with the
  server-announce loop: moved to `Project::shared` in Phase 0.
- `crates/workspace/src/multi_workspace.rs` — `MultiWorkspace`,
  `ProjectGroupKey`. `ProjectGroupKey::host: Option<RemoteConnectionOptions>`
  becomes `Entity<Host>` at the runtime layer in Phase 2.
