# Pixel Snapping Handoff

This document is a handoff for the GPUI pixel-snapping problem in
[taffy.rs](/D:/source/zed/crates/gpui/src/taffy.rs) and related paint-time
snapping in [window.rs](/D:/source/zed/crates/gpui/src/window.rs).

It is intentionally opinionated. It records:

- what we are trying to achieve
- which constraints are hard
- which approaches were tried
- why each approach was rejected or considered unacceptable
- where the code currently stands

The goal is not to defend the current implementation. The goal is to help the
next agent avoid re-running the same dead ends.

## What We Want

We want pixel snapping that satisfies these layout properties as often as
possible:

1. **Edge closure**
   - If two edges touch in Taffy's raw layout, they should still touch after
     snapping.
   - In particular, a full-width or full-height child should exactly reach the
     parent's snapped content edge.

2. **Placement stability**
   - Translating a parent in absolute space should not change the child's
     snapped position relative to that parent.
   - Children should not "swim" inside their parent because the parent moved.

3. **Velocity coherence**
   - During resize, nearby siblings should move at the same apparent rate.
   - We want to avoid the perceptual bug where one child jumps a pixel while a
     neighboring child stays still for the same resize step, unless that is
     mathematically unavoidable.

## Additional Non-Negotiable Constraints

These are not soft preferences. They materially change which solutions are
acceptable.

1. **Border thickness is authoritative**
   - This is a hard requirement.
   - Any scheme that preserves closure by silently changing rounded border
     thickness is unacceptable.

2. **Framework-layer fix, not control-specific workarounds**
   - A temporary control-level override may help diagnose the issue, but it is
     not the intended end state.

3. **No disabling subpixel text rendering as a "solution"**
   - That was explored and rejected.

4. **No heavy solver in the layout hot path**
   - A joint quantization solver is mathematically interesting, but considered
     too expensive and too much cognitive overhead for this path.

5. **No brittle dependence on reconstructing Taffy's internal math**
   - If the fix depends on exactly reproducing floating-point operation order to
     rediscover topology, that is not considered robust enough.

6. **Be honest about irreducible cases**
   - Some centered-content phase issues are fundamental to integer quantization.
   - We should not confuse those with fixable framework bugs.

## The Fundamental Irreducible Case

Centered content creates an unavoidable stay-jump-stay-jump pattern.

If a child of width `C` is centered in a container of width `W`, its offset is:

`(W - C) / 2`

Each 1-device-pixel change in `W` changes that offset by 0.5 device pixels. No
integer snapping scheme can make that perfectly smooth. Two centered elements
whose widths differ by an odd number of device pixels will be permanently out of
phase and jump on alternating resize steps.

This matters because not all remaining "jitter" is evidence of a bug. Some of
it is quantization.

That said, we still observed clearly unacceptable non-fundamental behavior in
GPUI, including cases where related labels appeared to move at different times
in ways that felt worse than browser or WinUI behavior.

## The Real Fixable Problem

The fixable problem is the tradeoff between:

- **proportional rescaling**, which preserves closure but causes position-based
  motion distortion
- **direct relative rounding**, which preserves placement stability much better
  but can leave 1-device-pixel seams at full-width/full-height boundaries

The desire is to "have our cake and eat it too":

- keep border thickness authoritative
- keep placement stability
- keep the improved resize feel of direct relative rounding
- still close full-width/full-height children exactly against the snapped parent
  content edge

So far, every attempt to get all of that at once has had a real cost.

## Approaches Explored

### 1. Independent local rounding

Concept:

- Round boxes locally and independently after layout.

Why it was insufficient:

- It did not preserve closure in important cases.
- It allowed disagreement between the child's rounded far edge and the parent's
  snapped content boundary.
- The core arithmetic problem is:

  `round(a - b - c) != round(a) - round(b) - round(c)`

Concrete example:

- At 150% scale, a `1px` border is `1.5dp`.
- If border thickness is rounded independently per side, snapped insets can
  disagree with the raw content interval by 1dp.
- A full-width child can then miss the parent's snapped content edge by 1dp.

This is the seam problem.

### 2. Control-specific fixes

Concept:

- Adjust specific controls so labels align differently or inherit different text
  alignment behavior.
- Example: changing composite button content from inherited centered text to a
  centered row with left-aligned text.

Why it was rejected:

- It improved specific controls, but only by working around a framework bug.
- The required fix is in the framework layer, not in individual controls.

These fixes were useful diagnostically, but not acceptable as the final answer.

### 3. Text-origin snapping / subpixel text changes

Concept:

- Snap label origins more aggressively, or disable subpixel text rendering at
  fractional scale.

What happened:

- One text-origin change normalized subpixel offset but made some labels shift a
  full pixel relative to nearby icons.
- Disabling subpixel text rendering was explicitly rejected as unacceptable.

