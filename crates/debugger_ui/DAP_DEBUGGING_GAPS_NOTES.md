# DAP Debugging — Gaps & Fix Notes

Working notes for `feat/debugging-improvements`. Purpose: track concrete gaps
in Zed's built-in DAP debugger (`dap`, `dap_adapters`, `debug_adapter_extension`,
`debugger_tools`, `debugger_ui`), reproduce them, find root causes, then fix.

Not a design doc — update freely as we learn more. Delete sections once fixed
and folded into a commit; keep only what's still open.

## How to use this file

- One entry per gap under **Open gaps**.
- Move an entry to **Resolved** with the commit hash once fixed, instead of
  deleting it — keeps a record of what was actually wrong.
- Keep repro steps concrete (adapter, language, exact values expected vs seen)
  so gaps are re-testable after each fix.

## Open gaps

### 1. Variable/watch values unreliable — large gap between expected and shown data

- **Symptom:** debug values are not reliable; a "huge gap" between what the
  debuggee actually has and what the debugger UI shows.
- **Adapter/language:** JavaScript/Node — Next.js 16.3.6 dev server with
  Turbopack (`next dev`).
- **Repro steps:**
  1. Set a breakpoint in a React server/client component that destructures a
     prop, e.g. `{profile?.status ?? ""}` in `Example.tsx`.
  2. Hit the breakpoint via `next dev` (Turbopack).
  3. Open Variables (Local scope) and/or evaluate the destructured prop name
     in Console.
- **Expected vs actual:** expected — Local scope shows `profile` (and other
  in-scope component locals/props) with real values; Console evaluate of
  `profile`/`data` returns those same values. Actual — Local scope shows only
  `_` (bound to `fakeJSXCallSite`, a React dev-mode internal for JSX
  call-site/owner-stack tracking) and `this = undefined`; none of the
  component's real locals are present. Console evaluate of `profile` returns
  Node's built-in `ƒ profile()` REPL/CPU-profiler function (shadowing the
  absent local), and `data` throws `ReferenceError: data is not defined`.
- **Root cause (likely):** the debugger is paused inside a **different frame
  than the UI claims** — `fakeJSXCallSite`, a React/Turbopack dev
  instrumentation wrapper, not the authored `Example` render body — while
  the UI still labels it "Local: Example" and highlights the authored
  source line (`Example.tsx:19`) as if the mapping were correct. Matches
  source-map failures seen in the same session's terminal output:
  ```
  Could not read source map for .../hmr-client.ts: Unexpected end of JSON input
  Could not read source map for .../react-server-dom-turbopack-client.node.development.js: ENOENT ...
  ```
  i.e. broken/unreadable source maps for some Turbopack-generated dev files
  cause a frame-to-source-position mismatch: the real paused frame and the
  frame the UI displays disagree. Variables/Console aren't lying about the
  actual frame's scope (`fakeJSXCallSite` genuinely has no `profile`/`data`);
  the bug is Zed reporting the wrong source location for that frame.
- **Confirmed:** same breakpoint (`Example.tsx:19`), Turbopack disabled
  (plain `next dev` / Webpack) — evaluating `data` now correctly returns the
  real value (a 5-item array of records), where it previously threw
  `ReferenceError: data is not defined` under Turbopack. Same breakpoint,
  only variable changed was the bundler → **root cause confirmed as
  Turbopack-specific broken/unreadable source maps**, not a general
  `crates/dap`/`crates/debugger_ui` frame-resolution bug.
- **Suspected area:** none in Zed's own frame/scope handling — this appears
  to be an upstream Next.js/Turbopack dev-mode source-map generation issue
  (some generated files' `.map`s are missing/malformed, per the `ENOENT`/
  `Unexpected end of JSON input` errors), which js-debug then can't use to
  resolve the real authored frame. Possible Zed-side mitigation: detect
  unreadable/missing source maps for the active frame and surface a warning
  in the UI (e.g. "source map unavailable — showing generated code frame")
  instead of silently mislabeling the frame as the authored location.
- **Status:** root cause confirmed (Turbopack source-map breakage). Not
  fixable at the Next.js/Turbopack end from here; only actionable Zed-side
  improvement is the "warn when source map is missing/broken" mitigation
  above. Low priority relative to gap #2 unless the mitigation turns out to
  be cheap.

### 2. Child-session focus thrashes on npm lifecycle scripts (JS/Node)

- **Symptom:** running an npm-script-based debug target (e.g. `npm start`
  where `prestart` runs `npm run clean && npm run build`) produces multiple
  short-lived debug sessions, all labeled after the real entrypoint (e.g. two
  tabs both named `bin.mjs [<pid>]`, one struck-through/terminated). Frames,
  Console, and Variables panes stay empty; terminal shows repeated
  `Debugger attached.` / `Waiting for the debugger to disconnect...`.
- **Adapter/language:** JavaScript/Node (`crates/dap_adapters/src/javascript.rs`),
  npm-script launch target.
- **Repro steps:**
  1. Debug-launch an npm script whose `prestart` (or similar lifecycle hook)
     runs other Node-based commands (`rimraf`, `tsc`, a bundler), with a
     config that auto-attaches child processes.
  2. Observe the session tab strip and Frames/Console/Variables while the
     lifecycle scripts run, before the actual entrypoint starts.
- **Expected vs actual:** expected — only the actual app entrypoint
  (`bin.mjs`) gets a debug session; lifecycle-script child processes either
  aren't attached to, or are attached to but not given UI focus. Actual — every
  Node child process npm spawns (including throwaway ones like `rimraf`) gets
  its own session, and the panel auto-focuses each new one as it spawns.
- **Root cause (found):** `handle_start_debugging_request` in
  `crates/debugger_ui/src/debugger_panel.rs:438-483` creates a child session
  for every `startDebugging` reverse-request from the JS adapter. Focus is
  decided by:
  ```rust
  // Focus child sessions if the parent has never emitted a stopped event;
  // this improves our JavaScript experience, as it always spawns a "main"
  // session that then spawns subsessions.
  let parent_ever_stopped = parent_session.update(cx, |this, _| this.has_ever_stopped());
  Self::register_session(this, session, !parent_ever_stopped, cx).await?;
  ```
  This heuristic can't distinguish "a real target process that just hasn't
  hit a breakpoint yet" from "a throwaway lifecycle-script process that will
  never hit one" — so on npm-script-heavy launches, focus keeps jumping to
  empty short-lived sessions instead of settling on the real one.
