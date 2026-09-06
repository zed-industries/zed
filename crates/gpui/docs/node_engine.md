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

There are two things a render can put in the tree. **Entities** you can refer to, so
you manage them, and notify is their contract. **Components** (`Component`,
`render(&self, window, cx)`) are managed for you and cannot be referred to: a value
built by the parent's render, plus the `use_state` entities it creates. It has no state
of its own to notify about, so it gets `&mut App`, not a `Context`.

A component renders inline by default, as part of its parent's node, in an element-id
scope of `(type name, nth inline view of that type in the node)` — Flutter's
type-and-position identity — so two siblings of one type keep separate `use_state`.
(Every inline view, `RenderOnce` included, gets this scope; it fixes the sibling
collision `RenderOnce` had.) `.cached()` mounts a component as a node of its own with
an entity holding the value, and requires `PartialEq`: when the parent renders again,
the node is re-rendered only if the new value differs from the one it last rendered, or
something it read was notified. That is SwiftUI's `.equatable()`; it is the one place
props equality exists, and the type system makes it opt-in, since callbacks cannot be
compared and a hand-written `PartialEq` has to choose to skip them.

Where `.cached()` shows up in practice is the data for deciding what to promote
automatically. Auto-promotion would key on local state (`use_state`) only; promoting
on entity reads would promote everything, since every render reads the theme.

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
- [x] Recordings own their listeners and input handlers (`OutputItem::MouseListener`,
  `OutputItem::InputHandler`, leased out of their slot for a call). `PlatformInputHandler`
  resolves the rendered frame's input handler through its context on every call. A
  nested mouse-event dispatch runs no listeners, as before the node engine.
- [x] **Critical path.** Notify is the contract: removed entity revisions,
  `dependency_revisions`, `EntityMap::end_query`, `ElementInputHandler::query`.
  The oracle (gpui, editor at 200 stress steps, workspace, project/outline panel,
  terminal, title bar) surfaced no missing `cx.notify()` in those suites. Interactive
  use of Zed is the remaining discovery surface; a view that goes stale under the node
  engine is a mutation without notify at that site.
- [x] Collapse the access API to `cx.track_reads(&mut set, |cx| …)`;
  `begin/end_access_scope` remain as its private halves. Still open:
  `suspend/restore_access_tracking` is used once, around a render's notifications
  (`render_notifications`), which could instead dirty the intersection of
  `pending_notifications` and the frame's reads at the end of draw.
- [x] Profile the full-rebuild overhead. `Workbench/update/full` (the only multi-node
  fixture) was 12.5% slower than `main` on this machine; the cause was the line layout
  cache evicting text that reused views never looked up, so every rebuild reshaped it
  (CoreText at 12.5% of samples vs 0.9% on `main`). Resolved by node-owned text (below).
  Engine bookkeeping measures at ~1.5% on `editor_render`.
