# Design: Board auto-detection and one-button build & flash

Status: **draft**, pending user review before an implementation plan is written.

## Motivation

Today, building and flashing an AVR sketch requires a hand-run shell script (as in `prototypes/0001-hello-blink` and `prototypes/0002-arduino-core`) and manual board/port selection. This design gives Citadel's IDE an Arduino-IDE-like experience: on connect, the board is identified automatically, and a single status-bar action builds and flashes the current project.

This directly implements the "Board and toolchain detection" section of [RFC 0002](../../rfcs/0002-product-scope-and-dx.md#board-and-toolchain-detection), which explicitly gates this work behind two open decisions. Both are resolved by this design (see "RFC 0002 open items resolved" below).

## Scope

In scope:
- Serial port enumeration and polling.
- Chip signature read + USB VID:PID-based board disambiguation, with persistent memory.
- A status bar item showing the detected board/port and a "Build and Upload" action.
- Build pipeline: compile the vendored `ArduinoCore-avr` core (cached), compile the project's `cpp/` sketch, build the project's `rust/` crate, link, `avr-objcopy` to `.hex`.
- Flash via `avrdude`.
- Per-device (VID:PID-keyed) one-time warning when the detected chip is a recognized-but-not-smoke-tested AVR part.

Out of scope (recorded so it stays a decision, not an omission):
- RFC 0002's full process-isolation failsafe layer (a persistent backend process that survives an IDE crash and keeps owning the serial port). This design uses GPUI `background_spawn` for build/flash subprocesses, which satisfies "UI liveness" (a hung `avrdude` cannot block a keystroke) but not full process isolation. Marked in code with a `ponytail:` comment noting the upgrade path.
- RFC 0004's build-identity marker (embedding a commit/tree hash in the flashed image). Independent RFC, independent work.
- A "community verified chip" registry (crowdsourced reporting of which unverified chips actually work). Raised during design review; requires a Citadel-owned backend that does not exist today (Citadel cannot piggyback on the `telemetry`/`client` crates' existing pipeline, which is Zed Industries' own collector). Deferred as a future idea, not part of this design.
- Non-AVR targets, JTAG/SWD debugging — already out of scope per RFC 0002.

## RFC 0002 open items resolved

1. **Board disambiguation (chip signature alone can't tell Uno from Nano from Pro Mini):** resolved as USB VID:PID-based memory. On first connection from an unrecognized VID:PID, the status bar shows "Unknown board — click to identify"; clicking opens a picker (Uno / Nano / Pro Mini / other). The choice is persisted keyed by VID:PID, so subsequent connections of the same physical adapter resolve silently. Wrong guesses are correctable the same way (re-click to re-pick).
2. **Detected target may not be smoke-tested against the pinned nightly:** resolved as "accept broadly, warn once per device." Any AVR chip signature avr-gcc recognizes (`-mmcu=<chip>`) is accepted and built; if the signature isn't ATmega328P (the only chip actually verified against the pinned nightly by `prototypes/0001-hello-blink` and `prototypes/0002-arduino-core`), a warning toast is shown. The warning is remembered per VID:PID (same storage as board identity) so it fires once per physical device, not once per build — this matters in practice: the design was revised specifically because ATmega328PB (signature `0x1E9516`, distinct from ATmega328P's `0x1E950F`) is a real device in daily use (a school lab board) and a warning on every single build would be noise, not signal.

## Architecture

New crate `crates/citadel_build/`, following the `citadel_new_project` naming and structure convention.

```
crates/citadel_build/
├── Cargo.toml
└── src/
    ├── citadel_build.rs   # crate root, registers the status bar item + action
    ├── board_detect.rs    # serial port polling, signature read, VID:PID resolution
    ├── board_registry.rs  # known chip signatures -> {name, -mmcu flag, verified: bool}
    └── build_pipeline.rs  # core cache, compile, link, objcopy, avrdude invocation
```

Dependencies added: `serialport` (cross-platform serial enumeration), `db` (existing sqlite-backed key-value store, for VID:PID -> board / warning-acknowledged persistence — same crate that already backs other small persisted IDE state).

### Data flow

```
[serial port poll, ~1-2s interval]
        │
        ▼
[new port appeared] ──▶ [read chip signature via a bootloader probe,
                          same STK500 handshake avrdude itself uses]
        │                          │
        │                          ▼
        │                 [lookup in board_registry by signature]
        │                          │
        │              ┌───────────┴────────────┐
        │              ▼                         ▼
        │      known, verified              known, unverified
        │      (e.g. ATmega328P)            (e.g. ATmega328PB)
        │              │                         │
        │              │                 [warned this VID:PID before?]
        │              │                    │            │
        │              │                   no           yes
        │              │                    │            │
        │              │              [show warning   [skip warning]
        │              │               toast, record]      │
        │              └───────────┬────────┘─────────────-┘
        │                          ▼
        │              [VID:PID known in db?]
        │                 │              │
        │                yes             no
        │                 │              │
        │        [resolve board name] [show "unknown board,
        │                 │             click to identify"]
        ▼                 ▼
   [status bar updates: "Board: <name> (<port>)"]
```

### Build pipeline (triggered by the status bar action)

1. Resolve the current workspace root; require `rust/` and `cpp/` directories (the `citadel_new_project` scaffold shape). If absent, the action is disabled.
2. Ensure the cached `core.a` exists for the vendored `ArduinoCore-avr` (build once, reuse across builds/projects; see "Core vendoring" below).
3. Compile `cpp/*.cpp` for the project.
4. `cargo build --release -Z build-std=core --target avr-none` for `rust/`, using the project's pinned `rust-toolchain.toml` (as scaffolded by `citadel_new_project`).
5. Link (`avr-g++`, `-Wl,--gc-sections`) against `core.a`, the sketch object, and the Rust staticlib.
6. `avr-objcopy -O ihex -R .eeprom` to produce `.hex`.
7. `avrdude` flash using the resolved board's baud/programmer settings and the resolved port.

All of steps 2-7 run inside a single `cx.background_spawn`, matching the existing pattern in `citadel_new_project`'s `write_scaffold`/`git_init_and_commit`. Failures at any step show a `StatusToast` (reusing `show_error_toast_in_workspace`-style helper) with the failing step named, so a build failure and a flash failure are visibly distinct.

### Core vendoring

`ArduinoCore-avr` (pinned to tag `1.8.8`, matching `prototypes/0002-arduino-core`) is vendored once as a git submodule under `assets/arduino-core/ArduinoCore-avr/`, and bundled into the Citadel binary via the existing `crates/assets` RustEmbed pipeline (adding an `#[include = "arduino-core/**/*"]` line) — no new embedding mechanism. At first use, the embedded core sources are extracted to a local cache directory and `core.a` is compiled once; subsequent builds (of the same or different projects) reuse the cached archive, only recompiling the user's sketch and Rust crate. This mirrors `prototypes/0002-arduino-core/build.sh`'s already-verified archive-then-link approach, but pays the core-compile cost once instead of per build.

### Board registry

A static table mapping AVR device signatures to `{ display_name, mmcu_flag, verified }`. Initial entries: ATmega328P (`verified: true`, matches the pinned-nightly smoke tests), plus other common AVR parts (e.g. ATmega328PB, ATmega2560, ATmega32U4) with `verified: false`. `verified: false` entries still build and flash — the flag only controls the once-per-device warning toast, not whether the action is available.

## Error handling

- Missing toolchain (`avr-gcc`/`avrdude` not on `PATH`): detected up front, shown as a toast, action stays enabled (so it's retried after the user fixes their `PATH`) rather than being permanently disabled.
- Port claimed by another process, signature read failure, compile/link/flash failure: each surfaces its own toast, reusing the pattern already established in `crates/citadel_new_project/src/new_project.rs`.
- Ambiguous/ unknown board: not an error — the status bar shows the identification picker instead of failing silently.

## Testing

- `board_registry` lookup and `board_detect`'s VID:PID resolution logic: unit tests, no hardware required (fake signature bytes, fake `db` entries).
- `build_pipeline`'s command construction (arguments passed to `avr-gcc`/`avrdude`, given a fake project layout): unit tests asserting the argument list, not actually invoking the toolchain.
- End-to-end build + flash: manual verification on the ELEGOO UNO R3 at `/dev/ttyACM0`, matching the precedent set by `prototypes/0001-hello-blink` and `prototypes/0002-arduino-core`. A second manual pass with an ATmega328PB board is worth doing given it motivated the per-device-warning design, though not build-blocking.
