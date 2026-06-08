# Plan: Show `info_count` in the Status Bar

The goal is consistency: the status bar should reflect what the diagnostics panel
shows. When `include_warnings` is true the panel now shows errors, warnings, and
info-level diagnostics, so the status bar counter should too.

No new icons, toolbar buttons, or actions. The `include_warnings` setting name
stays the same. Changes touch six areas.

---

## 1. Extend `DiagnosticSummary` — `crates/project/src/lsp_store.rs`

**Struct** — add `pub info_count: usize`.

**`DiagnosticSummary::new()`** — add an arm to the severity match:
```rust
DiagnosticSeverity::INFORMATION => this.info_count += 1,
```

**`is_empty()`** — no change. It gates the checkmark icon ("no errors or
warnings"). An info-only workspace still showing the checkmark is acceptable.

**`diagnostic_summary()`** — add `summary.info_count += path_summary.info_count`
alongside the existing error/warning accumulation.

**`diagnostic_summary_for_path()`** — expand the fold from a 2-tuple to a 3-tuple
`(error_count, warning_count, info_count)` and include it in the returned struct.

**`to_proto()`** — add `info_count: self.info_count as u32`.

---

## 2. Extend the proto message — `crates/proto/proto/lsp.proto`

Add a new field to `message DiagnosticSummary`:
```proto
uint32 info_count = 5;
```

Protobuf field ordering means old clients silently ignore the new field, so this
is backward-compatible. No DB schema or migration change is needed — diagnostics
are not persisted in the collab database.

After updating the schema, run the repo's normal proto regeneration/build path so
the Rust bindings for `proto::DiagnosticSummary` pick up the new field. Also keep
the proto change Buf-clean (`buf lint` / `buf format`) so CI passes.

---

## 3. Propagate `info_count` through proto serialization — `crates/project/src/lsp_store.rs`

Every site in `lsp_store.rs` that constructs `proto::DiagnosticSummary { ...,
error_count, warning_count }` needs `info_count` added as well.

Every site in `lsp_store.rs` that deserializes a proto message back into
`DiagnosticSummary { error_count, warning_count }` needs
`info_count: message_summary.info_count as usize`.

---

## 4. Show `info_count` in the status bar — `crates/diagnostics/src/items.rs`

In `render()`, keep the outer checkmark-vs-counts decision based on
`error_count` and `warning_count` only, so an info-only workspace still renders
the checkmark. Within the existing non-checkmark branch, after the existing
`.when(warning_count > 0, ...)` child, add:

```rust
let include_warnings = ProjectSettings::get_global(cx).diagnostics.include_warnings;
// ...
.when(self.summary.info_count > 0 && include_warnings, |this| {
    this.child(
        Icon::new(IconName::Info)
            .size(IconSize::Small)
            .color(Color::Info),
    )
    .child(Label::new(self.summary.info_count.to_string()).size(LabelSize::Small))
})
```

`IconName::Info` and `Color::Info` already exist and are used elsewhere; no new
icons are introduced.

The auto-enable logic in the `on_click` handler (which sets `include_warnings =
true` when the user clicks the status bar with errors = 0 and warnings > 0) does
not need to change — info-only scenarios are rare and can be handled in a follow-up.

---

## 5. Fix the collab integration test — `crates/collab/tests/integration/integration_tests.rs`

`test_collaborating_with_diagnostics` constructs `DiagnosticSummary` literals.
Add `info_count: 0` to each, or derive `Default` on the struct and use
`..Default::default()` struct-update syntax.

---

## 6. Fix the diagnostics unit test — `crates/diagnostics/src/diagnostics_tests.rs`

`test_buffer_diagnostics_multiple_servers` asserts on `*buffer_diagnostics.summary()`
using a `DiagnosticSummary` literal. Add `info_count: 0` there too.

---

## Summary of files touched

| File | Change |
|------|--------|
| `crates/project/src/lsp_store.rs` | Add `info_count` to struct, `new()`, `is_empty()` (no-op), `diagnostic_summary()`, `diagnostic_summary_for_path()`, `to_proto()`, and all proto construction/deserialization sites |
| `crates/proto/proto/lsp.proto` | Add `uint32 info_count = 5` to `DiagnosticSummary` message and regenerate/build the Rust bindings as required by the repo's proto pipeline |
| `crates/diagnostics/src/items.rs` | Render `info_count` in status bar when `include_warnings` is true |
| `crates/collab/tests/integration/integration_tests.rs` | Add `info_count: 0` to `DiagnosticSummary` literals |
| `crates/diagnostics/src/diagnostics_tests.rs` | Add `info_count: 0` to `DiagnosticSummary` literal |