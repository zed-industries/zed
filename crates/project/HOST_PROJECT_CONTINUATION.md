# Continuation prompt: Host / Project split, next session

Use this as the starting prompt for a fresh agent session.

---

## Context

We are partway through a multi-session refactor that splits
`crates/project/src/project.rs` into a `Host` (machine-bound services)
and `Project` (per-workspace state). The architecture, motivation,
strategy, and progress are all in `crates/project/HOST_PROJECT_SPLIT.md`
— **read that document top-to-bottom first** before doing anything.

We're working on the `host-project-refactor` branch. The branch has 12
commits over `main`. All 179 collab tests pass at HEAD
(`cargo nextest run -p collab`).

## What's done

- **Phase 0** (make stores "dumb": no `downstream_client` field, no
  `shared`/`unshared` methods, outbound broadcasts move to `Project`
  via events): `BreakpointStore`, `TaskStore`, `SettingsObserver`,
  `DapStore`, `WorktreeStore`. `BookmarkStore` was already dumb.
  `BufferStore` is partial (inbound rpc forwards moved; local-side
  `save_local_buffer` / `local_worktree_entry_changed` and
  `create_buffer_for_peer` still use `downstream_client`).
- **Phase 1** (per-project state moves to `Project` directly, no
  wrapper entities): `WorktreeStore`. `WorktreeHandle::Strong/Weak`
  and `retain_worktrees` flag now live on `Project` and
  `HeadlessProject`.

## What's next

`LspStore` Phase 0 (and Phase 1 if scope allows in one session).

**Why LspStore next, not BufferStore**: see the "Order-of-attack
revision" callout in `HOST_PROJECT_SPLIT.md`. Short version:
`BufferStore::create_buffer_for_peer` must move to `Project`, but
`LspStore`'s rpc handlers (e.g. `handle_lsp_command`,
`handle_apply_code_action`, `handle_open_buffer_for_symbol`,
`handle_format_buffers`) call it. Cleanly moving the call requires
those handlers to *also* be project-side. So `LspStore`'s Phase 0
(which moves its rpc handler responsibilities to `Project`) must come
first, otherwise we'd need `WeakEntity<Project>` back-refs as
temporary scaffolding.

`GitStore` Phase 0 follows immediately after `LspStore` (it has the
same `handle_open_commit_message_buffer` situation).

`BufferStore` Phase 1 (Option B: move `shared_buffers` and
`create_buffer_for_peer` to `Project`) lands cleanly after
`LspStore` + `GitStore`.

## Specific things `LspStore` Phase 0 needs to handle

Read these spots before designing the migration:

1. **`LspStore::shared`** in `crates/project/src/lsp_store.rs`
   walks all servers and sends `proto::StartLanguageServer` to each.
   This is the "announce on share" pattern; it must move to
   `Project::shared` (filtered to the project's worktrees, via
   `lsp_store.read(cx).language_server_statuses`).

2. **`active_entry: Option<ProjectEntryId>`** is cached on `LspStore`.
   It's used in `deserialize_workspace_edit` to decide whether to emit
   a snippet. Per-project state. Should move to `Project` (or the
   call site should pass `active_entry` in).

3. **`downstream_client: Option<(AnyProtoClient, u64)>`** on
   `LspStore` — the standard Phase 0 removal. Outbound sends move to
   `Project::on_lsp_store_event` via events. Many sites in
   `lsp_store.rs` and `lsp_store/log_store.rs` etc.

4. **rpc handlers** registered via `LspStore::init`. The Phase 0
   move is for the *handlers that need `Project` access* (because
   they call `BufferStore::create_buffer_for_peer` or
   `serialize_project_transaction_for_peer`). Those handlers move to
   `Project`. Others can stay on `LspStore`.

   Search: `grep -n 'create_buffer_for_peer\|serialize_project_transaction_for_peer' crates/project/src/lsp_store.rs crates/project/src/lsp_command.rs crates/project/src/lsp_store/lsp_ext_command.rs`

5. **`LspCommand::response_to_proto`** trait is called from inside
   `LspStore::handle_lsp_command` and friends. Some implementations
   call `lsp_store.buffer_store().create_buffer_for_peer(...)`
   (e.g. `PerformRename`, `GetDefinitions`, `GetReferences`,
   `GetLspRunnables`, etc., via `location_link_to_proto`). These will
   need restructuring — either response_to_proto returns a structure
   that the calling Project handler then orchestrates, or
   response_to_proto takes a callback / project handle. Worth
   sketching the design before writing code.

6. **HeadlessProject**: `LspStore::shared(REMOTE_SERVER_PROJECT_ID,
   session, cx)` happens during `HeadlessProject::new`. After Phase 0,
   that becomes `HeadlessProject` doing the equivalent setup itself
   (subscribing to lsp events, forwarding to session).

## Patterns established in earlier commits

Look at these recent commits for the Phase 0 + Phase 1 pattern:

- `worktree_store: Phase 0 - move broadcast logic up to listeners`
- `worktree_store: Phase 1 - move strong handle retention to Project`
- `project: Phase 0 - move collab broadcast logic out of dumb stores`
  (the BreakpointStore/TaskStore/SettingsObserver/DapStore one)

Each follows the same shape:

- Add new event variants for state changes that need broadcasting.
- Move outbound `client.send(...)` calls from inside the store to
  `Project::on_<store>_event` (and `HeadlessProject::on_<store>_event`),
  gated on the project's collab-share state.
- Read `project_id` from rpc envelopes instead of from `downstream_client`.
- Delete `pub fn shared`, `pub fn unshared`, `downstream_client` field.

Plus, for stores that have per-project state to relocate (like
`WorktreeStore`'s strong/weak handles):

- Move the per-project fields from the store to `Project` and
  `HeadlessProject` directly (Flavor 1: no wrapper entities).
- The host-side store keeps only the registry/scanner/inbound-rpc role.

## Test loop

Use `cargo nextest run -p collab --no-fail-fast` after each commit.
**Don't** use `cargo test -p collab` directly — it has known
parallel-flake issues unrelated to our changes (~50 tests fail due to
test parallelism, even on `main`). `nextest` runs each test in
isolation and is reliable.

The collab integration tests have caught real regressions in this
refactor (e.g. `test_project_reconnect` caught an eager-observer
issue in `WorktreeStore` Phase 0). Trust the suite.

## Ground rules from earlier sessions

- **Pure mechanical / isomorphic refactor**: no behavioral change.
  Don't simplify, don't refactor unrelated code, don't change tests.
- Commit each meaningful checkpoint; small commits are good. PR title
  format: `<crate>: <imperative>`. Each commit body should explain
  what moved and why; no behavioral changes if possible.
- `./script/clippy` (not raw `cargo clippy`) for lints.
- Avoid adding new files when extending existing ones is reasonable.

## First step suggestion

Read `crates/project/HOST_PROJECT_SPLIT.md`, then read
`crates/project/src/lsp_store.rs` (it's a big file — use grep + outline
mode, you don't need to read it all). Then sketch the
`LspStore::shared` migration as the easiest starting point — that
piece is mostly mechanical, mirrors what `WorktreeStore` Phase 0 did,
and gets us into the file.

Stop and check in with the user before tackling the
`response_to_proto` / `create_buffer_for_peer` cascade — that part
needs a design conversation, not code.
