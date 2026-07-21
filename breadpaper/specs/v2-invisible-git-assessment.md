# BreadPaper V2 — Invisible Git: safety & maintainability assessment

**Status:** Complete — go
**Date:** 2026-07-21 · **Spec:** `v2-invisible-git.md`

## 1. Verdict

The invisible checkpoint engine is **feasible, safe, and maintainable**. It shipped
as a self-contained `breadpaper::history` module driving a new general-purpose
checkpoint primitive in the `git` crate, verified by automated tests at the
plumbing level and by a live end-to-end run against a real vault. One material
surprise was found and fixed during verification: the spec's invisibility premise
("only an entry literally named `.git` is ever discovered") is **stale** — Zed's
worktree scanner also heuristically registers *bare* repositories, which
discovered our history repo the moment it was created (§4 below).

## 2. What shipped

- **`crates/git/src/repository.rs`** — the reusable plumbing (all next to the
  existing agent-checkpoint code it mirrors):
  - `RealGitRepository::new_with_separate_git_dir(git_dir, work_tree, …)` —
    opens a repo whose git-dir is not named `.git`; commands are scoped with
    `GIT_DIR`/`GIT_WORK_TREE` env vars (such repos are undiscoverable from cwd).
  - `RealGitRepository::init_separate_git_dir(…)` — `git init` with an external
    git-dir, leaving **nothing** named `.git` in the work tree.
  - `RealGitRepository::checkpoint_onto_ref(ref, message, author, max_bytes)` —
    the spec's checkpoint: temp index (real index never touched), the existing
    `checkpoint.gitignore` + size-cap excludes, `add --all` → `write-tree` →
    **no-op check against `HEAD^{tree}`** → `commit-tree -p HEAD` →
    `update-ref`. The ref advances only after the commit is fully written
    (crash-safety invariant §6.7). A successful commit is followed by
    `git gc --auto` (plumbing never triggers auto-GC on its own).
  - Small enablers: `GitBinary::envs` now extends instead of replacing;
    `exclude_files` takes the size cap as a parameter (default unchanged).
