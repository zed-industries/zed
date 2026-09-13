# AIDE — an agentic IDE without chat

## Thesis

Chat is removed as a **human input surface**, not as an agent capability. The swarm keeps
full agentic depth: parallel models, tool use, subagents, worktrees, memory. What goes away
is the message box.

The human has exactly three input surfaces, all durable and all git- or issue-backed:

1. **Code and comments** — a `//?` directive is a request for work.
2. **The board** — backlog / in progress / done, which the AI triages and expands.
3. **Accept/reject on diffs** — the strongest and highest-bandwidth signal in the system.

The agents have three output surfaces:

1. **Proposals** — buffer diffs reviewed in place, per hunk.
2. **Issues** — created, triaged, decomposed, closed.
3. **Memory** — durable notes that evolve and feed future work.

Everything else (transcripts, tool calls, terminals) is **inspectable but not writable**.

> **The line that must not drift:** an inspectable transcript is not chat *as long as there
> is no input affordance attached to it*. The moment you can type into an agent's log, chat
> is back. Inspection surfaces are read-only. Steering happens through code, issues, and
> review.

---

## Layer model

```
L4  Human      code + //? directives | board | in-file diff review | read-only inspectors
                     │                    │                  ▲
                     ▼                    ▼                  │
L3  Orchestrator   long-lived "plotter". Observes repo, triages issues, decomposes,
                   schedules, spawns. Big model, slow cadence, own scratchpad workspace.
                     │                                       │
                     ▼                                       │
L2  Workers        one per in-progress issue. Own git worktree. Tool loop.
                   Produces a Proposal. Small/fast model + injected research.
                     │                                       │
                     ▼                                       │
L1  Researchers    short-lived, read-only, no write tools. Answer a specific question,
                   write findings to Memory. Cheap and parallel.
                     │                                       │
                     ▼                                       │
L0  Substrate      Issue store · Memory store · Worktree pool · ActionLogs · Governor ──┘
```

The arrow back up from L0 to L4 is the point: **every layer's output lands in a human
surface without a conversation happening.**

---

## The unifying primitive: Issue == Intent

Do not build a separate in-memory work queue alongside an issue tracker. They are the same
thing, and collapsing them is what makes the product coherent.

```rust
struct Issue {
    id: IssueId,                       // local ULID, optionally mirrored to a GH issue number
    origin: IssueOrigin,
    status: Status,                    // Backlog | Triaged | InProgress | Proposed | Done | Rejected
    title: String,
    body: String,                      // markdown; agents append findings here
    anchor: Option<CodeAnchor>,        // buffer + Anchor range, for directive-origin issues
    parent: Option<IssueId>,           // decomposition tree
    blocked_by: Vec<IssueId>,
    claim: Option<Claim>,              // which worker owns it, and when the lease expires
    worktree: Option<WorktreeId>,
    proposal: Option<ProposalId>,
}

enum IssueOrigin {
    Directive { buffer: PathBuf },     // a //? comment — fast-path, skips triage
    Diagnostic { .. },                 // LSP error, failing test
    Human,                             // typed on the board
    Agent { parent: IssueId },         // orchestrator decomposition
    Github { number: u64 },            // imported
}
```

`Status` is the board. Triage is the orchestrator moving things through it and splitting
them. A `//?` directive is just an issue that enters at `InProgress` with a code anchor.

**Storage:** local-first, in `.aide/issues/*.md` (frontmatter + body), git-ignored or
committed as the team prefers. GitHub sync is a **one-way-reconcilable mirror**, not the
source of truth — you cannot make a swarm's inner loop depend on network round-trips to
api.github.com.

**GitHub integration:** shell out to `gh` CLI first. `crates/git_hosting_providers/src/providers/github.rs`
is permalink/blame-oriented and has no Issues API; writing one is real work and `gh` handles
auth, enterprise hosts, and rate limits for free. Revisit only if `gh` becomes a bottleneck.

---

## What Zed already gives you

Verified against the tree; this is why the build is mostly rewiring.

