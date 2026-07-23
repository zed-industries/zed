# Citadel

*[日本語](./README.ja.md)*

**Citadel** (aka *Citadel-Duino*) is a fork of [Zed](https://github.com/zed-industries/zed), being reworked into a next-generation IDE dedicated to Arduino (AVR) embedded development.

> This is an independent, non-commercial fork. It is not affiliated with, endorsed by, or sponsored by Zed Industries, Inc. "Zed" is a trademark of Zed Industries, Inc.

### Philosophy

> "If it compiles, it runs safely. Never freeze the system, never lock the chip."

### Why Citadel exists

| Frustration | Countermeasure |
|---|---|
| IDEs are so heavy they kill productivity | Zed's GPU-accelerated rendering (GPUI) and a Rust-powered core cut through it |
| Code compiles fine, then freezes on real hardware for no clear reason | Core logic is written in Rust (no_std), eliminating memory corruption at compile time |
| A crash freezes the IDE and leaves the microcontroller's port locked up | Backend runs as a fully separate process; auto-inserted watchdog timer + rescue sequence provide fail-safety at both the process and hardware level |
| Small code changes are hard to track cleanly in Git | Zed's built-in inline Git integration and branch-linked builds keep progress traceable end to end |

Full architecture, scope, and roadmap: see the project RFC (in progress).

### Status

Early-stage fork — not yet functional as an Arduino IDE. Currently at the RFC / toolchain-verification stage.

### Developing

Build instructions are currently unchanged from upstream Zed:

- [Building on macOS](./docs/src/development/macos.md)
- [Building on Linux](./docs/src/development/linux.md)
- [Building on Windows](./docs/src/development/windows.md)

### Licensing

Citadel is a fork of Zed and inherits its licensing as-is: source code is licensed primarily under **GPL-3.0-or-later**, with **Apache-2.0** components where marked (see [LICENSE-GPL](./LICENSE-GPL) and [LICENSE-APACHE](./LICENSE-APACHE)).

Note for users: GPL applies to the IDE itself, not to code you write and compile with it. Sketches/firmware you build using Citadel are **not** subject to GPL just because the IDE is.

License information for third-party dependencies must be correctly provided for CI to pass. We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses — see upstream Zed's `script/licenses/zed-licenses.toml` for the mechanism, inherited unchanged for now.
