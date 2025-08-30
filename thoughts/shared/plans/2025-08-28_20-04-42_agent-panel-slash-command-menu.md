# Agent Panel Slash Command Menu Implementation Plan

## Overview

Add a searchable "/" command menu to the agent panel that appears when users type "/" - but only if the connected ACP agent supports custom slash commands. This enables users to discover and execute commands from `.claude/commands/` directory through a clean UI interface.

## Current State Analysis

**Agent Panel Message Editor**: Uses sophisticated completion system for "@" mentions with `PopoverMenu + Picker` pattern. Has existing slash command detection (`parse_slash_command()`) but only for highlighting/prevention, not menus.

**ACP Capability System**: Well-established capability negotiation during `initialize()` with `PromptCapabilities`. UI adapts reactively via `AcpThreadEvent::PromptCapabilitiesUpdated`.

**UI Patterns**: Perfect existing patterns in text thread slash command picker using `PopoverMenu + Picker` with `SlashCommandSelector` and `PickerDelegate` traits.

**ACP Protocol**: Clear extension patterns via external `agent-client-protocol` crate with request/response enums and method dispatch.

### Key Discoveries:
- `crates/agent_ui/src/acp/message_editor.rs:1573` - Existing slash detection foundation
- `crates/agent_ui/src/acp/completion_provider.rs:763` - Pattern for "@" completion triggers  
- `crates/agent_ui/src/slash_command_picker.rs:54` - Exact UI pattern we need to follow
- `crates/agent_servers/src/acp.rs:152` - Capability storage and distribution

## Desired End State

When user types "/" in agent panel message editor:
- **Agent supports commands**: Searchable menu appears with commands from `.claude/commands/`
- **Agent doesn't support commands**: No menu appears (preserves current behavior)
- **Command execution**: Selected commands execute via ACP protocol, stream results to thread view
- **Keyboard navigation**: Arrow keys, Enter to select, Escape to dismiss

### Verification:
- Menu appears only when `supports_custom_commands` capability is true
- Commands populated from ACP `list_commands()` RPC call
- Selected commands execute via ACP `run_command()` RPC call
- Results stream back as `SessionUpdate` notifications

## What We're NOT Doing

