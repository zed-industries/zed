# Adding a new settings section in current Zed (Settings trait, settings_content, default.json, update_settings_file)

# Zed settings API reference: adding a new settings section

Architecture (4 required pieces + 1 optional):
1. A **content struct** (`*SettingsContent`, all fields `Option<T>`) in the `settings_content` crate, added as a field on `SettingsContent`.
2. **Defaults** for every field in `assets/settings/default.json` (embedded via RustEmbed; `from_settings` deliberately `.unwrap()`s, so a missing default panics at startup).
3. A **resolved settings struct** in your feature crate implementing `settings::Settings` and derived with `#[derive(RegisterSetting)]` (auto-registers via `inventory` — no manual `register()` call needed).
4. Reads via `MySettings::get_global(cx)`; writes via `settings::update_settings_file(...)`.
5. (Optional) an entry in the Settings UI: `crates/settings_ui/src/page_data.rs`.

Note: the `settings` crate re-exports everything from `settings_content` (`pub use ::settings_content::*;` — `/Users/user/zed/crates/settings/src/settings.rs:33`), so feature crates only import from `settings::`.

---

## 1. Feature-crate side: `crates/project_panel/src/project_panel_settings.rs` (full pattern)

Resolved struct + `RegisterSetting` derive (lines 14–41):
```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    DockSide, ProjectPanelEntrySpacing, ProjectPanelSortMode, ProjectPanelSortOrder,
    RegisterSetting, Settings, ShowDiagnostics, ShowIndentGuides,
};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, RegisterSetting)]
pub struct ProjectPanelSettings {
    pub button: bool,
    pub hide_gitignore: bool,
    pub default_width: Pixels,
    pub dock: DockSide,
    // ... plain (non-Option) resolved fields ...
    pub indent_guides: IndentGuidesSettings,
    pub scrollbar: ScrollbarSettings,
    pub auto_open: AutoOpenSettings,
}
```
Nested resolved sub-structs are plain serde types (lines 43–66):
```rust
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettings {
    pub show: Option<ShowScrollbar>,      // None = inherit editor setting
    pub horizontal_scroll: bool,
}
```
`Settings` trait impl — maps `SettingsContent` -> resolved struct, `.unwrap()` on every field because default.json must provide it (lines 97–150, abridged):
```rust
impl Settings for ProjectPanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let project_panel = content.project_panel.clone().unwrap();
        Self {
            button: project_panel.button.unwrap(),
            hide_gitignore: project_panel.hide_gitignore.unwrap(),
            default_width: px(project_panel.default_width.unwrap()),
            dock: project_panel.dock.unwrap(),
            indent_guides: IndentGuidesSettings {
                show: project_panel.indent_guides.unwrap().show.unwrap(),
            },
            scrollbar: {
                let scrollbar = project_panel.scrollbar.unwrap();
                ScrollbarSettings {
                    show: scrollbar.show.map(ui_scrollbar_settings_from_raw),
                    horizontal_scroll: scrollbar.horizontal_scroll.unwrap(),
                }
            },
            auto_open: {
                let auto_open = project_panel.auto_open.unwrap();
                AutoOpenSettings {
                    on_create: auto_open.on_create.unwrap(),
                    on_paste: auto_open.on_paste.unwrap(),
                    on_drop: auto_open.on_drop.unwrap(),
                }
            },
            // ...
        }
    }
}
```

### Registration mechanics
`#[derive(RegisterSetting)]` (`/Users/user/zed/crates/settings_macros/src/settings_macros.rs:85-105`) expands to an `inventory::submit!` of a `settings::private::RegisteredSetting { settings_value, from_settings, id }`. `SettingsStore::from_settings_content` calls `load_settings_types()` which iterates `inventory::iter::<RegisteredSetting>()` (`/Users/user/zed/crates/settings/src/settings_store.rs:420-424`) — so **deriving `RegisterSetting` is all you need; there is no per-crate `MySettings::register(cx)` call anymore** (a manual `Settings::register(cx)` still exists at `/Users/user/zed/crates/settings/src/settings_store.rs:76-84` but is unnecessary with the derive). The store itself is created once in `settings::init` (`/Users/user/zed/crates/settings/src/settings.rs:125-129`):
```rust
pub fn init(cx: &mut App) {
    let settings = SettingsStore::new(cx, &default_settings());
    cx.set_global(settings);
    ...
}
pub fn default_settings() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/default.json")  // RustEmbed of ../../assets
}
```

