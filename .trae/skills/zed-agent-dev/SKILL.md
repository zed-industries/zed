---
name: "zed-agent-dev"
description: "Comprehensive guide for Zed's agent architecture. Covers ACP traits, GUI-agent communication, native agent implementation, and LLM request flow from initialization to completion."
---

# Zed Agent Development Guide

## Architecture Overview

Zed's agent system uses a layered architecture with three main layers:
1. **GUI Layer** (`agent_ui`) - User interface components
2. **Protocol Layer** (`acp_thread`) - ACP traits defining agent communication
3. **Implementation Layer** (`agent`, `agent_servers`) - Actual agent implementations

The same ACP traits are used for both:
- **Native Agent**: Zed's built-in agent using `language_model` crate
- **External Agents**: Claude Code, Codex, Gemini CLI, custom agents via stdio

## Crate Responsibilities

| Crate | Purpose |
|-------|---------|
| `acp_thread` | Core ACP traits (`AgentConnection`, `AgentModelSelector`, etc.) and `AcpThread` entity |
| `agent` | Native agent implementation (`NativeAgent`, `NativeAgentConnection`, `Thread`) |
| `agent_servers` | External agent connections (`AcpConnection`, Claude/Codex/Gemini servers) |
| `agent_ui` | GUI components (`AgentPanel`, `TextThreadEditor`, thread views) |
| `agent_client_protocol` | External crate defining ACP wire protocol types |
| `language_model` | LLM provider traits and registry |

## Core ACP Traits (GUI-Agent Interface)

All traits defined in `crates/acp_thread/src/connection.rs`.

### AgentConnection - Main Communication Trait

The central trait that GUI uses to interact with any agent backend:

```rust
pub trait AgentConnection {
    fn telemetry_id(&self) -> SharedString;

    fn new_session(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>>;

    fn supports_load_session(&self, cx: &App) -> bool;
    fn load_session(self: Rc<Self>, session: AgentSessionInfo, project: Entity<Project>, cwd: &Path, cx: &mut App) -> Task<Result<Entity<AcpThread>>>;
    fn supports_close_session(&self, cx: &App) -> bool;
    fn close_session(&self, session_id: &acp::SessionId, cx: &mut App) -> Task<Result<()>>;
    fn supports_resume_session(&self, cx: &App) -> bool;
    fn resume_session(self: Rc<Self>, session: AgentSessionInfo, project: Entity<Project>, cwd: &Path, cx: &mut App) -> Task<Result<Entity<AcpThread>>>;

    fn auth_methods(&self) -> &[acp::AuthMethod];
    fn authenticate(&self, method: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>>;

    fn prompt(
        &self,
        user_message_id: Option<UserMessageId>,
        params: acp::PromptRequest,
        cx: &mut App,
    ) -> Task<Result<acp::PromptResponse>>;

    fn retry(&self, session_id: &acp::SessionId, cx: &App) -> Option<Rc<dyn AgentSessionRetry>>;
    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App);
    fn truncate(&self, session_id: &acp::SessionId, cx: &App) -> Option<Rc<dyn AgentSessionTruncate>>;
    fn set_title(&self, session_id: &acp::SessionId, cx: &App) -> Option<Rc<dyn AgentSessionSetTitle>>;
    fn model_selector(&self, session_id: &acp::SessionId) -> Option<Rc<dyn AgentModelSelector>>;
    fn telemetry(&self) -> Option<Rc<dyn AgentTelemetry>>;
    fn session_modes(&self, session_id: &acp::SessionId, cx: &App) -> Option<Rc<dyn AgentSessionModes>>;
    fn session_config_options(&self, session_id: &acp::SessionId, cx: &App) -> Option<Rc<dyn AgentSessionConfigOptions>>;
    fn session_list(&self, cx: &mut App) -> Option<Rc<dyn AgentSessionList>>;
    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;
}
```

### Supporting Traits

