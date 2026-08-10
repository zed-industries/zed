# Design: Serial Monitor and Serial Plotter

Status: draft, pending user review before an implementation plan is written.

## Motivation

Citadel already gives users board auto-detection and one-button build & flash (`docs/superpowers/specs/2026-08-09-board-detect-build-flash-design.md`). An embedded-focused IDE that stops there is missing table-stakes functionality: seeing what the running sketch actually prints over serial, sending input back to it, and visualizing numeric telemetry in real time. Arduino IDE calls these the Serial Monitor and Serial Plotter; this design adds equivalents to Citadel.

## Scope

In scope:
- Serial Monitor: live text display of received bytes, a send box (with selectable line ending) for writing back to the device, and saving/exporting the received log to a plain text file.
- Serial Plotter: a separate OS window with a real-time line chart, parsing received lines as numeric values (Arduino Plotter-style `label:value` or bare comma/space/tab-separated numbers).
- Baud rate: a sensible default, changeable from the Monitor panel's UI.
- Coordination with `citadel_build`'s flash pipeline: the Monitor's connection is automatically paused (port closed) immediately before `avrdude` runs and resumed (port reopened) immediately after, so build & flash and the monitor never fight over the same port.

Out of scope (recorded so it stays a decision, not an omission):
- Multiple simultaneous port tabs. `board_detect`'s current model is "one detected board at a time"; the Monitor and Plotter follow that and target a single port. Multi-port tabbed UI (mirroring `TerminalPanel`'s `Pane`/tab-bar machinery) is a natural future extension once multi-board detection itself is in scope.
- Persisting Monitor/Plotter settings (baud rate, line-ending choice) across restarts. Session-only state; every launch starts from the default baud rate.
- A polished legend-editing UI for the Plotter beyond showing each parsed series' label and current value.

## Architecture

New crate `crates/citadel_serial_monitor/`, following the `citadel_build`/`citadel_new_project` naming and structure convention. It is a separate crate from `citadel_build` rather than a new module there because it introduces a genuinely new logical component — a persistent, always-on serial connection and two new UI surfaces (a dock panel and a floating window) — where `citadel_build`'s existing code is all transient, request/response subprocess orchestration (`avrdude` invoked once per build, never held open).

```
crates/citadel_serial_monitor/
├── Cargo.toml
└── src/
    ├── citadel_serial_monitor.rs  # crate root: actions!, init(), SerialConnection entity + GlobalSerialConnection
    ├── serial_monitor_panel.rs    # Panel impl: text log, baud selector, send box
    └── serial_plotter_window.rs   # floating window: canvas-drawn line chart, parses the same line stream
```

`citadel_serial_monitor` gains a dependency on `citadel_build` (not the reverse — `citadel_build` never references `citadel_serial_monitor`, avoiding a circular crate dependency). `citadel_build` gains two small GPUI events, `FlashStarted`/`FlashFinished`, emitted from its existing `BoardMonitor` entity immediately before and after its existing `avrdude` call in `citadel_build.rs`'s `start_build_and_upload`; `build_pipeline.rs` (the pure command-execution layer) is untouched. `SerialConnection` subscribes to those two events itself (using the same `citadel_build::board_detect::GlobalBoardMonitor` dependency it already needs for the default-port lookup below) and calls its own internal pause/resume in response — `citadel_build` has no knowledge that a serial monitor exists.

### `SerialConnection` (GPUI entity, wrapped in `GlobalSerialConnection`)

The single owner of the actual OS serial port handle — there is exactly one persistent connection in the process, shared by both the Monitor panel and the Plotter window, so they never try to open the same port twice. Fields: `port_name: Option<String>`, `baud_rate: u32`, `is_open: bool`, `lines: VecDeque<String>` (capped at 1000 entries, oldest dropped first). This buffer is redisplay state for the Monitor panel, not a durable log — "Save Log" writes whatever is currently in the buffer (up to the last 1000 lines), so a session that has produced more than 1000 lines since connecting will only export the most recent 1000. This matches the buffer's stated purpose (bounding memory use) rather than promising a complete session transcript; a user who needs the full history should save before the buffer would roll over, or this limit can be revisited later if it proves too small in practice.

Public API: `connect(port_name, baud, cx)`, `disconnect(cx)`, `send(bytes, cx)`. Pausing/resuming around a flash is internal (triggered by the `FlashStarted`/`FlashFinished` subscription above), not called externally.

Connecting spawns a `cx.background_spawn` task that opens the port via `serialport::new(...).open()`, then loops: blocking-read available bytes, split on line boundaries (carrying a partial-line buffer across reads for lines split across two reads), and for each complete line send it back to the foreground via an unbounded channel — the same `events_tx`/`events_rx`-fed-by-a-background-task shape `terminal.rs` and `board_detect.rs`'s poll loop already use in this codebase, just without alacritty's ANSI/vte grid (there is nothing to reuse from `Terminal` itself; see the "Corrections/precedent notes" below). The foreground task receiving from that channel calls `cx.emit(SerialLineReceived(String))` per line, or `cx.emit(SerialConnectionError(String))` on a read error (e.g. device unplugged), which also flips `is_open` to `false`.

