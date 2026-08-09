# Board Auto-Detection and One-Button Build & Flash — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Give Citadel an Arduino-IDE-like experience: serial ports are polled and identified automatically (chip signature + USB VID:PID memory), the status bar shows "Board: \<name\> (\<port\>)" with a "Build and Upload" control, and clicking it compiles the vendored `ArduinoCore-avr` core (cached), the project's `cpp/` and `rust/`, links, and flashes via `avrdude` — all off the main thread.

This implements the design approved and committed at [`docs/superpowers/specs/2026-08-09-board-detect-build-flash-design.md`](../specs/2026-08-09-board-detect-build-flash-design.md), which itself resolves both "Open" items [RFC 0002](../../rfcs/0002-product-scope-and-dx.md#board-and-toolchain-detection) flags as blocking this class of work:

1. Chip signature alone can't distinguish Uno/Nano/Pro Mini → resolved via USB VID:PID memory (a one-time picker per physical device, persisted).
2. A detected chip may not be smoke-tested against the pinned nightly → resolved as "accept broadly, warn once per device" (only ATmega328P is actually verified by the existing prototypes; other AVR chips — including ATmega328PB, the board the user's school uses — still build and flash, with a one-time-per-device warning).

Explicitly out of scope (recorded in the spec, not re-litigated here): RFC 0002's full process-isolation backend (this plan uses GPUI `background_spawn` only, which satisfies "UI liveness" but not "a crashed IDE can't leave the port held"), RFC 0004's build-identity marker, and a crowdsourced "community verified chip" registry (would need a Citadel-owned backend that doesn't exist).

**Architecture:** New crate `crates/citadel_build/`, mirroring `crates/citadel_new_project/`'s naming/structure convention, split so pure logic is unit-tested without hardware and GPUI-dependent code is added last:

```
crates/citadel_build/
├── Cargo.toml
└── src/
    ├── citadel_build.rs   # crate root: actions!, init(), BoardIndicator (status item), BuildAndUpload orchestration
    ├── board_registry.rs  # pure: chip signature table, board-family (Uno/Nano/ProMini/Other) avrdude defaults
    ├── board_detect.rs    # pure resolution/parsing logic + impure BoardMonitor entity (serial poll, avrdude signature read, kvp)
    ├── board_picker.rs    # modal Picker<BoardPickerDelegate> for Uno/Nano/Pro Mini/Other (mirrors line_ending_selector.rs)
    └── build_pipeline.rs  # pure command-argument builders + impure toolchain execution (core cache, compile, link, objcopy, flash)
```

**Tech Stack:** Rust, GPUI, `serialport` (new workspace dependency — port enumeration), `db::kvp::KeyValueStore` (existing, persistence), `avr-gcc`/`avr-g++`/`avr-ar`/`avr-objcopy`/`avrdude` (external toolchain, invoked via `util::command::new_command`), `toml`+`serde` (existing workspace deps, parsing the project's `rust/Cargo.toml` package name).

## Plan provenance and review corrections

Produced by a Plan subagent from the approved spec plus two Explore subagents' findings on this codebase's exact conventions (status-bar/action wiring, `db` crate persistence, subprocess spawning), then reviewed against the live source before being finalized. Two GPUI/`ui` API mistakes were caught and are already fixed in the task steps below — noted here so they aren't reintroduced:

- `IconName::Play` does not exist (`crates/icons/src/icons.rs`) — the real variant is `IconName::PlayFilled`.
- `Button` has no `.icon(...)` method (`crates/ui/src/components/button/button.rs`) — use `.start_icon(Icon::new(IconName::PlayFilled))`.

Everything else load-bearing was checked against the real source and matches as written: `Workspace::status_bar()` (`crates/workspace/src/workspace.rs:2602`), `StatusBar::add_right_item` (`crates/workspace/src/status_bar.rs:421`), `db::kvp::KeyValueStore::global(cx)` (`crates/db/src/kvp.rs`), `cx.observe_new::<T>`'s closure signature `Fn(&mut T, Option<&mut Window>, &mut Context<T>)` (`crates/gpui/src/app.rs:2021`), `paths::data_dir()` (`crates/paths/src/paths.rs:144`), `cx.asset_source()` (`crates/gpui/src/app.rs:1896`), `Project::visible_worktrees` (`crates/project/src/project.rs:2403`), `Worktree::abs_path()` (`crates/worktree/src/worktree.rs:831`), and `Picker::nonsearchable_uniform_list` + the `PickerDelegate` trait shape (`crates/picker/src/picker.rs`). A precedent for a status item that self-registers via its own `cx.observe_new::<Workspace>` (rather than being wired centrally in `crates/zed/src/zed.rs`) exists at `crates/go_to_line/src/go_to_line.rs`, confirming Task 8/9's approach is a real pattern, not a novel one.

Optional simplification worth taking during implementation: `crates/db/src/kvp.rs` also exposes a `write_and_log(cx, || async { ... })` helper (used by `Dismissable::set_dismissed`) that collapses the "spawn a background write + log errors" boilerplate this plan hand-rolls in a couple of places (Task 4's picker `confirm`, Task 7's warning-flag write). Not required, but prefer it if it reads cleaner in context.

## Global Constraints

- Do not re-litigate the design spec's scope decisions. This plan only turns the spec into tasks.
- Chip signature read reuses `avrdude` itself (`avrdude -c arduino -p m328p -P <port> -b 115200 -F -U signature:r:-:i`) rather than hand-rolling the STK500 handshake in Rust — `avrdude` is already a hard dependency for flashing, and `-F` skips its own signature-mismatch check so this works for any chip.
- Cross-cutting detection state lives in a GPUI `Global` wrapping an `Entity<BoardMonitor>` (same pattern as `GlobalLanguageModelRegistry`, `crates/language_model/src/registry.rs:18-21`). The status bar indicator and the `BuildAndUpload` action handler both read this global; only `board_detect.rs`'s poll loop writes it.
- Persistence uses the flat `KeyValueStore` (not `ScopedKeyValueStore`) — one string-prefixed key per value, matching `crates/recent_projects/src/dev_container_suggest.rs:16-31`'s precedent. Keys: `citadel_build_board_{vid:04x}_{pid:04x}` (board display name) and `citadel_build_chip_warning_shown_{vid:04x}_{pid:04x}` (presence = warning already shown).
- `show_error_toast_in_workspace` is duplicated (not imported) from `citadel_new_project::new_project` — it's a private ~15-line function there, not a public API; duplicating it is a smaller diff than extracting a shared helper crate for one function.
- `build_pipeline.rs`'s pure half takes no `Fs`-trait dependency — its command-argument construction only needs paths, not file contents, so it's tested with plain `Path`/`PathBuf` fixtures.
- Board-family selection (Uno/Nano/Pro Mini/Other, from the picker) only decides the display name and the `avrdude` programmer/baud defaults. It never decides `-mmcu` — that always comes from the chip-signature lookup in `board_registry.rs`, independent of what the user picked, matching the spec's "any AVR chip signature avr-gcc recognizes is accepted and built."
- The `avrdude` baud-per-board-family table is a heuristic (Nano/Pro Mini bootloaders commonly run at 57600 vs. Uno's 115200) — marked with a `ponytail:` comment noting the upgrade path (a per-VID:PID baud override) if a picked family's default turns out wrong for a specific board.
- Linux `serialport` with USB VID/PID metadata needs `libudev-dev` (Debian/Ubuntu) or equivalent at build time — a build prerequisite, same class as the already-assumed `avr-gcc`/`avrdude`. Called out in Task 2.
- Menu entry goes in the existing **Run** menu (`crates/zed/src/zed/app_menus.rs`, near `MenuItem::action("Spawn Task", ...)`) rather than a new menu — semantically the right home, and free command-palette discoverability.
- `citadel_build`'s status bar item registers itself via its own `cx.observe_new::<Workspace>` inside `citadel_build::init()` (precedent: `crates/go_to_line/src/go_to_line.rs`), not by editing `crates/zed/src/zed.rs`'s central status-bar-assembly block — keeps that file untouched.

---

### Task 1: Vendor `ArduinoCore-avr` under `assets/` and embed it

**Files:**
- Create: `assets/arduino-core/ArduinoCore-avr/` (git submodule)
- Modify: `.gitmodules` (new entry, in addition to the existing `prototypes/0002-arduino-core/vendor/ArduinoCore-avr` entry — same upstream URL, different path, independently pinned)
- Modify: `crates/assets/src/assets.rs` (add `#[include = "arduino-core/**/*"]`)

**Interfaces:**
- Consumes: nothing (first task)
- Produces: `arduino-core/ArduinoCore-avr/cores/arduino/*.{c,cpp,S}` and `.../variants/standard/pins_arduino.h`, reachable at runtime via `cx.asset_source().list(...)`/`.load(...)`. Consumed by Task 8's core-cache extraction.

- [ ] **Step 1:** `git submodule add https://github.com/arduino/ArduinoCore-avr.git assets/arduino-core/ArduinoCore-avr`, then inside it `git checkout 86df345b3cf46754a5db38fb983ec2808ce31303` (tag `1.8.8`, the same commit `prototypes/0002-arduino-core` already verified). Confirm with `git -C assets/arduino-core/ArduinoCore-avr rev-parse HEAD`.

- [ ] **Step 2:** Verify `cores/arduino/main.cpp` and `variants/standard/pins_arduino.h` exist under the new path.

- [ ] **Step 3:** In `crates/assets/src/assets.rs`'s `#[derive(RustEmbed)]` block, add `#[include = "arduino-core/**/*"]` after `#[include = "*.md"]`.

- [ ] **Step 4:** `cargo check -p assets`. This is also the first real test that RustEmbed can walk a ~10k-file submodule without choking — if the include glob needs narrowing (excluding `.git`/docs/examples subfolders not needed at runtime), do it here, since this is the only task that owns `assets.rs`.

- [ ] **Step 5: Commit**

```bash
cd /home/gooya/citadel
git add .gitmodules assets/arduino-core/ArduinoCore-avr crates/assets/src/assets.rs
git commit -m "$(cat <<'EOF'
Vendor ArduinoCore-avr under assets/ and embed it via RustEmbed

Pinned to release tag 1.8.8 (same commit already verified by
prototypes/0002-arduino-core), at a separate submodule path from that
prototype's own vendor copy so citadel_build's core cache doesn't
depend on prototype code sticking around.
EOF
)"
```

---

### Task 2: Crate scaffold + `board_registry.rs` (pure, unit-tested)

**Files:**
- Create: `crates/citadel_build/Cargo.toml`, `crates/citadel_build/src/citadel_build.rs` (module decls only for now), `crates/citadel_build/src/board_registry.rs`
- Modify: root `Cargo.toml` — `[workspace] members` (insert `"crates/citadel_build",` after `"crates/citadel_new_project",`), `[workspace.dependencies]` (insert `citadel_build = { path = "crates/citadel_build" }`; add `serialport = "4"` near other bare version-pinned deps like `serde_json` — check crates.io for the current stable 4.x minor at implementation time rather than trusting a guessed patch version)

**Interfaces:**
- Consumes: nothing (independent of Task 1)
- Produces: `pub struct ChipInfo { signature: [u8;3], mmcu: &'static str, display_name: &'static str, verified: bool }`, `pub fn lookup_chip(signature: [u8;3]) -> Option<&'static ChipInfo>`, `pub enum BoardKind { Uno, Nano, ProMini, Other }`, `pub fn board_kind_display_name(kind) -> &'static str`, `pub fn avrdude_defaults(kind) -> (&'static str, u32)`. Consumed by Tasks 3, 4, 5.

Cargo.toml `[dependencies]`: `anyhow`, `db`, `gpui`, `notifications`, `paths`, `picker`, `serialport`, `serde` (with `derive` feature — needed by Task 5's `toml::from_str`), `toml`, `ui`, `util`, `workspace`, all `.workspace = true`. `[dev-dependencies]`: `gpui` with `test-support`.

- [ ] **Step 1:** Register the workspace member + dependencies as above.

- [ ] **Step 2:** Write the crate manifest (license `GPL-3.0-or-later`, matching `citadel_new_project`; `[lib] path = "src/citadel_build.rs"`).

- [ ] **Step 3 (write failing tests first):** in `board_registry.rs`, a `#[cfg(test)] mod tests` covering: `lookup_chip` finds ATmega328P (`[0x1E,0x95,0x0F]`, `mmcu="atmega328p"`, `verified: true`) and ATmega328PB (`[0x1E,0x95,0x16]`, `mmcu="atmega328pb"`, `verified: false`); returns `None` for an unrecognized signature; `board_kind_display_name` for all four `BoardKind` variants; `avrdude_defaults` always returns programmer `"arduino"`; Uno defaults to baud `115200`.

- [ ] **Step 4:** `cargo test -p citadel_build` — expect compile errors (nothing implemented yet).

- [ ] **Step 5:** Implement `board_registry.rs`: `ChipInfo` struct + `KNOWN_CHIPS: &[ChipInfo]` static table (ATmega328P verified; ATmega328PB, ATmega328, ATmega2560, ATmega32U4 unverified — signatures: `328P=1E950F`, `328PB=1E9516`, `328=1E9514`, `2560=1E9801`, `32U4=1E9587`), `lookup_chip`, `BoardKind` enum, `board_kind_display_name`, and `avrdude_defaults` (Uno: arduino/115200; Nano/ProMini: arduino/57600; Other: arduino/115200) with the `ponytail:` comment on the baud heuristic per Global Constraints.

- [ ] **Step 6:** Crate root `citadel_build.rs`: `mod board_registry;`.

- [ ] **Step 7:** `cargo test -p citadel_build` — expect 7 passed.

- [ ] **Step 8: Commit**

```bash
cd /home/gooya/citadel
git add Cargo.toml crates/citadel_build
git commit -m "$(cat <<'EOF'
Add citadel_build crate scaffold and the board/chip registry

Pure, unit-tested static tables: AVR chip signature -> {mmcu,
display name, verified}, and board-family -> avrdude programmer/baud
defaults. No I/O, no GPUI dependency yet.
EOF
)"
```

---

### Task 3: `board_detect.rs` — pure resolution and parsing logic

**Files:** Modify `citadel_build.rs` (`mod board_detect;`); create `board_detect.rs`.

**Interfaces:**
- Consumes: `serialport::{SerialPortInfo, SerialPortType, UsbPortInfo}` (fields are all `pub`, so tests construct them directly, no I/O).
- Produces: `pub type VidPid = (u16, u16)`, `vid_pid_of(port) -> Option<VidPid>`, `newly_connected_ports(previous: &HashSet<String>, current: &[SerialPortInfo]) -> Vec<&SerialPortInfo>`, `enum BoardIdentity { Known(String), Unknown }`, `resolve_board_identity(stored_name: Option<String>) -> BoardIdentity`, `enum WarningDecision { ShowAndRecord, Skip }`, `decide_unverified_chip_warning(chip_verified: bool, already_warned: bool) -> WarningDecision`, `parse_signature_from_avrdude_ihex(stdout: &[u8]) -> anyhow::Result<[u8;3]>`, `board_kvp_key(VidPid) -> String`, `warning_kvp_key(VidPid) -> String`. Consumed by Task 4 (`board_kvp_key`) and Task 7 (`BoardMonitor`).

- [ ] **Step 1 (write failing tests first):** `vid_pid_of` returns the pair for a USB port / `None` for non-USB; `newly_connected_ports` excludes already-known port names and diffs by *name* (not VID:PID — two identical adapters plugged in at once share a VID:PID but get distinct OS names); `resolve_board_identity` maps `Some(name)`→`Known`, `None`→`Unknown`; `decide_unverified_chip_warning` is `ShowAndRecord` only when `(!verified, !already_warned)`, else `Skip`; `parse_signature_from_avrdude_ihex` parses a real intel-hex signature record (e.g. `:030000001E950F3B\n:00000001FF\n` → `[0x1E,0x95,0x0F]`) and errors on output with no hex record; `board_kvp_key`/`warning_kvp_key` produce stable, zero-padded-hex keys (e.g. `(0x2341, 0x43)` → `citadel_build_board_2341_0043`).

- [ ] **Step 2:** `cargo test -p citadel_build` — expect compile errors.

- [ ] **Step 3:** Implement. `parse_signature_from_avrdude_ihex`: find the first `:`-prefixed line ≥15 chars, read 3 signature bytes as hex starting at character offset 9 (after the `:` + 8-char record header: 2-char length + 4-char address + 2-char type).

- [ ] **Step 4:** Declare `mod board_detect;` in the crate root.

- [ ] **Step 5:** `cargo test -p citadel_build` — expect 19 passed (7 + 12).

- [ ] **Step 6: Commit**

```bash
cd /home/gooya/citadel
git add crates/citadel_build/src/citadel_build.rs crates/citadel_build/src/board_detect.rs
git commit -m "$(cat <<'EOF'
Add pure VID:PID resolution and avrdude-signature parsing logic

Unit-tested without hardware: port diffing, board-identity resolution
from a (fake) persisted name, the once-per-device unverified-chip
warning decision, and parsing avrdude's intel-hex signature-read
output. The impure serial-polling / avrdude-invoking / kvp-touching
glue that calls these is a separate task.
EOF
)"
```

---

### Task 4: `board_picker.rs` — board identification modal

**Files:** Modify `citadel_build.rs` (`mod board_picker;`); create `board_picker.rs`.

**Interfaces:**
- Consumes: `board_registry::{BoardKind, board_kind_display_name}`, `board_detect::{VidPid, board_kvp_key}`.
- Produces: `pub struct BoardPicker` implementing `ModalView`, opened via `pub fn toggle(vid_pid: VidPid, on_picked: Arc<dyn Fn(&str, &mut App) + Send + Sync>, workspace: &WeakEntity<Workspace>, window: &mut Window, cx: &mut App)`. Consumed by Task 8's `BoardIndicator`.

Model this directly on `crates/line_ending_selector/src/line_ending_selector.rs`'s `Picker<D: PickerDelegate>` shape: `Picker::nonsearchable_uniform_list(delegate, window, cx)`, `ModalView`, `EventEmitter<DismissEvent>`, `Focusable`. This module is UI-only (no polling, no signature reads) and depends on a plain callback rather than on `BoardMonitor` (Task 7) directly — keeps it buildable/testable in isolation, same spirit as `citadel_new_project`'s scaffold-before-action ordering.

- [ ] **Step 1:** `mod board_picker;` in the crate root.

- [ ] **Step 2:** Implement `BoardPicker` (holds `picker: Entity<Picker<BoardPickerDelegate>>`) and `BoardPickerDelegate` (`PickerDelegate` impl over a static `const BOARD_KINDS: [BoardKind; 4] = [Uno, Nano, ProMini, Other]`). `confirm()`: writes the picked display name to the kvp store keyed by `board_kvp_key(vid_pid)` (via `cx.background_spawn` or `db::write_and_log`), calls `on_picked(display_name, cx)`, then dismisses. `render_match` renders each `BoardKind` as a `ListItem` with its display name.

- [ ] **Step 3:** `cargo check -p citadel_build` — expect success. No tests here (pure GPUI-rendering glue); exercised by Task 10's manual hardware verification, same as `citadel_new_project`'s own modal-opening code.

- [ ] **Step 4: Commit**

```bash
cd /home/gooya/citadel
git add crates/citadel_build/src/citadel_build.rs crates/citadel_build/src/board_picker.rs
git commit -m "$(cat <<'EOF'
Add the board identification picker modal

Uno/Nano/Pro Mini/Other, modeled directly on
line_ending_selector::LineEndingSelector's Picker<PickerDelegate>
shape. Persists the choice to the kvp store keyed by VID:PID and
calls back into the caller rather than depending on BoardMonitor
directly, so this module stays testable/buildable in isolation.
EOF
)"
```

---

### Task 5: `build_pipeline.rs` — command construction (pure, unit-tested)

**Files:** Modify `citadel_build.rs` (`mod build_pipeline;`); create `build_pipeline.rs`.

**Interfaces:**
- Consumes: nothing beyond `std::path::Path` (independent of Tasks 1-4)
- Produces: `pub struct CommandSpec { program: &'static str, args: Vec<String>, current_dir: Option<PathBuf>, env: HashMap<String,String> }` and pure builders: `core_object_compile_args`, `core_archive_args`, `sketch_compile_args`, `rust_build_command`, `link_args`, `objcopy_args`, `avrdude_flash_args`, `parse_cargo_package_name`. Consumed by Task 6.

Port `prototypes/0002-arduino-core/build.sh`'s exact flags/invocation order into these functions (read that file before implementing) — same `avr-gcc`/`avr-g++` flag set (`-Os -w`, per-language std flags, `-ffunction-sections -fdata-sections`, the `-DF_CPU=16000000L -DARDUINO=10808 -DARDUINO_AVR_UNO -DARDUINO_ARCH_AVR` define set), `avr-ar rcs` for the archive, `-Wl,--gc-sections` at link, `avr-objcopy -O ihex -R .eeprom` for the hex, and an `avrdude -c <programmer> -p <mmcu> -P <port> -b <baud> -U flash:w:<hex>:i` flash command.

- [ ] **Step 1 (write failing tests first):** assert exact argument lists for each builder against fake paths (e.g. `core_object_compile_args` for a `.c` source uses `avr-gcc`/`-std=gnu11`, for a `.cpp` source uses `avr-g++`/`-std=gnu++11`; `core_archive_args` produces `avr-ar rcs <archive> <objs...>`; `rust_build_command` sets `current_dir` to the `rust/` dir, args `["build","--release","-Z","build-std=core","--target","avr-none"]`, env `RUSTFLAGS="-C target-cpu=<mmcu>"`; `link_args` includes `-Wl,--gc-sections`, `-L<rust lib dir>`, `-l<crate name>`, and the core archive path; `objcopy_args` produces exactly `-O ihex -R .eeprom <elf> <hex>`; `avrdude_flash_args` produces `-c <programmer> -p <mmcu> -P <port> -b <baud> -U flash:w:<hex>:i`; `parse_cargo_package_name` reads `[package] name` from a `Cargo.toml` string and errors when there's no `[package]` table).

- [ ] **Step 2:** `cargo test -p citadel_build` — expect compile errors.

- [ ] **Step 3:** Implement the builders as plain data-construction functions (no I/O, no subprocess spawn — that's Task 6). `parse_cargo_package_name` uses a small local `#[derive(serde::Deserialize)]` struct with `toml::from_str`.

- [ ] **Step 4:** Declare `mod build_pipeline;`.

- [ ] **Step 5:** `cargo test -p citadel_build` — expect 29 passed (19 + 10).

- [ ] **Step 6: Commit**

```bash
cd /home/gooya/citadel
git add crates/citadel_build/src/citadel_build.rs crates/citadel_build/src/build_pipeline.rs crates/citadel_build/Cargo.toml
git commit -m "$(cat <<'EOF'
Add pure avr-gcc/avr-g++/avrdude command-argument construction

Unit-tested against fake project layouts, asserting the exact
argument lists each toolchain step would be invoked with -- mirrors
prototypes/0002-arduino-core/build.sh's flags/invocation order. No
subprocess is actually spawned here; execution is a separate task.
EOF
)"
```

---

### Task 6: `build_pipeline.rs` continued — core cache and toolchain execution (impure)

**Files:** Modify `build_pipeline.rs` (append execution functions; same file, no test additions — this half is intentionally not unit-tested, matching the design spec's own testing section and the `write_scaffold`(tested)/`git_init_and_commit`(manual-verified) split in `citadel_new_project`).

**Interfaces:**
- Consumes: Task 5's builders; `util::command::new_command` (`crates/citadel_new_project/src/new_project.rs:74-86`'s established pattern — `.output().await` returns `std::process::Output{status,stdout,stderr}`, giving free output capture for error toasts).
- Produces: `enum BuildStep`, `struct BuildError { step, message }`, `async fn check_toolchain_available() -> Result<(), Vec<&'static str>>` (checks `avr-gcc`/`avr-g++`/`avr-ar`/`avr-objcopy`/`avrdude`/`cargo` are on `PATH` up front, so a missing-toolchain failure names itself instead of surfacing as an opaque first-step error), `async fn ensure_core_archive(core_source_dir, cache_dir, mmcu) -> Result<PathBuf, BuildError>` (compiles every `.c`/`.cpp` under `<core_source_dir>/cores/arduino` into `<cache_dir>/<mmcu>/core.a` once, reuses the existing archive on subsequent calls — this is the "pay the core-compile cost once" behavior the design calls for), `struct BuildTarget { project_root, core_source_dir, core_cache_dir, mmcu, port_name, avrdude_programmer, avrdude_baud }`, `async fn build_and_flash(target: BuildTarget) -> Result<PathBuf, BuildError>` (runs preflight → core cache → compile `cpp/io.cpp` → `cargo build` `rust/` → parse the crate name from `rust/Cargo.toml` → link → objcopy → avrdude flash, returning the produced `.hex` path). Consumed by Task 8.

- [ ] **Step 1:** Implement a small `run(step, spec: &CommandSpec) -> Result<(), BuildError>` helper wrapping `new_command(spec.program).args(&spec.args)...output().await`, mapping a non-zero exit to `BuildError{step, message: stderr}`.

- [ ] **Step 2:** Implement `check_toolchain_available`, `ensure_core_archive` (list `.c`/`.cpp` files under `cores/arduino` via `std::fs::read_dir`, compile each via `core_object_compile_args`, archive via `core_archive_args`), and `build_and_flash` (wires Task 5's builders end to end as described above).

- [ ] **Step 3:** `cargo check -p citadel_build` — expect success.

- [ ] **Step 4: Commit**

```bash
cd /home/gooya/citadel
git add crates/citadel_build/src/build_pipeline.rs
git commit -m "$(cat <<'EOF'
Add toolchain execution to build_pipeline: core cache, compile, link, flash

Wires the Task 5 pure command builders to util::command::new_command,
adds the once-per-mmcu core.a cache (skips recompiling
ArduinoCore-avr on repeat builds), and assembles the full pipeline
into build_and_flash(). Not unit-tested (needs a real AVR toolchain
on PATH) -- covered by the manual hardware verification task instead.
EOF
)"
```

---

### Task 7: `BoardMonitor` — serial polling entity and global (impure)

**Files:** Modify `board_detect.rs` (append the GPUI entity/global — same file, uses Task 3's pure functions).

**Interfaces:**
- Consumes: `board_detect`'s pure functions, `board_registry::lookup_chip`, `db::kvp::KeyValueStore`, `serialport::available_ports`.
- Produces: `struct DetectedBoard { port_name, vid_pid, identity: BoardIdentity, chip_verified: Option<bool>, mmcu: Option<&'static str> }`, `struct BoardMonitor { detected: Option<DetectedBoard>, .. }`, `struct GlobalBoardMonitor(pub Entity<BoardMonitor>)` (`impl Global`), `pub fn init(cx: &mut App)` (creates the entity, `cx.set_global`, starts polling), and event `UnverifiedChipDetected` (`impl EventEmitter<UnverifiedChipDetected> for BoardMonitor`). Consumed by Task 8.

- [ ] **Step 1:** `BoardMonitor::new` spawns a `cx.spawn(async move |this, cx| loop { ... cx.background_executor().timer(Duration::from_secs(2)).await })` poll loop (mirrors `crates/auto_update/src/auto_update.rs:462-479`'s poll-loop shape and `crates/gpui/src/executor.rs`'s `timer`). Each tick: `cx.background_spawn(async { serialport::available_ports().unwrap_or_default() }).await`, then `this.update(cx, |this, cx| this.apply_poll(ports, cx))`; break the loop if the entity was dropped (`.update` returns `Err`).

- [ ] **Step 2:** `apply_poll`: for each `newly_connected_ports` result with a `vid_pid_of`, call `begin_identify`; if the currently-`detected` port disappeared from the current port list, clear `self.detected` (board unplugged); `cx.notify()`.

- [ ] **Step 3:** `begin_identify`: read the stored board name via `KeyValueStore::global(cx).read_kvp(&board_kvp_key(vid_pid))`, resolve via `resolve_board_identity`; spawn a task that calls `read_chip_signature(port_name)` (shells `avrdude -c arduino -p m328p -P <port> -b 115200 -F -U signature:r:-:i` and parses via `parse_signature_from_avrdude_ihex`), looks the result up in `board_registry::lookup_chip`, sets `self.detected`, and calls `maybe_warn_unverified_chip`.

- [ ] **Step 4:** `maybe_warn_unverified_chip`: if the chip lookup succeeded and `decide_unverified_chip_warning(verified, already_warned)` says `ShowAndRecord`, write the warning-shown flag to the kvp store and `cx.emit(UnverifiedChipDetected)` (an event, not a direct toast call, since `BoardMonitor` has no `Workspace` handle — the toast display lives in Task 8's `citadel_build.rs`, which does).

- [ ] **Step 5:** `cargo check -p citadel_build`. Note: this is the one module whose exact GPUI async-closure signatures (`Context::spawn`, `AsyncApp` vs `&mut AsyncApp`) are worth double-checking against the live API while implementing — small adjustments may be needed to compile even though the overall shape is confirmed correct against `auto_update.rs`'s working precedent.

- [ ] **Step 6: Commit**

```bash
cd /home/gooya/citadel
git add crates/citadel_build/src/board_detect.rs
git commit -m "$(cat <<'EOF'
Add BoardMonitor: serial polling entity and cross-workspace global

Polls serialport::available_ports() every 2s off the main thread,
reads new ports' chip signatures via avrdude, resolves VID:PID board
identity from the kvp store, and emits UnverifiedChipDetected for the
once-per-device warning. Exposed as GlobalBoardMonitor so both the
status bar indicator and the BuildAndUpload action can read the
current detection state.
EOF
)"
```

---

### Task 8: `citadel_build.rs` — status bar indicator, action, orchestration

**Files:** Modify `citadel_build.rs` (replace `mod`-only content with the full crate root).

**Interfaces:**
- Consumes: everything from Tasks 2-7 (`board_registry`, `board_detect::{BoardMonitor, GlobalBoardMonitor, DetectedBoard, UnverifiedChipDetected}`, `board_picker::BoardPicker`, `build_pipeline::{BuildTarget, build_and_flash}`).
- Produces: `pub fn init(cx: &mut App)` and the `citadel_build::BuildAndUpload` action, consumed by Task 9's `crates/zed` wiring.

- [ ] **Step 1:** `actions!(citadel_build, [ /* doc comment */ BuildAndUpload ]);` (same shape as `citadel_new_project::NewProject`).

- [ ] **Step 2:** `init(cx)`: calls `board_detect::init(cx)`, then `cx.observe_new(|workspace: &mut Workspace, window, cx| { let Some(window) = window else { return }; ... })` (the `Option<&mut Window>` unwrap is correct per `cx.observe_new`'s real signature) that creates a `BoardIndicator` entity and adds it via `workspace.status_bar().update(cx, |status_bar, cx| status_bar.add_right_item(indicator, window, cx))`, and registers the `BuildAndUpload` action calling `start_build_and_upload(workspace, cx)`.

- [ ] **Step 3:** `show_error_toast_in_workspace`/`show_success_toast_in_workspace` — duplicate the `StatusToast`-based helper from `citadel_new_project::new_project` (error variant) plus a green-check success variant, per Global Constraints.

- [ ] **Step 4:** `start_build_and_upload(workspace, cx)`: reads `cx.try_global::<GlobalBoardMonitor>()`'s `detected` board; bails with a toast if no board, no readable `mmcu`, or the board identity is still `Unknown` (tell the user to click the indicator to identify it first); resolves the picked `BoardKind` back from the stored display name (needs a small `board_kind_from_display_name` addition to `board_registry.rs`, with a round-trip unit test against `board_kind_display_name`); resolves the project root from `workspace.project().read(cx).visible_worktrees(cx).next()`, bailing with a toast if none or if `rust/`/`cpp/` don't exist (not a Citadel project); builds a `BuildTarget` (core cache dir under `paths::data_dir().join("citadel_build").join("arduino-core-1.8.8")`); then `cx.spawn` → `cx.background_spawn` to (a) extract the embedded core sources into the cache dir if not already present (via `cx.asset_source()`, so this step must stay in this GPUI-aware module rather than `build_pipeline.rs`), (b) call `build_pipeline::build_and_flash(target)`, and show a success/error toast with the result.

- [ ] **Step 5:** `extract_core_sources_if_needed(asset_source, dest_dir)`: no-ops if `dest_dir/cores/arduino/main.cpp` already exists; otherwise lists `asset_source.list("arduino-core/ArduinoCore-avr")`, strips the `arduino-core/ArduinoCore-avr/` prefix from each path, and writes each file under `dest_dir`.

- [ ] **Step 6:** `BoardIndicator` (`Render` + `StatusItemView`): holds a `WeakEntity<Workspace>` and two `Subscription`s (`cx.observe(&monitor, ...)` for repaint-on-change, `cx.subscribe(&monitor, ...)` for `UnverifiedChipDetected` → toast). `render()` reads `cx.global::<GlobalBoardMonitor>().0.read(cx).detected`; renders nothing (`div()`) if no board connected; otherwise a small `h_flex()` with two `Button`s: one showing `"Board: {name} ({port})"` or `"Unknown board ({port}) — click to identify"` (on click, opens `board_picker::BoardPicker::toggle(...)` with an `on_picked` callback that updates `monitor.detected.identity` and `cx.notify()`s), and one **`"Build and Upload"`** button with **`.start_icon(Icon::new(IconName::PlayFilled))`** (not `.icon(IconName::Play)` — see the review-corrections note above) that calls `start_build_and_upload`.

- [ ] **Step 7:** `StatusItemView` impl: `set_active_pane_item` is a no-op (board detection is workspace-global, not tab-scoped — same shape as `activity_indicator::ActivityIndicator`'s); `hide_setting` returns `None` (the indicator already self-hides via the empty-`div()` render when nothing's connected — same reasoning `image_info::ImageInfo`/`vim::ModeIndicator` use).

- [ ] **Step 8:** `cargo test -p citadel_build` — expect all prior tests (29) + 1 new (`board_kind_from_display_name` round-trip) = 30 passing, and the crate compiling including the new GPUI code. This is the largest, most speculative chunk of code in the plan — fix any small signature mismatches against the live `gpui`/`workspace`/`ui`/`notifications` APIs as they surface during `cargo check`.

- [ ] **Step 9: Commit**

```bash
cd /home/gooya/citadel
git add crates/citadel_build/src/citadel_build.rs crates/citadel_build/src/board_registry.rs
git commit -m "$(cat <<'EOF'
Wire up BoardIndicator, BuildAndUpload, and the full orchestration flow

Status bar shows "Board: <name> (<port>)" (or the unknown-board
prompt) plus a separate Build and Upload button. Clicking the board
name reopens the identification picker; clicking Build and Upload
extracts+caches the core, builds cpp/ and rust/, links, objcopies,
and flashes -- all off the main thread via cx.background_spawn, with
distinct error/success toasts per step and a one-time-per-device
unverified-chip warning toast.
EOF
)"
```

---

### Task 9: Wire into `crates/zed`

**Files:**
- Modify: `crates/zed/Cargo.toml` (add dependency, next to `citadel_new_project`)
- Modify: `crates/zed/src/main.rs` (call `citadel_build::init(cx);`)
- Modify: `crates/zed/src/zed/app_menus.rs` (Run menu entry)

**Interfaces:**
- Consumes: `citadel_build::init` and `citadel_build::BuildAndUpload` (Task 8).
- Produces: nothing consumed by later tasks (final wiring task before manual verification).

- [ ] **Step 1:** In `crates/zed/Cargo.toml`, add `citadel_build = { workspace = true }` directly above `citadel_new_project = { workspace = true }`.

- [ ] **Step 2:** In `crates/zed/src/main.rs`, add `citadel_build::init(cx);` directly above `citadel_new_project::init(cx);`.

- [ ] **Step 3:** In `crates/zed/src/zed/app_menus.rs`, in the `"Run"` menu's `items` vec, insert `MenuItem::action("Build and Upload", citadel_build::BuildAndUpload)` directly after `MenuItem::action("Spawn Task", zed_actions::Spawn::ViaModal { reveal_target: None })`.

- [ ] **Step 4:** `cargo check -p zed` — expect success.

- [ ] **Step 5: Commit**

```bash
cd /home/gooya/citadel
git add crates/zed/Cargo.toml crates/zed/src/main.rs crates/zed/src/zed/app_menus.rs
git commit -m "$(cat <<'EOF'
Wire citadel_build into the zed binary

Adds the dependency, calls citadel_build::init(cx) alongside the
other feature-crate inits, and adds "Build and Upload" to the Run
menu next to Spawn Task (same semantic category: build/run the
project).
EOF
)"
```

---

### Task 10: Manual hardware verification (human required)

**Files:** None (verification only; if any fix is needed, it becomes a new small commit on top, not part of this task's own commit)

**Interfaces:**
- Consumes: everything from Tasks 1-9.
- Produces: nothing consumed by later tasks (final task).

Nothing in Tasks 1-9 exercises a real serial port, a real `avrdude` handshake, or a real chip. Do not consider this plan complete until a human confirms the outcomes below.

- [ ] **Step 1:** `which avr-gcc avr-g++ avr-ar avr-objcopy avrdude` — all resolve. If any are missing, install the AVR toolchain + avrdude first (same prerequisite `prototypes/0001-hello-blink`/`0002-arduino-core` already assumed).

- [ ] **Step 2:** Confirm the ELEGOO UNO R3 is connected: `ls -la /dev/ttyACM0`.

- [ ] **Step 3:** `cargo run -p zed`. In the running editor, use `citadel_new_project`'s "New Project..." to scaffold an empty test project (or reuse an existing scaffolded one).

- [ ] **Step 4:** With the ELEGOO UNO R3 connected, confirm the status bar shows either "Unknown board (/dev/ttyACM0) — click to identify" (first time this VID:PID is seen) or "Board: Arduino Uno (/dev/ttyACM0)" (if already identified in a prior run). If unknown, click it, pick "Uno" in the picker, confirm the label updates without needing to unplug/replug.

- [ ] **Step 5:** Click "Build and Upload" (status bar button and/or Run > Build and Upload). Confirm: a first build takes noticeably longer than subsequent ones (core cache miss vs. hit — check `core.a` exists under `paths::data_dir()`'s `citadel_build/arduino-core-1.8.8/atmega328p/`); on success, a green toast names the produced `.hex` path; `avrdude`'s write+verify actually ran and the board's actual I/O behavior (whatever `cpp/io.cpp` does) matches expectations. Then introduce a deliberate compile error in `cpp/io.cpp`, click Build and Upload again, confirm a red error toast names the failing step and quotes `avr-g++`'s stderr, and that no stale `.hex` gets (re-)flashed.

- [ ] **Step 6 (requires a second AVR board with a non-ATmega328P signature — per the design spec, an ATmega328PB board is the motivating case, since it's what the user's school uses):** connect it. Confirm a warning toast appears once ("chip isn't one of the toolchain-verified parts..."). Trigger another build against the same board (same VID:PID); confirm the warning does **not** appear a second time. Confirm build+flash still succeeds despite the warning. If no second board is available, skip this step and note it as unverified in the PR description — the spec itself frames this as "worth doing... though not build-blocking."

- [ ] **Step 7:** If everything in Steps 4-6 passes, no further commit is required — the plan is complete. If any step required a code fix, make the fix, verify with a fresh `cargo run -p zed` + repeat the relevant step, then commit the fix separately with a message describing what hardware behavior it corrects.

---

### Critical Files for Implementation

- `docs/superpowers/specs/2026-08-09-board-detect-build-flash-design.md`
- `prototypes/0002-arduino-core/build.sh` (exact compiler flags/invocation order to port)
- `crates/citadel_new_project/src/new_project.rs` (toast helper, `new_command` usage, action-registration pattern to mirror)
- `crates/line_ending_selector/src/line_ending_indicator.rs` + `line_ending_selector.rs` (status item + picker pattern to mirror)
- `crates/db/src/kvp.rs` (persistence pattern)
- `crates/citadel_build/src/citadel_build.rs`
- `crates/citadel_build/src/board_detect.rs`
- `crates/citadel_build/src/build_pipeline.rs`