```rust
pub trait AgentSessionTruncate {
    fn run(&self, message_id: UserMessageId, cx: &mut App) -> Task<Result<()>>;
}

pub trait AgentSessionRetry {
    fn run(&self, cx: &mut App) -> Task<Result<acp::PromptResponse>>;
}

pub trait AgentSessionSetTitle {
    fn run(&self, title: SharedString, cx: &mut App) -> Task<Result<()>>;
}

pub trait AgentTelemetry {
    fn thread_data(&self, session_id: &acp::SessionId, cx: &mut App) -> Task<Result<serde_json::Value>>;
}

pub trait AgentSessionModes {
    fn current_mode(&self) -> acp::SessionModeId;
    fn all_modes(&self) -> Vec<acp::SessionMode>;
    fn set_mode(&self, mode: acp::SessionModeId, cx: &mut App) -> Task<Result<()>>;
}

pub trait AgentSessionConfigOptions {
    fn config_options(&self) -> Vec<acp::SessionConfigOption>;
    fn set_config_option(&self, config_id: acp::SessionConfigId, value: acp::SessionConfigValueId, cx: &mut App) -> Task<Result<Vec<acp::SessionConfigOption>>>;
    fn watch(&self, cx: &mut App) -> Option<watch::Receiver<()>>;
}

pub trait AgentSessionList {
    fn list_sessions(&self, request: AgentSessionListRequest, cx: &mut App) -> Task<Result<AgentSessionListResponse>>;
    fn supports_delete(&self) -> bool;
    fn delete_session(&self, session_id: &acp::SessionId, cx: &mut App) -> Task<Result<()>>;
    fn delete_sessions(&self, cx: &mut App) -> Task<Result<()>>;
    fn watch(&self, cx: &mut App) -> Option<smol::channel::Receiver<SessionListUpdate>>;
    fn notify_refresh(&self);
    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;
}

pub trait AgentModelSelector: 'static {
    fn list_models(&self, cx: &mut App) -> Task<Result<AgentModelList>>;
    fn select_model(&self, model_id: acp::ModelId, cx: &mut App) -> Task<Result<()>>;
    fn selected_model(&self, cx: &mut App) -> Task<Result<AgentModelInfo>>;
    fn watch(&self, cx: &mut App) -> Option<watch::Receiver<()>>;
    fn should_render_footer(&self) -> bool;
}
```

### AgentServer Trait (External Agent Factory)

Defined in `crates/agent_servers/src/agent_servers.rs`:

```rust
pub trait AgentServer: Send {
    fn logo(&self) -> ui::IconName;
    fn name(&self) -> SharedString;
    fn connect(&self, root_dir: Option<&Path>, delegate: AgentServerDelegate, cx: &mut App) -> Task<Result<(Rc<dyn AgentConnection>, Option<task::SpawnInTerminal>)>>;
    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;
    fn default_mode(&self, cx: &App) -> Option<acp::SessionModeId>;
    fn set_default_mode(&self, mode_id: Option<acp::SessionModeId>, fs: Arc<dyn Fs>, cx: &mut App);
    fn default_model(&self, cx: &App) -> Option<acp::ModelId>;
    fn set_default_model(&self, model_id: Option<acp::ModelId>, fs: Arc<dyn Fs>, cx: &mut App);
    fn favorite_model_ids(&self, cx: &mut App) -> HashSet<acp::ModelId>;
    // ... config options methods
}
```

## Two AgentConnection Implementations

### 1. NativeAgentConnection (Built-in Zed Agent)

Located in `crates/agent/src/agent.rs`.

```
NativeAgentConnection
    └── wraps Entity<NativeAgent>
            └── manages HashMap<SessionId, Session>
                    └── Session { thread: Entity<Thread>, acp_thread: Entity<AcpThread> }
```

Key points:
- Wraps `Entity<NativeAgent>` which holds all sessions
- Each session has both `Thread` (internal message processing) and `AcpThread` (UI representation)
- `prompt()` calls `Thread::send()` which triggers LLM completion
- Events from Thread are forwarded to AcpThread via `handle_thread_events()`