- **Possible directions:** don't auto-focus a child session until it actually
  stops/has frames (or a timeout elapses with no activity); or let the adapter
  hint which child is "the real one" (e.g. via a naming/role convention from
  `startDebugging` args) instead of relying purely on `parent_ever_stopped`.
- **Status:** root cause identified, fix not yet started. Next.js repro
  planned (dev server spawns its own worker/child processes — good stress
  test for the same focus logic, likely surfaces more edge cases than a
  single-script npm lifecycle).
- **Next.js repro update — likely more severe than originally scoped:**
  during HMR testing (`next dev`, Turbopack), observed two session tabs both
  labeled `start-server.js [7360]` open simultaneously (same PID, same
  label). Separately, **Continue no longer makes forward progress at all**:
  clicking Continue re-hits a breakpoint immediately, repeatedly, regardless
  of which breakpoint is set or which file — it lands on the breakpoint and
  loops over and over. Working theory: this is the same child-session/focus churn
  as above, but now affecting **Continue routing**, not just which session
  gets UI focus — if a duplicate/child session re-attaches and re-verifies
  breakpoints on every continue (e.g. triggered by Turbopack's HMR/recompile
  cycle spawning a fresh `startDebugging` child), the resulting fresh
  `stopped` event would look indistinguishable from "the same breakpoint
  firing again," while the actual Continue request may be going to a
  session that isn't the one shown in the UI. Needs a focused repro
  (isolate: does this happen without any file edits/HMR involved, or only
  after at least one HMR recompile?) before treating as confirmed root
  cause.
- **Repro TODO:** (1) hit a breakpoint with a completely static file, no HMR
  edits — does Continue work normally? (2) if yes, edit the file once to
  trigger one HMR recompile, hit the breakpoint again, try Continue — does
  it break at that point? This isolates whether HMR-triggered child-session
  churn is the actual trigger.

### 3. Console evaluate results aren't expandable

- **Symptom:** evaluating an expression (object/array) in the Console prints
  its string form but can't be expanded/inspected inline — you have to
  switch to the Variables tab to browse the same value's structure.
- **Adapter/language:** general (not adapter-specific — it's how the Console
  is implemented).
- **Root cause (found):** the Console is a plain text buffer
  (`crates/debugger_ui/src/session/running/console.rs:157-248`,
  `add_messages`) — each `OutputEvent.output` string is inserted into an
  editor-style buffer with ANSI highlighting, nothing else. The session layer
  already populates `OutputEvent.variables_reference` when an evaluate
  succeeds (`crates/project/src/debugger/session.rs:2830-2836`), but
  `console.rs` never reads `variables_reference` anywhere — grepped for it,
  zero matches. The DAP data needed to expand the result is available and
  simply unused by the Console UI.
- **Expected:** like VS Code's Debug Console, an evaluated object/array
  result should be an expandable tree node (using its
  `variables_reference`), not dead text.
- **Suspected area:** `crates/debugger_ui/src/session/running/console.rs` —
  needs a structured message type (text vs. expandable-result) instead of
  flattening everything through the plain-text buffer; could likely reuse
  `variable_list.rs`'s existing tree-rendering for `variablesReference`
  values.
- **Status:** root cause identified, fix not started.

### 4. Console completion menu doesn't respond to arrow keys / Enter

- **Symptom:** typing in the Console query bar to trigger completions, then
  pressing arrow keys or Enter, doesn't navigate/accept the completion list —
  you have to type the full variable name yourself.
- **Adapter/language:** general (Console input widget).
- **Root cause (found):** `crates/debugger_ui/src/session/running/console.rs`
  wires a real `CompletionProvider` onto the query bar's nested `Editor`
  (`console.rs:93-100`, `ConsoleQueryBarCompletionProvider`), but the outer
  `Console` container (`key_context("DebugConsole")`) also binds the same
  keys the completion menu needs:
  ```rust
  // console.rs:455 (Console::render)
  .on_action(cx.listener(Self::evaluate))       // bound to Confirm
  .on_action(cx.listener(Self::watch_expression))
  ...
  // console.rs:464-465 (query bar row)
  .on_action(cx.listener(Self::previous_query)) // bound to SelectPrevious
  .on_action(cx.listener(Self::next_query))     // bound to SelectNext
  ```
  These are registered one level up from the actual completion-menu-owning
  Editor. When a completion popup is open, Confirm/SelectPrevious/SelectNext
  should be consumed by the Editor's own completion-accept/navigate handling
  first; instead the console-level history-navigation and evaluate-submit
  handlers appear to win, so arrow keys move through console history (not
  the completion list) and Enter submits the typed text as-is (not the
  selected completion).
- **Expected:** when the completion popup is visible, arrow keys should move
  the completion selection and Enter should accept it — console
  history/evaluate should only take those keys when no popup is showing.
- **Suspected area:** keybinding/context precedence between
  `key_context("DebugConsole")` (console.rs:454) and the nested Editor's own
  completion-menu context; likely needs the console-level bindings gated to
  not fire while the query bar's completion menu is open (mirroring how the
  main code editor's own history/nav actions defer to its completion menu).
- **Status:** root cause hypothesis identified (not yet stepped through at
  runtime), fix not started.

### 5. Hovering code during a debug session shows LSP type info, not the runtime value

- **Symptom:** hovering a variable in the editor while paused at a
  breakpoint shows the same hover as when not debugging (type/interface
  signature from the LSP) instead of (or in addition to) the variable's
  actual current runtime value.
- **Note:** not yet directly confirmed by hovering (see gap #7 — what was
  actually tested turned out to be the always-on inline-value decoration,
  not a hover popup). Still worth testing directly: hover `profile` while
  paused and see whether the popup is LSP-only or includes a runtime value.
