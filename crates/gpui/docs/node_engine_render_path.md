# The node engine's render path, step by step

A reading guide to what happens when a window draws a frame under the node engine, for
anyone who wants to reason about its cost. It follows one frame top-down, then one view
node through its three phases, and ends with where the per-node time goes and what is
left to take out. Paths are relative to `crates/gpui/src/`; names are the functions to
read. The companion `node_engine.md` records the decisions and the plan; this document
is about the mechanics.

## 1. The shape of a frame

```
Window::draw                                   (window.rs)
  invalidate_entities                          notified entities -> dirty nodes
  draw_frame
    begin_node_engine_frame                    full refresh? all dirty? drop layouts
    draw_roots
      root_element.request_layout              phase 1: LAYOUT   (mount nodes, reuse or render)
      root_element.prepaint_as_root            phase 2: PREPAINT (compute layout, hitboxes, dispatch nodes)
      prepaint_deferred_draws                  deferred roots, in priority order
      prompt / drag / tooltip prepaint         more roots
      root_element.paint                       phase 3: PAINT    (scene, listeners, cursor styles)
      paint_deferred_draws, prompt/drag/tooltip paint
    finish_node_engine_frame                   reconcile roots, retire layout trees
    swap rendered_frame <-> next_frame         events now dispatch against this frame
```

The engine (`node_engine.rs`, `NodeEngine`) is owned by the window. It holds:

- `nodes: SlotMap<ViewNodeId, ViewNode>` — one node per mounted view occurrence. A node
  keeps its `output: NodeOutput` (the recording), `layout: Option<LayoutId>` (root of its
  retained Taffy subtree), `cache_key` (ambient inputs it was drawn under), `parent`,
  `children`, `accessed_entities` (what its render read), `view_id` (the entity whose
  notify re-renders it), `painted`.
- `roots` / `next_roots` — the frame as an ordered list of root nodes (window root, then
  deferred roots by priority, then prompt / drag / tooltip). Queries walk this list.
- `occurrences: FxHashMap<ViewOccurrence, ViewNodeId>` — how a mount finds its node again:
  keyed by `(element path, parent node, nth)`.
- `consumers: FxHashMap<EntityId, FxHashSet<ViewNodeId>>` — reverse of
  `accessed_entities`: which nodes read a given entity.
- `dirty_nodes`, `frame_bound_nodes`, `mounted_this_frame` — per-frame sets.
- `traversal_stack: Vec<(ViewNodeId, MetadataPhase)>` — the nodes being drawn, innermost
  last. Everything pushed while drawing lands in the innermost node's output for the
  current phase.

## 2. Dirtiness: from `cx.notify()` to a dirty node

```rust
// app.rs
pub fn notify(&mut self, entity_id: EntityId) {
    // ... for each window currently displaying the entity:
    invalidator.invalidate_view(entity_id, self);   // adds to the invalidator's set of notified entities
}
```

Nothing happens to nodes at notify time. When the window next draws:

```rust
// window.rs
fn invalidate_entities(&mut self) {
    let mut views = self.invalidator.take_views();        // the notified entity ids
    self.node_engine.invalidate_entities(&views);          // -> dirty nodes
    views.clear();
    self.invalidator.replace_views(views);
}

// node_engine.rs
pub(crate) fn invalidate_consumers(&mut self, source: EntityId) {
    let Some(consumers) = self.consumers.get(&source) else { return };
    for consumer in consumers {
        let mut node_id = Some(*consumer);
        // A parent's output contains its children's; stop at the first node already dirty.
        while let Some(id) = node_id && self.dirty_nodes.insert(id) {
            node_id = self.nodes.get(id).and_then(|node| node.parent);
        }
    }
}
```

So: **reads establish dirtiness** (a node is a consumer of every entity its render read,
including its own view entity), and **notify establishes change**. A dirty node and all
its ancestors rebuild; clean siblings are replayed. If a notified entity has no recorded
consumer at all, every node is dirtied (nothing says which output depended on it).

Cost, per notified entity: one `consumers` lookup, one `dirty_nodes` insert per node on
the path to the root (stopping early), plus the pre-existing `mark_view_dirty` walk.

## 3. Beginning a frame

```rust
// window.rs
fn begin_node_engine_frame(&mut self) {
    let full_refresh_reason = /* window.refresh(), image eviction, prompt, a11y, inspector */;
    self.node_engine.begin_frame(full_refresh_reason);     // full refresh => all nodes dirty
    if self.node_engine.discard_dirty_layouts() {          // every node dirty?
        self.layout_engine.clear();                         // then nothing can reuse a layout: drop the tree
    }
}
```

