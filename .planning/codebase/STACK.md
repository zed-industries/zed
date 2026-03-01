# Technology Stack

**Analysis Date:** 2026-03-01

## Languages

**Primary:**
- Rust 1.93 - Main language for the entire codebase, editor core, and all native components

**Secondary:**
- TypeScript/JavaScript - Extensions written in WASM, component development
- WASM (WebAssembly) - Extensions platform (`wasm32-wasip2` target), web components (`wasm32-unknown-unknown` target)
- SQL - Database queries and migrations via SeaORM and SQLx

## Runtime

**Environment:**
- Rust toolchain 1.93 with minimal profile
- Tokio 1.x - Async runtime for concurrent operations
- GPUI executor - Custom async executor and foreground thread scheduler for UI operations

**Package Manager:**
- Cargo - Workspace-based monorepo with 227+ internal crates
- Lockfile: `Cargo.lock` present

**Build Targets:**
- macOS, Linux, Windows (native platforms)
- Linux musl (`x86_64-unknown-linux-musl`) for remote server
- WebAssembly (`wasm32-unknown-unknown`) for web platform
- WASM Component Model (`wasm32-wasip2`) for extensions

## Frameworks

**Core UI:**
- GPUI 0.2.2 - GPU-accelerated immediate mode UI framework (Apache-2.0 licensed, published to crates.io)
  - Platform-specific: `gpui_macos`, `gpui_windows`, `gpui_linux`, `gpui_web`
  - Graphics: `gpui_wgpu` (WebGPU), Metal on macOS, Direct3D on Windows
  - Location: `crates/gpui/`

**Editor Core:**
- Custom buffer and editor implementation
- Tree-Sitter 0.26 - Language parsing with grammar support for 30+ languages
- Language Server Protocol (LSP) via `lsp` crate - Language integration

**Async/Concurrency:**
- Tokio 1.x - Async runtime with multi-threaded and single-threaded features
- Smol 2.0 - Lightweight async executor
- GPUI Tokio integration (`gpui_tokio`) - Bridge between GPUI and Tokio runtimes
- Futures 0.3 - Stream and future utilities
- Async-channel 2.5.0, Postage 0.5 - Message passing

**Testing:**
- Custom GPUI test framework (`gpui/test-support`)
- Criterion 0.5 - Benchmark harness
- Pretty assertions 1.3.0 - Assertion library

**Build/Dev:**
- Prost 0.9 - Protocol Buffers code generation
- Proc-macros: `gpui_macros`, `ui_macros`, `settings_macros`, `sqlez_macros`
- Cargo metadata 0.19 - Build script support
- Custom script setup in `script/` directory

## Key Dependencies

**Critical:**
- `taffy` 0.9.0 - Flexbox layout engine (critical for UI rendering)
- `resvg` 0.45.0 - SVG rendering
- `tree-sitter` 0.26 - Syntax tree parsing
- `livekit` 0.7.32 - Real-time communication and collaboration
- `tokio` 1.x - Async runtime foundation
- `parking_lot` 0.12.1 - High-performance synchronization

**HTTP & Networking:**
- `zed-reqwest` 0.12.15 (forked, custom version) - HTTP client with custom patches
- `http_client` - Custom abstraction over reqwest
- `async-tungstenite` 0.31.0 - WebSocket client
- `tokio-tungstenite` 0.26 - WebSocket with Tokio
- `async-tungstenite` 0.31.0 with Tokio rustls

**Serialization:**
- `serde` 1.0.221 - Serialization framework
- `serde_json` 1.0.144 - JSON (with preserve_order and raw_value features)
- `serde_json_lenient` 0.2 - Lenient JSON parsing
- `bincode` 1.2.1 - Binary serialization

**Crypto & Security:**
- `rsa` 0.9.6 - RSA cryptography
- `sha2` 0.10 - SHA-2 hashing
- `rustls` 0.23.26 - TLS implementation
- `jsonwebtoken` 10.0 - JWT handling
- `zeroize` 1.8 - Secure zeroing of sensitive data

