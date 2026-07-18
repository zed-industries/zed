---
title: AI in Zed
description: Understand Zed's AI features, agent paths, model providers, and setup routes.
---

# AI

Zed's AI docs are organized around three areas:

| Area         | Use it to choose                    | Examples                                                             |
| ------------ | ----------------------------------- | -------------------------------------------------------------------- |
| Agents       | How agentic work runs in Zed        | Zed Agent, External Agents, Terminal Threads                         |
| Model access | How Zed connects to language models | Zed-hosted models, API access, subscriptions, gateways, local models |
| Features     | Which AI workflow you want to use   | Agentic editing, inline edits, edit prediction, Git assistance       |

Start with [AI Quick Start](./quick-start.md) if you know what you want to do. Use [AI by Company](./by-company.md) if you know the company, subscription, model provider, agent, or CLI you want to use.

## Agent Paths {#agent-paths}

Agent paths decide how agentic work runs in Zed.

- [Zed Agent](./zed-agent.md): Zed's native agent. It can use models configured through [LLM Providers](./llm-providers.md), including Zed-hosted models, provider API keys, supported subscriptions, gateways, and local models. It also uses built-in tools, profiles, skills, instructions, and MCP servers.
- [External Agents](./external-agents.md): ACP-integrated agents that run through their own process and configuration.
- [Terminal Threads](./terminal-threads.md): terminal-backed threads for running an agent CLI or TUI directly in Zed.

The [Threads Sidebar](./parallel-agents.md#threads-sidebar) is where you organize agent work. You can run multiple agent threads and Terminal Threads at once, each using a different agent and working against different projects.

See [Agents](./agents.md) for a comparison.

## Model Access {#model-access}

Model access controls which models power the Zed Agent and other model-backed Zed AI features. Zed can use hosted models, provider API access, subscription sign-in, gateways, and local models.

See [LLM Providers](./llm-providers.md) to choose a model access path.

## AI Features {#ai-features}

Zed has several AI-powered workflows:

- [Agent Panel](./agent-panel.md): prompt agents, add context, review changes, and manage active threads.
- [Parallel Agents](./parallel-agents.md): run multiple threads across projects and worktrees.
- [Inline Assistant](./inline-assistant.md): transform a selection in place.
- [Edit Prediction](./edit-prediction.md): accept AI completions while you type.
- [Git commit generation](../git.md#ai-support-in-git): generate commit messages from the Git panel.

## Configure AI {#configure-ai}

Use [AI Quick Start](./quick-start.md) to choose the right setup path, configure model access, add agents or MCP servers, control tools, and turn AI off.

For privacy, provider data boundaries, and opt-in data sharing, see [AI Privacy](./privacy-and-security.md) and [Feedback and Training Data](./ai-improvement.md).
