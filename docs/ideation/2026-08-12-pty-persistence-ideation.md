---
date: 2026-08-12
topic: pty-persistence
focus: Preserve integrated-terminal PTYs across Zed updates and restarts
mode: repo-grounded
---

# Ideation: PTY Persistence Across Zed Restarts

## Grounding Context

- `crates/terminal/src/terminal.rs`: `TerminalBuilder::new` opens the PTY inside the GUI process. `Drop for Terminal` shuts down the sender, terminates the child process group, then schedules a hard kill after 100 ms.
- `crates/terminal_view/src/terminal_view.rs`: terminal serialization stores cwd and custom title, skips task terminals, and deserialization creates a fresh shell.
- `crates/terminal_view/src/persistence.rs`: panel persistence restores split/pane layout and item IDs, not PID, PTY, process state, grid, or scrollback.
- `crates/gpui/src/app.rs` and platform restart implementations: restart replaces the GUI without old/new process overlap; shutdown futures have a 200 ms budget.
- Existing reusable patterns include the detached server and reconnectable sockets in `crates/remote_server/src/server.rs`, typed IPC handshake in `crates/cli/src/cli.rs`, and helper keepalive in `crates/crashes/src/crashes.rs`.
- External precedent: [tmux](https://github.com/tmux/tmux/wiki/Getting-Started) and [WezTerm](https://wezterm.org/multiplexing.html) keep PTYs in a background server. [VS Code](https://code.visualstudio.com/docs/terminal/advanced) explicitly distinguishes process reconnection from process revive.

## Topic Axes

- PTY/process ownership
- Screen and scrollback continuity
- Lifecycle policy for update, quit, and crash
- Cross-platform and cross-version compatibility
- Security, recovery, and UX

## Ranked Ideas

### 1. Native per-user PTY broker

**Description:** A small per-user, release-channel-scoped broker owns local PTYs, child process groups, and drain loops. Zed windows hold opaque session handles and attach or detach without controlling process lifetime.

**Axis:** PTY/process ownership

**Basis:** `direct:` GUI-owned PTYs are unconditionally terminated by `Drop for Terminal`. `external:` tmux and WezTerm preserve live processes by making the GUI a reconnectable client.

**Rationale:** Moving the PTY master outside the replaceable GUI is the only option that preserves the same PID, job-control state, SSH connection, REPL memory, and open file descriptors.

**Downsides:** High implementation cost, a shared broker failure domain, and distinct Unix PTY versus Windows ConPTY backends.

**Confidence:** 96%

**Complexity:** High

**Status:** Explored

### 2. Broker-authoritative screen snapshot and delta stream

**Description:** Keep the VT parser, main and alternate grids, cursor modes, title, and bounded scrollback beside the PTY. Attach sends an atomic versioned snapshot followed by generation- and sequence-numbered deltas; gaps trigger resynchronization.

**Axis:** Screen and scrollback continuity

**Basis:** `direct:` current persistence stores only cwd and title. `reasoned:` a living process is insufficient when it emits terminal control sequences while no GUI parser is attached.

**Rationale:** This preserves both the live process and the exact screen users expect to recover.

**Downsides:** Requires cross-process terminal-state representation, memory limits, and backpressure.

**Confidence:** 92%

**Complexity:** High

**Status:** Unexplored

### 3. Stable session manifest with attach-first restoration

**Description:** Persist a broker endpoint, opaque session capability, generation, cwd and title, and recovery policy separately from pane layout. Restoration attempts attachment before creating a shell; unavailable sessions may offer a visibly labeled reconstruction.

**Axis:** Security, recovery, and UX

**Basis:** `direct:` pane layout persists but `TerminalView::deserialize` always creates a fresh shell. `external:` VS Code distinguishes reconnection from process revive.

**Rationale:** This joins the existing pane restore path to actual process identity without silently creating duplicates.

**Downsides:** Requires safe capability storage, schema migration, and stale-manifest cleanup.

**Confidence:** 95%

**Complexity:** Medium

**Status:** Unexplored

### 4. Explicit lifecycle dispositions with fenced leases

**Description:** Replace destructor policy with explicit `Terminate`, `Detach`, `PreserveForRestart`, and `RecoverAfterCrash` dispositions. A lease bounds orphan lifetime, while fencing tokens guarantee a single writable client.

**Axis:** Lifecycle policy for update, quit, and crash

**Basis:** `direct:` object destruction cannot distinguish pane close, quit, restart, crash, or view relocation, yet the current destructor decides process death.

**Rationale:** Lifecycle intent becomes testable and no longer depends on shutdown timing.

**Downsides:** Requires clear product defaults for each exit path and deterministic handling of stale clients.

**Confidence:** 93%

**Complexity:** Medium

**Status:** Unexplored

### 5. Versioned, capability-negotiated local protocol

**Description:** Define additive create, attach, snapshot, input, resize, signal, detach, and terminate messages with adjacent-release compatibility. Use owner-only Unix sockets on macOS and Linux and ACL-restricted named pipes on Windows.

**Axis:** Cross-platform and cross-version compatibility

**Basis:** `direct:` an updated GUI must reconnect to a broker from the previous release. Existing Zed detached-server and IPC patterns provide proven local primitives.

**Rationale:** A broker cannot preserve update-time sessions unless old and new binaries fail compatibly and unauthorized clients cannot control sessions.

**Downsides:** Creates a durable protocol and a cross-version CI burden.

**Confidence:** 90%

**Complexity:** High

**Status:** Unexplored

### 6. Declared persistence classes for shells, tasks, and remote sessions

**Description:** Classify sessions as restart-surviving interactive shells, durable watch or service tasks, disposable tasks, or externally managed remote attachments. Roll out local interactive shells first.

**Axis:** Security, recovery, and UX

**Basis:** `direct:` task terminals are already excluded from serialization, and project quit shuts down remote server processes, so these session classes already have different lifetime boundaries.

**Rationale:** Explicit classes avoid resurrecting builds or sensitive sessions while allowing durable workflows later.

**Downsides:** Requires defaults for task authors, users, and remote projects.

**Confidence:** 88%

**Complexity:** Medium

**Status:** Unexplored

## Rejection Summary

| Idea | Reason Rejected |
|---|---|
| Stronger cwd/title/command reconstruction | Reconstruction cannot preserve PID, SSH, REPL, job-control, or file-descriptor state. |
| Shutdown-time PTY fd or ConPTY handle transfer | The PTY is already GUI-owned, shutdown has a 200 ms budget, and Windows handle semantics enlarge the failure surface. |
| Automatic tmux wrapper | Useful opt-in workaround, but installation, configuration, key handling, and Windows support do not provide one product contract. |
| One broker per workspace | Better isolation does not initially justify process multiplication and ambiguous terminal movement or workspace-close semantics. |
| Process-only continuation without screen state | Same PID alone still loses detached output, alternate-screen state, and expected visual continuity. |
| Always-on persistence across explicit Quit | Exceeds the stated update and restart problem; retain it as a later policy option. |

## Refinement: Agent-Process Revival

Live PTY continuation and agent-session revival are separate recovery layers:

1. **Attach to the original PTY first.** If the broker or host proves that the recorded PTY incarnation is still alive, Zed reconnects without launching a process.
2. **Revive the logical agent session second.** If the PTY or agent process has ended, Zed may launch a new process that resumes the harness session.
3. **Create a plain replacement shell last.** If neither identity is recoverable, Zed must label the result as reconstruction rather than continuation.

The second layer requires harness-specific knowledge. `terminal_init_command` is currently one opaque shell string that runs for both new and restored terminal threads. It cannot identify a harness session, distinguish launch from resume, or safely choose a per-terminal resume command. Foreground command detection in `agent_panel.rs` recognizes agent binaries only for telemetry and is not a recovery contract.

### Proposed Recovery Record

Persist these optional fields beside `TerminalThreadMetadata`:

- terminal-agent profile identifier
- versioned opaque session locator or host-owned resume path
- host PTY identifier and incarnation, when available
- last observed agent disposition: running, exited, detached, or unknown
- recovery policy: prompt, automatic, or never

Do not persist the complete command line, environment, terminal contents, or credentials. A configured profile resolves the opaque locator to a structured launch specification.

### Proposed Recovery Precedence

| Recorded state | Action | User-visible claim |
|---|---|---|
| Matching live PTY incarnation | Attach | Continued live terminal |
| Shell alive, agent process exited, valid session locator | Relaunch the adapter in that shell | Resumed agent session in a new process |
| PTY gone, valid session locator | Create a shell, then relaunch the adapter | Resumed agent session in a new process |
| Missing, stale, or incompatible locator | Offer a plain shell | Reconstructed terminal |

The host authority must fence concurrent recovery so two Zed processes cannot resume the same logical session. Explicit terminal close clears both the live-session capability and the revival record.

### Adapter Boundary

A terminal-agent profile must define:

- how a new session is launched
- how its stable session locator is obtained
- how a session is resumed
- whether automatic revival is safe
- which profile/version changes invalidate revival

This belongs behind a narrow terminal-agent adapter boundary, not in PTY transport and not in ACP. ACP already demonstrates capability-gated logical-session load/resume, but terminal TUIs need harness-specific launch and identity handling.

### Recommended First Release

- Add one explicit OMP terminal-agent profile and a versioned resume-path record.
- Default revival to a user action after an exited process; allow profile-level automatic revival later.
- Keep `terminal_init_command` for ordinary new shells. Do not rerun it as the recovery mechanism for a terminal that has revival metadata.
- Let live broker attachment win over OMP relaunch in every case.
- Label resumed sessions as a new process so users do not confuse conversation continuity with PID, REPL, SSH, or file-descriptor continuity.
- Defer inferred recovery for arbitrary foreground programs and replay of arbitrary command history.

This validates the same session-boundary and host-authority split visible in Orca without requiring every harness adapter in the first release. Claude Code, Codex CLI, Gemini CLI, and other harnesses can be added only after each has a reliable session-identity source; resume flags alone are insufficient.

### Meeting Tests

- Updating Zed while the PTY remains live reattaches and never starts a second agent process.
- Exiting OMP while retaining its session record exposes one resume action and relaunches the same logical session exactly once.
- A stale PTY incarnation cannot suppress agent revival or receive input.
- A stale or incompatible OMP resume path produces an explicit error and leaves a usable shell.
- Explicit terminal close removes the revival record and prevents later resurrection.
- A restored ordinary shell never becomes an agent session solely because its previous foreground process name matched a known agent binary.
