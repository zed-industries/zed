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