| Need | Existing | Location |
|---|---|---|
| Proposal + per-hunk review engine | `ActionLog` — baselines buffers, maintains `BufferDiff`, `keep_edits_in_range` / `reject_edits_in_ranges` / `keep_all_edits` / `stale_buffers` | `crates/action_log/src/action_log.rs` |
| In-file review UI | `AgentDiff` — diff overlay in open editors, keep/reject keybindings, toolbar | `crates/agent_ui/src/agent_diff.rs` |
| Parallel agents in worktrees | `create_sibling_thread` with `use_new_worktree` / `base_ref` | `crates/agent/src/thread.rs:782` |
| Blocking subagents | `create_subagent` → `SubagentHandle` | `crates/agent/src/thread.rs:746` |
| Heterogeneous models per agent | `AvailableAgents` / `AvailableModel`, `SiblingThreadRequest::model` | `crates/agent/src/thread.rs:844` |
| Git worktree lifecycle | `create_worktree`, `worktree list/add/remove/repair` | `crates/git/src/repository.rs:2141` |
| Agent tool suite (~30) | grep, find_path, go_to_definition, find_references, diagnostics, edit_file, terminal | `crates/agent/src/tools/` |
| Agent-authored terminals | `create_terminal` → `acp_thread::Terminal` | `crates/acp_thread/src/terminal.rs` |
| Read-only inspector tab precedent | `AcpTools` is a workspace `Item` rendering live protocol traffic | `crates/acp_tools/src/acp_tools.rs:673` |
| Loop-prevention primitive | `BufferEditSource::{User, Agent, Remote}` on `BufferEvent::Edited` | `crates/language/src/buffer.rs:301` |
| Retrieval | BM25, git-log recency, LSP definitions, syntactic excerpt expansion | `crates/edit_prediction_context/` |
| Streaming edits into a buffer | `streaming_diff`, `buffer_codegen` | `crates/streaming_diff/`, `crates/agent_ui/src/buffer_codegen.rs` |
| Cursor-local prediction plug-in | `EditPredictionDelegate` trait; `EditPrediction::Jump` crosses files | `crates/edit_prediction_types/src/edit_prediction_types.rs:168` |
| Markdown instruction bundles | `agent_skills`, `prompt_store` | `crates/agent_skills/agent_skills.rs` |

`ThreadEnvironment` is a **trait**. Implementing your own is how you take over spawning,
worktree assignment, and budget enforcement without forking the agent loop itself.

---

## The three hard problems

Everything else is execution. These three decide whether the product works.

### 1. Worktree proposals must land in the main workspace

A worker edits files in `.aide/worktrees/issue-1234/`. Your editor has the main checkout
open. If reviewing means switching workspaces, the in-file review thesis is dead — you're
back to context-switching per agent, which is what the board was supposed to prevent.

**Resolution:** the worker's result is transported as a patch and **replayed into the main
workspace's buffers as agent-sourced edits through a per-proposal `ActionLog`.** The human
reviews in the file they already have open, in the tree they already trust. Accept applies;
reject discards; the worktree is then reaped.

```
worker in worktree ──▶ commit/diff ──▶ Proposal{patch, base_oid} ──▶ replay into main buffers
                                                                      via ActionLog(agent edits)
                                                                                │
                                                          base moved? ──yes──▶ re-run in fresh worktree
                                                                    └──no───▶ in-file review
```

The `base_oid` check is what makes this safe. If main has moved under the proposal, do not
attempt a merge — re-run the worker on the new base. Re-running is cheap; a silently
mis-rebased AI patch is expensive.

### 2. Concurrent proposals in one file

`AgentDiff` assumes a single active thread per workspace. Two workers touching the same file
produce one merged, unattributable diff — the human cannot tell which agent proposed what,
and keep/reject becomes meaningless.

**v1:** a **file-level lease** in the substrate. An issue claims the paths it will touch;
overlapping issues queue. Coarse, correct, and it makes the board honest about parallelism.
Multi-overlay diffs with per-agent attribution is a real project — take it on in v2 with
evidence, not up front.

### 3. The governor

Parallel frontier models, spawned autonomously, triggered by comments and diagnostics, is an
unbounded spend generator. This is not a polish item; it is load-bearing from the first
milestone.

```rust
struct Governor {
    max_concurrent_workers: usize,        // hard cap, default ~3
    max_concurrent_researchers: usize,    // ~8, cheap models
    hourly_token_budget: TokenBudget,     // per tier
    per_file_cooldown: Duration,          // anti-thrash
    per_issue_turn_limit: u8,
    kill_switch: bool,                    // status bar, one click, kills every task
}
```

Every spawn goes through it. Spend is visible in the status bar at all times. The kill switch
is not a menu item.

**And the failure that will actually happen first:** the agent writes a `//↳` comment, the
directive watcher sees a comment change, fires a new issue, forever, overnight, at frontier
rates. **Filter every signal on `BufferEditSource::Agent`.** Write this filter before you
write the first model call.

---

## Agent-to-agent communication

Agents still need to talk. That channel is *not* human-facing, and is deliberately narrow:

- **Researcher → Worker:** via **Memory**. Researchers cannot write code, only notes.
- **Orchestrator → Worker:** via the **Issue** (body, acceptance criteria, injected memory refs).
- **Worker → Orchestrator:** via **Issue status transitions** and the Proposal.
- **Worker → Worker:** *nothing direct.* Coordination goes through the orchestrator and the
  issue graph. Direct worker-to-worker messaging reintroduces a conversation substrate with
  none of chat's reviewability.