`SerialConnection` never connects on its own initiative. It does not watch `board_detect::GlobalBoardMonitor` and auto-connect when a board is detected — the user must explicitly open the Monitor panel or the Plotter window to start a connection. (Auto-connecting to whatever port the board detector currently sees would silently start reading from a device the user didn't ask to read from.) When the user does open the Monitor panel or Plotter window with no port chosen yet, it defaults the port field to `GlobalBoardMonitor`'s currently `detected` board's port name, if any — a default, not an automatic action.

### `SerialMonitorPanel` (implements `workspace::dock::Panel`)

Registered the same way `TerminalPanel` and `citadel_build`'s own status item register themselves: `citadel_serial_monitor::init(cx)` calls `cx.observe_new::<Workspace>(...)` to register the panel's toggle action, and `crates/zed/src/zed.rs`'s `initialize_panels` gains one line adding `SerialMonitorPanel::load(...)` alongside `TerminalPanel`/`ProjectPanel`/etc., docked at the bottom by default.

Unlike `TerminalPanel`, `SerialMonitorPanel` does not own a `Pane`/`PaneGroup` — with a single target port there is nothing to tab between, so it is one `Panel` directly rendering:
- A header row: a port field (defaults to the currently detected board's port via `GlobalBoardMonitor`, if any, but always editable — without this, the panel would be unusable whenever no board is auto-detected), a baud-rate field (default 9600, common Arduino default), a Connect/Disconnect button, and a "Save Log" button.
- A scrolling, virtualized text view of `SerialConnection.lines`, appended to as `SerialLineReceived` events arrive (`cx.subscribe(&connection, ...)` → append → `cx.notify()`).
- A footer send row: a single-line text input, a line-ending dropdown (default None, then `\n` / `\r` / `\r\n`), and a Send button that calls `SerialConnection::send`.

"Save Log" opens the standard save-file prompt (reusing the same workspace file-dialog path `citadel_new_project` already uses for its own prompts) and writes the buffered lines as plain text.

### `SerialPlotterWindow`

Opened via an action, following the exact pattern already in this codebase at `crates/zed/src/zed.rs`'s `open_about_window`: check `cx.windows()` for an existing window downcasting to `SerialPlotterWindow` and `activate_window()` it if found, otherwise `cx.open_window(WindowOptions { kind: WindowKind::Floating, is_resizable: true, .. }, |window, cx| cx.new(SerialPlotterWindow::new))`.

Subscribes to the same `SerialConnection` global and the same `SerialLineReceived` events the Monitor panel does. If no connection is open yet when the window opens, it connects using the same "default to the currently detected board's port, if any" rule as the Monitor panel.

Each received line is tokenized on `,`, tab, or space. A token matching `<label>:<number>` contributes to a named series; a bare `<number>` token contributes to an auto-named series (`value1`, `value2`, ...) by position, matching the Arduino Plotter's own convention. Unparseable tokens are silently dropped — the Monitor panel remains the place to see the raw line if a parse is unexpectedly empty.

Each series keeps a ring buffer of the most recent 500 points. Rendering uses GPUI's existing `gpui::canvas()` element and `Window::paint_path` (`crates/gpui/src/window.rs:4044`) to draw each series as a polyline in a distinct color, with a small legend (label + latest value) alongside. No charting crate is added — nothing in the workspace's existing dependencies or `crates/gpui` provides a chart widget, but a multi-series line plot over a fixed-size ring buffer is well within what `canvas()`/`paint_path` already do for other custom-drawn UI in this codebase (e.g. `mermaid_render`), so a small dedicated element is the right amount of code rather than a new dependency.

### Build & flash coordination

In `citadel_build.rs`'s `start_build_and_upload`, immediately before calling `build_pipeline::build_and_flash(target)`, check `GlobalSerialConnection`: if it is open and its `port_name` matches `target.port_name`, call `pause_for_flash(cx)` (closes the port). After `build_and_flash` returns (success or failure), if it was paused for this flash, call `resume_after_flash(cx)` (reopens at the same baud rate). If the connection is closed or open on a different port, neither call happens — no behavior change for that case.

## Error handling

- Port open failure (already held by another process, permission denied, device gone): shown as a toast; the panel/window's connection state stays `Disconnected` with the port/baud fields still editable and a Connect button to retry — it does not silently keep retrying in a loop.
- Read error mid-connection (e.g. device unplugged while connected): `SerialConnectionError` event → toast naming the error, `is_open` flips to `false`, both the Monitor panel and the Plotter window reflect the disconnected state (shared global state, single source of truth).
- Plotter parse failures: silently skipped per-token, per the Scope section above — this is expected and routine (e.g. a sketch's ordinary log lines interleaved with its numeric telemetry lines), not an error condition worth surfacing.
- Build & flash coordination: if `pause_for_flash` itself fails to close the port cleanly, `build_and_flash` still proceeds — `avrdude` opening a port already closed on the Rust side either succeeds (the common case) or fails with its own toast from the existing build-error path; this design does not add a new failure mode here beyond what `citadel_build` already surfaces for flash failures.

## Testing

- Line-splitting (bytes arriving in arbitrary chunks, including a line split across two reads) and the Plotter's token parser (`label:value`, bare numbers, mixed/invalid tokens) are pure functions, unit-tested without hardware.
- `SerialConnection`'s state transitions (`connect`/`disconnect`/`pause_for_flash`/`resume_after_flash`, including pausing when not connected and resuming when nothing was paused) are unit-tested against the entity's state, not real I/O.
- End-to-end: manual verification on real hardware (ELEGOO UNO R3), using a test sketch that both `Serial.println`s plain text and prints comma-separated numeric values, confirming: Monitor shows both correctly, Send actually reaches the device (e.g. an echo sketch), Plotter draws a moving line for the numeric values, and a Build and Upload while the Monitor is open disconnects/reconnects without the user having to do anything — matching the precedent `citadel_build`'s own Task 10 set for hardware-dependent verification that automated tests can't cover.
