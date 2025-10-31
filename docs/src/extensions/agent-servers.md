# Agent Server Extensions

Agent Servers are programs that provide AI agent implementations through the [Agent Client Protocol (ACP)](https://agentclientprotocol.com). Agent Server Extensions let you package up an Agent Server so that users can install the extension and have your agent easily available to use in Zed.

You can see the agent servers that have already been exposed as extensions on [https://zed.dev/extensions](https://zed.dev/extensions?filter=agent-servers).

## Defining Agent Server Extensions

An extension can register one or more agent servers in the `extension.toml` like so:

```toml
[agent_servers.my-agent]
name = "My Agent"

[agent_servers.my-agent.targets.darwin-aarch64]
archive = "https://github.com/owner/repo/releases/download/v1.0.0/agent-darwin-arm64.tar.gz"
cmd = "./agent"
args = ["--serve"]

[agent_servers.my-agent.targets.linux-x86_64]
archive = "https://github.com/owner/repo/releases/download/v1.0.0/agent-linux-x64.tar.gz"
cmd = "./agent"
args = ["--serve"]

[agent_servers.my-agent.targets.windows-x86_64]
archive = "https://github.com/owner/repo/releases/download/v1.0.0/agent-windows-x64.zip"
cmd = "./agent.exe"
args = ["--serve"]
```

### Required Fields

- `name`: A human-readable display name for the agent server (shown in menus)
- `targets`: Platform-specific configurations for downloading and running the agent

### Target Configuration

Each target key uses the format `{os}-{arch}` where:

- **os**: `darwin` (macOS), `linux`, or `windows`
- **arch**: `aarch64` (ARM64) or `x86_64`

Each target must specify:

- `archive`: URL to download the archive from (supports `.tar.gz`, `.zip`, etc.)
- `cmd`: Command to run the agent server (relative to the extracted archive)
- `args`: Command-line arguments to pass to the agent server (optional)

### Optional Fields

You can also optionally specify:

- `sha256`: SHA-256 hash string of the archive's bytes. Zed will check this after the archive is downloaded and give an error if it doesn't match, so doing this improves security.
- `env`: Environment variables to set in the agent's spawned process.
- `icon`: Path to an SVG icon (relative to extension root) for display in menus.

### Complete Example

Here's a more complete example with all optional fields:

```toml
[agent_servers.example-agent]
name = "Example Agent"
icon = "icon/agent.svg"

[agent_servers.example-agent.env]
AGENT_LOG_LEVEL = "info"
AGENT_MODE = "production"

[agent_servers.example-agent.targets.darwin-aarch64]
archive = "https://github.com/example/agent/releases/download/v2.0.0/agent-darwin-arm64.tar.gz"
cmd = "./bin/agent"
args = ["serve", "--port", "8080"]
sha256 = "abc123def456..."

[agent_servers.example-agent.targets.linux-x86_64]
archive = "https://github.com/example/agent/releases/download/v2.0.0/agent-linux-x64.tar.gz"
cmd = "./bin/agent"
args = ["serve", "--port", "8080"]
sha256 = "def456abc123..."
```

## Installation Process

When a user installs your extension and selects the agent server:

1. Zed downloads the appropriate archive for the user's platform
2. The archive is extracted to a cache directory
3. Zed launches the agent using the specified command and arguments
4. Environment variables are set as configured
5. The agent server runs in the background, ready to assist the user

Archives are cached locally, so subsequent launches are fast.

## Distribution Best Practices

### Use GitHub Releases

GitHub Releases are a reliable way to distribute agent server binaries:

1. Build your agent for each platform (macOS ARM64, macOS x86_64, Linux x86_64, Windows x86_64)
2. Package each build as a compressed archive (`.tar.gz` or `.zip`)
3. Create a GitHub release and upload the archives
4. Use the release URLs in your `extension.toml`

## SHA-256 Hashes

It's good for security to include SHA-256 hashes of your archives in `extension.toml`. Here's how to generate it:

### macOS and Linux

```bash
shasum -a 256 agent-darwin-arm64.tar.gz
```

### Windows

```bash
certutil -hashfile agent-windows-x64.zip SHA256
```

Then add that string to your target configuration:

```toml
[agent_servers.my-agent.targets.darwin-aarch64]
archive = "https://github.com/owner/repo/releases/download/v1.0.0/agent-darwin-arm64.tar.gz"
cmd = "./agent"
sha256 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
```

## Testing

To test your Agent Server Extension:

1. [Install it as a dev extension](./developing-extensions.md#developing-an-extension-locally)
2. Open the [Agent Panel](../ai/agent-panel.md)
3. Select your Agent Server from the list
4. Verify that it downloads, installs, and launches correctly
5. Test its functionality by conversing with it and watching the [ACP logs](../ai/external-agents.md#debugging-agents)

This is all you need to distribute an agent server through Zed's extension system!
