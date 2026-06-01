# libghostty-vt-sys

Raw FFI bindings for `libghostty-vt`, the Ghostty terminal emulation library.

Zed depends on the published `libghostty-vt` Rust crate, but patches its `libghostty-vt-sys` dependency to this local crate via `[patch.crates-io]`. The wrapper crate stays external; this sys crate is vendored only because Zed needs different build behavior than the published sys crate currently provides.

## Why this crate is vendored

- Zed must bundle the terminal backend. Users should not need to install Ghostty, place a Ghostty dynamic library on their system, or configure a runtime library path for Zed to start.
- Zed needs a static `libghostty-vt.a` build so release artifacts can link the backend into Zed directly.
- The Ghostty source revision must be pinned so every build uses the same terminal backend code.
- The build has to link Ghostty's static native dependencies (`simdutf` and `highway`) explicitly.
- On macOS, Zed still targets older systems than Ghostty's default Zig build target, so fetched Ghostty source is patched to use Zed's macOS 10.15.7 deployment target. Externally supplied source must already include that patch.
- The generated FFI bindings are checked in so regular builds and docs.rs do not need bindgen or a Zig toolchain just to type-check the Rust API surface.

## Build behavior

`build.rs` looks for Ghostty source in this order:

1. `GHOSTTY_SOURCE_DIR`
2. `crates/libghostty-vt-sys/vendor/ghostty`
3. a fresh clone of the pinned Ghostty commit into Cargo's `OUT_DIR`

The build invokes:

```sh
zig build -Demit-lib-vt --prefix <out-dir>/ghostty-install
```

and links:

- `libghostty-vt.a`
- `libsimdutf.a`
- `libhighway.a`

Set `GHOSTTY_ZIG_SYSTEM_DIR` to a prepared Zig package directory, or place that package directory at `crates/libghostty-vt-sys/vendor/zig`, to make the build invoke:

```sh
zig build -Demit-lib-vt --prefix <out-dir>/ghostty-install --system <zig-package-dir>
```

Without a prepared Zig package directory, Zig may fetch packages itself. That default keeps local development easy; CI and release builders should provide `GHOSTTY_SOURCE_DIR` or `vendor/ghostty` plus `GHOSTTY_ZIG_SYSTEM_DIR` or `vendor/zig` when they need hermetic/offline builds.

Set `GHOSTTY_SOURCE_DIR` to point at a local Ghostty checkout while developing or testing a newer Ghostty revision. This source is treated as external and is not patched by the build script; point it at an already-patched checkout if you need the macOS 10.15.7 target.

## Removing the patch

This vendored sys crate should go away once the published `libghostty-vt-sys` crate supports the requirements above directly: static bundled builds, explicit static dependency linkage, a Zed-compatible macOS deployment target, pinned-source or otherwise reproducible builds, and checked-in bindings suitable for docs and CI.
