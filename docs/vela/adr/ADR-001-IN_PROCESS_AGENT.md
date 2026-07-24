# ADR-001: Use Zed's in-process Rust agent

- Status: Accepted
- Date: 2026-07-23
- Zed baseline: `6297c88f428a99741a7bfb33f31dfe98123bb8e4`

## Context

CodeIDE needs structured chat, selected-code context, language-server tools, reviewable code changes, model configuration, and recoverable sessions. The AI runtime must not run as a child process. LSP, DAP, terminals, compilers, and formatters remain external processes because their protocols require that model.

The inspected Zed baseline already contains:

- `crates/agent`: native agent, threads, persistence, permissions, and semantic tools;
- `crates/agent_ui`: conversation UI, context mentions, agent diff, and model selection;
- `crates/language_model` and `crates/language_models`: model abstractions and compatible providers;
- `crates/project`: direct access to buffers, LSP, Git, and diagnostics;
- `crates/git_ui/src/branch_diff.rs`: merge-base branch diff and base picker;
- `crates/agent_ui/src/agent_diff.rs`: existing agent change review UI.

Pi is TypeScript/Node.js. Embedding or spawning it would violate the runtime constraint and duplicate capabilities already present in Zed.

## Decision

CodeIDE will extend Zed's native Rust agent in process. Agent tools call `Entity<Project>` and existing Rust services directly. Pi remains a design reference only and is not a runtime dependency.

Implementation must prefer extending existing Zed crates over creating parallel `codeide_*` crates when an equivalent abstraction already exists. New crates require a clear ownership boundary that cannot fit an existing crate.

ACP support may remain available for compatibility, but it is not the default CodeIDE agent path.

## Initial ownership map

| Requirement | Primary existing area |
|---|---|
| Agent loop and thread state | `crates/agent` |
| Chat and context UI | `crates/agent_ui` |
| Model providers and capabilities | `crates/language_model`, `crates/language_models` |
| Definitions, references, diagnostics | `crates/agent/src/tools`, `crates/project` |
| Agent change review | `crates/agent_ui/src/agent_diff.rs` |
| Branch comparison base | `crates/git_ui/src/branch_diff.rs` |
| Settings UI/store | `crates/settings`, `crates/settings_ui` |
| File tree | `crates/project_panel` |

## Consequences

### Positive

- No Node.js or Pi runtime process.
- Direct access to buffers, anchors, LSP state, Git, and GPUI.
- Existing Zed behavior and tests can be reused.
- Fewer protocol conversion and process-recovery paths.

### Negative

- Agent failures share the IDE process and need strict task, cancellation, and panic boundaries.
- Changes to core Zed crates increase upstream merge cost.
- Pi-specific session behavior must be selectively implemented in Rust when Zed lacks it.

## Validation tasks

1. Run one native model stream and cancel it without blocking GPUI.
2. Add a selection to a thread and verify anchor behavior after edits.
3. Produce an agent edit and open the existing Agent Diff.
4. Invoke definition and diagnostics tools through `Entity<Project>`.
5. Document gaps before adding new crates.
