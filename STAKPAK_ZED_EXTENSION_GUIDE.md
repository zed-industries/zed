# Stakpak Zed Agent Server Extension - Implementation Guide

## Overview

This document provides a complete guide for integrating Stakpak as an Agent Server Extension in Zed Editor using the new Agent Server Extensions feature (PR going live this week).

## What is Stakpak?

Stakpak is a terminal-native DevOps Agent built in Rust that specializes in:
- Infrastructure code generation (Terraform, Kubernetes, Dockerfile, GitHub Actions)
- DevOps operations with enterprise-grade security (mTLS, secret redaction)
- Real-time progress streaming for long-running operations
- Documentation research and semantic code search

**Key Feature**: Stakpak already implements the Agent Client Protocol (ACP) via the `stakpak acp` command.

- **Repository**: https://github.com/stakpak/agent
- **License**: Apache-2.0
- **Latest Releases**: Available at https://github.com/stakpak/agent/releases

## Zed Agent Server Extensions

Agent Server Extensions allow packaging ACP-compliant agents for easy distribution through Zed's extension system. The new feature (documented in [PR ea273e738d3e4eada4a4741aa8d6431ca7cc3d44](https://github.com/zed-industries/zed/blob/ea273e738d3e4eada4a4741aa8d6431ca7cc3d44/docs/src/extensions/agent-servers.md)) enables:

- **Automatic binary distribution** across platforms (macOS, Linux, Windows)
- **Version management** with archive downloads and caching
- **Security verification** via SHA-256 hashes
- **Easy installation** for end users (no manual configuration)

## Implementation Steps

### Step 1: Create Extension Repository

Create a new GitHub repository named `stakpak-zed-extension` with this structure:

```
stakpak-zed-extension/
├── extension.toml          # Main configuration file
├── icon.svg               # Stakpak logo (16x16, monochrome)
├── LICENSE                # Apache-2.0 or MIT
└── README.md              # User documentation
```

### Step 2: Create `extension.toml`

This is the core configuration file that tells Zed how to download and run Stakpak:

```toml
id = "stakpak"
name = "Stakpak Agent"
version = "0.1.0"
schema_version = 1
authors = ["Stakpak Team <team@stakpak.dev>"]
description = "Enterprise-grade DevOps agent with security features and infrastructure code generation"
repository = "https://github.com/stakpak/stakpak-zed-extension"

[agent_servers.stakpak]
name = "Stakpak"
icon = "icon.svg"

# Environment variables (if needed)
[agent_servers.stakpak.env]
# STAKPAK_LOG_LEVEL = "info"  # Optional: Add if you want to control logging

# macOS ARM64 (M1/M2/M3/M4 Macs)
[agent_servers.stakpak.targets.darwin-aarch64]
archive = "https://github.com/stakpak/agent/releases/download/v0.2.66/stakpak-aarch64-apple-darwin.tar.gz"
cmd = "./stakpak"
args = ["acp"]
sha256 = "REPLACE_WITH_ACTUAL_SHA256"

# macOS x86_64 (Intel Macs)
[agent_servers.stakpak.targets.darwin-x86_64]
archive = "https://github.com/stakpak/agent/releases/download/v0.2.66/stakpak-x86_64-apple-darwin.tar.gz"
cmd = "./stakpak"
args = ["acp"]
sha256 = "REPLACE_WITH_ACTUAL_SHA256"

# Linux x86_64
[agent_servers.stakpak.targets.linux-x86_64]
archive = "https://github.com/stakpak/agent/releases/download/v0.2.66/stakpak-x86_64-unknown-linux-gnu.tar.gz"
cmd = "./stakpak"
args = ["acp"]
sha256 = "REPLACE_WITH_ACTUAL_SHA256"

# Windows x86_64
[agent_servers.stakpak.targets.windows-x86_64]
archive = "https://github.com/stakpak/agent/releases/download/v0.2.66/stakpak-x86_64-pc-windows-msvc.zip"
cmd = "./stakpak.exe"
args = ["acp"]
sha256 = "REPLACE_WITH_ACTUAL_SHA256"
```

