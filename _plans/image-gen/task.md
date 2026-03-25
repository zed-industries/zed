# Image Generator Panel Tasks

## Current Status: Not started

## Completed
(none yet)

## In Progress
- [ ] **CURRENT:** Step 1 - Extend `google_ai` crate
  - [ ] Add `ImageConfig` struct to `crates/google_ai/src/google_ai.rs`
  - [ ] Add `response_modalities` and `image_config` fields to `GenerationConfig`
  - [ ] Add non-streaming `generate_content()` function
  - [ ] Verify it compiles: `cargo build -p google_ai`

## Pending
- [ ] Step 2 - Create `image_gen_panel` crate
  - [ ] Create `crates/image_gen_panel/Cargo.toml`
  - [ ] Create `crates/image_gen_panel/src/image_gen_panel.rs`
  - [ ] Add to root `Cargo.toml` workspace members
  - [ ] Implement `ImageModel` enum (3 models)
  - [ ] Implement `AspectRatio` enum (5 options)
  - [ ] Implement `ImageGenPanel` struct
  - [ ] Implement `new()` constructor
  - [ ] Implement `load()` async function
  - [ ] Implement `Panel` trait
  - [ ] Implement `Focusable` trait
  - [ ] Implement `EventEmitter<PanelEvent>`
  - [ ] Implement `Render` trait (UI layout)
  - [ ] Implement `generate_image()` method (API call + decode)
  - [ ] Implement `decode_image()` helper (base64 → RenderImage)
  - [ ] Define `ToggleFocus` action
  - [ ] Verify it compiles: `cargo build -p image_gen_panel`

- [ ] Step 3 - Wire up registration
  - [ ] Add `image_gen_panel` to `crates/zed/Cargo.toml` dependencies
  - [ ] Add import in `crates/zed/src/zed.rs`
  - [ ] Add `ImageGenPanel::load()` call in `initialize_panels()`
  - [ ] Add to `futures::join!` macro
  - [ ] Verify full build: `cargo build -p zed`

- [ ] Step 4 - Test
  - [ ] Launch dev Zed with `env -u CLAUDECODE ZED_RELEASE_CHANNEL=nightly cargo run`
  - [ ] Verify Image icon in left dock
  - [ ] Test image generation with each model
  - [ ] Test different aspect ratios

## Blocked
(none)

## Notes
- The user wants this kept simple - no over-engineering
- All three models use the same generateContent endpoint
- Image response comes as base64 PNG in InlineDataPart
