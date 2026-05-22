# libghostty-vt-sys

Raw FFI bindings for `libghostty-vt`, the Ghostty terminal emulation library.

Zed depends on the published `libghostty-vt` Rust crate, but patches its `libghostty-vt-sys` dependency to this local crate via `[patch.crates-io]`. The wrapper crate stays external; this sys crate is vendored only because Zed needs different build behavior than the published sys crate currently provides.

## Why this crate is vendored

- Zed must bundle the terminal backend. Users should not need to install Ghostty, place a Ghostty dynamic library on their system, or configure a runtime library path for Zed to start.
- Zed needs a static `libghostty-vt.a` build so release artifacts can link the backend into Zed directly.
- The Ghostty source revision must be pinned and reproducible for CI and release builds.
- The build has to link Ghostty's static native dependencies (`simdutf` and `highway`) explicitly.
- On macOS, Zed still targets older systems than Ghostty's default Zig build target. For fetched sources, this crate patches Ghostty's deployment target from macOS 13 to macOS 11 before building.
- The generated FFI bindings are checked in so regular builds and docs.rs do not need bindgen or a Zig toolchain just to type-check the Rust API surface.

## Build behavior

By default, `build.rs` fetches `ghostty-org/ghostty` at the pinned commit in `GHOSTTY_COMMIT`, builds it with:

```sh
zig build -Demit-lib-vt --prefix <out-dir>/ghostty-install
```

and links:

- `libghostty-vt.a`
- `libsimdutf.a`
- `libhighway.a`

Set `GHOSTTY_SOURCE_DIR` to point at a local Ghostty checkout while developing or testing a newer Ghostty revision. When this override is used, the source is treated as external and the macOS deployment-target patch is not applied; point it at an already-patched checkout if you need the macOS 11 target.

## Removing the patch

This vendored sys crate should go away once the published `libghostty-vt-sys` crate supports the requirements above directly: static bundled builds, explicit static dependency linkage, a Zed-compatible macOS deployment target, pinned-source or otherwise reproducible release builds, and checked-in bindings suitable for docs and CI.