### Settings trait (key methods), `/Users/user/zed/crates/settings/src/settings_store.rs:60-128`
```rust
pub trait Settings: 'static + Send + Sync + Sized {
    const PRESERVED_KEYS: Option<&'static [&'static str]> = None;

    /// Read the value from default.json.
    /// This function *should* panic if default values are missing.
    fn from_settings(content: &SettingsContent) -> Self;

    fn register(cx: &mut App);                                        // rarely needed; derive instead
    fn get<'a>(path: Option<SettingsLocation>, cx: &'a App) -> &'a Self;  // per-worktree/local
    fn get_global(cx: &App) -> &Self;
    fn try_get(cx: &App) -> Option<&Self>;
    fn try_read_global<R>(cx: &AsyncApp, f: impl FnOnce(&Self) -> R) -> Option<R>;
    fn override_global(settings: Self, cx: &mut App);                 // tests
}
```

## 2. Reading settings
```rust
let settings = ProjectPanelSettings::get_global(cx);   // returns &ProjectPanelSettings
```
Real use: `/Users/user/zed/crates/project_panel/src/project_panel_settings.rs:90`. Reacting to changes — observe the global `SettingsStore` and diff (`/Users/user/zed/crates/project_panel/src/project_panel.rs:812-839`):
```rust
let mut project_panel_settings = *ProjectPanelSettings::get_global(cx);
cx.observe_global_in::<SettingsStore>(window, move |this, window, cx| {
    let new_settings = *ProjectPanelSettings::get_global(cx);
    if project_panel_settings != new_settings {
        // react to specific field diffs...
        project_panel_settings = new_settings;
        cx.notify();
    }
})
.detach();
```

## 3. Writing settings: `update_settings_file`
Free function, `/Users/user/zed/crates/settings/src/settings_file.rs:269-283`:
```rust
pub fn update_settings_file(
    fs: Arc<dyn Fs>,
    cx: &App,
    update: impl 'static + Send + FnOnce(&mut SettingsContent, &App),
)
pub fn update_settings_file_with_completion(...) -> futures::channel::oneshot::Receiver<anyhow::Result<()>>
```
(Also available as a method: `SettingsStore::update_settings_file(&self, fs, update)`, `/Users/user/zed/crates/settings/src/settings_store.rs:616-622`.)

Real call site — `/Users/user/zed/crates/project_panel/src/project_panel.rs:474-485`:
```rust
workspace.register_action(|workspace, _: &ToggleHideGitIgnore, _, cx| {
    let fs = workspace.app_state().fs.clone();
    update_settings_file(fs, cx, move |setting, _| {
        setting.project_panel.get_or_insert_default().hide_gitignore = Some(
            !setting.project_panel.get_or_insert_default().hide_gitignore.unwrap_or(false),
        );
    })
});
```
Real call site via store — `/Users/user/zed/crates/git_ui/src/git_panel.rs:3816-3826`:
```rust
cx.update_global::<SettingsStore, _>(|store, _cx| {
    store.update_settings_file(fs, move |settings, _cx| {
        settings.git_panel.get_or_insert_default().sort_by = Some(GitPanelSortBy::Path);
    });
});
```
The closure mutates `SettingsContent`; the store serializes the diff back into the user's `settings.json` preserving comments/formatting (via `new_text_for_update`, settings_store.rs:831).

