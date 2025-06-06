# Context Servers

Extensions may provide [context servers](../ai/mcp.md) for use in the Assistant.

## Example extension

To see a working example of an extension that provides context servers, check out the [`postgres-context-server` extension](https://github.com/zed-extensions/postgres-context-server).

This extension can be [installed as a dev extension](./developing-extensions.md#developing-an-extension-locally) if you want to try it out for yourself.

## Defining context servers

A given extension may provide one or more context servers. Each context server must be registered in the `extension.toml`:

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

This method should return the command to start up a context server, along with any arguments or environment variables necessary for it to function.

If you need to download the context server from an external source—like GitHub Releases or npm—you can also do this here.
