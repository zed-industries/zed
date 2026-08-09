# RFC 0001: Rust/C hybrid architecture — open technical challenges

Status: **draft / unresolved**. This RFC tracks whether the architecture described in the [README](../../README.md#architecture-the-rustc-boundary) is actually buildable, not just desirable. It does not re-derive the architecture itself — see the README for the boundary rules and the build-flow diagram.

## Verdict so far

No part of the design is a research risk — every piece has prior art (mainly [avr-hal](https://github.com/Rahix/avr-hal)) proving it works in isolation. What's unproven is stitching them together the way Citadel needs: an *existing* Arduino C++ sketch calling into Rust logic, with the IDE enforcing the boundary automatically. That's an integration/engineering risk, not a feasibility risk. Concretely: **if the five items below are each resolved, the architecture is implementable as designed.**

## Open items

### 1. Rust's AVR target is nightly-only

`rustc`'s AVR backend (`avr-none` / historically `avr-unknown-gnu-atmega328`) isn't on stable — it needs `cargo +nightly build -Z build-std=core --target ...`. Citadel has to pin and vendor a specific nightly rather than relying on whatever's installed, since the LLVM AVR backend has had regressions across nightlies historically.

- **Needs:** a toolchain-pinning strategy (bundled nightly, or a checked `rust-toolchain.toml` the IDE enforces) and a smoke-test suite run against nightly updates before Citadel bumps its pinned version.
- **Resolved (proof of mechanism):** [`prototypes/0001-hello-blink/rust-toolchain.toml`](../../prototypes/0001-hello-blink/rust-toolchain.toml) pins `nightly-2026-08-06`; `build.sh` builds against exactly that toolchain with `-Z build-std=core --target avr-none -C target-cpu=atmega328p`. What's still open: the smoke-test suite that runs against nightly *updates* before Citadel bumps its pin doesn't exist yet — this only proves one nightly works, not the update process.

### 2. `.hex` generation is `avr-objcopy`, not `avr-gcc`, directly

The build-flow diagram says "avr-gcc merges both into one .hex," but avr-gcc only produces the linked ELF; `avr-objcopy -O ihex` converts that to `.hex`. Arduino's own build already hides this step behind one command — Citadel needs to replicate it, not invent it.

- **Needs:** no design work, just an implementation task (link ELF, then objcopy).
- **Resolved:** implemented in [`prototypes/0001-hello-blink/build.sh`](../../prototypes/0001-hello-blink/build.sh) (link with `avr-g++`, then `avr-objcopy -O ihex -R .eeprom`).

### 3. Binary size with two toolchains and no cross-LTO

GCC (avr-g++) and LLVM (rustc) can't LTO across each other, so each brings its own copies of runtime helpers (e.g. software multiply/divide, memcpy). On flash-constrained chips (ATmega328p / Arduino Uno, 32 KB) this could matter; on larger chips (ATmega2560 / Mega, 256 KB) it's likely noise.

- **Needs:** a real measurement — build a representative sketch + Rust logic crate, compare `.hex` size against a hypothetical single-toolchain build, before deciding whether size is actually a problem worth solving.
- **Data point, not resolved:** `prototypes/0001-hello-blink` (`avr-size`) measures `text=274 bytes, bss=5 bytes` — about 0.8% of the ATmega328P's 32 KB flash. This is a minimal blink-only program, not a representative sketch, so it doesn't yet answer whether duplicated runtime helpers (multiply/divide/memcpy) become a real problem once a sketch is large enough to pull them in from both toolchains. Still open until measured against something more representative.

### 4. Static analysis that rejects logic in user C/C++

The IDE must parse the user's sketch (not vendored libraries — those are allowed to contain arbitrary C++) and reject `if`/`for`/`while`/ternaries/etc. Zed already ships a tree-sitter C/C++ grammar, so the parsing infrastructure exists; what's undesigned:

- Scope: only the user's `.ino`/top-level sketch files, never `libraries/` or vendored headers.
- Macros: a `#define` that expands to a branch (or a user-defined macro at all) can hide logic from a pre-expansion AST walk. Simplest starting rule: disallow user-defined function-like macros in sketch files entirely, only trust macros from the pinned Arduino core.
- Edge case: is a trivial `for` in `setup()` (e.g. initializing N pins) actually banned, or only in `loop()`? Needs an explicit decision, not just "no for anywhere."

- **Needs:** a design decision on scope + macro policy, then a tree-sitter-based lint pass (prior art: Zed's existing diagnostics/language-server plumbing).

### 5. ABI/link interop between avr-g++ objects and rustc-LLVM objects

Both target the same AVR calling convention, so this is expected to work, but needs an actual prototype to confirm: who owns `main()` (the Arduino C++ runtime should, with Rust compiled as a `#![no_std]` `staticlib` exposing only `extern "C"` functions), what the linker script/memory layout looks like, and how C++ static initializers and Rust's own init (`#[panic_handler]`, no `alloc` unless proven necessary) coexist without both trying to run startup code.

- **Needs:** a minimal end-to-end prototype — one sketch, one Rust crate, one linked `.hex`, flashed and verified on real hardware — before this item can be marked resolved.
- **Resolved:** [`prototypes/0001-hello-blink`](../../prototypes/0001-hello-blink) builds one `.cpp` I/O layer + one `#![no_std]` Rust logic crate, links them with `avr-g++`, and produces a `.hex`. `avr-nm` confirms `citadel_setup`/`citadel_loop`/`citadel_tick` all resolve across the toolchain boundary with zero undefined symbols — this answers the ABI-compatibility half of the question. The ownership/init-coexistence design decided here: a small `cpp/runtime.cpp` owns `main()` and the only loop construct in the whole prototype (analogous to the vendored Arduino core's `wiring.c`), calling into `citadel_setup()`/`citadel_loop()` in `cpp/io.cpp`, which are themselves branch-free straight-line I/O hand-off (matching the boundary rule in §4/README). Flashed to an ELEGOO UNO R3 (Arduino Uno-compatible board) via `avrdude`; write and verify succeeded and the onboard LED blink was visually confirmed — the end-to-end hardware loop is proven.

## Definition of done for this RFC

Each item above moves from "open" to "resolved" only after a working prototype or measurement, not a design doc. When all five are resolved, this RFC should be closed and folded into the main architecture description in the README.
