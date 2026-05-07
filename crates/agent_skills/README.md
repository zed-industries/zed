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

## Frontmatter parsing

### Strict validation, with `bail!` on cosmetic issues

`name` must match `[a-z0-9-]{1,64}` and `description` must be 1–1024 characters and non-empty. If either fails, we reject the skill outright with a load error that surfaces in the UI.

Some implementations are more lenient — they warn but load anyway, on the theory that interop is more important than rule enforcement. The spec arguably encourages this approach for cross-tool compatibility.

We deliberately go strict because:

1. The validation rules in the spec are short and easy to follow. A skill that fails them is almost certainly authored incorrectly, not legitimately diverging.
2. Surfacing the error loud-and-early makes skill authoring better. The user fixes the typo and moves on, instead of silently getting an entry in the catalog that doesn't match what they wrote.
3. Lenient parsing is additive. If we later get reports of legitimate skills failing to load, we can loosen specific checks without breaking anything that currently works.

The only field beyond the spec that we honor is `disable-model-invocation` (see below). Unknown fields are silently ignored, which is the standard YAML behavior.

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

### `user-invocable: false` (we don't support, yet)

The inverse — a skill that only the model can invoke, hidden from the slash command list — exists in some other tools, with the use case being "background reference" skills that aren't meaningful as user-typed commands.

We didn't implement it because the use case is rare in practice and the implementation cost is real (filtering the slash command list, plus a second frontmatter field that interacts with the first). If a real user need shows up, this is a small additive change.

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

- **Trust check before loading project-local skills**: a freshly cloned untrusted repo's `.agents/skills/` is currently loaded into the catalog. Edit-gating limits the damage but doesn't prevent a hostile description in the catalog itself. This is the highest-value item left.
- **Override warnings surfaced in the UI**: currently log-only.
- **Lenient frontmatter parsing**: still strict.
- **Compaction protection**: not applicable yet — the agent doesn't compact conversations. When that lands, skill tool outputs should be exempt.
- **Per-skill picker dialog**: slash autocomplete should already surface skills, but a dedicated picker UI (with descriptions, search) would be a small UX win.
- **`user-invocable: false`**: the model-only counterpart to `disable-model-invocation`. Defer until there's a real use case.
- **Argument substitution in skill bodies**: some tools support `$ARGUMENTS` substitution when invoking via slash command. Useful but additive.
- **Dynamic context injection**: shell commands embedded in SKILL.md that get expanded before the model sees the body. Powerful but requires its own security model.

## Where to start reading

- `skill.rs` — types, frontmatter parsing, discovery, override merge.
- `crates/agent/src/tools/skill_tool.rs` — the `skill` tool, the `<skill_content>` renderer, XML escape helper.
- `crates/agent/src/agent.rs` — slash command registration (`build_available_commands_for_project`), slash command activation (`send_skill_invocation`), live reload (`watch_global_skills_directory` and `maintain_project_context`).
- `crates/prompt_store/src/prompts.rs` — catalog construction (`ProjectContext::new`), where `disable-model-invocation` filtering happens.
- `crates/agent/src/templates/system_prompt.hbs` — catalog rendering in the system prompt.
- `crates/agent/src/tools/tool_permissions.rs` — sensitive-path classification for skill files (`SensitiveSettingsKind::AgentSkills`) and the global-skills fast path used by `read_file` and `list_directory`.
