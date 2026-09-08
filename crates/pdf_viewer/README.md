# Zed PDF Viewer (`pdf_viewer`)

A production-grade, high-performance, GPU-accelerated PDF viewer built natively in Rust for the [Zed](https://zed.dev) editor.

`pdf_viewer` integrates directly into Zed's workspace tab system, delivering Chrome-parity continuous scrolling, 512x512 tile-based off-screen rendering, LRU memory caching, and multi-threaded worker pools with generational stale-frame cancellation.

```
 ┌────────────────────────────────────────────────────────────────────────┐
 │ [icon] manual.pdf                                                [x]  │
 ├────────────────────────────────────────────────────────────────────────┤
 │ [ |< ] [ < ] Page 4 of 28 [ > ] [ >| ] │ [ - ] [ 100% ] [ + ] [ Fit ]  │
 ├────────────────────────────────────────────────────────────────────────┤
 │                                                                        │
 │                      ┌───────────────────────┐                         │
 │                      │ Page 3 (Partial)      │                         │
 │                      └───────────────────────┘                         │
 │                      ┌───────────────────────┐                         │
 │                      │ Page 4 [Active]       │                         │
 │                      │  Tile (0,0)  Tile(1,0)│                         │
 │                      │  Tile (0,1)  Tile(1,1)│                         │
 │                      └───────────────────────┘                         │
 │                      ┌───────────────────────┐                         │
 │                      │ Page 5 (Partial)      │                         │
 │                      └───────────────────────┘                         │
 │                                                                        │
 └────────────────────────────────────────────────────────────────────────┘
```

---

## Chrome-Parity Architecture

- **📜 Continuous Document Stack:** Virtualized continuous vertical layout. Rather than single-page flips, documents stack smoothly with viewport culling (only pages and tiles within the visible viewport are processed).
- **🧩 512x512 Tile-Based Rendering:** Pages are subdivided into standard 512x512 device pixel tiles. Zoom factors are bucketed into 10% steps to eliminate visual stuttering during fluid trackpad zooms.
- **⚡ Multi-Threaded Render Pool (`RenderPool`):** Dedicated worker threads rasterize tiles asynchronously off the main UI thread. Render requests carry an atomic generation counter—when you zoom or pan rapidly, obsolete in-flight requests are immediately dropped.
- **🧠 Memory-Bounded LRU Cache (`TileCache`):** Cached tiles are bounded by a byte budget (default 256MB). Tiles distant from the current viewport are pruned to keep memory footprint minimal.
- **🎯 Cursor-Anchored Zoom & Gestures:**
  - `Ctrl` (or `Cmd` on macOS) + Mouse Wheel zooms smoothly anchored directly beneath your cursor.
  - Trackpad pinch-to-zoom (`PinchEvent`).
  - Middle-click and click-and-drag smooth canvas panning with adaptive cursor styles.
- **⚡ Pluggable Engine Architecture:**
  - **Poppler / CLI Backend:** High-speed Linux/Unix backend using system `pdftoppm` with sub-rectangle clipping (`-x -y -W -H -scale-to-x -scale-to-y`) rendering directly to stdout PNG streams (no temporary disk files).
  - **Mock Backend:** Deterministic SVG-based engine for offline testing, headless CI, and instant automated tests without external C libraries.
- **📑 Full Zed Tab & Workspace Integration:** Implements `workspace::Item`, `workspace::ProjectItem`, and `workspace::SerializableItem`. Supports split panes, tab dragging, git status indicators, and breadcrumbs.
- **💾 Session Persistence:** Automatically saves and restores your exact continuous scroll coordinates `(scroll_x, scroll_y)` and zoom level across editor restarts via SQLite (`sqlez`).

---

## Keybindings & Gestures

| Action | macOS Shortcut | Linux / Windows |
| :--- | :--- | :--- |
| **Cursor-Anchored Zoom** | `Cmd + Scroll Wheel` / `Pinch` | `Ctrl + Scroll Wheel` / `Pinch` |
| **Continuous Pan** | `Click + Drag` / `Scroll Wheel` | `Click + Drag` / `Scroll Wheel` |
| **Next Page** | `PageDown` / `Cmd + Down` | `PageDown` / `Ctrl + Down` |
| **Previous Page** | `PageUp` / `Cmd + Up` | `PageUp` / `Ctrl + Up` |
| **First Page** | `Home` | `Home` |
| **Last Page** | `End` | `End` |
| **Zoom In** | `Cmd + =` | `Ctrl + =` |
| **Zoom Out** | `Cmd + -` | `Ctrl + -` |
| **Reset Zoom (100%)** | `Cmd + 0` | `Ctrl + 0` |
| **Fit to Width** | `Cmd + Shift + W` | `Ctrl + Shift + W` |

---

## Directory Structure

```
crates/pdf_viewer/
├── Cargo.toml
├── README.md
├── LICENSE-GPL
└── src/
    ├── pdf_engine.rs     # Abstract engine traits, Mock engine, & Poppler pdftoppm engine
    ├── pdf_item.rs       # ProjectItem implementation & file loaders
    ├── pdf_viewer.rs     # Continuous document scroll, gesture handlers, & UI
    ├── persistence.rs    # SQLite database schema for persisting scroll and zoom state
    ├── render_pool.rs    # Multi-threaded background worker pool with generational cancellation
    ├── tile_cache.rs     # 512x512 tile LRU cache with memory budgeting
    └── tests.rs          # Comprehensive automated unit tests
```

---

## Verification & Testing

Run all unit tests:
```bash
cargo test -p pdf_viewer
```

Check the entire Zed workspace:
```bash
cargo check -p zed
```

Launch Zed with the native PDF viewer:
```bash
cargo run -p zed
```
