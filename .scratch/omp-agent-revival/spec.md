# Spec: OMP Agent-Session Revival in the Agent Panel

## Problem Statement

When Zed updates or restarts, terminals running a TUI coding agent (OMP) are not recoverable. Zed restores a terminal by spawning a fresh shell and re-running a global `terminal_init_command`, which starts a **new** agent session instead of resuming the old one. Users lose in-flight agent work across updates, and there is no way to tell Zed that a terminal is an agent session worth reviving.

## Solution

Zed gains a dedicated **OMP agent terminal** entry in the agent panel, distinct from plain terminal threads. Each such terminal carries a Zed-assigned **resume path**. When the OMP process exits or Zed quits, the terminal becomes a **sleeping session**. On reopening the worktree, Zed automatically resumes the sleeping session (new process, same logical session) exactly once, showing a restored banner. If resume fails, the user keeps a usable shell and an explicit error.

## User Stories

1. As a user running an OMP agent in the agent panel, I want a dedicated "new OMP agent terminal" entry, so that agent sessions are distinct from plain shells and not confused with `terminal_init_command`.
2. As a user, I want Zed to assign each OMP terminal a resume path, so that the session is identified deterministically without parsing TUI output.
3. As a user, I want an agent session to become **sleeping** when the OMP process exits, so that it can be resumed later.
4. As a user, I want an agent session to become **sleeping** when Zed quits with an OMP agent running, so that it survives app restarts.
5. As a user, I want Zed to auto-resume a sleeping session when I reopen the worktree, so that I do not have to manually restart the agent.
6. As a user, I want exactly one resume per session even when multiple windows or processes observe it, so that I never get duplicate agent processes.
7. As a user, I want a "session restored (new process, same session)" banner on resume, so that I know the conversation continued but the process is new.
8. As a user, I want a failed resume to leave me a usable shell with an explicit error, so that I can recover manually without a silent new-agent fallback.
9. As a user, I want a manually slept session to resume only when I open its tab, so that I control when it relaunches.
10. As a user, I want stale session records to be cleaned up, so that old terminals do not spuriously resurrect.
11. As a user, I want closing a terminal to clear its sleeping record, so that a closed terminal never comes back.
12. As a user, I want revival metadata to persist across Zed updates, so that sessions survive binary replacement.
13. As a user, I want a plain shell that happens to run `omp` not to become a revivable agent session, so that only intentional agent terminals are recoverable.

## Implementation Decisions

- A dedicated **OMP agent terminal** entry kind in the agent panel, separate from terminal threads that reuse `terminal_init_command`. Creation is the source of session identity.
- Zed assigns a per-terminal resume path; launches OMP with `omp --session-dir <path>`; resumes with `omp --resume <path>` (resume path preferred over session id). The resume path is the persisted locator.
- Persisted revival fields on terminal metadata: profile id, opaque resume path, session-boundary state (`live` / `sleeping` / `cleared`), claim key. Never the command line, environment, or credentials.
- Session-boundary transitions: `live` at creation; `sleeping` on OMP process exit or app quit; `cleared` on explicit close or staleness.
- Auto-resume on worktree activation, gated by claim-key fencing (exactly-once), an `automaticResumeBlockedBy` opt-out, a restore-on-tab-open mode for manually slept sessions, and an 18-minute staleness timeout.
- Resumed sessions show a restored banner.
- Resume failure: explicit error + usable shell; no silent new-agent fallback.
- OMP-specific settings block; the resume argv is built by a per-agent function so more adapters can be added later.
- In-process claim keys plus reliance on OMP's own session-file locking for fencing across Zed processes.
- Minimal telemetry: an `Agent Terminal Session Resumed` event.
- Local-only first release; remote/WSL explicitly out of scope.

> Resume argv shape (from prototype/Orca parity):
> ```ts
> get_agent_resume_argv("omp", { key: "session_id", id }, resumePath)
>   => ["omp", "--resume", resumePath?.trim() || id]
> claimKey = `${worktreeId}\0${agent}\0${session.key}\0${session.id}`
> ```

## Testing Decisions

- A good test asserts external behavior, not implementation details: the panel auto-resumes a sleeping session exactly once, or the store persists and clears revival fields.
- **Primary seam (P):** `AgentPanel` GPUI test — on load with a sleeping OMP session, auto-resume runs exactly once, shows the restored banner, and a failed resume leaves a usable shell with an error. Prior art: existing `agent_panel.rs` GPUI tests using `TestAppContext`.
- **Supporting seam (S1):** `TerminalThreadMetadataStore` SQLite tests — migration plus revival-field persistence and delete. Prior art: existing `mod tests` in `terminal_thread_metadata_store.rs`.
- **Supporting seam (S2):** pure resume-argv and session-boundary decision tests — claim-key dedup, newest-wins, staleness timeout. Mirrors Orca's shared-module pattern.

## Out of Scope

- Remote/WSL sessions.
- Other harness adapters (Claude Code, Codex, Gemini).
- Inferred revival from arbitrary foreground programs.
- A full PTY broker that preserves the original process/PID — this spec covers agent-session revival (new process, same logical session) only.
- Replaying arbitrary command history.

## Further Notes

- Model follows Orca v1.4.180's sleeping-session + host-authority design, verified from its `app.asar` source: automatic resume on worktree activation, claim-key fencing, `automaticResumeBlockedBy`, restore-on-tab-open, 18-minute staleness, and a session-restored banner.
- A resumed session is a new process with the same logical session. PID, file descriptors, SSH connections, and REPL memory are not preserved — only the conversation and work context the harness resumes.