**Important Notes:**
- Replace version `v0.2.66` with the latest version from releases
- Update URLs to match actual release asset names (check https://github.com/stakpak/agent/releases/latest)
- The `args = ["acp"]` tells Stakpak to start in ACP server mode
- SHA-256 hashes must be generated for each archive (see below)

### Step 3: Generate SHA-256 Hashes

For each binary archive, generate and verify SHA-256 hashes:

**On macOS/Linux:**
```bash
# Download each archive
curl -LO https://github.com/stakpak/agent/releases/download/v0.2.66/stakpak-aarch64-apple-darwin.tar.gz

# Generate SHA-256
shasum -a 256 stakpak-aarch64-apple-darwin.tar.gz
```

**On Windows:**
```bash
# Download archive
curl -LO https://github.com/stakpak/agent/releases/download/v0.2.66/stakpak-x86_64-pc-windows-msvc.zip

# Generate SHA-256
certutil -hashfile stakpak-x86_64-pc-windows-msvc.zip SHA256
```

Alternatively, GitHub sometimes provides checksums in release notes or a separate checksums file.

### Step 4: Create Icon (Optional but Recommended)

Create a monochrome SVG icon following Zed's guidelines:
- **Size**: 16x16 bounding box
- **Padding**: 1-2 pixels
- **Style**: Monochrome only (no gradients)
- **Format**: Clean SVG (process through [SVGOMG](https://jakearchibald.github.io/svgomg/))
- **Opacity**: Can use opacity for visual layering

Save as `icon.svg` in the extension root.

### Step 5: Create README.md

```markdown
# Stakpak Agent for Zed

Enterprise-grade DevOps agent with security features and infrastructure code generation capabilities.

## Features

- 🔒 **Security Hardened**: mTLS encryption, dynamic secret redaction, privacy mode
- 🛠️ **DevOps Optimized**: Async task management, real-time progress streaming
- 🧠 **Adaptive Intelligence**: Rule books, persistent knowledge, subagents
- 📦 **IaC Generation**: Terraform, Kubernetes, Dockerfile, GitHub Actions

## Usage

1. Install the Stakpak extension in Zed
2. Get an API key from [stakpak.dev](https://stakpak.dev)
3. Set environment variable: `export STAKPAK_API_KEY=<your-key>`
4. Open Agent Panel in Zed and select "Stakpak"

## Authentication

Stakpak requires an API key. Get one for free (no card required):
1. Visit [stakpak.dev](https://stakpak.dev)
2. Click "Login" → "Create API Key"
3. Set `STAKPAK_API_KEY` environment variable

## Links

- [Stakpak Documentation](https://stakpak.dev)
- [GitHub Repository](https://github.com/stakpak/agent)
- [Agent Client Protocol](https://agentclientprotocol.com)

## License

Apache-2.0
```

### Step 6: Add LICENSE File

Copy the Apache-2.0 license from the main Stakpak repository or use MIT license.

### Step 7: Test Locally

1. **Build/clone your extension locally**
2. **In Zed Editor:**
   - Open Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`)
   - Run `zed: install dev extension`
   - Select your extension directory
3. **Test the agent:**
   - Open Agent Panel
   - Select "Stakpak" from the agent list
   - Verify it downloads, extracts, and launches correctly
   - Test basic functionality (send a message, check responses)
4. **Debug if needed:**
   - Check Zed logs for errors
   - Verify archive URLs are correct
   - Confirm SHA-256 hashes match
   - Test the command locally: `stakpak acp`

### Step 8: Submit to Zed Extensions

Once tested and working:

1. **Fork** `zed-industries/extensions` repository
   ```bash
   # On GitHub, click Fork button
   git clone https://github.com/YOUR_USERNAME/extensions.git
   cd extensions
   ```

2. **Add your extension as a submodule**
   ```bash
   git submodule add https://github.com/stakpak/stakpak-zed-extension.git extensions/stakpak
   git add extensions/stakpak
   ```

3. **Update `extensions.toml`** in the repository root
   ```toml
   [stakpak]
   submodule = "extensions/stakpak"
   version = "0.1.0"
   ```

4. **Sort extensions** (if sort script exists)
   ```bash
   pnpm sort-extensions  # Or npm run sort-extensions
   ```

5. **Commit and push**
   ```bash
   git commit -m "Add Stakpak agent server extension"
   git push origin main
   ```

6. **Open Pull Request** to `zed-industries/extensions`

### Step 9: PR Description

Use this template when submitting your PR:

```markdown
# Add Stakpak Agent Server Extension

This PR adds Stakpak as an agent server extension for Zed, leveraging the new Agent Server Extensions feature.

## About Stakpak

Stakpak is an enterprise-grade DevOps agent built in Rust with:
- Agent Client Protocol (ACP) support
- Security features (mTLS, secret redaction, privacy mode)
- Infrastructure code generation (Terraform, Kubernetes, Dockerfile, GitHub Actions)
- Real-time task management and progress streaming
- Documentation research and semantic search

**Repository**: https://github.com/stakpak/agent (362⭐, Apache-2.0)

## What's Included

- ✅ Agent server configuration for all platforms (macOS ARM/Intel, Linux, Windows)
- ✅ SHA-256 hashes for security verification
- ✅ Monochrome SVG icon following design guidelines
- ✅ Apache-2.0 license
- ✅ Comprehensive README with setup instructions

## Testing

- [x] Tested locally using `zed: install dev extension`
- [x] Verified agent launches correctly on macOS ARM64
- [x] Confirmed ACP communication works end-to-end
- [x] Tested infrastructure code generation features
- [x] Validated authentication flow with API key

## Technical Details

- **ACP Command**: `stakpak acp`
- **Authentication**: Requires `STAKPAK_API_KEY` environment variable (free, no card required)
- **Binaries**: Pre-built releases from GitHub with verified checksums
- **Protocol**: Implements Agent Client Protocol v1.0

## Links

- Extension Repository: https://github.com/stakpak/stakpak-zed-extension
- Stakpak Main Repository: https://github.com/stakpak/agent
- Stakpak Website: https://stakpak.dev
- ACP Specification: https://agentclientprotocol.com
```

## Key Requirements Checklist

Before submitting, ensure:

- [ ] `extension.toml` has all required fields (id, name, version, schema_version, authors, description, repository)
- [ ] Agent servers configured for all 4 platforms (darwin-aarch64, darwin-x86_64, linux-x86_64, windows-x86_64)
- [ ] Archive URLs point to valid GitHub releases
- [ ] SHA-256 hashes are correct for each archive
- [ ] `cmd` field points to the binary (with .exe for Windows)
- [ ] `args = ["acp"]` starts Stakpak in ACP server mode
- [ ] Icon is monochrome SVG, 16x16, optimized
- [ ] LICENSE file exists (Apache-2.0 or MIT)
- [ ] README.md includes usage instructions and authentication setup
- [ ] Tested locally in Zed before submitting
- [ ] Extension repo is public on GitHub

## Authentication Considerations

Stakpak requires an API key for operation. In your README and PR description, clearly document:

1. **How to get an API key** (free at stakpak.dev, no card required)
2. **How to set it**: `export STAKPAK_API_KEY=<key>`
3. **Alternative**: Users can also run `stakpak login --api-key $STAKPAK_API_KEY` to save it to config

Consider adding to `extension.toml`:
```toml
[agent_servers.stakpak.env]
# Users should set STAKPAK_API_KEY in their shell environment
# Get your free API key at: https://stakpak.dev
```

## Next Steps

1. ✅ Verify latest Stakpak release version and binary naming convention
2. ✅ Download binaries and generate SHA-256 hashes
3. ✅ Create `stakpak-zed-extension` repository on GitHub
4. ✅ Add all files (extension.toml, icon.svg, LICENSE, README.md)
5. ✅ Test locally in Zed
6. ✅ Fork `zed-industries/extensions`
7. ✅ Add as submodule and update extensions.toml
8. ✅ Submit PR with detailed description
9. ✅ Wait for review and approval from Zed team

## Timeline

- **Now**: Agent Server Extensions PR is going live this week
- **After PR merge**: You can submit Stakpak extension immediately
- **Review time**: Typically 1-2 weeks for extension approval
- **After approval**: Stakpak will appear in Zed's extension marketplace

## Technical Architecture

```
Zed Editor
    ↓
Extension System (loads extension.toml)
    ↓
Downloads archive for user's platform
    ↓
Extracts to cache directory
    ↓
Runs: stakpak acp
    ↓
ACP Server starts (listens on stdio)
    ↓
Zed communicates via Agent Client Protocol
    ↓
User interacts through Zed's Agent Panel
```

## Resources

- **Zed Agent Server Extensions Docs**: https://github.com/zed-industries/zed/blob/ea273e738d3e4eada4a4741aa8d6431ca7cc3d44/docs/src/extensions/agent-servers.md
- **Zed Extensions Repository**: https://github.com/zed-industries/extensions
- **Zed Extension Development Docs**: https://zed.dev/docs/extensions
- **Agent Client Protocol**: https://agentclientprotocol.com
- **Stakpak Repository**: https://github.com/stakpak/agent
- **Stakpak Website**: https://stakpak.dev

## Questions or Issues?

If you encounter any issues during implementation:
1. Check Zed's extension development docs
2. Look at existing agent server extensions for reference
3. Open an issue in the Zed repository for extension-related questions
4. Reach out to Stakpak team for agent-specific issues

---

**Document Version**: 1.0  
**Last Updated**: November 2, 2025  
**Author**: Generated for Stakpak Zed Extension integration

