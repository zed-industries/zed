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

## Phase 1: ACP Protocol Extension

### Overview
Add new slash command RPC methods and capabilities to the agent-client-protocol crate, then integrate them into Zed's ACP connection layer.

### Changes Required:

#### 1. Protocol Types (External Crate)
**File**: `/Users/nathan/src/agent-client-protocol/rust/agent.rs`
**Changes**: Add new request/response types and trait methods

```rust
// Add around line 108 after the cancel method in the Agent trait
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

// Add around line 372 after PromptCapabilities
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
**Changes**: Implement new trait methods in AcpConnection

```rust
// Add around line 340 after existing method implementations
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
- [ ] Protocol crate compiles: `cd /Users/nathan/src/agent-client-protocol && cargo check`
- [ ] Protocol tests pass: `cd /Users/nathan/src/agent-client-protocol && cargo test`
- [ ] Schema generation works: `cd /Users/nathan/src/agent-client-protocol && cargo run --bin generate`
- [ ] Zed compiles successfully: `./script/clippy`
- [ ] No linting errors: `cargo clippy --package agent_servers --package acp_thread`

#### Manual Verification:
- [ ] New trait methods are properly defined in connection interface
- [ ] ACP connection implements new methods correctly
- [ ] Capability detection helper is available for UI layer
- [ ] JSON schema includes new request/response types

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
**Changes**: Extend `is_completion_trigger()` method

```rust
// Modify around line 763 to add slash command detection
pub fn is_completion_trigger(
    &self,
    buffer: &Entity<Buffer>,
    position: language::Anchor,
    text: &str,
    _trigger_in_words: bool,
    cx: &mut Context<Editor>,
) -> bool {
    // Existing @ mention logic...
    if let Some(_) = MentionCompletion::try_parse(&line, position.column) {
        return true;
    }
    
    // Add slash command detection
    if let Some(thread) = &self.thread {
        if thread.read(cx).supports_custom_commands(cx) {
            if let Some(_) = SlashCommandCompletion::try_parse(&line, position.column) {
                return true;
            }
        }
    }
    
    false
}
```

#### 3. Command Completion Generation
**File**: `crates/agent_ui/src/acp/completion_provider.rs`
**Changes**: Extend `completions()` method

```rust
// Add around line 700 in completions() method after mention handling
// Handle slash command completions
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

// Add new method around line 850
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
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone())?;
        let connection = thread.read_with(cx, |thread, _| thread.connection().clone())?;
        
        // Fetch commands from agent
        let commands = connection.list_commands(&session_id, cx).await?;
        
        // Filter commands matching typed prefix
        let matching_commands: Vec<_> = commands
            .into_iter()
            .filter(|cmd| cmd.name.starts_with(&completion.name))
            .collect();
        
        // Convert to completion responses
        let mut completions = Vec::new();
        for command in matching_commands {
            let new_text = format!("/{}", command.name);
            let completion = project::Completion {
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
                confirm: Some(Arc::new(SlashCommandConfirmation {
                    command: command.name,
                    requires_argument: command.requires_argument,
                    thread: thread.downgrade(),
                })),
                ..Default::default()
            };
            completions.push(completion);
        }
        
        Ok(vec![project::CompletionResponse {
            completions,
            is_incomplete: false,
        }])
    })
}
```

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
- [ ] Code compiles successfully: `./script/clippy`
- [ ] No linting errors: `cargo clippy --package agent_ui --package acp_thread`
- [ ] Type checking passes: `cargo check --package agent_ui --package acp_thread`

#### Manual Verification:
- [ ] Typing "/" in agent panel triggers completion when agent supports commands
- [ ] No "/" completion appears when agent doesn't support commands
- [ ] Command list fetched from agent via `list_commands()` RPC
- [ ] Command selection triggers `run_command()` RPC
- [ ] Menu dismisses properly on Escape or click-outside

---

## Phase 3: Agent Implementation Support

