# Spec: Improve Zed Native Agent System Prompt (Multi-Model)

A spec following Addy Osmani's spec-driven-development methodology. The resulting prompt must work acceptably across Claude (Anthropic), GPT-5 family (OpenAI), and major open-weight families (Llama, Qwen, DeepSeek, Mistral, Codestral, and reasoning variants like DeepSeek-R1 / QwQ).

---

## Phase 1: Specify

### Assumptions

Surfaced explicitly so they can be challenged before any work starts:

- The system prompt must produce acceptable behavior across Anthropic, OpenAI, and a range of open-weight models. No single vendor's idiom takes priority; the prompt is engineered to the lowest common denominator while not over-taxing the strongest models.
- The prompt is rendered server-side via Handlebars in `crates/agent/src/templates.rs` and reaches the model verbatim (after any vendor-specific role-message wrapping by the harness).
- The Zed GUI relies on the existing path-fenced code-block syntax. The format is structural for the editor, not stylistic, and cannot be replaced without GUI changes. All target models can learn it from a small number of examples.
- Prompt caching (Anthropic), automatic prefix caching (OpenAI), and prefix caching in vLLM / TGI (open-weight) all reward a stable static prefix. The mechanism varies but the principle is universal.
- Model-specific conditional branches in Handlebars (`{{#if (eq family 'anthropic')}}`) are out of scope unless measurement justifies them. Default to one prompt for all.
- Changes to the summary / title prompts and adjacent edit-prompt files (`edit_file_prompt_*.hbs`, `create_file_prompt.hbs`, `terminal_assistant_prompt.hbs`, `content_prompt_v2.hbs`) are out of scope; tracked in a separate spec.
- Tool definitions and tool descriptions are out of scope; this spec covers only `system_prompt.hbs`.
- The harness (Zed's request-construction code) is responsible for vendor-specific message format, role wrapping, tool-call format, and prompt-caching breakpoints. Spec changes that need new harness behavior are flagged explicitly.

### Model Matrix

Explicit list of in-scope models and their relevant constraints. The prompt must produce acceptable results on each.

- **Claude Opus 4.7** — primary reasoning target. Adaptive thinking enabled. Strong literal instruction-following. Sensitive to over-emphatic language ("CRITICAL", "MUST") on 4.5+. Strong with XML tags. 200k context.
- **Claude Sonnet 4.6** — fast workhorse. Same family behaviors as Opus 4.7. 200k context.
- **Claude Haiku 4.5** — small, fast. Benefits from clearer structure than Opus does. 200k context.
- **GPT-5 family (5.4, 5.5, future variants)** — strong instruction adherence ("surgical precision" per OpenAI). Reasoning effort and verbosity are API-controlled, separate from prompt. Markdown is off by default in the API; the prompt should not assume rich markdown rendering. Reads XML specs well. 400k context (high tier).
- **GPT-5-codex** — code-tuned variant. Receives a different opencode prompt today; under this spec it gets the same prompt but should perform equivalently.
- **OpenAI o-series reasoning models (o1, o3, future)** — reasoning models. OpenAI's published guidance: "perform best with straightforward prompts. Some prompt engineering techniques, like instructing the model to 'think step by step,' may not enhance performance (and can sometimes hinder it)." The prompt must not embed chain-of-thought scaffolding.
- **Llama 3.3 / 4 (70B+)** — strong instruction-following. Handles XML and Markdown. Tool use via Hermes-style or vendor-specific templates. 128k context.
- **Qwen 3 (32B / 72B / 235B)** — strong code and tool use. Trained on multi-format tool calling. Reasoning variant exists (QwQ). 128k+ context.
- **DeepSeek-V3** — non-reasoning, strong code. 128k context.
- **DeepSeek-R1** — reasoning model, idiosyncratic; treat like o-series for chain-of-thought guidance.
- **Mistral Large / Codestral** — terse by default; benefits from explicit structure. 128k context.
- **Smaller open-weight models (7B–13B)** — best-effort target only. Likely to ignore complex instructions, may need shorter prompt variant. Out of strict scope for this v1 but should not be actively broken.

Capabilities the prompt MUST NOT assume:

- Adaptive / interleaved thinking (Opus 4.7 has it; many do not).
- Native parallel tool calls (most do; smaller open-weights may not).
- Native XML tool-call format (varies by harness).
- Markdown rendering on the client (off by default for OpenAI API; on for Claude).
- Date/training-cutoff awareness (varies wildly across vendors and open-weight releases).

### Objective

**Purpose.** Improve the quality of Zed's native agent's responses on coding tasks across the model matrix above by aligning the system prompt with cross-vendor published best practices, while preserving Zed-specific guarantees (custom code-block format, tool-conditional sections, multi-agent guidance, no-tools fallback).

**Users.** Every Zed user invoking the native agent, regardless of which model they have configured. Secondary users: Zed engineers maintaining the prompt; users running open-weight models locally via Ollama / llama.cpp / LM Studio.

**Success definition.** Equal or better pass rate on Zed's internal agent eval suite across the model matrix, with reduced static-prefix token count, no regressions on a fixed manual smoke-test set run against three model families, and preserved prefix-caching integrity.

### Tech Stack / Context

- File: `crates/agent/src/templates/system_prompt.hbs`
- Rendering: Handlebars via `crates/agent/src/templates.rs`
- Invocation: `crates/agent/src/thread.rs` (`build_request_messages` around line 3019)
- Available context variables today: `available_tools`, `worktrees`, `has_rules`, `user_rules`, `os`, `shell`, `model_name`
- New context variables proposed: `today`, `is_git_repo` (per worktree), `model_family` (one of: `anthropic`, `openai`, `llama`, `qwen`, `deepseek`, `mistral`, `other`) — only added if measurement justifies model-aware branches
- Helper: custom `contains` for tool-availability checks
- Test entry point: `./script/clippy`, `cargo test -p agent`

### Commands

- Build / lint: `./script/clippy` (per Zed CLAUDE.md — never `cargo clippy`)
- Test: `cargo test -p agent`
- Render prompt locally: `cargo test -p agent test_system_prompt_rendering -- --nocapture` (add this test if it does not exist)
- Token-count measurement: pipe rendered output through tokenizers for each target family — `cl100k_base` (OpenAI), Claude's tokenizer endpoint (Anthropic), and the SentencePiece / tokenizer.json for the dominant open-weight model. Record all three.
- Cross-model smoke test: a script that fans out the smoke-test prompts to one model per family and writes outputs to disk for human review.
- Eval (if available): `script/agent-eval --suite=core --models=opus-4-7,gpt-5,llama-3-70b,qwen-3-72b`

### Project Structure

Files in scope:

- `crates/agent/src/templates/system_prompt.hbs` — primary edit target
- `crates/agent/src/templates.rs` — rendering, helper registration; new context fields
- `crates/agent/src/thread.rs` — assembly site; cache-breakpoint review
- `tests/system_prompt_smoke_set.md` — new file capturing 12 smoke-test prompts (10 categories + 2 model-edge-case prompts)
- `tests/system_prompt_smoke_results/` — new directory holding per-model output snapshots

Files out of scope (explicit, to prevent scope creep):

- `crates/agent_settings/src/prompts/summarize_thread*.txt`
- `crates/agent/src/templates/edit_file_prompt_*.hbs`
- `crates/agent/src/templates/create_file_prompt.hbs`
- `assets/prompts/content_prompt*.hbs`
- `assets/prompts/terminal_assistant_prompt.hbs`
- Tool definitions and tool descriptions

### Code Style

Conventions to mimic from the existing template, plus deliberate choices for cross-model parseability:

- Section headers as `##` Markdown — preserved for human editability of the template.
- **Examples wrapped in `<example>` and `<bad_example_do_not_do_this>` XML tags** — already the pattern; extended to all worked examples added by this spec. Anthropic, OpenAI's GPT-5 guide, and most open-weight models all handle XML tags well; this is the most cross-portable structural marker.
- **Multi-line authoritative blocks** (rules from CLAUDE.md / .rules, user rules) wrapped in `<project_rules>` / `<user_rules>` XML tags, replacing the current six-backtick fences. XML reduces ambiguity when project rules themselves contain Markdown or backticks.
- Conditional sections continue to use `{{#if (gt (len available_tools) 0)}}` / `{{#if (contains available_tools 'tool_name')}}`.
- Loops continue with `{{#each worktrees}}`.
- Inline variables remain `{{os}}`, `{{shell}}`, `{{today}}`, `{{model_name}}`.
- Naming: new helpers in `templates.rs` follow the existing `contains` style; new context fields are snake_case.

### Testing Strategy

Three layers, in increasing cost order. Each layer is repeated per model family.

- **Static checks.** A unit test in `templates.rs` that renders the prompt with representative inputs and asserts:
  - Exactly one occurrence of the path-fenced code-block warning
  - Dynamic context block appears below the `<!-- STATIC PREFIX ENDS HERE -->` sentinel
  - Static-prefix token count within ±5% of recorded baseline (measured for each tokenizer in the matrix)
  - Valid Handlebars rendering for `available_tools ∈ {[], [grep], [grep, update_plan], [grep, update_plan, spawn_agent]}`
- **Cross-model smoke-test set.** Twelve prompts in `tests/system_prompt_smoke_set.md` run manually (or via script) against at least one model per family before and after each phase. Outputs recorded in `tests/system_prompt_smoke_results/<phase>/<model>/<prompt-id>.md`. Categories:
  - bug-fix (single-file)
  - feature (multi-file)
  - refactor
  - explore-and-explain (no edits)
  - multi-file edit
  - planning (uses `update_plan` if available)
  - parallel reads (5+ files)
  - no-tools mode
  - custom-rules respect (CLAUDE.md present)
  - fail-and-recover (a failing tool call)
  - **(new)** small-context-budget (run on a model with 32k context to verify the prompt does not crowd it)
  - **(new)** non-reasoning model (run on DeepSeek-V3 or similar to verify the prompt does not assume thinking)
- **Eval harness.** Run the internal agent eval at end of each phase across at least three families. Block merge if pass rate drops by more than 1 percentage point on any single family.

### Boundaries

**Always do**

- Preserve the path-fenced code-block format and its strictness.
- Preserve tool-conditional sections; never inline tool-specific guidance unconditionally.
- Preserve the `{{else}}` no-tools fallback branch.
- Keep the prompt model-agnostic in structure by default; conditional sections key off tools and rules, not model identity.
- Place all dynamic / per-conversation content (today, worktrees, model_name, user_rules, is_git_repo) below a single `<!-- STATIC PREFIX ENDS HERE -->` marker so the cacheable prefix is stable across all vendor caching schemes.
- Use OpenAI's prescribed file-reference format for inline references: `path:line[:column]` or `path#Llinen[Ccolumn]`, no URIs, no line ranges. This format is unambiguous for all target models.
- Wrap worked examples in `<example>` XML tags for cross-model parseability.
- Phrase rules positively when both forms are equivalent and the positive form is no less specific.
- Prefer specific, observable instructions ("read the file before answering questions about its contents") over abstract principles ("be careful").

**Ask first**

- Adding XML structural tags around major sections beyond examples and rules (changes parseability, may help some families more than others — measure).
- Removing whole sections such as `## Calling External APIs`, `## Fixing Diagnostics`, or `## Multi-agent delegation`.
- Introducing model-family-aware conditional branches (`{{#if (eq model_family 'anthropic')}}`) — only after measurement justifies.
- Adding instructions that depend on adaptive / interleaved thinking.
- Adding chain-of-thought scaffolding ("think step by step", "first reason about X then…") — most reasoning models perform worse with this.
- Replacing the summary / title prompts (separate spec).
- Adding a Reflexion-style failure-reflection paragraph (item 17 from prior research — measure first).

**Never do**

- Add product-self-branding ("you are the best coding agent on the planet").
- Add vendor-specific identity ("you are Claude", "you are GPT") — the prompt is shared.
- Add emotional-appeal phrasing ("the user is blind", "you must iterate forever", "this is unacceptable") — measured worse by Aider; likely worse universally.
- Add per-request data (request IDs, session IDs, exact timestamps with seconds) above the static-prefix marker — invalidates caching everywhere.
- Add mandatory TodoWrite-style pressure ("you MUST always use this tool, you may forget important tasks otherwise") — Anthropic warns against this for Claude 4.5+; smaller open-weight models may comply but produce noise.
- Add chain-of-thought instructions for reasoning models (universally discouraged across OpenAI, DeepSeek-R1 docs, QwQ docs).
- Bypass the GUI's code-block format.
- Inline project metadata, file lists, or dependency trees into the system prompt (Cursor's "dynamic context discovery" — let tools fetch).
- Add Markdown features (tables, footnotes) that some clients render as plain text — keep to bullets, headers, fenced code blocks, inline code.

**Tension noted, not resolved here.** Anthropic prefers softer imperatives; smaller open-weight models often need explicit `MUST` to comply. The compromise: reserve strong imperatives for genuinely high-stakes rules (security, secrets, destructive commands, the GUI code-block format) and use plain instructions for the rest. Re-evaluate per category after eval results.

### Success Criteria

Specific and measurable:

- Imperative-word density (count of `NEVER`, `MUST`, `CRITICAL`, `ALWAYS`, `IMPORTANT`) reduced from baseline by ≥ 40% (less aggressive than Anthropic-only spec; preserves emphasis where weaker models need it).
- Negative-phrasing instances (`do not`, `don't`, `never`) reduced by ≥ 25%.
- Static-prefix token count reduced by ≥ 10% measured on the Claude tokenizer; verified within ±5% of that figure on `cl100k_base` and the Llama tokenizer (acceptable variance across tokenizers).
- Code-block formatting section reduced from ~80 lines to ~25 lines.
- Worked examples in `<example>` tags increased from 0 (in agent-behavior sections) to ≥ 3.
- Internal eval pass rate equal or higher than baseline on at least three model families (Anthropic, OpenAI, one open-weight). No more than 1 percentage-point regression on any single family in any single category.
- Manual smoke-test set: 12/12 prompts produce acceptable behavior on Opus 4.7, GPT-5, and one open-weight model (Llama 3.3 70B or Qwen 3 72B).
- No-tools fallback branch verified to render and behave acceptably on at least one model per family.
- Cache prefix integrity: rendered prompt for the same `(available_tools, has_rules, user_rules)` tuple is byte-identical above the `<!-- STATIC PREFIX ENDS HERE -->` marker across calls when only `today` / worktrees vary.

### Open Questions

Resolve before Phase 2:

- Which models does Zed's native agent actually serve through the LLM provider abstraction today, and what is the priority ranking?
- Does Zed have an internal agent eval suite, and which models can it run against?
- What is Zed's current prompt-caching architecture per provider? Single block, or already split static + dynamic?
- For OpenAI: is the harness using the Responses API (recommended for GPT-5) or Chat Completions? OpenAI documents measurable eval gains for Responses.
- For OpenAI: does the harness re-enable Markdown rendering per prompt? GPT-5's API default is plain text.
- For open-weight: which serving stack is most common among Zed users (Ollama, vLLM, llama.cpp, LM Studio)? Affects realistic prefix-caching behavior.
- Are there in-flight changes to `system_prompt.hbs` from other contributors that this spec needs to coordinate with?
- Does the Zed harness inject `<system-reminder>` tags? If yes, what triggers them?
- Should the spec ship a single prompt for all models, or accept a small number of model-family-keyed branches if measurement shows large gaps? Default: single prompt; revisit at end of Phase 3.
- For reasoning models (Opus 4.7 adaptive thinking, o-series, R1, QwQ): is the prompt acceptable when thinking is implicit, or does any current language assume non-thinking behavior?

---

## Phase 2: Plan

Implementation strategy. Each phase is independently shippable; later phases depend on earlier ones only via the static-prefix structure introduced in Phase 2C.

### Phase 2A — Mechanical edits (text-only, lowest risk)

Dependencies: none.

- Audit imperative language; soften where the rule is not high-stakes.
- Convert negative phrasings to positive equivalents where clearer and not less specific.
- Tighten the code-block formatting section to ~25 lines (one good example, one bad example, no meta-commentary).
- Add file-reference convention paragraph using OpenAI's syntax.
- Add anti-overengineering paragraph (replacing or augmenting current `## Communication` end).
- Add `<system-reminder>` semantics line in `## Tool Use` (only if confirmed in Open Questions).
- Add "don't communicate via tool calls" line in `## Communication`.
- Add positive editing-vs-creation rule in `## Searching and Reading`.
- Remove any chain-of-thought scaffolding currently embedded (audit needed).

### Phase 2B — New content (text additions, cross-model parseability)

Dependencies: 2A complete.

- Add 3 worked examples in `<example>` tags:
  - Parallel tool use
  - `update_plan` cadence (inside the `{{#if (contains available_tools 'update_plan')}}` block)
  - Search → read → edit chain
- Refine parallel-tools paragraph with OpenAI's "decide all resources up front, batch everything" framing — but in vendor-neutral language.
- Convert the user-rules and project-rules fences from six-backtick to `<project_rules>` / `<user_rules>` XML tags.

### Phase 2C — Plumbing for cache-safe dynamic content (cross-vendor)

Dependencies: 2A and 2B merged.

- Add `today: String`, `is_git_repo: bool` to template context, populated in `templates.rs`.
- Restructure `system_prompt.hbs` so all dynamic content sits below a single `<!-- STATIC PREFIX ENDS HERE -->` marker.
- Add unit test asserting static-prefix byte-stability across two calls with different `today` values.
- Audit `crates/agent/src/thread.rs` to confirm cache breakpoints (per provider) align with the marker:
  - Anthropic: `cache_control: {type: "ephemeral"}` on the static block
  - OpenAI: leverage automatic prefix caching by sending the static prefix unchanged
  - Open-weight (vLLM / TGI): prefix caching is automatic if the prefix is byte-identical

### Phase 2D — Cross-model smoke-test infrastructure

Dependencies: 2A merged.

- Build `tests/system_prompt_smoke_set.md` with 12 prompts and expected acceptable-behavior criteria.
- Add a script (`script/run-smoke-tests`) that runs the set against a configurable list of models and writes outputs to `tests/system_prompt_smoke_results/<phase>/<model>/`.
- Run baseline against Opus 4.7, GPT-5, Llama 3.3 70B (or Qwen 3 72B); record results.

### Phase 2E — Future work, separate specs

Track these as separate spec documents, do not bundle here:

- Compaction template rewrite (`summarize_thread_detailed_prompt.txt`).
- Mode-transition reminder injection (ask ↔ write).
- Reflexion-style failure-reflection paragraph (only after 2A+2B+2C+2D have a measured baseline).
- Model-family-aware conditional branches (only if measurement justifies).
- Smaller open-weight model variant (a shorter prompt for 7B-13B targets).

---

## Phase 3: Tasks

Each task is a reviewable, independently mergeable PR with stated acceptance criteria.

### Task 1 — Establish multi-model baseline

- Add a unit test that renders `system_prompt.hbs` with the four input matrix combinations and writes the output.
- Record token counts for each combination on three tokenizers (Claude, cl100k_base, Llama).
- Build the smoke-test set (12 prompts).
- Run smoke tests against Opus 4.7, GPT-5, and one open-weight model; commit results.
- **Acceptance:** baseline committed; no behavior change.

### Task 2 — Soften imperatives, positivize phrasings, remove chain-of-thought

- Mechanical pass on `system_prompt.hbs`.
- Reduce imperative count by ≥ 40%; reduce negative phrasings by ≥ 25%; remove any "think step by step" / "reason through X first" language.
- Re-run smoke-test set on three families.
- **Acceptance:** counts hit targets; no smoke-test regression on any family.

### Task 3 — Tighten code-block formatting section

- Reduce to ~25 lines per the boundary rules.
- Re-run smoke-test set with attention to "explore-and-explain" and "small-context-budget" categories.
- **Acceptance:** ≤ 25 lines; smoke-test categories pass on three families.

### Task 4 — Five small content additions

- File-reference convention; anti-overengineering; system-reminder semantics (if applicable); don't-communicate-via-tools; positive editing-vs-creation.
- **Acceptance:** prompt renders; smoke-test passes; total static-body token count increases by ≤ 100 tokens net (additions ≤ deletions from Tasks 2-3).

### Task 5 — Add three worked examples in XML tags

- Inside `<example>` tags in their respective sections.
- **Acceptance:** examples present; smoke-test "multi-file edit" and "parallel reads" categories show no regression on any family.

### Task 6 — Refine parallel-tools paragraph

- Add "decide all resources up front, batch everything" framing in vendor-neutral language.
- **Acceptance:** smoke-test "parallel reads" category shows ≥ 1 additional case where the model parallelizes correctly on at least two of three families.

### Task 7 — Convert rules fences to XML tags

- Replace six-backtick fences with `<project_rules>` / `<user_rules>` tags.
- **Acceptance:** smoke-test "custom-rules respect" category shows no regression; prompt renders correctly when project rules contain Markdown or backticks.

### Task 8 — Plumb `today` and `is_git_repo`

- Add fields to template context, render below the static-prefix marker.
- Unit test for stable static prefix.
- **Acceptance:** test passes; prefix-caching hit rate (if measurable) shows no degradation on any vendor.

### Task 9 — Cache-architecture audit (per vendor)

- Read `crates/agent/src/thread.rs` and confirm cache breakpoints align with the marker for Anthropic, OpenAI, and any open-weight pathway.
- Document findings; file separate issues if refactors are needed.
- **Acceptance:** audit comment merged; issues filed if applicable.

### Task 10 — Final cross-model eval and merge gate

- Run internal eval against at least three families.
- Re-run full smoke-test set.
- **Acceptance:** all success criteria from Phase 1 met; ship.

---

## Phase 4: Implement

Order: Tasks 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10.

Tasks 2–7 are pure prompt edits and can each be a small PR. Task 8 is the only one that touches Rust code beyond test scaffolding. Task 9 is read-only investigation.

After each task, re-run Task 1's render-and-baseline check to confirm the diff matches expectations. After Tasks 2, 5, 7, and 10, re-run the smoke-test set on three model families.

If any task introduces a smoke-test regression on a single family, do not proceed; investigate whether the regression is genuine or model idiosyncratic. If genuine, reduce scope or roll back. If single-model idiosyncratic, document and proceed (the spec accepts variance below 1 percentage point per family).

---

## Living Documentation

This spec is the source of truth for this work. Update it when:

- An assumption is invalidated (revise the assumption, re-evaluate downstream items).
- A boundary is renegotiated (change the boundary, document the rationale).
- A success criterion is met or amended (mark or revise; do not silently delete).
- A new constraint is discovered (add to Boundaries: `Never do`).
- A model family in the matrix is deprecated, added, or significantly upgraded (re-evaluate cross-model implications).

When complete, condense this spec into a final-decisions section at the bottom of the PR description, and link the PR from this document.
