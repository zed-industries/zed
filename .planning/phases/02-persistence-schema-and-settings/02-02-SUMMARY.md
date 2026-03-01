---
phase: 02-persistence-schema-and-settings
plan: "02"
subsystem: settings
tags: [settings, persistent-undo, configuration]
dependency_graph:
  requires:
    - 02-01 (SQLite schema and persistence DB)
  provides:
    - PersistentUndoSettingsContent in settings_content crate
    - PersistentUndoSettings resolved struct with Settings impl
    - persistent_undo field on SettingsContent
    - default.json block with enabled: false, max_entries: 10000
  affects:
    - Phase 3 (reads PersistentUndoSettings::get_global(cx).enabled and .max_entries)
tech_stack:
  added: []
  patterns:
    - RegisterSetting derive macro with inventory::submit! auto-registration
    - with_fallible_options macro for Option<T> content structs
    - Settings trait impl with from_settings reading SettingsContent fields
key_files:
  created: []
  modified:
    - crates/settings_content/src/editor.rs
    - crates/settings_content/src/settings_content.rs
    - crates/editor/src/editor_settings.rs
    - assets/settings/default.json
    - crates/settings/src/vscode_import.rs
decisions:
  - "Used RegisterSetting derive macro for auto-registration via inventory::submit! — no explicit register(cx) call needed in editor init()"
  - "PersistentUndoSettingsContent placed in editor.rs (settings_content crate) following the pattern for editor-adjacent settings"
  - "persistent_undo field placed alphabetically before proxy in SettingsContent"
metrics:
  duration: "~8min"
  completed: "2026-03-01"
  tasks_completed: 2
  tasks_total: 2
  files_modified: 5
---

# Phase 2 Plan 2: Persistent Undo Settings Summary

**One-liner:** PersistentUndoSettings wired into Zed's settings system with enabled (default: false) and max_entries (default: 10000) via RegisterSetting auto-registration.

## Tasks Completed

| Task | Name | Commit | Key Files |
|------|------|--------|-----------|
| 1 | Add PersistentUndoSettingsContent to settings_content | 4896c90746 | editor.rs, settings_content.rs |
| 2 | Add PersistentUndoSettings resolved struct and default.json | 52080864a0 | editor_settings.rs, default.json, vscode_import.rs |

## Decisions Made

1. **Auto-registration via inventory:** `RegisterSetting` derive macro uses `inventory::submit!` which automatically registers `PersistentUndoSettings` when `SettingsStore::new()` calls `load_settings_types()`. No explicit `register(cx)` call is needed in `editor::init()`. This matches how `EditorSettings` and `VimSettings` work.

2. **Content struct placement:** `PersistentUndoSettingsContent` was added to `crates/settings_content/src/editor.rs` (not a new file) following the pattern for editor-adjacent settings. The struct is re-exported via `pub use editor::*` in `settings_content.rs`.

3. **SettingsContent field ordering:** The `persistent_undo` field was placed before `proxy` alphabetically in `SettingsContent`, consistent with the existing alphabetical field ordering convention.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Added persistent_undo: None to vscode_import.rs SettingsContent initializer**
- **Found during:** Task 2 (cargo check -p editor)
- **Issue:** `SettingsContent` has an exhaustive struct initializer in `crates/settings/src/vscode_import.rs` that needed the new `persistent_undo` field.
- **Fix:** Added `persistent_undo: None` to the `settings_content()` method in `vscode_import.rs` in alphabetical position before `proxy`.
- **Files modified:** `crates/settings/src/vscode_import.rs`
- **Commit:** 52080864a0

## Verification Results

- `cargo check -p settings_content` — Finished successfully
- `cargo check -p editor` — Finished successfully
- `assets/settings/default.json` contains `"persistent_undo"` block with `"enabled": false` and `"max_entries": 10000`
- `PersistentUndoSettingsContent` has `#[with_fallible_options]`, `JsonSchema`, `MergeFrom` derives
- `PersistentUndoSettings` has `#[derive(RegisterSetting)]`

## Self-Check: PASSED

Files verified:
- FOUND: crates/settings_content/src/editor.rs (PersistentUndoSettingsContent struct)
- FOUND: crates/settings_content/src/settings_content.rs (persistent_undo field)
- FOUND: crates/editor/src/editor_settings.rs (PersistentUndoSettings struct + Settings impl)
- FOUND: assets/settings/default.json (persistent_undo block)
- FOUND: crates/settings/src/vscode_import.rs (persistent_undo: None)

Commits verified:
- FOUND: 4896c90746 (Task 1)
- FOUND: 52080864a0 (Task 2)
