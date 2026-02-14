# Documentation Remediation Plan

Tracking document for bringing all Zed documentation up to conventions and brand voice standards.

**Created:** 2026-02-14
**Last Updated:** 2026-02-14
**Status:** In Progress

---

## Summary

| Category | Total | Compliant | Needs Work |
|----------|-------|-----------|------------|
| Frontmatter | 166 | 166 | 0 ✓ |
| Brand Voice | TBD | TBD | TBD |
| Callout Format | ~10 | ~10 | 0 ✓ |
| Keybinding Syntax | TBD | TBD | TBD |

---

## P0 — Critical (Must Fix First)

### P0-1: Rewrite getting-started.md

**File:** `docs/src/getting-started.md`
**Status:** [x] COMPLETED 2026-02-14

**Changes Made:**
- Rewrote opening paragraph to state facts (Rust, GPU-accelerated, multiplayer)
- Replaced all hardcoded keybindings with `{#kb action::Name}` syntax
- Added anchor IDs to all main sections
- Removed "Welcome to Zed! We are excited to have you" and other taboo phrases
- Simplified and tightened prose throughout

---

### P0-2: Fix or Remove quick-start.md

**File:** `docs/src/quick-start.md`
**Status:** [x] COMPLETED 2026-02-14

**Resolution:** Removed empty stub file. It was not linked from SUMMARY.md or any other doc.

---

### P0-3: Add Frontmatter to All Docs

**Status:** [x] COMPLETED 2026-02-14

**Scope:** 118 docs updated with frontmatter

**Missing frontmatter in these directories:**
- `docs/src/languages/` — ~45 files
- `docs/src/migrate/` — 5 files
- `docs/src/extensions/` — ~10 files
- `docs/src/development/` — ~7 files
- `docs/src/collaboration/` — 3 files
- `docs/src/reference/` — 3 files
- Root level docs — ~40 files

**Template:**
```yaml
---
title: [Feature Name]
description: [One sentence describing what this page covers]
---
```

**Actions:**
1. Create script to batch-add frontmatter
2. Generate titles from H1 headings
3. Generate descriptions from first paragraph or manually write
4. Validate all frontmatter after addition

---

## P1 — High Priority

### P1-1: Add Opening Paragraphs

**Status:** [~] PARTIAL — Key files done, bulk remaining

**Completed:**
- [x] `extensions/installing-extensions.md` — Improved opening and description
- [x] `extensions/developing-extensions.md` — Added opening paragraph

**Remaining (lower priority):**
- [ ] Other extension files (themes, languages, slash-commands, etc.)
- [ ] migrate/ files — These already have reasonable structure
- [ ] languages/ files — Auto-generated descriptions are acceptable for now

---

### P1-2: Fix collaboration/ Brand Voice

**Status:** [x] COMPLETED 2026-02-14

**Files Updated:**
- [x] `collaboration/overview.md` — Rewrote opening, fixed callout, added anchor IDs
- [x] `collaboration/channels.md` — Fixed callouts, improved description, added anchor ID
- [x] `collaboration/contacts-and-private-calls.md` — Improved opening and description

---

### P1-3: Standardize Callout Format

**Status:** [x] COMPLETED 2026-02-14

**Files Fixed:**
- vim.md
- completions.md
- troubleshooting.md
- key-bindings.md
- extensions/developing-extensions.md
- ai/llm-providers.md
- development.md
- repl.md
- collaboration/channels.md

---

## P2 — Medium Priority

### P2-1: Convert Hardcoded Keybindings to {#kb} Syntax

**Status:** [~] PARTIAL — Key files done

