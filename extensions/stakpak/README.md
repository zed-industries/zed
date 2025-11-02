# Stakpak Agent for Zed

Enterprise-grade DevOps agent with security features and infrastructure code generation capabilities.

## Features

- 🔒 **Security Hardened**: mTLS encryption, dynamic secret redaction, privacy mode
- 🛠️ **DevOps Optimized**: Async task management, real-time progress streaming
- 🧠 **Adaptive Intelligence**: Rule books, persistent knowledge, subagents
- 📦 **IaC Generation**: Terraform, Kubernetes, Dockerfile, GitHub Actions

## Installation

1. Open Zed Editor
2. Open the Command Palette (`Cmd+Shift+P` on macOS or `Ctrl+Shift+P` on Windows/Linux)
3. Search for "zed: extensions"
4. Search for "Stakpak Agent" in the extensions list
5. Click "Install"

The extension will automatically download the appropriate Stakpak binary for your platform.

## Usage

1. Get an API key from [stakpak.dev](https://stakpak.dev)
2. Set environment variable: `export STAKPAK_API_KEY=<your-key>`
3. Restart Zed (or start it from a terminal with the environment variable set)
4. Open Agent Panel in Zed
5. Select "Stakpak" from the agent list
6. Start interacting with the agent

## Authentication

Stakpak requires an API key to function. Getting one is free and requires no credit card:

1. Visit [stakpak.dev](https://stakpak.dev)
2. Click "Login" → "Create API Key"
3. Copy your API key
4. Set the environment variable:
   ```bash
   # Add to your shell profile (~/.zshrc, ~/.bashrc, etc.)
   export STAKPAK_API_KEY=your_api_key_here
   ```
5. Restart your terminal and Zed

Alternatively, you can use the Stakpak CLI to save the API key:
```bash
stakpak login --api-key $STAKPAK_API_KEY
```

## Supported Platforms

- macOS (Apple Silicon and Intel)
- Linux (x86_64)
- Windows (x86_64)

## Capabilities

Stakpak agent excels at:

- **Infrastructure Code Generation**: Generate Terraform, Kubernetes manifests, Dockerfiles, and GitHub Actions workflows
- **DevOps Operations**: Execute complex DevOps tasks with built-in error handling and retries
- **Documentation Research**: Search and analyze documentation with semantic understanding
- **Code Analysis**: Perform semantic code search and analysis
- **Security**: All operations are performed with mTLS encryption and automatic secret redaction

## Links

- [Stakpak Website](https://stakpak.dev)
- [Stakpak GitHub Repository](https://github.com/stakpak/agent)
- [Agent Client Protocol](https://agentclientprotocol.com)
- [Extension Repository](https://github.com/stakpak/stakpak-zed-extension)

## Troubleshooting

### Agent not connecting

1. Verify your API key is set correctly:
   ```bash
   echo $STAKPAK_API_KEY
   ```
2. Make sure Zed was started from a terminal with the environment variable set
3. Check Zed logs for any error messages

### Binary download issues

If the extension fails to download the binary, you can manually install Stakpak:

```bash
# macOS (ARM64)
curl -LO https://github.com/stakpak/agent/releases/download/v0.2.66/stakpak-darwin-aarch64.tar.gz
tar -xzf stakpak-darwin-aarch64.tar.gz
sudo mv stakpak /usr/local/bin/

# Verify installation
stakpak --version
```

Then configure Zed to use the system-installed binary (see Zed extension settings).

## Support

For issues, questions, or feature requests:

- Extension issues: [stakpak-zed-extension issues](https://github.com/stakpak/stakpak-zed-extension/issues)
- Stakpak agent issues: [agent issues](https://github.com/stakpak/agent/issues)
- General questions: [stakpak.dev](https://stakpak.dev)

## License

Apache-2.0

Copyright (c) 2025 Stakpak Team

