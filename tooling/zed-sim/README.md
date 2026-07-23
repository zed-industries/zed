# Zed Sim

An internal, staff-only launcher for experiencing Zed in controlled states —
starting with a **truly fresh, first-time setup**. It runs the real Zed binary
in a disposable profile, so nothing it does touches your primary Zed install.

See [`SPEC.md`](./SPEC.md) for the product overview and [`PLAN.md`](./PLAN.md)
for the build plan and roadmap.

## Run it

```sh
cargo run -p zed-sim
```

This starts a small local control panel and opens it in your browser. Pick a
state and click **Launch**.

### Finding your Zed binary

By default the tool looks for Zed in the standard macOS locations
(`/Applications/Zed.app`, `Zed Preview.app`, `Zed Nightly.app`). If yours is
elsewhere, point at it:

```sh
cargo run -p zed-sim -- --zed "/path/to/Zed.app/Contents/MacOS/zed"
# or
ZED_SIM_BINARY="/path/to/zed" cargo run -p zed-sim
```

### Other flags

- `--port <PORT>` — pin the control panel to a fixed port (default: ephemeral).
- `--no-open` — don't open a browser automatically; the URL is printed to the
  terminal.

## States (Phase 1)

- **Brand-new user** — signed out, pristine first-run onboarding.
- **Signed in** — a fresh profile that goes through the real sign-in flow.

The Pro / Trial / Business states are shown as "coming soon" and arrive in
Phase 2 (see `PLAN.md`).

## Impersonation (real accounts, optional)

You can also launch a session signed in as another GitHub account **without any
GitHub login** — you just supply a username. This uses Zed's internal
impersonation path: it resolves the username via GitHub's public API, finds or
creates that user on the backend, and signs in. No OAuth, no password, no
keychain.

This is powerful (an impersonation token can become *any* account on its
backend), so it must point at a **preview backend, never production**.

### Setup

1. Copy the example config and fill it in:
   ```sh
   cp tooling/zed-sim/zed-sim.config.example.json tooling/zed-sim/zed-sim.config.json
   ```
   The real `zed-sim.config.json` is gitignored.
2. Set `server_url` to your **preview** backend URL.
3. Add the GitHub usernames you want under `accounts`.
4. Supply the internal token. Prefer the env var so the secret never lands in a
   file:
   ```sh
   ZED_SIM_IMPERSONATE_TOKEN="<preview-token>" cargo run -p zed-sim
   ```
   (`ZED_SIM_SERVER_URL` likewise overrides `server_url` if you'd rather pass it
   via env.)

Once configured, the **Impersonate** section lists your accounts; click one to
launch a fresh Zed already signed in as that account.

### Important: run from a terminal

Stock Zed only honors impersonation when its stdout is a TTY. The launched Zed
inherits this tool's stdout, so **run the tool from a terminal** (as `cargo run`
does) for impersonation to take effect. Note that stock Zed prints the token to
that terminal during sign-in — keep the preview token low-privilege.

## How it stays disposable

Each launch creates a throwaway profile under your system temp directory
(`<temp>/zed-sim/<uuid>`) and starts Zed with `--user-data-dir` pointed at it,
so config, data, db, and logs are all isolated there. Each profile also gets a
unique `credentials_url`, which keeps its sign-in separate from your real Zed's
saved login in the OS keychain.

Click **Wipe scratch profiles** (or delete `<temp>/zed-sim`) to reset.

## Notes / limitations

- macOS-first. On other platforms, pass `--zed` explicitly.
- Launches are fire-and-forget; closing a simulated Zed window does not remove
  its profile — use **Wipe** for that.