- **`crates/breadpaper/src/history.rs`** (new, ~470 lines) — the service:
  per-workspace `HistoryService` created from `observe_new::<Workspace>`
  (V1's activation pattern), detecting vaults per visible worktree, lazily
  initializing `<vault>/.breadpaper/history` on a background task, with
  idle-debounce + heartbeat + wired pre-AI-write (`checkpoint_before_ai_write`)
  + best-effort close/quit triggers, single-flight with one pending coalesced
  checkpoint, and failure handling that disables-and-logs-once (init) or
  logs-at-debug-and-retries (per-checkpoint).
- **`crates/breadpaper/src/vault.rs`** — `[history]` config table
  (`enabled`, `idle_debounce_seconds`, `heartbeat_minutes`, `max_file_bytes`),
  all defaulted, table optional.
- **`crates/worktree/src/worktree.rs`** — the invisibility fix (§4).
- **`assets/settings/default.json`** — `**/.breadpaper/history` added to
  `file_scan_exclusions`.
- **`crates/zed/src/main.rs`** — one line: `breadpaper::history::init(cx)`.

## 3. Upstream delta (the maintainability question)

| File | Nature | Size |
|---|---|---|
| `crates/git/src/repository.rs` | additive: new constructor + 2 methods + 2 tiny refactors; ~190 further lines are new tests | ~170 lines non-test |
| `crates/worktree/src/worktree.rs` | one guarded branch in the FS-event git probe | +20 |
| `assets/settings/default.json` | one default exclusion entry | +1 |
| `crates/zed/src/main.rs` | registration | +1 |

Everything else lives in the `breadpaper` crate. No existing function changed
behavior for existing callers (`envs` extend is only observable if a caller sets
overlapping keys, and none does; `exclude_files` callers pass the old constant).
All 62 pre-existing `git` crate tests pass unchanged. **Rebase outlook:** the
git-crate additions sit beside the agent checkpoint code and touch nothing that
churns often; the worktree guard is inside a stable event loop but is the one
hunk most likely to need hand-merging on a big upstream rebase. Verdict: small,
isolated, additive — comparable to V1's footprint per capability.

## 4. The invisibility finding (spec premise was stale)

Spec §4.2 assumed repositories are discovered only via entries literally named
`.git`. That is true of the *scan* path, but `worktree.rs` also has a
**bare-repository heuristic** (`is_dot_git`): during FS-event processing, any
ancestor directory containing `HEAD` + `config` files is registered as a git
dir. In the first live run, the moment the service ran `git init`, the event
probe registered `~/BreadPaper/.breadpaper/history` and the GitStore opened it
(confirmed twice in `Zed.log`) — it would have appeared in git UI.

**Fix:** the heuristic (and only the heuristic — literal `.git` is unaffected)
now skips paths covered by `file_scan_exclusions`, and `**/.breadpaper/history`
was added to the defaults. Rationale: a user who scan-excludes a directory has
declared it invisible to the workspace; auto-registering a repository from
inside it contradicts that. This is a general upstream rule, not a
BreadPaper-specific carve-out. Verified live post-fix: a full
init → initial checkpoint → idle checkpoint cycle produced **zero** repository
opens in Zed's log. *Caveat:* a user who wholesale-overrides
`file_scan_exclusions` without the history entry re-exposes the repo to the
heuristic; acceptable for V2, worth revisiting if it bites.

## 5. Safety results (tested trigger matrix)

Automated (`cargo test -p git`, new tests `test_checkpoint_onto_ref*`):

- Initial checkpoint on an unborn branch; second checkpoint parented on first
  (walkable linear chain, verified via `rev-parse checkpoints^`).
- **No-op discipline:** unchanged tree → no commit, no ref move.
- **Excludes:** a `max_file_bytes` file stays out of the snapshot while the
  checkpoint of everything else succeeds.
- **Restore (DoD 8):** `restore_checkpoint` returns edited files to the
  checkpointed state; files outside the snapshot are left alone.
- **User-repo coexistence (DoD 6):** with a real `.git` in the vault, a
  checkpoint leaves the user's status byte-identical, history and branches
  untouched, and `.git` never enters the snapshot.

Live run (dev build against the author's real `~/BreadPaper` V1 vault):

- Existing V1 vault upgraded transparently: `history/` created lazily, initial
  checkpoint recorded (`checkpoint: initial <utc>`, author `BreadPaper`),
  zero visible side effects (DoD 1).
- Edit → idle checkpoint within the debounce window (DoD 2); an edit racing the
  in-flight initial checkpoint was captured by it and the subsequent idle
  attempt correctly no-opped — the coalescing design working as intended.
- 45+ seconds of quiescence produced **no** checkpoint churn: the engine's own
  writes never re-trigger it (DoD 4).
- `SIGTERM` mid-session (crash-equivalent): **no corruption** (`git fsck`
  clean), previous head intact; the interrupted edit was caught up by the next
  launch's initial checkpoint, which continued the same chain. This is §6.7's
  invariant observed in practice. Note: the close trigger is wired via
  `on_app_quit` + service-drop and is genuinely *best-effort* — a SIGTERM kill
  bypasses it by design; nothing already committed is ever lost.
- Failure modes: missing git binary / init failure → service disables for that
  vault, logs once, editing unaffected (exercised in code review + the error
  path is the same one the tests drive through `Result`); per-checkpoint
  failure logs at debug and re-marks dirty so the heartbeat retries.

Does it ever lose data? **No vault data is ever at risk** — the engine only
reads the work tree; the only write into the vault is `git restore`, which V2
never invokes outside tests. History-side, at-risk windows are the gaps between
triggers (by design, bounded by the heartbeat) and edits made in the final
seconds before a hard kill (caught up at next launch).

## 6. Known limitations (accepted for V2)

- Media files (`*.png`, `*.jpg`, …) are excluded by the reused
  `checkpoint.gitignore` **regardless of size**, not just at ≥2 MB — matches
  spec §6.5's "baseline + size cap", but worth restating: vault images are not
  versioned at all in V2 (spec §8.3).
- Two windows on the same vault run two services against one repo; git's ref
  locking makes the race safe (one side may log a failed checkpoint and retry).
  Same-vault-in-two-processes remains out of scope (spec §3).
- Heartbeat is polled at 60 s granularity, so a heartbeat checkpoint can land
  up to a minute after the configured floor elapses.
- `is_dot_git`'s heuristic guard depends on `file_scan_exclusions` defaults
  (§4 caveat).

## 7. Suggested `.rules` additions

- "Zed's worktree scanner registers git repositories not only for entries named
  `.git` but for any directory containing `HEAD` + `config` (bare-repo
  heuristic in `is_dot_git`, FS-event path). Anything that creates a repo-shaped
  directory inside a worktree must pair it with a `file_scan_exclusions` entry,
  which the event probe now honors."
- "Shelling out to git anywhere in the workspace must go through
  `GitBinary::build_command` — clippy denies `new_command` for this via
  `disallowed-methods`."
