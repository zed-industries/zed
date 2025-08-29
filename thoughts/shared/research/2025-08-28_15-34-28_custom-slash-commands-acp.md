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

We're adding support for custom slash commands to Agent Client Protocol for the **agent panel** (not assistant 1/text threads). The client should be able to:

- List available commands
- Run a command with arguments (check Claude Code behavior)

In the Claude Code ACP adapter, we want implement the _agent_ side of the protocol:

- List commands by reading out of the `.claude/commands` directory
- Run commands via the SDK

We need to update the protocol to support the new RPCs for listing and running commands.
We need to understand how to run commands via the SDK.

**Important Note**: This is for the agent panel UX, NOT the existing assistant/text thread slash commands. The existing slash command infrastructure is for assistant 1/text threads and is not relevant to this implementation.

## Summary

The research reveals the architecture needed for implementing custom slash commands in the **agent panel** via ACP:

**Agent Panel Architecture**: Separate UI system from assistant/text threads with dedicated components (`AgentPanel`, `AcpThreadView`) and message handling through ACP protocol integration.

**ACP Protocol**: JSON-RPC based with clear patterns for adding new RPC methods through request/response enums, method dispatch, and capability negotiation. Handles session management, tool calls, and real-time message streaming.

**Claude Commands Structure**: Markdown-based command definitions in `.claude/commands/` with consistent format, metadata, and programmatic parsing potential.

**SDK Integration**: Claude Code ACP adapter bridges ACP protocol with Claude SDK, providing tool execution and session management through MCP servers.

**Note**: The existing Claude Code slash command system (`SlashCommand` trait, `assistant_slash_command` crate) is **not relevant** - that's for assistant 1/text threads. The agent panel needs its own custom command implementation.

## Detailed Findings

### Agent Panel Architecture

**Core Infrastructure** (`crates/agent_ui/`):

- `agent_panel.rs:24` - Main `AgentPanel` struct and UI component
- `acp/thread_view.rs:315` - `AcpThreadView` component for individual agent conversations
- `acp/message_editor.rs` - Message input component with slash command integration for agent panel
- `acp.rs` - ACP module entry point connecting to external ACP agents

**Agent Panel vs Assistant Distinction**:

The agent panel is **completely separate** from the assistant/text thread system:
- Agent panel uses ACP (Agent Client Protocol) for external agent communication
- Assistant uses internal Zed slash commands and text thread editors
- Different UI components, different input handling, different protocol integration

**ACP Integration Flow**:

1. External ACP agent process spawned via `agent_servers/src/acp.rs:63-76`
2. JSON-RPC connection established over stdin/stdout at line 84
3. Protocol initialization with capability negotiation at line 131
4. Sessions created via `new_session()` request for isolated conversations
5. User input converted to `PromptRequest` and sent to ACP agent
6. Agent responses stream back as `SessionUpdate` notifications
7. UI updates processed in `AcpThread::handle_session_update()`

**Current Input Handling**:

- Message composition through `MessageEditor` and specialized ACP message editor
- Standard chat input without custom command support currently
- Integration with model selector, context strip, and profile management

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

- `crates/agent_ui/src/agent_panel.rs:24` - Main AgentPanel component
- `crates/agent_ui/src/acp/thread_view.rs:315` - AcpThreadView UI component
- `crates/agent_ui/src/acp/message_editor.rs` - Agent panel message input
- `crates/agent_servers/src/acp.rs:63-162` - ACP connection establishment
- `crates/acp_thread/src/acp_thread.rs:826` - ACP thread creation
- `agentic-coding-protocol/rust/agent.rs:604-610` - ACP request enum pattern
- `agentic-coding-protocol/rust/acp.rs:355-371` - Method dispatch logic
- `claude-code-acp/src/acp-agent.ts:1-500` - ACP adapter implementation
- `.claude/commands/*.md` - Command definition files

## Architecture Insights

**Agent Panel System**: Completely separate from assistant/text threads, uses ACP protocol for external agent communication with JSON-RPC over stdin/stdout, manages sessions with unique IDs, and provides real-time message streaming with UI updates.

**ACP Protocol**: Designed for extensibility with capability negotiation, type safety through Rust enums, symmetric bidirectional design, and JSON-RPC foundation. Handles tool calls, permissions, and session management.

**Command Definitions**: Human-readable markdown with programmatically parseable structure, consistent metadata patterns, and workflow automation framework stored in `.claude/commands/`.

