# Plan: Remove GPUI Dependency from `settings_content`

## Goal

Remove the `gpui` dependency from the `settings_content` crate to make it a pure data representation crate with no UI framework dependencies. This will improve compile times and reduce coupling.

## Current GPUI Usage Analysis

### Types Currently Used from GPUI

| Type | Files | Usage |
|------|-------|-------|
| `SharedString` | agent.rs, language.rs, terminal.rs, settings_content.rs, merge_from.rs | String type for UI text |
| `Modifiers` | language.rs, merge_from.rs | Keyboard modifier keys struct |
| `FontFeatures` | theme.rs, terminal.rs, merge_from.rs | OpenType font features map |
| `FontWeight` | theme.rs, terminal.rs, merge_from.rs | Font weight (100-900) |
| `FontStyle` | theme.rs | Normal/Italic/Oblique enum |
| `FontFallbacks` | theme.rs | List of fallback font names |
| `Pixels` | theme.rs | Pixel measurement unit |
| `AbsoluteLength` | terminal.rs | Length measurement type |
| `WindowBackgroundAppearance` | theme.rs | Window transparency setting |

## Migration Strategy

### Phase 1: Create Content Types for Complex GPUI Types

For types that have behavior or methods we need to mirror, create new "Content" versions:

#### 1.1 `ModifiersContent` (new type in `settings_content`)
```rust
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ModifiersContent {
    #[serde(default)]
    pub control: bool,
    #[serde(default)]
    pub alt: bool,
    #[serde(default)]
    pub shift: bool,
    #[serde(default)]
    pub platform: bool,
    #[serde(default)]
    pub function: bool,
}

impl ModifiersContent {
    pub fn modified(&self) -> bool {
        self.control || self.alt || self.shift || self.platform || self.function
    }
}
```

Location: Add to `language.rs` or create new `input.rs` module

#### 1.2 `FontFeaturesContent` (new type)
```rust
/// OpenType font features as a map of feature tag to value.
/// This mirrors gpui::FontFeatures but without the Arc wrapper.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(transparent)]
pub struct FontFeaturesContent(pub Vec<(String, u32)>);
```

Location: Add to `theme.rs`

#### 1.3 `FontFallbacksContent` (new type or use existing)
```rust
/// Already can use Vec<FontFamilyName> directly
pub type FontFallbacksContent = Vec<FontFamilyName>;
```

### Phase 2: Replace Direct GPUI Type Usage

#### 2.1 Replace `SharedString` with `Arc<str>`

Files to modify:
- [ ] `agent.rs`: Change `SharedString` → `Arc<str>`
- [ ] `language.rs`: Remove `SharedString` import (check if actually used)
- [ ] `terminal.rs`: `Shell::WithArguments { title_override: Option<SharedString> }` → `Option<Arc<str>>`
- [ ] `settings_content.rs`: Remove import

#### 2.2 Replace `Modifiers` with `ModifiersContent`

Files to modify:
- [ ] `language.rs`: `toggle_on_modifiers_press: Option<Modifiers>` → `Option<ModifiersContent>`

#### 2.3 Replace `FontFeatures` with `FontFeaturesContent`

Files to modify:
- [ ] `theme.rs`: `ui_font_features: Option<FontFeatures>` → `Option<FontFeaturesContent>`
- [ ] `theme.rs`: `buffer_font_features: Option<FontFeatures>` → `Option<FontFeaturesContent>`
- [ ] `terminal.rs`: `font_features: Option<FontFeatures>` → `Option<FontFeaturesContent>`
- [ ] `theme.rs`: `default_font_features()` → return `FontFeaturesContent`

#### 2.4 Replace `FontWeight` with `FontWeightContent`

Already have `FontWeightContent` enum. Need to:
- [ ] `theme.rs`: Change `ui_font_weight: Option<FontWeight>` → `Option<FontWeightContent>`
- [ ] `theme.rs`: Change `buffer_font_weight: Option<FontWeight>` → `Option<FontWeightContent>`
- [ ] `terminal.rs`: Change `font_weight: Option<FontWeight>` → `Option<FontWeightContent>`
- [ ] `theme.rs`: Update `default_buffer_font_weight()` to return `FontWeightContent`

#### 2.5 Replace `FontFallbacks` usage

- [ ] `theme.rs`: `default_font_fallbacks()` → return `Vec<FontFamilyName>` or remove if unused

#### 2.6 Replace `Pixels` conversions

- [ ] `theme.rs`: Remove `impl From<FontSize> for Pixels` (move to consumer)
- [ ] `theme.rs`: Remove `impl From<Pixels> for FontSize` (move to consumer)
- [ ] Keep `FontSize` as `f32` wrapper

#### 2.7 Handle `AbsoluteLength` in terminal.rs

