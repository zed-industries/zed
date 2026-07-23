# Zed Sim — Build Plan

A step-by-step plan for the agent/engineer to follow. Companion to `SPEC.md`.
This plan targets **Phase 1 (MVP)** in detail and sketches Phase 2 so the MVP is
built to grow into it.

## Goal of the MVP

A standalone launcher that runs the **real, stock Zed binary** in an isolated,
disposable environment, in one of two states:

1. **Brand-new user** — fresh first-run, signed out.
2. **Signed in** — fresh profile + the real sign-in flow.

No changes to the Zed application are required for the MVP. All MVP behavior is
achieved by controlling how Zed is *launched*.

## Branch

Work on a new branch: `zed-sim-mvp`.

## How the MVP works (grounded in existing Zed behavior)

These are the exact levers the launcher uses. All already exist in the codebase:

- **Isolated state dir** — Zed accepts `--user-data-dir <DIR>`, which roots
  config, data, db, and logs under one folder.
  - CLI arg: `Args.user_data_dir` in `crates/zed/src/main.rs`.
  - Implementation: `paths::set_custom_data_dir` in `crates/paths/src/paths.rs`
    (sets `config_dir`, `data_dir`, `database_dir`, `logs_dir` under the custom root).
  - => Launch with a fresh empty dir = a genuinely fresh install.
- **First-run / onboarding** — onboarding shows when the `FIRST_OPEN` key is
  absent from the key-value store (a fresh `--user-data-dir` has it unset).
  - See `onboarding::FIRST_OPEN`, used in `restore_or_create_workspace` and
    `open_workspaces` in `crates/zed/src/...`.
  - => Brand-new user state needs nothing beyond a fresh dir + signed out.
- **Keychain isolation** — credentials live in the OS keychain, keyed by the
  `credentials_url` setting (defaults to `server_url`). This is the *one* piece
  not covered by `--user-data-dir`.
  - Fields live in `SettingsContent` (`crates/settings`): `credentials_url`,
    `server_url`.
  - => For the **Signed in** state, write a unique `credentials_url` into the
    profile's `settings.json` so the disposable session does NOT read or clobber
    the user's real saved login. This field exists precisely to run instances
    side by side without keychain collisions.

## Crate / file layout

Create an in-repo Rust binary under `tooling/zed-sim/` (matches `tooling/xtask`,
`tooling/perf` conventions):

```
tooling/zed-sim/
  Cargo.toml          # bin crate; clap + a tiny HTTP server + serde
  src/
    main.rs           # entry: parse args, start local web UI, open browser
    server.rs         # minimal HTTP server: serve index, handle POST /launch
    launch.rs         # profile creation + Zed process launch
    profile.rs        # temp-dir lifecycle, settings.json writing, cleanup
    states.rs         # the State catalog (enum + metadata for the UI)
  ui/
    index.html        # state list + Launch button (inline CSS/JS, no build step)
  SPEC.md
  PLAN.md
  README.md           # how staff run it
```

Add `tooling/zed-sim` to the workspace `members` in the root `Cargo.toml`.
Keep dependencies minimal and from the workspace where possible (`clap`,
`serde`, `serde_json`, `anyhow`). For the HTTP server prefer something tiny
already in the lockfile (e.g. `tiny_http`, which `client` already uses) rather
than pulling a new framework.

## States (MVP)

Model as an enum in `states.rs`, each with a UI label, description, and a
`LaunchConfig`:

```rust
enum SimState {
    NewUser,    // signed out, fresh
    SignedIn,   // fresh + real sign-in flow
}
```

`LaunchConfig` (MVP fields):
- `fresh_profile: bool` (always true for MVP)
- `isolate_credentials: bool` (true for SignedIn => write unique `credentials_url`)
- `server_url: Option<String>` (None => default prod; reserved for preview later)

The catalog should also *list* the Phase 2 states (Pro, Trial, Trial-expired,
Business) as disabled/"coming soon" entries so the UI is built for the full set.

## Implementation steps

1. **Scaffold the crate.** `Cargo.toml` with `[[bin]]` (or `[lib] path` per repo
   rules — prefer an explicit path, e.g. `path = "src/main.rs"`). Wire into the
   workspace. Confirm `cargo build -p zed-sim` succeeds with a stub `main`.
2. **Locate the Zed binary.** Add a config step that resolves the Zed executable:
   - Accept `--zed <path>` and/or a `ZED_SIM_BINARY` env var.
   - Default discovery on macOS: `/Applications/Zed.app/Contents/MacOS/zed`
     (also check `Zed Preview.app`, `Zed Nightly.app`).
   - Fail with a clear message if not found.
3. **Profile lifecycle (`profile.rs`).**
   - Create a temp dir (e.g. under the system temp or `tooling/zed-sim/.profiles/`).
   - For `isolate_credentials`, write `<profile>/config/settings.json` containing
     a unique `credentials_url` (e.g. `"zed-sim://<uuid>"`). Note the config dir
     is `<user-data-dir>/config` per `paths::config_dir`.
   - Provide a `cleanup()` that deletes the temp dir.
