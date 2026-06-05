---
title: Privacy for Business - Zed Business
description: How Zed Business handles data privacy across your organization, including enforced protections for prompts and training data.
---

# Privacy for Business

Zed Business removes the per-member data-sharing options that Free and Pro
expose. These protections are on by default for every Business organization.
Administrators can adjust them from
[Admin Controls](./admin-controls.md); individual members can't opt in or out.

## What's enforced by default

For all members of a Zed Business organization:

- **No prompt sharing:** Conversations and prompts are never shared with Zed.
  Members can't opt into
  [AI feedback via ratings](../ai/ai-improvement.md#ai-feedback-with-ratings).
  Administrators can enable Agent Thread Feedback to allow this.
- **No training data sharing:** Code context is never shared with Zed for
  [Edit Prediction model training](../ai/ai-improvement.md#edit-predictions).
  Members can't opt in individually. Administrators can enable Edit Prediction
  Feedback to allow this.

These protections are enforced server-side and apply to all org members.

## How individual plans differ

On Free and Pro, data sharing is opt-in:

- Members can rate AI responses, which shares that conversation with Zed.
- Members can opt into Edit Prediction training data collection for open source projects.

Neither option is available to Zed Business members.

## What data still leaves the organization

These controls cover what Zed stores and trains on. They don't change how AI inference works: when members use Zed's hosted models, prompts and code context are still sent to the relevant provider (Anthropic, OpenAI, Google, etc.) to generate responses. Zed maintains zero-data retention agreements with these providers. See [AI Improvement](../ai/ai-improvement.md#data-retention-and-training) for details.

[Bring-your-own-key](../ai/llm-providers.md) and [external agents](../ai/external-agents.md) are subject to each provider's own terms; Zed has no visibility into how they handle data.

## Additional admin controls

Administrators have additional options in [Admin Controls](./admin-controls.md):

- Disable Zed-hosted models entirely via the Zed Model Provider toggle, so no
  prompts reach Zed's infrastructure
- Disable Edit Predictions org-wide
- Disable Edit Prediction Feedback
- Disable Agent Thread Feedback
- Disable real-time collaboration

See [Admin Controls](./admin-controls.md) for the full list.
