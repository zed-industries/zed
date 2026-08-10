# Serial Monitor and Serial Plotter Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Give Citadel a Serial Monitor dock panel (live text log, send box, log export) and a Serial Plotter floating window (real-time numeric line chart), sharing one persistent serial connection that automatically pauses around `citadel_build`'s flash pipeline.

**Architecture:** A new crate `crates/citadel_serial_monitor/` owns a single `SerialConnection` GPUI entity (the only thing in the process that ever holds an open OS serial port handle) plus a `SerialMonitorPanel` (implements `workspace::dock::Panel`, single pane, no tabs — matches board_detect's single-detected-board scope) and a `SerialPlotterWindow` (a floating OS window, following this codebase's existing `open_about_window` pattern). `citadel_build` (existing crate) is modified minimally to emit two new GPUI events around its flash step; `citadel_serial_monitor` depends on `citadel_build` to subscribe to those events (this is a one-directional dependency — `citadel_build` does not know `citadel_serial_monitor` exists).

**Tech Stack:** Rust, GPUI (`Panel` trait, `cx.open_window`, `canvas()`/`PathBuilder` for the chart, `Editor::single_line` for text inputs, `uniform_list` for the scrolling log), `serialport` (already a workspace dependency), `futures::channel::mpsc` (thread-to-entity message passing, same shape `crates/terminal/src/terminal.rs` already uses).

## Global Constraints

