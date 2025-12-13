# Zed Todo List Feature Implementation Plan

## Overview

This plan outlines the implementation of a structured todo list feature in Zed that integrates with the existing thread/agent system. The feature provides role-based task management similar to Claude Code's todo list functionality, but using Zed's SQLite database infrastructure.

## Key Features

- **Thread Integration**: Todo lists associated with specific threads
- **Role-Based Tasks**: Tasks assigned to specific roles (Manager, Frontend Architect, etc.)
- **Progress Tracking**: Current task advancement and completion status
- **Cascade Deletion**: When threads are deleted, associated todos are automatically deleted
- **Agent Tools**: MCP-style tools for programmatic todo management
- **UI Integration**: Seamless integration with existing Zed UI patterns

## Implementation Phases

### Phase 1: Core Database & Backend (Priority: High)

#### Database Schema Design

```sql
-- Todo lists table (one todo list per thread)
CREATE TABLE IF NOT EXISTS todo_lists (
    id TEXT PRIMARY KEY,
    thread_id TEXT NOT NULL,
    title TEXT NOT NULL,
    parent_todo_id TEXT, -- For nested todo lists
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (thread_id) REFERENCES threads(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_todo_id) REFERENCES todo_lists(id) ON DELETE CASCADE
);

-- Individual tasks table
CREATE TABLE IF NOT EXISTS todo_tasks (
    id TEXT PRIMARY KEY,
    todo_list_id TEXT NOT NULL,
    role TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    context TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    is_completed BOOLEAN DEFAULT FALSE,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (todo_list_id) REFERENCES todo_lists(id) ON DELETE CASCADE
);

-- Progress tracking table (current task pointer)
CREATE TABLE IF NOT EXISTS todo_progress (
    todo_list_id TEXT PRIMARY KEY,
    current_task_index INTEGER DEFAULT 0,
    total_tasks INTEGER DEFAULT 0,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (todo_list_id) REFERENCES todo_lists(id) ON DELETE CASCADE
);

-- Performance indexes
CREATE INDEX IF NOT EXISTS idx_todo_lists_thread_id ON todo_lists(thread_id);
CREATE INDEX IF NOT EXISTS idx_todo_tasks_todo_list_id ON todo_tasks(todo_list_id);
CREATE INDEX IF NOT EXISTS idx_todo_tasks_order ON todo_tasks(todo_list_id, order_index);
```

#### Rust Database Integration

**File: `crates/agent/src/db.rs`**

