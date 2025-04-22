# Context Servers

Context servers are a mechanism for pulling context into the Assistant from an external source.
They are powered by the [Model Context Protocol](./model-context-protocol.md).

Currently Zed supports context servers providing [slash commands](./commands.md) for use in the Assistant.

## Installation

Context servers can be installed via [extensions](../extensions/context-servers.md).

If you don't already have a context server, check out one of these:

- [Postgres](https://github.com/zed-extensions/postgres-context-server)
- [GitHub](https://github.com/LoamStudios/zed-mcp-server-github)
- [Puppeteer](https://github.com/zed-extensions/mcp-server-puppeteer)
- [BrowserTools](https://github.com/mirageN1349/browser-tools-context-server)
- [Brave Search](https://github.com/zed-extensions/mcp-server-brave-search)
- [Prisma](https://github.com/aqrln/prisma-mcp-zed)
- [Framelink Figma](https://github.com/LoamStudios/zed-mcp-server-figma)
- [Linear](https://github.com/LoamStudios/zed-mcp-server-linear)

Browse all available MCP extensions either on [Zed's website](https://zed.dev/extensions?filter=context-servers) or directly in Zed via the `zed: extensions` action in the Command Palette.

## Configuration

Context servers may require some configuration in order to run or to change their behavior.

You can configure each context server using the `context_servers` setting in your `settings.json`:

```json
{
  "context_servers": {
    "postgres-context-server": {
      "settings": {
        "database_url": "postgresql://postgres@localhost/my_database"
      }
    }
  }
}
```

If desired, you may also provide a custom command to execute a context server:

```json
{
  "context_servers": {
    "my-context-server": {
      "command": {
        "path": "/path/to/my-context-server",
        "args": ["run"],
        "env": {}
      },
      "settings": {
        "enable_something": true
      }
    }
  }
}
```
