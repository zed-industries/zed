# Node engine

GPUI is immediate-mode: `render` describes the whole frame every time it runs. The
node engine memoises views. Each mounted view occurrence is a **node** that keeps the
entities its render read, its Taffy subtree, and a recording of the frame effects it
produced (scene operations, hitboxes, listeners, dispatch nodes, text layouts). When a
frame is requested, a node whose inputs have not changed replays its recording instead
of rendering; dirty nodes rebuild and their ancestors rebuild with them.

This document records the decisions the design rests on and the work that remains.
The first half should stay true as the code changes; the second half is a plan and
should be edited by the PRs that complete it.

## Vocabulary

- **Node** – one mount of a view at one place in the tree. The same `Entity<V>`
  rendered in two places is two nodes.
- **Memoised** – a node's output was reused. Not "retained": the authoring model is
  unchanged, and "retained" is used only where Taffy subtrees are literally kept
  across frames.
- **Mounted / unmounted** – a node exists / has been removed. This is the lifecycle
  that local state, images, and hooks hang off.
- **Dirty** – a node must rebuild. **Notified** – an entity announced a change.
  These are different things; see below.

## Decisions

### Reads establish dirtiness; notify establishes change

While a node renders, every entity read is recorded as a dependency of that node.
`cx.notify(entity)` does what it always did: it marks the windows that read the
entity dirty and runs the entity's observers. It does not notify anything downstream.
When the window next draws, the engine expands the dirty set through its
source → consumer map and marks the consuming nodes, and their ancestors, dirty.

The only bridge between the two is the consumer map. Nothing in `App` walks render
dependencies, and no node is ever the target of `Effect::Notify`. A future change
that makes a view (rather than a node) a consumer must not start firing that view's
observers when a dependency changes.

### Notify is the contract

A view that mutates state without `cx.notify()` is stale until it notifies. The
engine does not compensate for missing notifications (no revision counters, no
"any `update` is a change" rule), because that rule cannot tell a read-only
`update` from a mutation and forces side doors like `ElementInputHandler::query`.
Missing notifications are application bugs, found with the oracle (below) and fixed
where they occur.

### Everything a render reads is tracked, in one of three ways

1. It is an entity read, recorded as a dependency.
2. It is part of `ViewNodeCacheKey` (bounds, content mask, text style, rem size,
   scale factor, opacity, image cache) and compared before reuse.
3. It is an ambient input whose mutation invalidates the nodes that read it.

There is no fourth category. Adding an ambient input to `Window` or `App` that
renders can observe means adding it to (2) or (3). Falling back to `Window::refresh`
is acceptable only as a stopgap and must be listed in the plan.

### Mount lifecycle

- A node is created the first time its occurrence renders and removed at the end of
  the first frame in which its parent rendered without it.
- Unmount happens in `finish_frame` / `reconcile_children`, after the frame, never
  mid-render.
- A full refresh rebuilds every recording but does **not** unmount. Local state
  survives `window.refresh()`.
- Node-local state (`use_keyed_state`, components) is dropped at unmount. Its
  entities' release callbacks are therefore the unmount hook for anything they own.

### Nodes are engine storage, not entities

`NodeId` is an engine-local slotmap key. `EntityId` is for things renders read.
Nodes have no refcounts, no observers, no access tracking, and cannot appear in
dependency sets. Keyed local state remains real entities, owned by the node.

### Identity is positional within a node

An element's identity is `(node, Location::caller(), nth occurrence)`, overridden by
an explicit key where children reorder. The full path — what `GlobalElementId`
encodes today — is computed on demand by walking node parents, for the inspector
and accessibility only.

### Components

A component (`render(&self, window, cx)`) is inputs plus the `use_state` entities it
creates. Its inputs belong to the parent and are replaced in place on every parent
render; it is not remounted. It has no state of its own to notify about, so it does
not receive its own `Context`. Memoisation is an opt-in wrapper that promotes the
component to its own node. There is no props equality; "parent rendered" means
"child renders".

### The oracle

The reference implementation is the node engine under full refresh. A frame produced
incrementally must equal the frame produced by `window.refresh()` from the same
state. `test_workspace_rendering_stress` in `editor` asserts this over a 3-pane
workspace; the helper and a gpui-only seeded fixture belong in gpui.

## Plan

Ordered by dependency. Items marked **critical path** unblock several others.

### Engine shape

- [x] **Critical path.** Store nodes in a `SlotMap<ViewNodeId, ViewNode>` inside
  `NodeEngine`; ancestor dirtiness through the node's `parent`. Deleted
  `NodeRenderDecision` (now `reuse` / `reuse_layout` returning `Option`), the
  access-scope pool (`App::track_reads` fills a caller-owned set), the `Window`
  forwarding layer, and node ids in dependency sets. Measured 715 → 699 µs on
  `editor_render`; the single-node fixture barely exercised entity-map traffic, so
  the remaining full-rebuild overhead is elsewhere. Still open: `consumers` values
  are `FxHashSet`s and could be `SmallVec`s; DFS-order node layout.