- **Adapter/language:** general (editor hover, not adapter-specific).
- **Root cause (found):** there is no debug-aware hover integration in the
  editor at all — grepped `crates/editor/src` for any debug/DAP hover hook
  and found nothing. Zed does have inline-value infrastructure
  (`crates/dap/src/inline_value.rs`, wired into
  `crates/editor/src/inlays/inlay_hints.rs`) that already shows runtime
  values as inline annotations next to code while paused, but that data path
  isn't reused by the hover popover — hover always falls through to plain
  LSP hover.
- **Expected:** like VS Code, hovering a variable while paused/stopped in an
  active debug session should show (or prioritize) its current runtime
  value, using the same data the inline-value feature already has.
- **Suspected area:** editor hover popover construction (wherever LSP hover
  results are assembled) needs a debug-session-aware source added ahead of
  or alongside LSP hover, reusing `crates/dap/src/inline_value.rs`'s value
  resolution.
- **Status:** DEPRIORITIZED. Feasible (found the exact mechanism —
  `SemanticsProvider::inline_values` already resolves debug values via
  `project.active_debug_session(cx)`, `show_hover` in
  `crates/editor/src/hover_popover.rs:271` could call it alongside
  `provider.hover(...)`), but the inline decoration (gap #7) already shows
  the value on the line without hovering — same payoff, and #7's fix is
  scoped to debugger-specific rendering instead of the shared hover path
  used by every file/language. Revisit only if #7 turns out insufficient.
  Concerns noted before deferring: hover_popover.rs is high-traffic/shared
  code (regression risk for all hover, not just debug), adds async work to
  every hover unless tightly gated, and position-matching an `InlayHint`'s
  range to "the exact token under the mouse" wasn't verified as precise.

### 6. Attach process picker lists every OS process, not just debuggable ones

- **Symptom:** opening Attach → "Select the process you want to attach the
  debugger to" lists the entire Windows process table (browser helpers,
  system services, unrelated desktop apps, ...), alphabetically sorted, with
  the JavaScript adapter selected in the
  bottom-right dropdown — none of those processes could ever be valid attach
  targets for that adapter.
- **Adapter/language:** general (Attach flow, not adapter-specific — same for
  any adapter).
- **Root cause (found):** `get_processes_for_project` in
  `crates/debugger_ui/src/attach_modal.rs:363-417` builds the candidate list
  from `sysinfo::System::new_with_specifics(...).processes()` with **no
  filtering at all** — every process on the machine becomes a `Candidate`,
  sorted by name. The adapter-type dropdown shown in the modal plays no role
  in narrowing this list; it's purely a fuzzy-search-everything picker.
- **Expected:** like VS Code/js-debug, the process list should be filtered to
  targets the selected adapter could actually attach to — for JavaScript,
  Node processes (by executable name `node`/`node.exe`, and ideally only
  ones with an open inspector port); similarly for other adapters (Python →
  `python`/`python3`, etc.).
- **Suspected area:** `get_processes_for_project` /
  `attach_modal.rs` — needs adapter-aware filtering before building
  `Candidate`s, likely keyed off the adapter's executable name convention or
  (better, but more work) actually probing which processes have a
  debug/inspector port open.
- **Confirmed worse than initially scoped:** searching the literal PID of the
  actual target process (`7360`, visible in the running session tab
  `start-server.js [7360]`) does **not** surface it. Instead the result list
  is entirely Chrome/Edge helper processes with unrelated PIDs (`13484`,
  `22172`, `4448`, `29024`, `16852`, `15716`, `17080`) — none matching `7360`,
  none Node. The fuzzy matcher is almost certainly searching across the full
  `command` string built from `process.cmd()` (`attach_modal.rs:405-409`),
  and Chrome/Edge command lines are huge (many flags/hashes/GUIDs), so `7360`
  fuzzy-matches noise buried in those strings while ranking the actual target
  at or near the bottom (or it's simply not in the unfiltered `sysinfo` list
  under a name the search matches). This isn't just "too much to scroll
  through" — the search actively surfaces false positives ahead of the real
  target, making even a PID-exact search unreliable.
- **Status:** root cause identified, fix not started. Confirmed the most
  broken of the six — makes Attach nearly unusable even when you know the
  exact PID you're looking for.

### 7. Inline values (and Console evaluate — see gap #3) show shallow, non-expandable summaries

- **Symptom:** while paused, the always-on inline decoration next to a
  variable (e.g. `const result: {item: {…}, list: Array(5),
  other: Array(5)} = await fetchData();`, shown directly on
  the code line, no hovering needed) only prints a shallow one-level summary
  — nested objects render as literal `{…}`, arrays as `Array(N)` — with no
  way to expand or click into them. Confirmed: clicking the `{…}`/`Array(5)`
  text does nothing. To see the real nested contents you have to abandon
  this view and go to the Variables tab.
- **Adapter/language:** general (inline-value decoration feature, not
  adapter-specific).
- **Relation to other gaps:** same underlying shape as gap #3 (Console
  evaluate results not expandable) — in both cases Zed has real DAP
  `variablesReference` data available (the actual nested object/array
  contents) but the rendering surface only shows a flattened preview string
  with no interactive drill-down, unlike the Variables tab's proper tree
  view. Distinct from gap #5, which is about the hover *popup* specifically
  (still untested) — this one is the always-visible inline decoration.
- **Suspected area:** the inline-value rendering path
  (`crates/dap/src/inline_value.rs`, `crates/editor/src/inlays/inlay_hints.rs`)
  — likely needs either a click target that opens the Variables tree scoped
  to that reference, or an inline expand-in-place affordance, instead of a
  dead preview string.
- **Status:** confirmed (always-on, not hover-triggered; not expandable).
  Fix not started.

### 8. ~~Attach to a real Node process fails immediately: "debugger shutdown unexpectedly"~~ (RESOLVED)

