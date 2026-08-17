# Unreleased

# 1.1.0 (2025-01-24)

- Update `objc2` dep to `0.6`.
- Bump MSRV to 1.71.

# 1.0.0 (2024-09-09)

- Bump Rust Edition from 2018 to 2021.
- Make `Layer`'s implementation details private; it is now a struct with `as_ptr`, `into_raw` and `is_existing` accessor methods.
- Add support for tvOS, watchOS and visionOS.
- Use `objc2` internally.
- Move `Layer` constructors to the type itself.
  - `appkit::metal_layer_from_ns_view` is now `Layer::from_ns_view`.
  - `uikit::metal_layer_from_ui_view` is now `Layer::from_ui_view`.

  `raw-window-handle` types are also no longer exposed directly in the API.
  This allows us to decouple the library from `raw-window-handle`'s versioning.
- Added `Layer::from_layer` to construct a `Layer` from a `CALayer` directly.
- Fixed layers not automatically resizing to match the super layer they were created from.

# 0.4.0 (2023-10-31)
- Update `raw-window-handle` dep to `0.6.0`.
- Remove `metal_layer_from_ns_window` and `metal_layer_from_ui_window`.

# 0.3.2 (2023-10-31)
- Bump version of `cocoa` and `core-graphics` dependencies.

# 0.3.1 (2022-11-25)
- Only build docs.rs for `darwin` and `ios`.

# 0.3.0 (2022-07-28)
- Update `raw-window-handle` dep to `0.5.0`.

# 0.2.0 (2021-12-02)
- Update `raw-window-handle` dep to `0.4.2`. Rename `macos` -> `appkit` and `ios` -> `uikit` following `raw-window-handle`.

# 0.1.2
- Update `cocoa` and `core-graphics` dependencies.

# 0.1.1
- iOS: support layer from ui window.

# 0.1.0
- Initial release! 🎉
