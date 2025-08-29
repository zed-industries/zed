---
date: 2025-08-28 15:34:28 PDT
researcher: Mikayla Maki
git_commit: 425291f0aed2abe148e1a8ea4eda74569e25c2b7
branch: claude-experiments
repository: zed
topic: "Custom Slash Commands for Agent Client Protocol"
tags: [research, codebase, acp, slash-commands, claude-code, protocol-extension]
status: complete
last_updated: 2025-08-28
last_updated_by: Nathan
last_updated_note: "Added detailed findings from agent-client-protocol and claude-code-acp repositories"
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

**Core Structure** (`agent-client-protocol/rust/`):

- JSON-RPC based bidirectional communication via symmetric `Agent`/`Client` traits
- Type-safe request/response enums with `#[serde(untagged)]` routing
- Capability negotiation via `AgentCapabilities` and `ClientCapabilities`
- Auto-generated JSON Schema from Rust types via `JsonSchema` derives

**Agent Trait Methods** (`agent.rs:18-108`):
- `initialize()` - Connection establishment and capability negotiation
- `authenticate()` - Authentication using advertised methods
- `new_session()` - Creates conversation contexts
- `load_session()` - Loads existing sessions (capability-gated)
- `prompt()` - Processes user prompts with full lifecycle
- `cancel()` - Cancels ongoing operations

**Client Trait Methods** (`client.rs:19-114`):
- `request_permission()` - Requests user permission for tool calls
- `write_text_file()` / `read_text_file()` - File operations (capability-gated)
- `session_notification()` - Handles session updates from agent

**RPC Infrastructure Pattern**:

1. **Method Constants** - Define at lines `agent.rs:395-415` / `client.rs:451-485`
2. **Request/Response Structs** - With `#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]`
3. **Schema Annotations** - `#[schemars(extend("x-side" = "agent", "x-method" = "method_name"))]`
4. **Untagged Enums** - Add to `ClientRequest`/`AgentResponse` enums for message routing
5. **Trait Methods** - Add to `Agent`/`Client` traits with `impl Future` signatures
6. **Connection Methods** - Implement in `ClientSideConnection`/`AgentSideConnection`
7. **Message Handling** - Update `MessageHandler` implementations for dispatch

**Protocol Versioning** (`version.rs:4-20`):
- Current: V1 with backward compatibility
- Breaking changes require version bump
- Non-breaking additions use capability flags

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

### Claude Code ACP Adapter Implementation

**Architecture** (`claude-code-acp/src/`):

- `acp-agent.ts:51` - `ClaudeAcpAgent` class implementing complete ACP `Agent` interface
- `mcp-server.ts:9` - Internal MCP proxy server for file operations and permissions
- `tools.ts:22` - Tool format conversion between Claude SDK and ACP representations
- Session management with UUID tracking and Claude SDK `Query` objects

**Agent Interface Implementation** (`acp-agent.ts:51-218`):
- `initialize()` at line 63: Declares capabilities (image, embedded_context) and auth methods
- `newSession()` at line 84: Creates UUID sessions with MCP server integration
- `prompt()` at line 140: Main query execution using Claude SDK with real-time streaming
- `cancel()` at line 211: Properly handles session cancellation and cleanup

**Session Lifecycle** (`acp-agent.ts:84-134`):
1. Generate UUID session ID and create pushable input stream
2. Configure MCP servers from ACP request parameters
3. Start internal HTTP-based MCP proxy server on dynamic port
4. Initialize Claude SDK query with working directory, MCP servers, tool permissions
5. Enable `mcp__acp__read` while disabling direct file tools for security

**Query Execution Flow** (`acp-agent.ts:140-209`):
1. Convert ACP prompt to Claude format via `promptToClaude()` at line 237
2. Push user message to Claude SDK input stream
3. Iterate through Claude SDK responses with real-time streaming
4. Handle system, result, user, and assistant message types
5. Convert Claude messages to ACP format via `toAcpNotifications()` at line 312
6. Stream session updates back to ACP client

**MCP Proxy Architecture** (`mcp-server.ts:9-449`):
- **Internal HTTP Server**: Creates MCP server for Claude SDK integration
- **Tool Implementations**:
  - `read` (lines 19-94): Proxies to ACP client's `readTextFile()`
  - `write` (lines 96-149): Proxies to ACP client's `writeTextFile()`
  - `edit` (lines 152-239): Text replacement with line tracking
  - `multi-edit` (lines 241-318): Sequential edit operations
