---
title: AI Privacy - Zed
description: Understand how Zed handles AI prompts, code context, hosted model requests, provider data boundaries, feedback, training data, and privacy controls.
---

# AI Privacy

This page explains the privacy and trust boundaries for AI features in Zed,
including [Zed Agent](./zed-agent.md), [Edit Prediction](./edit-prediction.md),
[Inline Assistant](./inline-assistant.md), and
[Git commit generation](../git.md#ai-support-in-git).

Zed does not retain your prompts or code context by default. For
[Zed-hosted models](../account/zed-hosted-models.md), Zed has no-training
commitments from model providers, and provider agreements require zero data
retention for inference requests except for
[provider-designated models with safety retention](#provider-safety-retention),
such as Anthropic's Mythos-class models.
Zed only retains AI data when you explicitly share feedback or opt in to
training data collection.

## AI Request Paths {#ai-request-paths}

| Path                                                         | Who handles model requests                        | What to know                                                                                                                                                                                                                        | Details                                                                                           |
| ------------------------------------------------------------ | ------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| [Zed-hosted models](../account/zed-hosted-models.md)         | Zed routes requests to hosted model providers     | Provider agreements prohibit training on your prompts or code context and require zero data retention for inference requests, except for provider-designated models with safety retention, such as Anthropic's Mythos-class models. | [Zed-hosted model commitments](#data-retention-and-training)                                      |
| [Provider API keys](./use-api-access.md)                     | The configured provider                           | The provider handles requests under its own terms. Provider keys saved through Zed are stored in the system keychain, not in `settings.json`.                                                                                       | [Use API Access](./use-api-access.md)                                                             |
| [Existing subscriptions](./use-an-existing-subscription.md)  | The subscription provider                         | The provider handles requests under the subscription terms.                                                                                                                                                                         | [Use an Existing Subscription](./use-an-existing-subscription.md)                                 |
| [Gateways](./use-a-gateway.md)                               | The configured gateway and upstream providers     | The gateway and upstream providers handle requests under their own terms.                                                                                                                                                           | [Use a Gateway](./use-a-gateway.md)                                                               |
| [Local models](./use-a-local-model.md)                       | The local server or self-hosted endpoint          | The local server handles requests according to how you configured that server.                                                                                                                                                      | [Use a Local Model](./use-a-local-model.md)                                                       |
| [External Agents](./external-agents.md)                      | The External Agent and its configured providers   | The External Agent handles model requests under its own terms. Tool and MCP behavior depends on agent and ACP configuration.                                                                                                        | [External Agents](./external-agents.md)                                                           |
| [Terminal Threads](./terminal-threads.md)                    | The CLI or TUI running in the terminal            | The CLI or TUI owns its auth, model routing, tools, instructions, MCP configuration, and data handling.                                                                                                                             | [Terminal Threads](./terminal-threads.md)                                                         |
| [Edit Prediction](./edit-prediction.md)                      | The selected edit prediction provider             | Each keystroke can send local editing context to the selected provider. Zeta requests are processed transiently unless training data collection is enabled; third-party providers follow their own terms.                           | [Edit Prediction](./edit-prediction.md), [Feedback and Training Data](./ai-improvement.md)        |
| [Agent tools](./tools.md), [MCP](./mcp.md), and integrations | Zed, configured MCP servers, and external systems | Tools can read, edit, search, run commands, fetch URLs, or call external systems depending on profile, MCP server, and tool permission settings.                                                                                    | [Agent Profiles](./agent-profiles.md), [Tool Permissions](./tool-permissions.md), [MCP](./mcp.md) |
| Project trust and instructions                               | Zed and the trusted worktree                      | Project-local instructions and skills are loaded from trusted worktrees. External Agents and Terminal Threads may read their own instruction files.                                                                                 | [Worktree Trust](../worktree-trust.md), [Skills](./skills.md), [Instructions](./instructions.md)  |

## Zed-Hosted Model Commitments {#data-retention-and-training}

For Zed-hosted models, Zed has commitments from model providers that prohibit
training on your prompts or code context and require zero data retention for
inference requests, except for
[provider-designated models with safety retention](#provider-safety-retention),
such as Anthropic's Mythos-class models. The public provider documents linked below describe provider programs or default
API terms; Zed-hosted model requests are governed by Zed's provider agreements.

| Provider  | No training reference                                   | Zero-data-retention reference                                                                                                                                                                      |
| --------- | ------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Anthropic | [Yes](https://www.anthropic.com/legal/commercial-terms) | [Yes](https://privacy.anthropic.com/en/articles/8956058-i-have-a-zero-data-retention-agreement-with-anthropic-what-products-does-it-apply-to), except [covered models](#provider-safety-retention) |
| Google    | [Yes](https://cloud.google.com/terms/service-terms)     | [Yes](https://cloud.google.com/terms/service-terms), see Service Terms sections 18 and 20(h)                                                                                                       |
| OpenAI    | [Yes](https://openai.com/enterprise-privacy/)           | [Yes](https://platform.openai.com/docs/guides/your-data)                                                                                                                                           |

### Provider Safety Retention for Designated Models {#provider-safety-retention}

Some providers require limited data retention for specific models as a condition
of offering them, on every platform where those models are available. Anthropic
retains prompts and outputs for models it designates as covered models (its
Mythos-class models, such as Claude Fable 5) for 30 days for trust and safety
purposes. Zed cannot opt out of this retention; it applies wherever these models
are served. See
[Anthropic's data retention practices for Mythos-class models](https://support.claude.com/en/articles/15425996-data-retention-practices-for-mythos-class-models).

For these models:

- The no-training commitment still applies. Retained data is used for safety
  review, not model training.
- Zed does not retain your prompts or outputs. Retention happens at the
  provider, under the provider's documented access controls and deletion
  timelines.
- All other Zed-hosted models keep zero-data-retention handling.

If you don't want provider-side retention, use a model that the provider has
not designated for safety retention. Switching to
[your own API key](./use-api-access.md) or
[subscription](./use-an-existing-subscription.md) does not avoid this retention
for covered models, because providers apply it on every platform where those
models are offered.

## AI Data Retained by Zed {#ai-data-retained-by-zed}

Zed may retain AI data only when you explicitly share it or opt in:

- [Response ratings and feedback](./ai-improvement.md#ai-feedback-with-ratings)
  can send a conversation thread to Zed for review and improvement.
- [Edit Prediction training data](./ai-improvement.md#edit-predictions) is
  collected only when you opt in, the project is open source, and the file is not
  excluded.

See [Feedback and Training Data](./ai-improvement.md) for the full list of what
can be stored in each opt-in case.

## Controls and Related Privacy Docs {#controls-and-related-privacy-docs}

- [Telemetry](../telemetry.md): What telemetry Zed collects and how to control
  it.
- [Privacy for Business](../business/privacy.md): How Zed Business enforces
  privacy settings across an organization.
- [Admin Controls](../business/admin-controls.md): How owners and admins control
  Zed-hosted models, Edit Prediction, and feedback sharing.
- [AI Quick Start](./quick-start.md#turn-ai-off): How to turn AI off.
- [Privacy Policy](https://zed.dev/privacy-policy): Zed's privacy policy.
- [Subprocessors](https://zed.dev/subprocessors): Zed's subprocessors.
- [Terms of Service](https://zed.dev/terms): Zed's terms.
