# Project Search Presets - Implementation Research

## 1. Settings System

### Defining Settings Structs
Location: `crates/settings/src/settings_content.rs`

Settings structs use these derives:
```rust
#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
```

`JsonSchema` derive automatically generates schema for editor autocompletion.

### SettingsContent
The main `SettingsContent` struct (line 35-166) holds all settings fields. Map fields use `IndexMap<String, T>` for ordered keys.

Example map field from `settings_content/agent.rs:70`:
```rust
pub profiles: Option<IndexMap<Arc<str>, AgentProfileContent>>,
```

### Registered Settings Types
Settings are accessed via types implementing `Settings` trait. Pattern from `workspace/src/workspace_settings.rs:12-90`:

```rust
#[derive(RegisterSetting)]
pub struct WorkspaceSettings {
    pub confirm_quit: bool,
    // ...
}

impl Settings for WorkspaceSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            confirm_quit: content.workspace.confirm_quit.unwrap(),
            // ...
        }
    }
}
```

### Default Values
Defaults are in `assets/settings/default.json`. Empty maps use `{}`.

## 2. Project-Specific Settings Access

### SettingsLocation (`settings_store.rs:138-142`)
```rust
pub struct SettingsLocation<'a> {
    pub worktree_id: WorktreeId,
    pub path: &'a RelPath,
}
```

### Getting Worktree from Workspace
```rust
workspace.project().read(cx).visible_worktrees(cx).next()
```

### Accessing with Location
```rust
let location = SettingsLocation {
    worktree_id: worktree.read(cx).id(),
    path: RelPath::empty(),
};
let settings = SomeSettings::get(Some(location), cx);
```

Settings resolution is hierarchical - project `.zed/settings.json` values override user settings.

## 3. Actions

### Defining Actions
`project_search.rs` line 52-65:
```rust
actions!(search, [ToggleFocus, NextField, SearchInNew, ...]);
```

### Registering on Workspace
In `ProjectSearchBar::register()` (line 158-193):
```rust
workspace.register_action(|workspace, action: &SomeAction, window, cx| {
    // handle
});
```

## 4. Picker System

### PickerDelegate Trait (`picker/src/picker.rs:92-259`)
Key methods:
- `match_count()` - number of items
- `selected_index()` / `set_selected_index()` - selection state
- `placeholder_text()` - search placeholder
- `update_matches()` - filter on query
- `confirm()` - handle selection
- `dismissed()` - handle cancel
- `render_match()` - render each item
- `no_matches_text()` - empty state message (default: "No matches")

### Creating Picker
```rust
Picker::uniform_list(delegate, window, cx)
```

### Opening as Modal
```rust
workspace.toggle_modal(window, cx, move |window, cx| {
    SomePicker::new(data, window, cx)
});
```

### Reference Examples
- Simple picker: `settings_ui/src/components/theme_picker.rs`
- Action → modal flow: `language_selector/src/language_selector.rs:39-84`

## 5. Project Search Internals

### DeploySearch Action (`workspace/src/pane.rs:187-198`)
```rust
pub struct DeploySearch {
    pub replace_enabled: bool,
    pub included_files: Option<String>,
    pub excluded_files: Option<String>,
}
```

### How Include/Exclude Are Applied (`project_search.rs:1116-1127`)
```rust
if let Some(included_files) = action.included_files.as_deref() {
    search.included_files_editor
        .update(cx, |editor, cx| editor.set_text(included_files, window, cx));
    search.filters_enabled = true;
}
if let Some(excluded_files) = action.excluded_files.as_deref() {
    search.excluded_files_editor
        .update(cx, |editor, cx| editor.set_text(excluded_files, window, cx));
    search.filters_enabled = true;
}
```

### ProjectSearchView Struct (`project_search.rs:223-242`)
```rust
pub struct ProjectSearchView {
    workspace: WeakEntity<Workspace>,
    included_files_editor: Entity<Editor>,
    excluded_files_editor: Entity<Editor>,
    filters_enabled: bool,
    // ...
}
```

## 6. Files of Interest

| File | Relevance |
|------|-----------|
| `crates/settings/src/settings_content.rs` | Settings struct definitions |
| `crates/settings/src/settings_store.rs` | Settings access, SettingsLocation |
| `crates/workspace/src/workspace_settings.rs` | Example registered settings type |
| `crates/picker/src/picker.rs` | PickerDelegate trait |
| `crates/language_selector/src/language_selector.rs` | Action → modal → picker example |
| `crates/search/src/project_search.rs` | Project search, DeploySearch handling |
| `crates/workspace/src/pane.rs:187-198` | DeploySearch action definition |
| `assets/settings/default.json` | Default settings values |