Why this was rejected:

- It attacked symptoms in text rendering rather than the geometric source of the
  problem.
- It also produced regressions that were perceptually worse than the original
  issue.

### 4. Proportional rescaling

This was the original layout snapping approach.

Concept:

- In parent-content-relative coordinates, map child positions through:

  `snapped = round(raw * snapped_parent_content_size / raw_parent_content_size)`

What it gets right:

- **Edge closure:** very good
- A child at `0` maps to `0`, and a child at `raw_content_extent` maps to
  `snapped_content_extent`

Why it was rejected:

- **Placement stability:** bad
- **Velocity coherence:** bad

Visual failure mode:

- It behaves like a rubber sheet.
- Children farther from the origin accumulate more distortion than children
  nearer the origin.
- The parent can translate while descendants re-quantize relative to it.
- During resize, edges farther from the origin can jump sooner than nearby
  edges.

This fixed closure by smearing the residual across the entire interval, and that
looked wrong.

### 5. Direct relative rounding

This became the preferred baseline after rejecting rescaling.

Concept:

- In parent-content-relative coordinates, round child edges directly:

  `snapped = round(raw_relative_edge)`

What it gets right:

- **Placement stability:** much better
- **Velocity coherence:** much better than proportional rescaling
- No position-dependent slope distortion

Why it was not enough:

- **Edge closure:** fails at parent-content boundaries when snapped border/padding
  insets do not match the raw content interval exactly

Visual failure mode:

- Full-width or full-height children can leave a visible 1dp strip at the far
  edge of a bordered parent.

This is the cleanest interior behavior we found, but it does not solve closure
by itself.

### 6. Boundary-first snapping

Concept:

- Snap outer and content boundaries directly, then derive border thickness from
  those snapped boxes.

Why it was rejected:

- It gives up exact border thickness fidelity.
- That is unacceptable here.

This is important: some conceptually simple schemes become unacceptable the
moment border thickness is treated as sacred.

### 7. Joint quantization / solver

Concept:

- Solve a constrained rounding problem per parent so snapped segment lengths sum
  exactly to the parent's snapped content interval while staying close to raw
  lengths.

Why it was rejected:

- Too much complexity for the hot path
- Too much cognitive overhead
- Not acceptable as the first-line framework solution

This may be mathematically elegant, but it is currently outside the acceptable
design space.

### 8. Explicit edge-ownership metadata

Concept:

- Instead of inferring whether a child edge coincides with the parent content
  boundary from floats, record boundary ownership explicitly.

Why it was not pursued:

- There is low confidence that such metadata can be propagated through all of
  the relevant layout situations reliably enough.
- The concern is that it becomes another brittle parallel topology system.

This is cleaner than float equality, but confidence in the approach is low.

### 9. Endpoint-preserving direct rounding via exact equality

This was an experiment in [taffy.rs](/D:/source/zed/crates/gpui/src/taffy.rs).

Concept:

- Keep direct relative rounding for interior edges.
- If the child's raw far edge exactly equals the parent's raw content extent,
  use the parent's snapped content extent instead of rounding independently.

What it gets right:

- Preserves the direct-rounding interior behavior
- Preserves border thickness as authoritative
- Fixes the full-width/full-height seam in the cases it recognizes

Why it is uncomfortable:

- It depends on reconstructing parent raw content size from Taffy output.
- It depends on exact floating-point equality between separately reconstructed
  values.
- To make exact equality reliable, the code now tries to match Taffy's floating
  point operation order.

Even when it appeared to work empirically, it was not trusted as the right
long-term shape of the fix.

The core discomfort is architectural:

- the framework is rediscovering semantic boundary ownership from duplicated
  floating-point arithmetic
- success depends on the arithmetic staying aligned forever

That was considered too brittle, and the experiment was reverted.

## What We Learned From XAML / WinUI

The WinUI/XAML codebase was inspected in
`D:\source\microsoft-ui-xaml`.

Relevant files include:

- [uielement.cpp](/D:/source/microsoft-ui-xaml/src/dxaml/xcp/core/core/elements/uielement.cpp)
- [framework.cpp](/D:/source/microsoft-ui-xaml/src/dxaml/xcp/core/core/elements/framework.cpp)
- [Grid.cpp](/D:/source/microsoft-ui-xaml/src/dxaml/xcp/core/core/elements/Grid.cpp)
- [Border.cpp](/D:/source/microsoft-ui-xaml/src/dxaml/xcp/core/core/elements/Border.cpp)
- [TextBlock.cpp](/D:/source/microsoft-ui-xaml/src/dxaml/xcp/core/text/TextBlock/TextBlock.cpp)
- [HWCompNodeWinRT.cpp](/D:/source/microsoft-ui-xaml/src/dxaml/xcp/components/comptree/HWCompNodeWinRT.cpp)