Extend the existing `ThreadsDatabase` with todo-related methods:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ui::SharedString;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbTodoList {
    pub id: SharedString,
    pub thread_id: acp::SessionId,
    pub title: SharedString,
    pub parent_todo_id: Option<SharedString>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbTodoTask {
    pub id: SharedString,
    pub todo_list_id: SharedString,
    pub role: SharedString,
    pub title: SharedString,
    pub content: SharedString,
    pub context: SharedString,
    pub order_index: usize,
    pub is_completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbTodoProgress {
    pub todo_list_id: SharedString,
    pub current_task_index: usize,
    pub total_tasks: usize,
    pub updated_at: DateTime<Utc>,
}

impl ThreadsDatabase {
    // Todo list management (thread_id parameter removed - use current thread context)
    pub async fn create_todo_list_for_current_thread(
        &self,
        title: String,
        parent_todo_id: Option<String>,
        current_thread_id: acp::SessionId,
    ) -> Result<SharedString>;
    
    pub async fn get_todo_list(&self, todo_list_id: &str) -> Result<Option<DbTodoList>>;
    
    pub async fn get_thread_todo_lists(
        &self,
        thread_id: acp::SessionId,
    ) -> Result<Vec<DbTodoList>>;
    
    pub async fn delete_todo_list(&self, todo_list_id: &str) -> Result<()>;
    
    // Task management
    pub async fn add_task_to_todo_list(
        &self,
        todo_list_id: &str,
        role: String,
        title: String,
        content: String,
        context: String,
        order_index: usize,
    ) -> Result<SharedString>;
    
    pub async fn get_todo_tasks(&self, todo_list_id: &str) -> Result<Vec<DbTodoTask>>;
    
    pub async fn mark_task_completed(&self, task_id: &str) -> Result<()>;
    
    pub async fn update_task(&self, task_id: &str, updates: DbTodoTask) -> Result<()>;
    
    // Progress tracking
    pub async fn get_todo_progress(&self, todo_list_id: &str) -> Result<Option<DbTodoProgress>>;
    
    pub async fn update_todo_progress(&self, progress: &DbTodoProgress) -> Result<()>;
    
    pub async fn advance_current_task(&self, todo_list_id: &str) -> Result<Option<DbTodoTask>>;
    
    // Cleanup methods (called when threads are deleted)
    pub async fn delete_thread_todos(&self, thread_id: acp::SessionId) -> Result<()>;
    
    // Enhanced thread deletion with todo cleanup
    pub async fn delete_thread(&self, thread_id: acp::SessionId) -> Result<()>;
}
```

#### Database Migration

**File: `crates/agent/src/db.rs`** - Add to the existing database setup:

```rust
connection.exec(indoc! {"
    CREATE TABLE IF NOT EXISTS todo_lists (
        id TEXT PRIMARY KEY,
        thread_id TEXT NOT NULL,
        title TEXT NOT NULL,
        parent_todo_id TEXT,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        FOREIGN KEY (thread_id) REFERENCES threads(id) ON DELETE CASCADE,
        FOREIGN KEY (parent_todo_id) REFERENCES todo_lists(id) ON DELETE CASCADE
    );
    
    CREATE TABLE IF NOT EXISTS todo_tasks (
        id TEXT PRIMARY KEY,
        todo_list_id TEXT NOT NULL,
        role TEXT NOT NULL,
        title TEXT NOT NULL,
        content TEXT NOT NULL,
        context TEXT NOT NULL,
        order_index INTEGER NOT NULL,
        is_completed BOOLEAN DEFAULT FALSE,
        completed_at TEXT,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        FOREIGN KEY (todo_list_id) REFERENCES todo_lists(id) ON DELETE CASCADE
    );
    
    CREATE TABLE IF NOT EXISTS todo_progress (
        todo_list_id TEXT PRIMARY KEY,
        current_task_index INTEGER DEFAULT 0,
        total_tasks INTEGER DEFAULT 0,
        updated_at TEXT NOT NULL,
        FOREIGN KEY (todo_list_id) REFERENCES todo_lists(id) ON DELETE CASCADE
    );
    
    -- Indexes for performance
    CREATE INDEX IF NOT EXISTS idx_todo_lists_thread_id ON todo_lists(thread_id);
    CREATE INDEX IF NOT EXISTS idx_todo_tasks_todo_list_id ON todo_tasks(todo_list_id);
    CREATE INDEX IF NOT EXISTS idx_todo_tasks_order ON todo_tasks(todo_list_id, order_index);
})?()
.map_err(|e| anyhow!("Failed to create todo tables: {}", e))?;
```

### Phase 2: Agent Tools Implementation

#### Tool Structure Pattern

Each tool follows this pattern:
- `Input` struct with JSON schema (NO thread_id parameter)
- `Output` struct for responses  
- Implements `AgentTool` trait with name, kind, title, description, and run methods
- Constructor receives `WeakEntity<Thread>` to access current thread context
- Returns `Task<Result<Output>>` for async execution

#### Create Todo List Tool

**File: `crates/agent/src/tools/create_todo_list_tool.rs`**

```rust
use anyhow::Result;
use gpui::{App, SharedString, Task, WeakEntity};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use agent_client_protocol as acp;
use std::sync::Arc;
use crate::{AgentTool, Thread, ToolCallEventStream};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateTodoListInput {
    /// Title of the todo list
    title: String,
    /// ID of parent todo list (for nested lists)
    #[serde(default)]
    parent_todo_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateTodoListOutput {
    /// The ID of the created todo list
    todo_list_id: String,
}

pub struct CreateTodoListTool {
    thread: WeakEntity<Thread>,
    db: Arc<Mutex<ThreadsDatabase>>,
}

impl CreateTodoListTool {
    pub fn new(thread: WeakEntity<Thread>, db: Arc<Mutex<ThreadsDatabase>>) -> Self {
        Self {
            thread,
            db,
        }
    }
}

impl AgentTool for CreateTodoListTool {
    type Input = CreateTodoListInput;
    type Output = CreateTodoListOutput;
    
    fn name() -> &'static str {
        "create_todo_list"
    }
    
    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }
    
    fn description() -> &'static str {
        "Creates structured todo lists for managing multi-step tasks with role-based assignments"
    }
    
    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Create a new todo list for structured task management".into()
    }
    
    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<CreateTodoListOutput>> {
        let thread = self.thread.clone();
        let db = self.db.clone();
        let fut = async move {
            // Get the current thread ID from the thread context
            let thread_id = thread.read().id().clone();
            
            let todo_list_id = db.lock().await.create_todo_list_for_current_thread(
                input.title,
                input.parent_todo_id,
                thread_id,
            ).await?;
            
            Ok(CreateTodoListOutput { todo_list_id })
        };
        Task::ready(fut.await)
    }
}
```

#### Manage Todo Tasks Tool

**File: `crates/agent/src/tools/manage_todo_tasks_tool.rs`**

```rust
use gpui::{WeakEntity, App, Task};
use std::sync::Arc;
use crate::{AgentTool, Thread, ToolCallEventStream};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AddTaskInput {
    /// The ID of the todo list to add the task to
    todo_list_id: String,
    /// The role responsible for this task (Manager, Frontend Architect, etc.)
    role: String,
    /// The title of the task
    title: String,
    /// The task content/description
    content: String,
    /// Context/guidelines for the task
    context: String,
    /// Position in the task list (0-based index)
    #[serde(default)]
    order_index: usize,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetTasksInput {
    /// The ID of the todo list to get tasks from
    todo_list_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MarkTaskCompleteInput {
    /// The ID of the task to mark as complete
    task_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ManageTodoTasksOutput {
    /// The ID of the created task or success message
    task_id: Option<String>,
    /// List of tasks if requested
    tasks: Option<Vec<TodoTaskDisplay>>,
    /// Success status
    success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoTaskDisplay {
    pub id: String,
    pub role: String,
    pub title: String,
    pub content: String,
    pub context: String,
    pub order_index: usize,
    pub is_completed: bool,
    pub completed_at: Option<String>,
}

pub struct ManageTodoTasksTool {
    db: Arc<Mutex<ThreadsDatabase>>,
}

impl ManageTodoTasksTool {
    pub fn new(db: Arc<Mutex<ThreadsDatabase>>) -> Self {
        Self { db }
    }
}
```

#### Get Next Task Tool

**File: `crates/agent/src/tools/get_next_task_tool.rs`**

```rust
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetNextTaskInput {
    /// The ID of the todo list to advance
    todo_list_id: String,
    /// Whether to format as markdown for display
    #[serde(default)]
    format_as_markdown: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetNextTaskOutput {
    /// Formatted markdown showing current task and progress
    #[serde(skip_serializing_if = "Option::is_none")]
    markdown: Option<String>,
    /// Current task details if not formatted
    #[serde(skip_serializing_if = "Option::is_none")]
    current_task: Option<TodoTaskDisplay>,
    /// Progress information
    progress: TodoProgressDisplay,
    /// Whether all tasks are completed
    is_completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoProgressDisplay {
    pub current_task_index: usize,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub todo_list_id: String,
}

pub struct GetNextTaskTool {
    db: Arc<Mutex<ThreadsDatabase>>,
}

impl GetNextTaskTool {
    pub fn new(db: Arc<Mutex<ThreadsDatabase>>) -> Self {
        Self { db }
    }
}
```

#### Tool Registration

**File: `crates/agent/src/tools.rs`**

```rust
// Add new tool exports
pub use create_todo_list_tool::*;
pub use manage_todo_tasks_tool::*;
pub use get_next_task_tool::*;

// Add to tools! macro invocation
tools! {
    // ... existing tools ...
    CreateTodoListTool,
    ManageTodoTasksTool,
    GetNextTaskTool,
}
```

### Phase 3: Basic UI Implementation

#### Todo Panel Component

**File: `crates/agent_ui/src/todo_ui/todo_panel.rs`**

```rust
use gpui::{prelude::*, Entity, ViewContext, WeakEntity};
use std::sync::Arc;
use ui::SharedString;

pub struct TodoPanel {
    thread: WeakEntity<Thread>,
    // Add other necessary fields
}

impl TodoPanel {
    pub fn new(thread: WeakEntity<Thread>) -> Self {
        TodoPanel {
            thread,
            // Initialize other fields
        }
    }
}

impl Render for TodoPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .border_1()
            .border_color(gpui::rgb(0.2, 0.2, 0.2))
            .rounded-md()
            .p_4()
            .w_full()
            .child(self.render_header(cx))
            .child(self.render_todo_lists(cx))
            .child(self.render_create_button(cx))
    }
    
    fn render_header(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .child(
                ui::h3()
                    .text("Todo Lists")
            )
            .child(
                ui::Button::new("create-todo")
                    .label("New Todo List")
                    .on_click(cx.listener(|_, _, cx| {
                        // Handle new todo list creation
                    }))
            )
    }
    
    fn render_todo_lists(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // Get current thread ID from the thread entity
        let thread_id = self.thread.read(cx).id().clone();
        
        // Query database for todo lists associated with this thread
        // Render list of todo lists with progress indicators
        div().child("Todo lists will be displayed here")
    }
    
    fn render_create_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        ui::Button::new("create-todo-list")
            .label("Create Todo List")
            .full_width()
            .on_click(cx.listener(|_, _, cx| {
                // Open todo creation modal
            }))
    }
}
```

#### Integration with Agent Panel

**File: `crates/agent_ui/src/agent_panel.rs`**

Add todo section to the agent panel:

```rust
// Add to AgentPanel struct
pub struct AgentPanel {
    // ... existing fields ...
    todo_panel: Option<Entity<TodoPanel>>,
}

impl AgentPanel {
    pub fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .child(self.render_thread_list(cx))
            .child(self.render_main_content(cx))
    }
    
    fn render_main_content(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .child(self.render_thread_content(cx))
            .when_some(self.current_thread, |this, thread, cx| {
                // Pass the thread entity directly to the todo panel
                this.child(TodoPanel::new(thread).render(cx))
            })
    }
}
```

### Phase 4: Advanced Features

#### Todo List Creation Modal

**File: `crates/agent_ui/src/todo_ui/create_todo_modal.rs`**

```rust
use gpui::{prelude::*, ViewContext, WindowHandle};
use std::sync::Arc;
use ui::SharedString;

pub struct CreateTodoModal {
    thread: WeakEntity<Thread>,
    // Form state fields
}

impl CreateTodoModal {
    pub fn new(thread: WeakEntity<Thread>) -> Self {
        CreateTodoModal {
            thread,
            // Initialize form state
        }
    }
}

impl Render for CreateTodoModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        ui::Modal::new("create-todo-modal")
            .modal_title("Create Todo List")
            .child(self.render_form(cx))
            .child(self.render_actions(cx))
    }
    
    fn render_form(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .space_y_4()
            .child(self.render_title_field(cx))
            .child(self.render_tasks_section(cx))
    }
    
    fn render_title_field(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        ui::FormField::new("Title")
            .child(
                ui::TextInput::new("todo-title")
                    .placeholder("Enter todo list title")
                    .on_change(cx.listener(|_, value, _| {
                        // Update title state
                    }))
            )
    }
    
    fn render_tasks_section(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .space_y_2()
            .child(ui::h4().text("Tasks"))
            .child(self.render_task_list(cx))
            .child(
                ui::Button::new("add-task")
                    .label("Add Task")
                    .on_click(cx.listener(|_, _, cx| {
                        // Add new task field
                    }))
            )
    }
    
    fn render_actions(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .justify_end()
            .space_x_2()
            .child(
                ui::Button::new("cancel")
                    .label("Cancel")
                    .on_click(cx.listener(|this, _, cx| {
                        this.dismiss();
                    }))
            )
            .child(
                ui::Button::new("create")
                    .label("Create")
                    .variant(ui::ButtonVariant::Primary)
                    .on_click(cx.listener(|this, _, cx| {
                        this.create_todo_list();
                    }))
            )
    }
}
```

#### Thread Integration

**File: `crates/agent_ui/src/acp/thread_view.rs`**

Add todo progress display to thread headers:

```rust
impl AcpThreadView {
    fn render_thread_header(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .space_x_4()
                    .child(self.render_thread_title(cx))
                    .child(self.render_todo_progress(cx)) // New integration
            )
    }
    
    fn render_todo_progress(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // Get thread ID from current thread context
        let thread_id = self.thread.read(cx).id().clone();
        
        // Query database for todo lists associated with this thread
        // Show progress indicators and current tasks
        div()
            .flex()
            .items_center()
            .space_x_2()
            .child(
                ui::Icon::new("check-circle")
                    .color(ui::Color::Success)
            )
            .child(ui::text("2/5 tasks completed"))
    }
}
```

### Phase 5: Integration & Cleanup

#### Enhanced Thread Deletion

**File: `crates/agent/src/thread.rs`** (or wherever thread deletion is handled)

```rust
impl ThreadsDatabase {
    pub async fn delete_thread(&self, thread_id: acp::SessionId) -> Result<()> {
        let mut connection = self.connection.lock().await;
        
        // Use transaction for data consistency
        connection.exec("BEGIN TRANSACTION").await?;
        
        try {
            // 1. Delete all todo progress records for this thread's todo lists
            connection.exec(indoc! {"
                DELETE FROM todo_progress 
                WHERE todo_list_id IN (
                    SELECT id FROM todo_lists WHERE thread_id = ?
                )
            }, &[thread_id.as_str()]).await?;
            
            // 2. Delete all todo tasks for this thread's todo lists  
            connection.exec(indoc! {"
                DELETE FROM todo_tasks 
                WHERE todo_list_id IN (
                    SELECT id FROM todo_lists WHERE thread_id = ?
                )
            }, &[thread_id.as_str()]).await?;
            
            // 3. Delete all todo lists for this thread
            connection.exec(indoc! {"
                DELETE FROM todo_lists WHERE thread_id = ?
            }, &[thread_id.as_str()]).await?;
            
            // 4. Delete the thread itself (existing logic)
            connection.exec(indoc! {"
                DELETE FROM threads WHERE id = ?
            }, &[thread_id.as_str()]).await?;
            
            connection.exec("COMMIT").await?;
            Ok(())
        } catch (error) {
            connection.exec("ROLLBACK").await?;
            Err(error)
        }
    }
}
```

#### Performance Optimization

```rust
// Add caching for frequently accessed todo data
pub struct TodoCache {
    // Cache recently accessed todo lists and their progress
}