4. **Launch (`launch.rs`).**
   - Spawn the Zed binary with `--user-data-dir <profile>` (+ any future env).
   - Do NOT block on it; return once spawned so the UI stays responsive.
   - Log the launched profile path so the user can find/reset it.
5. **Local web UI (`server.rs` + `ui/index.html`).**
   - On start, bind `127.0.0.1:0`, print the URL, and open it in the browser.
   - `GET /` serves `index.html`: a vertical list of states (label + one-line
     description) each with a **Launch** button; Phase 2 states shown disabled.
   - `POST /launch` with `{ "state": "new_user" | "signed_in" }` creates a
     profile and launches Zed; respond with the profile path / status.
   - Add a **"Wipe scratch profiles"** button => `POST /cleanup`.
   - Keep all CSS/JS inline in `index.html` — no front-end build step.
6. **README.** Short "how to run": `cargo run -p zed-sim`, then click states.
   Document the `--zed` / `ZED_SIM_BINARY` override.
7. **Manual verification** (see Acceptance criteria).

## Acceptance criteria (MVP)

- `cargo run -p zed-sim` opens a local page listing the states.
- Clicking **Brand-new user** launches Zed showing first-run onboarding, signed
  out, with none of my real settings/extensions/keymap present.
- Clicking **Signed in** launches a fresh Zed where I can complete the real
  sign-in flow, and doing so does **not** alter my primary Zed's saved login.
- Launching a state twice yields two independent fresh sessions.
- **Wipe** removes all scratch profiles created by the tool.
- My real `~/Library/Application Support/Zed`, settings, and primary keychain
  login are untouched throughout.

## Phase 2 (sketch — do not build yet)

Fabricated signed-in plan states (Pro, Trial, Trial-expired, Business). Requires
a small change in Zed itself, gated behind a staff-only build flag.

- Today `ZED_SIMULATE_PLAN` (`crates/client/src/user.rs`, `UserStore::plan`) is
  `#[cfg(debug_assertions)]` and only covers `plan()` (free/trial/pro).
- Expand into a richer **state profile** read at startup (env var or a file in
  the user-data-dir) that can populate the relevant `PlanInfo` fields:
  `plan`, `trial_started_at`, `subscription_period`, `is_account_too_young`,
  `has_overdue_invoices`, and `usage` — plus a fake "signed-in" identity so
  signed-in-only UI renders without a real connection.
  - Plan enum: `crates/cloud_api_types/src/plan.rs`.
  - `PlanInfo`: same crate (`plan.rs`).
- Put it behind a cargo feature (e.g. `staff-sim`), and add **one** CI line to
  emit a "Zed Sim" artifact next to nightly builds. The launcher auto-fetches it
  like an updater — no manual build management for staff.
- Security posture: purely local/cosmetic, no production access, no master key.

## Impersonation (Phase 1.5 — BUILT)

Real accounts via impersonation of pre-made **preview** accounts, by username
only (no GitHub OAuth). Implemented in `config.rs` + `server.rs`.

- Levers: `ZED_IMPERSONATE` + `ZED_ADMIN_API_TOKEN` + `ZED_SERVER_URL`, set as
  env on the launched process => `authenticate_as_admin` =>
  `POST /internal/users/impersonate` (`crates/client/src/client.rs`). Not
  debug-gated; works on the stock binary.
- The cloud API host is derived from `server_url` by
  `HttpClientWithUrl::build_zed_cloud_url` (`localhost:3000`->`localhost:8787`,
  otherwise same host), so pointing at preview = setting `ZED_SERVER_URL`.
- The endpoint resolves the username via GitHub's public API and find-or-creates
  the user, so any real public GitHub username works and need not be owned.
- Config: a gitignored `zed-sim.config.json` (server_url + accounts allow-list)
  plus `ZED_SIM_IMPERSONATE_TOKEN` / `ZED_SIM_SERVER_URL` env overrides. No
  secret is committed. Usernames are validated against the allow-list.
- The key MUST be preview-scoped. Do NOT use a production-capable key here.

### Known constraints (stock-binary limitations)

- **TTY requirement:** stock Zed only auto-triggers impersonation when stdout is
  a TTY (`authenticate()` in `crates/zed/src/main.rs`). The launched Zed
  inherits this tool's stdout, so the tool must be run from a terminal
  (`cargo run`). A future Phase 2 build flag could drop this requirement.
- **Token in terminal:** stock Zed `eprintln!`s the token during admin sign-in.
  It lands in the local terminal only; keep the preview token low-privilege.

## Open items (track, not blocking the MVP)

- [ ] Confirm a preview backend with usable test accounts exists.
- [ ] Confirm form factor preference (local web page assumed) holds once seen.
- [ ] Decide scratch-profile location and retention default (assumed: temp dir,
      wiped on demand).