Key findings:

1. **XAML does not appear to satisfy all three desired properties simultaneously**
   - It uses local plateau-aware rounding:
     `round(value * scale) / scale`
   - It rounds layout values in many places, independently
   - It uses panel-specific rounding and special cases
   - It also relies on compositor pixel snapping

2. **XAML does not have a magic global closure mechanism**
   - It rounds `X`, `Y`, `Width`, and `Height` independently in Arrange
   - It rounds desired sizes during Measure
   - Grid rounds some inputs and intermediates internally
   - Border thickness is rounded per side

3. **XAML has special cases because local rounding alone is not enough**
   - Framework code has compensating logic for cases where mixed rounded and
     unrounded values would otherwise fight
   - Text measurement is special-cased to avoid clipping

4. **XAML explicitly recognizes compositor jitter**
   - It documents "jiggle" in composition/pixel-snapping scenarios
   - It disables pixel snapping during certain transform animations or inertia
     to prevent jitter

Conclusion:

- XAML does not offer a simple trick that gets perfect closure, perfect
  placement stability, and perfect resize smoothness all at once.
- It instead separates concerns:
  - local layout rounding
  - panel-specific fixes
  - border/text special cases
  - compositor pixel snapping and jitter suppression

That suggests that not all remaining GPUI resize feel problems necessarily live
in `taffy.rs`.

## Browser Comparison Notes

A browser repro page was also built to compare feel against Chrome.

Qualitative observations:

- Chrome often felt smoother overall during resize
- Chrome also showed very aggressive SVG snapping/distortion in some cases
- Chrome could still show text/icon desynchronization in some frames
- GPUI had clearer cases where related labels seemed to move on different
  frames in ways that felt worse than Chrome

The takeaway is not that the browser is geometrically perfect. The takeaway is
that GPUI's current behavior still has a perceptual quality problem even if some
of the remaining artifacts are mathematically unavoidable.

## Current Code State

The current implementation in [taffy.rs](/D:/source/zed/crates/gpui/src/taffy.rs):

- uses direct relative rounding as the baseline
- preserves border thickness through snapped insets
- does **not** currently special-case exact boundary coincidence
- therefore still has the known full-width/full-height seam problem when
  snapped border/padding insets disagree with the raw content interval

The current implementation in [window.rs](/D:/source/zed/crates/gpui/src/window.rs):

- no longer contains the temporary rounding debug helpers
- still performs paint-time snapping for non-Taffy primitives

There is still a TODO in [taffy.rs](/D:/source/zed/crates/gpui/src/taffy.rs)
about duplicated border snapping logic between layout and paint.

## What Is Considered Unacceptable Going Forward

The next agent should assume the following are rejected unless there is a very
strong new argument:

1. Reintroducing proportional rescaling
2. Giving up exact border-thickness fidelity
3. Fixing this in controls instead of the framework
4. Disabling subpixel text rendering as the answer
5. Adding a solver to the layout hot path
6. Leaning harder into duplicated-float-equality topology reconstruction

## What Still Seems Plausible

The remaining plausible space appears to be some combination of:

1. Improving the architecture so boundary ownership is not inferred from
   duplicated float arithmetic
2. Reducing disagreement between layout-time and paint-time snapping
3. Auditing whether some of the remaining resize feel issues are actually in the
   paint/compositor path rather than Taffy snapping itself
4. Accepting the truly irreducible centered-parity cases and focusing only on
   the framework-caused incoherence

## Questions For The Next Agent

1. Is there a principled way to preserve border thickness and direct relative
   rounding without reconstructing boundary coincidence from duplicated float
   math?

2. Can Taffy expose enough structural information to identify true boundary
   attachment without a parallel metadata system in GPUI?

3. How much of the remaining perceived resize jitter is actually caused by
   paint/compositor snapping in [window.rs](/D:/source/zed/crates/gpui/src/window.rs)
   rather than the layout snapping in [taffy.rs](/D:/source/zed/crates/gpui/src/taffy.rs)?

4. Is there a way to make layout and paint share a single snapped box-model
   authority without compromising border thickness?

5. If the answer is still "some jitter is fundamental," can we at least make
   the remaining behavior look more like WinUI or browsers in perceptual terms,
   even if not in exact arithmetic?

## Bottom Line

The project is trying to achieve all of the following at once:

- exact edge closure
- placement stability
- good resize feel
- exact border-thickness fidelity

Proportional rescaling helped closure but looked wrong.
Direct rounding looked better but broke closure.
An endpoint-equality experiment repaired some closure, but it was not trusted as
a robust architectural answer and has been reverted in this handoff state.

The next step needs to be more principled than:

- "just rescale again"
- "just infer it from float equality"
- "just add more heuristics"

That is the actual handoff problem.
