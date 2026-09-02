# `kkpdf-zed`: Native PDF Viewer for Zed Editor

A native, high-performance PDF viewing engine and GPUI workspace item engineered for the [Zed Editor](https://zed.dev).

---

## 0. Architectural Identity & Delivery Model

> **Important**: This is a **native workspace crate**, not an installable WASM extension.  
> As of current Zed versions, Zed's WebAssembly extension API (`zed_extension_api`) contains no custom UI or file preview surfaces. `kkpdf-zed` is designed as a native crate to be registered within the Zed workspace (`crates/kkpdf_viewer` or integrated upstream into `zed-industries/zed`).

`extension.toml` in this repository is a **documentation stub** preserving metadata identity for a potential future extension surface or upstream contribution.

---

## 1. Core Architectural Pillars

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                             KKPDF-ZED CORE PIPELINE                              │
└──────────────────────────────────────────────────────────────────────────────────┘
                                        │
                      [PDF Document on Disk / Memory Buffer]
                                        │
                                        ▼
                  [Thread-Safe Document Handle (`document.rs`)]
                                        │
                                        ▼
             [Async Background Worker Pool (`gpui::Task` / Threads)]
                                        │
                       (Zero UI-Thread Blocking Rasterization)
                                        │
                                        ▼
                  [Memory-Budgeted LRU Cache (`cache.rs`)]
                                        │
                       (Visible Viewport + 1-Page Margin)
                                        │
                                        ▼
                [Luminosity Tone-Mapping Engine (`ui/page.rs`)]
                                        │
                       (Inverts White/Black; Preserves Images)
                                        │
                                        ▼
                  [Native GPUI Canvas / Bitmap Painter (`view.rs`)]
```

1. **Zero UI-Thread Blocking (120 FPS Target)**: PDF parsing and rasterization never execute on GPUI's main render loop. Work is dispatched to background tasks that stream RGBA framebuffers into memory.
2. **Viewport Virtualization & LRU Caching**: Pages are rasterized on-demand for the visible viewport plus a 1-page prefetch margin. Stale or passed-by in-flight render requests are cancelled during fast scrolls to prevent thread pool starvation.
3. **Luminosity-Threshold Dark Mode**: Unlike simple blanket RGB inversion (which turns photos and charts into negative ghosts), `kkpdf-zed` implements selective tone mapping—remapping near-white canvas backgrounds to editor background tones and near-black text to theme light text, while preserving saturated color images.
4. **Scroll & Zoom Preserving Live Reload**: Watches the underlying PDF on disk (via file system notifications) and reloads seamlessly without resetting the user's scroll percentage or zoom level.

---

## 2. Scope Boundaries (v1)

- **Bitmap-Rendered Continuous Scroll**: Version 1 is engineered strictly as a high-fidelity, high-frame-rate read-only document viewer.
- **Excluded from v1**:
  - Text selection and copying.
  - In-document text search (`Ctrl+F`).
  - Interactive PDF form field editing.
- *Rationale*: Delivering an ultra-fast, stutter-free viewing experience with zero UI locks takes precedence. Text extraction and selection layer synchronization will be introduced in subsequent milestones.

---

## 3. Security Posture & Threat Model

- **In-Process Parsing**: PDF rasterization is performed via Google's native C++ [Pdfium](https://pdfium.googlesource.com/pdfium/) engine linked in-process with the editor.
- **Accepted Risk in v1**: Because Pdfium runs in-process without sandboxing (e.g., without separate process IPC or WebAssembly memory isolation), malformed or malicious PDFs could theoretically trigger memory vulnerabilities in the underlying C++ library.
- **Mitigation & Future Hardening**: Memory budgets are strictly capped, document handles are isolated behind thread boundaries, and a future sandboxed worker process model is planned for untrusted file browsing.

---

## 4. Repository Layout

```
kkpdf-zed/
├── Cargo.toml               # Native crate manifest
├── extension.toml           # Documentation stub
├── README.md                # Architecture & operational guide
└── src/
    ├── lib.rs               # Crate entrypoint & exports
    ├── document.rs          # Thread-safe PdfDocument handle
    ├── rasterizer.rs        # Async background rendering pipeline
    ├── cache.rs             # Memory-budgeted LRU cache with eviction
    ├── watcher.rs           # Live-reload file watcher & scroll state
    ├── view.rs              # GPUI Render & Workspace Item implementation
    └── ui/
        ├── page.rs          # Bitmap painting & luminosity recolor
        └── toolbar.rs       # Zoom, fit-width & page jump controls
```

---

## 5. Local Development & Testing

Run unit and integration tests across the caching and rasterization engines:

```bash
cargo test
```
