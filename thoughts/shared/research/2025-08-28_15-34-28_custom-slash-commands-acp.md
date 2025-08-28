---
date: 2025-08-28 15:34:28 PDT
researcher: Mikayla Maki
git_commit: 565782a1c769c90e58e012a80ea1c2d0cfcdb837
branch: claude-experiments
repository: zed
topic: "Custom Slash Commands for Agent Client Protocol"
tags: [research, codebase, acp, slash-commands, claude-code, protocol-extension]
status: complete
last_updated: 2025-08-28
last_updated_by: Mikayla Maki
---

# Research: Custom Slash Commands for Agent Client Protocol

**Date**: 2025-08-28 15:34:28 PDT
**Researcher**: Mikayla Maki
**Git Commit**: 565782a1c769c90e58e012a80ea1c2d0cfcdb837
**Branch**: claude-experiments
**Repository**: zed

## Research Question

We're adding support for custom slash commands to Agent Client Protocol. The client should be able to:

- List available commands
- Run a command with arguments (check Claude Code behavior)

In the Claude Code ACP adapter, we want implement the _agent_ side of the protocol:

- List commands by reading out of the `.claude/commands` directory
- Run commands via the SDK

We need to update the protocol to support the new RPCs for listing and running commands.
We need to understand how to run commands via the SDK.

## Summary

The research reveals a comprehensive architecture for implementing custom slash commands in ACP:

**Claude Code Slash Commands**: Well-established system with 15+ built-in commands, extensible architecture, and consistent execution patterns through the `SlashCommand` trait.

**ACP Protocol**: JSON-RPC based with clear patterns for adding new RPC methods through request/response enums, method dispatch, and capability negotiation.

**Claude Commands Structure**: Markdown-based command definitions in `.claude/commands/` with consistent format, metadata, and programmatic parsing potential.

**SDK Integration**: Claude Code ACP adapter bridges ACP protocol with Claude SDK, providing tool execution and session management through MCP servers.

## Detailed Findings

### Claude Code Slash Command Architecture

**Core Infrastructure** (`crates/assistant_slash_command/`):

- `assistant_slash_command.rs:1-200` - `SlashCommand` trait with `name()`, `description()`, `run()`, `complete_argument()`, `requires_argument()` methods
- `slash_command_registry.rs:1-150` - Global registry for command registration and lookup
- `slash_command_working_set.rs:1-100` - Working set management for active commands
- `extension_slash_command.rs:1-250` - Extension-based command support

**Execution Flow**:

1. Commands registered in `agent_ui.rs:403-448` via `slash_command_registry.register_command()`
2. User types `/command args` in editor
3. `SlashCommandCompletionProvider` provides completions
4. `TextThreadEditor.run_command()` calls `command.run()`
5. Commands return stream of `SlashCommandEvent`s
6. Events processed and inserted into context

**Built-in Commands** (`crates/assistant_slash_commands/`):

- File operations: `/file`, `/tab`, `/selection`
- Development: `/cargo-workspace`, `/diagnostics`, `/symbols`
- Context: `/context-server`, `/fetch`, `/prompt`
- Utility: `/now`, `/delta`

### Agent Client Protocol RPC Patterns

**Core Structure** (`agentic-coding-protocol/`):

- JSON-RPC based bidirectional communication
- Type-safe request/response enums with static dispatch
- Capability negotiation for feature opt-in
- Auto-generated JSON Schema from Rust types

**RPC Method Pattern**:

1. Define request/response structs with `#[derive(Serialize, Deserialize, JsonSchema)]`
2. Add method name constant: `const NEW_METHOD_NAME: &str = "new/method"`
3. Add variants to `ClientRequest`/`AgentRequest` enums
4. Update trait definition with async method signature
5. Add to dispatch logic in `decode_request()` and `handle_request()`

**Existing Methods**:

- `initialize` - Capability negotiation and authentication
- `session/new`, `session/load` - Session management
- `session/prompt` - Message processing
- `fs/read_text_file`, `fs/write_text_file` - File operations
- `session/request_permission` - Permission requests

### .claude/commands Directory Structure

**Format**: Markdown files with consistent structure:

```markdown
# Command Name

[Description]

## Initial Response

[Standardized first response]

## Process Steps

### Step 1: [Phase Name]

[Instructions]

## Important Guidelines

[Constraints and behaviors]
```

**Metadata Extraction Points**:

- H1 title for command name and description
- "Initial Response" section for invocation behavior
- Sequential process steps under "Process Steps"
- Checkbox lists (`- [ ]`, `- [x]`) for progress tracking
- Code blocks with executable commands

**Command Categories**:

- Development workflow: `create_plan.md`, `implement_plan.md`, `validate_plan.md`, `commit.md`
- Research: `research_codebase.md`, `debug.md`
- Project management: `ralph_plan.md`, `founder_mode.md`

### Claude Code ACP Adapter Implementation

**Architecture** (`claude-code-acp/src/`):

- `acp-agent.ts` - Main `ClaudeAcpAgent` implementing ACP Agent interface
- `mcp-server.ts` - Internal MCP server for file operations and permissions
- `tools.ts` - Tool conversion between Claude and ACP formats
- Session management with unique IDs and Claude SDK `Query` objects

**Integration Pattern**:

