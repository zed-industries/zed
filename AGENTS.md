# Zed Context Server Integration Guide

## Project Overview

**Project**: Context Server Integration in Zed editor - supporting both local (stdio) and remote (HTTP/SSE) Model Context Protocol (MCP) servers with full backward compatibility.

**Repository**: https://github.com/zed-industries/zed
**MCP Specification**: https://modelcontextprotocol.io/specification
**RMCP SDK**: https://crates.io/crates/rmcp (v0.6.2+)
**License**: GPL-3.0-or-later
**Rust Edition**: 2024

## Architecture Overview

### Core Design Principles

1. **Unified Settings Format**: Single JSON structure supporting multiple transport types
2. **Backward Compatibility**: All existing MCP configurations continue working
3. **Feature Flag Control**: Use `rmcp` feature flag for HTTP/SSE transport support
4. **Consistent UI**: Most UI works across all transport types
5. **Progressive Enhancement**: Stdio works without RMCP, HTTP/SSE requires RMCP feature

### Transport Support Matrix

|Transport|Status|Feature Flag|Description|
|---|---|---|---|
|**Stdio**|✅ Stable|None required|Local process communication|
|**HTTP**|🚧 Implemented|`rmcp`|REST API communication|
|**SSE**|🚧 Implemented|`rmcp`|Server-Sent Events streaming|
|**WebSocket**|📋 Future|`rmcp`|Bidirectional streaming (planned)|

## Settings.json Format Specification

### Complete Format Reference

The `context_servers` section in `settings.json` supports multiple server types:

#### **Custom Stdio Server (Local Process)**
```json
{
  "context_servers": {
    "filesystem": {
      "source": "custom",
      "enabled": true,
      "command": "/usr/local/bin/mcp-server-filesystem", 
      "args": ["--root", "/home/user/projects"],
      "env": {
        "DEBUG": "1",
        "LOG_LEVEL": "info"
      }
    }
  }
}
```

#### **Custom HTTP Server (Remote API)**
```json
{
  "context_servers": {
    "remote-api": {
      "source": "custom",
      "enabled": true,
      "transport": {
        "type": "http",
        "url": "https://api.example.com/mcp",
        "headers": {
          "Authorization": "Bearer your-api-token",
          "Content-Type": "application/json"
        }
      }
    }
  }
}
```

#### **Custom SSE Server (Server-Sent Events)**
```json
{
  "context_servers": {
    "streaming-server": {
      "source": "custom", 
      "enabled": true,
      "transport": {
        "type": "sse",
        "url": "https://streaming.example.com/mcp/events",
        "headers": {
          "Authorization": "Bearer your-streaming-token",
          "Accept": "text/event-stream"
        }
      }
    }
  }
}
```

#### **Extension Server (Managed by Extensions)**
```json
{
  "context_servers": {
    "extension-provided": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "project_id": "my-project-123",
        "api_endpoint": "custom-value",
        "timeout": 30000
      }
    }
  }
}
```

#### **Complete Example with Mixed Types**
```json
{
  "context_servers": {
    "filesystem": {
      "source": "custom",
      "enabled": true,
      "command": "mcp-server-filesystem",
      "args": ["--root", "/home/user"]
    },
    "git": {
      "source": "custom", 
      "enabled": true,
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-git"]
    },
    "remote-api": {
      "source": "custom",
      "enabled": true,
      "transport": {
        "type": "http",
        "url": "https://api.example.com/mcp",
        "headers": {
          "Authorization": "Bearer token123"
        }
      }
    },
    "extension-server": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "key": "value"
      }
    }
  }
}
```

### Field Specifications

#### **Common Fields**
- `source`: `"custom" | "extension"` - Required. Determines server type
- `enabled`: `boolean` - Optional, defaults to `true`

#### **Custom Stdio Fields**
- `command`: `string` - Path to executable 
- `args`: `string[]` - Command line arguments
- `env`: `object` - Environment variables (optional)

#### **Custom Transport Fields**
- `transport.type`: `"http" | "sse"` - Transport protocol
- `transport.url`: `string` - Server endpoint URL
- `transport.headers`: `object` - HTTP headers (optional)

