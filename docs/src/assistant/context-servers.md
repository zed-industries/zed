# Context Servers

Context servers are a mechanism for pulling context into the Assistant from an external source. They are powered by the [Model Context Protocol](./model-context-protocol.md).

Currently Zed supports context servers providing [slash commands](./commands.md) for use in the Assistant.

## Installation

Context servers can be installed via [extensions](../extensions/context-servers.md).

If you don't already have a context server, check out one of these:

- [Postgres Context Server](https://github.com/zed-extensions/postgres-context-server)

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