### Overview
Prepare the Claude Code ACP adapter to implement the new slash command RPC methods by adding command parsing and execution.

### Changes Required:

#### 1. Command Parsing Module
**File**: `claude-code-acp/src/command-parser.ts` (new file)
**Changes**: Add markdown command parser

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

#### 2. ACP Agent Method Implementation
**File**: `claude-code-acp/src/acp-agent.ts`
**Changes**: Add new RPC method handlers

```typescript
// Add import
import { CommandParser, CommandInfo } from './command-parser';

// Add to ClaudeAcpAgent class around line 50
private commandParser?: CommandParser;

// Modify constructor around line 100
constructor(options: ClaudeAcpAgentOptions) {
  // ... existing initialization
  
  // Initialize command parser if .claude/commands exists
  if (options.cwd) {
    this.commandParser = new CommandParser(options.cwd);
  }
}

// Add capability declaration in initialize() around line 150
async initialize(request: InitializeRequest): Promise<InitializeResponse> {
  return {
    protocol_version: VERSION,
    agent_capabilities: {
      prompt_capabilities: {
        image: true,
        audio: false,
        embedded_context: true,
        supports_custom_commands: !!this.commandParser, // Enable if commands exist
      },
    },
    auth_methods: ['claude-code'],
  };
}

// Add new RPC method handlers around line 400
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

async runCommand(request: RunCommandRequest): Promise<void> {
  if (!this.commandParser) {
    throw new Error('Commands not supported');
  }

  const command = await this.commandParser.getCommand(request.command);
  if (!command) {
    throw new Error(`Command not found: ${request.command}`);
  }

  // Execute command by sending its content as a system prompt to Claude SDK
  const session = this.sessions.get(request.session_id);
  if (!session) {
    throw new Error('Session not found');
  }

  try {
    let systemPrompt = command.content;
    
    // If command requires arguments and args provided, append them
    if (command.requires_argument && request.args) {
      systemPrompt += `\n\nArguments: ${request.args}`;
    }

    // Create new query with command content as system prompt
    const query = query({
      prompt: systemPrompt,
      options: {
        cwd: session.cwd,
        mcpServers: session.mcpServers,
        allowedTools: session.allowedTools,
        disallowedTools: session.disallowedTools,
      },
    });

    // Stream results back to session
    for await (const chunk of query) {
      // Convert query response to session update format
      const update = this.convertQueryChunkToSessionUpdate(chunk);
      await this.sendSessionUpdate(request.session_id, update);
    }

  } catch (error) {
    console.error('Command execution failed:', error);
    // Send error as session update
    await this.sendSessionUpdate(request.session_id, {
      type: 'agent_message_chunk',
      content: {
        type: 'text',
        text: `Error executing command: ${error.message}`,
      },
    });
  }
}
```

### Success Criteria:

#### Automated Verification:
- [ ] TypeScript compilation passes: `npm run typecheck` (in claude-code-acp)
- [ ] ESLint passes: `npm run lint` (in claude-code-acp)
- [ ] Command parser unit tests pass: `npm test command-parser` 

#### Manual Verification:
- [ ] `.claude/commands/*.md` files are correctly parsed for metadata
- [ ] `list_commands()` returns available commands with descriptions
- [ ] `run_command()` executes command content via Claude SDK
- [ ] Command results stream back as session updates
- [ ] Error handling works for missing commands or execution failures

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

No migration needed - this is a new feature that gracefully degrades for agents that don't support custom commands. Existing agent panel behavior is preserved.

## References

- Original research: `thoughts/shared/research/2025-08-28_15-34-28_custom-slash-commands-acp.md`
- Text thread slash command picker: `crates/agent_ui/src/slash_command_picker.rs:54-348`
- ACP completion provider: `crates/agent_ui/src/acp/completion_provider.rs:763`
- Agent capability negotiation: `crates/agent_servers/src/acp.rs:131-156`