- NOT modifying existing assistant/text thread slash commands
- NOT implementing command parsing/execution logic in Zed (that's agent-side)
- NOT adding command discovery beyond what agents provide
- NOT changing the UI for agents that don't support custom commands

## Implementation Approach

Follow the existing "@" mention completion pattern but trigger on "/" instead. Use capability negotiation to control menu visibility. Extend ACP integration to call new RPC methods when available.

## Repository Dependencies & PR Strategy

### **Multi-Repository Architecture**
This feature spans three repositories that must be coordinated:

1. **`agent-client-protocol`**: External crate defining the protocol
2. **`zed`**: Main editor with ACP client integration  
3. **`claude-code-acp`**: Reference agent implementation

### **Dependency Chain**
```
agent-client-protocol (Phase 1) 
    ↓ 
zed (Phase 2) - temporarily depends on local ACP changes
    ↓
claude-code-acp (Phase 3) - uses published ACP version
```

### **Development Workflow**

#### Step 1: Local Development Setup
```bash
# Work on ACP protocol extension locally
cd /Users/nathan/src/agent-client-protocol
# Make Phase 1 changes...

# Point Zed to local ACP version for testing
cd /Users/nathan/src/zed
# Update Cargo.toml to reference local path:
# agent-client-protocol = { path = "../agent-client-protocol" }
```

#### Step 2: Testing & Validation
- Test all phases end-to-end with local dependencies
- Verify Phase 1+2 integration works correctly
- Validate Phase 3 against local ACP changes

#### Step 3: PR Sequence
1. **First PR**: `agent-client-protocol` with new slash command methods
2. **Second PR**: `zed` referencing published ACP version (after #1 merges)
3. **Third PR**: `claude-code-acp` using new ACP capabilities

### **Temporary Dependency Management**
During development, Zed's `Cargo.toml` will need:
```toml
[dependencies]
# Temporary local reference for development/testing
agent-client-protocol = { path = "../agent-client-protocol" }

# After ACP PR merges, switch to:  
agent-client-protocol = "0.2.0-alpha.1"  # or appropriate version
```

### **Cross-Repository Verification**
Before opening PRs:
- [x] ACP protocol extension compiles and tests pass
- [x] Zed compiles against local ACP changes
- [ ] End-to-end slash command flow works locally
- [ ] Claude ACP adapter works with generated types

## Phase 1: ACP Protocol Extension

### Overview
Add new slash command RPC methods and capabilities to the agent-client-protocol crate, then integrate them into Zed's ACP connection layer.

### Changes Required:

#### 1. Protocol Types (External Crate)
**File**: `/Users/nathan/src/agent-client-protocol/rust/agent.rs`
**Changes**: Add new request/response types and trait methods following exact ACP patterns

**Step 1: Add Method Constants** (after line 415):
```rust
/// Method name for listing custom commands in a session.
pub const SESSION_LIST_COMMANDS: &str = "session/list_commands";
/// Method name for running a custom command in a session.  
pub const SESSION_RUN_COMMAND: &str = "session/run_command";
```

**Step 2: Add Request/Response Structs** (after PromptCapabilities at line 371):
```rust
/// Request parameters for listing available commands.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(extend("x-side" = "agent", "x-method" = "session/list_commands"))]
#[serde(rename_all = "camelCase")]
pub struct ListCommandsRequest {
    /// The session ID to list commands for.
    pub session_id: SessionId,
}

/// Response containing available commands.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(extend("x-side" = "agent", "x-method" = "session/list_commands"))]
#[serde(rename_all = "camelCase")]
pub struct ListCommandsResponse {
    /// List of available commands.
    pub commands: Vec<CommandInfo>,
}

/// Information about a custom command.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommandInfo {
    /// Command name (e.g., "create_plan", "research_codebase").
    pub name: String,
    /// Human-readable description of what the command does.
    pub description: String,
    /// Whether this command requires arguments from the user.
    pub requires_argument: bool,
}

/// Request parameters for executing a command.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(extend("x-side" = "agent", "x-method" = "session/run_command"))]
#[serde(rename_all = "camelCase")]
pub struct RunCommandRequest {
    /// The session ID to execute the command in.
    pub session_id: SessionId,
    /// Name of the command to execute.
    pub command: String,
    /// Optional arguments for the command.
    pub args: Option<String>,
}
```

**Step 3: Add Agent Trait Methods** (after `cancel()` method at line 107):
```rust
/// Lists available custom commands for a session.
///
/// Returns all commands available in the agent's `.claude/commands` directory
/// or equivalent command registry. Commands can be executed via `run_command`.
fn list_commands(
    &self,
    arguments: ListCommandsRequest,
) -> impl Future<Output = Result<ListCommandsResponse, Error>>;

/// Executes a custom command within a session.
///
/// Runs the specified command with optional arguments. The agent should
/// stream results back via session update notifications.
fn run_command(
    &self,
    arguments: RunCommandRequest,
) -> impl Future<Output = Result<(), Error>>;
```

**Step 4: Add Enum Routing Variants** (to `ClientRequest` enum around line 423):
```rust
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ClientRequest {
    InitializeRequest(InitializeRequest),
    AuthenticateRequest(AuthenticateRequest),
    NewSessionRequest(NewSessionRequest),
    LoadSessionRequest(LoadSessionRequest),
    PromptRequest(PromptRequest),
    ListCommandsRequest(ListCommandsRequest),  // ADD THIS
    RunCommandRequest(RunCommandRequest),      // ADD THIS
}
```

**Step 5: Add AgentResponse Enum Variant** (find AgentResponse enum):
```rust
ListCommandsResponse(ListCommandsResponse),  // ADD THIS
```

#### 2. Capability Extension
**File**: `/Users/nathan/src/agent-client-protocol/rust/agent.rs`
**Changes**: Extend PromptCapabilities with custom command support

```rust
// Modify PromptCapabilities struct around line 358
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PromptCapabilities {
    /// Agent supports [`ContentBlock::Image`].
    #[serde(default)]
    pub image: bool,
    /// Agent supports [`ContentBlock::Audio`].
    #[serde(default)]
    pub audio: bool,
    /// Agent supports embedded context in `session/prompt` requests.
    #[serde(default)]
    pub embedded_context: bool,
    /// Agent supports custom slash commands via `list_commands` and `run_command`.
    #[serde(default)]
    pub supports_custom_commands: bool,
}
```

#### 3. AgentConnection Trait Extension
**File**: `crates/acp_thread/src/connection.rs`
**Changes**: Add new methods to trait definition

```rust
// Add these methods to AgentConnection trait around line 80
fn list_commands(&self, session_id: &acp::SessionId, cx: &mut App) -> Task<Result<acp::ListCommandsResponse>>;
fn run_command(&self, request: acp::RunCommandRequest, cx: &mut App) -> Task<Result<()>>;
```

#### 4. ACP Connection Implementation  
**File**: `crates/agent_servers/src/acp.rs`
**Changes**: Implement new trait methods in AcpConnection following existing patterns

**Step 1: Add ClientSideConnection Methods** (after existing methods around line 340):
```rust
impl acp::ClientSideConnection {
    /// Lists available custom commands for a session.
    pub async fn list_commands(
        &self,
        request: acp::ListCommandsRequest,
    ) -> Result<acp::ListCommandsResponse, acp::Error> {
        self.connection
            .request(acp::ClientRequest::ListCommandsRequest(request))
            .await
            .and_then(|response| match response {
                acp::AgentResponse::ListCommandsResponse(response) => Ok(response),
                _ => Err(acp::Error::internal_error("Invalid response type")),
            })
    }

    /// Executes a custom command in a session.
    pub async fn run_command(
        &self,
        request: acp::RunCommandRequest,
    ) -> Result<(), acp::Error> {
        self.connection
            .request(acp::ClientRequest::RunCommandRequest(request))
            .await
            .map(|_| ())
    }
}
```

**Step 2: Implement AgentConnection Trait Methods** (for AcpConnection struct):
```rust
fn list_commands(&self, session_id: &acp::SessionId, cx: &mut App) -> Task<Result<acp::ListCommandsResponse>> {
    let conn = self.connection.clone();
    let session_id = session_id.clone();
    cx.foreground_executor().spawn(async move {
        conn.list_commands(acp::ListCommandsRequest { session_id }).await
    })
}

fn run_command(&self, request: acp::RunCommandRequest, cx: &mut App) -> Task<Result<()>> {
    let conn = self.connection.clone();
    cx.foreground_executor().spawn(async move {
        conn.run_command(request).await
    })
}
```

**Step 3: Update Message Dispatch Logic** (in `AgentSide::decode_request()` around line 493):
```rust
// Add cases to the match statement:
acp::SESSION_LIST_COMMANDS => {
    if let Ok(request) = serde_json::from_value::<acp::ListCommandsRequest>(params) {
        Ok(acp::ClientRequest::ListCommandsRequest(request))
    } else {
        Err(acp::Error::invalid_params("Invalid list_commands parameters"))
    }
}
acp::SESSION_RUN_COMMAND => {
    if let Ok(request) = serde_json::from_value::<acp::RunCommandRequest>(params) {
        Ok(acp::ClientRequest::RunCommandRequest(request))
    } else {
        Err(acp::Error::invalid_params("Invalid run_command parameters"))
    }
}
```

#### 5. Capability Detection
**File**: `crates/acp_thread/src/acp_thread.rs`
**Changes**: Add capability checking helper

```rust
// Add around line 280 after other capability methods
pub fn supports_custom_commands(&self, cx: &App) -> bool {
    self.prompt_capabilities.get(cx).supports_custom_commands
}
```

### Success Criteria:

#### Automated Verification:
- [x] Protocol crate compiles: `cd /Users/nathan/src/agent-client-protocol && cargo check`
- [x] Protocol tests pass: `cd /Users/nathan/src/agent-client-protocol && cargo test`
- [ ] Schema generation works: `cd /Users/nathan/src/agent-client-protocol && cargo run --bin generate`
- [ ] Schema includes new methods: `grep -A5 -B5 "session/list_commands\|session/run_command" /Users/nathan/src/agent-client-protocol/schema/schema.json`
- [x] Zed compiles successfully: `./script/clippy`
- [x] No linting errors: `cargo clippy --package agent_servers --package acp_thread`
- [x] ACP thread capability method compiles: `cargo check --package acp_thread`

#### Manual Verification:
- [x] New trait methods are properly defined in Agent trait (`/Users/nathan/src/agent-client-protocol/rust/agent.rs`)
  - Verify `list_commands()` method signature at line ~115
  - Verify `run_command()` method signature at line ~127
- [x] Request/response enums updated in ClientRequest (`agent.rs:~423`) and AgentResponse enums
- [x] Method constants added (`SESSION_LIST_COMMANDS`, `SESSION_RUN_COMMAND`) after line 415
- [x] PromptCapabilities extended with `supports_custom_commands: bool` field
- [x] ClientSideConnection methods implemented with proper error handling
- [ ] Message dispatch logic updated in `AgentSide::decode_request()`
- [x] AgentConnection trait extends with new methods (`crates/acp_thread/src/connection.rs`)
- [x] AcpConnection implements trait methods (`crates/agent_servers/src/acp.rs`)
- [x] AcpThread has `supports_custom_commands()` helper method

---

## Phase 2: Slash Command Menu UI

### Overview
Add "/" detection and command menu to the ACP message editor, following the existing "@" completion pattern.

### Changes Required:

#### 1. Command Info Types
**File**: `crates/agent_ui/src/acp/completion_provider.rs`
**Changes**: Add command completion types

```rust
// Add around line 50 after existing completion types
#[derive(Debug, Clone)]
pub struct SlashCommandCompletion {
    pub name: String,
    pub description: String,
    pub requires_argument: bool,
    pub source_range: Range<usize>,
    pub command_range: Range<usize>,
}

impl SlashCommandCompletion {
    fn try_parse(line: &str, cursor_offset: usize) -> Option<Self> {
        // Parse "/" followed by optional command name
        if let Some(remainder) = line.strip_prefix('/') {
            let mut chars = remainder.char_indices().peekable();
            let mut command_end = 0;
            
            // Find end of command name (alphanumeric + underscore)
            while let Some((i, ch)) = chars.next() {
                if ch.is_alphanumeric() || ch == '_' {
                    command_end = i + ch.len_utf8();
                } else {
                    break;
                }
            }
            
            Some(SlashCommandCompletion {
                name: remainder[..command_end].to_string(),
                description: String::new(),
                requires_argument: false,
                source_range: 0..cursor_offset,
                command_range: 1..command_end + 1, // Skip the "/"
            })
        } else {
            None
        }
    }
}
```

#### 2. Completion Trigger Detection
**File**: `crates/agent_ui/src/acp/completion_provider.rs`
**Changes**: Extend `is_completion_trigger()` method following existing patterns

**Current Pattern Analysis**: The completion provider implements the `CompletionProvider` trait and integrates with the editor's completion system. The `is_completion_trigger()` method at line 763 currently only handles "@" mentions.

```rust
// Modify the existing is_completion_trigger() method around line 763
impl CompletionProvider for ContextPickerCompletionProvider {
    fn is_completion_trigger(
        &self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        text: &str,
        _trigger_in_words: bool,
        cx: &mut Context<Editor>,
    ) -> bool {
        let buffer = buffer.read(cx);
        let position = position.to_point(&buffer);
        let line_start = Point::new(position.row, 0);
        let mut lines = buffer.text_for_range(line_start..position).lines();
        let Some(line) = lines.next() else {
            return false;
        };

        // Existing @ mention logic - KEEP THIS
        if let Some(_) = MentionCompletion::try_parse(&line, position.column) {
            return true;
        }
        
        // ADD: Slash command detection (only if agent supports commands)
        if let Some(thread) = &self.thread {
            if thread.read(cx).supports_custom_commands(cx) {
                if let Some(_) = SlashCommandCompletion::try_parse(&line, position.column) {
                    return true;
                }
            }
        }
        
        false
    }
}
```

**Pattern Notes**: 
- Integrates with existing `@` mention system without conflicts
- Only triggers when agent capability `supports_custom_commands` is true
- Uses same line parsing approach as existing mention system
- Maintains backward compatibility

#### 3. Command Completion Generation
**File**: `crates/agent_ui/src/acp/completion_provider.rs`
**Changes**: Extend `completions()` method using existing async patterns

**Current Pattern Analysis**: The completion provider's `completions()` method at line 639 returns `Task<Result<Vec<project::CompletionResponse>>>` and uses `cx.spawn()` for async operations. It handles different completion types via pattern matching.

```rust
// Modify the existing completions() method around line 700
// ADD this after the existing mention completion logic:

// Handle slash command completions (only if agent supports them)
if let Some(thread) = &self.thread {
    if thread.read(cx).supports_custom_commands(cx) {
        if let Some(slash_completion) = SlashCommandCompletion::try_parse(&line, cursor_offset) {
            return self.complete_slash_commands(
                slash_completion,
                buffer.clone(),
                cursor_anchor,
                cx,
            );
        }
    }
}

// ADD new method following existing async patterns (around line 850):
fn complete_slash_commands(
    &self,
    completion: SlashCommandCompletion,
    buffer: Entity<Buffer>,
    cursor_anchor: language::Anchor,
    cx: &mut Context<Editor>,
) -> Task<Result<Vec<project::CompletionResponse>>> {
    let Some(thread) = self.thread.clone() else {
        return Task::ready(Ok(Vec::new()));
    };
    
    cx.spawn(async move |cx| {
        // Get session info using existing patterns
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone())?;
        let connection = thread.read_with(cx, |thread, _| thread.connection().clone())?;
        
        // Fetch commands from agent via new ACP method
        let response = connection.list_commands(&session_id, cx).await?;
        
        // Filter commands matching typed prefix (fuzzy matching like mentions)
        let matching_commands: Vec<_> = response.commands
            .into_iter()
            .filter(|cmd| {
                // Support both prefix matching and fuzzy matching
                cmd.name.starts_with(&completion.name) ||
                cmd.name.to_lowercase().contains(&completion.name.to_lowercase())
            })
            .collect();
        
        // Convert to project::Completion following existing patterns
        let mut completions = Vec::new();
        for command in matching_commands {
            let new_text = format!("/{}", command.name);
            let completion_item = project::Completion {
                old_range: completion.source_range.clone(),
                new_text,
                label: command.name.clone().into(),
                server_id: language::LanguageServerId(0), // Not from language server
                kind: Some(language::CompletionKind::Function),
                documentation: if !command.description.is_empty() {
                    Some(language::Documentation::SingleLine(command.description.clone()))
                } else {
                    None
                },
                // Custom confirmation handler for command execution
                confirm: Some(Arc::new(SlashCommandConfirmation {
                    command: command.name,
                    requires_argument: command.requires_argument,
                    thread: thread.downgrade(),
                })),
                ..Default::default()
            };
            completions.push(completion_item);
        }
        
        // Return single completion response (like existing mentions)
        Ok(vec![project::CompletionResponse {
            completions,
            is_incomplete: false,
        }])
    })
}
```

**Integration Notes**:
- Follows same async pattern as existing mention completions at line 639
- Uses `thread.read_with()` pattern for safe entity access
- Implements fuzzy matching similar to existing completion types
- Returns single `CompletionResponse` following established patterns
- Integrates custom confirmation handler via `confirm` field

#### 4. Command Confirmation Handler
**File**: `crates/agent_ui/src/acp/completion_provider.rs`
**Changes**: Add confirmation handler for slash commands

```rust
// Add around line 950
#[derive(Debug)]
struct SlashCommandConfirmation {
    command: String,
    requires_argument: bool,
    thread: WeakEntity<AcpThread>,
}

impl language::CompletionConfirm for SlashCommandConfirmation {
    fn confirm(
        &self,
        completion: &project::Completion,
        buffer: &mut Buffer,
        mut cursor_positions: Vec<language::Anchor>,
        trigger_text: &str,
        _workspace: Option<&Workspace>,
        window: &mut Window,
        cx: &mut Context<Buffer>,
    ) -> Option<Task<Result<Vec<language::Anchor>>>> {
        if self.requires_argument {
            // Keep cursor after command name for argument input
            return None; // Let default behavior handle text insertion
        }
        
        // Execute command immediately
        let Some(thread) = self.thread.upgrade() else {
            return None;
        };
        
        let command = self.command.clone();
        let task = cx.spawn(async move |cx| {
            thread
                .update(cx, |thread, cx| {
                    thread.run_command(command, None, cx)
                })
                .ok();
            Ok(cursor_positions)
        });
        
        Some(task)
    }
}
```

#### 5. Command Execution Method
**File**: `crates/acp_thread/src/acp_thread.rs`
**Changes**: Add command execution method

```rust
// Add around line 450 after other public methods
pub fn run_command(
    &mut self,
    command: String,
    args: Option<String>,
    cx: &mut Context<Self>,
) -> Task<Result<()>> {
    let session_id = self.session_id.clone();
    let connection = self.connection.clone();
    
    cx.spawn(async move |this, cx| {
        let request = acp::RunCommandRequest {
            session_id,
            command,
            args,
        };
        
        connection.run_command(request, cx).await?;
        
        // The agent will send back results via SessionUpdate notifications
        // which will be handled by existing handle_session_update() logic
        Ok(())
    })
}
```

### Success Criteria:

#### Automated Verification:
- [x] Code compiles successfully: `./script/clippy`
- [x] No linting errors: `cargo clippy --package agent_ui --package acp_thread`
- [x] Type checking passes: `cargo check --package agent_ui --package acp_thread`
- [x] Completion provider compiles: `cargo check --package agent_ui --lib`
- [ ] Slash command parsing works: Test `SlashCommandCompletion::try_parse()` with various inputs

#### Manual Verification (REVISED - Simpler Approach):
- [x] **Refactored to Simpler Architecture**: 
  - [x] Remove complex `CompositeCompletionProvider` and `AgentSlashCommandCompletionProvider`
  - [x] Extend existing `ContextPickerCompletionProvider` with optional thread field
  - [x] Add `set_thread()` method for lifecycle management
  - [ ] Add slash command detection to `is_completion_trigger()`
  - [ ] Add slash command completion to `completions()` method
- [ ] **Slash Command Integration**: 
  - [ ] Parse slash commands using existing `SlashCommandLine` from assistant_slash_command
  - [ ] Fetch commands via ACP `list_commands()` RPC when thread supports it
  - [ ] Execute commands via ACP `run_command()` RPC with proper confirmation
  - [ ] Only show slash completions when `supports_custom_commands = true`
- [x] **MessageEditor Integration**:
  - [x] Add `set_thread()` method to update completion provider when thread is ready
  - [ ] Call `set_thread()` in ThreadView when thread transitions to Ready state
- [ ] Integration Testing:
  - [ ] Typing "/" in agent panel triggers completion when `supports_custom_commands = true`
  - [ ] No "/" completion appears when `supports_custom_commands = false`
  - [ ] Command list fetched from agent via `list_commands()` RPC call
  - [ ] Command selection triggers `run_command()` RPC call
  - [ ] Menu shows command descriptions from agent
  - [ ] Fuzzy matching works (typing "/cr" shows "create_plan")
  - [ ] Menu dismisses properly on Escape or click-outside
  - [ ] Commands execute and stream results back to thread view

---

## Phase 3: Agent Implementation Support

### Overview
Prepare the Claude Code ACP adapter to implement the new slash command RPC methods by adding command parsing and execution.

### **CRITICAL ARCHITECTURE NOTE** 
The TypeScript types in `claude-code-acp` are **automatically generated** from the Rust protocol definitions. The ACP repository uses a code generation pipeline:

1. **Rust → JSON Schema**: `cargo run --bin generate` creates `schema/schema.json` from Rust types
2. **JSON Schema → TypeScript**: `node typescript/generate.js` creates TypeScript types from the schema

**This means the new `ListCommandsRequest`, `RunCommandRequest`, etc. types will be automatically available in TypeScript after we extend the Rust protocol in Phase 1.**

### Changes Required:

#### 1. Command Parsing Module
**File**: `claude-code-acp/src/command-parser.ts` (new file)
**Changes**: Add markdown command parser (TypeScript types will be auto-generated from Phase 1)

```typescript
import * as fs from 'fs';
import * as path from 'path';

export interface CommandInfo {
  name: string;
  description: string;
  requires_argument: boolean;
  content?: string; // Full command content for execution
}

export class CommandParser {
  private commandsDir: string;
  private cachedCommands?: CommandInfo[];

  constructor(cwd: string) {
    this.commandsDir = path.join(cwd, '.claude', 'commands');
  }

  async listCommands(): Promise<CommandInfo[]> {
    if (this.cachedCommands) {
      return this.cachedCommands;
    }

    try {
      if (!fs.existsSync(this.commandsDir)) {
        return [];
      }

      const files = fs.readdirSync(this.commandsDir)
        .filter(file => file.endsWith('.md'));

      const commands: CommandInfo[] = [];
      for (const file of files) {
        const filePath = path.join(this.commandsDir, file);
        const content = fs.readFileSync(filePath, 'utf-8');
        const commandInfo = this.parseCommandFile(content, file);
        if (commandInfo) {
          commands.push(commandInfo);
        }
      }

      this.cachedCommands = commands;
      return commands;
    } catch (error) {
      console.error('Failed to list commands:', error);
      return [];
    }
  }

  private parseCommandFile(content: string, filename: string): CommandInfo | null {
    const lines = content.split('\n');
    let name = '';
    let description = '';
    let requires_argument = false;

    // Extract command name from H1 title
    const titleMatch = lines.find(line => line.startsWith('# '));
    if (titleMatch) {
      name = titleMatch.replace('# ', '').trim().toLowerCase().replace(/\s+/g, '_');
    } else {
      // Fall back to filename without extension
      name = path.basename(filename, '.md');
    }

    // Extract description (text after H1, before first H2)
    const titleIndex = lines.findIndex(line => line.startsWith('# '));
    if (titleIndex >= 0) {
      const nextHeaderIndex = lines.findIndex((line, i) => 
        i > titleIndex && line.startsWith('## '));
      const endIndex = nextHeaderIndex >= 0 ? nextHeaderIndex : lines.length;
      
      description = lines
        .slice(titleIndex + 1, endIndex)
        .join('\n')
        .trim()
        .split('\n')[0] || ''; // First non-empty line as description
    }

    // Check if command requires arguments (heuristic)
    requires_argument = content.includes('arguments') || 
                      content.includes('parameter') ||
                      content.includes('[arg]') ||
                      content.includes('{arg}');

    return {
      name,
      description,
      requires_argument,
      content
    };
  }

  async getCommand(name: string): Promise<CommandInfo | null> {
    const commands = await this.listCommands();
    return commands.find(cmd => cmd.name === name) || null;
  }

  // Clear cache when commands directory changes
  invalidateCache(): void {
    this.cachedCommands = undefined;
  }
}
```

#### 2. Regenerate TypeScript Types
**Prerequisites**: After completing Phase 1 Rust protocol extension
**Commands**: Generate TypeScript types from updated Rust definitions

```bash
# From agent-client-protocol repository root:
cd /Users/nathan/src/agent-client-protocol
npm run generate
```

This will automatically create TypeScript types for:
- `ListCommandsRequest`
- `ListCommandsResponse` 
- `RunCommandRequest`
- `CommandInfo`
- Updated `PromptCapabilities` with `supports_custom_commands`

#### 3. ACP Agent Method Implementation
**File**: `claude-code-acp/src/acp-agent.ts`
**Changes**: Add new RPC method handlers following existing session management patterns

**Current Architecture Analysis**: The `ClaudeAcpAgent` at line 51 implements the ACP `Agent` interface with UUID-based session management. Sessions use Claude SDK `Query` objects with MCP proxy integration. The `prompt()` method at line 140 shows the pattern for query execution and result streaming.

```typescript
// Step 1: Add import for command parser and auto-generated ACP types
import { CommandParser, CommandInfo } from './command-parser';
import type { 
  ListCommandsRequest, 
  ListCommandsResponse, 
  RunCommandRequest 
} from '@zed-industries/agent-client-protocol';

// Step 2: Extend ClaudeAcpAgent class (add to class definition around line 51)
export class ClaudeAcpAgent implements Agent {
  private sessions: Map<string, Session> = new Map();
  private client: Client;
  private commandParser?: CommandParser;  // ADD THIS

  // Step 3: Modify constructor to initialize command parser (around line 60)
  constructor(
    client: Client,
    options: { cwd?: string } = {}
  ) {
    this.client = client;
    
    // Initialize command parser if .claude/commands directory exists
    if (options.cwd && fs.existsSync(path.join(options.cwd, '.claude', 'commands'))) {
      this.commandParser = new CommandParser(options.cwd);
    }
  }

  // Step 4: Update initialize() method to advertise capability (around line 68)
  async initialize(request: InitializeRequest): Promise<InitializeResponse> {
    return {
      protocol_version: VERSION,
      agent_capabilities: {
        prompt_capabilities: {
          image: true,
          audio: false,
          embedded_context: true,
          supports_custom_commands: !!this.commandParser, // Advertise support
        },
      },
      auth_methods: [/* existing auth methods */],
    };
  }

  // Step 5: Implement listCommands following existing async patterns (after line 218)
  async listCommands(request: ListCommandsRequest): Promise<ListCommandsResponse> {
    if (!this.commandParser) {
      return { commands: [] };
    }

    try {
      const commands = await this.commandParser.listCommands();
      return {
        commands: commands.map(cmd => ({
          name: cmd.name,
          description: cmd.description,
          requires_argument: cmd.requires_argument,
        }))
      };
    } catch (error) {
      console.error('Failed to list commands:', error);
      return { commands: [] };
    }
  }

  // Step 6: Implement runCommand integrating with existing session flow
  async runCommand(request: RunCommandRequest): Promise<void> {
    if (!this.commandParser) {
      throw new Error('Commands not supported');
    }

    const command = await this.commandParser.getCommand(request.command);
    if (!command) {
      throw new Error(`Command not found: ${request.command}`);
    }

    const session = this.sessions.get(request.session_id);
    if (!session) {
      throw new Error('Session not found');
    }

    try {
      // Build prompt from command content following existing patterns
      let commandPrompt = command.content;
      
      if (command.requires_argument && request.args) {
        commandPrompt += `\n\nArguments: ${request.args}`;
      }

      // Execute via existing session input stream (recommended approach)
      // This integrates with existing prompt() flow and MCP proxy
      session.input.push({
        role: 'user',
        content: commandPrompt
      });

      // Results will be streamed back via existing query execution loop
      // at line 150 in prompt() method, no additional streaming needed

    } catch (error) {
      console.error('Command execution failed:', error);
      // Send error via existing session update mechanism
      await this.client.sessionUpdate({
        session_id: request.session_id,
        type: 'agent_message_chunk',
        content: {
          type: 'text',
          text: `Error executing command: ${error.message}`,
        },
      });
    }
  }
}
```

**Integration Notes**:
- **Auto-Generated Types**: All ACP protocol types are automatically generated from Rust definitions
- **Session Reuse**: Uses existing session's input stream and MCP configuration
- **Result Streaming**: Leverages existing `prompt()` method's streaming loop at line 150
- **Error Handling**: Uses established session update patterns from line 191
- **Tool Access**: Commands inherit session's MCP server and tool configurations

### Success Criteria:

#### Automated Verification:
- [ ] **Prerequisites completed**: Phase 1 Rust protocol extension must be completed first
- [ ] **TypeScript types generated**: `cd /Users/nathan/src/agent-client-protocol && npm run generate`
- [ ] **Types available**: Verify new types exist in `agent-client-protocol/typescript/schema.ts`
- [ ] TypeScript compilation passes: `cd /Users/nathan/src/claude-code-acp && npm run typecheck`
- [ ] ESLint passes: `cd /Users/nathan/src/claude-code-acp && npm run lint`
- [ ] Agent compiles: `cd /Users/nathan/src/claude-code-acp && npm run build`
- [ ] Command parser unit tests pass: `cd /Users/nathan/src/claude-code-acp && npm test -- --testNamePattern="command-parser"`

#### Manual Verification:
- [ ] **Code Generation Pipeline**:
  - [ ] Rust protocol changes trigger successful schema generation: `cargo run --bin generate`
  - [ ] JSON schema contains new method definitions: `grep -A5 -B5 "session/list_commands\|session/run_command" /Users/nathan/src/agent-client-protocol/schema/schema.json`
  - [ ] TypeScript types generated correctly: Check for `ListCommandsRequest`, `RunCommandRequest` types in schema.ts
- [ ] CommandParser class implemented (`claude-code-acp/src/command-parser.ts`):
  - `listCommands()` method reads `.claude/commands/*.md` files
  - `parseCommandFile()` extracts H1 titles and descriptions correctly
  - `getCommand()` returns full command content for execution
  - Proper error handling for missing directories and files
  - Command caching works correctly
- [ ] ClaudeAcpAgent class extended (`claude-code-acp/src/acp-agent.ts`):
  - Constructor initializes `commandParser` when `.claude/commands` exists
  - `initialize()` method advertises `supports_custom_commands` capability correctly
  - `listCommands()` method implemented and returns properly formatted response
  - `runCommand()` method integrated with existing session management
  - Command execution uses existing session input stream 
  - Error handling streams errors back via session updates
- [ ] **Type Integration**:
  - [ ] Auto-generated types imported correctly from `@zed-industries/agent-client-protocol`
  - [ ] TypeScript compiler recognizes new protocol method signatures
  - [ ] No type errors when implementing new agent methods
- [ ] Integration Testing:
  - [ ] Agent advertises `supports_custom_commands = true` when `.claude/commands` directory exists
  - [ ] Agent advertises `supports_custom_commands = false` when directory doesn't exist
  - [ ] `list_commands()` RPC returns commands from `.claude/commands/*.md` files  
  - [ ] Commands include correct name, description, requires_argument fields
  - [ ] `run_command()` executes command content via Claude SDK integration
  - [ ] Command results stream back as session updates to ACP client
  - [ ] Commands have access to session's MCP servers and tool permissions
  - [ ] Error handling works for missing commands, directories, execution failures
  - [ ] Command arguments are properly appended when provided
- [ ] End-to-End Testing:
  - [ ] Create test `.claude/commands/test.md` file with sample command
  - [ ] Verify command appears in Zed's "/" completion menu
  - [ ] Verify command executes and streams results to agent panel
  - [ ] Verify commands work with existing MCP proxy and tool permissions

---

## Testing Strategy

### Unit Tests:
- Command parser correctly extracts name, description, and argument requirements
- Slash completion parsing handles various input formats
- Capability detection works with different agent configurations

### Integration Tests:  
- End-to-end slash command flow from "/" keystroke to command execution
- Menu appearance/dismissal based on agent capabilities
- Command completion filtering and selection

### Manual Testing Steps:
1. Connect to agent without custom command support → verify no "/" menu
2. Connect to agent with custom command support → verify "/" shows menu
3. Type "/cr" → verify "create_plan" command appears in filtered list
4. Select command with arguments → verify argument input continues
5. Select command without arguments → verify immediate execution
6. Press Escape during menu → verify menu dismisses
7. Click outside menu → verify menu dismisses

## Performance Considerations

- Command list caching in agent to avoid repeated filesystem reads
- Debounced completion triggers to avoid excessive RPC calls  
- Async command execution to prevent UI blocking
- Menu virtualization for large command lists (if needed)

## Migration Notes

### User Experience
No migration needed - this is a new feature that gracefully degrades for agents that don't support custom commands. Existing agent panel behavior is preserved.

### Developer Coordination
**Important**: This feature requires coordinated releases across multiple repositories:

1. **ACP Protocol**: Must be released first with new slash command methods
2. **Zed**: Can only merge after ACP release is available  
3. **Agent Implementations**: Can adopt new capabilities independently

### Version Compatibility
- **Backward Compatible**: Old agents continue working without slash command menus
- **Forward Compatible**: New Zed version works with old agents (feature simply disabled)
- **Graceful Degradation**: UI adapts based on agent-advertised capabilities

### Rollout Strategy
1. **Phase 1 Release**: ACP protocol extension (no visible user changes)
2. **Phase 2 Release**: Zed UI implementation (menu appears only with compatible agents)
3. **Phase 3+ Rollout**: Agent implementations adopt new capabilities over time

## References

- Original research: `thoughts/shared/research/2025-08-28_15-34-28_custom-slash-commands-acp.md`
- Text thread slash command picker: `crates/agent_ui/src/slash_command_picker.rs:54-348`
- ACP completion provider: `crates/agent_ui/src/acp/completion_provider.rs:763`
- Agent capability negotiation: `crates/agent_servers/src/acp.rs:131-156`