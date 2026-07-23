# Zed Sim — Spec

**Audience:** Product Managers and other non-engineering Zed staff.
**Status:** Proposed (MVP not yet built).

## The problem

We can't easily *experience* Zed the way our users do.

- It's nearly impossible to see a **truly fresh, first-time setup**. Even after deleting the app, your machine still holds onto config, preferences, and saved login — so "reinstalling" never feels new.
- It's hard to step into other account states — **Pro**, **Business**, **on trial**, or **trial expired** — without owning an account that's actually in that state.
- When we spot odd experiences in the wild (a screen recording, a support video), we have no way to **reproduce that state** and feel it for ourselves.

This makes it hard to evaluate onboarding, upsells, gating, and the overall feel of each plan — the exact things Growth needs to reason about.

## What Zed Sim is

A small, internal **launcher**. You pick a state from a simple list, click **Launch**, and a real, disposable copy of Zed boots up in that state — without touching your actual Zed install.

- It runs **the real editor**, not a mockup or a component preview. Integrations and UX are the genuine article; only the *account/plan state* is set for you.
- Every session is **throwaway**. It lives in its own scratch space and can be wiped instantly, so you can experience a pristine first-run as many times as you want.
- It is **staff-only and internal**. It is never shipped to customers.

## What it is *not*

- Not a way to create real accounts, and not a GitHub-account generator.
- Not a billing or production tool — it doesn't change anything about real customers or real subscriptions.
- Not a redesign of Zed — it's a wrapper around the Zed you already know.

## States (target catalog)

| State | What you experience |
|---|---|
| **Brand-new user** | A genuine first-run: onboarding, signed out, nothing configured. |
| **Signed in** | The real sign-in flow, then the editor as a signed-in account. |
| **Pro** | The Pro experience. |
| **Pro Trial — active** | Mid-trial UX. |
| **Pro Trial — expired** | The end-of-trial upsell flow. |
| **Business — member** | Business as a regular team member. |
| **Business — admin** | Business with admin controls. |

Later additions (e.g. "near the edit-prediction cap," "account with overdue invoice") are part of this same tool — not a separate one.

## Rollout

- **Phase 1 (the MVP):** **Brand-new user** and **Signed in**. These are the highest-value states and need no changes to Zed itself — just the launcher. This is what unlocks "experience a truly fresh setup," our top priority.
- **Phase 2:** the fabricated plan states (Pro, Trial, Trial-expired, Business). These are injected locally, with no production access and no security-sensitive keys.
- **Optional:** genuinely-real Pro/Trial/Business sessions by signing into pre-made accounts on a non-production (preview) backend.

## Why you can trust what you see

- Phase 1 states are *real* Zed with a *real* sign-in — nothing is faked.
- Phase 2 states set what the editor *displays* (your plan, trial status). The MVP goal is faithful **UX and flows** — what a user in that state sees and feels — not server-enforced billing. Hard limits (e.g. actually hitting a usage cap) come later as an explicit control inside the tool.