- [ ] `terminal.rs`: Change `TerminalLineHeight::value() -> AbsoluteLength` to return `f32`
- [ ] Remove `px()` usage

### Phase 3: Move `From` Implementations to Consumer Crates

These `impl From` blocks convert Content types to gpui types. They need to move to crates that have both dependencies.

#### 3.1 Move from `theme.rs`:
- [ ] `impl Into<gpui::WindowBackgroundAppearance> for WindowBackgroundContent` → move to `theme` crate
- [ ] `impl From<FontStyleContent> for FontStyle` → move to `theme` crate
- [ ] `impl From<FontWeightContent> for FontWeight` → move to `theme` crate
- [ ] `impl From<FontSize> for Pixels` → move to `theme` crate
- [ ] `impl From<Pixels> for FontSize` → move to `theme` crate or remove

#### 3.2 Create new conversions in consumer crates:
- [ ] `impl From<ModifiersContent> for gpui::Modifiers` → add to `editor` crate
- [ ] `impl From<FontFeaturesContent> for gpui::FontFeatures` → add to `theme` crate
- [ ] `impl From<FontFamilyName> for gpui::SharedString` → already exists, keep

### Phase 4: Update `merge_from.rs`

Remove gpui types from `merge_from_overwrites!` macro:

```rust
merge_from_overwrites!(
    // ... existing primitive types ...
    // REMOVE these:
    // gpui::SharedString,
    // gpui::Modifiers,
    // gpui::FontFeatures,
    // gpui::FontWeight

    // ADD these instead (if not already covered):
    // ModifiersContent - derives MergeFrom
    // FontFeaturesContent - derives MergeFrom
    // FontWeightContent - derives MergeFrom
);
```

### Phase 5: Update Cargo.toml

- [ ] Remove `gpui.workspace = true` from `crates/settings_content/Cargo.toml`

### Phase 6: Update Consumer Crates

Crates that use settings_content types and need gpui conversions:

#### 6.1 `theme` crate
Add conversion implementations:
```rust
impl From<settings_content::FontWeightContent> for gpui::FontWeight { ... }
impl From<settings_content::FontStyleContent> for gpui::FontStyle { ... }
impl From<settings_content::FontFeaturesContent> for gpui::FontFeatures { ... }
impl From<settings_content::WindowBackgroundContent> for gpui::WindowBackgroundAppearance { ... }
```

#### 6.2 `editor` crate
Add conversion:
```rust
impl From<settings_content::ModifiersContent> for gpui::Modifiers { ... }
```

Or update usage site directly:
```rust
// Before:
if inlay_modifiers == &event.modifiers { ... }

// After:
let content_modifiers: ModifiersContent = event.modifiers.into();
if inlay_modifiers == &content_modifiers { ... }
// OR compare field-by-field
```

#### 6.3 `terminal_view` crate
Update to convert `FontFeaturesContent` → `FontFeatures`:
```rust
let font_features: gpui::FontFeatures = terminal_settings
    .font_features
    .as_ref()
    .map(|f| f.into())
    .unwrap_or_else(FontFeatures::disable_ligatures);
```

## Migration Checklist

### New Types to Create
- [ ] `ModifiersContent` struct
- [ ] `FontFeaturesContent` struct (or decide to use `HashMap<String, u32>`)

### Files to Modify in `settings_content`
- [ ] `language.rs` - Add `ModifiersContent`, update `InlayHintSettingsContent`
- [ ] `theme.rs` - Add `FontFeaturesContent`, update field types, remove `From` impls
- [ ] `terminal.rs` - Update `FontFeatures`/`FontWeight` usage, fix `value()` method
- [ ] `agent.rs` - Replace `SharedString` with `Arc<str>`
- [ ] `settings_content.rs` - Remove gpui import
- [ ] `merge_from.rs` - Remove gpui types from macro
- [ ] `Cargo.toml` - Remove gpui dependency

### Files to Modify in Consumer Crates
- [ ] `crates/theme/src/*.rs` - Add `From` implementations
- [ ] `crates/editor/src/element.rs` - Update `Modifiers` comparison
- [ ] `crates/terminal_view/src/terminal_element.rs` - Update font features handling

### Tests to Update
- [ ] `theme.rs` tests that reference `FontWeight::NORMAL` etc.
- [ ] Any tests constructing `InlayHintSettingsContent` with `Modifiers`

## Notes

- `FontFamilyName` already uses `Arc<str>` internally and has `From<SharedString>` / `Into<SharedString>` - this pattern works well
- The `FontWeightContent` enum already exists and maps to `FontWeight` - just need to use it as the field type
- `FontStyleContent` already exists similarly
- Consider whether `FontFeaturesContent` should be `Vec<(String, u32)>` or `HashMap<String, u32>` - Vec preserves order which may matter for some font engines
