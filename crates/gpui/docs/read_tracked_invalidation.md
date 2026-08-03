# Read-Tracked Invalidation: A Rendering & State Redesign for GPUI

**Status:** Design proposal. Milestone 1 is specified below (§9) as a
self-contained work order. Nothing in this document has shipped.

**One-line summary:** Stop deciding *what to re-render* with entity identity and
notify routing; decide it with the read ledger GPUI already maintains — extended
from window granularity down to element-tree boundaries — and reserve `notify()`
exclusively for the data plane.

**Terms used throughout** (skim now, refer back later):

| Term | Meaning |
|---|---|
| *entity* | A state cell of type `T` behind an `Entity<T>` handle, living in the app-wide `EntityMap`. Persistent, refcounted, identified by `EntityId`. |
| *notify / observe* | The explicit event system of the entity graph. `cx.notify()` announces "my data changed"; `cx.observe(&entity, ...)` registers a callback that runs when it does. Callbacks run from an effect queue (`App::flush_effects`, `crates/gpui/src/app.rs` ~1610). |
| *element* | A node in the per-frame UI tree built by the fluent DSL (`div().child(...)`). Elements are rebuilt every frame; they are values, not objects. |
| *element state* | Per-position storage that survives across frames, keyed by `GlobalElementId` (the stack of element ids from root to here). Lives in the frame (`Frame::element_states`). |
| *`use_state` / `use_keyed_state`* | Hooks (`crates/gpui/src/window.rs` ~3648) that allocate an `Entity<T>` whose lifetime is tied to an element position: "exists as long as this element renders in consecutive frames." |
| *struct `View` trait* | `gpui::View` (`crates/gpui/src/view.rs` ~182): lets a plain struct act as a view, reporting an optional `EntityId` as its identity. Distinct from the classic entity-backed view, `Entity<T: Render>`. |
| *the forwarder* | An observer installed by `use_keyed_state`: when the allocated state entity notifies, call `cx.notify(current_view)` — where `current_view` is the identity of the nearest enclosing view (`window.rs` ~3661). This document argues for its deletion. |
| *boundary* | An element-tree node that can independently skip re-rendering by reusing its previous frame. Today the only boundaries are cached entity-backed views (`Entity::cached`); this design generalizes them. |
| *ledger / read-set* | The record of which entities (and, later, globals) were read. GPUI already records every entity read (`EntityMap::accessed_entities`, `crates/gpui/src/app/entity_map.rs` ~156-179). A boundary's read-set is the slice of the ledger attributed to it. |
| *amplification* | The pattern this design eliminates: a component observing data it doesn't own must `cx.notify()` itself unconditionally, because self-notification is the only way to bust its identity-keyed cache. |
| *Projection* | From an unmerged branch: a lens handle (`Projection<P>` / `ProjectionMut<P>`, `crates/gpui/src/projection.rs`) onto part of an entity's state — e.g. a `ProjectionMut<String>` addressing one field of a form entity. Reviewing that branch is what surfaced this redesign (§2). |

---

## 1. The three registers of GPUI

GPUI programs are written in three registers, each with its own shape, lifetime,
and control flow — and each valuable precisely because of what it gives up:

| Register | Shape | Time | Control flow | Written as |
|---|---|---|---|---|
| **Entity graph** | arbitrary graph | persistent across frames | push: explicit events | plain Rust structs behind `Entity<T>` handles |
| **Element tree** | tree | one frame, memoized | pull: demand-driven | the fluent DSL; components |
| **Imperative drawing** | straight line | one instant | sequential command emission | `Element` impls: layout, prepaint, paint |

The entity graph can take any shape a domain needs — `Project`'s object graph
has nothing to do with rendering, and shouldn't. Its identity notion is
**allocation**: an `EntityId` minted at creation, lifetime governed by
refcounts. Its ethos is **explicitness**: no auto-notification anywhere; an
entity changes when its code says it changed. The drawing register at the other
end has no identity at all — just order — which is exactly where its
immediate-mode speed comes from.