- **Permission Integration**: Routes tool permission requests through ACP client

**Current Command Support**:
- **No existing slash command infrastructure** - all interactions use standard prompt interface
- **No `.claude/commands` directory integration** currently implemented
- **Command detection would require preprocessing** before Claude SDK integration

## Code References

### Zed Integration Layer
- `crates/agent_ui/src/agent_panel.rs:24` - Main AgentPanel component
- `crates/agent_ui/src/acp/thread_view.rs:315` - AcpThreadView UI component
- `crates/agent_ui/src/acp/message_editor.rs` - Agent panel message input
- `crates/agent_servers/src/acp.rs:63-162` - ACP connection establishment
- `crates/acp_thread/src/acp_thread.rs:826` - ACP thread creation

### Agent Client Protocol
- `agent-client-protocol/rust/agent.rs:18-108` - Agent trait with 6 core methods
- `agent-client-protocol/rust/client.rs:19-114` - Client trait for bidirectional communication
- `agent-client-protocol/rust/acp.rs:120` - ClientSideConnection implementation
- `agent-client-protocol/rust/acp.rs:341` - AgentSideConnection implementation
- `agent-client-protocol/rust/rpc.rs:30-367` - RPC connection infrastructure
- `agent-client-protocol/rust/agent.rs:333-371` - AgentCapabilities and PromptCapabilities
- `agent-client-protocol/rust/agent.rs:423-432` - ClientRequest/AgentResponse enum routing
- `agent-client-protocol/rust/generate.rs:24-77` - JSON schema generation

### Claude Code ACP Adapter
- `claude-code-acp/src/acp-agent.ts:51-218` - ClaudeAcpAgent implementing Agent interface
- `claude-code-acp/src/mcp-server.ts:9-449` - Internal MCP proxy server
- `claude-code-acp/src/tools.ts:22-395` - Tool format conversion
- `claude-code-acp/src/utils.ts:7-75` - Stream processing utilities

### Command Infrastructure
- `.claude/commands/*.md` - Command definition files (markdown format)
- No existing slash command infrastructure in claude-code-acp currently

## Architecture Insights

**Agent Panel System**: Completely separate from assistant/text threads, uses ACP protocol for external agent communication with JSON-RPC over stdin/stdout, manages sessions with unique IDs, and provides real-time message streaming with UI updates.

**ACP Protocol**: Designed for extensibility with capability negotiation, type safety through Rust enums, symmetric bidirectional design, and JSON-RPC foundation. Handles tool calls, permissions, and session management.

**Command Definitions**: Human-readable markdown with programmatically parseable structure, consistent metadata patterns, and workflow automation framework stored in `.claude/commands/`.

**Integration Patterns**: Claude Code ACP adapter provides proven pattern for bridging protocols, MCP servers enable tool execution proxying, session management handles concurrent interactions. Agent panel needs new command integration separate from existing slash commands.

## Implementation Recommendations

### 1. Protocol Extension for Custom Commands

Add new RPC methods following exact ACP patterns in `agent-client-protocol/rust/`:

**Method Constants** (`agent.rs:395-415`):
```rust
pub const SESSION_LIST_COMMANDS: &str = "session/list_commands";
pub const SESSION_RUN_COMMAND: &str = "session/run_command";
```

**Request/Response Types** (after `agent.rs:371`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(extend("x-side" = "agent", "x-method" = "session/list_commands"))]
#[serde(rename_all = "camelCase")]
pub struct ListCommandsRequest {
    pub session_id: SessionId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(extend("x-side" = "agent", "x-method" = "session/list_commands"))]
#[serde(rename_all = "camelCase")]
pub struct ListCommandsResponse {
    pub commands: Vec<CommandInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub requires_argument: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(extend("x-side" = "agent", "x-method" = "session/run_command"))]
#[serde(rename_all = "camelCase")]
pub struct RunCommandRequest {
    pub session_id: SessionId,
    pub command: String,
    pub args: Option<String>,
}
```

**Trait Extension** (add to `Agent` trait after `cancel()` at line 107):
```rust
fn list_commands(
    &self,
    arguments: ListCommandsRequest,
) -> impl Future<Output = Result<ListCommandsResponse, Error>>;