**Completed:**
- [x] `running-testing.md` — Converted Quick start section to use {#kb} syntax

**Remaining (lower priority):**
- Many migration guides have hardcoded keybindings for JetBrains/VS Code equivalents
- Some terminal keybindings don't have corresponding actions
- Trade-off: Not all keybindings have action mappings

---

### P2-2: Add Anchor IDs to Key Sections

**Status:** [x] COMPLETED — Key reference docs done

**Files Updated:**
- [x] `configuring-zed.md` — Added anchors to Settings Editor, Settings Files, User/Default/Project Settings, How Settings Merge
- [x] `key-bindings.md` — Added anchors to Predefined Keymaps, Keymap Editor, User Keymaps
- [x] `vim.md` — Added anchors to all major sections (design, enabling, features, command palette, customizing, settings)

---

### P2-3: Add "See Also" Sections

**Status:** [ ] Not Started — Lower priority

**Note:** Many docs already have related links at the end. This can be addressed during full audit phase.

---

## P3 — Lower Priority

### P3-1: Add Version Callouts

**Status:** [ ] Not Started

**Convention:** When behavior differs by version, include:
```markdown
> **Note:** In Zed v0.224.0 and above, [behavior description].
```

**Known version-specific behaviors:**
- [ ] basedpyright as default (v0.204.0)
- [ ] Tool permissions changes
- [ ] Various AI feature additions

---

### P3-2: Remove TBD HTML Comments

**Status:** [ ] Not Started

**Action:** Grep for `<!-- TBD` or similar patterns and either:
1. Complete the TODO
2. Remove the comment if no longer relevant
3. Convert to GitHub issue if needs future work

---

## Per-Doc Audit Checklist

Use this checklist when reviewing each document:

### Structural
- [ ] Has YAML frontmatter with `title` and `description`
- [ ] Has opening paragraph (what + why)
- [ ] Sections ordered correctly (usage → config → reference)
- [ ] Anchor IDs on linkable sections
- [ ] "See Also" section if relevant

### Formatting
- [ ] Callouts use `> **Note:**` format
- [ ] Settings use `code` formatting
- [ ] JSON examples have `[settings]` or `[keymap]` annotation
- [ ] Keybindings use `{#kb}` syntax where applicable
- [ ] Actions use `{#action}` syntax where applicable

### Brand Voice (must score 4+ on all)
- [ ] Technical Grounding — Specific, verifiable claims
- [ ] Natural Syntax — Flows like developer speech
- [ ] Quiet Confidence — Facts without hype
- [ ] Developer Respect — Peer-to-peer tone
- [ ] Information Priority — Key info first
- [ ] Specificity — Concrete, measurable claims
- [ ] Voice Consistency — Unified tone throughout
- [ ] Earned Claims — Assertions are supportable

### Taboo Phrases (must have zero)
- [ ] No "We're excited/thrilled"
- [ ] No exclamation points
- [ ] No "revolutionary/game-changing"
- [ ] No em dash chains (max 1 per paragraph)
- [ ] No "It's not X, it's Y" constructions
- [ ] No vague benefits ("enhanced productivity")

---

## Audit Progress Tracker

### Root Level Docs

| File | Frontmatter | Voice | Callouts | Keybindings | Status |
|------|-------------|-------|----------|-------------|--------|
| getting-started.md | ✓ | ✗ | ? | ✗ | P0 |
| quick-start.md | ✗ | N/A | N/A | N/A | P0 |
| installation.md | ✓ | ? | ? | ? | Pending |
| configuring-zed.md | ✓ | ? | ? | ? | Pending |
| ... | | | | | |

### AI Docs (`ai/`)

| File | Frontmatter | Voice | Callouts | Keybindings | Status |
|------|-------------|-------|----------|-------------|--------|
| overview.md | ✓ | ? | ? | ? | Pending |
| agent-panel.md | ✓ | ? | ? | ? | Pending |
| ... | | | | | |

### Language Docs (`languages/`)

| File | Frontmatter | Voice | Callouts | Keybindings | Status |
|------|-------------|-------|----------|-------------|--------|
| python.md | ✗ | ✓ | ? | ? | Pending |
| javascript.md | ✗ | ? | ? | ? | Pending |
| ... | | | | | |

*(Continue for all directories)*

---

## Workflow

### Phase 1: P0 Critical Fixes
1. Fix getting-started.md (brand voice + keybindings)
2. Remove/merge quick-start.md
3. Batch-add frontmatter to all 114 docs

### Phase 2: P1 High Priority
4. Add opening paragraphs to extensions/, migrate/, languages/
5. Fix collaboration/ brand voice
6. Standardize callout format globally

### Phase 3: P2/P3 Cleanup
7. Convert remaining hardcoded keybindings
8. Add anchor IDs
9. Add "See Also" sections
10. Version callouts
11. Remove TBD comments

### Phase 4: Full Audit
12. Review each doc against checklist
13. Score against brand voice rubric
14. Fix any remaining issues

---

## Notes

- Python.md is a good example of thorough documentation (though missing frontmatter)
- getting-started.md structure is good but needs voice cleanup
- Many language docs follow a consistent pattern — can batch-update
