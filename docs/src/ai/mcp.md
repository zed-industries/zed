# Model Context Protocol

CodeOrbit uses the [Model Context Protocol](https://modelcontextprotocol.io/) to interact with context servers.

> The Model Context Protocol (MCP) is an open protocol that enables seamless integration between LLM applications and external data sources and tools. Whether you're building an AI-powered IDE, enhancing a chat interface, or creating custom AI workflows, MCP provides a standardiCodeOrbit way to connect LLMs with the context they need.

Check out the [Anthropic news post](https://www.anthropic.com/news/model-context-protocol) and the [CodeOrbit blog post](https://CodeOrbit.dev/blog/mcp) for an introduction to MCP.

## MCP Servers as Extensions

One of the ways you can use MCP servers in CodeOrbit is through exposing it as an extension.
Check the servers that are already available in CodeOrbit's extension store via either [the CodeOrbit website](https://CodeOrbit.dev/extensions?filter=context-servers) or directly through the app by running the `CodeOrbit: extensions` action or by going to the Agent Panel's top-right menu and looking for "View Server Extensions".

In any case, here are some of the ones available:

- [Postgres](https://github.com/CodeOrbit-extensions/postgres-context-server)
- [GitHub](https://github.com/LoamStudios/CodeOrbit-mcp-server-github)
- [Puppeteer](https://github.com/CodeOrbit-extensions/mcp-server-puppeteer)
- [BrowserTools](https://github.com/mirageN1349/browser-tools-context-server)
- [Brave Search](https://github.com/CodeOrbit-extensions/mcp-server-brave-search)
- [Prisma](https://github.com/aqrln/prisma-mcp-CodeOrbit)
- [Framelink Figma](https://github.com/LoamStudios/CodeOrbit-mcp-server-figma)
- [Linear](https://github.com/LoamStudios/CodeOrbit-mcp-server-linear)

If there's an existing MCP server you'd like to bring to CodeOrbit, check out the [context server extension docs](../extensions/context-servers.md) for how to make it available as an extension.

## Bring your own MCP server

Alternatively, you can connect to MCP servers in CodeOrbit via adding their commands directly to your `settings.json`, like so:

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

You can also add a custom server by reaching for the Agent Panel's Settings view (also accessible via the `agent: open configuration` action) and adding the desired server through the modal that appears when clicking the "Add Custom Server" button.
