# Debugger Extensions

[Debug Adapter Protocol](https://microsoft.github.io/debug-adapter-protocol) Servers can be exposed as extensions for use in the [debugger](../debugger.md).

## Defining Debugger Extensions

A given extension may provide one or more DAP servers.
Each DAP server must be registered in the `extension.toml`:

```toml
[debug_adapters.my-debug-adapter]
# Optional relative path to the JSON schema for the debug adapter configuration schema. Defaults to `debug_adapter_schemas/$DEBUG_ADAPTER_NAME_ID.json`.
# Note that while this field is optional, a schema is mandatory.
schema_path = "relative/path/to/schema.json"
```

Then, in the Rust code for your extension, implement the `get_dap_binary` method on your extension:

```rust
impl zed::Extension for MyExtension {
    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String>;
}
```

This method should return the command to start up a debug adapter protocol server, along with any arguments or environment variables necessary for it to function.

If you need to download the DAP server from an external source—like GitHub Releases or npm—you can also do that in this function. Make sure to check for updates only periodically, as this function is called whenever a user spawns a new debug session with your debug adapter.

You must also implement `dap_request_kind`. This function is used to determine whether a given debug scenario will _launch_ a new debuggee or _attach_ to an existing one.
We also use it to determine that a given debug scenario requires running a _locator_.

```rust
impl zed::Extension for MyExtension {
    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String>;
}
```

These two functions are sufficient to expose your debug adapter in `debug.json`-based user workflows, but you should strongly consider implementing `dap_config_to_scenario` as well.

```rust
impl zed::Extension for MyExtension {
    fn dap_config_to_scenario(
        &mut self,
        _adapter_name: DebugConfig,
    ) -> Result<DebugScenario, String>;
}
```

`dap_config_to_scenario` is used when the user spawns a session via new process modal UI. At a high level, it takes a generic debug configuration (that isn't specific to any
debug adapter) and tries to turn it into a concrete debug scenario for your adapter.
Put another way, it is supposed to answer the question: "Given a program, a list of arguments, current working directory and environment variables, what would the configuration for spawning this debug adapter look like?".

## Defining Debug Locators

Zed offers an automatic way to create debug scenarios with _debug locators_.
A locator locates the debug target and figures out how to spawn a debug session for it. Thanks to locators, we can automatically convert existing user tasks (e.g. `cargo run`) and convert them into debug scenarios (e.g. `cargo build` followed by spawning a debugger with `target/debug/my_program` as the program to debug).

> Your extension can define its own debug locators even if it does not expose a debug adapter. We strongly recommend doing so when your extension already exposes language tasks, as it allows users to spawn a debug session without having to manually configure the debug adapter.

Locators can (but don't have to) be agnostic to the debug adapter they are used with. They are simply responsible for locating the debug target and figuring out how to spawn a debug session for it. This allows for a more flexible and extensible debugging experience.

Your extension can define one or more debug locators. Each debug locator must be registered in the `extension.toml`:

```toml
[debug_locators.my-debug-locator]
```

Locators have two components.
First, each locator is ran on each available task to figure out if any of the available locators can provide a debug scenario for a given task. This is done by calling `dap_locator_create_scenario`.

```rust
impl zed::Extension for MyExtension {
    fn dap_locator_create_scenario(
        &mut self,
        _locator_name: String,
        _build_task: TaskTemplate,
        _resolved_label: String,
        _debug_adapter_name: String,
    ) -> Option<DebugScenario>;
}
```

This function should return `Some` debug scenario when that scenario defines a debugging counterpart to a given user task.
Note that a `DebugScenario` can include a [build task](../debugger.md#build-tasks). If there is one, we will execute `run_dap_locator` after a build task is finished successfully.

```rust
impl zed::Extension for MyExtension {
    fn run_dap_locator(
        &mut self,
        _locator_name: String,
        _build_task: TaskTemplate,
    ) -> Result<DebugRequest, String>;
}
```

`run_dap_locator` is useful in case you cannot determine a build target deterministically. Some build systems may produce artifacts whose names are not known up-front.
Note however that you do _not_ need to go through a 2-phase resolution; if you can determine the full debug configuration with just `dap_locator_create_scenario`, you can omit `build` property on a returned `DebugScenario`. Please also note that your locator **will be** called with tasks it's unlikely to accept; thus you should take some effort to return `None` early before performing any expensive operations.

## Available Extensions

Check out all the DAP servers that have already been exposed as extensions [on Zed's site](https://zed.dev/extensions?filter=debug-adapters).

We recommend taking a look at their repositories as a way to understand how they are generally created and structured.

## Testing

To test your new Debug Adapter Protocol server extension, you can [install it as a dev extension](./developing-extensions.md#developing-an-extension-locally).