The element tree mediates. Rendering is a pipeline of shape-collapses: the tree
is a per-frame projection of the graph, and the command stream is a per-frame
projection of the tree. There is deliberately no graph↔commands edge in either
direction; the tree is the only adapter between the shape regimes.

The tree register was accreted rather than designed. Interaction state forced
its existence: a click is stateful — a press and a release that must find each
other across frames — and its identity is **positional** ("the third button in
the toolbar"), a place that may correspond to no domain object whatsoever. GPUI
v1 forced graph identity onto that state — mint an entity for every hoverable
widget — and drowned in bookkeeping. So the tree grew its own organs, one
pragmatic patch at a time: element ids to name positions, element state to store
things at them, `use_state` to allocate entities with positional lifetimes, the
struct `View` trait and `current_view` to route invalidation. In the tree,
identity = position (`GlobalElementId` path) and lifetime = continuity of
rendering. These are different notions of "same thing across time" than the
graph's, and they are not interconvertible.

By now the tree has almost every organ of a complete register: identity
(`ElementId`), storage (element state), composition (the DSL, components), even
memoization scaffolding (prepaint reuse). The one organ it never grew is **a
dependency semantics of its own** — an answer to "when is this position stale?"
It borrowed the graph's answer (notify), which is addressed by `EntityId`, so
the tree had to route its invalidation *through graph identity*. That borrowed
organ is `current_view`, and every bug documented in §2 is a register-confusion
bug traceable to it. This design gives the tree its own answer — stale when
something it **read** changed, or when explicitly invalidated in the tree's own
currency — and settles the tree's relationship to the graph into two typed
edges: reads down, writes up.

One rehabilitation up front: `use_state` is not the problem and survives
unchanged. Tree-lifetime allocation of graph-register state is the correct
bridge — an editor needs observation, async, real entity-hood, but its lifetime
is genuinely positional. Only the forwarder stapled to it is wrong. The
practical rule for app authors becomes: choose the register by asking what a
piece of state's identity *is*. A concept in your domain → entity. A place in
the UI → element state. A domain concept living at a place → `use_state`.

---

## 2. How we got here

This design fell out of a code review of the projections branch, which surfaced
a chain of increasingly fundamental problems:

1. **A hang.** A component identified by a data entity's id (via the struct
   `View` trait), containing `use_state`-allocated state that observes that same
   entity, loops forever inside `App::flush_effects`. Reproduced on `main` with
   plain entities — no projections involved — using the `view_example` `Input`:
   on the first write to the value, the effect queue never drains. This is not a
   projections bug.

2. **The loop's anatomy.** Three locally-innocent pieces in three files:
   - *Data → derived state* (component author): the editor observes its value
     and notifies itself when its cursor clamps
     (`crates/gpui/examples/view_example/example_editor.rs`).
   - *Internal state → view identity* (framework): the forwarder —
     `cx.observe(&state, move |_, cx| cx.notify(current_view))`
     (`window.rs` ~3661), `current_view` being the nearest `View::entity_id`
     (`view.rs` ~323, `window.rs` ~4670).
   - *The alias* (component): `View::entity_id` returning the **data entity's**
     id, merging "the widget" and "the widget's input data" into one graph node.
   The forwarder then publishes widget-internal churn as data-change events of
   an entity upstream of the widget's own state. Cycle. See §10 for the minimal
   reproduction.

3. **The three roles of an `EntityId`.** The diagnosis underneath the loop —
   referenced throughout this document:
   - **Role 1 — element identity:** keying element state, caching, sibling
     disambiguation.
   - **Role 2 — invalidation target:** "redraw the subtree rendering this."
   - **Role 3 — publication channel:** "this entity's data changed; run its
     observers."

   Roles 1 and 2 are safe to borrow from another entity; role 3 is not. The
   struct `View` trait hands out all three as a bundle, and its doc comment
   coaches the unsafe borrow ("a view typically holds the backing entity as a
   field and returns its `EntityId` here").

4. **Amplification.** The identity rule doesn't just permit loops; it causes
   over-rendering. The only cache-bust signal today is "identity entity
   notified" (`view.rs` ~390), so any component rendering data it doesn't own
   must convert upstream maybe-changes into unconditional self-notifies. The
   editor's cursor-clamp handler cannot be written in settling style (notify
   only when something actually changed) without breaking `editor.cached(..)`.
   At app scale this is the Pane-observes-items-and-notifies-itself pattern:
   whole-region re-renders purchased to bust one cache.

