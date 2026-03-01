# Feature Research

**Domain:** Persistent undo/redo for a code editor (Zed)
**Researched:** 2026-03-01
**Confidence:** HIGH (core behaviors verified against VS Code docs, Vim docs, Emacs package source, and Zed GitHub issues)

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete or inferior to Neovim/VS Code.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Survive tab close and reopen | Core use case — VS Code does this, Neovim does this, users expect parity. Zed GitHub issues #4942, #23164, and discussion #16485 all confirm this is the primary user pain point | MEDIUM | VS Code restores on reopen if file unchanged since close. Vim writes on every file save. Both approaches work. |
| Survive full editor restart | Neovim users specifically cite this as why they enable `undofile`. Zed issue #4942 was closed in favor of #15097 ("Serialize undo history") confirming this scope | MEDIUM | Requires writing history to disk at close/save, not just in-memory carryover between tab switches |
| Invalidate history when file modified externally | Vim implements this via file content hash (documented in vimhelp.org). Emacs undo-fu-session uses length + checksum. Users expect that undo won't corrupt a file that changed outside the editor | LOW | Zed already has file watching. The behavior is: detect external change, discard stale history. Zed PROJECT.md already identifies this as a validated requirement |
| Per-user opt-in/opt-out toggle | VS Code had to add `files.restoreUndoStack: false` in v1.45 immediately after shipping v1.44 persistent undo, because some users hated it. This pattern is well-established: ship enabled-by-default (or disabled-by-default) with a toggle | LOW | PROJECT.md already decided: disabled by default, opt-in via `persistent_undo.enabled` |
| Configurable entry limit | All editors expose this: Vim's `undolevels`, VS Code's local history max entries, undo-fu-session's `undo-fu-session-file-limit`. Users want a knob to bound storage | LOW | PROJECT.md has `persistent_undo.max_entries`, default 10,000. This is the right approach. |
| File pattern exclusions | Vim supports per-file exclusions via autocommands. Emacs undo-fu-session has `undo-fu-session-incompatible-files` (regex list) and also excludes temp dirs and encrypted files automatically. Users need to exclude generated files like `*.lock`, large binary-adjacent files like `*.csv` | LOW | PROJECT.md already has `persistent_undo.exclude` (glob patterns). Consistent with how Zed's other settings work. |
| Auto-prune stale entries | Undo files accumulate on disk. Editors that don't prune cause eventual disk complaints. Vim never auto-deletes undo files (documented caveat). Emacs undo-fu-session's `undo-fu-session-file-limit` removes oldest files when limit is hit. This is expected housekeeping | MEDIUM | Prune when: file no longer exists, file has been moved/renamed, history is older than N days. PROJECT.md calls this out. |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valued.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Undo history survives remote reconnection | Zed issue #31861 ("Serialize undo history so it can survive remote reconnects") identifies this as a concrete pain point unique to Zed's remote editing model. No other editor faces this because they don't have built-in remote editing | HIGH | Out of scope for v1 per PROJECT.md collab note, but the serialization foundation built for persistent undo directly enables this later |
| Graceful degradation on history mismatch | Rather than silently dropping history or crashing, surface a clear message: "Undo history unavailable — file was modified externally." Vim is silent by default (requires `set verbose`). Emacs undo-fu-session logs a warning but users rarely see it. An editor that communicates clearly wins developer trust | LOW | Confidence: MEDIUM. This is inferred from documented Vim/Emacs gaps, not a verified user request. |
| Storage directory transparency | Let users see where history files live and how large they are. No editor currently surfaces this in their UI. Could be as simple as a status message or a line in settings. | LOW | Out of scope for v1 but worth noting as differentiation opportunity |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Undo tree visualization (branching UI) | Vim's undotree plugin is wildly popular. Emacs undo-tree has a built-in visualizer. Users who know these tools ask for them | Non-linear undo visualization requires significant UI surface area (a panel, keyboard navigation, diff preview). It's complex to implement well, and Zed's linear Cmd-Z / Cmd-Shift-Z model doesn't expose branches at all today. Building a viz before the underlying persistence is solid is building on sand | Build the linear persistent undo first. Undo tree visualization is explicitly Out of Scope in PROJECT.md. Do not add it. |
| Cross-device sync | Users who want to resume work from a different machine might ask for this | Requires cloud storage, authentication, conflict resolution between different machines' undo histories, and encryption at rest. This is a product unto itself. | Git covers the important cross-device continuity story. Persistent undo is a local safety net, not a sync mechanism. PROJECT.md marks this Out of Scope. |
| Collaborative undo (multi-user) | In Zed's collab sessions, knowing what a collaborator undid is occasionally requested | Collab undo is a fundamentally different problem — operations from multiple authors, CRDT semantics, who can undo whose changes. The history format for single-user local undo doesn't extend to this without a rethink | Handled by Zed's collab layer separately. Out of Scope in PROJECT.md. |
| UI indicator showing history availability | Feature requests sometimes include "show a badge when persistent history is loaded" | Adds UI complexity and visual noise. The user behavior is simple: open file, hit Cmd-Z, it works. An indicator is only useful when it's missing, not when it's present — and when it's missing (external edit invalidated history), a brief notification or silent graceful degradation is sufficient | On failed restore (external edit detected), show a non-blocking notification: "Undo history cleared — file was modified externally." No persistent badge needed. PROJECT.md marks a persistent indicator Out of Scope. |
| Size-based limits (MB cap) | Seems intuitive — "don't use more than 50MB" | Entry count is more predictable from the user's mental model ("10,000 undos") and simpler to implement than tracking accumulated byte size of binary undo blobs. Notepad++ attempted complex undo-beyond-save and removed it in v7.7 due to memory overflow bugs | Entry count limit (PROJECT.md's `persistent_undo.max_entries`). Simpler, predictable, correct. |

