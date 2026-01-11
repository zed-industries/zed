# Project Search Presets - Implementation Plan

## Step 1: Add Settings Type

### Changes

**`crates/settings/src/settings_content.rs`**
- Add `ProjectSearchPreset` struct with `include: Option<String>` and `exclude: Option<String>`
- Add `project_search_presets: Option<IndexMap<String, ProjectSearchPreset>>` field to `SettingsContent`

**`assets/settings/default.json`**
- Add `"project_search_presets": {}`

### Tests
- Add unit test that parses settings JSON containing presets
- Verify preset names and values are correctly extracted

---

## Step 2: Add Action and Picker

### Changes

**`crates/search/src/project_search.rs`**
- Add `SearchWithPreset` to the `actions!` macro
- Create `SearchPresetPickerDelegate` struct implementing `PickerDelegate`:
  - Stores list of `(String, ProjectSearchPreset)` tuples (name + preset)
  - `match_count()` returns preset count
  - `render_match()` renders preset name as label
  - `no_matches_text()` returns "No search presets defined. Add presets to .zed/settings.json"
  - `update_matches()` filters presets by query string
  - `confirm()` stores selected preset for later use
- Create `SearchPresetPicker` view wrapping the picker
- Register `SearchWithPreset` action in `ProjectSearchBar::register()`:
  - Get first visible worktree from workspace
  - Load presets via settings with `SettingsLocation`
  - Open picker modal via `workspace.toggle_modal()`

### Tests
- Test that action with no presets shows empty state message
- Test that action with presets shows preset names in picker
- Test that filtering works correctly

---

## Step 3: Wire Picker to DeploySearch

### Changes

**`crates/search/src/project_search.rs`**
- In picker's `confirm()`:
  - Get selected preset's include/exclude values
  - Dispatch `DeploySearch` action with `included_files` and `excluded_files` set
  - Emit `DismissEvent` to close picker

### Tests
- Test that selecting a preset opens project search with correct include value
- Test that selecting a preset opens project search with correct exclude value
- Test that filters panel is expanded when preset has values
- Test preset with only include (no exclude)
- Test preset with only exclude (no include)
- Test preset with both include and exclude

---

## File Summary

| File | Changes |
|------|---------|
| `crates/settings/src/settings_content.rs` | Add `ProjectSearchPreset` struct and field |
| `assets/settings/default.json` | Add empty default |
| `crates/search/src/project_search.rs` | Add action, picker delegate, registration, wiring |

---

## Notes

- The picker delegate and view can be defined inline in `project_search.rs` - no need for a separate file
- Reuse existing `DeploySearch` action rather than duplicating filter-population logic
- Keep picker simple: just preset names, no secondary text or icons
