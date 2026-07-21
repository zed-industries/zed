# BreadPaper V2 — Invisible Git (checkpoint engine)

**Status:** Scope-locked, ready for implementation
**Owner:** Diego · **Date:** 2026-07-21
**Companion docs:** `../VISION.md` (principle 7, §7.2), `v1-daily-panel.md`, `v1-daily-panel-maintainability.md`

---

## 1. Summary

V2 gives every BreadPaper vault a **silent, always-on version history**. A background service watches the open vault and commits **checkpoints** of the whole vault into a **hidden, isolated git repository** — triggered by idle pauses, a slow heartbeat, and (by design) before any AI write. The user never types a git command, never sees a git pane, and never sees the word "git." History simply accrues underneath, so a later increment can offer jargon-free time-travel.

This is the **engine only**. V2 ships **no user-facing surface at all** — no restore command, no history browser, no toast, no `.git` in the file tree. That is a deliberate scope choice (see §3): prove the versioning engine is safe and invisible first; build the "restore this version" UX on top of it in V3.

Like V1, this is a **feasibility gate**. V1 asked "can we add a maintainable custom panel?" V2 asks: **can a background service checkpoint the vault reliably — never corrupting or losing data, never surfacing in Zed's git UI, and with a small, rebaseable core diff?** The good news, established during scoping: Zed's core *already* contains the exact primitive we need (§7.1), so V2 is mostly wiring, not new machinery.

## 2. Goals & success criteria

**Primary (the go/no-go):** Prove that an invisible checkpoint service is **safe and maintainable**. Safe = it never corrupts the vault, never destroys user edits, never blocks or slows editing, and never leaks into any git-facing UI. Maintainable = a small, isolated diff that leans on existing upstream checkpoint plumbing and survives a rebase — judged on the *final, real* implementation.

**Secondary (it must actually work):** Every meaningful state of the vault is recoverable. After a day of editing there is a walkable chain of checkpoints covering idle pauses and AI-write boundaries, and a checkpoint can be restored **at the plumbing level** (proven by test), so V3's UI is de-risked before a single button is drawn.

**Definition of done:**
1. Opening a valid vault with no history → the hidden history repo is initialized and an **initial checkpoint** of current state is recorded — with zero visible side effects (no git-pane entry, no `.git` in the tree, no notification).
2. Edit notes, then pause → a checkpoint is recorded within the idle-debounce window. (Dev-verifiable via `git --git-dir=<vault>/.breadpaper/history log`.)
3. Edit continuously without pausing → the **heartbeat** floor still produces at least one checkpoint within the configured interval.
4. **No-op discipline:** no checkpoint is created when the vault tree is unchanged since the last one (no empty commits, no churn).
5. A large binary/media file in the vault is **excluded** from the snapshot, and the checkpoint still succeeds.
6. A vault that is *itself* a user-managed git repo (has its own `.git`) → our history repo **coexists**: the user's `.git`, index, branches, and status are never touched, and Zed's git UI still shows only the user's repo — never ours.
7. `git` binary unavailable / repo-init failure / write-lock error → editing is completely unaffected; the service disables or retries gracefully and logs once (never a user-facing error, never a partial/corrupt commit).
8. `restore_checkpoint` works against the history repo, proven by an automated test (no UI ships).
9. A written **safety & maintainability assessment** (the real deliverable of the spike): does it ever lose data, does it stay out of git_ui, how big is the core diff, and does it rebase cleanly?

## 3. Non-goals (explicitly out of V2)

- **Any restore / undo / time-travel UI.** No history panel, no diff viewer, no "restore this version" command in the palette. This is the single biggest scope cut and it is intentional — the engine is invisible in V2, full stop. (V3.)
- **Conflict-recovery UX.** V2 assumes single-user, single-machine, single-process editing of a vault. Cross-device sync and merge/conflict handling are a separate, harder project (VISION §9).
- **Pushing to a remote / cross-device history sync.** The repo is local-only.
- **Pruning / GC policy UI or retention tuning.** We lean on git's own `--auto` GC (§6.6) and revisit only if growth becomes a real problem.
- **Removing Zed's git pane** (VISION §6 "Removed"). Orthogonal to this increment; our job is to not *appear* in it, not to delete it.
- **Versioning large binaries well.** V2 excludes them (§6.5); real large-file/media history is later.
- **Checkpointing non-vault workspaces.** The service only activates for a recognized vault, exactly like the panel.
- **Surfacing "pre-AI-write" checkpoints from third-party extensions/MCP.** The trigger is designed-for and wired to any *first-party* write path, but BreadPaper does not yet ship AI rituals (those are extensions — VISION §7.1), so in V2 this trigger is a ready hook, not a guaranteed interception of every AI write. Recorded honestly in §6.2.