## Feature Dependencies

```
[Write history to disk at close/save]
    └──requires──> [Undo history serialization format]
                       └──requires──> [text::Buffer UndoMap encoding]

[Restore history on file open]
    └──requires──> [Write history to disk at close/save]
    └──requires──> [File content hash/checksum for invalidation check]

[File pattern exclusions]
    └──requires──> [Write history to disk at close/save]
    (evaluated before writing, not after)

[Auto-prune stale entries]
    └──requires──> [Write history to disk at close/save]
    └──enhances──> [File pattern exclusions]

[Invalidate on external change]
    └──requires──> [File content hash stored alongside history]
    └──enhances──> [Restore history on file open]

[Entry limit enforcement]
    └──requires──> [Write history to disk at close/save]
    └──enhances──> [Auto-prune stale entries]
```

### Dependency Notes

- **Restore requires Write:** You cannot restore what has not been written. Write-on-close (or write-on-save) must land before restore-on-open.
- **Hash validation is a prerequisite for restore correctness:** Without it, restore can corrupt the buffer state if the file has changed. This is not optional.
- **Exclusions gate writes, not reads:** Check glob patterns before writing history, not at read time. If the file should be excluded, write nothing — there's nothing to restore.
- **Pruning enhances the entry limit:** When the entry count exceeds `max_entries`, prune oldest entries. When a tracked file no longer exists or has been renamed, prune its record.

## MVP Definition

### Launch With (v1)

Minimum viable product — what's needed to validate the concept and match Neovim/VS Code behavior.

- [ ] Write undo history to disk when a buffer is closed or saved — this is the gate for everything
- [ ] Restore undo history from disk when a file is reopened (only if file content unchanged since close)
- [ ] Hash/checksum file content and store alongside history; skip restore on mismatch
- [ ] Configuration: `persistent_undo.enabled` (default: false) — opt-in
- [ ] Configuration: `persistent_undo.max_entries` (default: 10,000)
- [ ] Configuration: `persistent_undo.exclude` (glob patterns, e.g., `["*.lock", "*.csv"]`)
- [ ] Auto-prune: clear history records for files that no longer exist

### Add After Validation (v1.x)

Features to add once core is working and opt-in rate shows adoption.

- [ ] Invalidate history on detected external file change (file watcher integration) — Zed already has file watching, wiring it to clear the persisted record is a natural follow-on
- [ ] More aggressive pruning: time-based expiry (clear entries older than N days), triggered on editor startup
- [ ] Non-blocking notification when history is cleared due to external modification: "Undo history cleared — this file was modified outside Zed"

### Future Consideration (v2+)

Features to defer until the persistence foundation is stable.

