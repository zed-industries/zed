# Zed vs opencode: System Prompt Comparison

A side-by-side comparison of the system prompts used by Zed's native agent (`crates/agent/src/templates/system_prompt.hbs`) and opencode (`packages/opencode/src/session/prompt/*.txt`), based on the `dev` branch of `sst/opencode` at the time of writing.

---

## 1. Architectural difference: one prompt vs many

This is the biggest structural difference, and it shapes everything else.

- Number of base prompts
  - Zed: **1** Handlebars template
  - opencode: **8 model-specific** prompts plus 1 stale (`copilot-gpt-5.txt`)
- Selection strategy
  - Zed: same prompt for every model; conditional sections turn on/off based on which **tools** are available
  - opencode: different prompt selected by **model ID substring match** (claude → anthropic.txt, gpt-4/o1/o3 → beast.txt, gemini → gemini.txt, etc.)
- Templating
  - Zed: Handlebars with `{{#if}}` / `{{#each}}` blocks
  - opencode: plain `.txt` files; environment is appended in code as a separate preamble
- Mode-specific overlays
  - Zed: none at the system-prompt level — modes affect available tools, which then conditionally activate sections
  - opencode: synthetic `<system-reminder>` injections (`plan.txt`, `build-switch.txt`, `max-steps.txt`)

Implication: Zed centralizes behavior and lets the toolset drive what the model sees. opencode hand-tunes a different prompt per model family.

## 2. Length and density

- Zed `system_prompt.hbs`: ~234 lines, ~12KB. About a third is one topic — a custom code-block syntax.
- opencode `anthropic.txt`: ~80 lines, ~5KB. Terse, with worked TodoWrite examples.
- opencode `beast.txt` (gpt-4/o1/o3): ~150 lines, much more "keep going, never stop" framing.
- opencode `gemini.txt`: ~180 lines, the most workflow-heavy with an explicit "New Applications" workflow.

Zed's prompt is the longest single prompt of the bunch, but most of the bulk is the code-block formatting rules.

## 3. Identity and persona

- Self-name
  - Zed: "a highly skilled software engineer" — no product name
  - opencode: "OpenCode, the best coding agent on the planet" (anthropic) or "opencode, an interactive CLI tool" (default)
- Surface type
  - Zed: implicitly GUI — no CLI references
  - opencode: explicitly CLI — "rendered in a monospace font", "displayed on a command line interface"
- Pronoun: both use second/first person.

Zed reads as model-neutral and surface-neutral. opencode leans into a specific tool identity and CLI rendering context.

## 4. Tone and verbosity

A striking divergence.

- Zed — almost no instruction on response length
  - "Be conversational but professional"
  - "Refrain from apologizing all the time"
  - That's it. No 4-line cap, no examples.
- opencode `default.txt` — extreme terseness mandate
  - "You MUST answer concisely with fewer than 4 lines"
  - Examples like `user: 2 + 2 / assistant: 4`
  - "One word answers are best"
- opencode `anthropic.txt` — softer
  - "Your responses should be short and concise"
- opencode `beast.txt` — opposite extreme
  - "Your thinking should be thorough and so it's fine if it's very long"

opencode is bipolar across model families. Zed stays neutral.

## 5. Tool-use philosophy

- Parallel tool calls
  - Both have nearly identical wording: independent calls in parallel, dependent calls sequential. Clearly the same lineage.
- Placeholders
  - Both: "Never use placeholders or guess missing parameters" — verbatim match.
- Timeouts
  - Zed: explicit, "specify `timeout_ms`" for long-running commands
  - opencode: not mentioned
- Off-tools fallback
  - Zed: a whole alternate `{{else}}` branch for "no tools available" mode
  - opencode: none — assumes tools always available
- Tool selection vs Bash
  - Zed: implied via `grep` / `find_path` guidance
  - opencode: explicit — "Read instead of cat/head/tail, Edit instead of sed/awk"

## 6. Task management / planning

- Zed
  - A "Planning" section, conditionally included only if the `update_plan` tool is enabled
  - Detailed criteria for when to use a plan vs not, when to mark `in_progress`
  - Key restraint: "Do not use plans for simple or single-step queries"
  - Plans are opt-in via tool availability
- opencode `anthropic.txt`
  - TodoWrite is treated as **mandatory** and constant
  - "Use these tools VERY frequently"
  - "If you do not use this tool when planning… that is unacceptable"
  - "IMPORTANT: Always use the TodoWrite tool to plan and track tasks"
  - Includes worked examples showing the model literally announcing additions to the todo list

Tonal difference: Zed nudges, opencode pressures.

## 7. Code-block formatting

This is where Zed is radically different from every other agent prompt.