## 4. Core concepts

### 4.1 Checkpoint
A **checkpoint** is a full snapshot of the vault's working tree at a moment in time, stored as a git commit in the hidden history repo. Checkpoints form a **linear chain** (each parented on the previous), so history is walkable for V3. A checkpoint is cheap: git stores only changed blobs, and unchanged snapshots are skipped entirely (§6.4).

### 4.2 The hidden history repo
Each vault has its own git repository whose **git-dir is `<vault>/.breadpaper/history`** and whose **work-tree is the vault root**. It is a normal git repo in every respect except two, both of which make it invisible:

- **It is not named `.git`.** Zed's worktree scanner registers a repository only when it finds an entry literally named `.git` inside a worktree (`worktree.rs` `DOT_GIT` match, `insert_git_repository`). A git-dir named `history` under the already-hidden `.breadpaper/` directory is therefore **never discovered**, never enters `GitStore.repositories`, and never appears in `git_ui`.
- **It is fully isolated from any user git.** We drive it with an explicit git-dir + work-tree and our own index, so if the vault *also* has a real `.git`, the two never interact.

`.breadpaper/` is already hidden and already the designated home for "invisible-git state" (VISION §4.4). Keeping history inside the vault means it travels with the vault (copy the folder, keep the history).

### 4.3 Invisible ≠ dangling (a clarifying divergence from Zed's agent checkpoints)
Zed's existing checkpoint feature keeps commits **dangling** (no ref) specifically so they don't disturb the *user's real repo* that the git UI is watching. Our history repo is a different situation: **nothing is watching it**, because it's never discovered (§4.2). So we don't need danglingness for invisibility — and we actively *don't want* it, because V3 needs a walkable chain. V2 therefore maintains a **real ref that advances** (e.g. `HEAD` on a `checkpoints` branch) with each checkpoint parented on the last. Invisibility comes from **repo isolation**, not from orphaning commits.

## 5. Storage layout & config additions

### 5.1 Layout additions (per vault)
```
<vault-root>/
  .breadpaper/
    config.toml          # V1 marker + config (gains an optional [history] table)
    history/             # NEW: the hidden git-dir for this vault's checkpoint repo
      HEAD, objects/, refs/, ...   # a normal git-dir, just not named .git
  daily/  templates/  welcome.md   # (V1, unchanged)
```
`history/` is created lazily by the service on first activation (§6.1), so **existing V1 vaults are upgraded transparently** on next open — no re-scaffold required.

### 5.2 `config.toml` additions (V2)
```toml
[history]
enabled              = true       # master switch; omit table entirely = same as true
idle_debounce_seconds = 20        # commit this long after edits stop
heartbeat_minutes     = 5         # floor: commit if dirty and this long since last checkpoint
max_file_bytes        = 2000000   # skip blobs larger than this (matches Zed's checkpoint default)
```
Every key has a sane default; the feature works with the `[history]` table entirely absent. `[history].enabled = false` fully disables the service for that vault.

## 6. Behavior specification