## 4. Content struct: `settings_content` crate
Root struct `SettingsContent` — `/Users/user/zed/crates/settings_content/src/settings_content.rs:114-116`, your section is a top-level `Option` field (line 212):
```rust
#[with_fallible_options]
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SettingsContent {
    ...
    pub project_panel: Option<ProjectPanelSettingsContent>,   // line 212
    ...
}
```
Section content struct — `/Users/user/zed/crates/settings_content/src/workspace.rs:718-822` (abridged). **Every field is `Option<T>`**; doc comments become the JSON-schema docs shown to users and must state the default:
```rust
use settings_macros::{MergeFrom, with_fallible_options};

#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct ProjectPanelSettingsContent {
    /// Whether to show the project panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Whether to hide gitignore files in the project panel.
    ///
    /// Default: false
    pub hide_gitignore: Option<bool>,
    /// Customize default width (in pixels) taken by project panel
    ///
    /// Default: 240
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_width: Option<f32>,
    pub scrollbar: Option<ProjectPanelScrollbarSettingsContent>,
    pub auto_open: Option<ProjectPanelAutoOpenSettings>,
    // ...
}
```
Required derives/attributes on content structs: `#[with_fallible_options]` (proc-macro attr that adds `#[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "settings::deserialize_fallible")]` to every `Option` field — settings_macros.rs:107-134), plus `Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug`. Enums used in settings get `#[serde(rename_all = "snake_case")]` and typically `Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq, strum::VariantArray, strum::VariantNames` (see `ProjectPanelEntrySpacing`, workspace.rs:824-839, or `HideMouseMode`, settings_content.rs:89-112). `MergeFrom` (from `settings_macros`) implements layered merging default < user < project.

## 5. `assets/settings/default.json` — REQUIRED
Yes, every new field needs a default entry, otherwise `from_settings`'s `.unwrap()` panics on startup (trait doc, settings_store.rs:70-74: "This function *should* panic if default values are missing"). Existing section at `/Users/user/zed/assets/settings/default.json:777`:
```jsonc
"project_panel": {
    // Whether to show the project panel button in the status bar
    "button": true,
    "hide_gitignore": false,
    "default_width": 240,
    "dock": "right",
    ...
}
```

## 6. Optional: Settings UI page entry
To surface the setting in Zed's graphical settings editor, add a `SettingsPageItem::SettingItem` in `/Users/user/zed/crates/settings_ui/src/page_data.rs` (example, lines 4964-4986):
```rust
SettingsPageItem::SettingItem(SettingItem {
    title: "Hide .gitignore",
    description: "Whether to hide the gitignore entries in the project panel.",
    field: Box::new(SettingField {
        organization_override: None,
        json_path: Some("project_panel.hide_gitignore"),
        pick: |settings_content| {
            settings_content.project_panel.as_ref()?.hide_gitignore.as_ref()
        },
        write: |settings_content, value, _| {
            settings_content.project_panel.get_or_insert_default().hide_gitignore = value;
        },
    }),
    metadata: None,
    files: USER,
}),
```

## Checklist for a new section `foo`
1. `crates/settings_content/src/<module>.rs`: define `FooSettingsContent` (`#[with_fallible_options]`, all-`Option` fields, doc comments with `Default:`); add `pub foo: Option<FooSettingsContent>` to `SettingsContent` in `settings_content.rs`.
2. `assets/settings/default.json`: add `"foo": { ... }` with every field's default.
3. Feature crate: `#[derive(Deserialize, Debug, Clone, PartialEq, RegisterSetting)] pub struct FooSettings { ... }` + `impl Settings for FooSettings { fn from_settings(content) { content.foo.clone().unwrap() -> unwrap each field } }`. Crate needs `settings`, `serde`, `schemars` and `gpui` deps.
4. Read with `FooSettings::get_global(cx)`; write with `settings::update_settings_file(fs, cx, |content, _| { content.foo.get_or_insert_default().field = Some(v); })`; react with `cx.observe_global_in::<SettingsStore>(...)` diffing old vs new.
5. Optionally add settings_ui `page_data.rs` items.