#### **Extension Fields**  
- `settings`: `object` - Extension-specific configuration

## Core Implementation Files

### Settings Type Definitions

#### **`crates/project/src/project_settings.rs`** - Main Settings Structure
```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, SettingsUi, SettingsKey)]
pub struct ProjectSettings {
    #[serde(default)]
    pub context_servers: HashMap<Arc<str>, ContextServerSettings>,
    // ... other fields
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum ContextServerSettings {
    Custom {
        #[serde(default = "default_true")]
        enabled: bool,
        #[serde(flatten)]
        config: ContextServerConfig,
    },
    Extension {
        #[serde(default = "default_true")]
        enabled: bool,
        settings: serde_json::Value,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum ContextServerConfig {
    Stdio {
        command: PathBuf,
        args: Vec<String>,
        #[serde(default)]
        env: Option<HashMap<String, String>>,
    },
    Transport {
        transport: TransportConfig,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum TransportConfig {
    #[serde(rename = "http")]
    Http {
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
    },
    #[serde(rename = "sse")]
    Sse {
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
    },
}
```

### Runtime Management

#### **`crates/project/src/context_server_store.rs`** - Core Management
```rust
pub struct ContextServerStore {
    // Maps server IDs to their settings from settings.json
    context_server_settings: HashMap<Arc<str>, ContextServerSettings>,
    // Maps server IDs to their runtime state  
    servers: HashMap<ContextServerId, ContextServerState>,
    // Dependencies for loading settings
    worktree_store: Entity<WorktreeStore>,
    project: WeakEntity<Project>,
    registry: Entity<ContextServerDescriptorRegistry>,
}

pub enum ContextServerConfiguration {
    Custom { config: ContextServerConfig },
    Extension { 
        config: ContextServerConfig,
        settings: serde_json::Value,
    },
}
```

#### **`crates/context_server/src/context_server.rs`** - Server Instances
```rust
pub struct ContextServer {
    id: ContextServerId,
    #[cfg(feature = "rmcp")]
    service: RwLock<Option<Arc<RunningService<RoleClient, ()>>>>,
    #[cfg(not(feature = "rmcp"))]
    client: RwLock<Option<()>>,
    configuration: ContextServerTransport,
}

pub enum ContextServerTransport {
    Stdio(ContextServerCommand, Option<PathBuf>),
    #[cfg(feature = "rmcp")]
    Http { url: String, headers: HashMap<String, String> },
    #[cfg(feature = "rmcp")]
    Sse { url: String, headers: HashMap<String, String> },
}
```

## Settings Loading Pipeline

### Data Flow Overview

```mermaid
graph TD
    A[settings.json] --> B[ProjectSettings::context_servers]
    B --> C[ContextServerStore::maintain_servers]
    C --> D[ContextServerConfiguration::from_settings]
    D --> E[ContextServer::new]
    E --> F[Running Context Server]
    
    G[Worktree Changes] --> C
    H[Settings Changes] --> C
    I[Extension Registry] --> D
    
    F --> J[Available Tools]
    F --> K[Agent Integration]
    F --> L[UI Status Updates]
```

### Key Processing Steps

1. **Settings Resolution**: `ProjectSettings::get(location, cx).context_servers`
2. **Configuration Creation**: `ContextServerConfiguration::from_settings()`
3. **Server Instantiation**: `ContextServer::new()` with transport-specific setup
4. **Service Initialization**: 
   - Stdio: Launch child process with command/args
   - HTTP: Configure HTTP client with URL/headers
   - SSE: Setup Server-Sent Events connection
5. **Tool Discovery**: Query server for available tools, prompts, resources
6. **UI Updates**: Emit status changes to agent panel and other UI components

## Migration System

### Historical Format Evolution

The settings format has evolved through automated migrations:

#### **Migration m_2025_06_16**: Add Source Field
- **Purpose**: Distinguish between custom and extension servers
- **Changes**: Added `"source": "custom"` or `"source": "extension"`