`begin_frame` resets per-frame state (`mounted_this_frame`, `changed_bounds`) and, on a
full refresh, extends `dirty_nodes` with every node. A focus change, hover change or
resize still calls `window.refresh()` today, so those frames rebuild everything; they
cost what every frame cost before the engine.

## 4. One node, phase 1: layout

A view becomes an element through `ViewElement<V>` (`view.rs`). `V: View` is the sealed
trait behind `Entity<T: Render>`, `AnyView`, `Component` and `RenderOnce`; a view with an
`element_id()` mounts as a node, one without renders inline. `Drawable` (`element.rs`)
wraps every element and, for one with an id, pushes the id on `element_id_stack` and
pushes a dispatch node before calling `request_layout`.

```rust
// view.rs, ViewElement::request_layout, node path (abridged)
let cache_key = window.view_node_key(Bounds::default());        // (a) ambient inputs, no bounds yet
let node_id = window.begin_node_occurrence(id.clone(), &cache_key);   // (b) find or create the node
let mut owned = window.node_engine.take_owned_entity(node_id);       // (c) component instance, if any
let entity_id = view.entity(&mut owned, window, cx);
window.node_engine.store_owned_entity(node_id, owned);
window.node_engine.set_view_id(node_id, entity_id);

if let Some(layout) = window.node_engine.reuse_layout(node_id, &cache_key)   // (d) clean, painted, key matches?
        .filter(|layout| window.layout_is_retained(*layout)) {
    window.finish_node_phase(node_id, false);               // grafted: nothing rendered
    return (layout, None);
}

window.restart_node_render(node_id);                        // (e) reseed text (unless cached last frame), reset output
let mut reads = window.node_engine.take_dependency_set();
let (layout, element) = cx.track_reads(&mut reads, |cx| {  // (f) record entity reads
    window.with_rendered_view(entity_id, |window| {
        let mut element = view.render(window, cx).into_any_element();   // the user's render
        (element.request_layout(window, cx), element)                   // children mount here, recursively
    })
});
let previous = window.node_engine.store_layout(node_id, layout);    // (g) new Taffy root
window.retire_layout(previous);                                     //     drop the old subtree
window.finish_node_phase(node_id, true);
```

What each step costs, per node:

- (a) `view_node_key`: refines the `text_style_stack` into a `TextStyle` (a clone with a
  `SharedString` font family and an `Arc` of font features), reads rem size, scale,
  opacity, content mask, image cache. Done again at prepaint with the real bounds.
- (b) `begin_occurrence` → `next_occurrence` builds `ViewOccurrence { element: GlobalElementId
  (Arc<[ElementId]> clone), parent, index }` and probes `occurrences` (hashing the whole
  element path) until it finds an id not yet in `mounted_this_frame` (another hash set),
  inserts into `mounted_this_frame`, pushes onto the parent's `next_children`, and
  `splice`s: pushes `OutputItem::Child(node, Layout)` into the parent's output and the
  node onto the traversal stack. Also `begin_text_use` on the text system.
- (c) two `SlotMap` lookups; only components use the owned entity.
- (d) `reuse_layout`: `dirty_nodes.contains`, `frame_bound_nodes.contains`,
  `cache_key.matches` (compares the two `TextStyle`s field by field).
- (e) `restart_node_render`: for each of the three phases, take the node's `TextUse`, seed
  its `(Arc<CacheKey>, Arc<LineLayout>)` pairs back into the frame's text cache (a hash
  insert per line), hand the buffers back; then `restart_render` clears the three item
  vectors, `dispatch_pushes`, `accessed_element_states`, `inline_views`, bumps
  `generation`.
- (f) `track_reads` opens an access scope on the entity map: every `entity.read(cx)` /
  `update` during the render inserts into an `FxHashSet<EntityId>`. `with_rendered_view`
  pushes the entity on `rendered_entity_stack` (for `use_state` and the dispatch tree).
- (g) `store_layout` swaps the root and returns the previous; `retire_layout` walks the
  previous subtree and removes it from Taffy (skipping descendants that are other nodes'
  retained roots, which the render has already re-attached under the new root).
- `finish_node_phase(rendered)`: `end_text_use` (pop the `TextUse`), store it in the
  node's phase, pop the traversal stack.

A **grafted** node (d) skips (e)–(g): it costs the key, the occurrence lookup, and the
extend of the parent's read set.

## 5. Phase 2: prepaint

Between layout and prepaint the window calls `compute_layout` on the root: Taffy lays out
the whole tree (retained subtrees included; Taffy caches per node, so unchanged subtrees
are cheap). Measured leaves (text) run their measure closures here, outside every node's
traversal, which is why `request_retained_measured_layout` wraps each measure in its own
`begin/end_text_use` and appends the result to the requesting node.

