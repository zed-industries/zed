# View-node draw engine prototype

Set `GPUI_EXPERIMENTAL_NODE_ENGINE=1` before creating a window to select the
prototype. The choice is made once in `Window::new`; there is no runtime engine
swap.

Run the sibling demo with:

```sh
GPUI_EXPERIMENTAL_NODE_ENGINE=1 cargo run -p gpui --example node_engine_siblings
```

The demo has three cached sibling views. The middle view notifies once per
second and prints each render invocation. After the cold frame, only the middle
view should continue printing.

## What is real

`NodeEngine` uses a dedicated `SlotMap<ViewNodeId, ViewNode>`. A GPUI entity was
not used for node storage because nodes are framework-private cache records with
window-local lifetimes, and creating entities would make node reads participate
in the application dependency graph. The slotmap also makes occurrence identity
separate from `EntityId`.

Each retained occurrence is keyed by its `GlobalElementId`, which contains the
full element-ID path for that window. Therefore the same `AnyView` can occupy
multiple slots without sharing a node. A node stores its parent, reconciled
children, occupying `AnyView`, current and previous bounds, cache inputs,
dependencies, and recording.

The entity map has a scoped access-collector stack. Reads enter the innermost
scope; popping a scope unions its reads into its parent. Repeated reads at nested
boundaries are retained, unlike the old frame-global set-difference helper. The
frame-global accessed set is still populated for the legacy window invalidation
machinery.

The node engine maintains an `EntityId -> ViewNodeId` occurrence multimap.
Window invalidations are drained into that map. A node depends on its own entity
and every entity read while recording. Clean occurrences graft; dirty
occurrences render and replace their recording. Removed child occurrences are
deleted recursively during child reconciliation.

The node-owned recording contains:

- a standalone `Scene`;
- hitboxes;
- tooltip request slots;
- cursor-style requests;
- the prepaint and paint lane positions needed by the adapters described below.

Scene composition replays the node-owned `Scene`, not the previous frame's scene
range. Damage is the union of every redrawn node's old and new bounds and is
logged at info level. Presentation remains unchanged and does not use the damage
rectangle.

The locality test has three sibling view occurrences and a dependency read by
only one sibling. It verifies both direct notification and dependency
notification rerender only that sibling. The same test drops every node,
performs a cold redraw, and compares all GPU-facing scene lanes with the
retained result.

## Prototype limits and adapters

Only entity-backed views explicitly rendered with `Entity::cached` or
`AnyView::cached` become retained nodes. This is a correctness boundary, not a
performance claim. Uncached views derive layout from their rendered element
tree, and Taffy is still cleared each frame. Skipping those render functions
would require retaining either layout inputs or semantic elements, neither of
which this prototype does. Root/container render functions therefore still run
while cached leaf render functions can be skipped.

Listener closures, platform input handlers, dispatch nodes, tab-stop insertion
history, deferred draws, accessed element-state entries, and text-layout leases
are move-only in the current `Frame` representation. Nodes do not yet own stable
forms of those lanes. `graft_view_node_prepaint` and
`graft_view_node_paint` use the existing previous-frame ranges to move or replay
them while the node owns the copyable lanes listed above. The adapter is marked
`TODO(node-engine)` in code. Calling this a fully owned multi-lane recording
would be dishonest.

Nested access scopes union child reads into their parent. With the current
layout/prepaint entry points, this means a cached parent depends on reads made by
its cached descendants. A dirty descendant can consequently dirty and rerender
that cached parent. Direct cached siblings still demonstrate locality, but this
prototype does not independently enter a damaged descendant through a clean
retained ancestor.

`Window::refresh`, prompts, an active accessibility tree, inspector presence,
and a previously recorded deferred draw force full node damage. Geometry
changes invalidate the node through its cache key. The prototype does not
translate recordings, preserve Taffy across frames, perform partial present, or
track global dependencies precisely.

The existing element-state map remains the semantic state store for both
engines. Nodes do not own or migrate element state.

## What fought back

The existing view cache looks close to a node because it has bounds, dependency
reads, and prepaint/paint ranges. The ranges hide an important ownership
constraint: several frame lanes contain mutable closures or platform-owned
objects and cannot be cloned into both a retained node and the active frame.
Moving those values each frame is also entangled with refreshed dispatch-node
IDs and text cache lease accounting.

Layout is the other hard boundary. GPUI normally renders an uncached view before
prepaint because its element tree supplies Taffy inputs. A node that retains
only draw output cannot relayout that view without rerendering it. Definite-size
cached views already provide the seam needed for an honest render-skipping
prototype.

`EntityId` is not an occurrence identity. `GlobalElementId` was already the
window-local path needed by the prototype, while the slotmap supplies stable
handles for parent/child and dependency indices.

## Production direction

A production engine should:

1. Retain layout inputs or a layout-only representation so every stateful view,
   including the root, can be entered without rerunning clean render functions.
2. Store recordings as segments around child boundaries. A dirty descendant can
   then replace its segment and propagate a recomposed recording through clean
   ancestors without invoking their render functions.
3. Replace frame-relative listener, dispatch, tab-stop, deferred-draw, and text
   indices with stable owned handles or lease objects.
4. Separate a node's direct reads from descendant reads. Descendant damage
   should schedule traversal through ancestors without treating every ancestor's
   render function as dirty.
5. Reconcile explicit child slot keys rather than relying on
   `GlobalElementId` as both occurrence path and prototype slot key.
6. Preserve Taffy state, compute geometry damage after layout, and feed damage
   to partial presentation.
7. Define production fallbacks for accessibility, inspector overlays, prompts,
   drag state, tooltips, and deferred draws instead of applying blanket damage.

The prototype should be treated as evidence about seams and ownership, not as a
benchmark of a complete second engine.