- [x] **Nodes are the frame.** Each node owns what it drew, per phase, in one
  `Vec<OutputItem>` (hitboxes, tooltips, cursor styles, window controls, tab stop
  operations, mouse listeners, input handlers, dispatch push/pop, test debug bounds),
  with `Child(node, phase)` items marking where a child's output belongs, plus element
  states, the text it looked up, its scene, and a lane of recorded dispatch nodes. A
  reused node's output stays where it is; a redrawn node overwrites its own. Queries
  (`hit_test`, `cursor_style`, `mouse_listeners`, `focused_input_handler`,
  `hit_window_control`, `tab_stops`) walk the tree in drawing order or reverse. The
  frame root — output drawn outside every node (deferred draws, `VisualTestContext::draw`)
  and the splices to the root nodes — is the one part rebuilt every frame, so it keeps a
  rendered/next pair swapped where the frames swap; once deferred draws are nodes and the
  test draw wraps its element in one, that pair collapses into `roots`/`next_roots`.
  `Frame` is down to focus, window-active, the dispatch tree, deferred draws, and the
  scene linearization. Gone: `RecordedMetadata`, `capture_metadata`, `PaintIndex`,
  `PrepaintStateIndex`, `ViewNodeRecording`, `LineLayoutRecording`, `record/replay_subtree`,
  `assert_metadata_unique`, `Frame::finish`'s state carry-over, `NodeLocalState`.
  - `DispatchTree` is unchanged from before the node engine: a flat per-frame tree with
    unstable ids. A node records its pushes and pops; once it has painted, the recorded
    nodes are refreshed from the live tree, and reuse walks the items pushing them back
    under the active node. Making the dispatch tree itself node-owned is a possible
    follow-up; it is not needed for correctness or the measured performance.
  - `use_state`/`use_keyed_state` are `with_element_state` storing
    `(Entity, Subscription)`; there is one element-state map, per node.
  - Text: the node is the retention. Each phase holds the `(key, layout)` pairs its
    elements looked up (`TextUse`), including text shaped inside Taffy measure closures
    (attributed to the node that requested the measured layout, since measuring runs
    outside the traversal). A redraw seeds the node's text into the frame cache first.
    The cache is back to one previous frame, as before the node engine.
  - One flat list vs. lanes: measured on `Workbench/update/full` (223 elements, 814
    items, `OutputItem` 88 bytes after moving dispatch node snapshots to their own lane),
    a hit test walks the frame in ~0.85µs and collecting mouse listeners ~1.2µs, about
    1ns per item; items are ~55% dispatch push/pop, ~30% mouse listeners, ~10%
    hitboxes. A real window is perhaps 10–30k items, so ~10–30µs per query and two or
    three queries per mouse event. Per-kind lanes with a shared splice table would let a
    hit test touch only hitboxes (~10× less) at the cost of a counts table per splice;
    do it if a walk shows up in a profile of real use, not before.
- [ ] Any per-frame-"use" cache in GPUI (atlas tiles; text is now node-owned) is a
  proxy for "still on screen" that reused views do not refresh. Audit for the same
  eviction pattern.
- [x] **Critical path.** Cut the legacy engine. `Option<NodeEngine>` becomes
  `NodeEngine`; delete the non-node branches of `ViewElement`, the duplicate
  `use_keyed_state`, and the `Window` forwarding layer. Tests wanting a reference
  frame force a refresh (`NodeEngine::new_eager` remains as the test reference).
- [x] `.cached(style)` becomes the ordinary node path with `style` refining the root
  layout; deprecate afterwards.
- [x] Deferred draws mark the current scope frame-bound instead of forcing a
  whole-window refresh. Prompts, accessibility, and the inspector still refresh.
- [ ] Deferred draws as nodes. A deferred draw is a root pinned to a position whose
  owner relationship is lifecycle, not containment. The owner's recording stores a
  `DeferredChild { node, priority, offset, ambient context }`; replay re-schedules it
  into the deferred pass, where the node decides reuse itself. Applies when the payload
  is an entity view (it can re-render itself); `deferred(anchored().child(view))` keeps
  the owner frame-bound until either anchoring moves into the view or fine-grained
  caching can re-run `Anchored` in its recorded context.
- [ ] Record positions relative to the node origin and translate on replay, so a
  clean subtree that moves is replayed rather than rebuilt and the cache key becomes
  size-only. `position: absolute` children resolve inside the node and deferred draws
  re-run through their frame-bound owner, so both stay correct. Everything recorded
  with a position needs the translate: scene primitives (paths per vertex, or an
  insertion offset), hitboxes, input-handler bounds, tooltip and cursor requests.
  `layout_unchanged` then ignores the root's `location`. After ownership and identity.
- [ ] Fine-grained caching: a dirty descendant rebuilds through clean ancestors.
  Containment, read dependencies, and layout consequences become three separate
  relations. Replay is mostly ready (recordings reference children by id); what is
  missing is the ambient context at each child reference (element offset, dispatch
  parent, element-id stack, image cache) and layout change reporting from Taffy (try
  upstream before forking) to schedule the scopes a relayout actually moved. Keep the
  parent walk until change reporting exists; validate with the oracle across flex
  reflow, unchanged outer bounds with inner changes, parent movement, clipping, and
  mount/unmount. Measure with `Workbench/update/row`: rebuilding a tiny node alone can
  cost more than its parent's rebuild saved. After ownership, identity, and offsets.
