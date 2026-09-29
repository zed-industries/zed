# View tree

GPUI has always had a tree of view mounts, implicitly. Entity views nest,
`defer_draw` hangs a subtree off the view that deferred it, element state is
keyed by a path through that tree, and a window draws several roots: its root
view, then prompts, drags and tooltips. `ViewTree` (`src/view_tree.rs`) makes
that structure explicit, without changing how anything draws.

This document records what the tree is, why it exists in this form, and what
the analysis that shaped it found. The analysis compared the earlier
`gpui-retained-node-engine` branch (zed-industries/zed#63800) with an
independent retained-mode fork of GPUI (longbridge/gpui-fast#2), and with
native window composition (zed-industries/zed#62379).

## What the tree is

- A **node** is one mount of an entity-backed view at one place in the element
  tree, keyed by the view's `GlobalElementId`. The same `Entity<V>` rendered in
  two places is two nodes. Stateless components (`RenderOnce`) are not nodes.
- A node is **mounted** the first frame its view is laid out and **unmounted**
  at the end of the first frame in which it is not visited. `window.refresh()`
  redraws every view without unmounting any.
- A node's **parent** is the node being laid out or prepainted when it was
  visited. Deferred draws capture the current node, so views inside a popover
  mount under the view that opened it.
- A view inside a cached view whose output was reused stays mounted, because
  its output was reused with it. Such a node has no prepaint or paint range of
  its own that frame.
- The **roots** are the nodes with no parent, in draw order.
- Each node records its bounds and the ranges its prepaint and paint output
  occupy in the frame. The frame itself stays flat: hit testing, dispatch,
  input handlers and the scene are unchanged.
- Each node **owns the element state** of the elements drawn inside it (not
  inside a nested view). A state is dropped when its node unmounts, or at the
  end of a frame that drew the node without accessing it. A node whose output
  was reused keeps all of its states, since reusing output accesses none of
  them. Elements a test draws outside every view keep their state in the tree
  too, through the frame after the one that last accessed it.

Element state is the tree's first reader. Keeping it on nodes replaced the
frame's flat state map and the list of accessed keys that reused output had
to copy forward. The tree costs one `GlobalElementId` lookup and a few index
snapshots per view per frame; a paired `editor_render` run could not
resolve a difference from `main` on a loaded machine (every fixture moved
between −10% and +4%, in both directions).

## Why a tree, and why this small

The tree is kept for two reasons:

1. **It names a structure GPUI already has.** Mounts, parents, deferred owners
   and the window's multiple roots exist today as conventions spread across
   `Window`, `Frame` and `ViewElement`. Naming them gives one place to ask
   "what is mounted, where, and since when".
2. **It is a building block for retained things that are not views.** A
   composition surface, an attached root read back by an embedder, or a native
   widget in the element tree is a thing with an identity, a lifetime, bounds
   and output that has to be addressed across frames. Each is another kind of
   node or root, not a new structure.

It is small on purpose. The earlier branch fused the tree with output reuse,
Taffy retention, dispatch replay, components and a notify contract, and
justified the tree by their performance. Taken apart, each of those either
does not need the tree or should arrive on its own, measured, when something
needs it.

## What the analysis found

### Performance does not need the tree

gpui-fast, developed independently from `main` at `7960b2a7c9`, retains views
as a cache over the flat frame: per-subtree records of ranges into the frame's
arrays, copied and shifted when reused. It reports large wins on its own
component gallery. So memoizing views is achievable without persistent nodes.

In Zed, frame cost is dominated by per-primitive work, not per-view work. The
earlier branch's own measurements put an editor at about 190 µs a frame whether
or not it changed, because replaying its glyphs costs about what painting them
does. Memoizing whole views wins on the chrome around editors (−64% on a row
update) and is a wash on editors themselves. The techniques that attack the
per-primitive cost are independent of the tree and can land on `main` first:

- Replaying last frame's draw orders in the bounds tree while the bounds come
  in the same (gpui-fast `c218767`, `8eed010`). This is the "replay floor" the
  earlier branch identified.
- Retaining Taffy nodes per element, keyed by element path, and writing a
  style or child list only when it differs, so a rebuilt view keeps Taffy's
  cache.
- Keeping text measurements across frames, and recoloring shaped text without
  reshaping it.

### Lifecycle features mostly exist flat already

`main` already has an unmount signal: element state not accessed during a
frame is dropped at the frame swap, so a `use_keyed_state` entity is released
the first frame its element stops rendering. A survey of Zed's app crates
found little demand the tree alone would meet: `use_state` in about fifteen
files, `.cached()` in two, `on_release` used only for entity cleanup, and image
ownership already fixed on `main` (#63934).

One real problem surfaced: `RenderOnce` state is scoped by type name only, so
two sibling components of one type collide, and `ui` components thread
explicit ids to dodge it. A `(type, nth)` scope fixes it and does not need the
tree. The count has to be per element-id scope instance, not per path per
frame: `list` renders an item to measure it and again to draw it, and which
items render before a given one changes as it scrolls, so a per-frame count
drifts and resets the state of everything inside the item. Pushing an element
id opens a fresh count, and `list` and `uniform_list` open one per item
without changing the id path. The first component of a type keeps the plain
type-name id, so only true siblings change paths.

The count is also fresh in each phase an id is pushed in, so a component that
an element renders lazily during prepaint (rather than at layout) can collide
with a sibling of the same type rendered at layout under the same id. No
element in GPUI does that without an id of its own.

### Damage regions need correspondence between frames

A damage rect is the bounds of last frame's output that was not carried over,
plus this frame's output that was newly drawn. Any retention scheme that knows
which output corresponds to which across frames can compute it; the tree makes
that correspondence convenient, not possible. What damage regions actually
need:

- Rebuilding a dirty view without rebuilding its ancestors. While ancestors
  rebuild with a dirty child, the root rebuilds on every changed frame and the
  damage is the whole window.
- Tracking focus, hover and input modality as inputs instead of refreshing the
  window, since a refresh is full damage.
- Damage from primitive runs, not node bounds: shadows, overflowing text and
  absolutely positioned children paint outside a node's bounds.
- Backend support for presenting a partial frame (buffer age or a persistent
  swapchain) on every platform.

### Composition is where the tree does work nothing else does

#62379 splits one flat scene into surfaces at the deferred-draw boundary, with
a native view between the base and the overlay. That works without nodes,
because any subtree paints a contiguous range. Generalizing it is where the
tree fits:

- Arbitrary GPUI subtrees as surfaces, not only "base" and "deferred overlay".
  An embedded GPUI surface is a root with an offset.
- Presenting only the surfaces whose content changed, which needs an exact,
  cheap change stamp per subtree.
- Native widgets in the element tree, as mobile will need: created on mount,
  kept through refreshes and reorders (so identity must be keyed, not only
  positional), moved when layout moves them even on frames where nothing around
  them was redrawn, and destroyed on unmount.

The presentation side (surface order, platform attachments, geometry) belongs
to the composition tree; the content side belongs to this tree. A GPUI surface
is fed by nodes. Surfaces are not roots: a native surface has no GPUI content,
and a root need not be a surface.

Flutter's platform views are the cautionary precedent: every native view with
content above it forces another surface, and interleaving has a per-frame
cost. Surface splits should be explicit and countable.

### Lessons carried over from the earlier branch

- **Keep the unit of lifecycle separate from the unit of reuse and of layout
  retention.** Fusing them made every later step a change to three files, and
  made the tree's case rest on wins it did not produce.
- **Notify held as a contract.** Zed's editor, workspace and panel stress tests
  surfaced no view that mutated state without notifying.
- **Globals cannot be tracked as reads.** `global_mut` and `default_global`
  report a write on every call, so a render that calls them (such as
  `ScrollbarAutoHide`) invalidates every reader every frame.
- **Close the `Window` read surface before caching output.** Anything a render
  reads that is not app state (mouse position, modifiers, hover, focus, the key
  context an ancestor set) must be a tracked input or part of a cache key, or a
  reused view goes stale silently.
- **Build a retained dispatch tree before reusing output.** Replaying a flat
  dispatch tree into a reused frame was the most frequent source of review
  findings on the earlier branch.

## Plan

Independent of the tree, on `main`:

- [ ] Draw-order replay in the bounds tree.
- [ ] Per-element Taffy retention with write diffing.
- [ ] Carried text measurements; recolor without reshaping.
- [x] `(type, nth)` scope for `RenderOnce` state.

On the tree, each step landing with something that uses it:

- [x] Mount and unmount views as a tree, indexing the flat frame.
- [x] Move element state onto nodes, so a node's state lives and dies with it.
- [ ] Attached roots: read back a root's output by identity, with a change
  stamp and hit regions.
- [ ] Composition surfaces fed by nodes (#62379).
- [ ] A dispatch tree kept on the nodes.
- [ ] Output reuse with read tracking, measured against the direct
  optimizations above.
- [ ] Focus, hover and modality as tracked inputs.
- [ ] Rebuilding a dirty view inside a reused parent.
- [ ] Damage from primitive runs, then backend support.

## Open questions

- How much does the tree cost on a quiet machine? The measurement above is
  inconclusive.
- Should identity become keyed (surviving reorders) before native widgets
  arrive, and how does that interact with `GlobalElementId` paths?
- Which consumer comes first: attached roots for embedding, or composition
  surfaces?