5. **The realization.** GPUI is *already* a read-tracked reactive system at
   window granularity. Every entity read lands in the ledger
   (`entity_map.rs` ~156-179). `Window::draw` harvests it
   (`record_entities_accessed`, `app.rs` ~1095), and `App::notify`
   (`app.rs` ~2611) dirties exactly the windows that read the notified entity.
   **Reads already decide *where* to re-render at the top level.** The element
   tree below just never got the same rule — it got the identity rule instead.
   Cached views even record per-subtree read-sets already
   (`ViewElementState.accessed_entities`, captured via
   `detect_accessed_entities`, `app.rs` ~1079 / `view.rs` ~405) and then consult
   them for nothing: `view.rs` ~390 checks only identity, and `view.rs` ~395
   merely re-registers the reads on reuse.

Before committing to the design, we commissioned an adversarial review: an agent
with full code access, instructed to build the strongest possible case against
the proposal and to rank objections by severity. Its findings materially
reshaped the design (the boundary-invalidation channel of §5 and the `Cached`
component of §6 both exist because of it) and are recorded with dispositions in
§7.

---

## 3. The model

Two planes, tied by exactly three links, each with one-way arrows:

| Link | Direction | Nature |
|---|---|---|
| **Reads** | entity graph → element tree | Implicit. Recorded in the ledger; drives all render invalidation. |
| **Allocation** (`use_state`) | element tree → entity graph | Explicit. Ownership, not an event edge. |
| **Event handlers** | element tree → entity graph | Explicit user code; the only writes. |

Rules:

- **Entity plane (unchanged — Zed's ethos):** `cx.notify()` is explicit and
  means "my data changed." Observers are the explicit data plane (role 3).
  No auto-notifies.
- **Render plane (new rule):** a boundary re-renders iff its recorded read-set
  intersects the set of entities notified (or globals updated) since it was last
  rendered — or it was explicitly invalidated through the render-plane channel
  (§5). Renders are memoized pulls. Elements never bust other elements; render
  output is not an input to anything, so invalidation cannot cascade through the
  tree.
- **For honest entity-backed views nothing changes:** an `Entity<T: Render>`
  view reads itself during render, so the read rule strictly subsumes the
  identity rule for it. The rules diverge only for borrowed identities — where
  the read rule works and the identity rule loops.

The forwarder is deleted. Internal state entities are dependencies of the
boundaries that read them; their explicit notifies reach those boundaries
through the ledger like any other entity's.

---

## 4. Bookkeeping: attribution by ownership, not draw order

The naive implementation — one append-only ledger, each boundary owning a
contiguous `[start..end)` range — is broken by GPUI's own draw structure:
paint-phase reads happen outside the prepaint capture window (`view.rs`
~405-415 wraps prepaint only), and deferred draws (`window.rs`,
`prepaint_deferred_draws`) execute after the main walk, re-rooted, so their
reads land outside their logical parent's range. A global index-*set*
additionally mis-attributes sibling reads (the second reader's insert is a
no-op).

All three problems share one wrong assumption: that attribution follows
**wall-clock draw order**. It should follow **logical ownership**:

- Maintain a **stack of open dependency records**. Opening a boundary pushes its
  record; every read (`EntityMap::read`/`read_any`, global access) appends the
  `DependencyId` to the innermost open record.
- **Phases are episodes.** A boundary's record is re-opened during its paint
  walk; paint reads append to the same record. Busting always evaluates *last
  frame's completed record*, so capture time never races the reuse decision.
  (Reading during paint remains bad form — that's what layout and prepaint are
  for — but it is captured, not silently dropped. A debug lint can come later.)
- **Deferred draws re-open their originator's record.** A `DeferredDraw` already
  carries its originating view; it carries its originating *boundary record*
  instead. When the deferred element (a context menu, tooltip, drag overlay)
  prepaints in a later round, push that record — its reads attribute to the
  boundary that deferred it, wherever the element lands in draw order. No
  discontiguous-range arithmetic; multi-phase rendering stops being a special
  case because every phase re-establishes which boundary is logically open.
- **Dedup in O(1):** keep a per-frame `last_attributed: FxHashMap<DependencyId,
  RecordId>`; on read, if `last_attributed[dep]` equals the current innermost
  record, skip the append. This replaces (and costs about the same as) today's
  global hash-set insert.
- **Parents don't need children's reads.** Busting uses a per-window reverse
  index `DependencyId → SmallVec<BoundaryId>` maintained at record time. A dirty
  boundary marks its **ancestor path** dirty so the top-down draw can reach it —
  and `GlobalElementId` is the stack of element ids, so ancestry is a prefix
  relation (this replaces `mark_view_dirty`'s dispatch-tree walk, `window.rs`
  ~1936). Ancestors re-run their render functions; clean boundaries off the
  dirty paths splice their previous prepaint ranges, exactly as `reuse_prepaint`
  does today.
- **Fan-out tier for widely-read dependencies.** Theme- and settings-class
  dependencies are read by effectively every boundary. When a dependency's
  reader count exceeds a threshold, flip it to a *broad* flag: notifying it
  refreshes the window (which is today's behavior for those cases anyway)
  instead of enumerating readers. This bounds reverse-index size and answers the
  bust-storm objection (§7, objection 5).
- **Lifecycle:** records and reverse-index entries are generation-stamped and
  expire with the element state they belong to. On cache reuse, the stored
  record re-registers into the window aggregate, as `extend_accessed` does now
  (`view.rs` ~395).
- **Globals** join the ledger as a second variant; `update_global` dirties
  readers through the same index. Window state (focus, hover position, mouse)
  stays on today's coarse whole-window refresh initially and can be promoted
  later.

```rust
enum DependencyId {
    Entity(EntityId),
    Global(TypeId),
}
```

---

## 5. The render-plane invalidation channel

The adversarial review's deepest finding: `cx.notify(current_view)` is
load-bearing far beyond the forwarder. Hover styling, active/click state,
scroll, tooltips, `request_animation_frame`, image loads, list state — the
interaction layer (call sites reported across `div.rs`, `text.rs`, `list.rs`,
`img.rs`, `window.rs`; verify the full list during Milestone 2) all say
"re-render me" by notifying the nearest view entity. Every one of those is
role 2 (invalidation) wearing role 3's (publication) clothes: hover does not
want observers to run; it wants its region repainted. This is the same
role-laundering as the forwarder, in the opposite direction — and it is why
"just delete view identity" would leave a vacuum.

So the channel these call sites actually need becomes first-class:

```rust
window.invalidate(boundary_id)  // dirty this boundary for the next frame.
                                // No Effect::Notify. No observers. Not an entity.
```

Interaction and animation state dirties the boundary it occurred within,
addressed by element id — the tree's own identity currency. Explicit
boundary-dirty overrides cache reuse regardless of read-sets and props. The
draw-phase suppression carve-out (`window.rs` ~157-164: notifications during a
draw mark state dirty but defer effects) carries over unchanged. Entities stop
hearing about hover; observers of an editor fire only when the editor *says*
something changed.

This resolves the review's two hardest objections at once: interactive content
inside cached subtrees stays live (the thing that made universal caching
unsound), and removing struct-view `entity_id` leaves no vacuum, because the
thing that identity was actually being used for gets its own addressing scheme.

---

## 6. The `Cached<T>` component

Caching stops being ambient (`.cached()` sprinkled on arbitrary elements — which
the review showed to be unsound) and becomes an explicit, visible boundary
component:

```rust
Cached::new(
    key,        // ElementId: state slot + sibling disambiguation (role 1, tree currency)
    props,      // T — the data funnel into the subtree
    render,     // fn(&T, &mut Window, &mut App) -> AnyElement  ← plain fn, NOT a closure
)
.compare(|prev, next| ...)   // user-supplied; defaults to T: PartialEq if available
.style(...)                  // the layout contract, explicit (cached contents aren't
                             // measured — the same requirement `cached()` hides today,
                             // view.rs ~262-271)
```

- **The `fn` barrier is the soundness argument.** Because the render function
  cannot capture, the subtree's inputs are exhaustively: `props` (compared),
  ledger reads (tracked), window context (cache-keyed), and boundary dirtiness
  (§5). Nothing can be smuggled in. (Non-capturing closures coerce to `fn`
  automatically, so the ergonomics stay closure-like.)
- **The stale-closure hazard dissolves as a corollary:** reuse requires props to
  compare equal, and every handler constructed inside captured only
  props-derived data — so replayed listeners are provably equivalent to fresh
  ones. Handlers passed *in* arrive via props as `Rc`s and can be compared by
  pointer.
- **Reuse condition:** geometry key matches ∧ read-record clean ∧ boundary not
  explicitly invalidated ∧ `compare(prev, next)`.
- **Debuggability flips from objection to feature:** boundaries are visible in
  the tree, and each `Cached` can report its read-set, its props diff, and its
  last bust reason to the inspector. "Why did this re-render" gets an artifact;
  today's answer is grepping for `cx.notify`.

`Entity::cached` / `AnyView::cached` keep working throughout (an entity view's
self-read makes the read rule equivalent or better).

### The struct `View` trait after this

`fn entity_id(&self) -> Option<EntityId>` is removed, replaced by an optional
**element key** for sibling/state disambiguation (`fn key(&self) ->
Option<ElementId>` or equivalent). An `EntityId` remains a perfectly good *key*
— a tab title keyed by its editor's id is using the id as a name, not a mailbox
(role 1 without role 3). `Entity<T: Render>` views are untouched.

### Projections after this (separable)

With invalidation read-derived, projections need no backing entities: a
`Projection<P>` becomes a plain `{ source, composed lens }` value, constructible
anywhere (the current render-context restriction existed only to mint an
identity for it). `observe` delegates to the source; equality-gated fine-grained
observation remains a possible opt-in data-plane tool. Most of `projection.rs`
melts away. The lens-change notify fix on that branch stands on its own
regardless.

---

## 7. Adversarial review: objections and dispositions

An agent with full code access was instructed to build the strongest case
against this design and rank each objection: fatal / serious-but-mitigable /
friction. Dispositions:

| # | Objection (severity as reviewed) | Disposition |
|---|---|---|
| 1 | Universal props-compared caching is unsound: hover/active/scroll live in element state, invisible to reads and props; reused subtrees replay stale listeners (fatal as specified) | **Absorbed.** The §5 boundary channel keeps interaction live under caches; the §6 `fn` barrier + props funnel closes the stale-capture hole. Ambient `.cached()`-anywhere is *not* shipped; `Cached` is the only new boundary form. |
| 2 | The ledger can't see paint reads; deferred draws break contiguous ranges; a global index-set mis-attributes sibling reads (fatal as first drafted) | **Conceded and redesigned.** §4 ownership attribution: records not ranges, phases as episodes, deferred draws re-open originator records, O(1) dedup. |
| 3 | `current_view` is load-bearing across the interaction layer; deleting view identity degrades those notifies to huge ancestor entities → bust storms (serious) | **Absorbed.** §5: those call sites want role 2 and now get a real role-2 channel. Large mechanical migration; the objection's site list is the checklist. |
| 4 | `use_keyed_state` doesn't *read* its entity, so forwarder deletion silently breaks components that only touch state in handlers (serious) | **Conceded, one-line fix:** allocation is a dependency; `use_keyed_state` records a ledger read unconditionally. |
| 5 | "The cost is already paid" is overstated: globals are untracked today; per-boundary bookkeeping is new; `refresh()` bypasses caching on many real frames; widely-read entities cap the win (serious/friction) | **Partially conceded.** The broad-dependency tier (§4) bounds fan-out; global tracking is a cheap tag; `refresh()` prevalence is a pre-existing ceiling this makes worth lowering, not a regression. Benefit claims must be re-validated by profiling in Milestones 1–2. |
| 6 | Observer/effect timing semantics must be preserved bit-for-bit (serious) | **Accepted as a constraint.** Milestones keep `Effect::Notify` semantics and draw-phase suppression untouched; read-rule busting is strictly additive in Milestone 1. |
| 7 | Implicit dependency tracking hurts debuggability and GPUI's explicitness ethos (friction) | **Flipped.** Boundaries are explicit components; read-sets are inspectable artifacts. The entity plane stays fully explicit. |
| 8 | A much smaller fix captures most of the benefit (decisive comparison) | **Adopted as sequencing.** The "smaller fix" *is* Milestone 1. Every milestone is a coherent stopping point. |

**Falsifiers — how we'd know this design is wrong:** Milestone 1 reveals
widespread dependence on forwarder-as-observer semantics beyond the audited
sites; profiling shows ledger/record overhead in typing latency or terminal
repaint; the broad-dependency tier ends up covering most dependencies
(fine-grained buys little in practice); Milestone 2's `current_view` migration
finds a genuine role-3 dependency that cannot be expressed as boundary
invalidation.

---

## 8. Milestones

1. **Contract swap, additive, behind existing gates** — read-set busting at the
   existing entity-backed cache sites; forwarder deletion with
   allocation-as-read; loop detection in `flush_effects`. *(Work order in §9.)*
2. **Render-plane channel** — `window.invalidate(boundary)`; migrate the
   `current_view` notify sites in `div.rs` / `text.rs` / `list.rs` / `img.rs` /
   `window.rs`; stop pushing `Effect::Notify` from the invalidator path for
   render-plane invalidations.
3. **`Cached<T>`** — the record-stack bookkeeping (§4), the component (§6),
   deferred-draw record re-opening, globals in the ledger.
4. **struct `View` trait surgery & projections-as-values** — remove
   `entity_id`, add element keys; collapse `projection.rs` to plain lens values.

---

## 9. Milestone 1 work order (self-contained worker prompt)

> You are implementing Milestone 1 of GPUI's read-tracked invalidation redesign
> in the Zed repository. Read this section fully, then the cited code, before
> writing anything. The design context is
> `crates/gpui/docs/read_tracked_invalidation.md` (§1–§8); you only need §9 to
> execute, but read §3 and §4 to understand intent, and the terms table at the
> top for vocabulary. Use `./script/clippy` instead of `cargo clippy`. Do not
> commit; leave changes in the working tree.
>
> **Goal:** make subtree cache invalidation consult recorded read-sets
> (additively — never bust less than today), delete the `use_keyed_state`
> forwarder safely, and add effect-loop detection. No public API changes. No
> `View`-trait changes.
>
> ### Task 1 — Expose the per-frame notified-entity set
> `WindowInvalidator::invalidate_view` (`crates/gpui/src/window.rs` ~153)
> accumulates notified entity ids in `dirty_views`;
> `Window::invalidate_entities` (~2947) drains it through `mark_view_dirty`
> (~1936), which ancestor-walks into `Window::dirty_views`. Preserve all of
> that, and additionally retain the **raw notified set** for the frame (before
> path-marking) as e.g. `Window::dirty_entities: FxHashSet<EntityId>`, cleared
> where `dirty_views` is cleared (~2842-2846).
>
> ### Task 2 — Read-set busting at existing cache sites (additive)
> In `ViewElement::prepaint`'s cached branch (`crates/gpui/src/view.rs`
> ~386-401), the reuse condition currently requires
> `!window.dirty_views.contains(&entity_id)`. Add:
> `element_state.accessed_entities.is_disjoint(&window.dirty_entities)`.
> The read-set is already captured at ~405 (`detect_accessed_entities`,
> `crates/gpui/src/app.rs` ~1079) and re-registered on reuse at ~395. Keep the
> identity check — this milestone only ever busts *more*.
>
> ### Task 3 — Allocation is a dependency; delete the forwarder
> In `Window::use_keyed_state` (`crates/gpui/src/window.rs` ~3648):
> (a) on **every** call (creation and lookup), record a read of the state entity
> in the ledger (`cx.entities.accessed_entities`) — components that only touch
> their state inside event handlers must still be treated as depending on it
> (see `crates/settings_ui/src/components/number_field.rs` ~286 for the
> pattern);
> (b) delete the `cx.observe(&new_state, move |_, cx| cx.notify(current_view))`
> forwarder (~3656-3666) and the now-unused `current_view` capture.
> Do **not** change `Effect::Notify` semantics or the draw-phase suppression in
> `invalidate_view` (~157-164); observers of entities must fire exactly as
> today.
>
> ### Task 4 — Effect-loop detection
> In `App::flush_effects` (`crates/gpui/src/app.rs` ~1610), count effects
> processed within one flush; past a large threshold (e.g. 1,000,000), panic
> with a message naming the likely cause (an observe/notify cycle between
> entities) and, in test builds, the most-frequently-notified `EntityId`s.
> Bound, don't alter, semantics.
>
> ### Task 5 — Tests (write these first where practical)
> 1. **Read-busting:** an entity-backed `cached()` view whose subtree reads a
>    *different* entity X (not its identity) re-renders when X is notified.
>    Must fail before Task 2, pass after. Follow the harness style in
>    `crates/gpui/src/projection.rs`'s test module (`HookView` / real
>    `window.draw` cycles).
> 2. **Forwarder-deletion safety:** a view whose `use_state` entity is updated
>    and notified from an observer/handler (never `.read()` in render beyond
>    the allocation) still re-renders. Must pass after Task 3.
> 3. **Loop class dead:** in
>    `crates/gpui/examples/view_example/example_tests.rs`, the
>    `nested_subforms_do_not_feed_back` test guards a feedback loop that
>    previously hung when `Subform::entity_id` returned
>    `Some(self.person.entity_id())`. Add a variant with exactly that identity
>    and assert it settles (`run_until_parked` returns; values correct). It
>    hangs before Task 3 and must pass after. Note the example tests require
>    `--features test-support`.
> 4. **Loop detector:** a deliberate two-entity observe/notify cycle panics
>    with the diagnostic instead of hanging.
>
> ### Acceptance
> - `cargo test -p gpui` and
>   `cargo test -p gpui --features test-support --example view_example` pass.
> - `./script/clippy -p gpui` clean.
> - Run `cargo test -p ui -p workspace` (or the nearest available UI-consuming
>   suites); **report** any failures that indicate dependence on forwarder
>   observer semantics rather than papering over them — that list is a primary
>   deliverable (see §7, falsifiers).
> - Summarize: what busts more often than before (expected: nothing
>   user-visible; read-busting is additive), and any `use_state` call sites
>   whose behavior you believe changed.

---

## 10. Appendix: the minimal loop (for posterity)

```text
value ──(clamp subscription: "data changed ⇒ cursor moved, notify editor")──▶ editor
  ▲                                                                             │
  └──(use_state forwarder: "internals changed ⇒ notify current_view" = value)───┘
```

Every edge is locally downstream *in intent*; the alias (`View::entity_id`
returning the value's id) folds "downstream of the widget" onto "upstream of the
data," and the effect queue never drains. Reproduced on `main` (no projections
involved) via `view_example`'s `Input` over a plain `Entity<String>`: hangs on
the first write. The same topology with a projection's backing entity, or with
the projection's *source* entity, also hangs. Positional identity (`None`)
avoids it at the cost of sibling state collisions — which is what motivated this
redesign instead of the workaround.