- [ ] Recordings own their listeners and input handlers; the frame holds
  `(NodeId, index)` references and dispatch resolves through node storage.
  `PlatformInputHandler` becomes a locator that resolves through its
  `AsyncWindowContext`. Removes both `Rc<RefCell<…>>` wrappers, the re-entrancy
  panic, and strong entity captures outliving the frame.
- [x] **Critical path.** Notify is the contract: removed entity revisions,
  `dependency_revisions`, `EntityMap::end_query`, `ElementInputHandler::query`.
  The oracle (gpui, editor at 200 stress steps, workspace, project/outline panel,
  terminal, title bar) surfaced no missing `cx.notify()` in those suites. Interactive
  use of Zed is the remaining discovery surface; a view that goes stale under the node
  engine is a mutation without notify at that site.
- [ ] Collapse the access API to one `cx.track_reads(|cx| …) -> (R, ReadSet)`.
  Remove `begin/end_access_scope`, `take/recycle_access_scope`,
  `suspend/restore_access_tracking`. `EntityMap::insert` must not record an access
  (creating is not reading), which removes the suspend in `fetch_asset`.
- [ ] Remove `begin/end_render_notifications`; at the end of draw, dirty the
  intersection of `pending_notifications` and the frame's accessed entities.
- [ ] Write frame effects into node recordings directly instead of the post-paint
  capture pass in `capture_view_node_recording`. Includes `record_layouts`, which
  currently re-hashes every line's `CacheKey` to pair keys with layouts.
- [ ] Profile the full-rebuild overhead on `benchmarks/editor_render` with the engine
  on and off, after the items above. Candidates in order: capture pass, cache-key
  `TextStyle` clone/compare, dependency-set bookkeeping, node reads, `retain()` /
  `layout_unchanged` walks.
- [ ] **Critical path.** Cut the legacy engine. `Option<NodeEngine>` becomes
  `NodeEngine`; delete the non-node branches of `ViewElement`, the duplicate
  `use_keyed_state`, and the `Window` forwarding layer. Tests wanting a reference
  frame force a refresh.
- [ ] `.cached(style)` becomes the ordinary node path with `style` refining the root
  layout; deprecate afterwards.
- [ ] Deferred draws and prompts mark the current scope frame-bound instead of
  forcing a whole-window refresh.
- [ ] Record positions relative to the node origin and translate on replay, so a
  clean subtree that moves is replayed rather than rebuilt. `layout_unchanged` then
  ignores the root's `location`.
- [ ] `uniform_list` and `list` use `request_retained_measured_layout` (their
  captures are plain values).
- [ ] `next_occurrence` uses a per-location counter instead of `siblings.contains`.
- [ ] Trim `TaffyLayoutEngine` bookkeeping: upstream node iteration to Taffy so
  `allocated_nodes` can go; `stale_layouts` becomes a local; `previous_layouts` is
  only the layout snapshot, not also the reachability marker in `retain()`;
  `layout_inputs` lives with the node whose root it describes.

### Correctness

- [ ] A `Graft` decision that falls through to render (no previous layout) must
  call `restart_render`; otherwise `next_children` is stale.
- [ ] Replace `expect` with a fallback render in `replay_scene`, `replace_layout`,
  and similar paths where reuse can simply be declined.
- [ ] Track remaining ambient inputs as dependencies instead of `refresh()`: focus,
  window active state, viewport size, mouse position. Focus is the largest source of
  full rebuilds. Globals are already tracked.

### Identity on the node tree

- [ ] Element identity → `(node, Location::caller(), nth)`; `GlobalElementId` shrinks
  to the local suffix and full paths are computed on demand.
- [ ] Inspector: identity and per-element overrides on node state; overrides read as
  entities so edits dirty exactly one node. Deletes the inspector full-refresh
  fallback.
- [ ] Accessibility: stable per-mount ids; partial `TreeUpdate`s for rebuilt
  subtrees. Deletes the accessibility full-refresh fallback.

### Capabilities the lifecycle enables

- [ ] Images: node-local `Entity` holding the load; completion notifies; release
  calls `drop_image`; a per-window `WeakEntity` lookup shares tiles between nodes.
  Retire `ImageCache`'s lifecycle logic; keep a decode cache as a plain LRU.
- [ ] `window.on_unmount` for plain `Entity<V>` views (components already get it via
  `on_release`).
- [ ] GPU damage regions from `changed_bounds` through submission and presentation,
  with backend buffer-age handling.
- [ ] Unify `RenderOnce` and `Component` into one `render(&self)` trait: default is
  inline and re-rendered with the parent; `memo(key, value)` opts into a node.
  Mechanical migration of `#[derive(IntoElement)]` and the `ui` crate.

### Testing and housekeeping

- [ ] Oracle helper on `VisualTestContext` (`assert_incremental_matches_full_refresh`)
  and a gpui-only fixture (nested views, `uniform_list`, wrapped text, focus, hover,
  scroll, deferred popover, resize) driven by a seeded step sequence. Keep
  `test_workspace_rendering_stress` as a consumer; tune its step count for CI.
- [ ] Resolve the `gpui::Component` / `component::Component` name clash before the
  trait is public.