### 2. AcpConnection (External Agents)

Located in `crates/agent_servers/src/acp.rs`.

```
AcpConnection
    └── communicates via acp::ClientSideConnection (JSON-RPC over stdio)
            └── spawns external process (claude, codex, gemini, custom)
```

Key points:
- Spawns external process and communicates via stdin/stdout
- Uses `agent_client_protocol` crate for JSON-RPC protocol
- Implements `acp::Client` trait for handling requests from external agent
- Session updates come via `session_notification()` and are forwarded to AcpThread

## Key Data Structures

### AcpThread (UI State)

Located in `crates/acp_thread/src/acp_thread.rs`:

```rust
pub struct AcpThread {
    parent_session_id: Option<acp::SessionId>,
    title: SharedString,
    entries: Vec<AgentThreadEntry>,
    plan: Plan,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    shared_buffers: HashMap<Entity<Buffer>, BufferSnapshot>,
    send_task: Option<Task<()>>,
    connection: Rc<dyn AgentConnection>,
    session_id: acp::SessionId,
    token_usage: Option<TokenUsage>,
    prompt_capabilities: acp::PromptCapabilities,
    terminals: HashMap<acp::TerminalId, Entity<Terminal>>,
    // ...
}

pub enum AgentThreadEntry {
    UserMessage(UserMessage),
    AssistantMessage(AssistantMessage),
    ToolCall(ToolCall),
}

pub enum AcpThreadEvent {
    NewEntry,
    TitleUpdated,
    TokenUsageUpdated,
    EntryUpdated(usize),
    EntriesRemoved(Range<usize>),
    ToolAuthorizationRequired,
    Retry(RetryStatus),
    SubagentSpawned(acp::SessionId),
    Stopped,
    Error,
    LoadError(LoadError),
    // ...
}
```

### Thread (Internal Agent State - Native Only)

Located in `crates/agent/src/thread.rs`:

```rust
pub struct Thread {
    id: acp::SessionId,
    prompt_id: PromptId,
    messages: Vec<Message>,
    running_turn: Option<RunningTurn>,
    tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
    model: Option<Arc<dyn LanguageModel>>,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    // ...
}

pub enum Message {
    User(UserMessage),
    Agent(AgentMessage),
    Resume,
}

pub enum ThreadEvent {
    UserMessage(UserMessage),
    AgentText(String),
    AgentThinking(String),
    ToolCallAuthorization(ToolCallAuthorization),
    ToolCall(acp::ToolCall),
    ToolCallUpdate(acp::ToolCallUpdate),
    SubagentSpawned(acp::SessionId),
    Retry(RetryStatus),
    Stop(acp::StopReason),
}
```

## Complete Flow: User Message to LLM Response

### Native Agent Flow

```
1. User types message in GUI (TextThreadEditor/MessageEditor)
       ↓
2. GUI calls AcpThread via AgentConnection::prompt()
       ↓
3. NativeAgentConnection::prompt() receives acp::PromptRequest
       ↓
4. Converts prompt to UserMessageContent[], calls Thread::send(id, content, cx)
       ↓
5. Thread::send() pushes UserMessage, calls run_turn()
       ↓
6. Thread::run_turn() starts RunningTurn with ThreadEventStream
       ↓
7. run_turn_internal() loop:
   a. build_completion_request() - creates LanguageModelRequest with system prompt, messages, tools
   b. model.stream_completion(request) - calls LLM provider
   c. Process LanguageModelCompletionEvent stream:
      - Text → pending_message.content.push(Text)
      - Thinking → pending_message.content.push(Thinking)
      - ToolUse → spawn tool execution task
   d. Wait for tool results, push to pending_message.tool_results
   e. flush_pending_message() → messages.push(Agent(pending_message))
   f. If tools called, continue loop (send tool results to model)
   g. If no tools, return (turn complete)
       ↓
8. ThreadEventStream sends events to channel
       ↓
9. NativeAgentConnection::handle_thread_events() receives events:
   - ThreadEvent::AgentText → AcpThread::push_assistant_content_block()
   - ThreadEvent::ToolCall → AcpThread::upsert_tool_call()
   - ThreadEvent::Stop → return PromptResponse
       ↓
10. AcpThread emits AcpThreadEvent, UI updates
```

