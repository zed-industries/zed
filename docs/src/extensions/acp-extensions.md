# Agent Extensions

[Agent Client Protocol](https://agentclientprotocol.com/) servers can be exposed as extensions for use in the [agent panel](../ai/agent-panel.md).

## Defining Agent Extensions

A given extension may provide one or more agent servers.
Each agent server must be registered in the `extension.toml`:

```toml
[agent_servers.my-acp-server]
# Required, this shows up in the Zed UI when the user selects an agent server
name = "My ACP Server"
# Optional, SVG icon path relative to the extension's root directory. This will be displayed next to the agent name.
icon = "./path/to/icon.svg"

# A series of target specifications for automatically downloading and maintaining your agent binary.
# The target format is "{os}-{arch}" where:
# - os: "darwin" (macOS), "linux", "windows"
# - arch: "aarch64" (arm64), "x86_64"
[agent_servers.opencode.targets.darwin-aarch64]
# The URL to download the agent binary from. We support `zip` and `tar.gz` compression formats automatically.
# If this agent is distributed via npm, use `npm view {package_name} dist.tarball` to get the URL.
archive = "https://github.com/org/repository/releases/latest/download/agent-darwin-arm64.zip"
# The command to run the agent binary, relative to the extracted binary directory.
# If your agent is run with node, use `node` as the command and Zed will automatically substitute it with a recent node runtime.
cmd = "node"
# The arguments to pass to the above command.
args = ["./my-agent.mjs", "--acp"]
# Optional SHA-256 hash of the archive for verification.
# If not provided and the URL is a GitHub release, we'll attempt to fetch and verify it from GitHub.
sha256 = "12345678deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
```

Unlike other Zed extensions, there is no WASM component to Agent Extensions.

See the [OpenCode agent extension](https://github.com/sst/opencode-zed-extension) for an example of how these extensions work.

## Available Extensions

Check out all the ACP servers that have already been exposed as extensions [on Zed's site](https://zed.dev/extensions?filter=agent-servers).

We recommend taking a look at their repositories as a way to understand how they are generally created and structured.

## Testing

To test your new Agent Client Protocol server extension, you can [install it as a dev extension](./developing-extensions.md#developing-an-extension-locally).

## Publishing and distributing

See the ["Publishing your extension"](./developing-extensions.md#publishing-your-extension) section.
