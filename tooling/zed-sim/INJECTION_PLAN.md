# Zed Sim — Phase 2 Injection Plan

How the launcher's greyed-out **Pro / Trial / Trial-expired / Business** states
become real, fully offline, with no backend and no token. For review by the
engineering owner before the sensitive wiring lands.

## Idea in one sentence

In a **staff-only build**, synthesize a fake "authenticated user + plan" at
startup and feed it through the exact same code path the real server response
uses — so the whole editor lights up as that state, with nothing faked beyond
the initial payload.

## Why this is clean (the two chokepoints)

Researching `crates/client` shows the entire signed-in experience funnels
through two points:

1. **`UserStore::update_authenticated_user(GetAuthenticatedUserResponse)`**
   (`crates/client/src/user.rs`) sets the current user *and* the plan info
   (plan, `trial_started_at`, `subscription_period`, `usage`,
   `is_account_too_young`, `has_overdue_invoices`) in one shot, then emits
   `PrivateUserInfoUpdated`. This is the same method the real network response
   and websocket updates call.
2. **`Client` status** (`crates/client/src/client.rs`): the UI treats any status
   other than `Status::SignedOut` as "signed in". `Status::Authenticated` is
   enough — it does **not** require a real peer/RPC connection, so this works
   completely offline.

So injection = build one `GetAuthenticatedUserResponse` + set status to
`Authenticated`. No scattering of `if simulated` checks across the UI.

## Compile-time gating (the safety story)

- A new cargo feature **`staff-sim`** on the `client` crate (default OFF), and a
  matching `staff-sim` feature on the `zed` binary crate that enables
  `client/staff-sim` and the startup hook.
- All injection code is behind `#[cfg(feature = "staff-sim")]`. **Release builds
  (without the feature) contain zero injection code** — it is not compiled in,
  not dormant. This is the property to confirm with security.
- CI adds one job that builds a "Zed Sim" artifact with `--features staff-sim`,
  alongside the existing nightly artifacts. The launcher auto-fetches it.

## Pieces

1. **`crates/client/src/sim_state.rs`** (feature-gated) — DONE in this commit.
   - `SimAuthState` catalog + `from_env()` (reads `ZED_SIM_STATE`).
   - `synthesize_response(state) -> GetAuthenticatedUserResponse` — pure data,
     reuses the shape of the existing test helper.
   - Personal-plan states first: free signed-in, Pro, Trial active, Trial
     expired (= `ZedFree` + a past `trial_started_at`, which triggers the real
     end-of-trial upsell per `agent_panel.rs::should_render_trial_end_upsell`).
2. **Apply entry** (NEXT increment, the sensitive part):
   - Feature-gated `pub fn` on `UserStore` to call the private
     `update_authenticated_user` with a synthesized response.
   - Feature-gated way to set `Client` status to `Authenticated`.
   - A single `apply_from_env(client, user_store, cx)` called once at startup.
3. **Startup hook** (NEXT) — in `crates/zed/src/main.rs` `authenticate()`,
   behind `#[cfg(feature = "staff-sim")]`: if `ZED_SIM_STATE` is set, apply it
   instead of the normal credential path.
4. **Launcher wiring** (NEXT) — enable the greyed-out buttons; each sets
   `ZED_SIM_STATE=<state>` on the launched Zed (only meaningful for the staff
   build). No config, no token, no backend.

## Business states (later)

`ZedBusiness` member/admin need `organizations`, `default_organization_id`,
`plans_by_organization`, and member roles populated in the response. Modelled
after `Organization` / `OrganizationConfiguration`. Deferred until the personal
states are proven end to end.

## Honest caveats

- **Cosmetic, by design.** This sets what the client *believes and displays*. It
  does not exercise real server enforcement (usage caps, billing). That matches
  the agreed MVP goal (reproduce what a user *sees and feels*).
- **Usage shaping** (e.g. "near the edit-prediction cap") is a follow-up — first
  cut uses `Unlimited`/comfortable usage.
- Anything that makes a live RPC/collab call will still no-op offline; that is
  acceptable for UX/flow review.
