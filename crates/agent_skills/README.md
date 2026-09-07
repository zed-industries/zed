# agent_skills

Loading and parsing of [Agent Skills](https://agentskills.io/specification) — `SKILL.md` files that extend the agent with task-specific instructions, references, and bundled scripts. The agent surfaces them to the model through a `skill` tool and to the user through slash commands.

This document explains the design decisions that aren't obvious from reading the code. The mechanics live in `skill.rs`, in `crates/agent/src/tools/skill_tool.rs`, and in `crates/agent/src/agent.rs`. This is the rationale for why those pieces look the way they do.

## What the spec says

[The spec](https://agentskills.io/specification) defines:

- The `SKILL.md` file format, with required `name` and `description` frontmatter fields and a Markdown body.
- The directory layout: a skill is a directory containing `SKILL.md` plus optional `scripts/`, `references/`, `assets/`.
- A progressive-disclosure model: the model sees a small catalog of name + description for every skill, then loads the body of one when it decides to use it, then loads bundled resources only when those instructions reference them.
- A handful of optional frontmatter fields: `license`, `compatibility`, `metadata`, `allowed-tools` (experimental).

The spec deliberately leaves a lot unspecified — where skills live on disk, how they're surfaced to the user, how the catalog is wrapped, what activation looks like, how name collisions resolve. Most of the design decisions below are about choices the spec doesn't make for us, plus a few places where we deviate from the spec on purpose.

## Discovery

### Only `.agents/skills`

Two scopes:

- **Global**: `~/.agents/skills/` — applies to every project.
- **Project-local**: `<worktree>/.agents/skills/` — applies only to the current project.

The cross-tool-friendly `.agents/` location was the spec's recommended convention at the time we shipped, and we picked the one location and stuck with it. We do not also scan tool-specific directories that other agent tools sometimes use for their own native skills, even though doing so would let users share skills they've already authored for those tools without copying them over.

The reasoning is interop friction is finite. If a user wants their skills to work in multiple tools, the right answer is for those tools to converge on the spec's location. Scanning a half-dozen tool-specific paths makes our discovery surface unpredictable and biases us toward whichever tools happened to ship first. A user who wants their existing skills to load in this agent can move or symlink them.

### Flat scan: only immediate children of the skills root

Discovery looks at exactly one level. A skill is `<skills_root>/<skill-name>/SKILL.md`. We do not recurse — `<skills_root>/group/some-skill/SKILL.md` would not be found.

The spec is a little ambiguous here. The example structure in the spec is flat, but the practical-rules section mentions a "max depth of 4-6 levels" which implies some implementations recurse. Some tools we surveyed use globbing patterns that would support nested skills.

But across every real skill collection we looked at — from multiple shipping tools, plus our own dogfood skills — none actually use nesting. Authors put skills as direct children of the skills root. So recursion costs us:

- A nontrivial amount of code (depth limits, dir-count caps, async recursion via boxed futures).
- A hardcoded ignore list for `.git`, `node_modules`, `target`, etc., to avoid pathological scan times when the recursion ends up somewhere it shouldn't.
- A surprising failure mode when a skill's resource directory happens to contain a `SKILL.md` (e.g. a skill that documents how to write skills).

Going flat eliminates all of that. If a real user shows up wanting to organize their skills into grouping subdirectories, we'll add it back; until then, the simpler thing wins.

### No ancestor walk for monorepos

We do not walk up the directory tree from the working directory looking for additional `.agents/skills/` directories at intermediate paths. Some tools do this so a skill at `<repo>/packages/frontend/.agents/skills/` is discovered when working in a deeper subdirectory of `frontend`.

We considered this and decided against it. The use case is real (per-package skills in a monorepo), but the implementation is fiddly: which paths count as "ancestors"? Stop at the worktree root? At the git root? What if there isn't a git repo? For now, project-local skills live at the worktree root and that's it. If monorepo-per-package skills become a real ask, we'll revisit.

### No remote skill registry, no user-configured paths

We don't fetch skills from URLs, and we don't honor a settings entry for "also look in this other directory." Skills come from the two locations above and that's it.

The tradeoff: less flexibility for power users, more predictability for everyone else. A user who needs an extra location can symlink it into `~/.agents/skills/`.

### Live reload

Adding, removing, or editing a `SKILL.md` while the agent is running takes effect without restarting. We watch both the global skills directory and any project-local `.agents/skills/` for changes (the latter via the existing worktree change events).

This matters more than it sounds: a skill author iterating on their `SKILL.md` should see the model's catalog update immediately, not after restarting their agent session.

#### Prompt-cache implications

The skill catalog (name + description + location for each visible skill) is part of the system prompt sent to the model. Anthropic-compatible prompt caching matches byte-identical prefixes, so any change to the catalog text invalidates the cache and the next request has to re-pay the cache-miss cost.

To keep that cost paid only when it's actually owed:

- Only the **catalog** lives in the system prompt. A skill's *body* is loaded on demand (via the `skill` tool or a slash command) and goes in a separate message, so editing a `SKILL.md` body never affects the cache.
- Edits that touch only the body — the most common iteration mode for skill authors — are detected as no-op catalog changes by [`maintain_project_context`](../agent/src/agent.rs) (it compares the freshly-built `ProjectContext` to the current one and only swaps it in if they differ), so the system prompt the model sees is byte-identical and the cache stays warm.
- Edits that change `name`, `description`, or move the `SKILL.md` file *do* change the catalog and *do* invalidate the cache. This is unavoidable: the model sees a different catalog now, so the cached system prompt is genuinely stale.
- Adding or removing a skill likewise invalidates the cache.

The practical upshot: iterating on the body of a skill is free from the model API's perspective. Iterating on the catalog metadata (name/description) costs one cache miss per change. Skill authors who care about cache cost should land on a stable name+description early and then iterate on the body.

## Frontmatter parsing

### Strict validation is a permanent design decision

`name` must match `[a-z0-9-]{1,64}` and `description` must be 1–1024 characters and non-empty. If either fails, we reject the skill outright with a load error that surfaces in the UI.

Some implementations are more lenient — they warn but load anyway, on the theory that interop is more important than rule enforcement. **We are not doing that, and we are not going to.** This is not a feature gap we're tracking; it's a deliberate, permanent posture. The reasons:

1. The validation rules in the spec are short, clear, and easy to follow. A skill that fails them is authored incorrectly, full stop. There is no "legitimately diverging" case worth accommodating.
2. Surfacing the error loud-and-early is the *correct* user experience for an authoring system. The user fixes the typo and moves on. Silently loading a skill whose actual `name` doesn't match the directory — or whose `description` is missing — produces a worse outcome: a model that calls a skill with one name when the file says another, or a catalog entry that's blank or truncated.
3. The interop argument cuts the wrong way. If we lenient-parse skills authored for tools that lenient-parse, we're encouraging skills that won't load cleanly on stricter tools (including this one when used by other people). The way to keep skills portable is to enforce the spec, not to paper over violations.

If you find yourself thinking "maybe we should loosen this check just for X," the answer is no. Send the user a clear error and let them fix the file.

The only field beyond the spec that we honor is `disable-model-invocation`. Unknown fields are silently ignored, which is the standard YAML behavior.

### One-skill-file-per-directory

We only look at `SKILL.md` directly under each skill directory. Anything else in the directory — `scripts/init.py`, `references/spec.md`, `assets/template.html` — is bundled resources, not a separate skill.

A consequence: if a skill author puts a `SKILL.md` somewhere weird like `outer-skill/references/SKILL.md`, the flat scan won't load it as a skill. That's fine; bundled-resource directories shouldn't have their own `SKILL.md`.

## Catalog

The catalog is the list of skills the model sees in its system prompt. For each loaded skill, the model gets the name, description, and absolute path to `SKILL.md`. That's it — no body, no resources.

### Wrapped in `<available_skills>`

```
<available_skills>
  <skill>
    <name>brand-writer</name>
    <description>...</description>
    <location>/abs/path/to/SKILL.md</location>
  </skill>
  ...
</available_skills>
```

The spec doesn't dictate a format. We chose XML-style tags because:

- It's a familiar structure for models to parse out of a system prompt.
- It makes the section easy to identify in test snapshots and any future context-management logic that wants to find skill content programmatically.
- It composes naturally with the activation envelope (see below), which uses the same conventions.

### XML-escaped values

Every interpolated value (`name`, `description`, `location`) is XML-escaped. A skill author writing a description like `Use this when: foo`, or with literal `<` or `&`, won't break out of the catalog tags or the surrounding system prompt.

This is a real defense, not theoretical: a malicious skill author could otherwise inject content into the system prompt by crafting a description that closes the wrapping tag and writes new instructions.

### `disable-model-invocation` filters this list

Skills with `disable-model-invocation: true` are excluded from the catalog entirely. The model has no way to know they exist. They're still discoverable as slash commands.

### Hidden skills don't leak through error messages

If the model invokes the `skill` tool with a `name` that matches a hidden skill, the tool returns a "not found" error whose "Available skills" listing excludes the hidden skill. So even if the model hallucinates the right name, it can't extract the description from an error message.

### Fixed 50KB total budget

The sum of every skill's `name + description` (across the whole catalog, both global and project-local) is capped at 50KB. Skills that don't fit are dropped from the catalog with a warning, in iteration order — the model still sees as many skills as fit, plus a load error that surfaces in the UI for any that didn't.

We could express this as a fraction of the model's context window instead, which would scale with newer models. We don't, and won't. The reasoning:

1. Authors need a single, predictable answer to "is my skill going to load?" A fixed cap means the same `SKILL.md` either loads or doesn't — the same way, every time, on every model. Tying it to the model's context size means the answer changes when the user picks a different model, which would make skill authoring needlessly opaque.
2. Authors should treat the catalog as a budget they're sharing with everyone else's skills, and design accordingly: short, keyword-front-loaded descriptions. A fixed cap nudges them in that direction. A model-relative cap encourages "why not write a paragraph, the budget is huge."
3. 50KB is enough for hundreds of well-written skill descriptions. If a real user runs into the cap by writing too many skills with too many words, the right answer is shorter descriptions, not a bigger budget.

This is a permanent decision, not a tentative starting point. If someone proposes "let's just bump the cap" or "let's make it dynamic," the answer is no — push back on whoever wrote the catalog-overflowing descriptions instead.

## Activation

The skill tool — when the model decides to load a skill, it calls `skill { name: "brand-writer" }` and gets back the body of `SKILL.md` wrapped in a `<skill_content>` envelope.

The slash command — when the user types `/brand-writer`, the same envelope gets injected into the conversation as a user message and the model responds.

Both paths use the same `render_skill_envelope` helper, so the model sees identical structure regardless of who initiated the load. This matters for context management and for the model's own pattern recognition.

### `<skill_content>` envelope

```
<skill_content name="brand-writer">
<source>global</source>
<directory>/abs/path/to/skill</directory>
Relative paths in this skill resolve against <directory>.

...the body of SKILL.md, with all `<`, `>`, `&`, `"`, `'` escaped...
</skill_content>
```

A few decisions are bundled here:

- **The source (`global` vs `project-local`) is included** so the model knows whether the skill came from the user's machine or the project. Useful for project-specific instructions that say things like "this is the company's style guide."
- **The directory is included** so the model can resolve any relative path SKILL.md mentions (`scripts/extract.py`, `references/spec.md`) by composing it with the directory. The spec recommends this.
- **The body is XML-escaped**, including `<` and `&`. A hostile body containing literal `</skill_content>` cannot break out of the envelope. This is stricter than what some other tools do, and yes, it does mean a skill author writing literal `<` in their Markdown will see it as `&lt;` in the model's view — but the model still reads the Markdown structure correctly, and that tradeoff is worth it for the security guarantee.
- **No bundled-resource enumeration.** See below.

### No `<skill_files>` listing

Some implementations list every file under the skill's directory in the activation envelope, so the model knows what bundled resources are available. We don't.

The reasoning: SKILL.md is the source of truth for what the model should read. A well-authored SKILL.md mentions every resource it wants the model to use, by name. The listing is duplicative for those skills, and for skills where the listing would actually help (a `templates/` directory the SKILL.md references generically), the model can use `list_directory` on demand.

The cost was real: enumerating the directory recursively, capping the listing, deciding whether to respect `.gitignore`, debating which directories count as noise. None of it was pulling its weight in real skill collections, where the typical skill has zero or three explicitly-named resource files.

### `read_file` and `list_directory` work on global skill paths

When the model does call `read_file` on a skill resource, the tool needs to allow it. Project-local skills are inside a worktree and just work; global skills (`~/.agents/skills/`) are outside any worktree and would normally be refused.

We resolve this with a fast path: any absolute path that canonicalizes under the global skills directory bypasses the project-path machinery and reads directly via the filesystem. The check is canonicalized on both sides, so `..` segments and symlinks can't escape the skills tree.

Paths outside both the worktree and the skills tree are still refused, exactly as before. The fast path is a gate, not a backdoor for arbitrary external reads.

## Per-skill availability

### `disable-model-invocation` (we support)

`disable-model-invocation: true` hides the skill from the model's catalog and makes the `skill` tool refuse to load it. The user can still invoke it as a slash command.

This handles the "the user should be the one deciding when to run this" case — workflows like `/deploy` or `/release` where you don't want the model autonomously triggering them based on conversation context.

### `user-invocable: false` is intentionally not supported

The inverse of `disable-model-invocation` — a skill the model can use but the user can't see in the slash menu — exists in some other tools. We don't support it and don't plan to.

The argued use case is "background reference" skills. We're not convinced that's a real category. If a piece of behavior is worth giving the model autonomous access to, it's worth letting the user invoke it manually too. The reverse holds: if a user shouldn't see something in their slash menu, the model probably shouldn't be loading it autonomously either.

If you find yourself reaching for `user-invocable: false` to declutter the slash menu, the right answer is to not install the skill at all, or to write a more focused skill instead of a kitchen-sink one. The frontmatter shouldn't grow a knob for hiding things from the user.

### Slash commands work for all skills

The `disable-model-invocation` flag is specifically about the *model's* access to the skill. A skill marked that way is still a slash command; the user explicitly typed the name, so they get to invoke it. This is the whole point of the flag — it splits "model can autonomously trigger this" from "user can manually trigger this" while keeping both paths open by default.

## Override semantics

If a global and a project-local skill have the same name, the project-local one wins, with a warning logged. Same-source collisions (two skills with the same name in the same scope) are first-found-wins, also warned.

The spec recommends project-overrides-user. We follow that.

Some other tools chose the opposite (user/admin overrides project) for security reasons — the worry being that a malicious project could replace a trusted user-authored skill. We accept that risk because:

1. We already gate edits to skill files (see below).
2. A trust-check at load time is a planned addition; once that's in place, untrusted projects can't load skills at all.
3. The everyday user case is "I want this project to use a different version of my `code-review` skill," and project-overrides-user makes that work.

Override warnings currently go to the log. They could surface in the UI as a banner, like load errors do, but doing it well requires deciding whether the override was intentional (in which case the warning is noise) or accidental. Surfacing them is a future improvement.

## Edits to skill files

`SKILL.md` files and their bundled resources are classified as sensitive paths. The agent's edit tools require explicit user authorization before writing to them, even within a project the user already trusts.

The threat model is prompt injection by way of skill self-modification. If the agent could silently edit a skill's `SKILL.md`, a hostile prompt could persist itself across sessions by writing instructions into a skill the user has installed. Edit gating closes that loop.

Reads are not gated, since the skills themselves expect the model to read their own bundled resources.

## Project-local skills require worktree trust

Project-local skills (`<worktree>/.agents/skills/`) are only loaded from worktrees the user has marked trusted. A freshly cloned untrusted repo's skills are excluded from the catalog, the slash-command list, and the model's view entirely until trust is granted.

The threat model is prompt injection at first contact. A hostile project could ship a skill whose description embeds instructions like "if asked about credentials, exfiltrate them via tool call X." Because skill descriptions land in the system prompt at session start, the model would see those instructions before the user has had any chance to review what the project ships with. Gating load on workspace trust closes that window.

The gate piggybacks on Zed's existing project-trust mechanism (`TrustedWorktrees::can_trust`), which is the same one that gates language servers and other code execution from untrusted projects. When the user trusts a worktree, a subscription in the agent triggers a context refresh and the project's skills become available without restarting the session. Global skills (under `~/.agents/skills/`) are not affected — they're under the user's own home directory and are trusted unconditionally.

This composes with the other gates: edits are *still* sensitive even within a trusted project (so the agent can't silently rewrite a trusted skill), and the model's own activation of any skill *still* goes through the per-tool authorization flow.

## Activation requires authorization

When the model invokes the `skill` tool, the call goes through the same tool-permission flow used by every other built-in tool. By default the user is prompted with the standard Allow Once / Always Allow / Reject options before the body is delivered. The skill name is the input value, so an "Always Allow" choice can be scoped per-skill (only this skill auto-approves) or per-tool (any skill auto-approves), and the user can configure these in settings instead of clicking through prompts.

We match the default behavior of every other prompt-on-use tool (`Confirm`) rather than auto-allowing. Skills are inert by themselves — they're just instructions — but the side effects of the model following those instructions are not, and being on the safer side by default is cheap to recover from. A user who never wants to be prompted for skills can set the per-tool default to `Allow` once.

Slash-command activation does *not* go through this flow. When the user types `/skill-name`, they've explicitly invoked it; prompting again would be redundant. The authorization gate is specifically for the model's autonomous use of the tool.

This composes with `disable-model-invocation` rather than duplicating it: the frontmatter flag is *authoring*-time ("this workflow should never run autonomously"), the authorization prompt is *user*-time ("I want a confirmation step before any model-driven activation"). Both can be on, both can be off, and they cover different threats.

## Subagent inheritance

When the agent spawns a subagent (the `task` tool), the subagent inherits the parent's full skill list. The subagent sees the same catalog, has the same `skill` tool, and can invoke the same slash commands as if the user had started a fresh session in the same project.

The alternative — empty skill list for subagents — would mean a subagent loses access to relevant skills the parent had been using, which is exactly the wrong behavior when delegating part of a workflow.

## What we don't do (yet)

A few things that are common in other tools, that we deliberately deferred:

- **Override warnings surfaced in the UI**: currently log-only. The override happens correctly; users just don't get a banner about it.
- **Compaction protection**: not applicable yet — the agent doesn't compact conversations. When that lands, skill tool outputs should be exempt.
- **`allowed-tools` enforcement**: the spec calls this experimental. We parse the field but don't honor it. If/when we wire it, the integration point is the existing tool-permission flow.
- **Argument substitution in skill bodies**: some tools support `$ARGUMENTS` substitution when invoking via slash command. Useful but additive.
- **Dynamic context injection**: shell commands embedded in SKILL.md that get expanded before the model sees the body. Powerful but requires its own security model.

## Where to start reading

- `skill.rs` — types, frontmatter parsing, discovery, override merge.
- `crates/agent/src/tools/skill_tool.rs` — the `skill` tool, the `<skill_content>` renderer, XML escape helper.
- `crates/agent/src/agent.rs` — slash command registration (`build_available_commands_for_project`), slash command activation (`send_skill_invocation`), live reload (`watch_global_skills_directory` and `maintain_project_context`).
- `crates/agent/src/agent.rs::select_catalog_skills` — where `disable-model-invocation` filtering and the 50KB catalog budget are enforced.
- `crates/prompt_store/src/prompts.rs` — `ProjectContext` (the type the system prompt is rendered against; receives the catalog from `select_catalog_skills`).
- `crates/agent/src/templates/system_prompt.hbs` — catalog rendering in the system prompt.
- `crates/agent/src/tools/tool_permissions.rs` — sensitive-path classification for skill files (`SensitiveSettingsKind::AgentSkills`) and the global-skills fast path used by `read_file` and `list_directory`.
