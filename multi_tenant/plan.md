# Multi-tenant `Host` refactor — plan index

**This is the single entry point for the multi-tenant refactor.** Every
sub-document is linked below. If you're an agent picking up work on this
branch, start here and follow the link relevant to your task.

---

## How to use these docs

These docs are **living state**. They reflect the current branch and get
edited as work lands.

If you (agent or human) do any of the following, update the relevant doc
**in the same PR / commit** that does the work:

- **Fix one of the bugs** in `bugs.md` → move it from open to done in
  `bugs.md` *and* check off the corresponding box in `shipping_plan.md` §1.
- **Migrate a store** through Phase 2 ownership filtering → update
  `shipping_plan.md` §4 with the same `[x] StoreName — fixed: …` format used
  for `DapStore`, `BreakpointStore`, etc.
- **Land an `..._filtered` accessor / `Project::…` facade** for an
  ownership-blind call site → check the box in `shipping_plan.md` §2 and
  note the new accessor name.
- **Make a product decision** on one of the open semantics questions →
  record the decision in `shipping_plan.md` §3 *and* in
  `git_store_host_future_questions.md` if it's git-shaped.
- **Discover a new bug** that the existing docs don't list → add it to
  `bugs.md` with file:line, code excerpt, and proposed fix shape. Add a
  matching checkbox to `shipping_plan.md` §1.
- **Add new tests** for the patterns → check the box under
  `shipping_plan.md` §4 "Missing direct unit tests".
- **Move active-repo / git invariants forward** → update
  `git_store_host_context.md`.
- **Move worktree-sharing invariants forward** → update
  `worktree_store_property_test.md`.

If a section grows past one screen of unresolved items, propose splitting it
into a sub-doc and link from here.

If you discover that something in these docs is stale (the code doesn't
match the doc), correct the doc rather than the code, unless the code change
is intended.

---

## Documents

### [`shipping_plan.md`](./shipping_plan.md) — main task list

The canonical 5-section plan: §1 named regressions, §2 ownership-blind
call-site audit (~375 sites), §3 product semantics to decide, §4 incomplete
refactors and missing tests, §5 coordination / manual testing / rollout.

Use this to find the next ticket to pick up. Every `[ ]` is an open task,
every `[x]` is done with a brief description of what landed.

### [`bugs.md`](./bugs.md) — concrete bug triage

Detailed bug-by-bug report for the §1 items in `shipping_plan.md`. Each bug
has the exact file:line, a code excerpt, an explanation of the multi-tenant
failure mode, and a proposed fix shape. Ordered by impact: ship-blockers
first, sibling-clobbers second, with a suggested fix order at the end.

Read this before fixing any §1 bug.

### [`git_store_host_context.md`](./git_store_host_context.md) — git refactor state

Background on the `GitStore` portion of the refactor: why active-repository
state moved off the host store onto `Project::active_repository_id`, what
"leak" means in this codebase (project-resource leaks inside host stores,
not just GPUI entity leaks), the GPUI leak-detector support that exists, and
the shape of the property test that validates `GitStore` host sharing.

Read this before touching `GitStore` or anything that subscribes to
`GitStoreEvent`.

### [`git_store_host_future_questions.md`](./git_store_host_future_questions.md) — git open questions

Open product/semantics questions for `GitStore` that don't block the
immediate refactor: visible worktree sharing rules, remote/collab semantics,
invisible worktrees, async stale-result mitigation, shared-repo UI behaviour
across projects.

Read this when making a §3 product call that touches git.

### [`worktree_store_property_test.md`](./worktree_store_property_test.md) — worktree property-test direction

Invariants and operation set for a dedicated `WorktreeStore` property test
(the visible-worktree-sharing invariants that were pulled out of the
`GitStore` property test). Use this when implementing or extending that
test.

---

## Working on this branch

- `crates/project/tests/integration/multi_tenant.rs` is the scaffold for
  two-Project tests. Add the failing test there *before* fixing a bug.
- Fixes follow an established pattern: per-project `HashSet` on `Project`,
  filtered accessor on the store, ownership-gated handler. See the `[x]`
  notes under `shipping_plan.md` §4 for end-to-end examples (`DapStore`,
  `BreakpointStore`, `TaskStore`, `SettingsObserver`, `BookmarkStore`,
  `ImageStore`).
- The compiler is the audit tool for ownership-blind call sites. See
  `shipping_plan.md` §2 "Approach: let the compiler do the audit".
- Build hygiene: this repo uses `cargo build` (not `-p <crate>`) so the LSP
  shares the same cache. Don't run `./script/clippy` to validate work
  unless explicitly asked.