- **Symptom:** attaching to a confirmed-correct, currently-running Node
  process (verified PID via `Get-Process -Name node`) fails immediately:
  ```json
  Tried to launch debugger with: {
    "type": "pwa-node",
    "request": "attach",
    "processId": 34856,
    "cwd": "C:\\path\\to\\project",
    "console": "externalTerminal",
    "sourceMaps": true,
    "pauseForSourceMap": true,
    "sourceMapRenames": true
  }
  error: debugger shutdown unexpectedly
  ```
- **Adapter/language:** JavaScript/Node, Attach flow specifically.
- **Root cause (found, fixed):** `crates/dap_adapters/src/javascript.rs:113-115`
  unconditionally defaulted `"console"` to `"externalTerminal"` for **every**
  debug config, launch or attach:
  ```rust
  configuration
      .entry("console")
      .or_insert("externalTerminal".into());
  ```
  For `attach`, there's no process being spawned — you're attaching to an
  already-running PID — so instructing js-debug to open an external terminal
  is meaningless at best; more likely js-debug tries to actually wire up that
  terminal as part of session startup, fails (no terminal spawner configured
  in this context), and the whole adapter process dies immediately — matching
  the observed "launch request sent, then instant shutdown."
- **Fix applied:** guard the default behind checking the config's `request`
  field; only default `console` to `externalTerminal` for `launch`:
  ```rust
  let is_attach = configuration.get("request").and_then(Value::as_str) == Some("attach");
  if !is_attach {
      configuration
          .entry("console")
          .or_insert("externalTerminal".into());
  }
  ```
- **Status:** fix applied and confirmed necessary (config no longer sends
  `console: externalTerminal` for attach), but **not sufficient** — this was
  a real bug but not the actual blocker. See below for the real root cause,
  found via `RUST_LOG=debug` + the adapter's own stderr
  (`crates/dap/src/transport.rs:264` logs it at `debug` level, invisible by
  default — worth its own follow-up, see "Notes" below).
- **Real root cause (confirmed via adapter stderr):**
  ```
  Error: Could not connect to debug target at http://localhost:9229:
  Could not find any debuggable target
  ```
  The attach request Zed sends has **no `port` field at all**:
  ```json
  {"type": "pwa-node", "request": "attach", "processId": 33184,
   "cwd": "...", "sourceMaps": true, ...}
  ```
  js-debug falls back to Node's hardcoded default inspector port `9229`.
  The actual target (Next.js dev server under Turbopack) has its inspector
  on a dynamically-assigned port (seen `60278`, `64676`, `52734` across
  different runs — never `9229`), and js-debug isn't auto-discovering it.
  Structurally confirmed: `AttachRequest` in
  `crates/task/src/debug_format.rs:56-59` only has a `process_id` field —
  **there is no way to specify a port anywhere in Zed's attach data model,
  for any adapter**, so this can't be fixed by editing `javascript.rs`
  alone; it needs a new field threaded through `AttachRequest` →
  `ZedDebugConfig`/`DebugRequest::Attach` → the Attach modal UI (so the port
  shown in the dev server's own banner can be entered) → each
  adapter's `config_from_zed_format` (at minimum `javascript.rs`, to emit
  `"port"` in the JSON when set).
