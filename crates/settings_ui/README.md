# Settings UI Guide

This guide explains how to add settings to Zed's Settings UI, with a focus on enum-based settings that render as dropdowns.

## Overview

Settings in Zed are defined in four locations:
1. **Settings Content** (`settings/src/settings_content/*.rs`) - Defines the JSON schema and serialization
2. **Project Settings** (`project/src/project_settings.rs`) - Defines the runtime types used by the application
3. **Settings UI** (`settings_ui/src/page_data.rs`) - Defines how settings appear in the Settings window
4. **Renderer Registration** (`settings_ui/src/settings_ui.rs`) - Registers how enum types are rendered (dropdowns, etc.)

## Creating an Enum Setting

### Step 1: Define the Settings Content Enum

Location: `crates/settings/src/settings_content/project.rs` (or other content files)

```rust
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum MyEnumSetting {
    #[default]
    OptionOne,
    OptionTwo,
    OptionThree,
}
```

**Required Derives:**
- Standard: `Copy, Clone, Debug, PartialEq, Default`
- Serialization: `Serialize, Deserialize, JsonSchema`
- Settings: `MergeFrom`
- **UI Support (for dropdowns)**: `strum::VariantArray, strum::VariantNames`

**Required Attributes:**
- `#[serde(rename_all = "snake_case")]` - JSON uses snake_case
- `#[default]` - Mark one variant as the default

**Note:** `strum::Display` and `strum::EnumIter` are NOT required for dropdown rendering.

### Step 2: Add to Settings Structure

```rust
#[skip_serializing_none]
#[derive(Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct MySettings {
    /// Description of the setting
    ///
    /// Default: option_one
    pub my_enum: Option<MyEnumSetting>,
}
```

### Step 3: Define the Project Settings Enum

Location: `crates/project/src/project_settings.rs`

```rust
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum MyEnumSetting {
    #[default]
    OptionOne,
    OptionTwo,
    OptionThree,
}

impl From<settings::MyEnumSetting> for MyEnumSetting {
    fn from(action: settings::MyEnumSetting) -> Self {
        match action {
            settings::MyEnumSetting::OptionOne => MyEnumSetting::OptionOne,
            settings::MyEnumSetting::OptionTwo => MyEnumSetting::OptionTwo,
            settings::MyEnumSetting::OptionThree => MyEnumSetting::OptionThree,
        }
    }
}

pub struct MySettings {
    /// Description
    ///
    /// Default: option_one
    pub my_enum: MyEnumSetting,
}
```

### Step 4: Add Merge Logic

In `ProjectSettings::from_settings`:

```rust
my_settings: {
    let my = content.my_settings.unwrap();
    MySettings {
        my_enum: my.my_enum.unwrap().into(),
    }
}
```

### Step 5: Add to Default Settings

Location: `assets/settings/default.json`

```json
{
  "my_settings": {
    "my_enum": "option_one"
  }
}
```

### Step 6: Add to Settings UI

Location: `crates/settings_ui/src/page_data.rs`

```rust
SettingsPageItem::SectionHeader("My Settings"),
SettingsPageItem::SettingItem(SettingItem {
    title: "My Enum",
    description: "Description of what this setting does.",
    field: Box::new(SettingField {
        json_path: Some("my_settings.my_enum"),
        pick: |settings_content| {
            settings_content
                .my_settings
                .as_ref()?
                .my_enum
                .as_ref()
        },
        write: |settings_content, value| {
            settings_content
                .my_settings
                .get_or_insert_default()
                .my_enum = value;
        },
    }),
    metadata: None,
    files: USER,
}),
```

### Step 7: Register the Enum Renderer

**CRITICAL STEP:** Location: `crates/settings_ui/src/settings_ui.rs`

Add your enum type to the `init_renderers` function so the UI knows how to render it:

```rust
fn init_renderers(cx: &mut App) {
    cx.default_global::<SettingFieldRenderer>()
        // ... other renderers ...
        .add_basic_renderer::<settings::MyEnumSetting>(render_dropdown)
        // ... more renderers ...
        ;
}
```

**This step is required!** Without registering the renderer, your enum will display as a blue "no renderer" label instead of a proper dropdown.

## Key Patterns

### Reading Settings

The `pick` closure navigates the settings structure:
```rust
pick: |settings_content| {
    settings_content
        .parent
        .as_ref()?              // Return None if parent doesn't exist
        .child
        .as_ref()?              // Return None if child doesn't exist
        .field
        .as_ref()               // Return the Option<T>
}
```

### Writing Settings

The `write` closure updates the settings structure:
```rust
write: |settings_content, value| {
    settings_content
        .parent
        .get_or_insert_default()  // Create parent if it doesn't exist
        .child
        .get_or_insert_default()  // Create child if it doesn't exist
        .field = value;           // Set the value
}
```

