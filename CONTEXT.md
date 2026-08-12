# Agent Panel Terminal Revival

Terms around keeping agent-session terminals recoverable across Zed updates and restarts. Model follows Orca's sleeping-session + host-authority design.

## Language

**Agent session**:
A terminal running a TUI coding agent (OMP, Claude Code, Codex, etc.) that can be resumed through a provider locator even after its process exits.
_Avoid_: shell, terminal thread

**Sleeping session**:
An agent session whose process has ended but whose resume locator is retained so a new process can relaunch it.
_Avoid_: dead session, zombie

**Resume locator**:
The minimal provider-owned value that identifies the CLI resume target. For OMP this is a file path Zed assigns and controls; the session id is a fallback.
_Avoid_: command line, session metadata

**Session boundary**:
The lifecycle transition an agent session moves through: live (process running), sleeping (process ended, resumable), cleared (no longer resumable).
_Avoid_: restart, reconnect

**Claim key**:
The identity that fences resume so a given session is relaunched exactly once even if multiple windows or processes observe it.
_Avoid_: lock, idempotency token

**Automatic resume**:
Relaunching a sleeping session on activation without user action.
_Avoid_: auto-restore

**Restore-on-tab-open**:
A manually slept session that resumes only when its tab is opened, not on activation.
_Avoid_: lazy restore