# V1 Daily Panel — Maintainability Assessment

**Status:** Complete — this is the go/no-go deliverable required by `v1-daily-panel.md` §2.7
**Author:** Claude (implementation session) · **Date:** 2026-07-20
**Verdict: GO.** The custom-panel approach is viable and the fork tax looks low.

## 1. How invasive is the core diff?

All feature logic lives in a new, self-contained crate (`crates/breadpaper/`, four
files: vault config/discovery/scaffolding, date formatting + template expansion,
the GPUI timeline panel, and the startup hook). The delta against upstream Zed
(after the V1-final scope: Timeline header with Today/Yesterday/This Week/Last
Week, `breadpaper:` open commands, panel open-by-default in vaults, classic
layout as the default) is **8 files, +44/−27 lines**, and every touched line is
either a registration one-liner, a default-value flip, or the one deliberate
behavior change:

| File | Change | Risk on rebase |
|---|---|---|
| `Cargo.toml` (root) | +2: workspace member + dependency entry | Trivial; alphabetical lists merge cleanly |
| `Cargo.lock` | +18: lockfile entry for the new crate | Auto-regenerable |
| `crates/zed/Cargo.toml` | +1: `breadpaper.workspace = true` | Trivial |
| `crates/zed/src/zed.rs` | +11: `TimelinePanel::load` + `join!` arm + post-join `show_panel_if_vault` call in `initialize_panels` | Low; the panel-loading pattern is stable upstream |
| `crates/zed/src/main.rs` | +1 `breadpaper::init(cx)`; −20/+5 in `restore_or_create_workspace` | **The one real conflict surface** (see below) |
| `assets/keymaps/default-{macos,linux}.json` | +1 each: `breadpaper::ToggleFocus` binding | Trivial |
| `assets/settings/default.json` | 5 dock defaults flipped so new users get the classic (editor-focused) layout | Low; value-only changes, though upstream layout reshuffles would need re-flipping |

The only *behavioral* upstream change is in `restore_or_create_workspace`: when
there is no session to restore, BreadPaper opens/scaffolds the default vault
instead of Zed's onboarding view or an empty workspace. This replaces two
branches (~20 lines) and removes the now-unused `onboarding` import. If upstream
reworks that function, the rebase will conflict *there and only there* — and the
resolution is mechanical (re-point the final `else` at
`breadpaper::open_startup_vault`).

## 2. Does it rebase cleanly?

Tested against `upstream/main` (zed-industries/zed) via `git merge-tree`:
**clean, zero conflicts**. Caveat: the fork is currently only one commit behind
upstream, so this is a weak empirical signal. The structural argument above is
the stronger one — six of the seven touched files are append-only registration
points that upstream churns rarely and merges trivially.

Recommended hygiene going forward: rebase weekly while the delta is this small,
so any conflict in `restore_or_create_workspace` is caught while the upstream
change is fresh.

## 3. What the runtime shook out (traps for the next panel)

- **`activation_priority` must be globally unique.** This fork's dock asserts it
  (`dock.rs:719`) and panics at startup on collision. Taken: 0 agent, 1 project,
  2 terminal, 3 git, 5 collab, 6 outline, 7 debug. The timeline panel uses **4**
  — the last free small integer. The next BreadPaper panel needs 8+.
- Panels need no settings registration and no DB persistence to work; hardcoding
  position/size (à la `TestPanel`) keeps the panel fully self-contained. The
  cost: dock position/size aren't remembered across restarts. Fine for V1.
- The fork's `AsyncApp::update` is infallible (returns `R`, not `Result<R>`) —
  upstream blog examples and older code suggest otherwise.

## 4. Deliberate V1 shortcuts (recorded, not hidden)

- **`std::fs` instead of Zed's `Fs` trait** for vault I/O, following the
  `journal` crate's precedent. Simpler, but invisible to `FakeFs`-based GPUI
  tests — the file logic is covered by 11 unit tests against real temp dirs
  instead. Worth revisiting when invisible-git arrives (it will want the `Fs`
  trait's atomic writes anyway).
- **Vault detection runs on the foreground thread** (a stat + ~200-byte read on
  worktree events). Cheap, but should move to the background if config files
  ever grow.
- Zed's onboarding flow is unreachable rather than removed — smallest possible
  diff, dead code accepted.
- Keybinding is `cmd-alt-t` / `ctrl-alt-t` ("t" for today). `ctrl-alt-t` may
  collide with GNOME's terminal shortcut on Linux; acceptable for a
  macOS-first V1.

## 5. Verification record

- 11 unit tests (vault detect/scaffold/no-clobber, config defaults, date tokens,
  template expansion, note create/never-overwrite/missing-template) — all pass.
- `cargo check` + `cargo clippy --deny warnings` clean for `breadpaper` and
  `zed` (clippy run in debug profile scoped to the two crates; the repo's
  `script/clippy --release --workspace` didn't fit on available disk).
- Live first-run test (fresh `$HOME`): sample vault scaffolded at
  `~/BreadPaper/` with config, template, and `welcome.md`; workspace opened;
  first frame rendered; no panics. Today/Yesterday clicks created
  `daily/2026-07-20.md` / `daily/2026-07-19.md` with correct per-note date
  substitution.
- Live non-vault test: opening a plain folder shows the panel's
  not-a-vault state without crashing and scaffolds nothing.

## 6. Go/no-go rationale

The spec asked whether a custom GPUI panel can be added and **kept** without
pain. Evidence says yes: the panel itself required zero upstream modifications;
registration is seven mostly-one-line touches; the sole behavioral override is
localized to one function with a mechanical resolution strategy. The pane-heavy
vision (Areas pane, Context pane) can proceed on this pattern — each new panel
costs ~4 upstream lines plus a unique priority.