```rust
// view.rs, ViewElement::prepaint, node path (abridged)
let cache_key = window.view_node_key(bounds);            // now with bounds
window.set_view_id(entity_id);
window.enter_node_prepaint(node_id);                     // splice Child(node, Prepaint), begin_text_use
if grafted {
    if node.cache_key.matches(&cache_key, false) && window.retained_layout_unchanged(layout) {
        window.graft_view_node_prepaint(node_id);        // replay: push recorded dispatch nodes, re-attach deferred roots
        window.finish_node_phase(node_id, false);
        return Graft;
    }
    window.restart_node_render(node_id);                 // box moved or ambient input changed: render after all
}
let element = cx.track_reads(&mut reads, |cx| {
    let element = element.or_else(|| render + request_layout + replace_retained_layout);
    element.prepaint(window, cx);                        // hitboxes, dispatch nodes, deferred draws, tooltips
    element
});
window.finish_node_phase(node_id, true);                 // reconcile_children: unmount what did not come back
```

`retained_layout_unchanged` compares the node's root `taffy::Layout` with the one recorded
when it last drew (`TaffyLayoutEngine::root_layouts`). A subtree's layout is a function of
its root's box and its own retained styles, so the root is all that needs comparing.

Everything an element does in prepaint becomes an `OutputItem` in the node's prepaint
output: `insert_hitbox` → `Hitbox`, `Drawable::prepaint` → `DispatchPush`/`DispatchPop`
(the live dispatch node id plus an index into the node's `dispatch_nodes` lane),
`defer_draw` → `Root(node, priority)` (and a `DeferredDraw` queued for the deferred pass),
tooltips, tab stops. Items are 88 bytes; a `div` with an id pushes two dispatch items and
one hitbox.

Replaying a grafted node's prepaint (`graft_view_node_prepaint`) walks its prepaint items
and, for each `DispatchPush`, pushes the *recorded copy* of the dispatch node
(`push_recorded`) into this frame's dispatch tree; for each `Root`, re-queues the
deferred root under the dispatch node that is active at that point.

## 6. Phase 3: paint

```rust
// view.rs, ViewElement::paint, node path (abridged)
window.enter_node_paint(node_id);
match state {
    Graft => window.graft_view_node_paint(node_id),       // replay the node's scene into the frame scene
    Render { cache_key, reads } => {
        window.begin_view_node_paint(node_id);            // take the node's scene buffers, start recording
        cx.track_reads(&mut reads, |cx| element.paint(window, cx));
        window.finish_view_node_paint(node_id);           // store scene; snapshot_dispatch_nodes
        window.store_node_render(node_id, cache_key, reads);   // record root layout; store_render
    }
}
window.finish_node_phase(node_id, rendered);
```

- The scene: `Scene::record_operation` writes every primitive both into the frame's
  primitive arrays and into the node's `ViewNodeScene` recording (child nodes are recorded
  as `Child` segments). Replay copies a recording into the frame arrays
  (`replay_recording`).