- Single-port scope only: both the Monitor panel and the Plotter window target one connection at a time. No tabbed multi-port UI (matches `board_detect`'s current single-detected-board model).
- Session-only state: baud rate and line-ending choice are not persisted across restarts. No new `db`/kvp usage in this crate.
- `citadel_serial_monitor` depends on `citadel_build`; `citadel_build` must **not** gain a dependency on `citadel_serial_monitor` (this was the design spec's original wording — corrected here, see "Corrections applied during review" below).
- `SerialConnection.lines` is capped at 1000 entries (oldest dropped first); "Save Log" exports whatever is currently buffered, not a guaranteed full session transcript.
- `SerialConnection` never connects on its own initiative — only in response to the user opening the Monitor panel or Plotter window (defaulting the port field to the currently-detected board's port, if any, via `citadel_build::board_detect::GlobalBoardMonitor`).
- Plot line parsing: tokens split on `,`/tab/space; `label:value` names a series explicitly, a bare number gets an auto name (`value1`, `value2`, ...); unparseable tokens are silently skipped.
- No new external crate dependency for charting — draw the line chart with GPUI's own `canvas()` element and `PathBuilder`/`window.paint_path`.

## Corrections applied during review

The committed design spec (`docs/superpowers/specs/2026-08-10-serial-monitor-plotter-design.md`) has two gaps found while turning it into concrete tasks, both fixed in the tasks below and in the spec itself:

1. **Circular crate dependency.** The spec says "`citadel_build` gains a dependency on `citadel_serial_monitor`... so its build orchestration can call two functions to pause/resume the monitor". But `citadel_serial_monitor` also needs `citadel_build::board_detect::GlobalBoardMonitor` to default the Monitor/Plotter's port field to the currently-detected board — that's a dependency in the *other* direction, which would make the two crates depend on each other (does not compile). Fixed by flipping the flash-coordination dependency: `citadel_build` gains two small GPUI events (`FlashStarted`/`FlashFinished`, emitted from its existing `BoardMonitor` entity around the existing `avrdude` call) and otherwise doesn't change; `citadel_serial_monitor` (which already needs to depend on `citadel_build` for the default-port lookup) subscribes to those events itself and calls its own internal pause/resume. `citadel_build` never imports or references `citadel_serial_monitor`.
2. **No port selector in the Monitor panel's UI.** The spec's `SerialMonitorPanel` header row lists "baud-rate dropdown, Connect/Disconnect, Save Log" but no way to pick or see which port is targeted — unusable when no board is auto-detected, or when the user wants a different port than the detected one. Fixed by adding a port field (a plain text input, pre-filled with the detected board's port name as a default, editable) to the header row in Task 5.

Both corrections are also applied as small edits to the spec doc in Task 1... actually applied directly to the spec doc as part of this planning pass (see the spec file's own updated Architecture section) rather than as a plan task, since they're documentation fixes, not implementation work.

## File Structure

```
crates/citadel_build/src/board_detect.rs        # MODIFY: add FlashStarted/FlashFinished events
crates/citadel_build/src/citadel_build.rs        # MODIFY: emit them around build_and_flash

crates/citadel_serial_monitor/
├── Cargo.toml
└── src/
    ├── citadel_serial_monitor.rs   # crate root: actions!, init(), toast helpers, OpenSerialPlotter handler
    ├── serial_connection.rs        # pure split_lines() + impure SerialConnection entity/Global
    ├── plot_parser.rs              # pure parse_plot_line()
    ├── serial_monitor_panel.rs     # Panel impl: port/baud fields, log view, send row, save log
    └── serial_plotter_window.rs    # floating window: canvas-drawn multi-series line chart

crates/zed/Cargo.toml                            # MODIFY: add citadel_serial_monitor dependency
crates/zed/src/main.rs                           # MODIFY: call citadel_serial_monitor::init(cx)
crates/zed/src/zed.rs                            # MODIFY: add SerialMonitorPanel to initialize_panels
crates/zed/src/zed/app_menus.rs                  # MODIFY: add "Open Serial Plotter" to the Run menu
Cargo.toml                                       # MODIFY: workspace members + citadel_serial_monitor dep
```

---

### Task 1: `citadel_build` — emit `FlashStarted`/`FlashFinished` events

**Files:**
- Modify: `crates/citadel_build/src/board_detect.rs` (add two event types)
- Modify: `crates/citadel_build/src/citadel_build.rs` (emit them in `start_build_and_upload`)

**Interfaces:**
- Produces: `pub struct FlashStarted { pub port_name: String }`, `pub struct FlashFinished { pub port_name: String }`, both `impl EventEmitter<_> for BoardMonitor`. Consumed by Task 4 (`citadel_serial_monitor`'s `SerialConnection` subscribes to these on `citadel_build::board_detect::BoardMonitor`).

- [ ] **Step 1:** In `crates/citadel_build/src/board_detect.rs`, immediately after the existing `SignatureReadFailed` event definition (search for `pub struct SignatureReadFailed`), add:

```rust
/// Emitted immediately before `citadel_build` invokes `avrdude` to flash
/// `port_name`. Unrelated to board identification -- exists so other
/// crates (e.g. a serial monitor) can react to the port being about to be
/// exclusively claimed by avrdude, without `citadel_build` needing to know
/// who's listening or why.
pub struct FlashStarted {
    pub port_name: String,
}

impl EventEmitter<FlashStarted> for BoardMonitor {}

/// Emitted after the flash attempt for `port_name` finishes, whether it
/// succeeded or failed. Mirrors `FlashStarted`.
pub struct FlashFinished {
    pub port_name: String,
}

impl EventEmitter<FlashFinished> for BoardMonitor {}
```

- [ ] **Step 2:** In `crates/citadel_build/src/citadel_build.rs`, update the `board_detect` import line from:

```rust
use board_detect::{
    BoardIdentity, GlobalBoardMonitor, SignatureReadFailed, UnverifiedChipDetected,
};
```

to:

```rust
use board_detect::{
    BoardIdentity, FlashFinished, FlashStarted, GlobalBoardMonitor, SignatureReadFailed,
    UnverifiedChipDetected,
};
```

- [ ] **Step 3:** In `start_build_and_upload`, find this block (it constructs `target` and then spawns the build):

```rust
    let (programmer, baud) = avrdude_defaults(board_kind);
    let core_dir = paths::data_dir()
        .join("citadel_build")
        .join("arduino-core-1.8.8");
    let target = BuildTarget {
        project_root,
        core_source_dir: core_dir.clone(),
        core_cache_dir: core_dir.clone(),
        mmcu: mmcu.to_string(),
        port_name: detected.port_name.clone(),
        avrdude_programmer: programmer.to_string(),
        avrdude_baud: baud,
    };
    let asset_source = cx.asset_source().clone();
```

Add one line right after it (a clone of the port name, used only for the two new events — `target.port_name` itself gets moved into `build_and_flash` later):

```rust
    let asset_source = cx.asset_source().clone();
    let port_name = detected.port_name.clone();
```

- [ ] **Step 4:** In the same function, find the `cx.spawn` block:

```rust
    cx.spawn(async move |workspace, cx| {
        let extract_result = cx
            .background_spawn({
                let core_dir = core_dir.clone();
                async move { extract_core_sources_if_needed(asset_source.as_ref(), &core_dir) }
            })
            .await;

        if let Err(error) = extract_result {
            workspace
                .update(cx, |workspace, cx| {
                    show_error_toast_in_workspace(workspace, error.to_string(), cx);
                })
                .log_err();
            return;
        }

        let build_result = cx.background_spawn(build_and_flash(target)).await;

        workspace
            .update(cx, |workspace, cx| match build_result {
                Ok(hex_path) => show_success_toast_in_workspace(
                    workspace,
                    format!("Build and upload succeeded: {}", hex_path.display()),
                    cx,
                ),
                Err(error) => show_error_toast_in_workspace(workspace, error.to_string(), cx),
            })
            .log_err();
    })
    .detach();
```

Replace it with (adds a `monitor` capture, and the two `cx.emit` calls around the `build_and_flash` call):

```rust
    cx.spawn(async move |workspace, cx| {
        let extract_result = cx
            .background_spawn({
                let core_dir = core_dir.clone();
                async move { extract_core_sources_if_needed(asset_source.as_ref(), &core_dir) }
            })
            .await;

        if let Err(error) = extract_result {
            workspace
                .update(cx, |workspace, cx| {
                    show_error_toast_in_workspace(workspace, error.to_string(), cx);
                })
                .log_err();
            return;
        }

        monitor
            .update(cx, |_, cx| {
                cx.emit(FlashStarted {
                    port_name: port_name.clone(),
                });
            })
            .log_err();

        let build_result = cx.background_spawn(build_and_flash(target)).await;

        monitor
            .update(cx, |_, cx| {
                cx.emit(FlashFinished {
                    port_name: port_name.clone(),
                });
            })
            .log_err();

        workspace
            .update(cx, |workspace, cx| match build_result {
                Ok(hex_path) => show_success_toast_in_workspace(
                    workspace,
                    format!("Build and upload succeeded: {}", hex_path.display()),
                    cx,
                ),
                Err(error) => show_error_toast_in_workspace(workspace, error.to_string(), cx),
            })
            .log_err();
    })
    .detach();
```

`monitor` is already in scope at this point in the function (it's the `Entity<BoardMonitor>` looked up at the top of `start_build_and_upload` to read `detected`), so no new variable capture setup is needed beyond it already being a plain local.

- [ ] **Step 5:** Verify the crate still builds and all existing tests still pass:

Run: `cargo test -p citadel_build`
Expected: `test result: ok. 39 passed; 0 failed;` (same count as before — this task adds no new tests, it adds event plumbing with no pure-logic branch to unit test, matching this crate's existing convention of not unit-testing GPUI entity behavior).

- [ ] **Step 6:** Commit:

```bash
git add crates/citadel_build/src/board_detect.rs crates/citadel_build/src/citadel_build.rs
git commit -m "$(cat <<'EOF'
Emit FlashStarted/FlashFinished events around avrdude flashing

A future serial monitor needs to know when citadel_build is about to
exclusively claim the serial port for flashing, and when it's done, so
it can release and reacquire its own connection around that window.
citadel_build doesn't need to know who's listening.
EOF
)"
```

---

### Task 2: `citadel_serial_monitor` crate scaffold + pure line-splitting

**Files:**
- Create: `crates/citadel_serial_monitor/Cargo.toml`
- Create: `crates/citadel_serial_monitor/src/citadel_serial_monitor.rs` (module decls only for now)
- Create: `crates/citadel_serial_monitor/src/serial_connection.rs`
- Modify: root `Cargo.toml` (`[workspace] members` + `[workspace.dependencies]`)

**Interfaces:**
- Produces: `pub fn split_lines(carry: &mut Vec<u8>, chunk: &[u8]) -> Vec<String>`. Consumed by Task 4.

- [ ] **Step 1:** In root `Cargo.toml`, find the `"crates/citadel_build",` line under `[workspace] members` and add a line right after it:

```toml
    "crates/citadel_build",
    "crates/citadel_serial_monitor",
```

- [ ] **Step 2:** In root `Cargo.toml`, find `citadel_build = { path = "crates/citadel_build" }` under `[workspace.dependencies]` and add a line right after it:

```toml
citadel_build = { path = "crates/citadel_build" }
citadel_serial_monitor = { path = "crates/citadel_serial_monitor" }
```

- [ ] **Step 3:** Write `crates/citadel_serial_monitor/Cargo.toml`:

```toml
[package]
name = "citadel_serial_monitor"
version = "0.1.0"
edition.workspace = true
publish.workspace = true
license = "GPL-3.0-or-later"

[lints]
workspace = true

[lib]
name = "citadel_serial_monitor"
path = "src/citadel_serial_monitor.rs"

[dependencies]
anyhow.workspace = true
citadel_build.workspace = true
editor.workspace = true
futures.workspace = true
gpui.workspace = true
notifications.workspace = true
serialport.workspace = true
ui.workspace = true
util.workspace = true
workspace.workspace = true

[dev-dependencies]
gpui = { workspace = true, features = ["test-support"] }
```

- [ ] **Step 4:** Write `crates/citadel_serial_monitor/src/citadel_serial_monitor.rs` with module declarations only (filled in by later tasks):

```rust
mod plot_parser;
pub mod serial_connection;
mod serial_monitor_panel;
mod serial_plotter_window;
```

- [ ] **Step 5 (TDD — write failing tests first):** Write `crates/citadel_serial_monitor/src/serial_connection.rs`:

```rust
/// Splits `chunk` into complete lines, carrying any trailing partial line
/// over in `carry` for the next call. Recognizes `\n` as the line
/// terminator and strips a trailing `\r` (so both `\n`- and
/// `\r\n`-terminated sketches work). Invalid UTF-8 is replaced per
/// `String::from_utf8_lossy` rather than erroring -- a serial device can
/// send anything, and one garbled line shouldn't stop the monitor.
pub fn split_lines(carry: &mut Vec<u8>, chunk: &[u8]) -> Vec<String> {
    carry.extend_from_slice(chunk);
    let mut lines = Vec::new();
    while let Some(newline_index) = carry.iter().position(|&byte| byte == b'\n') {
        let mut line_bytes: Vec<u8> = carry.drain(..=newline_index).collect();
        line_bytes.pop(); // remove the '\n' itself
        if line_bytes.last() == Some(&b'\r') {
            line_bytes.pop();
        }
        lines.push(String::from_utf8_lossy(&line_bytes).into_owned());
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_lines_single_complete_line() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"hello\n");
        assert_eq!(lines, vec!["hello".to_string()]);
        assert!(carry.is_empty());
    }

    #[test]
    fn test_split_lines_multiple_lines_one_chunk() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"x\ny\nz\n");
        assert_eq!(lines, vec!["x".to_string(), "y".to_string(), "z".to_string()]);
        assert!(carry.is_empty());
    }

    #[test]
    fn test_split_lines_partial_line_carried_over() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"hello\nworl");
        assert_eq!(lines, vec!["hello".to_string()]);
        assert_eq!(carry, b"worl");

        let lines = split_lines(&mut carry, b"d\n");
        assert_eq!(lines, vec!["world".to_string()]);
        assert!(carry.is_empty());
    }

    #[test]
    fn test_split_lines_strips_carriage_return() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"a\r\nb\r\n");
        assert_eq!(lines, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_split_lines_no_newline_yet() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"abc");
        assert!(lines.is_empty());
        assert_eq!(carry, b"abc");
    }

    #[test]
    fn test_split_lines_invalid_utf8_does_not_panic() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, &[0xFF, b'\n']);
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn test_split_lines_empty_line() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"\n");
        assert_eq!(lines, vec!["".to_string()]);
    }
}
```

- [ ] **Step 6:** Run: `cargo test -p citadel_serial_monitor`
Expected: `test result: ok. 7 passed; 0 failed;`

- [ ] **Step 7:** Commit:

```bash
git add Cargo.toml crates/citadel_serial_monitor
git commit -m "$(cat <<'EOF'
Add citadel_serial_monitor crate scaffold and line-splitting logic

Splits bytes arriving from a serial port into complete lines, carrying
a partial trailing line across reads. This is the shared parsing step
both the Serial Monitor's text log and the Serial Plotter's per-line
value parser build on.
EOF
)"
```

---

### Task 3: Pure plot-line parser

**Files:**
- Create: `crates/citadel_serial_monitor/src/plot_parser.rs`
- Modify: `crates/citadel_serial_monitor/src/citadel_serial_monitor.rs` (`mod plot_parser;` already added in Task 2 — no change needed here, this task only fills in the file)

**Interfaces:**
- Produces: `pub struct PlotPoint { pub label: String, pub value: f32 }`, `pub fn parse_plot_line(line: &str) -> Vec<PlotPoint>`. Consumed by Task 6 (`serial_plotter_window.rs`).

- [ ] **Step 1 (TDD):** Write `crates/citadel_serial_monitor/src/plot_parser.rs`:

```rust
/// One value parsed out of a received serial line, for the Serial Plotter.
#[derive(Debug, Clone, PartialEq)]
pub struct PlotPoint {
    pub label: String,
    pub value: f32,
}

/// Parses one received line into zero or more labeled numeric points,
/// following the Arduino Plotter convention: tokens are separated by `,`,
/// tab, or space. A token shaped `<label>:<number>` names its series
/// explicitly; a bare `<number>` token is assigned to an auto-named series
/// (`value1`, `value2`, ... by position among the bare tokens in this
/// line). Tokens that parse as neither are silently skipped -- ordinary
/// log lines interleaved with numeric telemetry are expected, not an
/// error.
pub fn parse_plot_line(line: &str) -> Vec<PlotPoint> {
    let mut points = Vec::new();
    let mut bare_index = 0;
    for token in line.split(|c: char| c == ',' || c == '\t' || c == ' ') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        if let Some((label, value)) = token.split_once(':') {
            if let Ok(value) = value.trim().parse::<f32>() {
                points.push(PlotPoint {
                    label: label.trim().to_string(),
                    value,
                });
                continue;
            }
        }
        if let Ok(value) = token.parse::<f32>() {
            bare_index += 1;
            points.push(PlotPoint {
                label: format!("value{bare_index}"),
                value,
            });
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_labeled_values() {
        let points = parse_plot_line("temp:23.5,humidity:60");
        assert_eq!(
            points,
            vec![
                PlotPoint { label: "temp".to_string(), value: 23.5 },
                PlotPoint { label: "humidity".to_string(), value: 60.0 },
            ]
        );
    }

    #[test]
    fn test_parse_bare_values_auto_named() {
        let points = parse_plot_line("23.5,60");
        assert_eq!(
            points,
            vec![
                PlotPoint { label: "value1".to_string(), value: 23.5 },
                PlotPoint { label: "value2".to_string(), value: 60.0 },
            ]
        );
    }

    #[test]
    fn test_parse_mixed_labeled_and_bare() {
        let points = parse_plot_line("label:1.0, 2.0");
        assert_eq!(
            points,
            vec![
                PlotPoint { label: "label".to_string(), value: 1.0 },
                PlotPoint { label: "value1".to_string(), value: 2.0 },
            ]
        );
    }

    #[test]
    fn test_parse_skips_non_numeric_tokens() {
        let points = parse_plot_line("hello world 42");
        assert_eq!(points, vec![PlotPoint { label: "value1".to_string(), value: 42.0 }]);
    }

    #[test]
    fn test_parse_empty_line() {
        assert_eq!(parse_plot_line(""), Vec::new());
    }

    #[test]
    fn test_parse_labeled_token_with_non_numeric_value_is_skipped() {
        assert_eq!(parse_plot_line("label:notanumber"), Vec::new());
    }

    #[test]
    fn test_parse_tab_separated() {
        let points = parse_plot_line("1.0\t2.0");
        assert_eq!(
            points,
            vec![
                PlotPoint { label: "value1".to_string(), value: 1.0 },
                PlotPoint { label: "value2".to_string(), value: 2.0 },
            ]
        );
    }
}
```

- [ ] **Step 2:** Run: `cargo test -p citadel_serial_monitor`
Expected: `test result: ok. 14 passed; 0 failed;` (7 from Task 2 + 7 new)

- [ ] **Step 3:** Commit:

```bash
git add crates/citadel_serial_monitor/src/plot_parser.rs
git commit -m "Add pure serial-plot-line parser for the Serial Plotter"
```

---

### Task 4: `SerialConnection` entity (impure) — connect, disconnect, send, pause/resume

**Files:**
- Modify: `crates/citadel_serial_monitor/src/serial_connection.rs` (append; `split_lines` from Task 2 stays as-is above this)

**Interfaces:**
- Consumes: Task 1's `citadel_build::board_detect::{BoardMonitor, FlashStarted, FlashFinished, GlobalBoardMonitor}`; Task 2's `split_lines`.
- Produces: `pub struct SerialConnection { pub port_name: Option<String>, pub baud_rate: u32, pub is_open: bool, pub lines: VecDeque<String>, .. }` with `pub fn connect(&mut self, port_name: String, baud_rate: u32, cx: &mut Context<Self>)`, `pub fn disconnect(&mut self, cx: &mut Context<Self>)`, `pub fn send(&self, bytes: Vec<u8>, cx: &mut Context<Self>)`; `pub struct GlobalSerialConnection(pub Entity<SerialConnection>)` (`impl Global`); `pub struct SerialLineReceived(pub String)`, `pub struct SerialConnectionError(pub String)` (both `impl EventEmitter<_> for SerialConnection`); `pub fn init(cx: &mut App)`; `pub fn default_port_name(cx: &App) -> Option<String>`. Consumed by Task 5 (`serial_monitor_panel.rs`) and Task 6 (`serial_plotter_window.rs`).

This is the crate's most GPUI/threading-heavy module. `cargo check` is the primary verification step (no unit tests for the impure entity itself, matching `citadel_build::board_detect::BoardMonitor`'s own precedent — its threading/I/O is verified by Task 9's manual hardware pass, not unit tests).

- [ ] **Step 1:** Append this to the bottom of `crates/citadel_serial_monitor/src/serial_connection.rs` (below the existing `split_lines` function and its `#[cfg(test)]` module):

```rust
use citadel_build::board_detect::{BoardMonitor, FlashFinished, FlashStarted, GlobalBoardMonitor};
use futures::channel::mpsc::{UnboundedSender, unbounded};
use futures::StreamExt;
use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Global, Subscription};
use serialport::SerialPort;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const MAX_BUFFERED_LINES: usize = 1000;
pub const DEFAULT_BAUD_RATE: u32 = 9600;

/// Emitted for every complete line received on the open connection.
pub struct SerialLineReceived(pub String);
impl EventEmitter<SerialLineReceived> for SerialConnection {}

/// Emitted when the connection fails to open or a read/write fails while
/// connected. Also flips `is_open` to `false`.
pub struct SerialConnectionError(pub String);
impl EventEmitter<SerialConnectionError> for SerialConnection {}

pub struct GlobalSerialConnection(pub Entity<SerialConnection>);
impl Global for GlobalSerialConnection {}

pub fn init(cx: &mut App) {
    let connection = cx.new(SerialConnection::new);
    cx.set_global(GlobalSerialConnection(connection));
}

/// Reads `citadel_build::board_detect::GlobalBoardMonitor`'s currently
/// detected board, if any, to use as the default port for a freshly opened
/// Monitor panel or Plotter window. Never triggers a connection by itself.
pub fn default_port_name(cx: &App) -> Option<String> {
    cx.try_global::<GlobalBoardMonitor>()?
        .0
        .read(cx)
        .detected
        .as_ref()
        .map(|detected| detected.port_name.clone())
}

enum SerialConnectionMessage {
    Opened,
    Line(String),
    Error(String),
}

/// Bundles the OS handle (shared with the background reader loop via a
/// mutex so `send()` can write without a second open handle) and the
/// channel used to report read/write errors back to the entity.
struct OpenPort {
    handle: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
    message_tx: UnboundedSender<SerialConnectionMessage>,
}

/// The single owner of the process's one persistent serial port connection.
/// Shared by the Monitor panel and the Plotter window via
/// `GlobalSerialConnection` so they never try to open the same port twice.
pub struct SerialConnection {
    pub port_name: Option<String>,
    pub baud_rate: u32,
    pub is_open: bool,
    pub lines: VecDeque<String>,
    open: Option<OpenPort>,
    /// Remembers (port_name, baud_rate) while paused for a flash, so
    /// `resume_after_flash` can reconnect at the same settings.
    paused: Option<(String, u32)>,
    _subscriptions: Vec<Subscription>,
}

impl SerialConnection {
    fn new(cx: &mut Context<Self>) -> Self {
        let mut subscriptions = Vec::new();
        if let Some(monitor) = cx.try_global::<GlobalBoardMonitor>() {
            let monitor = monitor.0.clone();
            subscriptions.push(cx.subscribe(&monitor, Self::handle_flash_started));
            subscriptions.push(cx.subscribe(&monitor, Self::handle_flash_finished));
        }

        Self {
            port_name: None,
            baud_rate: DEFAULT_BAUD_RATE,
            is_open: false,
            lines: VecDeque::new(),
            open: None,
            paused: None,
            _subscriptions: subscriptions,
        }
    }

    fn handle_flash_started(
        &mut self,
        _monitor: Entity<BoardMonitor>,
        event: &FlashStarted,
        cx: &mut Context<Self>,
    ) {
        if self.is_open && self.port_name.as_deref() == Some(event.port_name.as_str()) {
            self.pause_for_flash(cx);
        }
    }

    fn handle_flash_finished(
        &mut self,
        _monitor: Entity<BoardMonitor>,
        event: &FlashFinished,
        cx: &mut Context<Self>,
    ) {
        let should_resume = self
            .paused
            .as_ref()
            .is_some_and(|(port_name, _)| port_name == &event.port_name);
        if should_resume {
            self.resume_after_flash(cx);
        }
    }

    pub fn connect(&mut self, port_name: String, baud_rate: u32, cx: &mut Context<Self>) {
        self.paused = None;
        self.port_name = Some(port_name.clone());
        self.baud_rate = baud_rate;

        let (message_tx, mut message_rx) = unbounded::<SerialConnectionMessage>();
        let handle: Arc<Mutex<Option<Box<dyn SerialPort>>>> = Arc::new(Mutex::new(None));
        self.open = Some(OpenPort {
            handle: handle.clone(),
            message_tx: message_tx.clone(),
        });

        cx.background_spawn(async move {
            run_serial_reader(port_name, baud_rate, handle, message_tx).await;
        })
        .detach();

        cx.spawn(async move |this, cx| -> anyhow::Result<()> {
            while let Some(message) = message_rx.next().await {
                this.update(cx, |this, cx| match message {
                    SerialConnectionMessage::Opened => {
                        this.is_open = true;
                        cx.notify();
                    }
                    SerialConnectionMessage::Line(line) => {
                        this.push_line(line, cx);
                    }
                    SerialConnectionMessage::Error(error) => {
                        this.is_open = false;
                        cx.emit(SerialConnectionError(error));
                        cx.notify();
                    }
                })?;
            }
            anyhow::Ok(())
        })
        .detach();
    }

    pub fn disconnect(&mut self, cx: &mut Context<Self>) {
        self.close_port(cx);
        self.port_name = None;
        self.paused = None;
    }

    pub fn send(&self, bytes: Vec<u8>, cx: &mut Context<Self>) {
        let Some(open) = &self.open else { return };
        let handle = open.handle.clone();
        let message_tx = open.message_tx.clone();
        cx.background_spawn(async move {
            let write_result = {
                let Ok(mut guard) = handle.lock() else { return };
                match guard.as_mut() {
                    Some(port) => port.write_all(&bytes).map_err(|error| error.to_string()),
                    None => return,
                }
            };
            if let Err(error) = write_result {
                // ponytail: best-effort -- if the foreground entity (and its
                // message_rx loop) is already gone, there's no one left to
                // tell and nothing to recover.
                message_tx
                    .unbounded_send(SerialConnectionMessage::Error(error))
                    .ok();
            }
        })
        .detach();
    }

    fn pause_for_flash(&mut self, cx: &mut Context<Self>) {
        let Some(port_name) = self.port_name.clone() else { return };
        self.paused = Some((port_name, self.baud_rate));
        self.close_port(cx);
    }

    fn resume_after_flash(&mut self, cx: &mut Context<Self>) {
        if let Some((port_name, baud_rate)) = self.paused.take() {
            self.connect(port_name, baud_rate, cx);
        }
    }

    /// Tears down the open handle (if any) without touching `port_name` or
    /// `paused` -- shared by `disconnect` (which clears both after) and
    /// `pause_for_flash` (which needs `paused` to survive).
    fn close_port(&mut self, cx: &mut Context<Self>) {
        if let Some(open) = self.open.take() {
            cx.background_spawn(async move {
                if let Ok(mut guard) = open.handle.lock() {
                    *guard = None;
                }
            })
            .detach();
        }
        self.is_open = false;
        cx.notify();
    }

    fn push_line(&mut self, line: String, cx: &mut Context<Self>) {
        self.lines.push_back(line.clone());
        if self.lines.len() > MAX_BUFFERED_LINES {
            self.lines.pop_front();
        }
        cx.emit(SerialLineReceived(line));
        cx.notify();
    }
}

/// Runs on a background thread for the lifetime of one connection. Opens
/// the port, publishes the handle into `handle` for `send()` to use, then
/// loops reading with a short timeout (so it notices `handle` being cleared
/// to `None` by `close_port` within about 100ms) until the port closes or
/// errors.
async fn run_serial_reader(
    port_name: String,
    baud_rate: u32,
    handle: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
    message_tx: UnboundedSender<SerialConnectionMessage>,
) {
    let port = match serialport::new(&port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open()
    {
        Ok(port) => port,
        Err(error) => {
            message_tx
                .unbounded_send(SerialConnectionMessage::Error(error.to_string()))
                .ok();
            return;
        }
    };

    {
        let Ok(mut guard) = handle.lock() else { return };
        *guard = Some(port);
    }
    if message_tx
        .unbounded_send(SerialConnectionMessage::Opened)
        .is_err()
    {
        return;
    }

    let mut carry = Vec::new();
    let mut read_buf = [0u8; 1024];
    loop {
        let read_result = {
            let Ok(mut guard) = handle.lock() else { return };
            match guard.as_mut() {
                Some(port) => port.read(&mut read_buf),
                None => return, // closed by close_port
            }
        };

        match read_result {
            Ok(0) => continue,
            Ok(byte_count) => {
                for line in split_lines(&mut carry, &read_buf[..byte_count]) {
                    if message_tx
                        .unbounded_send(SerialConnectionMessage::Line(line))
                        .is_err()
                    {
                        return;
                    }
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(error) => {
                message_tx
                    .unbounded_send(SerialConnectionMessage::Error(error.to_string()))
                    .ok();
                return;
            }
        }
    }
}
```

- [ ] **Step 2:** Run: `cargo check -p citadel_serial_monitor`
Expected: success. Fix any small signature mismatches against the live `gpui`/`serialport`/`futures` APIs as they surface — this module's `cx.subscribe`/`cx.spawn`/`cx.background_spawn` closure signatures were checked against this codebase's existing conventions (`crates/citadel_build/src/board_detect.rs`'s `BoardMonitor`, `crates/terminal/src/terminal.rs`'s `events_tx`/`events_rx` pattern) but this is still the plan's most speculative single block of code.

- [ ] **Step 3:** Run: `cargo test -p citadel_serial_monitor`
Expected: `test result: ok. 14 passed; 0 failed;` (unchanged from Task 3 — this task adds no new pure-logic tests, per the note above).

- [ ] **Step 4:** Commit:

```bash
git add crates/citadel_serial_monitor/src/serial_connection.rs
git commit -m "$(cat <<'EOF'
Add the SerialConnection entity: connect, disconnect, send, pause/resume

The single owner of the process's one persistent serial port handle.
Subscribes to citadel_build's FlashStarted/FlashFinished events to
automatically release the port around a build & flash and reconnect
afterward, so the monitor and avrdude never fight over the same port.
EOF
)"
```

---

### Task 5: `SerialMonitorPanel` — dock panel UI

**Files:**
- Create: `crates/citadel_serial_monitor/src/serial_monitor_panel.rs`
- Modify: `crates/citadel_serial_monitor/src/citadel_serial_monitor.rs` (fill in `init`, actions, toast helpers)

**Interfaces:**
- Consumes: Task 4's `serial_connection::{SerialConnection, GlobalSerialConnection, SerialLineReceived, SerialConnectionError, default_port_name, DEFAULT_BAUD_RATE}`.
- Produces: `pub struct SerialMonitorPanel` (`impl workspace::dock::Panel`), `pub async fn load(workspace: WeakEntity<Workspace>, cx: AsyncWindowContext) -> anyhow::Result<Entity<Self>>`, action `citadel_serial_monitor::ToggleFocus`. Consumed by Task 7 (`crates/zed/src/zed.rs`'s `initialize_panels`).

- [ ] **Step 1:** Replace `crates/citadel_serial_monitor/src/citadel_serial_monitor.rs`'s contents with:

```rust
mod plot_parser;
pub mod serial_connection;
mod serial_monitor_panel;
mod serial_plotter_window;

use gpui::{App, Context, SharedString, actions};
use notifications::status_toast::StatusToast;
use serial_plotter_window::open_serial_plotter_window;
use ui::prelude::*;
use workspace::Workspace;

actions!(
    citadel_serial_monitor,
    [
        /// Toggles focus on the Serial Monitor dock panel.
        ToggleFocus,
        /// Opens the Serial Plotter floating window.
        OpenSerialPlotter
    ]
);

pub fn init(cx: &mut App) {
    serial_connection::init(cx);

    cx.observe_new(|workspace: &mut Workspace, _window, cx| {
        workspace.register_action(|_workspace, _: &OpenSerialPlotter, window, cx| {
            open_serial_plotter_window(window, cx);
        });
    })
    .detach();
}

/// Shows a dismissible error toast in `workspace`. Duplicated (not shared)
/// from `citadel_new_project::new_project`/`citadel_build`'s helper of the
/// same name and shape, per the established convention against cross-crate
/// UI coupling for a single small helper.
pub(crate) fn show_error_toast_in_workspace(
    workspace: &mut Workspace,
    message: impl Into<SharedString>,
    cx: &mut Context<Workspace>,
) {
    let toast = StatusToast::new(message, cx, |this, _| {
        this.icon(
            Icon::new(IconName::XCircle)
                .size(IconSize::Small)
                .color(Color::Error),
        )
        .dismiss_button(true)
    });
    workspace.toggle_status_toast(toast, cx);
}
```

Note `OpenSerialPlotter`'s handler takes `window` (needed by `cx.open_window` in Task 6) — `workspace.register_action`'s closure signature is `(workspace, action, window, cx)`, matching `citadel_build.rs`'s existing `BuildAndUpload` registration shape but using `window` this time instead of discarding it.

- [ ] **Step 2:** Write `crates/citadel_serial_monitor/src/serial_monitor_panel.rs`:

```rust
use crate::serial_connection::{
    DEFAULT_BAUD_RATE, GlobalSerialConnection, SerialConnection, SerialConnectionError,
    SerialLineReceived, default_port_name,
};
use crate::show_error_toast_in_workspace;
use editor::Editor;
use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, Pixels, Render,
    Subscription, WeakEntity, Window, actions, px, uniform_list,
};
use ui::prelude::*;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;

actions!(
    citadel_serial_monitor_panel,
    [
        /// Connects (or disconnects) the Serial Monitor's connection.
        ToggleConnection,
        /// Sends the send box's contents to the connected device.
        SendToDevice
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineEnding {
    None,
    Newline,
    CarriageReturn,
    CarriageReturnNewline,
}

impl LineEnding {
    fn as_bytes(self) -> &'static [u8] {
        match self {
            LineEnding::None => b"",
            LineEnding::Newline => b"\n",
            LineEnding::CarriageReturn => b"\r",
            LineEnding::CarriageReturnNewline => b"\r\n",
        }
    }

    fn label(self) -> &'static str {
        match self {
            LineEnding::None => "No line ending",
            LineEnding::Newline => "Newline",
            LineEnding::CarriageReturn => "Carriage return",
            LineEnding::CarriageReturnNewline => "Both NL & CR",
        }
    }

    fn next(self) -> Self {
        match self {
            LineEnding::None => LineEnding::Newline,
            LineEnding::Newline => LineEnding::CarriageReturn,
            LineEnding::CarriageReturn => LineEnding::CarriageReturnNewline,
            LineEnding::CarriageReturnNewline => LineEnding::None,
        }
    }
}

pub struct SerialMonitorPanel {
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    connection: Entity<SerialConnection>,
    port_editor: Entity<Editor>,
    baud_editor: Entity<Editor>,
    send_editor: Entity<Editor>,
    line_ending: LineEnding,
    position: DockPosition,
    _subscriptions: Vec<Subscription>,
}

impl SerialMonitorPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: gpui::AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            Self::new(workspace, window, cx)
        })
    }

    fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let workspace_handle = cx.entity().downgrade();

        cx.new(|cx| {
            // Invariant: citadel_serial_monitor::init(cx) (which sets this
            // global) runs during app startup, before any workspace (and
            // therefore any panel) is created -- see crates/zed/src/main.rs.
            let connection = cx.global::<GlobalSerialConnection>().0.clone();

            let default_port = default_port_name(cx).unwrap_or_default();
            let port_editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_text(default_port, window, cx);
                editor.set_placeholder_text("Port (e.g. /dev/ttyACM0)", window, cx);
                editor
            });
            let baud_editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_text(DEFAULT_BAUD_RATE.to_string(), window, cx);
                editor
            });
            let send_editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text("Send to device...", window, cx);
                editor
            });

            let mut subscriptions = Vec::new();
            subscriptions.push(cx.subscribe(&connection, |_this, _connection, _event: &SerialLineReceived, cx| {
                cx.notify();
            }));
            subscriptions.push(cx.subscribe_in(
                &connection,
                window,
                |this, _connection, event: &SerialConnectionError, _window, cx| {
                    let message = event.0.clone();
                    this.workspace
                        .update(cx, |workspace, cx| {
                            show_error_toast_in_workspace(workspace, message, cx);
                        })
                        .ok();
                    cx.notify();
                },
            ));

            Self {
                focus_handle: cx.focus_handle(),
                workspace: workspace_handle,
                connection,
                port_editor,
                baud_editor,
                send_editor,
                line_ending: LineEnding::None,
                position: DockPosition::Bottom,
                _subscriptions: subscriptions,
            }
        })
    }

    fn toggle_connection(&mut self, _: &ToggleConnection, _window: &mut Window, cx: &mut Context<Self>) {
        if self.connection.read(cx).is_open {
            self.connection.update(cx, |connection, cx| connection.disconnect(cx));
            return;
        }

        let port_name = self.port_editor.read(cx).text(cx).trim().to_string();
        if port_name.is_empty() {
            if let Some(workspace) = self.workspace.upgrade() {
                workspace.update(cx, |workspace, cx| {
                    show_error_toast_in_workspace(workspace, "Enter a port name first.", cx);
                });
            }
            return;
        }

        let baud_text = self.baud_editor.read(cx).text(cx);
        let Ok(baud_rate) = baud_text.trim().parse::<u32>() else {
            if let Some(workspace) = self.workspace.upgrade() {
                workspace.update(cx, |workspace, cx| {
                    show_error_toast_in_workspace(workspace, "Baud rate must be a number.", cx);
                });
            }
            return;
        };

        self.connection
            .update(cx, |connection, cx| connection.connect(port_name, baud_rate, cx));
    }

    fn send_to_device(&mut self, _: &SendToDevice, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.send_editor.read(cx).text(cx);
        let mut bytes = text.into_bytes();
        bytes.extend_from_slice(self.line_ending.as_bytes());
        self.connection.update(cx, |connection, cx| connection.send(bytes, cx));
        self.send_editor.update(cx, |editor, cx| editor.set_text("", window, cx));
    }

    fn cycle_line_ending(&mut self, cx: &mut Context<Self>) {
        self.line_ending = self.line_ending.next();
        cx.notify();
    }

    fn save_log(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let lines: Vec<String> = self.connection.read(cx).lines.iter().cloned().collect();
        let start_dir = std::env::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let receiver = cx.prompt_for_new_path(&start_dir, Some("serial-log.txt"));
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_this, cx| {
            let Ok(Ok(Some(path))) = receiver.await else { return };
            let contents = lines.join("\n");
            let write_result = cx
                .background_spawn(async move { std::fs::write(&path, contents) })
                .await;
            if let Err(error) = write_result {
                workspace
                    .update(cx, |workspace, cx| {
                        show_error_toast_in_workspace(workspace, error.to_string(), cx);
                    })
                    .ok();
            }
        })
        .detach();
    }
}

impl EventEmitter<PanelEvent> for SerialMonitorPanel {}

impl Focusable for SerialMonitorPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for SerialMonitorPanel {
    fn persistent_name() -> &'static str {
        "SerialMonitorPanel"
    }

    fn panel_key() -> &'static str {
        "SerialMonitorPanel"
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(
            position,
            DockPosition::Bottom | DockPosition::Left | DockPosition::Right
        )
    }

    fn set_position(&mut self, position: DockPosition, _window: &mut Window, cx: &mut Context<Self>) {
        self.position = position;
        cx.notify();
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(300.)
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<ui::IconName> {
        Some(ui::IconName::SignalHigh)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Serial Monitor")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(crate::ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        11
    }
}

impl Render for SerialMonitorPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_open = self.connection.read(cx).is_open;
        let line_count = self.connection.read(cx).lines.len();
        let connection = self.connection.clone();

        v_flex()
            .key_context("SerialMonitorPanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::toggle_connection))
            .on_action(cx.listener(Self::send_to_device))
            .size_full()
            .child(
                h_flex()
                    .gap_2()
                    .p_2()
                    .child(self.port_editor.clone())
                    .child(self.baud_editor.clone())
                    .child(Button::new("toggle-connection", if is_open { "Disconnect" } else { "Connect" })
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.toggle_connection(&ToggleConnection, window, cx);
                        })))
                    .child(Button::new("save-log", "Save Log")
                        .on_click(cx.listener(|this, _, window, cx| this.save_log(window, cx)))),
            )
            .child(
                uniform_list(
                    "serial-monitor-log",
                    line_count,
                    move |range, _window, cx| {
                        connection
                            .read(cx)
                            .lines
                            .iter()
                            .skip(range.start)
                            .take(range.end - range.start)
                            .map(|line| Label::new(line.clone()))
                            .collect()
                    },
                )
                .size_full(),
            )
            .child(
                h_flex()
                    .gap_2()
                    .p_2()
                    .child(self.send_editor.clone())
                    .child(
                        Button::new("line-ending", self.line_ending.label())
                            .on_click(cx.listener(|this, _, _window, cx| this.cycle_line_ending(cx))),
                    )
                    .child(Button::new("send", "Send").on_click(cx.listener(|this, _, window, cx| {
                        this.send_to_device(&SendToDevice, window, cx);
                    }))),
            )
    }
}
```

- [ ] **Step 3:** Run: `cargo check -p citadel_serial_monitor`
Expected: success after fixing any small mismatches. This panel is the plan's second-most speculative block (after Task 4): the exact `Editor::single_line`/`uniform_list`/`Panel` trait method signatures were checked against live source (`crates/editor/src/editor.rs`, `crates/gpui/src/elements/uniform_list.rs`, `crates/workspace/src/dock.rs`, `crates/git_ui/src/git_panel.rs`'s `impl Panel for GitPanel`), but small adjustments (e.g. `Editor::text(cx)` vs a different accessor name, exact `cx.prompt_for_new_path` error-shape unwrapping) should be expected and fixed here rather than treated as a plan error.

- [ ] **Step 4:** Commit:

```bash
git add crates/citadel_serial_monitor/src/citadel_serial_monitor.rs crates/citadel_serial_monitor/src/serial_monitor_panel.rs
git commit -m "$(cat <<'EOF'
Add SerialMonitorPanel: port/baud fields, log view, send box, save log

A single-pane workspace::dock::Panel (no tabs -- single-port scope, see
the design spec) showing SerialConnection's live text log, a send row
with a cyclable line-ending choice, and a Save Log button that writes
the buffered lines to a file the user picks.
EOF
)"
```

---

### Task 6: `SerialPlotterWindow` — floating window with a canvas-drawn chart

**Files:**
- Create: `crates/citadel_serial_monitor/src/serial_plotter_window.rs`
- Modify: `crates/citadel_serial_monitor/src/citadel_serial_monitor.rs` (already imports `open_serial_plotter_window` from Task 5's edit — no further change needed)

**Interfaces:**
- Consumes: Task 3's `plot_parser::{PlotPoint, parse_plot_line}`; Task 4's `serial_connection::{GlobalSerialConnection, SerialConnection, SerialLineReceived, SerialConnectionError, default_port_name, DEFAULT_BAUD_RATE}`.
- Produces: `pub fn open_serial_plotter_window(window: &mut Window, cx: &mut App)`. Consumed by Task 5's `OpenSerialPlotter` action handler (already wired).

Per the spec correction noted at the top of this plan: this window has no `Workspace` reference (it's an independent floating window, not workspace-scoped), so connection errors are shown as an inline banner in the window's own view rather than a `StatusToast`.

- [ ] **Step 1:** Write `crates/citadel_serial_monitor/src/serial_plotter_window.rs`:

```rust
use crate::plot_parser::parse_plot_line;
use crate::serial_connection::{
    DEFAULT_BAUD_RATE, GlobalSerialConnection, SerialConnection, SerialConnectionError,
    SerialLineReceived, default_port_name,
};
use editor::Editor;
use gpui::{
    App, Bounds, Context, Entity, FocusHandle, Focusable, Hsla, Pixels, Render, Size,
    Subscription, TitlebarOptions, Window, WindowBounds, WindowKind, WindowOptions, canvas, hsla,
    point, px,
};
use std::collections::VecDeque;
use ui::prelude::*;
use util::ResultExt;

const MAX_PLOT_POINTS: usize = 500;

#[derive(Clone)]
struct PlotSeries {
    label: String,
    color: Hsla,
    points: VecDeque<f32>,
}

fn series_color(index: usize) -> Hsla {
    const COLORS: [(f32, f32, f32); 5] = [
        (0.0, 0.7, 0.55),
        (0.33, 0.6, 0.45),
        (0.58, 0.7, 0.55),
        (0.13, 0.8, 0.55),
        (0.75, 0.6, 0.55),
    ];
    let (h, s, l) = COLORS[index % COLORS.len()];
    hsla(h, s, l, 1.0)
}

fn min_max(values: &[f32]) -> (f32, f32) {
    if values.is_empty() {
        return (0.0, 1.0);
    }
    let min = values.iter().copied().fold(f32::INFINITY, f32::min);
    let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    if (max - min).abs() < f32::EPSILON {
        (min - 1.0, max + 1.0)
    } else {
        (min, max)
    }
}

fn draw_series(series: &PlotSeries, bounds: Bounds<Pixels>, min_value: f32, max_value: f32, window: &mut Window) {
    if series.points.len() < 2 {
        return;
    }
    let range = (max_value - min_value).max(f32::EPSILON);
    let step_x = bounds.size.width.0 / (series.points.len() - 1).max(1) as f32;
    let mut builder = gpui::PathBuilder::stroke(px(2.));
    for (index, value) in series.points.iter().enumerate() {
        let x = bounds.origin.x.0 + index as f32 * step_x;
        let normalized = (value - min_value) / range;
        let y = bounds.origin.y.0 + bounds.size.height.0 * (1.0 - normalized);
        let point = point(px(x), px(y));
        if index == 0 {
            builder.move_to(point);
        } else {
            builder.line_to(point);
        }
    }
    if let Ok(path) = builder.build() {
        window.paint_path(path, series.color);
    }
}

pub struct SerialPlotterWindow {
    focus_handle: FocusHandle,
    connection: Entity<SerialConnection>,
    port_editor: Entity<Editor>,
    series: Vec<PlotSeries>,
    last_error: Option<String>,
    _subscriptions: Vec<Subscription>,
}

impl SerialPlotterWindow {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Invariant: citadel_serial_monitor::init(cx) runs at app startup,
        // before any window (and therefore this one) can be opened.
        let connection = cx.global::<GlobalSerialConnection>().0.clone();

        let default_port = default_port_name(cx).unwrap_or_default();
        let port_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(default_port.clone(), window, cx);
            editor.set_placeholder_text("Port (e.g. /dev/ttyACM0)", window, cx);
            editor
        });

        if !connection.read(cx).is_open && !default_port.is_empty() {
            connection.update(cx, |connection, cx| {
                connection.connect(default_port, DEFAULT_BAUD_RATE, cx)
            });
        }

        let mut subscriptions = Vec::new();
        subscriptions.push(cx.subscribe(&connection, |this, _connection, event: &SerialLineReceived, cx| {
            this.ingest_line(&event.0, cx);
        }));
        subscriptions.push(cx.subscribe(&connection, |this, _connection, event: &SerialConnectionError, cx| {
            this.last_error = Some(event.0.clone());
            cx.notify();
        }));

        Self {
            focus_handle: cx.focus_handle(),
            connection,
            port_editor,
            series: Vec::new(),
            last_error: None,
            _subscriptions: subscriptions,
        }
    }

    fn ingest_line(&mut self, line: &str, cx: &mut Context<Self>) {
        for point in parse_plot_line(line) {
            let series = match self.series.iter_mut().find(|series| series.label == point.label) {
                Some(series) => series,
                None => {
                    let color = series_color(self.series.len());
                    self.series.push(PlotSeries {
                        label: point.label.clone(),
                        color,
                        points: VecDeque::new(),
                    });
                    self.series.last_mut().expect("just pushed")
                }
            };
            series.points.push_back(point.value);
            if series.points.len() > MAX_PLOT_POINTS {
                series.points.pop_front();
            }
        }
        cx.notify();
    }

    fn reconnect(&mut self, cx: &mut Context<Self>) {
        let port_name = self.port_editor.read(cx).text(cx).trim().to_string();
        if port_name.is_empty() {
            return;
        }
        self.last_error = None;
        self.connection
            .update(cx, |connection, cx| connection.connect(port_name, DEFAULT_BAUD_RATE, cx));
    }

    fn render_legend(&self) -> impl IntoElement {
        v_flex().gap_1().p_2().children(self.series.iter().map(|series| {
            h_flex()
                .gap_2()
                .child(div().w(px(10.)).h(px(10.)).bg(series.color))
                .child(Label::new(series.label.clone()))
                .child(Label::new(
                    series
                        .points
                        .back()
                        .map(|value| value.to_string())
                        .unwrap_or_default(),
                ))
        }))
    }
}

impl Focusable for SerialPlotterWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SerialPlotterWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let series = self.series.clone();

        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .child(
                h_flex()
                    .gap_2()
                    .p_2()
                    .child(self.port_editor.clone())
                    .child(Button::new("reconnect", "Connect").on_click(cx.listener(|this, _, _window, cx| {
                        this.reconnect(cx);
                    }))),
            )
            .when_some(self.last_error.clone(), |this, error| {
                this.child(
                    div()
                        .p_2()
                        .bg(gpui::red())
                        .child(Label::new(error)),
                )
            })
            .child(
                h_flex()
                    .flex_1()
                    .child(
                        canvas(
                            move |bounds, _window, _cx| bounds,
                            move |bounds, _prepaint_bounds, window, _cx| {
                                let all_values: Vec<f32> =
                                    series.iter().flat_map(|s| s.points.iter().copied()).collect();
                                let (min_value, max_value) = min_max(&all_values);
                                for plot_series in &series {
                                    draw_series(plot_series, bounds, min_value, max_value, window);
                                }
                            },
                        )
                        .size_full(),
                    )
                    .child(self.render_legend()),
            )
    }
}

pub fn open_serial_plotter_window(window: &mut Window, cx: &mut App) {
    if let Some(existing) = window
        .app_windows(cx)
        .into_iter()
        .find_map(|handle| handle.downcast::<SerialPlotterWindow>())
    {
        existing
            .update(cx, |_, window, _cx| window.activate_window())
            .log_err();
        return;
    }

    let window_size = Size { width: px(640.), height: px(420.) };
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Serial Plotter".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            window_bounds: Some(WindowBounds::centered(window_size, cx)),
            is_resizable: true,
            is_minimizable: true,
            kind: WindowKind::Floating,
            ..Default::default()
        },
        |window, cx| {
            window.activate_window();
            cx.new(|cx| SerialPlotterWindow::new(window, cx))
        },
    )
    .log_err();
}
```

- [ ] **Step 2:** Run: `cargo check -p citadel_serial_monitor`
Expected: success after fixing small mismatches. Two spots most likely to need adjustment: whether window enumeration is `window.app_windows(cx)` or the App-level `cx.windows()` used by `crates/zed/src/zed.rs`'s `open_about_window` (that file's `open_about_window(cx: &mut App)` takes only `cx`, not `window`, and calls `cx.windows()` directly — since this task's handler is invoked from `workspace.register_action`'s `(workspace, action, window, cx)` closure and already has both, using plain `cx.windows()` exactly like `open_about_window` does is very likely simpler and correct; adjust to `cx.windows().into_iter().find_map(|w| w.downcast::<SerialPlotterWindow>())` if `window.app_windows` doesn't exist), and `gpui::red()` (a placeholder color call for the error banner — replace with any valid `Hsla`/`Rgba` constant this codebase's `gpui` version actually exports, e.g. `gpui::red()` or `Hsla { h: 0.0, s: 0.7, l: 0.5, a: 1.0 }`, if the exact helper name differs).

- [ ] **Step 3:** Commit:

```bash
git add crates/citadel_serial_monitor/src/serial_plotter_window.rs
git commit -m "$(cat <<'EOF'
Add SerialPlotterWindow: floating window with a canvas-drawn line chart

Parses received serial lines the same way the design's Arduino-Plotter
convention describes (label:value or bare comma/space/tab-separated
numbers), tracks up to 500 points per series in a ring buffer, and
draws them with GPUI's own canvas()/PathBuilder -- no charting crate
dependency added. Shares the same SerialConnection as the Monitor
panel so the two never open the port twice.
EOF
)"
```

---

### Task 7: Wire into `crates/zed`

**Files:**
- Modify: `crates/zed/Cargo.toml` (add `citadel_serial_monitor = { workspace = true }`)
- Modify: `crates/zed/src/main.rs` (add `citadel_serial_monitor::init(cx);`)
- Modify: `crates/zed/src/zed.rs` (add `SerialMonitorPanel` to `initialize_panels`)
- Modify: `crates/zed/src/zed/app_menus.rs` (Run menu: "Open Serial Plotter")

- [ ] **Step 1:** In `crates/zed/Cargo.toml`, find the line `citadel_build = { workspace = true }` (or equivalent) and add a line after it:

```toml
citadel_serial_monitor = { workspace = true }
```

- [ ] **Step 2:** In `crates/zed/src/main.rs`, find `citadel_build::init(cx);` and add a line right after it (must run after, so `citadel_serial_monitor::init`'s `board_monitor` subscription setup can find `citadel_build`'s `GlobalBoardMonitor` already registered):

```rust
citadel_build::init(cx);
citadel_serial_monitor::init(cx);
```

- [ ] **Step 3:** In `crates/zed/src/zed.rs`'s `initialize_panels` function, add the import at the top of the file (alongside other panel imports) and wire the panel in following the exact `add_panel_when_ready` pattern already used for `terminal_panel`/`git_panel`:

Find:
```rust
        let terminal_panel = TerminalPanel::load(workspace_handle.clone(), cx.clone());
```

Add right after it:
```rust
        let serial_monitor_panel =
            citadel_serial_monitor::serial_monitor_panel::SerialMonitorPanel::load(
                workspace_handle.clone(),
                cx.clone(),
            );
```

Then find the `futures::join!` call and add the new panel to it:
```rust
        futures::join!(
            add_panel_when_ready(project_panel, workspace_handle.clone(), cx.clone()),
            add_panel_when_ready(outline_panel, workspace_handle.clone(), cx.clone()),
            add_panel_when_ready(terminal_panel, workspace_handle.clone(), cx.clone()),
            add_panel_when_ready(serial_monitor_panel, workspace_handle.clone(), cx.clone()),
            add_panel_when_ready(git_panel, workspace_handle.clone(), cx.clone()),
            add_panel_when_ready(channels_panel, workspace_handle.clone(), cx.clone()),
            add_panel_when_ready(debug_panel, workspace_handle.clone(), cx.clone()),
            initialize_agent_panel(workspace_handle, cx.clone()).map(|r| r.log_err()),
        );
```

`SerialMonitorPanel::load` is `pub`, but `serial_monitor_panel` itself is a private (`mod`, not `pub mod`) module in `citadel_serial_monitor.rs` (Task 5, Step 1) — change that one line to `pub mod serial_monitor_panel;` so `crates/zed` can reach `SerialMonitorPanel` and its `load` function.

- [ ] **Step 4:** In `crates/citadel_serial_monitor/src/citadel_serial_monitor.rs`, change:
```rust
mod serial_monitor_panel;
```
to:
```rust
pub mod serial_monitor_panel;
```

- [ ] **Step 5:** In `crates/zed/src/zed/app_menus.rs`, find the Run menu's `MenuItem::action("Build and Upload", citadel_build::BuildAndUpload)` entry (added by the prior board-detect/build/flash plan) and add a line right after it:

```rust
MenuItem::action("Build and Upload", citadel_build::BuildAndUpload),
MenuItem::action("Open Serial Plotter", citadel_serial_monitor::OpenSerialPlotter),
```

- [ ] **Step 6:** Run: `cargo check -p zed`
Expected: success. Fix any import-path or signature issues surfaced here — this is the integration point where Task 5/6's more speculative code first gets compiled against the real `crates/zed` binary target.

- [ ] **Step 7:** Commit:

```bash
git add crates/zed/Cargo.toml crates/zed/src/main.rs crates/zed/src/zed.rs crates/zed/src/zed/app_menus.rs crates/citadel_serial_monitor/src/citadel_serial_monitor.rs
git commit -m "Wire citadel_serial_monitor into the zed binary"
```

---

### Task 8: Manual hardware verification (human required)

Nothing in Tasks 1–7 exercises a real serial port. Do not consider this plan complete until a human confirms, using the ELEGOO UNO R3 (or whatever board is on hand) and a test sketch:

- [ ] **Step 1:** Build a small test sketch (or reuse the existing `/home/gooya/test/` project from the board-detect/build/flash feature's hardware verification) whose `rust/` logic both writes plain log lines and writes comma-separated numeric telemetry, e.g. alternating `Serial.println("tick")`-style plain lines with `temp:23.5,humidity:60`-style lines. `cpp/io.cpp` stays a direct pass-through per this repo's Rust/C boundary rules (CLAUDE.md) — the actual `Serial.print`/`Serial.println` calls and any values/timing/formatting logic belong in the `rust/` crate via `extern "C"` calls into thin C wrappers, not as literal C string formatting.
- [ ] **Step 2:** `cargo run -p zed`; open the test project. Confirm the Serial Monitor panel appears in the bottom dock (via its toggle action / command palette) with a port field pre-filled to the currently detected board's port (if a board is connected and already identified via the existing board-detect feature), and an editable baud field.
- [ ] **Step 3:** Click Connect. Confirm: the log view starts showing received lines live; disconnecting and reconnecting works; entering a bad/nonexistent port name shows an error toast rather than crashing.
- [ ] **Step 4:** Type a message into the send box, pick a line ending, click Send — confirm it actually reaches the device (e.g. with an echo-style test sketch, or by observing an LED/behavior change gated on received input).
- [ ] **Step 5:** Click Save Log, pick a destination — confirm a plain text file with the buffered lines is written there.
- [ ] **Step 6:** Open the Serial Plotter (Run menu → "Open Serial Plotter", or the command palette). Confirm it also shows the port pre-filled and connects; confirm the numeric telemetry lines produce a moving multi-series line chart with a legend showing each series' current value; confirm plain non-numeric log lines are silently ignored by the plotter (no crash, no garbage points) while still showing normally in the Monitor panel's log.
- [ ] **Step 7:** With the Monitor panel connected and showing live data, click **Build and Upload**. Confirm: the Monitor panel automatically shows a disconnected state during the flash (so avrdude doesn't fail to claim the port), and automatically reconnects and resumes showing live data once the flash finishes — without the user touching the Monitor panel at all.
- [ ] **Step 8:** If any step needed a code fix, make it, re-verify with a fresh `cargo run -p zed`, and commit the fix separately describing the hardware behavior it corrects.

---

## Verification (end to end)

1. `cargo test -p citadel_serial_monitor` passes throughout (14 tests by Task 3: line-splitting, plot-line parsing) — no hardware needed for either.
2. `cargo test -p citadel_build` still passes at 39/39 after Task 1's event-emission addition.
3. `cargo check -p zed` (Task 7) confirms the whole editor still builds with the new crate wired in.
4. `./script/clippy` (per this repo's `CLAUDE.md` build guideline) on the new crate and the modified `citadel_build`/`zed` files before considering any task's commit final.
5. Task 8's manual hardware pass is the actual feature verification — automated tests intentionally stop at pure parsing logic, matching the design spec's own testing section and the precedent the board-detect/build/flash plan set for hardware-dependent verification.

## Critical files

- `docs/superpowers/specs/2026-08-10-serial-monitor-plotter-design.md` (source of truth for scope; updated during this planning pass to fix the dependency-direction and port-selector gaps noted above)
- `crates/citadel_build/src/board_detect.rs` (existing `BoardMonitor`/event patterns to mirror; gains `FlashStarted`/`FlashFinished`)
- `crates/terminal/src/terminal.rs` (the `events_tx`/`events_rx`-fed-by-a-background-task shape `SerialConnection`'s reader loop mirrors)
- `crates/zed/src/zed.rs` (`initialize_panels`, `open_about_window` — the panel-registration and floating-window patterns this plan reuses)
- New: `crates/citadel_serial_monitor/src/{citadel_serial_monitor,serial_connection,plot_parser,serial_monitor_panel,serial_plotter_window}.rs`

## After approval

Save this plan to `docs/superpowers/plans/2026-08-10-serial-monitor-plotter.md` (done), commit it, and execute task-by-task via the `superpowers:subagent-driven-development` skill, on the `serial-monitor-plotter` branch (already created off the now-merged `main`).
