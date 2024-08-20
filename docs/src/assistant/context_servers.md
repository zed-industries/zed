# Context Servers

A Context Server is an experimental interface for defining simple, language-agnostic slash commands in Zed's [Assistant](./assistant.md). Context Servers allow you to extend Zed's Assistant to interface with external capabilities and systems in a language-agnostic way.

If slash commands allow you to extend the Assistant with new capabilities, Context Servers follow a simple protocol for registering and making use of those capabilities.

## Using a Context Server

To configure Zed to use a Context Server, add the command required to start the server to your [settings](./configuring-zed.md):

```json
{
  "experimental": {
    "context_servers": [
      {
        "id": "python_context_server",
        "executable": "python",
        "args": ["-m", "my_context_server"]
      }
    ]
  }
}
```

## Developing a Context Server

A Context Server is a server listening for JSON-RPC requests over stdin/stdout. The server must follow the Model Context Protocol (defined below) in order to declare its capabilities such that Zed can make use of them.

### Should you write a Context Server?

[Extensions](./extensions.md) are also capable of adding slash commands to the Assistant.

If your slash commands are already implemented in a language other than Rust, wrapping them in a Context Server implementation will likely be the fastest way to plug them into Zed.

An Extension should be preferred when:

- Your slash commands are implemented in WebAssembly-compatible Rust
- You want Zed to manage distribution of your slash commands
- You want to publish your slash commands

### Implementing a Context Server

Context Servers must comply with the [Model Context Protocol (MCP)](./model_context_protocol). See [python-context-server](https://github.com/zed-industries/python-context-server) for a minimal working example.

Currently, Zed's client only implements the subset of the protocol required to support custom prompt insertions and manipulations, although this is likely to be extended in the future.