### Nested Objects

For deeply nested settings:
```rust
pick: |settings_content| {
    settings_content
        .git
        .as_ref()?
        .diff_views
        .as_ref()?
        .commit_quick_action
        .as_ref()
}

write: |settings_content, value| {
    settings_content
        .git
        .get_or_insert_default()
        .diff_views
        .get_or_insert_default()
        .commit_quick_action = value;
}
```

## Boolean Settings

For boolean settings, the pattern is simpler:

```rust
SettingsPageItem::SettingItem(SettingItem {
    title: "Enable Feature",
    description: "Whether to enable this feature.",
    field: Box::new(SettingField {
        json_path: Some("my_settings.enabled"),
        pick: |settings_content| {
            settings_content.my_settings.as_ref()?.enabled.as_ref()
        },
        write: |settings_content, value| {
            settings_content.my_settings.get_or_insert_default().enabled = value;
        },
    }),
    metadata: None,
    files: USER,
}),
```

## File Scope

Settings can be scoped to:
- `USER` - User's global settings
- `PROJECT` - Project-specific settings

```rust
files: USER,    // Most common
files: PROJECT, // For project-specific settings
```

## Troubleshooting

### "No Renderer" Error

If dropdowns show as blue labels saying "no renderer", this means the enum type is not registered in the renderer system. Check these items:

1. **Most Common Issue:** The enum is not registered in `settings_ui/src/settings_ui.rs` in the `init_renderers` function. Add:
   ```rust
   .add_basic_renderer::<settings::YourEnumType>(render_dropdown)
   ```

2. Ensure your enum has the required strum derives for dropdowns:
   - `strum::VariantArray`
   - `strum::VariantNames`
   
   Note: `strum::Display` and `strum::EnumIter` are NOT required.

3. Ensure serde uses snake_case:
   ```rust
   #[serde(rename_all = "snake_case")]
   ```

### Compilation Errors

Common issues:
1. Missing `From` implementation between settings content and project settings types
2. Forgetting to add the setting to `from_settings` merge logic
3. Mismatched enum variant names between content and project types
4. Forgetting to register the enum renderer in `init_renderers` (causes "no renderer" at runtime, not compile time)

## Example: Complete Enum Setting

This example shows a complete implementation from the git diff_views settings:

**Settings Content:**
```rust
#[derive(Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom,
         strum::VariantArray, strum::VariantNames)]
#[serde(rename_all = "snake_case")]
pub enum DiffViewQuickAction {
    #[default]
    OpenFromCursor,
    OpenHead,
    OpenFinder,
    OpenParent,
    OpenModified,
}
```

**Project Settings:**
```rust
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum DiffViewQuickAction {
    #[default]
    OpenFromCursor,
    OpenHead,
    OpenFinder,
    OpenParent,
    OpenModified,
}

impl From<settings::DiffViewQuickAction> for DiffViewQuickAction {
    fn from(action: settings::DiffViewQuickAction) -> Self {
        match action {
            settings::DiffViewQuickAction::OpenFromCursor => DiffViewQuickAction::OpenFromCursor,
            settings::DiffViewQuickAction::OpenHead => DiffViewQuickAction::OpenHead,
            settings::DiffViewQuickAction::OpenFinder => DiffViewQuickAction::OpenFinder,
            settings::DiffViewQuickAction::OpenParent => DiffViewQuickAction::OpenParent,
            settings::DiffViewQuickAction::OpenModified => DiffViewQuickAction::OpenModified,
        }
    }
}
```

**Settings UI:**
```rust
SettingsPageItem::SettingItem(SettingItem {
    title: "Quick Action",
    description: "Action to perform when using the keybind (alt+enter).",
    field: Box::new(SettingField {
        json_path: Some("git.diff_views.commit_quick_action"),
        pick: |settings_content| {
            settings_content
                .git
                .as_ref()?
                .diff_views
                .as_ref()?
                .commit_quick_action
                .as_ref()
        },
        write: |settings_content, value| {
            settings_content
                .git
                .get_or_insert_default()
                .diff_views
                .get_or_insert_default()
                .commit_quick_action = value;
        },
    }),
    metadata: None,
    files: USER,
}),
```

**Renderer Registration:**
```rust
// In settings_ui/src/settings_ui.rs, fn init_renderers()
fn init_renderers(cx: &mut App) {
    cx.default_global::<SettingFieldRenderer>()
        // ... other renderers ...
        .add_basic_renderer::<settings::DiffViewQuickAction>(render_dropdown)
        .add_basic_renderer::<settings::DiffViewFallbackAction>(render_dropdown)
        // ... more renderers ...
        ;
}
```

**Default Settings:**
```json
{
  "git": {
    "diff_views": {
      "commit_quick_action": "open_from_cursor"
    }
  }
}
```