### External Agent Flow

```
1. User types message in GUI
       ↓
2. GUI calls AcpThread via AgentConnection::prompt()
       ↓
3. AcpConnection::prompt() sends via JSON-RPC:
   connection.prompt(acp::PromptRequest { session_id, prompt })
       ↓
4. External agent process receives, runs LLM internally
       ↓
5. External agent sends session_notification() with SessionUpdate variants:
   - AgentMessageChunk, AgentThoughtChunk
   - ToolCall, ToolCallUpdate
   - Plan, AvailableCommandsUpdate
       ↓
6. ClientDelegate::session_notification() receives, forwards to AcpThread:
   thread.handle_session_update(update, cx)
       ↓
7. AcpThread::handle_session_update() processes each variant:
   - UserMessageChunk → push_user_content_block()
   - AgentMessageChunk → push_assistant_content_block()
   - ToolCall → upsert_tool_call()
   - ToolCallUpdate → update_tool_call()
       ↓
8. AcpThread emits AcpThreadEvent, UI updates
       ↓
9. When turn ends, prompt() returns PromptResponse
```

## Key Files Reference

### ACP Traits
- `crates/acp_thread/src/connection.rs` - All ACP trait definitions
- `crates/acp_thread/src/acp_thread.rs` - AcpThread entity, session update handling

### Native Agent
- `crates/agent/src/agent.rs` - NativeAgent, NativeAgentConnection impl
- `crates/agent/src/thread.rs` - Thread struct, message handling, LLM completion loop
- `crates/agent/src/tools/*.rs` - Built-in tool implementations

### External Agents
- `crates/agent_servers/src/agent_servers.rs` - AgentServer trait
- `crates/agent_servers/src/acp.rs` - AcpConnection impl, ClientDelegate
- `crates/agent_servers/src/claude.rs` - Claude Code server
- `crates/agent_servers/src/codex.rs` - OpenAI Codex server
- `crates/agent_servers/src/gemini.rs` - Google Gemini CLI server
- `crates/agent_servers/src/custom.rs` - Custom ACP servers

### GUI Components
- `crates/agent_ui/src/agent_panel.rs` - Main agent panel
- `crates/agent_ui/src/text_thread_editor.rs` - Thread editor, AgentPanelDelegate trait
- `crates/agent_ui/src/acp/thread_view.rs` - Thread view components
- `crates/agent_ui/src/acp/message_editor.rs` - Message input
- `crates/agent_ui/src/acp/model_selector.rs` - Model selection UI

### LLM Infrastructure
- `crates/language_model/src/language_model.rs` - LanguageModel trait
- `crates/language_model/src/registry.rs` - LanguageModelRegistry
- `crates/language_models/src/provider/*.rs` - Provider implementations

## Common Development Tasks

### Adding a New ACP Capability

1. Add method to `AgentConnection` trait in `crates/acp_thread/src/connection.rs`
2. Implement in `NativeAgentConnection` (`crates/agent/src/agent.rs`)
3. Implement in `AcpConnection` (`crates/agent_servers/src/acp.rs`)
4. Add UI support in `agent_ui` crate

### Adding a New Built-in Tool

1. Create tool struct implementing `AgentTool<Input>` trait
2. Register in `Thread::add_default_tools()` (`crates/agent/src/thread.rs`)
3. Tool automatically available to native agent

### Adding a New External Agent Server

1. Create struct implementing `AgentServer` trait
2. Register in `agent_server_store.rs`
3. Implement connection logic in `connect()` method

### Handling New Session Update Types

1. Add variant to `acp::SessionUpdate` (in `agent_client_protocol`)
2. Handle in `AcpThread::handle_session_update()`
3. Emit appropriate `AcpThreadEvent`
4. Handle event in UI components