- [ ] `uniform_list` and `list` use `request_retained_measured_layout` (their
  captures are plain values).
- [ ] `next_occurrence` uses a per-location counter instead of `siblings.contains`.
- [x] Trim `TaffyLayoutEngine` bookkeeping. `stale_layouts` is gone (stale nodes are
  removed in one reverse pass over `allocated_nodes`) and `clear_retained` folded into
  `clear`. What remains is intrinsic: Taffy has no node iteration, so `allocated_nodes`
  stays; `previous_layouts` is the snapshot of the kept subtrees, whose key set *is* the
  kept set, so using it as the marker in `retain` is one structure rather than two;
  `layout_inputs` holds one entry per live root.

### Correctness

- [x] A grafted layout whose bounds or ambient inputs then differ at prepaint calls
  `restart_render` before rendering, so `next_children` and the dependency set start
  clean.
- [ ] The `expect`s in `taffy.rs` (`replace_layout`: "retained layout was computed
  before prepaint") are engine invariants, kept as panics so a violated invariant is
  found rather than papered over by a fallback render.
- [ ] Track remaining ambient inputs as dependencies instead of `refresh()`: focus,
  window active state, viewport size, mouse position, input modality, hover. Focus is
  the largest source of full rebuilds. Globals are already tracked. Cut from the first
  PR: a full refresh costs what every frame cost before the node engine, so these are
  a missed win rather than a regression. Sketch: an `AmbientInput` read set per node
  (`Focus`, `WindowActive`, `ViewportSize`, `MousePosition`, `Hover(HitboxId)`),
  recorded through a `Cell` since the readers take `&Window`; the same
  dirty-then-ancestors expansion as entities, keyed by input. Mouse moves diff the
  hovered hitbox set (hitbox ids are stable across reuse) and dirty readers of hitboxes
  that entered or left it, which retires the `refresh()` in `div`'s hover listeners; a
  modality flip dirties every hover reader. After a layout change moves elements under
  a still mouse, the post-prepaint hit test must run the same diff and schedule a frame.

### Identity on the node tree

- [ ] Element identity → `(node, Location::caller(), nth)`; `GlobalElementId` shrinks
  to the local suffix and full paths are computed on demand.
- [x] Separate mount identity from state identity. `View::element_id()` says where a
  node mounts and `View::entity()` which entity backs it; a cached component mounts by
  `(type, nth)` and owns its instance entity, so it never impersonates its inputs. An
  `Entity<T: Render>` uses its own id for both, as before the node engine; a repeated
  mount of one entity gets the next occurrence, and element state is per node, so
  sibling mounts of one entity keep separate recordings and local state.
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
- [x] `Component`: inline by default, `.cached()` mounts a node and compares inputs
  with `PartialEq`. See Decisions.
- [ ] Retire `RenderOnce` in favour of `Component` (separate PR): mechanical migration
  of `#[derive(IntoElement)]` and the `ui` crate, then deprecate. The `component`
  crate's preview-registry trait shares the name; disambiguate imports as they bite.
- [ ] Auto-promote components that hold local state to nodes, once `.cached()` usage
  shows where it pays. Needs same-frame re-render of a promoted node and state
  hand-off from the parent's element-state map.

### Testing and housekeeping

- [x] Oracle helper `VisualTestContext::assert_incremental_matches_full_refresh`;
  `test_workspace_rendering_stress` in `editor` is its consumer (48 steps by default,
  `GPUI_STRESS_STEPS` to raise it).
- [ ] A gpui-only oracle fixture (nested views, `uniform_list`, wrapped text, focus,
  hover, scroll, deferred popover, resize) driven by a seeded step sequence.