// Batch operations for better performance
impl ThreadsDatabase {
    pub async fn get_multiple_todo_lists(&self, thread_ids: &[acp::SessionId]) -> Result<HashMap<acp::SessionId, Vec<DbTodoList>>> {
        // Efficient batch query for multiple threads
    }
}
```

## Implementation Checklist

### Phase 1: Core Database & Backend
- [ ] Add database schema migration with foreign key relationships
- [ ] Implement basic CRUD operations for todo lists and tasks
- [ ] Add progress tracking functionality
- [ ] Implement cascade deletion for thread cleanup
- [ ] Add database indexes for performance

### Phase 2: Agent Tools
- [ ] Create CreateTodoListTool with proper schema and thread context access
- [ ] Create ManageTodoTasksTool with add/get/complete operations
- [ ] Create GetNextTaskTool with progress advancement and markdown formatting
- [ ] Register tools with the agent system
- [ ] Add comprehensive error handling

### Phase 3: Basic UI
- [ ] Create TodoPanel component for todo list display with thread context
- [ ] Integrate todo panel with agent panel
- [ ] Add basic todo list visualization
- [ ] Implement thread-to-todo-list association display

### Phase 4: Advanced Features
- [ ] Create TodoListModal for creating new todo lists
- [ ] Add task creation and editing functionality
- [ ] Implement current task highlighting in conversations
- [ ] Add progress indicators and completion tracking

### Phase 5: Polish & Testing
- [ ] Add comprehensive error handling and user feedback
- [ ] Implement performance optimizations for large todo lists
- [ ] Add unit tests for all database operations
- [ ] Add integration tests for agent tools
- [ ] Add UI tests for todo functionality
- [ ] Performance testing with large datasets
- [ ] User experience testing and refinements

## Key Architectural Improvements

This implementation provides:

1. **Thread Context Awareness**: Tools automatically determine the current thread ID from their context
2. **Consistent Architecture**: Follows Zed's existing pattern where tools receive thread entities in constructors
3. **No Redundant Parameters**: Eliminates error-prone thread_id parameters that are redundant with context
4. **Thread Integration**: Todo lists tied to specific threads, with automatic cleanup
5. **Role-Based Management**: Structured task assignment with role definitions
6. **Progress Tracking**: Current task advancement and completion status
7. **Database Consistency**: SQLite storage with proper foreign key relationships
8. **Agent Tools**: MCP-style tools for programmatic todo management
9. **UI Integration**: Seamless integration with existing Zed UI patterns
10. **Cascade Deletion**: Automatic cleanup when threads are deleted

## Success Metrics

- Todo lists are created and managed through the agent tools using thread context
- Progress is tracked and displayed correctly in the UI
- Thread deletion properly cleans up associated todos
- Performance is acceptable with large numbers of tasks
- UI integrates seamlessly with existing Zed patterns
- Database operations are efficient and consistent
- Tools follow Zed's established architectural patterns

This comprehensive plan creates a todo list system in Zed that rivals Claude Code's functionality while following Zed's architectural patterns and integrating deeply with the existing thread/agent system.
```
