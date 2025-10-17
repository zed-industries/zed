# Command Safety System for AI Agents

This document describes the implementation of a command safety system for AI agents in Zed, which provides blacklisting and whitelisting capabilities for terminal commands to prevent dangerous operations.

## Overview

The command safety system adds protection against AI agents executing dangerous terminal commands by:

1. **Built-in Dangerous Command Detection** - Automatically detects and flags known dangerous commands
2. **User-Configurable Whitelist** - Allows users to explicitly allow specific commands to bypass safety checks
3. **User-Configurable Blacklist** - Allows users to explicitly block additional commands
4. **Cross-Platform Support** - Works across Windows, macOS, and Linux

## Architecture

### Core Components

1. **`command_safety.rs`** - Core safety logic and command pattern matching
2. **Settings Integration** - New settings in `agent_settings.rs` and `settings_content/agent.rs`
3. **Terminal Tool Integration** - Modified `terminal_tool.rs` to use safety checks

### Key Files Modified/Created

- `crates/assistant_tools/src/command_safety.rs` (NEW)
- `crates/assistant_tools/src/terminal_tool.rs` (MODIFIED)
- `crates/settings/src/settings_content/agent.rs` (MODIFIED)
- `crates/agent_settings/src/agent_settings.rs` (MODIFIED)

## How It Works

### 1. Command Safety Assessment

When an AI agent wants to execute a command, the system:

1. Parses the command input
2. Checks if the command is explicitly blacklisted by the user
3. Checks if the command is explicitly whitelisted by the user  
4. If neither, checks against built-in dangerous command patterns
5. Returns one of: `Safe`, `Dangerous(reason)`, or `Whitelisted`

### 2. Built-in Dangerous Commands

The system includes extensive patterns for dangerous commands across platforms:

#### Destructive Commands
- `rm -rf` (Unix/Linux/macOS)
- `del /s /q` (Windows)
- `format c:` (Windows)
- `dd if=/dev/zero of=/dev/sda` (Unix)
- `shred` (Unix)

#### System Modification Commands
- `fdisk`, `parted`, `diskpart` 
- `chmod 777 /etc`, `reg add HKLM`
- `systemctl disable`, `sc delete`

#### Execution Risks
- `curl ... | sh`
- `sudo rm`, `runas /elevated`

#### Sensitive Access
- Reading `/etc/passwd`, `/etc/shadow`
- Accessing SSH keys, certificates
- `env` command (can expose secrets)

#### Network Risks
- `nc -l` (network listeners)
- `netsh` modifications

#### And many more...

### 3. User Configuration

Users can configure the system through settings:

```json
{
  "agent": {
    "command_safety": {
      "whitelist": [
        "git *",
        "npm install",
        "cargo build"
      ],
      "blacklist": [
        "rm -rf",
        "curl * | sh"
      ],
      "use_builtin_blacklist": true
    }
  }
}
```

### 4. Confirmation Flow

- **Safe commands** - Execute without confirmation
- **Whitelisted commands** - Execute without confirmation (even if normally dangerous)
- **Dangerous commands** - Require user confirmation with explanation of the risk

## Configuration Options

### `command_safety.whitelist`
Array of command patterns that are always allowed. Supports:
- Exact matches: `"npm install"`
- Wildcard patterns: `"git *"`

### `command_safety.blacklist`  
Array of command patterns that are always blocked. Same pattern support as whitelist.

### `command_safety.use_builtin_blacklist`
Boolean (default: true) - Whether to use the built-in dangerous command detection.

## Safety Features

1. **Platform-Aware** - Different patterns for Windows/macOS/Linux
2. **Regex-Based Matching** - Sophisticated pattern matching to catch variations
3. **Categorized Risks** - Different types of dangers (Destructive, System Modification, etc.)
4. **Context Preservation** - Maintains backward compatibility with existing `always_allow_tool_actions`

## Usage Examples

### Default Behavior
```json
{
  "agent": {
    "command_safety": {
      "use_builtin_blacklist": true
    }
  }
}
```
- Safe commands execute immediately
- Dangerous commands require confirmation
- No custom whitelist/blacklist

### Development Workflow
```json
{
  "agent": {
    "command_safety": {
      "whitelist": [
        "git *",
        "npm *", 
        "cargo *",
        "node *",
        "python *"
      ],
      "use_builtin_blacklist": true
    }
  }
}
```

### Highly Restrictive
```json
{
  "agent": {
    "command_safety": {
      "blacklist": [
        "rm *",
        "del *", 
        "format *",
        "sudo *"
      ],
      "use_builtin_blacklist": true
    }
  }
}
```

## Integration with Existing Settings

The new system integrates with the existing `always_allow_tool_actions` setting:
- If `always_allow_tool_actions: true`, no confirmation is required (legacy behavior)
- If `always_allow_tool_actions: false`, the new safety system is used

## Implementation Details

### Command Normalization
Commands are normalized (trimmed, lowercased) before pattern matching.

### Wildcard Support
Simple wildcard matching with `*` is supported for user-defined patterns.

### Error Handling
If command parsing fails, the system defaults to requiring confirmation (fail-safe).

### Performance
Built-in patterns are compiled once using `LazyLock` for efficient matching.

## Testing

The system includes comprehensive tests for:
- Dangerous command detection across platforms
- Whitelist/blacklist functionality
- Wildcard pattern matching
- Platform-specific behavior

## Security Considerations

1. **Fail-Safe Design** - When in doubt, require confirmation
2. **No Command Execution** - Only pattern matching, no actual command analysis
3. **User Override** - Users can always whitelist commands they trust
4. **Transparency** - Clear explanation of why commands are flagged as dangerous

## Future Enhancements

Potential future improvements:
- Machine learning-based risk assessment
- Command argument analysis
- Integration with system security policies
- Audit logging of dangerous command attempts