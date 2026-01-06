# Agent Sharing Implementation Plan

## Overview

This document outlines the implementation plan for the **Agent Sharing** feature, which allows users to share their agent threads with others via a URL. When someone opens that URL, they can import the thread into their local Zed instance.

**URL Format:** `zed://agent/shared/<session_id>` (where session_id is a client-generated UUID)

**Imported Thread Naming:** Shared threads are imported with a 🔗 prefix (e.g., "🔗 Original Thread Title")

**Key Design Decision:** Thread IDs are client-generated UUIDs (`acp::SessionId`). When sharing, the client sends this ID to the server. Re-sharing the same thread updates the existing record rather than creating duplicates. This prevents database spam and allows users to update shared threads.

---

## Development Philosophy: Test-Driven Development

This feature should be implemented following **Test-Driven Development (TDD)** principles:

1. **Write tests first** - Before implementing any functionality, write failing tests that describe the expected behavior
2. **Red-Green-Refactor** - Get tests to fail (red), implement minimal code to pass (green), then refactor
3. **Integration tests are critical** - This feature involves client-server communication, so integration tests are essential

### Key Integration Test

**Primary integration test:** Set up two simulated Zed app instances, have one instance share a thread, then have the second instance import it via the share URL. Verify the imported thread matches the original.

