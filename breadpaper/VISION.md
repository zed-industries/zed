# BreadPaper — Product Vision (v0.1)

> **Working tagline:** _Your private second brain, powered by the LLM you already trust._
> _(alternatives to workshop below)_

**Status:** Discussion draft — for the founding design + engineering team.
**Author:** Diego · **Date:** 2026-07-20

**Decisions so far (2026-07-20):**
- **Fork strategy:** _fork required — scope now known._ A custom left-nav pane (Timeline) is a V1 must-have, and Zed's extension API **cannot render any custom panel/dock/UI** (confirmed against the extension API source, docs, and maintainer statements — see §7.1). Panels are core Rust/GPUI. So BreadPaper is a **fork whose custom surface is a small set of new GPUI panels + an invisible-git service**, with the AI rituals riding the *existing* extension + MCP rails (those need no fork). "Prototype first" still holds — but the prototype is a minimal fork, not an extension.
- **v1 audience:** _technical-first_ — ship rough and powerful for engineers who already live in editors; onboarding polish comes later. (§10 Q4)
- **Repo layout:** _single repo — the fork is the product._ Development happens directly on the Zed fork (`github.com/DiegoTavares/bpaper`, cloned to `~/dev/bpaper`), **not** a submodule. All non-Zed content (this doc, design docs, Area packages) lives isolated under `/breadpaper/` so the fork's delta against upstream stays legible and is trivially extractable later (`git filter-repo`). `upstream` remote → `zed-industries/zed` for ongoing rebases. The personal vault (`~/dev/bread-paper`) stays **out** of this repo — private data, separate concern. Named `bpaper`, distinct from the `bread-paper` vault to avoid on-disk/name collision.

---

## 1. The one-sentence pitch