- **Fix applied:** added `port: Option<u16>` to `AttachRequest`
  (`crates/task/src/debug_format.rs:56-82`); added an optional "Port" input
  to the Attach modal, shown above the process list, read at confirm time
  (`crates/debugger_ui/src/attach_modal.rs`); `javascript.rs`'s
  `config_from_zed_format` now emits `"port"` in the DAP config when set
  (`crates/dap_adapters/src/javascript.rs:208-213`). Fixed resulting
  compile errors at other `AttachRequest` construction sites
  (`new_process_modal.rs`, extension host WIT conversion — port defaulted
  to `None` there since it's not in the extension ABI yet). Deliberately did
  NOT add the same port passthrough to `python.rs` — debugpy's attach schema
  may need a nested `"connect": {"port": ...}}` shape rather than a flat
  `"port"` field, untested here; left as a follow-up.
- **Status:** RESOLVED — confirmed working end-to-end. Attach now connects
  to the correct inspector port instead of the wrong default (`9229`), and
  the "wrong frame" symptom from earlier testing (evaluate resolving to
  Node's global `profile()` instead of the real local) is also gone:
  evaluating a local variable now correctly returns real data
  (`{status: 'fulfilled', value: {…}}`), confirming both the connection
  and the frame/scope resolution are correct via attach.
- **Follow-up noticed during this test (not a regression, separate minor
  issue):** `ERROR [session.rs:1813] no adapter running to send request:
  ThreadsCommand` still logs once per session right at connect time — same
  early-request race noted during gap #8 investigation, but this time it
  didn't prevent the session from working. Low priority; see gap #10.

### 10. Harmless "no adapter running to send request: ThreadsCommand" race at session connect

- **Symptom:** every session start logs
  `ERROR [session.rs:1813] no adapter running to send request:
  ThreadsCommand` at the same moment the adapter's TCP connection is
  established — the request is sent before the transport is fully wired up.
  Does not appear to block or break the session (confirmed working attach
  in gap #8 still logged this).
- **Adapter/language:** general (session bring-up, not adapter-specific).
- **Suspected area:** whatever eagerly requests the thread list on session
  creation/UI mount (likely `crates/debugger_ui/src/session/running/stack_frame_list.rs`
  or similar) fires before `TransportDelegate::connect` finishes wiring the
  TCP stream — `crates/project/src/debugger/session.rs:1813` is where the
  `ThreadsCommand` fetch fails with "no adapter running."
- **Status:** confirmed present, cosmetic/log-noise only so far (not
  reproduced as blocking anything). Low priority — worth a quick look for
  whether it should just be deferred/retried rather than erroring, but not
  urgent.

### 9. ~~Debug adapter's own stderr is only logged at `debug` level — connection failures are a black box by default~~ (RESOLVED)

- **Symptom:** when a session fails to start (e.g. gap #8), the only thing
  visible in the log at default verbosity is a generic
  `ERROR debugger shutdown unexpectedly` — no indication of *why*. Diagnosing
  gap #8 required setting `RUST_LOG=debug` and grepping the app's log file
  to find the adapter's
  actual stderr output, which contained the real error
  (`Could not connect to debug target at http://localhost:9229: Could not
  find any debuggable target`).
- **Root cause (found):** `crates/dap/src/transport.rs:264` —
  `log::debug!("stderr: {line}")` — logs every line of the spawned debug
  adapter process's stderr at `debug` level unconditionally, regardless of
  whether the session is failing. At default (non-debug) log verbosity, this
  is completely invisible, so any adapter-side failure surfaces
  as an opaque one-liner with zero actionable detail.
- **Expected:** at minimum, when a session ends abnormally (the
  `ERROR debugger shutdown unexpectedly` path in
  `crates/debugger_ui/src/debugger_panel.rs`), the adapter's recent stderr
  output (or at least its last few lines) should be surfaced — either bumped
  to `warn`/`error` level in the log, or shown directly in the session's
  Console tab — instead of requiring debug logging to be enabled and the
  log file searched by hand.
- **Suspected area:** `crates/dap/src/transport.rs` (stderr capture) +
  `crates/debugger_ui/src/debugger_panel.rs` (abnormal-shutdown handling) —
  needs the stderr buffer/tail to be threaded into whatever fires the
  "shutdown unexpectedly" error, and surfaced in the UI, not just to the
  debug log.
- **Update — scope much smaller than first thought:** Zed already has a
  dedicated adapter-log viewer for exactly this
  (`crates/debugger_tools/src/dap_log.rs`, action `dev: Open Debug Adapter
  Logs` / `OpenDebugAdapterLogs`), wired via `DapClient::add_log_handler`
  (`LogKind::Adapter` for stderr) into a per-session log view — confirmed
  working: it shows the exact same `Could not connect to debug target at
  http://localhost:9229: ...` stderr, live, no `RUST_LOG` needed. So the
  capture/display mechanism isn't missing — it's just not discoverable from
  the failure itself. The `debug!`-level log line at `transport.rs:264` is a
  separate, lower-value logging channel (fine as debug-only, since the real
  UI path already exists).
- **Real fix (smaller):** when a session ends abnormally
  (`ERROR debugger shutdown unexpectedly`, `crates/dap/src/transport.rs:208`
  → surfaced by `crates/debugger_ui/src/debugger_panel.rs`), point
  at `dev: Open Debug Adapter Logs` instead of leaving it to guesswork — e.g.
  a toast/notification action button, or including the hint in whatever
  message reaches the Console tab.
- **Fix applied:** `crates/project/src/debugger/session.rs:987-998` (right
  where the "Tried to launch debugger with: ..." console message is already
  sent on a failed `initialize_sequence`) now also sends a follow-up Console
  line pointing at `dev: Open Debug Adapter Logs`.
- **Status:** RESOLVED — confirmed after clean rebuild + restart. Console
  tab now shows the hint line right after "Tried to launch debugger with:
  ...", before "error: debugger shutdown unexpectedly".

### 11. No way to disable/ignore all breakpoints from the UI

- **Symptom:** no button, icon, or discoverable way in the Breakpoints panel
  (or anywhere else visible) to disable all breakpoints at once (mute them
  without deleting) — only per-breakpoint toggling exists in the UI.
- **Root cause (found):** the feature already exists as an action —
  `ToggleIgnoreBreakpoints` (`crates/debugger_ui/src/debugger_ui.rs:56-57`),
  wired to `item.toggle_ignore_breakpoints(cx)`
  (`crates/debugger_ui/src/debugger_ui.rs:251-258`) — but has **zero UI
  affordance**: no toolbar button anywhere in `debugger_panel.rs` or
  `breakpoint_list.rs`, and no default keybinding in any of
  `assets/keymaps/*.json`. The only way to invoke it is by typing its full
  name into the command palette (`debugger: Toggle Ignore Breakpoints`),
  which nothing hints at.
- **Workaround (works today):** Ctrl+Shift+P → "toggle ignore breakpoints".
- **Expected:** a visible toggle in the Breakpoints panel's toolbar (the row
  with play/restart/step icons at the top), similar to VS Code's
  "breakpoints activation" toggle — plus a default keybinding.
- **Suspected area:** `crates/debugger_ui/src/debugger_panel.rs` (panel
  toolbar) or `crates/debugger_ui/src/session/running/breakpoint_list.rs`
  (panel-local toolbar) — add an `IconButton` bound to
  `ToggleIgnoreBreakpoints`; optionally add a default keymap entry.
- **Confirmed: the underlying suppression actually works.** Initially looked
  broken (tested via the command palette, breakpoints still appeared to
  trigger) — but that was based on the dots not changing color, not on
  actually continuing past a breakpoint. Traced
  `crates/project/src/debugger/session.rs:344-378`
  (`send_source_breakpoints`): when `ignore_breakpoints` is true, it sends
  an **empty breakpoints array** to the adapter (line 356-357:
  `if ignore_breakpoints { vec![] } else { ... }`), which genuinely
  suppresses all of them at the DAP level. So this is two separate gaps
  layered together:
  1. No UI affordance to invoke it (button/keybinding) — as above.
  2. **No visual feedback at all** once it's active — each breakpoint's
     dot keeps showing `state.is_enabled()` (a separate, untouched
     per-breakpoint field), so there's no way to tell "all breakpoints are
     currently being ignored" from the list, which is why it read as
     "broken" rather than "silently working."
- **Expected fix, part 2:** when `session.ignore_breakpoints` is true, dim/
  grey out every breakpoint dot in the list (visually distinct from a
  per-breakpoint disabled state), and/or show an active-state indicator on
  whatever toolbar toggle gets added for part 1.
- **Fix applied:** added an `IconButton` to
  `BreakpointList::render_control_strip`
  (`crates/debugger_ui/src/session/running/breakpoint_list.rs`) using the
  previously-completely-unused `IconName::DebugIgnoreBreakpoints` icon
  (confirmed via grep it had zero references anywhere before this — the
  button was clearly planned but never wired up), toggled state reflects
  `session.ignore_breakpoints()`, click calls
  `session.toggle_ignore_breakpoints(cx)` directly. Threaded a new
  `session_ignoring: bool` through `BreakpointEntry::render` →
  `LineBreakpoint`/`ExceptionBreakpoint`/`DataBreakpoint::render`, so every
  breakpoint dot renders `Color::Muted` while ignoring is active, restoring
  its normal color when toggled back off. `render_control_strip` gained a
  `cx: &App` parameter to read the session state; updated its three call
  sites (`debugger_panel.rs`, `session/running.rs`).
- **Status:** RESOLVED — compiles clean, awaiting visual confirmation in a
  rebuilt app.

### 12. Chained shell commands (`q1 && q2 && q3`) only ever debug the first one, then the whole session dies

- **Symptom:** launching a debug target whose command is a shell chain (e.g.
  `gulp clean && gulp build && gulp dev`) debugs `q1` (`gulp clean`)
  correctly; when that process exits, the *entire* debug session tears down
  — no second session is ever created, `q2`/`q3` run in the shell
  completely un-debugged. Confirmed via repro: "after the first one it kills
  it so it doesn't even reach the second query."
- **Root cause (found):** this is **not** gap #2 (child-session focus
  churn) — confirmed no second session is even attempted, so the
  `startDebugging`/child-session machinery never enters the picture. What's
  actually happening: `q1` is the *only* process ever attached to. When it
  exits, `Events::Terminated(_) => { self.shutdown(cx).detach(); }`
  (`crates/project/src/debugger/session.rs:1554-1556`) correctly tears down
  that (sole) session — that's correct DAP behavior for a single-process
  target. The shell then runs `q2`/`q3` as brand-new OS processes nothing
  is watching for.
- **The actual fix needed:** js-debug has a purpose-built mechanism for
  exactly this — the `node-terminal` launch type, which runs the given
  command in a terminal with Node's auto-attach watching it, so *any* Node
  process the shell spawns there (sequentially or otherwise) gets picked up
  automatically. It's already declared as a valid `type` in
  `crates/dap_adapters/src/javascript.rs`'s schema, reachable today via
  "Edit in debug.json" → `{"type": "node-terminal", "command": "..."}`.
- **Found and fixed a second bug blocking that fix:** `node-terminal`'s
  `command` handling (`javascript.rs:65-79`, pre-fix) used
  `ShellKind::Posix.split(&command)` — a naive whitespace word-split, not a
  real shell invocation. For `"gulp clean && gulp build && gulp dev"` this
  produced `runtimeExecutable: "gulp"`,
  `runtimeArgs: ["clean", "&&", "gulp", "build", "&&", "gulp", "dev"]` —
  `&&` passed as a **literal CLI argument** to `gulp`, not interpreted as a
  shell operator. The chain would never actually run correctly through this
  path even with `node-terminal` selected.
- **Fix applied:** replaced the naive split with
  `ShellBuilder::new(&Shell::System, cfg!(target_os = "windows")).build(Some(command), &[])`
  (`crates/util/src/shell_builder.rs` — the same shell-wrapping helper
  `crates/project/src/terminals.rs` and the remote/SSH transports already
  use elsewhere in this codebase). This wraps the whole command string as
  one real shell invocation (`cmd /S /C "..."` on Windows, `sh -c "..."` on
  POSIX) instead of splitting it — `&&`/`||`/pipes are now interpreted
  correctly, and each step in the chain spawns as its own genuine child
  process for js-debug's terminal auto-attach to pick up.
- **Status:** fix applied to the identified blocking bug, compiles clean.
  **Not yet confirmed end-to-end** — need to verify with `node-terminal` +
  a real `&&` chain that: (1) the command now actually runs correctly
  through a real shell, and (2) js-debug's terminal auto-attach genuinely
  attaches to each sequential `gulp <task>` process as the chain
  progresses, giving the `q1 → debug → q2 → debug → q3 → debug → done`
  behavior. That second part depends on js-debug's own auto-attach
  behavior, which hasn't been exercised in this repo before — worth testing
  before assuming the whole gap is closed.
- **Update — gap #12's shell fix confirmed partially working, surfaced a
  new, deeper bug (gap #13):** after the fix, real end-to-end test with
  `node-terminal` + `"gulp clean && gulp build && gulp dev"` against a
  gulp-based project: `gulp clean` runs and finishes correctly (confirmed
  in the debug session's own Terminal tab — real output, not garbled), and
  js-debug's auto-attach *does* fire again for the next process ("Debugger
  attached." appears a second time) — so the shell-wrapping fix and
  terminal auto-attach mechanism are both doing their job. But the chain
  then hangs completely: `gulp build` never starts, the Terminal tab sits
  frozen right after the second "Debugger attached.", Frames is empty, and
  clicking Continue is a no-op. This is now tracked separately as gap #13
  (see below) — it's a session/transport bug, not a further shell-parsing
  issue.

### 13. Second auto-attached session in a node-terminal chain never becomes controllable — dead request routing

- **Symptom:** with gap #12's fix applied (`node-terminal` + real shell
  chain), `gulp clean` runs and finishes correctly, and js-debug's
  auto-attach fires again for the next process ("Debugger attached." shown
  a second time in the session's Terminal tab) — but that second session is
  completely inert: the chain never progresses to `gulp build`, Frames is
  empty, and Continue does nothing.
- **Evidence (via `RUST_LOG=debug` + the app's log file, same method as
  gap #8):**
  ```
  INFO  [dap::transport] Debug adapter has connected to TCP server 127.0.0.1:62284
  INFO  [alacritty_terminal::tty::windows::conpty] Using Windows API for pseudoconsole
  INFO  [dap::transport] Debug adapter has connected to TCP server 127.0.0.1:62284
  INFO  [dap::transport] Debug adapter has connected to TCP server 127.0.0.1:62284
  ERROR [session.rs:1813] no adapter running to send request: ThreadsCommand
  ... (~11s later)
  INFO  [dap::transport] Debugger closed the connection
  INFO  [dap::transport] Debugger closed the connection
  ```
  **Three** separate "Debug adapter has connected to TCP server" events, all
  on the *same* port, in quick succession — not three different ports for
  three different sessions. A second test run showed the same pattern, plus
  two `ERROR [dap_store.rs:496] Could not find session: SessionId(N)` lines
  right at final teardown (`shutdown_session`,
  `crates/project/src/debugger/dap_store.rs:739-746` — returns that error
  when a session_id was already removed from `self.sessions`, i.e. a
  double-shutdown race between a session's own `Shutdown` event and its
  parent's recursive `shutdown_children` cascade at
  `dap_store.rs:748-753`).
- **Ruled out:** not a "paused but UI doesn't show it" issue — confirmed by
  testing: Frames tab is genuinely empty (no stack, not just uncollapsed),
  and Continue is a no-op, not "does nothing visible but the process is
  fine." The `ThreadsCommand` failure ("no adapter running") is not
  cosmetic here (contrast gap #10, where the same error appeared but didn't
  block anything) — in this multi-connection scenario it points at
  genuinely dead request routing for whichever session Zed ends up
  tracking as "the" active one.
- **Leading theory (not yet confirmed):** three TCP connections to one
  port suggests Zed may be accepting multiple redundant connections for
  what should be one logical session (or one root + child(ren) that get
  conflated), and only wiring up request/response routing
  (`pending_requests` in `crates/dap/src/transport.rs`) for one of them —
  leaving whichever session_id the UI actually tracks pointed at a
  connection that never receives responses. Needs tracing through:
  - `crates/dap/src/transport.rs` — how a `TcpListener` accepts and wires
    up multiple incoming connections on one port (does anything limit it to
    "first connection wins" for message routing while still logging every
    subsequent one as if it were also live?).
  - `crates/project/src/debugger/session.rs`/`dap_store.rs` — how child
    sessions spawned via terminal auto-attach get their `session_id` ↔
    transport/connection mapping established, and whether that mapping can
    end up pointing at a dead/superseded connection.
- **Root cause (found):** confirmed via two independent methods (stale log
  file investigation + live `dev: Open Debug Adapter Logs` viewer, both
  showing **zero** DAP protocol traffic on the child session — not delayed,
  never sent) that the child session's connection is never recognized by
  js-debug at all. Traced to `crates/debugger_ui/src/debugger_panel.rs`'s
  `handle_start_debugging_request`: it builds a child session's `binary` by
  cloning the **parent's entire binary** (`parent_session.read(cx).binary().cloned()`)
  and only overwriting `request_args` — `command`/`arguments`/`connection`
  (including the port) all carry over unchanged. `JsDebugAdapter::get_binary`
  (`dap_adapters/src/javascript.rs:174-195`) always sets
  `command: Some(<node path>)`, `arguments: [adapter_path, port, host]` —
  correct for the top-level session (it needs to actually spawn js-debug),
  but wrong for a child: every child session was trying to spawn **another**
  `node dapDebugServer.js <same port> <same host>` process — a duplicate
  js-debug server attempting to bind the exact port the parent's is already
  listening on. That fails inside the child Node process (`EADDRINUSE`),
  while Zed's own `.connect()` still succeeds against the *original*
  process (explains why every "Debug adapter has connected to TCP server"
  line showed the *same* port across 2-3 sessions) — but a genuine js-debug
  client (VS Code) never spawns a second process for children; it only
  opens a new connection to the existing server, which then pairs that
  socket to the pending child it announced via `startDebugging`. Zed's
  child connection was never paired with anything on js-debug's side, so no
  DAP traffic ever flowed on it — matching the confirmed "zero messages,
  not delayed" evidence exactly.
- **Fix applied:** `debugger_panel.rs`'s `handle_start_debugging_request` now
  clears `binary.command = None` and `binary.arguments = Vec::new()` before
  booting the child session, so `TcpTransport::start`
  (`crates/dap/src/transport.rs:516`, `if let Some(command) = &binary.command`)
  skips spawning a process entirely and the child session only connects to
  the parent's already-running adapter at `binary.connection`'s host/port.
- **Status:** REVERTED — disproven by live testing. Rebuilt, verified fresh
  process (confirmed via `Get-Process zeddev`/`StartTime`, ruling out a
  stale-binary or duplicate-instance testing artifact), reproduced the `&&`
  chain: with the fix applied, the shell hung indefinitely (4+ minutes, no
  further progress) at `Waiting for the debugger to disconnect...` right
  after `gulp clean` finished — strictly worse than the pre-fix behavior,
  which at least reached a second `Debugger attached.` within the usual
  ~10-11s window. Reverted the `binary.command = None`/`binary.arguments =
  Vec::new()` change (`debugger_panel.rs`'s `handle_start_debugging_request`
  back to its original form) rather than leave the app in a worse state.
  The underlying reasoning (duplicate process spawn attempting to bind an
  already-used port) may still be correct as *a* problem, but it is either
  not *the* blocking problem, or fixing it exposed a second, more
  fundamental issue — **`Waiting for the debugger to disconnect...`
  indicates the *first* session's own shutdown/disconnect handshake may be
  what's actually hanging** (`Session::shutdown()`,
  `crates/project/src/debugger/session.rs:2254-2305` —
  `TerminateCommand`/`DisconnectCommand`), blocking the shell's `&&` chain
  from ever reaching the point where a second process (and thus a second
  session) would even be spawned, independent of how that second session's
  binary gets built. **Also confirmed a separate, real bug found during
  this testing round: the app's log file stopped receiving writes
  entirely (stuck at a fixed `LastWriteTime`) across multiple full
  restarts** — logging pipeline itself may be silently wedging after some
  condition; not investigated further, but worth its own gap if it recurs.
  Root cause still open — next investigation should start from
  `Session::shutdown()`'s request/response handling for the *first*
  session, not child-session binary construction.
- **External research (2026-09-29):** `Waiting for the debugger to
  disconnect...` hanging is a **known, recurring issue in genuine upstream
  VS Code + js-debug**, not unique to this port — found multiple open
  GitHub issues on `microsoft/vscode` describing the identical symptom
  (`Debugger attached. Waiting for the debugger to disconnect...`,
  intermittent, hard to reproduce reliably), none with a clear documented
  root cause or fix from Microsoft's own maintainers (checked issue #79570
  specifically — labeled "info-needed," no resolution ever posted). This
  doesn't mean it's unfixable here, but it recalibrates expectations: this
  may be a genuine edge case in js-debug's own terminal-auto-attach
  protocol rather than a straightforward bug introduced by this port.
  Further progress likely requires either deep protocol-level tracing
  (reading js-debug's own source for its terminal-delegate/auto-attach
  disconnect logic) or live instrumented testing (temporary diagnostic
  logging + rebuild) rather than more black-box reproduction cycles.

### 14. node-terminal auto-attach actively breaks the build: pollutes internal tool subprocesses' stdio (severe)

- **Symptom:** tested a different, simpler scenario than gaps #12/#13 —
  `.zed/debug.json` pointed at a gulp-based project's existing npm `serve`
  script (via `npm run serve` → `gulp serve`, a *single* gulp invocation,
  not a shell `&&` chain at the debug-config level). gulp's `serve` task
  runs several
  subtasks internally, including spawning `tsc` (TypeScript compiler) and
  ESLint as their own child processes. Real observed output:
  ```
  [08:46:49] Error - [tsc] Debugger attached.
  [08:46:49] Error - [lint] Unexpected STDERR output from ESLint: Debugger attached.
  ```
- **Root cause:** the auto-attach mechanism attaches to **every** Node
  process spawned in that terminal, including internal build-tool
  subprocesses that were never meant to be debugged (`tsc`, ESLint's own
  worker). Each one prints its own `Debugger attached.` inspector banner to
  its stdout/stderr. gulp's ESLint plugin treats any unexpected stderr
  output from the ESLint subprocess as an error condition — so the
  debugger's own banner text gets misinterpreted as a real ESLint failure,
  surfacing as a **false error in the project's actual build output**.
- **Why this matters more than gaps #2/#12/#13:** those are UX/session-focus
  problems — annoying, but the underlying app still (mostly) runs. This one
  actively **corrupts the build pipeline's own output/error reporting** —
  a debugging feature is now causing false build failures, which is a much
  higher-severity class of problem for any gulp/webpack/similar project
  with internal tool subprocesses (near-universal for real-world
  frontend build setups, not an edge case).
- **Expected:** auto-attach should not attach to well-known internal
  build-tool subprocesses (`tsc`, `eslint`, and similar) at all, or at
  minimum should not let its own banner reach a stream the parent tool
  parses for errors. VS Code's own Node auto-attach has this exact class of
  problem and mitigates it via smart/heuristic exclusion — this repo has
  not attempted anything like that yet.
- **Suspected area:** wherever Zed injects the inspector-enabling
  environment (`NODE_OPTIONS` or similar) for `console: externalTerminal`/
  auto-attach sessions — needs either an exclusion list for known
  build-tool binaries, or a way to suppress the inspector's own stdout/
  stderr banner independent of exclusion (Node has `--inspect` banner
  suppression options in some versions).
- **Status:** confirmed via real repro, root cause understood at a
  conceptual level, no fix attempted yet. **Recommend not using
  node-terminal auto-attach for gulp-based projects with internal tsc/
  eslint subprocesses until this is fixed** — it can silently break the
  build, not just clutter the debugger UI.
- **Correlation with gap #13 (likely same root cause):** second repro (same
  `gulp serve` scenario) showed the exact same ~11s delay pattern as gaps
  #8/#13:
  ```
  [08:49:28] Error - [tsc] Debugger attached.
  [08:49:37] Error - [tsc] Waiting for the debugger to disconnect...
  [08:49:37] Error - 'tsc' sub task errored after 11 s
   exited with code 0
  [08:49:41] Error - [lint] Unexpected STDERR output from ESLint: Waiting for the debugger to disconnect...
  ```
  Working theory: each auto-attached subprocess (`tsc`, `lint`) starts with
  its inspector paused, waiting for a real debug client to take control
  (matches this adapter's own `pauseForSourceMap: true` default). Because
  of gap #13's dead session routing (`SessionState` stuck in `Booting`,
  never reaches `Running`, no real request/response flow — see that gap's
  entry), Zed never actually sends the `configurationDone`/continue that
  would un-pause it. The subprocess sits frozen until Node's own internal
  wait-for-debugger timeout (~10-11s) gives up and forces it to proceed/
  exit — by then gulp/ESLint's plugin has already treated the delay/leftover
  banner text as an error. **Gap #13 is very likely the actual root cause of
  gap #14, not a separate bug** — fixing #13's dead handshake/routing would
  likely resolve #14 as a side effect (subprocesses would get a real,
  functioning debug session that properly continues them instead of
  hanging until timeout).

## Resolved

- **Gap #6** — Attach process picker now filters to adapter-relevant
  processes (`node`/`deno`/`bun` for JS, `python`/`python3` for Python)
  instead of listing every OS process. `dap/src/adapters.rs`,
  `dap_adapters/src/javascript.rs`, `dap_adapters/src/python.rs`,
  `debugger_ui/src/attach_modal.rs`.
- **Gap #8** — Attach now supports an explicit port, fixing the
  "always guesses 9229" bug; confirmed working end-to-end (connects,
  correct frame/scope on evaluate). `task/src/debug_format.rs`,
  `debugger_ui/src/attach_modal.rs`, `dap_adapters/src/javascript.rs`.
- **Gap #9** — Console now hints at `dev: Open Debug Adapter Logs` on
  abnormal session shutdown. Fix in `crates/project/src/debugger/session.rs:987-1006`.

## Notes / context

- Relevant crates: `crates/dap` (protocol/client), `crates/dap_adapters`
  (per-language adapter configs), `crates/debug_adapter_extension` (extension
  integration), `crates/debugger_tools`, `crates/debugger_ui` (panel/UI).