#### **Migration m_2025_06_27**: Flatten Command Structure  
- **Purpose**: Simplify command configuration
- **Before**: `"command": {"path": "...", "args": [...], "env": {...}}`
- **After**: `"command": "...", "args": [...], "env": {...}`

### Legacy Format Support

Old configurations are automatically migrated:

```json
// OLD (pre-migration)
{
  "context_servers": {
    "server": {
      "command": {
        "path": "/usr/bin/server",
        "args": ["--flag"]
      }
    }
  }
}

// NEW (automatically migrated)  
{
  "context_servers": {
    "server": {
      "source": "custom",
      "enabled": true,
      "command": "/usr/bin/server", 
      "args": ["--flag"]
    }
  }
}
```

## UI Integration

### Configuration Modal

**File**: `crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs`

- **Parsing**: Converts JSON input to `ContextServerCommand` 
- **Creation**: Builds `ContextServerSettings::Custom` with `ContextServerConfig::Stdio`
- **Editing**: Handles existing server modification
- **Validation**: JSON schema validation and error reporting

### Agent Panel Integration

**File**: `crates/agent_ui/src/agent_configuration/agent_configuration.rs`

- **Status Display**: Shows server connection state (connected/error/disabled)
- **Tool Availability**: Lists available tools from each server
- **Configuration Access**: Links to server configuration modal

### Status Indicators

- **Green**: Server connected and responding
- **Red**: Connection error or server unavailable  
- **Yellow**: Connecting or initializing
- **Gray**: Server disabled

## Extension Integration

### Extension API

Extensions can provide context servers through the WIT interface:

**Files**:
- `crates/extension_api/wit/since_v0.*.0/settings.rs` - API definitions
- `crates/extension_host/src/wasm_host/wit/since_v0_6_0.rs` - Host implementation

### Extension Settings Flow

1. Extension declares context server in manifest
2. User configures extension-specific settings in settings.json
3. Extension provides command/configuration through API
4. Zed creates `ContextServerConfiguration::Extension` instance
5. Server runs with extension-provided command + user settings

## Agent Profile Integration  

### Per-Server Tool Control

**File**: `crates/agent_settings/src/agent_settings.rs`

```json
{
  "agent": {
    "profiles": {
      "profile-name": {
        "name": "Profile Name",
        "enable_all_context_servers": true,
        "context_servers": {
          "server-id": {
            "tools": {
              "tool-name": true,
              "disabled-tool": false
            }
          }
        }
      }
    }
  }
}
```

This allows fine-grained control over which tools from each context server are available to specific agent profiles.

## RMCP Integration

### Feature Flag Configuration

```toml
# Cargo.toml
[features]
default = ["rmcp"]
rmcp = ["dep:rmcp", "dep:tokio"]

[dependencies]
rmcp = { version = "0.6.2", optional = true, features = [
    "client",
    "transport-sse-client-reqwest", 
    "transport-streamable-http-client-reqwest",
    "transport-child-process",
    "tower",
    "auth"
] }
```

### Service API Usage

```rust
impl ContextServer {
    #[cfg(feature = "rmcp")]
    pub async fn list_tools(&self) -> Result<Vec<rmcp::model::Tool>> {
        let service = self.service.read();
        let service = service.as_ref().ok_or_else(|| anyhow!("Not initialized"))?;
        Ok(service.list_all_tools().await?)
    }

    #[cfg(feature = "rmcp")]
    pub async fn call_tool(&self, params: rmcp::model::CallToolRequestParam) 
        -> Result<rmcp::model::CallToolResult> {
        let service = self.service.read();
        let service = service.as_ref().ok_or_else(|| anyhow!("Not initialized"))?;
        Ok(service.call_tool(params).await?)
    }
}
```

## Visual Verification Guide

### Check Connection Status

1. **Agent Panel Status**:
   - Open Agent Panel: `Cmd+Shift+A` (macOS) / `Ctrl+Shift+A` (Windows/Linux)
   - Look for context server section with connection indicators
   - Green = connected, Red = error, Yellow = connecting