This test should be added to `crates/collab/src/tests/` following the patterns of existing integration tests.

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Phase 1: Feature Flag](#phase-1-feature-flag)
3. [Phase 2: Data Layer](#phase-2-data-layer)
4. [Phase 3: RPC Protocol](#phase-3-rpc-protocol)
5. [Phase 4: Server Implementation](#phase-4-server-implementation)
6. [Phase 5: Client Implementation](#phase-5-client-implementation)
7. [Phase 6: UI Changes](#phase-6-ui-changes)
8. [Phase 7: URL Handling](#phase-7-url-handling)
9. [Phase 8: Import Tracking & Sync](#phase-8-import-tracking--sync)
10. [Testing Strategy](#testing-strategy)
11. [Security Considerations](#security-considerations)
12. [Future Enhancements](#future-enhancements)

---

## Architecture Overview

### Flow Summary

1. **Share Flow:** User clicks share button → Client serializes thread with session_id → Sends to server → Server upserts in DB (insert or update based on session_id) → Client constructs URL using session_id and shows toast with copy button
2. **Import Flow:** User opens share URL → Zed parses session_id from URL → Fetches thread from server → Creates local copy with 🔗 prefix and `imported: true` → Opens thread view (uses same session_id as source)
3. **Sync Flow:** User clicks sync button on imported thread → Client fetches latest version from server using thread's session_id → Updates local thread with new messages → Shows toast confirming sync

---

## Phase 1: Feature Flag

### 1.1 Add Feature Flag

**File:** `crates/feature_flags/src/flags.rs`

Add a new feature flag to gate the share button UI:

```rust
pub struct AgentSharingFeatureFlag;

impl FeatureFlag for AgentSharingFeatureFlag {
    const NAME: &'static str = "agent-sharing";
}
```

This flag will:
- Be enabled for staff by default (standard behavior)
- Gate the share button visibility in the thread view UI
- Allow gradual rollout to users

---

## Phase 2: Data Layer

### 2.1 Shareable Thread Format

**File:** `crates/agent/src/db.rs`

Create a new `SharedThread` struct that represents the transferable format. This is similar to `DbThread` but omits machine-specific and user-specific data:

```rust
/// A thread format suitable for sharing across users/machines.
/// Omits machine-specific data like project snapshots and user-specific data like profiles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedThread {
    pub title: SharedString,
    pub messages: Vec<DbMessage>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub model: Option<DbLanguageModel>,
    #[serde(default)]
    pub completion_mode: Option<CompletionMode>,
    pub version: String,
}

impl SharedThread {
    pub const VERSION: &'static str = "1.0.0";

    /// Convert from a DbThread, stripping machine-specific data
    pub fn from_db_thread(thread: &DbThread) -> Self {
        Self {
            title: thread.title.clone(),
            messages: thread.messages.clone(),
            updated_at: thread.updated_at,
            model: thread.model.clone(),
            completion_mode: thread.completion_mode,
            version: Self::VERSION.to_string(),
        }
    }

    /// Convert to a DbThread for local storage.
    /// Prepends 🔗 to the title to indicate this is an imported shared thread.
    /// Sets `imported: true` to enable syncing (uses same session_id as source).
    pub fn to_db_thread(self) -> DbThread {
        DbThread {
            title: format!("🔗 {}", self.title).into(),
            messages: self.messages,
            updated_at: self.updated_at,
            detailed_summary: None,
            initial_project_snapshot: None,
            cumulative_token_usage: Default::default(),
            request_token_usage: Default::default(),
            model: self.model,
            completion_mode: self.completion_mode,
            profile: None,
            imported: true,
        }
    }

    /// Serialize to JSON bytes (uncompressed)
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    /// Deserialize from JSON bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(data)?)
    }
}
```

**Key Differences from `DbThread`:**

| Field | Included | Reason |
|-------|----------|--------|
| `title` | ✅ | Needed for display (prefixed with 🔗 on import) |
| `messages` | ✅ | Core thread content |
| `updated_at` | ✅ | Metadata |
| `model` | ✅ | Useful context |
| `completion_mode` | ✅ | Useful context |
| `detailed_summary` | ❌ | Can be regenerated |
| `initial_project_snapshot` | ❌ | Machine-specific |
| `cumulative_token_usage` | ❌ | User-specific billing |
| `request_token_usage` | ❌ | User-specific billing |
| `profile` | ❌ | User-specific |

### 2.2 Database Migration

**File:** `crates/collab/migrations/YYYYMMDDHHMMSS_add_shared_threads.sql`

```sql
CREATE TABLE shared_threads (
    id UUID PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(512) NOT NULL,
    data BYTEA NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_shared_threads_user_id ON shared_threads(user_id);
```

**Field Descriptions:**

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Client-generated session ID (primary key) |
| `user_id` | INTEGER | User who created the share |
| `title` | VARCHAR(512) | Thread title for preview |
| `data` | BYTEA | Compressed thread data |
| `created_at` | TIMESTAMP | When the share was first created |
| `updated_at` | TIMESTAMP | When the share was last updated |

### 2.3 Database ID Type

**File:** `crates/collab/src/db/ids.rs`

The shared thread ID is a UUID (matching the client's `acp::SessionId`). Add the ID type:

```rust
/// A client-generated UUID identifying a shared thread.
/// This matches the `acp::SessionId` used by the client.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, DeriveValueType)]
pub struct SharedThreadId(pub Uuid);

impl SharedThreadId {
    pub fn from_proto(id: String) -> Option<Self> {
        Uuid::parse_str(&id).ok().map(SharedThreadId)
    }

    pub fn to_proto(self) -> String {
        self.0.to_string()
    }
}
```

### 2.4 Database Entity

**File:** `crates/collab/src/db/tables/shared_thread.rs`

```rust
use crate::db::{SharedThreadId, UserId};
use sea_orm::entity::prelude::*;
use time::PrimitiveDateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "shared_threads")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: SharedThreadId,
    pub user_id: UserId,
    pub title: String,
    pub data: Vec<u8>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

**Update `crates/collab/src/db/tables.rs`:**

```rust
pub mod shared_thread;
```

### 2.5 Database Queries

**File:** `crates/collab/src/db/queries/shared_threads.rs`

```rust
use super::*;
use crate::db::tables::shared_thread;

impl Database {
    pub async fn upsert_shared_thread(
        &self,
        id: SharedThreadId,
        user_id: UserId,
        title: &str,
        data: Vec<u8>,
    ) -> Result<SharedThreadId> {
        let title = title.to_string();
        self.transaction(|tx| {
            let title = title.clone();
            let data = data.clone();
            async move {
                let now = Utc::now().naive_utc();

                let existing = shared_thread::Entity::find_by_id(id)
                    .one(&*tx)
                    .await?;

                match existing {
                    Some(existing) => {
                        if existing.user_id != user_id {
                            return Err(anyhow::anyhow!(
                                "Cannot update shared thread owned by another user"
                            ));
                        }

                        let mut active: shared_thread::ActiveModel = existing.into();
                        active.title = ActiveValue::Set(title);
                        active.data = ActiveValue::Set(data);
                        active.updated_at = ActiveValue::Set(now);
                        active.update(&*tx).await?;
                    }
                    None => {
                        shared_thread::ActiveModel {
                            id: ActiveValue::Set(id),
                            user_id: ActiveValue::Set(user_id),
                            title: ActiveValue::Set(title),
                            data: ActiveValue::Set(data),
                            created_at: ActiveValue::Set(now),
                            updated_at: ActiveValue::Set(now),
                        }
                        .insert(&*tx)
                        .await?;
                    }
                }

                Ok(id)
            }
        })
        .await
    }

    /// Get a shared thread by ID.
    pub async fn get_shared_thread(
        &self,
        share_id: SharedThreadId,
    ) -> Result<Option<(shared_thread::Model, String)>> {
        self.transaction(|tx| async move {
            let Some(thread) = shared_thread::Entity::find_by_id(share_id)
                .one(&*tx)
                .await?
            else {
                return Ok(None);
            };

            // Get the sharer's username
            let user = crate::db::tables::user::Entity::find_by_id(thread.user_id)
                .one(&*tx)
                .await?;

            let username = user
                .map(|u| u.github_login)
                .unwrap_or_else(|| "Unknown".to_string());

            Ok(Some((thread, username)))
        })
        .await
    }

    /// List shared threads for a user.
    /// NOTE: This is primarily for testing. Not exposed via RPC.
    #[cfg(test)]
    pub async fn list_shared_threads_for_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<shared_thread::Model>> {
        self.transaction(|tx| async move {
            let threads = shared_thread::Entity::find()
                .filter(shared_thread::Column::UserId.eq(user_id))
                .all(&*tx)
                .await?;

            Ok(threads)
        })
        .await
    }
}
```

**Update `crates/collab/src/db/queries.rs`:**

```rust
pub mod shared_threads;
```

---

## Phase 3: RPC Protocol

### 3.1 Proto Messages

**File:** `crates/proto/proto/zed.proto`

Add to the `Envelope` message's `oneof payload` (after line ~405, current max is 405):

```protobuf
// Agent thread sharing
ShareAgentThread share_agent_thread = 406;
GetSharedAgentThread get_shared_agent_thread = 407;
GetSharedAgentThreadResponse get_shared_agent_thread_response = 408; // current max
```

Note: `ShareAgentThread` uses the standard `Ack` response message.

Add message definitions (at the end of the file or in a logical location):

```protobuf
// Share an agent thread (upserts based on session_id)
// Response: Ack
message ShareAgentThread {
    string session_id = 1;  // Client-generated UUID (acp::SessionId)
    string title = 2;
    bytes thread_data = 3;  // SharedThread compressed JSON
}

// Retrieve a shared agent thread
message GetSharedAgentThread {
    string session_id = 1;  // UUID string
}

message GetSharedAgentThreadResponse {
    string title = 1;
    bytes thread_data = 2;  // SharedThread compressed JSON
    string sharer_username = 3;
    string created_at = 4;  // ISO 8601 timestamp
}
```

---

## Phase 4: Server Implementation

### 4.1 RPC Handlers

**File:** `crates/collab/src/rpc.rs`

Add handlers to `Server::new()` (around line 470):

```rust
.add_request_handler(share_agent_thread)
.add_request_handler(get_shared_agent_thread)
```

Add handler functions (at the end of the file, before `ResultExt`):

```rust
async fn share_agent_thread(
    request: proto::ShareAgentThread,
    response: Response<proto::ShareAgentThread>,
    cx: MessageContext,
) -> Result<()> {
    let user_id = cx.session.user_id()?;

    let share_id = SharedThreadId::from_proto(request.session_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid session ID format"))?;

    cx.session
        .db()
        .await
        .upsert_shared_thread(share_id, user_id, &request.title, request.thread_data)
        .await?;

    response.send(proto::Ack {})?;

    Ok(())
}

async fn get_shared_agent_thread(
    request: proto::GetSharedAgentThread,
    response: Response<proto::GetSharedAgentThread>,
    cx: MessageContext,
) -> Result<()> {
    let share_id = SharedThreadId::from_proto(request.session_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid session ID format"))?;

    let result = cx
        .session
        .db()
        .await
        .get_shared_thread(share_id)
        .await?;

    match result {
        Some((thread, username)) => {
            response.send(proto::GetSharedAgentThreadResponse {
                title: thread.title,
                thread_data: thread.data,
                sharer_username: username,
                created_at: thread.created_at.assume_utc().format(&time::format_description::well_known::Rfc3339).unwrap_or_default(),
            })?;
        }
        None => {
            return Err(anyhow::anyhow!("Shared thread not found"))?;
        }
    }

    Ok(())
}
```

---

## Phase 5: Client Implementation

### 5.1 Share Function

**File:** `crates/agent_ui/src/acp/thread_view.rs`

Add a method to `AcpThreadView`:

```rust
fn share_thread(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
    // Use as_native_thread to get the agent::Thread which has to_db()
    let Some(thread) = self.as_native_thread(cx) else {
        return;
    };

    let client = self.project.read(cx).client();
    let workspace = self.workspace.clone();

    // Get session_id for the share URL and the upsert
    let session_id = thread.read(cx).session_id().to_string();

    // Use Thread::to_db() directly instead of loading from history store
    let load_task = thread.read(cx).to_db(cx);

    cx.spawn(async move |_this, cx| {
        let db_thread = load_task.await;

        // Convert to shareable format
        let shared_thread = SharedThread::from_db_thread(&db_thread);
        let thread_data = shared_thread.to_bytes()?;
        let title = shared_thread.title.to_string();

        // Send to server (upserts based on session_id)
        client
            .request(proto::ShareAgentThread {
                session_id: session_id.clone(),
                title,
                thread_data,
            })
            .await?;

        // Construct URL using session_id
        let share_url = client::zed_urls::shared_agent_thread_url(&session_id);

        // Show toast with copy button (don't auto-copy)
        cx.update(|cx| {
            if let Some(workspace) = workspace.upgrade() {
                workspace.update(cx, |workspace, cx| {
                    struct ThreadSharedToast;
                    workspace.show_toast(
                        Toast::new(
                            NotificationId::unique::<ThreadSharedToast>(),
                            "Thread shared!",
                        )
                        .on_click(
                            "Copy URL",
                            move |_window, cx| {
                                cx.write_to_clipboard(ClipboardItem::new_string(
                                    share_url.clone(),
                                ));
                            },
                        ),
                        cx,
                    );
                });
            }
        })?;

        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
```

### 5.2 Import Function

**File:** `crates/agent/src/import.rs` (new file) or add to existing module

```rust
use crate::db::{SharedThread, DbThread};
use crate::history_store::HistoryStore;
use acp::SessionId;
use anyhow::Result;
use client::Client;
use gpui::{AsyncApp, Entity};
use rpc::proto;
use std::sync::Arc;

/// Import a shared thread from the server.
/// Uses the same session_id as the source thread to enable syncing.
/// Sets `imported: true` to mark it as an imported thread.
pub async fn import_shared_thread(
    session_id: &str,  // The shared thread's session_id (UUID string)
    client: Arc<Client>,
    history_store: Entity<HistoryStore>,
    cx: &mut AsyncApp,
) -> Result<SessionId> {
    // Fetch from server using the session_id
    let response = client
        .request(proto::GetSharedAgentThread {
            session_id: session_id.to_string(),
        })
        .await?;

    // Deserialize
    let shared_thread = SharedThread::from_bytes(&response.thread_data)?;

    // Convert to local format (adds 🔗 prefix and sets imported: true)
    let db_thread = shared_thread.to_db_thread();

    // Use the same session_id as the source - this enables syncing
    let session_id = SessionId::new(session_id);

    // Save to local history (upserts if already exists)
    history_store
        .update(cx, |store, cx| {
            store.save_acp_thread(session_id.clone(), db_thread, cx)
        })?
        .await?;

    Ok(session_id)
}

/// Sync an imported thread with its source on the server.
/// Returns the updated SharedThread if sync was successful.
pub async fn sync_imported_thread(
    session_id: &str,  // The thread's session_id (same as source)
    client: Arc<Client>,
) -> Result<SharedThread> {
    // Fetch latest version from server
    let response = client
        .request(proto::GetSharedAgentThread {
            session_id: session_id.to_string(),
        })
        .await
        .context("Failed to fetch shared thread - it may have been deleted")?;

    // Deserialize and return
    SharedThread::from_bytes(&response.thread_data)
}
```

---

## Phase 6: UI Changes

### 6.1 Share Button

**File:** `crates/agent_ui/src/acp/thread_view.rs`

Modify `render_thread_controls` to add a share button, gated by feature flag (around line 5656):

```rust
fn render_thread_controls(
    &self,
    thread: &Entity<AcpThread>,
    cx: &Context<Self>,
) -> impl IntoElement {
    let is_generating = matches!(thread.read(cx).status(), ThreadStatus::Generating);
    if is_generating {
        return self.render_generating(false).into_any_element();
    }

    // Existing buttons...
    let open_as_markdown = IconButton::new("open-as-markdown", IconName::FileMarkdown)
        .shape(ui::IconButtonShape::Square)
        .icon_size(IconSize::Small)
        .icon_color(Color::Ignored)
        .tooltip(Tooltip::text("Open Thread as Markdown"))
        .on_click(cx.listener(move |this, _, window, cx| {
            if let Some(workspace) = this.workspace.upgrade() {
                this.open_thread_as_markdown(workspace, window, cx)
                    .detach_and_log_err(cx);
            }
        }));

    // ... other existing buttons ...

    let mut container = h_flex()
        .w_full()
        .py_2()
        .px_5()
        .gap_px()
        .opacity(0.6)
        .hover(|s| s.opacity(1.))
        .justify_end();

    // Add share button if feature flag is enabled
    if cx.has_flag::<AgentSharingFeatureFlag>() {
        let share_button = IconButton::new("share-thread", IconName::Share)
            .shape(ui::IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Share Thread"))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.share_thread(window, cx);
            }));

        container = container.child(share_button);
    }

    container
        .child(open_as_markdown)
        .child(scroll_to_recent_user_prompt)
        .child(scroll_to_top)
        .into_any_element()
}
```

### 6.2 Required Imports

Add to the imports at the top of `thread_view.rs`:

```rust
use feature_flags::{FeatureFlagAppExt, AgentSharingFeatureFlag};
```

### 6.3 Ensure Share Icon Exists

Check if `IconName::Share` exists in `crates/ui/src/components/icon.rs`. If not, use an appropriate existing icon like `IconName::Link` or `IconName::ArrowUpRight`.

---

## Phase 7: URL Handling

### 7.1 OpenRequestKind Variant

**File:** `crates/zed/src/zed/open_listener.rs`

Add new variant to `OpenRequestKind` (around line 56):

```rust
#[derive(Debug)]
pub enum OpenRequestKind {
    // ... existing variants ...
    SharedAgentThread {
        session_id: String,  // UUID string
    },
}
```

### 7.2 URL Parsing

**File:** `crates/zed/src/zed/open_listener.rs`

Add parsing logic in `OpenRequest::parse` (around line 120):

```rust
// After the existing zed://agent check
} else if url == "zed://agent" {
    this.kind = Some(OpenRequestKind::AgentPanel);
} else if let Some(session_id) = url.strip_prefix("zed://agent/shared/") {
    // Handle shared agent thread URLs (session_id is a UUID string)
    // Validate it looks like a UUID before accepting
    if uuid::Uuid::parse_str(session_id).is_ok() {
        this.kind = Some(OpenRequestKind::SharedAgentThread {
            session_id: session_id.to_string(),
        });
    } else {
        log::error!("Invalid session ID in URL: {}", session_id);
    }
} else if let Some(schema_path) = url.strip_prefix("zed://schemas/") {
    // ... existing code ...
```

### 7.3 URL Handler

**File:** `crates/zed/src/main.rs`

Add handler in `handle_open_request` (around line 814):

```rust
OpenRequestKind::SharedAgentThread { session_id } => {
    cx.spawn(async move |cx| {
        let workspace =
            workspace::get_any_active_workspace(app_state.clone(), cx.clone()).await?;

        workspace.update(cx, |workspace, window, cx| {
            let client = workspace.project().read(cx).client();

            // Get the agent panel to access history store
            let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
                return;
            };

            let history_store = panel.read(cx).history_store().clone();

            cx.spawn_in(window, async move |workspace, mut cx| {
                // Import the thread using the session_id from the URL
                let new_session_id = agent::import::import_shared_thread(
                    &session_id,
                    client,
                    history_store.clone(),
                    &mut cx,
                ).await?;

                // Notify history store of the new thread
                history_store.update(&mut cx, |store, cx| {
                    store.refresh(cx);
                })?;

                // Open the imported thread in the agent panel
                workspace.update_in(&mut cx, |workspace, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.open_thread(new_session_id, window, cx);
                        });
                        panel.focus_handle(cx).focus(window, cx);
                    }
                })?;

                anyhow::Ok(())
            }).detach_and_log_err(cx);
        })
    })
    .detach_and_log_err(cx);
}
```

### 7.4 URL Helper

**File:** `crates/client/src/zed_urls.rs`

Add helper function:

```rust
/// Returns the URL for a shared agent thread.
/// Takes the session_id (UUID) which is used as the share identifier.
pub fn shared_agent_thread_url(session_id: &str) -> String {
    format!("zed://agent/shared/{}", session_id)
}
```

---

## Phase 8: Import Tracking & Sync

This phase adds the ability to track which threads were imported and sync them with their source.

### 8.1 Data Layer Changes

**File:** `crates/agent/src/db.rs`

Add the `imported` field to `DbThread`:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct DbThread {
    // ... existing fields ...
    #[serde(default)]
    pub imported: bool,
}
```

Since imported threads use the same session_id as the source, we only need a boolean to track whether syncing is available - the session_id itself serves as the identifier for fetching updates.

### 8.2 Helper Functions

**File:** `crates/agent_ui/src/acp/thread_view.rs`

Add helper to check if a thread is imported:

```rust
impl AcpThreadView {
    fn is_imported_thread(&self, cx: &Context<Self>) -> bool {
        let Some(thread) = self.as_native_thread(cx) else {
            return false;
        };
        thread.read(cx).is_imported()
    }
}
```

**File:** `crates/acp_thread/src/thread.rs`

Add method to expose `imported`:

```rust
impl AcpThread {
    /// Returns true if this thread was imported from a shared thread.
    pub fn is_imported(&self) -> bool {
        self.imported
    }
}
```

### 8.3 Sync Function

**File:** `crates/agent_ui/src/acp/thread_view.rs`

```rust
fn sync_thread(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
    if !self.is_imported_thread(cx) {
        return;
    }

    let Some(thread) = self.as_native_thread(cx) else {
        return;
    };

    let client = self.project.read(cx).client();
    let workspace = self.workspace.clone();
    let thread_entity = thread.clone();
    let history_store = self.history_store.clone();
    let session_id = thread.read(cx).id().clone();

    cx.spawn(async move |_this, cx| {
        // Fetch latest from server using the thread's session_id
        let shared_thread = sync_imported_thread(&session_id.to_string(), client).await?;

        // Update local thread with new content (imported flag preserved by to_db_thread)
        let db_thread = shared_thread.to_db_thread();

        // Save updated thread (same session_id, so it overwrites)
        history_store
            .update(&mut cx.clone(), |store, cx| {
                store.save_thread(session_id.clone(), db_thread, cx)
            })?
            .await?;

        // Reload the thread view
        thread_entity.update(&mut cx.clone(), |thread, cx| {
            thread.reload(cx);
        })?;

        cx.update(|cx| {
            if let Some(workspace) = workspace.upgrade() {
                workspace.update(cx, |workspace, cx| {
                    struct ThreadSyncedToast;
                    workspace.show_toast(
                        Toast::new(
                            NotificationId::unique::<ThreadSyncedToast>(),
                            "Thread synced with latest version",
                        )
                        .autohide(),
                        cx,
                    );
                });
            }
        })?;

        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
```

### 8.4 UI - Sync Button

**File:** `crates/agent_ui/src/acp/thread_view.rs`

Modify `render_thread_controls` to add a sync button for imported threads:

```rust
fn render_thread_controls(
    &self,
    thread: &Entity<AcpThread>,
    cx: &Context<Self>,
) -> impl IntoElement {
    // ... existing code ...

    let mut container = h_flex()
        .w_full()
        .py_2()
        .px_5()
        .gap_px()
        .opacity(0.6)
        .hover(|s| s.opacity(1.))
        .justify_end();

    // Add sync button for imported threads (if connected to collab)
    if self.is_imported_thread(cx) && self.project.read(cx).client().status().is_connected() {
        let sync_button = IconButton::new("sync-thread", IconName::ArrowCircle)
            .shape(ui::IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Sync with source thread"))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.sync_thread(window, cx);
            }));

        container = container.child(sync_button);
    }

    if cx.has_flag::<AgentSharingFeatureFlag>() && !self.is_imported_thread(cx) {
        let share_button = IconButton::new("share-thread", IconName::Share)
            .shape(ui::IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Share Thread"))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.share_thread(window, cx);
            }));

        container = container.child(share_button);
    }

    container
        .child(open_as_markdown)
        .child(scroll_to_recent_user_prompt)
        .child(scroll_to_top)
        .into_any_element()
}
```

**Note:** We hide the share button for imported threads to prevent confusion. Users should share from the original thread, not re-share imports. If this behavior is undesirable, we can allow sharing imported threads (they'll get their own new share URL).

### 8.5 Error Handling

The sync function should handle these error cases gracefully:

1. **Source thread deleted:** Show toast "The source thread is no longer available"
2. **Network error:** Show toast "Failed to sync - check your connection"
3. **Not connected to collab:** Disable sync button (handled in UI)

---

## Testing Strategy

### Unit Tests

1. **Serialization Tests** (`crates/agent/src/db.rs`)
   - Test `SharedThread::from_db_thread` correctly strips fields
   - Test `SharedThread::to_db_thread` correctly adds 🔗 prefix
   - Test `to_bytes`/`from_bytes` round-trip

2. **Database Tests** (`crates/collab/src/db/tests/`)
   - Test `upsert_shared_thread` creates new thread with given UUID
   - Test `upsert_shared_thread` updates existing thread when same user calls again
   - Test `upsert_shared_thread` fails when different user tries to update
   - Test `get_shared_thread` returns correct data
   - Test `get_shared_thread` returns None for invalid UUID

3. **URL Parsing Tests** (`crates/zed/src/zed/open_listener.rs`)
   - Test `zed://agent/shared/<uuid>` parses correctly
   - Test invalid UUIDs are handled gracefully
   - Test non-UUID strings are rejected

### Integration Tests

**File:** `crates/collab/src/tests/agent_sharing_tests.rs`

**Primary Integration Test - Two App Instances:**

```rust
#[gpui::test]
async fn test_share_and_import_thread(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    // Setup: Two connected clients
    let (server, client_a, client_b) = setup_two_clients(cx_a, cx_b).await;

    // Client A creates a thread with some content
    let session_id = acp::SessionId::new(uuid::Uuid::new_v4().to_string());
    let original_thread = create_test_thread("My Test Thread", vec![
        test_message(Role::User, "Hello, world!"),
        test_message(Role::Assistant, "Hello! How can I help you?"),
    ]);

    // Client A shares the thread (uses session_id as the share identifier)
    client_a
        .share_thread(&session_id, &original_thread)
        .await
        .expect("Failed to share thread");

    // Client B imports the thread via session_id
    let imported_session_id = client_b
        .import_shared_thread(&session_id.to_string())
        .await
        .expect("Failed to import thread");

    // Verify the imported thread uses the same session_id
    assert_eq!(imported_session_id.to_string(), session_id.to_string());

    // Verify the imported thread
    let imported_thread = client_b
        .load_thread(&imported_session_id)
        .await
        .expect("Failed to load imported thread");

    // Check title has 🔗 prefix
    assert!(imported_thread.title.starts_with("🔗 "));
    assert!(imported_thread.title.contains("My Test Thread"));

    // Check messages match
    assert_eq!(imported_thread.messages.len(), original_thread.messages.len());
    for (imported, original) in imported_thread.messages.iter().zip(&original_thread.messages) {
        assert_eq!(imported.role, original.role);
        // Compare message content...
    }
}

#[gpui::test]
async fn test_reshare_updates_existing(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    // Setup: Two connected clients
    let (server, client_a, client_b) = setup_two_clients(cx_a, cx_b).await;

    let session_id = acp::SessionId::new(uuid::Uuid::new_v4().to_string());

    // Client A shares a thread
    let original_thread = create_test_thread("Original Title", vec![
        test_message(Role::User, "First message"),
    ]);
    client_a.share_thread(&session_id, &original_thread).await.unwrap();

    // Client A updates and re-shares the same thread
    let updated_thread = create_test_thread("Updated Title", vec![
        test_message(Role::User, "First message"),
        test_message(Role::Assistant, "Response added"),
    ]);
    client_a.share_thread(&session_id, &updated_thread).await.unwrap();

    // Client B imports - should get the updated version
    let imported_session_id = client_b
        .import_shared_thread(&session_id.to_string())
        .await
        .unwrap();

    let imported_thread = client_b.load_thread(&imported_session_id).await.unwrap();

    assert!(imported_thread.title.contains("Updated Title"));
    assert_eq!(imported_thread.messages.len(), 2);
}

#[gpui::test]
async fn test_imported_thread_has_imported_flag(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    // Setup: Client A shares, Client B imports
    // Verify: Imported thread has imported: true
    // Verify: Imported thread has the SAME session_id as source
}

#[gpui::test]
async fn test_sync_imported_thread(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    // Setup: Client A shares a thread, Client B imports it
    // Client A adds more messages and re-shares
    // Client B syncs
    // Verify: Client B's thread now has the new messages
    // Verify: Client B's session_id is unchanged (same as source)
    // Verify: imported flag is still true
}

#[gpui::test]
async fn test_sync_deleted_source_thread(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    // Setup: Client A shares a thread, Client B imports it
    // Client A deletes the shared thread (or server removes it)
    // Client B attempts to sync
    // Verify: Appropriate error is returned
    // Verify: Local thread remains unchanged
}

#[gpui::test]
async fn test_import_nonexistent_thread(cx: &mut TestAppContext) {
    let (server, client) = setup_client(cx).await;

    // Try to import a thread with a random UUID that doesn't exist
    let fake_session_id = uuid::Uuid::new_v4().to_string();
    let result = client.import_shared_thread(&fake_session_id).await;

    assert!(result.is_err());
    // Verify error message indicates thread not found
}
```

---

## Security Considerations

### Access Control

- **No Authentication Required to View:** Shared threads are public by link (similar to Google Docs "anyone with link")
- **Rate Limiting:** Consider adding rate limits on:
  - Share creation (e.g., 100 shares per user per day)
  - Share retrieval (e.g., 1000 requests per IP per hour)

### Data Privacy

- **Stripped Data:** `SharedThread` intentionally omits:
  - Project snapshots (may contain file paths/contents)
  - Token usage (billing information)
  - User profiles
- **No Edit Access:** Importing creates a copy; original is never modified
- **Visual Indicator:** The 🔗 prefix clearly marks imported threads

### Content Moderation

- Consider adding (future enhancement):
  - Abuse reporting mechanism
  - Admin tools to view/delete reported shares
  - Terms of service for shared content

---

## Future Enhancements

### Phase 2 Features

1. **Share Management UI**
   - View all shares you've created
   - Delete your own shares
   - See access statistics

2. **Share Options**
   - Set expiration date
   - Password protection
   - Limit number of accesses

3. **Social Features**
   - Preview cards when sharing on social media
   - "Fork" count for popular shares

4. **Enhanced Sync Features**
   - Show visual diff before syncing if local modifications exist
   - Merge non-conflicting changes automatically
   - "Last synced" timestamp display
   - Auto-sync option for imported threads
   - Indicator showing when source thread has updates available

### Phase 3 Features

1. **Collections/Galleries**
   - Curated collections of shared threads
   - Search/discover public threads
   - "Featured" threads

2. **Team Sharing**
   - Share within a team/organization
   - Access control lists

---

## Implementation Checklist

- [x] **Phase 1: Feature Flag**
  - [x] Add `AgentSharingFeatureFlag` to `crates/feature_flags/src/flags.rs`

- [x] **Phase 2: Data Layer**
  - [x] Add `SharedThread` struct to `crates/agent/src/db.rs`
  - [x] Add `SharedThreadId` (UUID-based) to `crates/collab/src/db/ids.rs`
  - [x] Create database migration with UUID primary key and `updated_at` column
  - [x] Add `shared_thread` table entity
  - [x] Implement `upsert_shared_thread` query (insert or update based on session_id)

- [x] **Phase 3: RPC Protocol**
  - [x] Add proto messages to `zed.proto` (`ShareAgentThread` with session_id, uses `Ack` response)
  - [x] Run proto code generation

- [x] **Phase 4: Server Implementation**
  - [x] Add RPC handlers to `rpc.rs` (upsert logic, ownership check)
  - [x] Add handlers to `Server::new()`

- [x] **Phase 5: Client Implementation**
  - [x] Add `share_thread` method using `Thread::to_db()` and session_id
  - [x] Add `import_shared_thread` function
  - [x] Update toast to show "Copy URL" button instead of auto-copying

- [x] **Phase 6: UI Changes**
  - [x] Add share button to thread controls (gated by feature flag)
  - [x] Add share icon if needed

- [x] **Phase 7: URL Handling**
  - [x] Add `SharedAgentThread { session_id: String }` variant to `OpenRequestKind`
  - [x] Add URL parsing logic for UUID-based session_ids in `open_listener.rs`
  - [x] Add URL handler in `main.rs`
  - [x] Update `shared_agent_thread_url` to take `&str` session_id

- [ ] **Phase 8: Import Tracking & Sync**
  - [ ] Add `imported: bool` field to `DbThread` (defaults to false)
  - [ ] Update `SharedThread::to_db_thread()` to set `imported: true`
  - [ ] Add `is_imported()` method to `AcpThread`
  - [ ] Add `sync_imported_thread` function
  - [ ] Add `is_imported_thread` helper in `AcpThreadView`
  - [ ] Add sync button to thread controls for imported threads
  - [ ] Handle sync errors gracefully (deleted source thread, network issues)
  - [ ] Only show sync button when connected to collab

- [ ] **Testing (TDD - Write Tests First!)**
  - [x] Integration test: two app instances share/import flow
  - [x] Integration test: re-sharing same thread updates existing record
  - [x] Unit tests for `SharedThread` serialization
  - [x] Unit tests for `upsert_shared_thread` (create, update, ownership check)
  - [x] Unit tests for URL parsing with UUID validation
  - [x] Test imported thread has 🔗 prefix
  - [ ] Test imported thread has `imported: true`
  - [ ] Test imported thread has same session_id as source
  - [ ] Integration test: sync imported thread gets latest version
  - [ ] Integration test: sync fails gracefully for deleted source thread
  - [ ] Unit test: `imported` field is preserved across save/load

- [ ] **Documentation**
  - [ ] Update user documentation

---

## Dependencies

This feature depends on:
- User authentication (existing)
- Client-server RPC (existing)
- URL handler infrastructure (existing)
- Agent panel and thread view (existing)
- History store (existing)
- Feature flags system (existing)

No new external dependencies required.