- `snapshot_dispatch_nodes`: paint adds listeners and key contexts to the dispatch nodes
  prepaint pushed, so after paint the node copies (`clone_from`) each of its live dispatch
  nodes into its `dispatch_nodes` lane. This is what replay pushes back next frame. Nodes
  that stayed empty (most elements') are dropped from the recording along with their
  push/pop items, since a walk of the dispatch tree cannot tell they were there.
- `store_render`: adds the view entity to the read set, swaps it with the node's previous
  set, and diffs the two into `consumers` (`replace_dependencies`; a no-op when equal),
  drops element states the render did not access, clears `dirty_nodes` for the node,
  unions the old and new bounds into `changed_bounds`.

## 7. Ending a frame

```rust
// node_engine.rs
pub(crate) fn finish_frame(&mut self) -> Option<Bounds<Pixels>> {
    swap(&mut self.roots, &mut self.next_roots);      // next_roots was filled in drawing order
    for stale in old roots not in roots { self.remove_subtree(stale) }   // unmount
    self.full_refresh = false;
    self.last_frame_stats = ...;
}
```

Then the window drops the layout trees the engine retired this frame (removed nodes,
frame-bound nodes) and the few Taffy nodes requested outside any node, and the text
system swaps its frame caches. `reconcile_children` ran earlier, at each rendered node's
prepaint end: children in `children` but not `next_children` are removed, recursively,
which also removes their dependencies from `consumers` and collects their layout roots.

## 8. Queries against the rendered frame

Every query is a walk over `roots` in drawing order, descending into `Child` splices:

```rust
// node_engine.rs
pub(crate) fn walk(&self, frame: FrameOutput, mut visit: impl FnMut(OutputSlot, &OutputItem) -> ControlFlow<()>) {
    for phase in [Layout, Prepaint, Paint] {
        for root in self.frame_roots(frame) { self.walk_output(*root, phase, &mut visit)?; }
    }
}
```

`hit_test` (reverse, hitboxes), `mouse_listeners` (forward, leased out of their slot for
the call and put back), `cursor_style`, `focused_input_handler`, `tab_stops`,
`prepaint_tooltip`. On the 512-node fixture each walk is 0.15–0.2% of the frame; on a
real window (10–30k items) perhaps 10–30 µs per query, two or three per mouse event.

## 9. Where the per-node cost goes

Measured on `Siblings/all dirty/512` (512 views, ~3 µs each to render, all dirty every
frame): ~0.6 µs per node per frame over `main`, i.e. +20% (64 nodes: +18%). By ablation
of the ~330 µs of overhead per frame, before the passes listed below:

| mechanism | share | notes |
| --- | ---: | --- |
| text-use recording | 15% | 3 `begin/end` pairs per node + 1 per measured leaf |
| dependency read tracking | 7% | `FxHashSet` per render, `consumers` diff |
| dispatch-node snapshot | 7% | `clone_from` of 2 dispatch nodes per leaf after paint |
| cache-key `TextStyle` | 3% | built twice per node |
| scene recording | 1% | |
| node lifecycle | ~67% | occurrence lookup (path hash ×2 sets), output reset, item pushes (88 B memmoves), dirty propagation, frame walks, per-`ViewElement` dispatch push, `Arc` handle churn in text uses, `ViewNode`/`TextUse`/`ElementDrawPhase` moves and drops |

Self-time, by symbol, of what the branch adds: `memmove` +1.5%, `Arc` refcount ops
+1.9% (text-layout handles), entity-id hashing +1.4%, malloc/free +0.5%, then a long
tail of engine functions at 0.1–0.4% each.

Already removed: a quadratic `reconcile_children`; the per-frame Taffy reachability walk
with a full-tree layout snapshot (now incremental retention and root-only comparison);
an O(nodes) "is everything dirty" probe; `TextUse` reallocations; the text system's
locks; reseeding text the cache still holds from last frame; bubbling child reads into
the parent's dependency set (the window's tracked entities now come from `consumers`);
copying and replaying empty dispatch nodes; the window's unread `dirty_views` set. The
passes since the ablation took 512 from +22% to +20% and 64 from +21% to +18%.

Also fixed on the way, found by review of the retention protocol: a tree laid out with
`layout_as_root` inside a node (list items, editor blocks, the measured row of a
`uniform_list`) hangs off nothing the node retains, so retiring the node's root never
reached it and it leaked one tree per item per frame between full refreshes.
`Window::compute_layout` now marks such orphan roots as frame layout, and
`TaffyLayoutEngine::finish_frame` removes them unless a node painted with them
(`layout_trees_measured_inside_a_node_do_not_accumulate`).

## 10. What is left to take out, in order

1. **Text uses off the text system entirely.** Record `(key, layout)` handles on the
   window's traversal stack instead of a stack inside the text cache, so a node's use
   is a slice of one frame-level vector and the `Arc` handles are moved, not
   cloned-and-dropped per node. Removes the per-node `begin/end` pairs and most of the
   `atomic_sub`.
2. **Key nodes on the refinement stack, not a materialized `TextStyle`.** Compare the
   `text_style_stack` entries (or a running hash of them) instead of refining into a
   `TextStyle` twice per node.
3. **Dependency sets as small sorted vectors.** Nodes read 1–3 entities; a `SmallVec`
   with linear insert beats a hash set, and `replace_dependencies` becomes a merge.
4. **A node-owned dispatch tree**, which removes the snapshot altogether and gives
   stable dispatch node ids.
5. **Occurrence keys without path hashing.** A per-parent counter per element id, or
   keying occurrences by `(parent, last ElementId, nth)` since the parent already fixes
   the prefix.
6. **One dispatch push per `ViewElement`, not two.** `Drawable` pushes a dispatch node
   for the `ViewElement` and the view's root `div` pushes another.
7. **Per-node flags instead of the `dirty_nodes`/`frame_bound_nodes`/`mounted_this_frame`
   sets**, with counters for the stats.

Each is a bounded change under the same contract; the oracle tests
(`node_engine::oracle_tests`, `test_workspace_rendering_stress`) are the check. The
fixture and the paired-benchmark protocol are in `node_engine.md` under "Measuring".
