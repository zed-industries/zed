---
paths:
  - "crates/zed-ios/**/*.rs"
  - "crates/gpui/src/platform/ios/**/*.rs"
---

# iOS Platform Rust Rules

## Hard prohibitions
- NEVER use `std::process::Command`, `posix_spawn`, `fork`, or `exec` — iOS kills the app
- NEVER use `std::fs` paths outside the app sandbox container
- NEVER assume `~/.ssh/` exists — use iOS Keychain via Security.framework
- NEVER use `NSApplication`, `AppKit`, or any macOS-only framework
- NEVER use `CVDisplayLink` — iOS uses `CADisplayLink`
- NEVER hold blocking `Mutex`/`RwLock` on the main thread

## cfg gating
- Use `#[cfg(target_os = "ios")]` — never `#[cfg(unix)]` for iOS-specific code
- Share Apple-common code with `#[cfg(target_vendor = "apple")]` where appropriate
- The iOS platform module in GPUI follows the same shape as `platform/mac/`

## Platform trait implementation
- `IosPlatform` implements `Platform` (~40+ methods)
- `IosWindow` implements `PlatformWindow` (~37+ methods)
- `IosDisplay` implements `PlatformDisplay` (4 methods)
- Reference `platform/mac/` for Metal and CoreText code (high reuse)
- Reference `platform/wasm/` for minimum viable Platform implementation patterns

## Metal renderer
- Shaders in `platform/mac/shaders.metal` are 100% portable to iOS (verified)
- Replace `CVDisplayLink` with `CADisplayLink` + `preferredFrameRateRange`
- Replace `NSView` layer backing with `UIView` + `layerClass -> CAMetalLayer`
- Use `runtime_shaders` feature flag during development to bypass build-time compilation
- Release Metal texture caches when the app enters background (Jetsam has no swap)

## SSH transport
- Zed currently shells out to system `ssh` — this MUST be replaced with `russh` (in-process)
- The `RemoteConnection` trait in `crates/remote/` provides the abstraction seam
- Keys loaded from iOS Keychain via `SecItemCopyMatching` → passed as in-memory bytes

## Memory management
- Monitor `os_proc_available_memory()` — iOS has no swap, Jetsam kills via SIGKILL
- Register for `UIApplicationDidReceiveMemoryWarningNotification`
- Aggressively evict texture atlases and non-visible buffer caches when backgrounded