**Integration Patterns**: Claude Code ACP adapter provides proven pattern for bridging protocols, MCP servers enable tool execution proxying, session management handles concurrent interactions. Agent panel needs new command integration separate from existing slash commands.

## Implementation Recommendations

### 1. Protocol Extension for Custom Commands

Add new RPC methods to ACP schema following existing patterns in `agentic-coding-protocol/rust/`:

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

Add to request/response enums and implement in dispatch logic similar to existing ACP methods.

### 2. Agent Panel UI Integration

**Option A**: Extend ACP Message Editor
- Modify `crates/agent_ui/src/acp/message_editor.rs` to detect custom commands
- Add command completion/suggestion UI similar to existing patterns
- Trigger custom command execution through ACP protocol

**Option B**: New Command Interface
- Create dedicated command input component in agent panel
- Separate from regular message input to provide distinct UX
- Integrate with `AcpThreadView` for command results display

### 3. ACP Agent Implementation

In Claude Code ACP adapter (`claude-code-acp/src/acp-agent.ts`):

```typescript
async listCommands(request: ListCommandsRequest): Promise<ListCommandsResponse> {
  // Read .claude/commands directory
  // Parse markdown files for metadata
  // Return CommandInfo array
}

async runCommand(request: RunCommandRequest): Promise<RunCommandResponse> {
  // Find command definition in .claude/commands/
  // Execute via Claude SDK query with command content
  // Stream results back via session notifications using existing session update mechanism
}
```

### 4. Command Parsing and Execution

Implement markdown parser for `.claude/commands/*.md`:

```typescript
function parseCommandFile(content: string): CommandInfo {
  // Extract H1 title for name/description
  // Find "Initial Response" section
  // Parse metadata and requirements
  // Return structured command info
}
```

Execute commands by sending command content as system prompt to Claude SDK, similar to existing ACP query patterns.

## Open Questions

1. **Agent Panel UX**: Should custom commands be integrated into the existing message input or as a separate command interface?

2. **Command Arguments**: How should complex command arguments be structured and validated in the agent panel context?

3. **Command Context**: Should commands have access to current ACP session state, file context, or conversation history?

4. **Command Discovery**: Should commands be cached or re-read on each listing request? How does this integrate with ACP session lifecycle?

5. **Command Execution**: Should commands run in isolated contexts or share ACP session state?

6. **Error Handling**: What's the appropriate error handling strategy for command parsing and execution failures in the agent panel?

7. **UI Integration**: How should command execution progress and results be displayed within the `AcpThreadView` component?

###### FINAL CLAUDE PROMPT:

⏺ I've completed comprehensive research on adding custom
slash commands to the Agent Client Protocol for the
**agent panel** (NOT assistant/text threads). Here's what
I found:

Key Findings

Agent Panel Architecture: The agent panel is completely
separate from assistant/text threads, using ACP protocol
for external agent communication via JSON-RPC over
stdin/stdout. It has dedicated UI components like
AgentPanel and AcpThreadView.

ACP Protocol Patterns: The protocol uses JSON-RPC with
type-safe request/response enums. Adding new RPC methods
follows a clear pattern: define structs, add to enums,
update dispatch logic. Handles sessions, tool calls, and
real-time streaming.

.claude/commands Structure: Commands are defined as
structured Markdown files with consistent format -
perfect for programmatic parsing. Each contains metadata,
process steps, and execution guidelines.

SDK Integration: The Claude Code ACP adapter already
bridges ACP with the Claude SDK through MCP servers,
providing a proven pattern for command execution.

**IMPORTANT**: The existing Claude Code slash command
system (SlashCommand trait, assistant_slash_command crate)
is NOT relevant - that's for assistant 1/text threads.
The agent panel needs its own custom command
implementation.

Implementation Approach

You'll need to:

1. Extend ACP Protocol: Add ListCommandsRequest/Response
   and RunCommandRequest/Response to the schema
2. Agent Panel UI: Integrate custom commands into agent
   panel message input or create separate command interface
3. Parse Command Definitions: Read .claude/commands/\*.md
   files and extract metadata (name, description,
   requirements)
4. Execute Commands: Run commands via Claude SDK queries
   through existing ACP session mechanism
5. Handle Streaming: Use ACP's session notification
   system to stream command progress to AcpThreadView

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
