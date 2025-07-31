# MCP Server Extensions

[Model Context Protocol servers](../ai/mcp.md) can be exposed as extensions for use in the Agent Panel.

## Defining MCP Extensions

A given extension may provide one or more MCP servers.
Each MCP server must be registered in the `extension.toml`:

```toml
[context_servers.my-context-server]
```

Then, in the Rust code for your extension, implement the `context_server_command` method on your extension:

```rust
impl zed::Extension for MyExtension {
    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &zed::Project,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: get_path_to_context_server_executable()?,
            args: get_args_for_context_server()?,
            env: get_env_for_context_server()?,
        })
    }
}
```

This method should return the command to start up an MCP server, along with any arguments or environment variables necessary for it to function.

If you need to download the MCP server from an external source—like GitHub Releases or npm—you can also do that in this function.

## Available Extensions

Check out all the MCP servers that have already been exposed as extensions [on Zed's site](https://zed.dev/extensions?filter=context-servers).

We recommend taking a look at their repositories as a way to understand how they are generally created and structured.

## Testing

To test your new MCP server extension, you can [install it as a dev extension](./developing-extensions.md#developing-an-extension-locally).