BreadPaper is a desktop app — a private fork of the [Zed](https://zed.dev) editor — that turns a folder of plain Markdown files into a **guided, LLM-augmented second brain**. It ships with pre-built "Areas" (finance, weekly reviews, journaling, team notes) that each come with their own files, layout, and AI rituals, so a person gets the power of a hand-tuned Obsidian-plus-Claude-Code setup **without having to build it themselves.**

## 2. Why this, why now

Today the author runs a system that most people would love to have but almost nobody can assemble:

- A plain-text vault (Obsidian's format) edited in a fast, real code editor (Zed).
- A set of **LLM rituals** — "Friday finance," "week review," "daily closure," "journaling topic" — that read live data (Monarch, GitHub, GitLab), synthesize it, and **append** their findings to human-written notes.
- **Living source-of-truth files** (e.g. `finance_plan_2026.md`) the AI must read before it advises and must update when reality changes, so guidance never drifts.
- A **static HTML dashboard** the LLM feeds with structured data, which then computes its own warnings (time-sinks, lingering projects, carried-over goals).

The problem: this stack is held together by conventions in a `CLAUDE.md`, four slash-command files, two MCP servers, and a folder-naming discipline that only their author fully understands. It is powerful and completely non-transferable. Obsidian is open but generic; Zed is fast but is a code editor with no notion of "your life"; Claude Code is capable but unopinionated. **Nobody ships the opinion.**

BreadPaper is that opinion, productized: the folder structure, the rituals, the layouts, and the AI guardrails become **first-class, visible, editable features** instead of tribal knowledge.

## 3. Who it's for

- **Primary (v1):** technically comfortable people who already keep notes in plain text and want AI woven into their life-admin — but don't want to hand-build the plumbing. "Power journalers," indie hackers, engineers, PKM enthusiasts.
- **Aspirational (later):** anyone who wants a private, local-first "life OS" and is willing to bring their own LLM key. Non-technical users reached through onboarding that hides the machinery.

**Non-goal:** competing with Obsidian as a general note-taking platform, or with Notion as a team wiki. BreadPaper is opinionated and personal by design.

## 4. Principles (the soul of the product)

These are the invariants. Design and engineering decisions should be checkable against them.

1. **Your files, forever, in the open.** Everything is plain Markdown (or whatever format the user prefers) in a normal folder on disk. No proprietary database, no lock-in. If BreadPaper vanished tomorrow, the vault still opens in any editor.
2. **Augmentation, not replacement.** The AI *appends* its synthesis alongside your raw words — it never silently rewrites what you wrote. Your capture and the machine's reflection coexist in the same file (`# LLM Review`, `# AI Week Review`, `# Friday Finance`).
3. **Bring your own brain.** The user chooses and pays for their own LLM (Claude, local model, etc.) via their own key or a console integration. BreadPaper is not a subscription reseller of intelligence.
4. **Human-in-the-loop for anything that matters.** The AI computes and recommends; the human acts. It will tell you exactly how much to pay down your line of credit — it will not (and cannot) move the money.
5. **Living plans over frozen advice.** Canonical files are the source of truth. The AI reads them before advising and edits them when reality shifts, so the plan never drifts from the person.
6. **Modular life.** Nobody wants every module. Areas are opt-in. A user can run only daily notes, or add finance, journaling, team notes — each independently.
7. **Invisible versioning.** Git runs underneath for full history and safety, but the user never types a git command or sees a git pane. Time-travel, not source control.
8. **Everything is editable.** Skills, layouts, prompts, and templates are files the user (and their LLM) can open and change. Power users can rewrite the rituals; the app just ships great defaults.

## 5. The product experience

BreadPaper looks like a focused, three-pane writing environment. Zed's speed and editing quality are the foundation; the chrome around it is re-conceived for life-management rather than code.

### 5.1 Left rail A — **Timeline** (the "now" navigator)
A small, always-present list of the files you almost always want: **Today**, **Yesterday**, **This Week**, **Last Week**. One click (or keystroke) opens the right note. It resolves the app's naming conventions for the user (daily = `YYYYMMDD_MMM-DDD.md`, weekly = `YYYY_WW_MMM.md`) so they never think about filenames. Creating today's note if it doesn't exist yet is a single action — replacing the current "open Obsidian just to trigger a plugin" workaround.

### 5.2 Left rail B — **Areas** (the modular navigator)
A switchable list of the life-domains the user has enabled: _Daily & Weekly_, _Finance_, _Journaling_, _Team_, etc. Each Area is a bundle of folders, templates, a right-pane context view, and skills. Users add or remove Areas from a gallery. Beneath the Areas view, the full file tree remains available for people who want to roam freely.

### 5.3 Right rail — **Context** (page-aware companion)
The right pane changes with the open document:

- On a **daily note** → a time-block view of the day (a day-planner rail).
- On a **weekly note** → the week's calendar with meetings and important markers.
- On a **finance** file → the current dashboard: accounts, budgets, the computed sweep and LoC residual.
- On any file → the relevant **skills** for that context, one click away.

This is where BreadPaper stops feeling like a text editor and starts feeling like an instrument tuned to the thing you're doing.

### 5.4 **Skills view** — the rituals, made visible and editable
Every Area exposes its skills as first-class, inspectable objects, not hidden slash-commands. Example skills, drawn directly from the author's working setup:

| Skill | What it does |
|---|---|
| **Friday Finance** | Pulls live Monarch data, computes the credit-card sweep and line-of-credit residual, presents an ordered action list, waits for the user to actually move the money, then logs what happened into the day's note. |
| **Week Review** | Reads the week's daily notes, aggregates GitHub PRs + GitLab MRs, groups work by project, picks highlights, appends an AI review to the weekly file **and** feeds the dashboard. |
| **Daily Closure** | Reads checked/unchecked tasks, pulls the day's commits, scans recent days for multi-day context, and appends a review with suggestions. |
| **Journaling Topic** | Analyzes weeks of notes to detect avoidance/momentum and surfaces a neglected topic to write about. Read-only — the human owns the reflection. |

Each skill is openable, has a plain-language description, a prompt/logic body the user or their LLM can edit, and clear declarations of **what it reads** (data sources) and **what it writes** (which files, append vs. edit). Trust comes from that transparency.

### 5.5 **Onboarding** — teaching what's possible
A first-run flow that (a) points BreadPaper at a new or existing folder, (b) connects an LLM, (c) lets the user pick their starting Areas from a gallery, and (d) walks them through their first ritual (e.g. create today's note, run a daily closure). The goal is that within ten minutes a new user has done one real, valuable thing — not stared at a blank editor.

## 6. Relationship to Zed — kept / removed / added

**Kept**
- The editor core: speed, multi-format editing, full file-tree access, Markdown as the default.
- Zed's AI integration path that talks to external models via console/agent, so users bring their own LLM.

**Removed / disabled (initially)**
- The subscription-gated AI/billing model — BreadPaper users bring their own key; no reselling of intelligence.
- The Git pane and manual git surface — versioning becomes invisible (see §7).
- Editor chrome and affordances that assume "you are writing software," where they conflict with the life-OS framing.

**Added**
- The **Timeline** left rail (Today / Yesterday / This Week / Last Week).
- The **Areas** left rail + Area gallery / enable-disable.
- The page-aware **Context** right rail (time blocks, week calendar, finance dashboard).
- The **Skills view** (inspect + edit rituals; declared read/write scopes).
- **Onboarding** flow.
- **Invisible git** automation.
- An **Area package format** — the bundle (folders + templates + right-pane view + skills + docs) that makes a domain installable.

## 7. Technical shape (for the engineers)

_High-level and provisional — meant to frame feasibility, not prescribe implementation._

### 7.1 Settled constraint: the panes require a fork (Zed extensions can't render UI)

Confirmed 2026-07-20 against primary sources (Zed's `crates/extension_api` trait, `docs/src/extensions`, and maintainer statements): **the Zed extension API is entirely non-visual.** Extensions can contribute language servers, themes, slash commands, and MCP/context servers — but there is **no** method to render a custom panel, dock, view, or webview, in stable or nightly. All panels (project tree, outline, terminal, agent, git) are compiled Rust/GPUI inside the core, registered via a `Panel` trait not exposed to extensions; WASM extensions are sandboxed with no handle to the window.

Consequence for BreadPaper — a clean split:

- **Requires touching core (fork):** the Timeline pane, the Areas pane, the page-aware Context pane — each is a new GPUI `Panel` registered in the workspace dock. Plus the invisible-git background service.
- **Does _not_ require a fork:** the AI rituals. Daily Closure, Week Review, Friday Finance, and the Monarch/GitHub/GitLab connectors fit the existing **extension + MCP** model and can load into our fork as ordinary Zed extensions.

Design implication: keep the fork's custom surface **small and panel-shaped**, and push as much logic as possible into extensions/MCP so we stay mergeable with upstream. The relevant upstream hope — RFC #53403 "Visual Extension API" (Apr 2026) — is maintainer-gated and explicitly deprioritized, so it must not be counted on.

_Source pointers:_ `zed-industries/zed` `crates/extension_api/src/extension_api.rs`; `docs/src/extensions/developing-extensions.md`; Discussion #53403; Issues #17325, #18877, #21208.

### 7.2 Building blocks

- **Base:** private fork of Zed (Rust + GPUI). We inherit a fast, native, cross-platform editor. Risk: staying mergeable with upstream vs. diverging — mitigated by §7.1's small-fork/large-extension split.
- **Vault = folder on disk.** No new storage engine. Conventions (naming, PARA-style folders) are encoded in the app so the user doesn't maintain them by hand.
- **Areas as packages.** An Area is a declarative bundle: folder scaffolding + templates + a right-pane view spec + a set of skills + a `README`. Installing an Area writes its scaffolding into the vault and registers its views/skills. This is the key extensibility primitive and deserves early design attention.
- **Skills = portable, declarable rituals.** Today they're Claude Code slash-commands with implicit behavior. In BreadPaper a skill declares its **inputs** (files, MCP data sources), its **actions**, and its **outputs** (which files, append vs. edit) so the UI can show scope and the app can sandbox writes. The runtime executes them through the user's chosen LLM.
- **Data connectors via MCP.** Monarch, GitHub/GitLab, calendar, etc. arrive as MCP servers (the author already runs Obsidian + Monarch MCP). BreadPaper should make connecting an MCP source a first-class, guided step rather than hand-edited JSON.
- **Invisible versioning.** A background service commits meaningful checkpoints (autosave/idle/pre-AI-write) to a hidden git repo, exposes a human "history / restore this version" UI, and surfaces conflict recovery — all without the word "git" ever appearing.
- **Dashboards as an output type.** The `structured data (data.js) → static HTML that computes its own analytics` pattern is a repeatable Area capability: skills emit machine-readable feeds; a bundled viewer derives insight. Worth generalizing into the Area format.

## 8. Why it's valuable

- **It sells an opinion, not a blank canvas.** The hard part of PKM isn't the tool — it's designing the system. BreadPaper ships proven systems. That's the differentiated value Obsidian/Notion/Zed structurally can't offer.
- **Local-first + BYO-LLM is a real position.** Privacy-conscious, lock-in-averse users are underserved by cloud note apps. "Your files, your model, your machine" is a clear, honest promise.
- **The rituals compound.** Value grows the longer you use it — weeks of notes make the week-review and journaling skills smarter. That's retention that doesn't depend on a walled garden.
- **A genuine wedge exists:** people already cobbling Obsidian + Claude Code together (a visibly growing crowd) are proof the demand is real and currently unmet by a polished product.

## 9. Feasibility — the honest read

**Encouraging**
- The concept is already **de-risked by a working prototype**: the author's own vault _is_ BreadPaper minus the packaged UX. We're productizing a proven workflow, not inventing an unproven one.
- Zed gives us a world-class editor for free. The genuinely new surface area is chrome + orchestration, not a text engine.
- Markdown-on-disk means low storage/architecture risk and instant interop.

**Hard parts to respect**
- **Forking Zed is a serious commitment.** Rust + GPUI is a real codebase; keeping a private fork current with upstream is ongoing tax. We should decide early: deep fork vs. thin layer (extension/overlay) vs. building panes as Zed extensions where possible. This is the single biggest architectural fork-in-the-road.
- **Invisible git is deceptively subtle.** Autosave churn, merge conflicts, large binaries (the vault already holds multi-MB images), and "restore" UX are all edge-case minefields. Getting "never lose data, never show git" right is a project of its own.
- **Skills need a real trust + safety model.** The moment an AI can write to a user's files and read financial data, scope declarations, dry-runs, previews, and confirmation gates stop being nice-to-haves.
- **BYO-LLM UX is fiddly.** Keys, model choice, local vs. cloud, cost visibility, and graceful failure need thought so non-experts aren't stranded.
- **Onboarding a non-technical user into a fork of a code editor** is a real design challenge — the gap between "engineer's dream" and "my mom could use it" is wide, and v1 should pick a lane honestly.

**Provisional recommendation:** Build the **thinnest thing that proves the core loop** first — Timeline rail + one Area (Daily/Weekly) + one working skill (Daily Closure) + invisible git — on top of Zed, before committing to the full Areas/Skills package framework. Treat it as a personal tool that earns its way to being a product.

## 10. Open questions for the team

1. **Fork depth:** deep Zed fork, thin overlay, or extension-based? What keeps us mergeable with upstream long enough to matter?
2. **Area package format:** what's the minimum declarative spec for a bundle (folders + views + skills + connectors)?
3. **Skill contract:** how do we declare/enforce a skill's read/write scope so users can trust it and the app can sandbox it?
4. **Audience for v1:** technical-first (ship rough, powerful) or approachable-first (invest in onboarding early)? These pull the design in different directions.
5. **Invisible git:** what exactly triggers a checkpoint, and what does "restore" look like to someone who's never heard of a commit?
6. **Distribution & model:** open-source core? paid Areas? one-time vs. subscription (for the app, never the intelligence)?
7. **The name & tagline:** does "BreadPaper" land, and how do we say the value in one line? (see below)

## 11. Tagline candidates (to workshop)

- _Your private second brain, powered by the LLM you already trust._
- _The opinionated second brain. Your files, your model, your machine._
- _Plain text in. Clarity out. Your life, with an AI that actually knows it._
- _A second brain that ships with a system — not a blank page._
- _Local-first life OS. Bring your own brain._

---

_This is a starting point, not a spec. It exists to give designers and engineers a shared picture of the destination so we can argue productively about the route._