fn run_command(
    &self,
    arguments: RunCommandRequest,
) -> impl Future<Output = Result<(), Error>>;
```

**Enum Routing** (add to `ClientRequest` at line 423 and `AgentResponse`):
```rust
ListCommandsRequest(ListCommandsRequest),
RunCommandRequest(RunCommandRequest),
```

**Capability Extension** (add to `PromptCapabilities` at line 358):
```rust
/// Agent supports custom slash commands via `list_commands` and `run_command`.
#[serde(default)]
pub supports_custom_commands: bool,
```

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

In Claude Code ACP adapter, extend `ClaudeAcpAgent` class at `claude-code-acp/src/acp-agent.ts:51`:

**Add Command Parser** (new module at `src/command-parser.ts`):
```typescript
export interface CommandInfo {
  name: string;
  description: string;
  requires_argument: boolean;
  content?: string;
}

export class CommandParser {
  private commandsDir: string;
  private cachedCommands?: CommandInfo[];

  constructor(cwd: string) {
    this.commandsDir = path.join(cwd, '.claude', 'commands');
  }

  async listCommands(): Promise<CommandInfo[]> {
    // Parse *.md files, extract H1 titles and descriptions
  }

  async getCommand(name: string): Promise<CommandInfo | null> {
    // Return specific command with full content for execution
  }
}
```

**Extend ClaudeAcpAgent** (add after line 218):
```typescript
private commandParser?: CommandParser;

// In constructor around line 60:
if (options.cwd && fs.existsSync(path.join(options.cwd, '.claude', 'commands'))) {
  this.commandParser = new CommandParser(options.cwd);
}

// Update initialize() around line 68 to advertise capability:
agent_capabilities: {
  prompt_capabilities: {
    image: true,
    audio: false,
    embedded_context: true,
    supports_custom_commands: !!this.commandParser,
  },
}

async listCommands(request: ListCommandsRequest): Promise<ListCommandsResponse> {
  if (!this.commandParser) return { commands: [] };

  const commands = await this.commandParser.listCommands();
  return {
    commands: commands.map(cmd => ({
      name: cmd.name,
      description: cmd.description,
      requires_argument: cmd.requires_argument,
    }))
  };
}

async runCommand(request: RunCommandRequest): Promise<void> {
  if (!this.commandParser) throw new Error('Commands not supported');

  const command = await this.commandParser.getCommand(request.command);
  if (!command) throw new Error(`Command not found: ${request.command}`);

  // Execute command via existing session mechanism
  const session = this.sessions.get(request.session_id);
  if (!session) throw new Error('Session not found');

  // Create system prompt from command content
  let prompt = command.content;
  if (command.requires_argument && request.args) {
    prompt += `\n\nArguments: ${request.args}`;
  }

  // Inject as system message and process via existing prompt flow
  session.input.push({ role: 'user', content: prompt });

  // Stream results back via existing session update mechanism
  // (handled automatically by query execution loop at line 150)
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

## Follow-up Research 2025-08-28 20:29:47 MDT

After gaining access to the actual `agent-client-protocol` and `claude-code-acp` repositories, I updated this research document with concrete implementation details:

### Key New Findings

**Agent Client Protocol Structure**: The protocol uses a symmetric `Agent`/`Client` trait design with `#[serde(untagged)]` enum routing, JSON schema generation, and explicit capability negotiation. Adding new RPC methods requires specific patterns for method constants, request/response structs, trait extensions, and enum routing.

**Claude Code ACP Adapter**: Implements the full Agent interface using Claude SDK integration via MCP proxy servers. Currently has no slash command infrastructure - all interactions use the standard prompt interface. Command detection would require preprocessing before Claude SDK integration.

**Implementation Requirements**: More complex than initially understood - requires protocol extension, trait implementations, enum routing updates, capability advertisement, and coordination between three repositories (zed, agent-client-protocol, claude-code-acp).

### Updated Implementation Approach

1. **Protocol Extension**: Add `session/list_commands` and `session/run_command` methods following exact ACP patterns
2. **Capability System**: Extend `PromptCapabilities` with `supports_custom_commands` flag
3. **Agent Implementation**: Create command parser module in claude-code-acp to read `.claude/commands/*.md` files
4. **UI Integration**: Use existing completion provider patterns in Zed's agent panel message editor

The research now includes specific file:line references and concrete code examples for all three layers of the implementation.

###### ORIGINAL CLAUDE PROMPT (2025-08-28 15:34:28 PDT):

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