### 6.1 Activation & first init
- The service activates **only for a recognized vault** — it reuses V1's `Vault::detect` and observes new `Workspace`/`Project` entities, mirroring how the timeline panel wires up.
- On activation, if `<vault>/.breadpaper/history` does not exist → `git init` it as a **separate git-dir with work-tree = vault root**, set the author identity to a fixed BreadPaper identity (never the user's git identity), and record an **initial checkpoint** of the current vault state.
- If it already exists → open it and continue the existing chain.
- All of this happens on a **background task**; the foreground/editing path is never blocked.

### 6.2 Triggers (conservative / milestone — confirmed)
A checkpoint is attempted on any of:

| Trigger | Fires when | Rationale |
|---|---|---|
| **Idle debounce** | Vault files stopped changing for `idle_debounce_seconds` | The primary trigger; captures a natural "I paused" boundary without a commit-per-keystroke explosion. |
| **Heartbeat floor** | Vault is dirty **and** `heartbeat_minutes` elapsed since the last checkpoint | Guarantees progress even during long uninterrupted editing (idle never fires). |
| **Pre-AI-write** | Immediately before a **first-party** AI/skill write to the vault | Guarantees a clean pre-mutation restore point. In V2 this is a **wired hook** (see §3) — active whenever a first-party write path exists; not yet a guarantee over third-party extension writes. |
| **Workspace close / app quit** | Vault worktree closes or app shuts down, if dirty | Best-effort final safety net; must not delay shutdown (bounded, fire-and-forget). |

Edit detection reuses existing plumbing — either the autosave debounce (`workspace` `AutosaveSetting` / `DelayedDebouncedEditAction`) or `Worktree::observe_updates` on the vault worktree. Checkpoints are **serialized** (a single-flight queue): overlapping triggers coalesce into at most one in-flight checkpoint plus at most one pending.

### 6.3 What a checkpoint captures
The entire vault working tree, **minus excludes** (§6.5), snapshotted via a **temporary index** so the real/user index is never touched (the pattern Zed's `checkpoint()` already uses: copy-to-temp-index → `add --all` under excludes → `write-tree` → `commit-tree -p <prev>` → advance our ref). Commit messages carry the trigger + timestamp (e.g. `checkpoint: idle 2026-07-21T14:31:07Z`) so V3 can label points meaningfully.

### 6.4 No-op discipline
Before committing, compare the freshly written tree hash to the last checkpoint's tree. **Identical → do nothing** (no commit, no ref move). This keeps the heartbeat and close triggers from producing empty/duplicate history.

### 6.5 Excludes (what never enters history)
- **The history git-dir itself** (`.breadpaper/history/**`) — mandatory, or the repo snapshots its own objects and grows without bound.
- **Any user `.git`** present in the vault.
- **Large blobs** ≥ `max_file_bytes` (default 2 MB) — the vault is known to hold multi-MB images (VISION §9); they're skipped so history stays lean and fast.
- The **binary/media/DB/IDE exclude set** Zed already ships for checkpoints (`crates/git/src/checkpoint.gitignore`) is reused as the baseline.

Exclusion is best-effort and never fatal: an over-large or binary file simply isn't in the snapshot; the checkpoint of everything else still succeeds.

### 6.6 Repo growth / GC
Rely on git's built-in `gc --auto` semantics (invoked opportunistically off the background task, never on the editing path). Explicit retention/pruning policy is a non-goal (§3); revisit only if real-world growth demands it. Record observed growth in the assessment.

### 6.7 Failure modes (never surface to the user, never corrupt)
| Condition | Behavior |
|---|---|
| `git` binary missing/unusable | Disable the service for the session; log **once**; editing fully unaffected. |
| `history/` init fails (permissions, disk) | Disable for that vault; log once; no retry storm. |
| A single checkpoint fails (lock, transient I/O) | Log at debug; **leave prior history intact**; retry on the next trigger. Never leave a half-written ref/index. |
| Vault becomes non-vault / worktree closes | Tear down the task cleanly. |
| Disk full | Skip the checkpoint; never partially write; surface nothing (editing must not break). |

Invariant: **a checkpoint failure can never damage the vault or existing history.** Because commits are content-addressed and the ref only advances after a successful `commit-tree`, a crash mid-checkpoint leaves the previous checkpoint as the head.

## 7. Implementation notes (for engineering)

### 7.1 Reuse the existing checkpoint primitive — this is the maintainability win
Zed core already implements exactly the mechanism we need, currently used only by the agent:
- `GitRepository::checkpoint()` / `restore_checkpoint()` / `compare_checkpoints()` (`crates/git/src/repository.rs`), implemented in `RealGitRepository` via a **temp index** (`with_temp_index`, `GIT_INDEX_FILE`), runtime excludes (`checkpoint.gitignore` + the ≥2 MB rule), then `add --all` → `write-tree` → `commit-tree -p HEAD`. Author is forced via `checkpoint_author_envs()`.

Prefer to **drive this existing plumbing** against our isolated repo rather than re-implement git access. Zed shells out to the `git` binary through `GitBinary`/`RealGitRepository`; there is currently **no `GIT_DIR`/`GIT_WORK_TREE` env plumbing** (commands are scoped by `current_dir` + `GIT_INDEX_FILE`). Two viable paths, pick the smaller-diff one at implementation time:
  - **(a)** Construct a `RealGitRepository` pointed at `.breadpaper/history` (via `Fs::git_init` / `Fs::open_repo`) and adapt the checkpoint calls to advance a ref instead of dangling. Keeps everything in existing code paths.
  - **(b)** Add a minimal `GIT_DIR`/`GIT_WORK_TREE` injection to `GitBinary::build_command` and reuse the checkpoint methods verbatim. Slightly more invasive upstream, but tiny and general.

Either way, the **feature logic lives in the `breadpaper` crate** (a new self-contained module alongside `vault.rs` / `notes.rs` / `timeline_panel.rs`), keeping the upstream delta to registration + at most a small, general git-dir plumbing addition — the V1 pattern where "each new capability costs a few upstream lines."

### 7.2 Registration
Initialize alongside V1: a `breadpaper::history::init(cx)` next to `breadpaper::init(cx)` (`crates/zed/src/main.rs` ~line 744), and/or per-workspace wiring in `crates/zed/src/zed.rs` where vault detection and `show_panel_if_vault` already live. The service observes new `Workspace`/`Project` entities, detects a vault, and spawns a background checkpoint task per valid vault worktree (multiple open vaults → independent tasks/repos).

### 7.3 Use the `Fs` trait, not `std::fs`
The V1 maintainability assessment explicitly flagged that invisible-git "will want the `Fs` trait's atomic writes anyway." Any metadata BreadPaper writes itself (e.g. a small last-checkpoint marker, if needed) should go through Zed's `Fs` trait for atomicity, rather than V1's `std::fs` shortcut.

### 7.4 Threading & safety
All git work runs on `cx.background_spawn`. Never touch a real index (temp index only). Serialize checkpoints per vault (single-flight + coalesce). The ref advances **only** after a successful `commit-tree`, guaranteeing crash-safety (§6.7 invariant).

### 7.5 No panel, no priority
V2 adds **no** dock panel, so it consumes no `activation_priority` (the next BreadPaper panel still starts at 8 per the V1 assessment). There is no keybinding and no menu item — that's the point.

### 7.6 The deliverable
As in V1, the spike's real output includes the **safety & maintainability writeup** (§2): upstream files touched, whether it rebases cleanly, whether it ever loses data across the tested trigger matrix, and confirmation via `git_ui` inspection that the history repo is never discovered.

## 8. Open assumptions to confirm on review

1. **Repo location** = `<vault>/.breadpaper/history` (in-vault, travels with the folder), vs. an external store like `~/.local/share/breadpaper/checkpoints/<vault-hash>/` (keeps the vault directory pristine, but history doesn't travel). Spec assumes **in-vault**, per VISION §4.4.
2. **Trigger defaults** — `idle_debounce_seconds = 20`, `heartbeat_minutes = 5`. Acceptable, or tune?
3. **`max_file_bytes = 2 MB`** (matches Zed's default). The author's vault holds multi-MB images by design — confirm we're comfortable that those images are **not** versioned in V2.
4. **Pre-AI-write trigger status** — V2 wires the hook but ships no first-party AI writes (§3, §6.2). Confirm that "designed-for, not yet guaranteed over extension writes" is acceptable for this increment.
5. **Restore proof** — V2 proves `restore_checkpoint` via an automated test only, with **no** user-facing command. Confirm no hidden/debug restore affordance is wanted (even behind a flag).
6. **GC** — relying on git `--auto` with no explicit retention policy (§6.6). Acceptable for V2?

## 9. Decision log (from design interview, 2026-07-21)

- V2 = **engine only, zero user-facing surface**; restore/time-travel deferred to V3. "Invisible to users" taken literally for this increment.
- Triggers = **conservative / milestone**: idle-debounce + heartbeat floor + pre-AI-write (designed) + best-effort close/quit. No commit-per-save.
- History lives in a **per-vault hidden repo** at `.breadpaper/history` (git-dir not named `.git`), **invisible via non-discovery**, isolated from any user `.git`.
- **Invisible ≠ dangling**: unlike Zed's agent checkpoints, we keep a **real advancing ref** so history is walkable for V3; invisibility comes from repo isolation.
- **Reuse Zed's existing checkpoint plumbing** (`RealGitRepository::checkpoint`/`restore_checkpoint`, temp-index + `commit-tree`) — the reason V2 is mostly wiring, not new machinery. Feature logic stays in a self-contained `breadpaper` module; upstream delta stays small and rebaseable.
- **No-op discipline** (skip unchanged trees) and the **crash-safe "ref advances only on success"** invariant are load-bearing safety requirements, not optimizations.
- Large binaries are **excluded, not versioned**, in V2.
- Safety **and** maintainability writeup is the go/no-go deliverable, as in V1.
