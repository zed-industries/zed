# Merman Feature Requests

## Support `"look": "classic"` to disable rough.js rendering

Merman unconditionally renders all shapes using `roughr-merman` (a Rust port of
rough.js), producing hand-drawn style paths. Each simple rectangle becomes ~90
cubic bezier curves (~10 KB) plus a duplicate fill pass of ~350 curves (~25 KB).
For a 4-state diagram this adds up to ~150 KB of path data alone.

The actual wobble introduced by rough.js is under 2 pixels — invisible at
typical diagram zoom levels. The diagrams look clean and smooth, but cost
100× more bytes than equivalent `<rect>` elements.

Mermaid.js v11+ supports a `"look"` config option (`"classic"` vs
`"handDrawn"`). Adding support for `"look": "classic"` in merman would allow
callers to opt out of roughr and produce compact SVGs with simple geometric
primitives.

## Edge label collision avoidance for bidirectional edges

When two edges connect the same pair of states in opposite directions (e.g.
`Draft --> Review : submit` and `Review --> Draft : Rejected`), merman places
both edge labels at nearly the same y-coordinate with overlapping horizontal
extents. The labels render on top of each other, making both unreadable.

Merman should detect when edge labels overlap and offset them vertically or
along the edge path to avoid collisions.

## `foreign_object_label_fallback_svg_text` drops closing angle brackets

When a class diagram uses generics (e.g. `class Entity~ID~`), the foreignObject
contains `Entity&lt;ID>` (opening bracket escaped, closing bracket literal —
valid XML). However, `foreign_object_label_fallback_svg_text` generates fallback
`<text>` with `Entity&amp;lt;ID` — the closing `>` is silently dropped.

The downstream consumer can fix the double-escaped `&amp;lt;` → `&lt;`, but
cannot recover the missing `>` since there is no indicator of where it belonged.
The result is displayed as `Entity<ID` instead of `Entity<ID>`.
