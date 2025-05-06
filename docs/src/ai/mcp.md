# Model Context Protocol

<!-- todo! context servers link is broken -->
Zed uses the [Model Context Protocol](https://modelcontextprotocol.io/) to interact with [context servers](./context-servers.md):

> The Model Context Protocol (MCP) is an open protocol that enables seamless integration between LLM applications and external data sources and tools. Whether you're building an AI-powered IDE, enhancing a chat interface, or creating custom AI workflows, MCP provides a standardized way to connect LLMs with the context they need.

Check out the [Anthropic news post](https://www.anthropic.com/news/model-context-protocol) and the [Zed blog post](https://zed.dev/blog/mcp) for an introduction to MCP.

## Try it out

Want to try it for yourself? Here are some MCP servers available as Zed extensions:

- [Postgres](https://github.com/zed-extensions/postgres-context-server)
- [GitHub](https://github.com/LoamStudios/zed-mcp-server-github)
- [Puppeteer](https://github.com/zed-extensions/mcp-server-puppeteer)
- [BrowserTools](https://github.com/mirageN1349/browser-tools-context-server)
- [Brave Search](https://github.com/zed-extensions/mcp-server-brave-search)
- [Prisma](https://github.com/aqrln/prisma-mcp-zed)
- [Framelink Figma](https://github.com/LoamStudios/zed-mcp-server-figma)
- [Linear](https://github.com/LoamStudios/zed-mcp-server-linear)

Browse all available MCP extensions either on [Zed's website](https://zed.dev/extensions?filter=context-servers) or directly in Zed via the `zed: extensions` action in the Command Palette.

## Bring your own context server

If there's an existing MCP server you'd like to bring to Zed, check out the [context server extension docs](../extensions/context-servers.md) for how to make it available as an extension.

If you are interested in building your own MCP server, check out the [Model Context Protocol docs](https://modelcontextprotocol.io/introduction#get-started-with-mcp) to get started.
