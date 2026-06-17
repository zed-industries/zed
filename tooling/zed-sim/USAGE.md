# Zed Sim — How to Use

Zed Sim is an internal, staff-only tool for experiencing Zed in different
account states — a **truly fresh first-run**, **Pro**, **on-trial**, or
**trial-expired** — without creating accounts and without touching production.
It runs the *real* editor in a disposable profile; only the account/plan state
is set for you.

For the why and the design, see `SPEC.md` and `INJECTION_PLAN.md`.

---

## Prerequisites

- A checkout of this branch (`zed-sim-injection`).
- The Rust toolchain (same as building Zed normally).
- macOS, Apple Silicon (what the team builds on today).

## Quick start

From the repo root:

```sh
./tooling/zed-sim/run.command
```

That builds the staff Zed and opens a small control panel in your browser. Pick
a state, click **Launch**.

> **The first run does a full Zed build (10–30 min).** After that it's fast.
> You can also double-click `run.command` in Finder.

Prefer to do it by hand?

```sh
cargo build -p zed --features staff-sim   # build the staff binary once
cargo run -p zed-sim                       # start the launcher (auto-finds it)
```

## The states

| State | What you get |
|---|---|
| **Brand-new user** | A pristine first-run: signed out, onboarding, nothing configured. |
| **Signed in** | A fresh profile that goes through the real sign-in flow. |
| **Pro** | Signed in on Pro. |
| **Pro Trial — active** | Mid-trial UX. |
| **Pro Trial — expired** | The end-of-trial upsell flow. |

Each launch is its own disposable session. Click **Wipe scratch profiles** (or
delete `<temp>/zed-sim`) to reset.

## ⚠️ The one thing people miss

**The trial-end upsell only appears inside the Agent panel.** After launching
**Pro Trial — expired**, open the Agent panel — the upsell renders there, not in
the main editor.

## Is this safe? (yes)

The injected states are **fully offline**: no token, no credentials, no network
fetch, no collab connection, and telemetry is written off in each session. The
fake identity exists only in local memory — **nothing reaches production or your
analytics, and nothing can modify any real account.**

## Troubleshooting

- **An injected state opens a signed-*out* Zed.** The launcher used the wrong
  binary. Check the terminal line `Using Zed binary: …` — it should point at
  `…/target/debug/zed` in your checkout, not `/Applications`. Make sure you ran
  `cargo build -p zed --features staff-sim` (or used `run.command`, which does).
- **The trial-expired upsell didn't show.** Open the **Agent panel** (see above).
  It also hides if a non-Zed model is set as your default — use a fresh profile.
- **First launch is slow to open.** It's a debug build; the window takes a few
  seconds. Subsequent launches are quicker.

## Not included yet

- **Business (member/admin)** — deferred; needs organization modelling.
- **Impersonation** (real preview accounts) — present but parked until a preview
  backend + token exist; the section shows as disabled.
- Injected states are **cosmetic by design** — they reproduce what a user *sees
  and experiences*, not server-side enforcement (e.g. actually hitting a usage
  cap). That matches the tool's goal.

## Direct command (no launcher)

```sh
ZED_SIM_STATE=trial_expired ./target/debug/zed --user-data-dir /tmp/zsim
```

Swap `trial_expired` for `pro`, `trial`, or `free`.
