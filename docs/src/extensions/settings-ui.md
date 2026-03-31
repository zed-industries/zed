---
title: Extension Settings UI
description: "Add schema-driven settings UI to a Zed extension."
---

# Extension Settings UI {#extension-settings-ui}

Zed extensions can contribute their own settings UI through a schema-driven API.

This lets an extension:

- declare a JSON schema for its settings
- declare default settings
- automatically appear in Zed's **Settings > Languages & Tools > Extensions** section
- read effective settings from user and project settings files

## Requirements

Extension settings UI is available only for:

- WebAssembly extensions
- extensions using `zed_extension_api` `0.9.0` or newer

Theme-only, snippet-only, and other non-WASM extensions do not get custom settings pages.

## How Zed Stores Extension Settings

Extension settings live under the top-level `extensions` namespace:

```jsonc
{
  "extensions": {
    "my-extension": {
      "enabled": true
    }
  }
}
```

This works in both:

- the user `settings.json`
- the project `.zed/settings.json`

Zed merges extension settings in this order:

1. extension `default_settings`
2. user settings
3. project settings

Object values are deep-merged. Non-object leaf values are replaced by the higher-precedence layer.

## 1. Declare a Settings Contribution

In your extension, implement `settings_contribution` on the `Extension` trait:

```rust
use zed_extension_api as zed;

struct MyExtension;

impl zed::Extension for MyExtension {
    fn new() -> Self {
        Self
    }

    fn settings_contribution(&mut self) -> Option<zed::ExtensionSettingsContribution> {
        Some(zed::ExtensionSettingsContribution {
            settings_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "enabled": {
                        "type": "boolean",
                        "description": "Enable the extension's custom behavior."
                    },
                    "binary_path": {
                        "type": "string",
                        "description": "Override the path to the tool used by the extension."
                    },
                    "features": {
                        "type": "object",
                        "properties": {
                            "format_on_save": { "type": "boolean" }
                        },
                        "additionalProperties": false
                    }
                },
                "additionalProperties": false
            }),
            default_settings: serde_json::json!({
                "enabled": true,
                "features": {
                    "format_on_save": false
                }
            }),
        })
    }
}

zed::register_extension!(MyExtension);
```

## 2. What Zed Does With It

When Zed loads the extension, it:

1. calls `settings_contribution`
2. validates `default_settings` against `settings_schema`
3. registers a settings schema at `zed://schemas/settings/extensions/<extension-id>`
4. adds a page for the extension in the Settings UI
5. injects the extension's defaults into the settings merge stack

If the schema is invalid, or the defaults do not match the schema, Zed ignores the settings contribution and logs an error. The extension itself still loads.

## 3. How the Settings UI Behaves

Each contributing extension gets its own page under:

- `Settings`
- `Languages & Tools`
- `Extensions`

That page shows:

- the extension name and description
- typed controls for supported schemas
- a raw JSON editor fallback for unsupported schemas

Typed controls are available for this first-pass subset:

- root object schemas
- nested objects
- `boolean`
- `string`
- `integer`
- `number`
- string enums
- string arrays (`string[]`)

Zed still falls back to raw JSON when the root schema is unsupported.
For unsupported array fields, Zed keeps the rest of the page typed and falls back to raw JSON for just that field.

Both typed controls and the raw JSON fallback edit only the currently selected file:

- `User` writes to user settings
- `Project` writes to project settings

Zed does not copy inherited values into the project file automatically.

## 4. Read Settings From Your Extension

Use `zed::settings::ExtensionSettings` to read the effective settings for your extension.

For a single worktree:

```rust
#[derive(serde::Deserialize)]
struct MySettings {
    enabled: bool,
    binary_path: Option<String>,
}

fn load_settings_for_worktree(worktree: &zed::Worktree) -> zed::Result<MySettings> {
    zed::settings::ExtensionSettings::for_worktree("my-extension", worktree)
}
```

For a project:

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct MySettings {
    enabled: bool,
    binary_path: Option<String>,
}

fn load_settings_for_project(project: &zed::Project) -> zed::Result<MySettings> {
    zed::settings::ExtensionSettings::for_project("my-extension", project)
}
```

Use the same extension ID that appears in `extension.toml`.

## 5. Recommended Settings Shape

Use an object as the root of your settings:

```json
{
  "enabled": true,
  "binary_path": null
}
```

This gives the best behavior for:

- user/project overrides
- deep merges
- JSON schema validation
- future settings expansion

## 6. Pull-Only Model

Extension settings are currently pull-only.

That means:

- Zed does not push settings change notifications into the extension
- there is no `on_setting_changed` callback
- the extension should read settings when it needs them

For example, read settings when:

- building a command
- starting a language server
- computing configuration for a request

## 7. Example Settings Files

User settings:

```jsonc
{
  "extensions": {
    "my-extension": {
      "enabled": true,
      "features": {
        "format_on_save": true
      }
    }
  }
}
```

Project settings:

```jsonc
{
  "extensions": {
    "my-extension": {
      "binary_path": "/usr/local/bin/my-tool"
    }
  }
}
```

## 8. Best Practices

- Keep the schema strict with `"additionalProperties": false` unless you intentionally want free-form data.
- Put stable defaults in `default_settings` so the extension works without any manual configuration.
- Prefer small, composable object settings over many unrelated top-level fields.
- If you want typed controls, avoid arrays, union types, and schema combinators.
- Treat deserialization failures as runtime input errors and handle them gracefully inside the extension.
- Document each property with JSON Schema `description` fields so the editor experience is clearer.

## 9. Summary

To add a settings UI to your extension:

1. upgrade to `zed_extension_api` `0.9.0+`
2. implement `settings_contribution`
3. provide a JSON schema and matching defaults
4. read settings through `zed::settings::ExtensionSettings`
5. store overrides under `extensions.<your-extension-id>`

Once that is in place, Zed will automatically surface your extension in the Settings UI.