- [ ] Undo history survival across remote session reconnects (Zed-specific; requires collab layer coordination)
- [ ] Undo tree visualization (non-linear branching UI panel) — requires significant UI investment, not justified until linear persistence is proven valuable

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Write history on close/save | HIGH | MEDIUM | P1 |
| Restore history on open | HIGH | MEDIUM | P1 |
| Hash invalidation | HIGH | LOW | P1 |
| `persistent_undo.enabled` toggle | HIGH | LOW | P1 |
| `persistent_undo.max_entries` | MEDIUM | LOW | P1 |
| `persistent_undo.exclude` glob patterns | MEDIUM | LOW | P1 |
| Auto-prune nonexistent files | MEDIUM | LOW | P1 |
| File-watcher integration for external changes | MEDIUM | LOW | P2 |
| Time-based pruning | LOW | LOW | P2 |
| Non-blocking notification on history clear | MEDIUM | LOW | P2 |
| Remote reconnect undo survival | HIGH | HIGH | P3 |
| Undo tree visualization | LOW | HIGH | P3 |

**Priority key:**
- P1: Must have for launch
- P2: Should have, add when possible
- P3: Nice to have, future consideration

## Competitor Feature Analysis

| Feature | Vim/Neovim | VS Code | Emacs (undo-fu-session) | Our Approach |
|---------|------------|---------|------------------------|--------------|
| Persist across restart | Yes (`undofile`) | No (only within-session between tab switches) | Yes | Yes — this is the primary differentiator vs VS Code |
| Persist across tab close | Yes (`undofile`) | Yes (`files.restoreUndoStack`, in-memory) | Yes | Yes |
| Enabled by default | No (opt-in `set undofile`) | Yes (`files.restoreUndoStack: true`) | No (opt-in) | No (disabled by default per PROJECT.md) |
| File hash invalidation | Yes (documented in vimhelp.org) | Yes (file content check on restore) | Yes (length + checksum) | Yes — mandatory for correctness |
| File exclusion patterns | Via autocommands, undofile.vim plugin | Not for undo specifically | Yes (`undo-fu-session-incompatible-files`) | Yes (glob patterns via settings) |
| Entry count limit | `undolevels` (per session, not per file) | `workbench.localHistory.maxFileEntries` (for Timeline, not undo stack) | `undo-fu-session-file-limit` (per-file count) | Yes (`persistent_undo.max_entries`) |
| Auto-prune old files | Never (explicit caveat in docs) | Not applicable | Yes (removes oldest when limit hit) | Yes (prune on nonexistent files; consider time-based in v1.x) |
| Toggle on/off | Yes (`set undofile` / `set noundofile`) | Yes (`files.restoreUndoStack`) | Yes | Yes |
| Undo tree visualization | Plugin (undotree, vim-mundo) | No | Plugin (undo-tree.el) | Out of scope for v1 |
| Remote reconnect survival | N/A | N/A | N/A | Out of scope for v1; unique to Zed |

## Sources

- [VS Code 1.44 Release Notes — persisted undo/redo stack](https://code.visualstudio.com/updates/v1_44) — HIGH confidence (official docs)
- [VS Code Issue #95000 — Allow to disable persistent undo/redo stack](https://github.com/microsoft/vscode/issues/95000) — HIGH confidence (established pattern)
- [VS Code Issue #224682 — Persistent Undo/Redo Across Sessions (open)](https://github.com/microsoft/vscode/issues/224682) — HIGH confidence (confirms VS Code does NOT persist across restarts)
- [Vim/Neovim undo documentation (vimhelp.org)](https://vimhelp.org/undo.txt.html) — HIGH confidence (official docs, hash invalidation mechanism verified here)
- [undo-fu-session Emacs package (Codeberg)](https://codeberg.org/ideasman42/emacs-undo-fu-session) — HIGH confidence (package source, feature list verified directly)
- [Zed Discussion #16485 — Improve Undo, persist history buffer across tab closure](https://github.com/zed-industries/zed/discussions/16485) — HIGH confidence (primary user pain point, verified)
- [Zed Issue #4942 — Persist undo history across restarts](https://github.com/zed-industries/zed/issues/4942) — HIGH confidence (closed in favor of #15097, confirms scope)
- [Zed Issue #23164 — Zed should preserve file undo history when closed and opened again](https://github.com/zed-industries/zed/issues/23164) — HIGH confidence
- [GitHub mbbill/undotree — undo tree visualizer for Vim](https://github.com/mbbill/undotree) — MEDIUM confidence (describes plugin behavior)
- [Helix Issue #5287 — Save undo history through sessions](https://github.com/helix-editor/helix/issues/5287) — MEDIUM confidence (confirms cross-editor pattern)
- [Kakoune Issue #2021 — Persistent Undo](https://github.com/mawww/kakoune/issues/2021) — MEDIUM confidence

---
*Feature research for: Persistent undo/redo in Zed editor*
*Researched: 2026-03-01*
