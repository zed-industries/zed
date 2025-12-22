# Convergio Custom Chat Panel Architecture

## Overview

This document describes the architecture for a **completely custom chat panel** that bypasses Zed's agent_ui infrastructure and connects directly to Convergio's database for full synchronization between CLI and Zed.

## Problem Statement

The current implementation has critical issues:

1. **Two separate databases**: Zed uses `~/Library/Application Support/Zed/threads/threads.db` while Convergio CLI uses `~/Library/Containers/com.convergio.app/Data/data/convergio.db`
2. **No synchronization**: Conversations in CLI don't appear in Zed and vice versa
3. **Schema mismatches**: Zed's schema expects `agent_name` column that wasn't migrated
4. **Dependency on agent_ui**: We're fighting against Zed's internal infrastructure

## Solution: Custom Chat Panel

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           CONVERGIO ZED                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────────┐  │
│  │ConvergioPanel   │    │ConvergioChatView│    │ Ali Command Center  │  │
│  │ (Agent List)    │───▶│ (Custom Chat UI)│    │ (Terminal)          │  │
│  │ Left Dock       │    │ Right Dock      │    │ Bottom Dock         │  │
│  └─────────────────┘    └────────┬────────┘    └─────────────────────┘  │
│                                  │                                       │
│                                  │ ACP Protocol                         │
│                                  ▼                                       │
│                         ┌─────────────────┐                             │
│                         │ convergio-acp   │                             │
│                         │ (Server Process)│                             │
│                         └────────┬────────┘                             │
│                                  │                                       │
└──────────────────────────────────┼──────────────────────────────────────┘
                                   │
                                   │ SQLite Read/Write
                                   ▼
                    ┌──────────────────────────────┐
                    │    convergio.db              │
                    │    (Single Source of Truth)  │
                    │                              │
                    │  ┌────────────────────────┐  │
                    │  │ sessions               │  │
                    │  │ - id (session_id)      │  │
                    │  │ - user_name            │  │
                    │  │ - started_at           │  │
                    │  └────────────────────────┘  │
                    │                              │
                    │  ┌────────────────────────┐  │
                    │  │ messages               │  │
                    │  │ - session_id           │  │
                    │  │ - sender_name (agent)  │  │
                    │  │ - content              │  │
                    │  │ - type (user/assistant)│  │
                    │  │ - created_at           │  │
                    │  └────────────────────────┘  │
                    │                              │
                    │  ┌────────────────────────┐  │
                    │  │ agents                 │  │
                    │  │ - name                 │  │
                    │  │ - role                 │  │
                    │  │ - system_prompt        │  │
                    │  └────────────────────────┘  │
                    └──────────────────────────────┘
                                   ▲
                                   │ SQLite Read/Write
                                   │
                    ┌──────────────────────────────┐
                    │    Convergio CLI             │
                    │    (Terminal)                │
                    └──────────────────────────────┘