- Zed
  - ~80 lines mandating a custom fence syntax: ```` ```path/to/Something.blah#L123-456 ````
  - The path comes immediately after the opening backticks instead of a language
  - Three "good" examples
  - Four "bad_example_do_not_do_this" anti-examples
  - Explicit warning: "if you ever find yourself writing three backticks followed by a language name, STOP!"
- opencode
  - Standard fenced markdown with a language tag
  - Emphasizes the `file_path:line_number` reference pattern instead

Why? Zed's GUI parses code blocks for navigation and editing — the path is structural, not decorative. opencode renders plain markdown to a terminal.

## 8. Code references in prose

- Zed: implicit, uses the code-block format above
- opencode (anthropic + default): `file_path:line_number`, with a worked example like `src/services/process.ts:712`

## 9. Model and environment awareness

- Zed injects
  - Operating System
  - Default Shell
  - Model name
  - Worktree paths
- opencode appends a richer block built in code
  - Model ID and provider ID
  - Working directory
  - Workspace root folder
  - Whether it's a git repo
  - Platform
  - Today's date

opencode is more aggressive: it includes the date and git status, which matters for staleness reasoning. Zed omits both.

## 10. User & project rules

- Zed
  - Structured rules block at the end of the prompt
  - Per-worktree project rules (CLAUDE.md, .rules)
  - User-defined rules with optional titles
  - Wrapped in 6-backtick fences to avoid collision with normal code
- opencode
  - Not in the base prompt at all
  - Loaded via `instruction.ts` (AGENTS.md, README) into separate context

## 11. Things only opencode says

- "Professional objectivity" (anthropic.txt): prioritize accuracy over validating the user's beliefs; disagree when necessary
- Explicit `<system-reminder>` semantics: "authoritative system directives that you MUST follow"
- WebFetch redirect handling and Task-tool-for-search bias
- "DO NOT ADD ANY COMMENTS unless asked" (default.txt)
- "NEVER create files unless absolutely necessary"
- Mode reminders injected at runtime as `<system-reminder>` blocks

## 12. Things only Zed says

- The custom code-block fence syntax (no analog in opencode)
- "Before you read or edit a file, you must first find the full path. DO NOT ever guess a file path!"
- Multi-agent delegation section (only if `spawn_agent` enabled): coordinating sub-agents, assigning disjoint write scopes
- Calling External APIs section: when to pick versions, when API keys matter
- Fixing Diagnostics: "Make 1-2 attempts at fixing diagnostics, then defer to the user" — a Zed-editor concession
- No-tools fallback branch with an alternate persona

## 13. Title and summary prompts

- Title generation
  - Zed: `summarize_thread_prompt.txt`, 5 lines, "Generate a concise 3-7 word title…"
  - opencode: no separate file; title agent runs with its own prompt, user message is `"Generate a title for this conversation:\n"`
- Summary
  - Zed: `summarize_thread_detailed_prompt.txt`, generic 5-step markdown structure
  - opencode: `compaction.ts` SUMMARY_TEMPLATE — engineered to preserve agent state for resumption (Goal / Constraints / Progress / Decisions / Next Steps / Critical Context / Relevant Files)

opencode's compaction is significantly more sophisticated.

## 14. Claude Code lineage overlap

Both prompts clearly inherit phrasing from Anthropic's Claude Code system prompt. Shared lines:

- The parallel tool-call paragraph
- "Never use placeholders or guess missing parameters"
- The `<system-reminder>` framing (opencode keeps it explicitly; Zed omits it)
- Code reference pattern (opencode keeps `file_path:line_number`; Zed swapped to its custom fenced form)

opencode's `anthropic.txt` is much closer to a stock Claude-Code-style prompt. Zed has diverged more.

## 15. opencode's model selection logic (for context)

opencode chooses a prompt by substring-matching the model ID:

- `gpt-4` / `o1` / `o3` → `beast.txt` (aggressive "keep going forever" framing)
- `gpt` + `codex` → `codex.txt`
- `gpt` (any other) → `gpt.txt` — this is what GPT-5.4, GPT-5.5, and any future GPT-5.x receive, since they don't match `gpt-4` / `o1` / `o3`
- `gemini-` → `gemini.txt`
- `claude` → `anthropic.txt`
- `trinity` → `trinity.txt`
- `kimi` → `kimi.txt`
- otherwise → `default.txt`

Note: `copilot-gpt-5.txt` exists in the repo but is not referenced from `system.ts` — appears to be dead code.

## Summary

- Zed: one templated prompt, model-agnostic, GUI-aware, custom code-block format as centerpiece, conditional tools, no hard verbosity caps, restrained tone. Assumes a smart Claude-class model and lets tool availability shape behavior.
- opencode: eight model-tuned prompts, CLI-aware, leans hard on TodoWrite for Claude and "keep going forever" for GPT-4/o1/o3, uses runtime `<system-reminder>` injections for mode switching, varies tone and verbosity per model family. More opinionated and product-branded.
