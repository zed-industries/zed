# Model Context Protocol

Zed uses the [Model Context Protocol](https://modelcontextprotocol.io/) to interact with context servers.

> The Model Context Protocol (MCP) is an open protocol that enables seamless integration between LLM applications and external data sources and tools. Whether you're building an AI-powered IDE, enhancing a chat interface, or creating custom AI workflows, MCP provides a standardized way to connect LLMs with the context they need.

Check out the [Anthropic news post](https://www.anthropic.com/news/model-context-protocol) and the [Zed blog post](https://zed.dev/blog/mcp) for an introduction to MCP.

## MCP Servers as Extensions

One of the ways you can use MCP servers in Zed is through exposing them as an extension.
To learn how to do that, check out the [MCP Server Extensions](../extensions/mcp-extensions.md) page for more details.

### Available extensions

Many MCP servers have been exposed as extensions already, thanks to Zed's awesome community.
Check the ones are already available in Zed's extension store via any of these routes:

1. [the Zed website](https://zed.dev/extensions?filter=context-servers)
2. in the app, run the `zed: extensions` action
3. in the app, go to the Agent Panel's top-right menu and look for the "View Server Extensions" menu item

In any case, here are some of the ones available:

- [Postgres](https://github.com/zed-extensions/postgres-context-server)
- [GitHub](https://github.com/LoamStudios/zed-mcp-server-github)
- [Puppeteer](https://github.com/zed-extensions/mcp-server-puppeteer)
- [BrowserTools](https://github.com/mirageN1349/browser-tools-context-server)
- [Brave Search](https://github.com/zed-extensions/mcp-server-brave-search)
- [Prisma](https://github.com/aqrln/prisma-mcp-zed)
- [Framelink Figma](https://github.com/LoamStudios/zed-mcp-server-figma)
- [Linear](https://github.com/LoamStudios/zed-mcp-server-linear)
- [Resend](https://github.com/danilo-leal/zed-mcp-server-resend)

## Add your own MCP server

Creating an extension is not the only way to use MCP servers in Zed.
You can connect them by just adding their commands directly to your `settings.json`, like so:

```json
{
  "context_servers": {
    "some-context-server": {
      "command": {
        "path": "some-command",
        "args": ["arg-1", "arg-2"],
        "env": {}
      },
      "settings": {}
    }
  }
}
```

Alternatively, you can also add a custom server by reaching for the Agent Panel's Settings view (also accessible via the `agent: open configuration` action) and adding it through the modal that appears when clicking the "Add Custom Server" button.