```

## Database Schema (convergio.db)

### sessions table
```sql
CREATE TABLE sessions (
  id TEXT PRIMARY KEY,           -- Session ID (e.g., "sess_1_1234567890")
  user_name TEXT,                -- User name
  total_cost REAL DEFAULT 0,     -- Total cost for session
  total_messages INTEGER DEFAULT 0,
  started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  ended_at DATETIME
);
```

### messages table
```sql
CREATE TABLE messages (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  session_id TEXT NOT NULL,      -- Links to sessions.id
  type INTEGER NOT NULL,         -- 0=system, 1=user, 2=assistant, 3=tool
  sender_id INTEGER,
  sender_name TEXT,              -- Agent name (e.g., "ali", "amy-cfo")
  recipient_id INTEGER,
  content TEXT NOT NULL,         -- Message content (markdown)
  metadata_json TEXT,            -- JSON metadata
  input_tokens INTEGER DEFAULT 0,
  output_tokens INTEGER DEFAULT 0,
  cost_usd REAL DEFAULT 0,
  parent_id INTEGER,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### agents table
```sql
CREATE TABLE agents (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT UNIQUE NOT NULL,     -- Agent identifier (e.g., "ali")
  role INTEGER NOT NULL,         -- Role type
  system_prompt TEXT NOT NULL,
  specialized_context TEXT,
  color TEXT,
  tools_json TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## Component Design

### 1. ConvergioChatView (New Component)

**Location**: `crates/convergio_panel/src/chat_view.rs`

**Responsibilities**:
- Render chat messages from convergio.db
- Handle user input
- Send messages via ACP to convergio-acp
- Poll for new messages / subscribe to updates

**Struct Definition**:
```rust
pub struct ConvergioChatView {
    focus_handle: FocusHandle,
    agent_name: SharedString,
    session_id: Option<String>,
    messages: Vec<ChatMessage>,
    input_editor: Entity<Editor>,
    scroll_handle: ScrollHandle,
    db_connection: Option<Arc<Mutex<rusqlite::Connection>>>,
    acp_connection: Option<Entity<AcpConnection>>,
    _subscriptions: Vec<Subscription>,
    is_loading: bool,
    is_streaming: bool,
}

pub struct ChatMessage {
    id: i64,
    message_type: MessageType,
    sender_name: Option<String>,
    content: String,
    created_at: DateTime<Utc>,
    tokens: Option<(i64, i64)>,  // (input, output)
    cost: Option<f64>,
}

pub enum MessageType {
    System = 0,
    User = 1,
    Assistant = 2,
    Tool = 3,
}
```

**Key Methods**:
```rust
impl ConvergioChatView {
    /// Create new chat view for an agent
    pub fn new(
        agent_name: SharedString,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self;

    /// Load messages from convergio.db
    fn load_messages(&mut self, cx: &mut Context<Self>);

    /// Send a message via ACP
    fn send_message(&mut self, content: String, window: &mut Window, cx: &mut Context<Self>);

    /// Handle incoming ACP response
    fn handle_acp_response(&mut self, response: AcpResponse, cx: &mut Context<Self>);

    /// Render a single message
    fn render_message(&self, message: &ChatMessage, cx: &Context<Self>) -> impl IntoElement;

    /// Render the input area
    fn render_input(&self, window: &mut Window, cx: &Context<Self>) -> impl IntoElement;
}
```

### 2. ConvergioDb (New Component)

**Location**: `crates/convergio_panel/src/convergio_db.rs`

**Responsibilities**:
- Connect to convergio.db
- Query sessions and messages
- Watch for database changes

**Struct Definition**:
```rust
pub struct ConvergioDb {
    connection: Arc<Mutex<rusqlite::Connection>>,
    db_path: PathBuf,
}

impl ConvergioDb {
    /// Open connection to convergio database
    pub fn open() -> Result<Self>;

    /// Get all sessions for an agent
    pub fn sessions_for_agent(&self, agent_name: &str) -> Result<Vec<Session>>;

    /// Get messages for a session
    pub fn messages_for_session(&self, session_id: &str) -> Result<Vec<ChatMessage>>;

    /// Get the most recent session for an agent
    pub fn latest_session_for_agent(&self, agent_name: &str) -> Result<Option<Session>>;

    /// Watch for changes (using SQLite notify or polling)
    pub fn watch_changes(&self, callback: impl Fn() + Send + 'static);
}
```

### 3. Integration with ConvergioPanel

**Changes to `crates/convergio_panel/src/panel.rs`**:

```rust
// When user clicks on an agent, open ConvergioChatView instead of AcpThreadView
fn open_agent_chat(&mut self, agent_name: &str, window: &mut Window, cx: &mut Context<Self>) {
    // Create or get existing ConvergioChatView for this agent
    let chat_view = cx.new(|cx| {
        ConvergioChatView::new(
            agent_name.into(),
            self._workspace.clone(),
            window,
            cx,
        )
    });

    // Open in right dock or center
    if let Some(workspace) = self._workspace.upgrade() {
        workspace.update(cx, |workspace, cx| {
            // Add as item in right panel or open in center
            workspace.open_item_in_position(chat_view, DockPosition::Right, window, cx);
        });
    }
}
```

## File Watching Strategy

Since both CLI and Zed access the same database, we need to handle concurrent access:

1. **Polling approach** (simple):
   - Poll database every 1-2 seconds for new messages
   - Compare message count or latest timestamp
   - Update UI if changes detected

2. **SQLite WAL mode** (recommended):
   - Enable WAL mode for better concurrent access
   - Use `PRAGMA journal_mode=WAL;`

3. **File system watching** (advanced):
   - Watch convergio.db-wal for changes
   - Trigger reload when file modified

## ACP Integration

For sending messages, we still use ACP protocol to convergio-acp server:

```rust
// Send user message
let request = acp::SamplingRequest {
    messages: vec![acp::Message {
        role: "user",
        content: user_input,
    }],
    // ... other fields
};

let response = acp_connection.sample(request).await?;

// The response is automatically saved to convergio.db by convergio-acp
// We just need to reload messages from the database
self.load_messages(cx);
```

## Migration Path

1. **Phase 1**: Create ConvergioChatView that reads from convergio.db
2. **Phase 2**: Replace AcpThreadView usage in ConvergioPanel
3. **Phase 3**: Remove agent_ui dependency from convergio_panel
4. **Phase 4**: Delete unused Zed threads.db integration code

## Benefits

1. **Single source of truth**: One database for both CLI and Zed
2. **Automatic sync**: Both read/write to same database
3. **Full history**: All conversations preserved and accessible
4. **No migrations**: Use convergio's existing schema
5. **Independence**: No dependency on Zed's agent_ui internals
6. **Control**: Full control over UI/UX
7. **Performance**: Direct SQLite access, no intermediate layers

## Implementation Order

1. [ ] Create `convergio_db.rs` with database access
2. [ ] Create `chat_view.rs` with basic UI
3. [ ] Implement message rendering with markdown
4. [ ] Implement input handling
5. [ ] Connect to ACP for sending messages
6. [ ] Add polling for new messages
7. [ ] Update ConvergioPanel to use ConvergioChatView
8. [ ] Remove agent_ui dependency
9. [ ] Test full sync between CLI and Zed
