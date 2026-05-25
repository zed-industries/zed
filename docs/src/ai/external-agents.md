---
title: External Agents - Zed
description: Install and use ACP-integrated external agents such as Claude, Codex, OpenCode, Copilot, Cursor, and Pi Coding Agent in Zed.
---

# External Agents

External agents are agents that integrate with Zed through the [Agent Client Protocol (ACP)](https://agentclientprotocol.com). Zed hosts the thread in the [Agent Panel](./agent-panel.md) and [Threads Sidebar](./parallel-agents.md#threads-sidebar), while the external agent usually owns its own runtime, auth, model selection, tools, and native configuration.

Use [Terminal Threads](./terminal-threads.md) instead when you want to run a CLI or TUI directly in a terminal-backed thread.

## Install from the ACP Registry {#registry}

The ACP Registry is the primary way to install common external agents in Zed.

Open the registry with {#action zed::AcpRegistry}, or open [Agent Settings](./agent-settings.md) with {#action agent::OpenSettings}, click `Add Agent`, and choose `Install from Registry`.

After installation, the agent appears in the new-thread menu in the Agent Panel and Threads Sidebar.

## Common Agents {#common-agents}

Common external agents include:

- Claude
- Codex
- OpenCode
- Copilot
- Cursor
- Pi Coding Agent

This list is curated, not exhaustive. Open the ACP Registry in Zed for the current list of available agents.

## Start an External Agent Thread {#start-thread}

Open the [Agent Panel](./agent-panel.md), then use the agent selector or the new-thread menu to start a thread with an installed external agent.

You can also create keybindings for specific agents with {#action agent::NewExternalAgentThread}. Use the agent ID from the installed registry entry.

```json [keymap]
[
  {
    "bindings": {
      "cmd-alt-c": [
        "agent::NewExternalAgentThread",
        { "agent": { "custom": { "name": "codex-acp" } } }
      ]
    }
  }
]
```

## Configuration Boundaries {#configuration-boundaries}

External agents run as separate processes that communicate with Zed over ACP. This creates a boundary between Zed configuration and agent-native configuration.

| Capability                       | Behavior in external agent threads                                                         |
| -------------------------------- | ------------------------------------------------------------------------------------------ |
| Model/provider config            | Usually owned by the external agent                                                        |
| Auth/API keys/subscriptions      | Usually owned by the external agent                                                        |
| Zed Agent profiles               | Do not apply unless the integration says otherwise                                         |
| Zed Skills                       | Do not apply as Zed Skills                                                                 |
| Native agent skills/instructions | Depends on the agent                                                                       |
| Zed MCP servers                  | May be forwarded over ACP                                                                  |
| Native MCP config                | May also be read by the agent                                                              |
| Tool permissions                 | Zed ACP/tool forwarding permissions may apply; native tool permissions depend on the agent |

For Zed's native agent configuration, see [Zed Agent](./zed-agent.md).

## Agent-Specific Auth and Config {#agent-auth-config}

External agents may have their own sign-in flow, API key setup, subscription behavior, environment variables, and config files.

Examples:

- Claude Agent may use Claude Code auth and Claude-native config.
- Codex may use ChatGPT login, Codex API keys, OpenAI API keys, or Codex-native config.
- Cursor subscriptions do not configure Zed's LLM provider settings; use Cursor's agent or CLI setup where available.
- Pi Coding Agent is an agent harness. Configure provider auth in Pi.

If an external agent supports subscription-backed behavior, configure that in the agent unless the agent's Zed integration says otherwise.

## Remote Projects {#remote-projects}

External agents may read credentials locally, remotely, or through their own sign-in flow. Check the specific agent's setup path when using SSH, dev containers, or other remote projects.

Zed LLM provider API keys saved in the local keychain are not automatically the same as an external agent's credentials.

## Custom Agents {#custom-agents}

Use custom agents when you are developing an ACP-compatible agent or need to run an agent that is not in the registry.

Open [Agent Settings](./agent-settings.md), click `Add Agent`, and choose `Add Custom Agent`. Zed opens your settings file with an `agent_servers` entry.

```json [settings]
{
  "agent_servers": {
    "my-agent": {
      "type": "custom",
      "command": "node",
      "args": ["~/projects/agent/index.js", "--acp"],
      "env": {}
    }
  }
}
```

Registry-installed agents can also have per-agent settings under `agent_servers.<agent-id>`.

## Extension-Provided Agents {#extension-agents}

Some extensions can provide agents. Registry installation is the primary path for common agents, but extension-provided agents still exist.

For extension authoring, see [Agent Server Extensions](../extensions/agent-servers.md).

## MCP {#mcp}

Zed-configured [MCP servers](./mcp.md) may be forwarded to external agents over ACP. External agents may also read their own native MCP configuration.

If an MCP tool does not appear in an external agent, check both Zed's MCP server configuration and the agent's native MCP configuration.

## Debugging {#debugging}

Use {#action dev::OpenAcpLogs} from the Command Palette to inspect messages between Zed and an external agent.

Include ACP logs when reporting issues with external agents.
