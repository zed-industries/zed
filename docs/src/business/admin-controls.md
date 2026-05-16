---
title: Admin Controls - Zed Business
description: Configure AI, collaboration, and data sharing settings for your entire Zed Business organization.
---

# Admin Controls

Owners and admins can configure settings that apply to every member of the organization.

Most controls apply server-side to anything that routes through Zed's infrastructure. Some, like the Collaboration toggle, are enforced client-side and require members to be on a minimum Zed version. These controls don't cover [bring-your-own-key (BYOK) configurations](../ai/llm-providers.md), [external agents](../ai/external-agents.md), or [third-party extensions](../extensions.md), since those work independently of Zed's servers.

## Accessing admin controls

Admin controls are available to owners and admins in the organization dashboard at [dashboard.zed.dev](https://dashboard.zed.dev). Navigate to your organization, then select Data & Privacy from the sidebar to configure these settings.

---

## Collaboration

The **Collaboration** toggle controls whether members can use Zed's real-time collaboration features, including [Channels](../collaboration/channels.md), shared projects, and voice chat. Collaboration is off by default for Business organizations.

This control is configured from the Data & Privacy page in the organization dashboard. It is enforced client-side and requires members to be on Zed **0.233 or later**. Members on older versions will not have the setting enforced.

## Hosted AI models

The **Zed Model Provider** toggle controls whether members can use Zed's [hosted AI models](../ai/models.md):

- **On:** Members can use Zed's hosted models for AI features.
- **Off:** Members must bring their own API keys via [Providers](../ai/llm-providers.md) or use [external agents](../ai/external-agents.md) for AI features.

## Edit Predictions

The **Edit Prediction** toggle controls whether members can use Zed's hosted [Edit Predictions](../ai/edit-prediction.md) via the Zeta model family. Members using third-party providers or local models for edit predictions are not affected.

**Edit Prediction Feedback** controls whether members can submit feedback on edit predictions. This setting is only configurable when Edit Prediction is enabled.

## Agent Thread Feedback

The **Agent Thread Feedback** toggle controls whether members can submit feedback on agent thread responses. When disabled, members cannot rate or provide feedback on AI agent conversations.

## Data sharing

On Free and Pro, [data sharing with Zed for AI improvement](../ai/ai-improvement.md) is opt-in per member. On Business, it's off by default and controlled by the Agent Thread Feedback and Edit Prediction Feedback toggles above.
