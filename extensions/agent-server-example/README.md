# Agent Server Example Extension

This is an example extension that demonstrates how to create an ACP (Agent Client Protocol) agent server extension for Zed.

## What This Example Shows

This extension demonstrates how to use the GitHub Release launcher to download and run an agent binary. It uses the codex-acp agent as a concrete example.

## Extension Structure

```
agent-server-example/
├── Cargo.toml          # Rust package configuration
├── extension.toml      # Agent server configuration
├── src/lib.rs         # Minimal extension implementation
└── README.md          # This file
```

## Creating Your Own Agent Extension

### 1. Extension Manifest (`extension.toml`)

The main configuration goes in `extension.toml`:

```toml
id = "my-agent"
name = "My Agent"
description = "My custom ACP agent"
version = "0.1.0"
schema_version = 1
authors = ["Your Name <you@example.com>"]
repository = "https://github.com/your/repo"

[agent_servers.MyAgent]
args = ["--some-flag"]
env = { MY_VAR = "value" }

[agent_servers.MyAgent.launcher]
repo = "owner/repo"
tag = "v1.2.3"
asset_pattern = "my-agent-{arch}-{platform}.{ext}"
binary_name = "my-agent"
```

### 2. Launcher Types

Extensions support three types of launchers:

#### GitHub Release Launcher (recommended for distribution)

Downloads a binary from GitHub releases:

```toml
[agent_servers.MyAgent.launcher]
repo = "owner/repo"
tag = "v1.2.3"
asset_pattern = "my-agent-{arch}-{platform}.{ext}"
binary_name = "my-agent"
```

**Template Variables:**
- `{arch}`: `x86_64` or `aarch64`
- `{platform}`: `apple-darwin`, `pc-windows-msvc`, or `unknown-linux-gnu`
- `{ext}`: `zip` on Windows, `tar.gz` elsewhere

You can also use wildcards: `asset_pattern = "my-agent-*.tar.gz"`

#### NPM Launcher

Installs from npm registry:

```toml
[agent_servers.MyAgent.launcher]
package = "@myorg/my-agent"
version = "1.2.3"
entrypoint = "node_modules/@myorg/my-agent/dist/index.js"
```

#### Binary Launcher

Uses a binary already in PATH:

```toml
[agent_servers.MyAgent.launcher]
bin_name = "my-agent"
```

### 3. Extension Code (`src/lib.rs`)

The Rust code is minimal - just register the extension:

```rust
use zed_extension_api as zed;

struct MyAgentExtension;

impl zed::Extension for MyAgentExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(MyAgentExtension);
```

### 4. Build Configuration (`Cargo.toml`)

```toml
[package]
name = "my-agent"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
crate-type = ["cdylib"]

[dependencies]
zed_extension_api = { path = "../../crates/extension_api" }
```

## Building and Testing

### Build the Extension

```bash
cargo build --release --target wasm32-wasip1
```

### Install Locally for Testing

```bash
mkdir -p ~/.local/share/zed/extensions/installed/my-agent
cp extension.toml extension.wasm ~/.local/share/zed/extensions/installed/my-agent/
```

Then restart Zed.

### Verify It Works

1. Open Agent Panel (Cmd+Shift+A)
2. Look for your agent under "External Agents"
3. Select it and send a message
4. On first use, it will download/install the agent binary

## Key Features

### Automatic Download & Caching

Extensions automatically:
- Download the correct binary for the user's platform
- Verify SHA-256 checksums (if provided by GitHub)
- Cache the binary at `~/.local/share/zed/data/external_agents/`
- Skip re-downloading if already cached

### No Auto-Updates

Extensions hardcode a specific version. To update:
- Bump the version in `extension.toml`
- Users update the extension itself

### Protocol-Only Authentication

Extensions support only what the ACP protocol provides - no custom auth UI or login commands. This keeps extensions simple.

## Example: Different Asset Naming Conventions

Some projects use different naming conventions. Here's how to handle them:

### Standard Convention (like codex-acp)
```toml
asset_pattern = "my-agent-0.3.8-{arch}-{platform}.{ext}"
# Matches: my-agent-0.3.8-aarch64-apple-darwin.tar.gz
```

### Custom Convention (like OpenCode)
If the project uses different names (e.g., "darwin" instead of "apple-darwin"), use wildcards:
```toml
asset_pattern = "my-agent-*.zip"
# Matches any .zip file - works when there's one per platform
```

### Including Version in Pattern
```toml
tag = "v1.2.3"
asset_pattern = "my-agent-v1.2.3-{arch}-{platform}.{ext}"
```

## Publishing

Once your extension works locally:

1. Create a repository for your extension
2. Add installation instructions to your README
3. Submit to the Zed extension registry (coming soon)

## Troubleshooting

### "Binary not found in PATH"
- For binary launchers, ensure the binary is installed and in PATH
- Try running `which my-agent` in your terminal

### "No asset found matching pattern"
- Check the GitHub releases page for actual asset names
- Adjust your `asset_pattern` to match
- Use wildcards if needed: `"my-agent-*.tar.gz"`

### "SHA-256 mismatch"
- GitHub API provides checksums automatically
- If you see this error, the download was corrupted - try again

### Agent doesn't appear in menu
- Restart Zed after installing
- Check Extensions panel to verify it's installed
- Look at Zed logs for errors

## Learn More

- **ACP Protocol**: See the ACP specification for agent capabilities
- **Built-in Agents**: Look at `crates/project/src/agent_server_store.rs` for how built-in agents work
- **Extension API**: See `crates/extension_api/` for available APIs

## Contributing

This example is part of the Zed project. Contributions welcome at https://github.com/zed-industries/zed