2. **Tool Availability**:
   - In agent chat, type `@` to see available tools
   - Tools should be categorized by server name
   - Remote server tools appear alongside local tools

3. **Debug Logging**:
   ```bash
   RUST_LOG=context_server=debug,project=debug /path/to/zed
   ```
   Look for connection attempts, errors, and tool discoveries.

### Test Remote Server Manually

```bash
# Test HTTP server directly
curl -X POST "https://api.example.com/mcp" \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/list"
  }'
```

### Common Issues

1. **JSON Schema Validation Errors**: May occur due to cached schemas - restart Zed after changes
2. **Connection Failures**: Check URL, authentication, and server availability
3. **Missing Tools**: Verify server is responding to `tools/list` requests
4. **Feature Flag**: Ensure RMCP feature is enabled for HTTP/SSE transports

## Build & Development

### Build Commands

```bash
# Build with RMCP support (default)
cargo build --bin zed

# Build without RMCP (stdio only)  
cargo build --bin zed --no-default-features

# Build specific crate with RMCP
cargo build -p context_server --features rmcp

# Run tests
cargo test -p project context_server_settings
cargo test -p context_server --features rmcp
```

### Testing with Real Servers

1. **MCP Server Examples**:
   - `@modelcontextprotocol/server-filesystem`
   - `@modelcontextprotocol/server-git`
   - `mcp-server-sqlite`

2. **Start Local Server**:
   ```bash
   npx -y @modelcontextprotocol/server-filesystem /path/to/root
   ```

3. **Configure in settings.json**:
   ```json
   {
     "context_servers": {
       "filesystem": {
         "source": "custom",
         "enabled": true, 
         "command": "npx",
         "args": ["-y", "@modelcontextprotocol/server-filesystem", "/path/to/root"]
       }
     }
   }
   ```

## Implementation Status

### ✅ Completed

- Settings format specification and parsing
- Stdio transport (stable)
- HTTP transport (implemented with RMCP)
- SSE transport (implemented with RMCP)
- Migration system for backward compatibility
- UI integration for configuration and status
- Extension integration API
- Agent profile tool control
- Comprehensive test coverage

### 🚧 In Progress

- UI improvements for transport selection
- Connection retry logic and error handling
- Performance optimizations
- Documentation and examples

### 📋 Planned

- WebSocket transport support
- Advanced authentication methods (OAuth, JWT)
- Server discovery mechanisms
- Connection pooling and caching
- Metrics and monitoring integration

## Security Considerations

### Authentication
- HTTP Bearer tokens via headers
- Environment variable protection (redacted in logs)
- Secure credential storage integration

### Network Security
- HTTPS-only for remote connections
- Header-based authentication
- Request/response validation
- Timeout and retry limits

## Troubleshooting

### Common Error Messages

1. **"Transport configurations cannot be edited in this modal"**: HTTP/SSE servers need RMCP feature enabled
2. **"HTTP transport requested but RMCP feature not enabled"**: Build with `--features rmcp`
3. **"Context server not initialized"**: Server failed to connect, check logs and configuration
4. **JSON validation errors**: Schema cache issue, restart Zed or check format

### Debug Steps

1. Verify settings.json format matches specification exactly
2. Check server availability with manual HTTP requests  
3. Enable debug logging for detailed connection info
4. Test with simple stdio server first, then move to remote
5. Verify RMCP feature flag for HTTP/SSE transports

## Contributing Guidelines

### Code Standards

- Use RMCP types directly, never create wrapper types
- Test with both `--features rmcp` and `--no-default-features`
- Maintain backward compatibility for settings format
- Follow existing error handling patterns with `anyhow::Result`
- Add integration tests for new transport types

### Pull Request Checklist

- [ ] Settings parsing works for all transport types
- [ ] Tests pass with and without RMCP feature
- [ ] UI updates don't break existing functionality  
- [ ] Migration preserves existing configurations
- [ ] Documentation updated for new features

---

**Document Version**: 2.0.0  
**Last Updated**: December 2024  
**Status**: Current Implementation Guide  
**License**: GPL-3.0-or-later