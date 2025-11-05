# Model Context Protocol

Zed uses the [Model Context Protocol](https://modelcontextprotocol.io/) to interact with context servers.

> The Model Context Protocol (MCP) is an open protocol that enables seamless integration between LLM applications and external data sources and tools. Whether you're building an AI-powered IDE, enhancing a chat interface, or creating custom AI workflows, MCP provides a standardized way to connect LLMs with the context they need.

Check out the [Anthropic news post](https://www.anthropic.com/news/model-context-protocol) and the [Zed blog post](https://zed.dev/blog/mcp) for a general intro to MCP.

## Installing MCP Servers

### As Extensions

One of the ways you can use MCP servers in Zed is by exposing them as an extension.
Check out the [MCP Server Extensions](../extensions/mcp-extensions.md) page to learn how to create your own.

Thanks to our awesome community, many MCP servers have already been added as extensions.
You can check which ones are available via any of these routes:

1. [the Zed website](https://zed.dev/extensions?filter=context-servers)
2. in the app, open the Command Palette and run the `zed: extensions` action
3. in the app, go to the Agent Panel's top-right menu and look for the "View Server Extensions" menu item

In any case, here are some popular available servers:

- [Context7](https://zed.dev/extensions/context7-mcp-server)
- [GitHub](https://zed.dev/extensions/github-mcp-server)
- [Puppeteer](https://zed.dev/extensions/puppeteer-mcp-server)
- [Gem](https://zed.dev/extensions/gem)
- [Brave Search](https://zed.dev/extensions/brave-search-mcp-server)
- [Prisma](https://github.com/aqrln/prisma-mcp-zed)
- [Framelink Figma](https://zed.dev/extensions/framelink-figma-mcp-server)
- [Linear](https://zed.dev/extensions/linear-mcp-server)
- [Resend](https://zed.dev/extensions/resend-mcp-server)

### As Custom Servers

Creating an extension is not the only way to use MCP servers in Zed.
You can connect them by adding their commands directly to your `settings.json`, like so:

```json [settings]
{
  "context_servers": {
    "your-mcp-server": {
      "source": "custom",
      "command": "some-command",
      "args": ["arg-1", "arg-2"],
      "env": {}
    }
  }
}
```

Alternatively, you can also add a custom server by accessing the Agent Panel's Settings view (also accessible via the `agent: open settings` action).
From there, you can add it through the modal that appears when you click the "Add Custom Server" button.

## Using MCP Servers

### Configuration Check

Regardless of how you've installed MCP servers, whether as an extension or adding them directly, most servers out there still require some sort of configuration as part of the setup process.

In the case of extensions, after installing it, Zed will pop up a modal displaying what is required for you to properly set it up.
For example, the GitHub MCP extension requires you to add a [Personal Access Token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens).

In the case of custom servers, make sure you check the provider documentation to determine what type of command, arguments, and environment variables need to be added to the JSON.

To check if your MCP server is properly configured, go to the Agent Panel's settings view and watch the indicator dot next to its name.
If they're running correctly, the indicator will be green and its tooltip will say "Server is active".
If not, other colors and tooltip messages will indicate what is happening.

### Agent Panel Usage

Once installation is complete, you can return to the Agent Panel and start prompting.

Some models are better than others when it comes to picking up tools from MCP servers.
Mentioning your server by name always helps the model to pick it up.

However, if you want to _ensure_ a given MCP server will be used, you can create [a custom profile](./agent-panel.md#custom-profiles) where all built-in tools (or the ones that could cause conflicts with the server's tools) are turned off and only the tools coming from the MCP server are turned on.

As an example, [the Dagger team suggests](https://container-use.com/agent-integrations#zed) doing that with their [Container Use MCP server](https://zed.dev/extensions/mcp-server-container-use):

```json [settings]
"agent": {
  "profiles": {
    "container-use": {
      "name": "Container Use",
      "tools": {
        "fetch": true,
        "thinking": true,
        "copy_path": false,
        "find_path": false,
        "delete_path": false,
        "create_directory": false,
        "list_directory": false,
        "diagnostics": false,
        "read_file": false,
        "open": false,
        "move_path": false,
        "grep": false,
        "edit_file": false,
        "terminal": false
      },
      "enable_all_context_servers": false,
      "context_servers": {
        "container-use": {
          "tools": {
            "environment_create": true,
            "environment_add_service": true,
            "environment_update": true,
            "environment_run_cmd": true,
            "environment_open": true,
            "environment_file_write": true,
            "environment_file_read": true,
            "environment_file_list": true,
            "environment_file_delete": true,
            "environment_checkpoint": true
          }
        }
      }
    }
  }
}
```

### Tool Approval

Zed's Agent Panel includes the `agent.always_allow_tool_actions` setting that, if set to `false`, will require you to give permission for any editing attempt as well as tool calls coming from MCP servers.

You can change this by setting this key to `true` in either your `settings.json` or through the Agent Panel's settings view.

### External Agents

Note that for [external agents](./external-agents.md) connected through the [Agent Client Protocol](https://agentclientprotocol.com/), access to MCP servers installed from Zed may vary depending on the ACP agent implementation.

Regarding the built-in ones, Claude Code and Codex both support it, and Gemini CLI does not yet.
In the meantime, learn how to add MCP server support to Gemini CLI through [their documentation](https://github.com/google-gemini/gemini-cli?tab=readme-ov-file#using-mcp-servers).