**Code Generation & Reflection:**
- `schemars` 1.0 - JSON Schema generation
- `quote` 1.0.9 - Rust code generation
- `syn` 2.0.101 - Rust syntax parsing
- `proc-macro2` 1.0.93 - Procedural macro support

**Database:**
- `sqlez` (custom) - SQLite abstraction with macros
- `libsqlite3-sys` 0.30.1 - SQLite bindings (bundled)
- `sea-orm` 1.1.10 - ORM with PostgreSQL support
- `sqlx` 0.8 - SQL toolkit with PostgreSQL driver

**Terminal & Shell:**
- `alacritty_terminal` - Terminal emulation (custom fork)
- `portable-pty` 0.9.0 - PTY management
- Custom shell command parsing

**Audio & Media:**
- `cpal` 0.17 - Cross-platform audio
- `rodio` (custom fork from zed-industries) - Audio playback and recording
- `image` 0.25.1 - Image processing

**WASM & Runtime:**
- `wasmtime` 33 - WASM runtime with component model support
- `wasmtime-wasi` 33 - WASM POSIX-like interface
- `wasm-bindgen` 0.2.113 - WASM JavaScript bindings
- `wasm-encoder` 0.221 - WASM encoding

**Version Control:**
- `git2` 0.20.1 - Git operations (vendored libgit2)
- `gitcommit` custom grammar - Git commit parsing

**UI Components:**
- `lyon` 1.0 - 2D vector graphics
- `palette` 0.7.5 - Color management
- `resvg` 0.45.0, `usvg` 0.45.0 - SVG support
- `image` 0.25.1 - Image formats

**Text Processing:**
- `unicode-segmentation` 1.10 - Text segmentation
- `encoding_rs` 0.8 - Character encoding
- `rope` - Custom efficient text rope implementation
- `text` - Custom text primitive

**Utilities:**
- `anyhow` 1.0.86 - Error handling
- `thiserror` 2.0.12 - Error derive macro
- `log` 0.4.16 - Logging (with serde features)
- `env_logger` 0.11 - Logger implementation
- `regex` 1.5 - Regular expressions
- `lazy_static`, `once_cell` - Lazy initialization
- `smallvec`, `arrayvec` - Stack-allocated collections
- `indexmap` 2.7.0 - Ordered maps and sets

**AWS Integration:**
- `aws-config` 1.8.10 - AWS configuration
- `aws-sdk-bedrockruntime` 1.112.0 - Bedrock model inference
- `aws-sdk-kinesis` 1.51.0 - Event streaming
- `aws-sdk-s3` 1.15.0 - Object storage
- `aws-smithy-runtime-api` 1.9.2 - AWS SDK runtime
- `aws-smithy-types` 1.3.4 - AWS types

## Configuration

**Environment:**
- Configuration via environment variables with `dotenvy` 0.15.0
- Settings serialized to JSON via `serde_json`
- Custom settings infrastructure in `crates/settings/`
- `.zed/settings.json` user configuration files

**Build:**
- `Cargo.toml` workspace configuration with 246+ members
- Edition 2024 with Rust 1.93 features
- Profile customization:
  - `dev`: Split debuginfo, incremental, 16 codegen-units
  - `release`: Limited debug, thin LTO, 1 codegen-unit
  - `release-fast`: No LTO, 16 codegen-units for development

**Platform-Specific:**
- Windows: Win32 API bindings via `windows` 0.61 crate
- macOS: Cocoa framework via `cocoa` 0.26.0, `objc2-foundation` 0.3.1
- Linux: X11 and Wayland support via `scap` for screen capture

## Platform Requirements

**Development:**
- Rust 1.93 toolchain with components: rustfmt, clippy, rust-analyzer, rust-src
- Build targets: Linux musl, WebAssembly (wasip2 and unknown-unknown)
- FFmpeg or equivalent for video processing (optional)
- System dependencies for platform SDKs (Xcode on macOS, Visual Studio on Windows)

**Production:**
- Deployment: Desktop applications for macOS, Windows, Linux
- Collab server: Remote deployment with PostgreSQL or SQLite backend
- Extensions: WASM runtime on client side
- Real-time collaboration: LiveKit infrastructure for voice/video

---

*Stack analysis: 2026-03-01*
