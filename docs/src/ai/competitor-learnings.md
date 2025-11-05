# Competitor Feature Survey

This document records a quick survey of notable AI-assisted IDE features from Cursor and Windsurf, alongside opportunities we could explore in Zed.

## Cursor Highlights

- **Inline `/edit` commands** - Purpose-built slash commands such as `/edit`, `/fix`, and `/docstring` let users ask the agent for scoped edits without crafting prompts from scratch.
- **Composer (Cmd/Ctrl K)** - Generates multi-line code completions in-place with diff review, tuned for longer transformations than standard autocomplete.
- **Rules and project memory** - Project-level rules, quick command rules, and automatic context tracking keep the assistant aligned with repo conventions.
- **Agent (Task) mode** - Experimental workflow where users hand a higher-level goal to the agent, receive a plan, and apply file-by-file edits after review.
- **Quick access actions** - Command palette surfacing common AI tasks (explain, comment, test scaffolding) that feed curated prompts into chat.

## Windsurf Highlights

- **Ask vs. Task split** - Two entry points: quick Q&A chats ("Ask") and structured "Task" flows that produce a step-by-step plan before editing.
- **Plan validation loop** - Tasks render a plan, allow pruning or reordering, and show live status as the agent executes each step.
- **Sandbox command runner** - Ephemeral execution environment lets the agent or the user run commands or tests tied to a task without polluting the main workspace.
- **Context packs** - Built-in context selectors (recent files, tests, docs) make it easy to shape the evidence the agent receives.
- **Task history and replay** - Completed tasks can be reopened, inspected, or re-run to apply similar changes elsewhere.

## Opportunities for Zed

1. **AI Quick Actions palette** - Add an `agent.quick_actions` manifest that ships common prompts (Explain selection, Write tests, Improve performance) and exposes them through the command palette and key bindings, mirroring Cursor's curated slash commands.
2. **Guided Task threads** - Extend the Agent panel with an opt-in "Task" thread type that first synthesizes a plan (bullets with edit intent), then executes each step using existing MCP tooling, with checkpoints per step.
3. **Ephemeral run buffers** - Introduce sandboxed run buffers, a temporary workspace (backed by tmp dirs or container volumes) the agent can target when running tests or other commands, inspired by Windsurf's sandbox runner.
4. **Task summaries in history** - Keep lightweight metadata (goal, plan, files touched) for agent threads so users can revisit or duplicate successful flows, converging with Windsurf's task history.

> Next steps: validate scope with design or product, prototype Quick Actions (lowest lift), and spike on Task threads once UX flows are settled.