## Memory

Plain markdown files under `.aide/memory/`, one topic per file, with frontmatter for scope
(path glob, subsystem, or global) and provenance (which issue/agent wrote it, when).

Files, not a hidden database — the human can read, edit, delete, and diff them, which matches
the ethos of the rest of the product. Retrieval reuses `edit_prediction_context` scoring plus
frontmatter scope matching. Staleness is handled by recording the commit a note was written
against and decaying confidence as the referenced paths change.

`agent_skills` already loads markdown instruction bundles; reuse its loader rather than
writing a parallel one.

---

## Crate map

**Cut from the UI, keep as libraries:** `agent_panel`, `conversation_view`, `message_editor`,
the ACP-thread rendering stack. The `agent` crate stays — its tools and model plumbing are
the point.

**Untouched:** `editor`, `project`, `language`, `language_model`, `buffer_diff`, `action_log`,
`streaming_diff`, `edit_prediction_context`, `git`, `terminal`.

**New:**

```
crates/aide_substrate/   Issue store, Memory store, worktree pool, leases, Governor
crates/aide_signal/      buffer/diagnostic/save/test watchers → Signal  (agent-source filtered)
crates/aide_directive/   //? sigil parsing, anchored directive index, //↳ response writing
crates/aide_orchestrator/ the plotter: observe → triage → decompose → schedule → spawn
crates/aide_worker/      per-issue agent; custom ThreadEnvironment impl; worktree lifecycle
crates/aide_proposal/    patch transport, base_oid validation, replay into main ActionLogs
crates/aide_review/      agent_diff.rs forked → ActionLog-driven, thread-free
crates/aide_ui/          board Item, swarm inspector Item, governor status bar, editor Addon
```

`editor::Addon` (`crates/editor/src/editor.rs:762`, registered at `:10643`) attaches per-editor
UI and key context without touching editor internals. Lean on it to keep the fork thin.

---

## Fork discipline

Thin fork, weekly rebase. Confine edits to new crates plus init sites
(`crates/zed/src/main.rs:700-779`, `crates/zed/src/zed.rs:6049`) behind a settings flag.

`agent_diff.rs` is the one file requiring genuine hand-merging — budget for it every rebase.
Its coupling to `AcpThread` is shallow (title at `:540`, telemetry at `:305`, lifecycle events
at `:1458-1499`); everything else already flows from `ActionLog`, which is why the fork is
tractable.

---

## Milestones

Sequencing matters more than usual here, because **every layer above L0 produces proposals
that land in the same review surface.** If in-file review without chat doesn't feel good,
nothing built on top of it is worth anything.

| M | Deliverable | Proves |
|---|---|---|
| **M0** | Fork; pull `AgentPanel` from the dock; fork `agent_diff.rs` to be `ActionLog`-driven. Hand-construct an `ActionLog`, apply edits, review them. **No model involved.** | The core thesis, in days, for free |
| **M1** | `//?` → context → one model call → streamed edits → in-file review. Single file, one at a time. Governor + agent-source filter in place. | The product is real |
| **M2** | `//↳` rationale and `//↳?` question comments; editing an answer retriggers | The loop closes without chat |
| **M3** | Substrate: issue store, board Item, manual triage. Directives become issues. | Work is durable |
| **M4** | Orchestrator: autonomous triage, decomposition, scheduling. Still single-worker. | The plotter earns its keep |
| **M5** | Workers in worktrees + proposal replay (hard problem #1) + file leases (#2) | Parallelism |
| **M6** | Researchers + memory; injected context for cheap workers | The swarm compounds |
| **M7** | Swarm inspector + terminal tabs; GitHub sync via `gh` | Trust and interop |
| **M8** | Custom `EditPredictionDelegate` aware of nearby directives | Tier 0 joins the system |

M0 and M1 hold the risk. M5 holds the complexity. Everything between is substrate work with
known shape.

---

## Open questions

- **Do directives survive accept?** Keeping `//?` in the source is honest provenance and
  makes files self-documenting about AI authorship; stripping keeps the code clean. Probably
  a setting, defaulting to strip `//?` and keep `//↳` rationale.
- **Does the orchestrator get a workspace?** It wants scratchpad files and a place to think.
  A hidden worktree it can write freely (never proposed to the human) is likely right.
- **Board granularity vs. GitHub.** Local issues will be finer-grained than anything you'd
  want on a public tracker. Sync probably needs a `visibility:` frontmatter field rather than
  mirroring everything.