```typescript
let q = query({
  prompt: input,
  options: {
    cwd: params.cwd,
    mcpServers: { acp: mcpServerConfig },
    allowedTools: ["mcp__acp__read"],
    disallowedTools: ["Read", "Write", "Edit", "MultiEdit"],
  },
});
```

**Tool Execution Flow**:

1. ACP client makes tool request
2. Claude ACP agent converts to Claude SDK format
3. Internal MCP server proxies to ACP client capabilities
4. Results converted back to ACP format

## Code References

- `crates/assistant_slash_command/src/assistant_slash_command.rs:1-200` - Core SlashCommand trait
- `crates/agent_ui/src/agent_ui.rs:403-448` - Command registration point
- `agentic-coding-protocol/rust/agent.rs:604-610` - ACP request enum pattern
- `agentic-coding-protocol/rust/acp.rs:355-371` - Method dispatch logic
- `claude-code-acp/src/acp-agent.ts:1-500` - ACP adapter implementation
- `.claude/commands/*.md` - Command definition files

## Architecture Insights

**Slash Command System**: Highly modular with clear trait-based abstraction, supports both built-in and extension commands, uses streaming execution model with event-based progress reporting.

**ACP Protocol**: Designed for extensibility with capability negotiation, type safety through Rust enums, symmetric bidirectional design, and JSON-RPC foundation.

**Command Definitions**: Human-readable markdown with programmatically parseable structure, consistent metadata patterns, and workflow automation framework.

**Integration Patterns**: Claude Code ACP adapter provides proven pattern for bridging protocols, MCP servers enable tool execution proxying, session management handles concurrent interactions.

## Implementation Recommendations

### 1. Protocol Extension for Custom Commands

Add new RPC methods to ACP schema:

```rust
// New request types
pub struct ListCommandsRequest {
    pub session_id: SessionId,
}

pub struct RunCommandRequest {
    pub session_id: SessionId,
    pub command: String,
    pub args: Option<String>,
}

// Response types
pub struct ListCommandsResponse {
    pub commands: Vec<CommandInfo>,
}

pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub requires_argument: bool,
}
```

### 2. Agent Implementation

In Claude Code ACP adapter:

```typescript
async listCommands(request: ListCommandsRequest): Promise<ListCommandsResponse> {
  // Read .claude/commands directory
  // Parse markdown files for metadata
  // Return CommandInfo array
}

async runCommand(request: RunCommandRequest): Promise<RunCommandResponse> {
  // Find command definition in .claude/commands/
  // Execute via Claude SDK query with command content
  // Stream results back via session notifications
}
```

### 3. Command Parsing

Implement markdown parser for `.claude/commands/*.md`:

```typescript
function parseCommandFile(content: string): CommandInfo {
  // Extract H1 title for name/description
  // Find "Initial Response" section
  // Parse metadata and requirements
  // Return structured command info
}
```

## Open Questions

1. **Command Arguments**: How should complex command arguments be structured and validated?

2. **Command Context**: Should commands have access to current session state, file context, or conversation history?

3. **Command Discovery**: Should commands be cached or re-read on each listing request?

4. **Extension Commands**: How should extension-provided commands integrate with `.claude/commands/` file-based commands?

5. **Command Execution**: Should commands run in isolated contexts or share session state?

6. **Error Handling**: What's the appropriate error handling strategy for command parsing and execution failures?

###### FINAL CLAUDE PROMPT:

⏺ I've completed comprehensive research on adding custom
slash commands to the Agent Client Protocol. Here's what
I found:

Key Findings

Slash Command Architecture: Claude Code has a
well-established slash command system with 15+ built-in
commands using a SlashCommand trait. Commands are
registered globally and execute through streaming events.

ACP Protocol Patterns: The protocol uses JSON-RPC with
type-safe request/response enums. Adding new RPC methods
follows a clear pattern: define structs, add to enums,
update dispatch logic.

.claude/commands Structure: Commands are defined as
structured Markdown files with consistent format -
perfect for programmatic parsing. Each contains metadata,
process steps, and execution guidelines.

SDK Integration: The Claude Code ACP adapter already
bridges ACP with the Claude SDK through MCP servers,
providing a proven pattern for command execution.

Implementation Approach

You'll need to:

1. Extend ACP Protocol: Add ListCommandsRequest/Response
   and RunCommandRequest/Response to the schema
2. Parse Command Definitions: Read .claude/commands/\*.md
   files and extract metadata (name, description,
   requirements)
3. Execute Commands: Run commands via Claude SDK queries,
   similar to how the current ACP adapter works
4. Handle Streaming: Use ACP's session notification
   system to stream command progress

The research document is saved at thoughts/shared/researc
h/2025-08-28_15-34-28_custom-slash-commands-acp.md with
detailed implementation recommendations, code references,
and open questions for your consideration.

###### STATS ON PROMPT TO GENERATE THIS DOCUMENT:

Total cost: $2.87
Total duration (API): 12m 59.0s
Total duration (wall): 6m 52.2s
Total code changes: 242 lines added, 0 lines removed
Usage by model:
claude-3-5-haiku: 4.3k input, 175 output, 0 cache read, 0 cache write
claude-sonnet: 2.9k input, 31.3k output, 4.8m cache read, 256.1k cache write
