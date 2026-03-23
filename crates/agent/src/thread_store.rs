use crate::{DbThread, DbThreadMetadata, Message, ThreadsDatabase};
use acp_thread::UserMessageId;
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use chrono::Utc;
use gpui::{App, Context, Entity, Global, SharedString, Task, prelude::*};
use util::path_list::PathList;

struct GlobalThreadStore(Entity<ThreadStore>);

impl Global for GlobalThreadStore {}

pub struct ThreadStore {
    threads: Vec<DbThreadMetadata>,
}

impl ThreadStore {
    pub fn init_global(cx: &mut App) {
        let thread_store = cx.new(|cx| Self::new(cx));
        cx.set_global(GlobalThreadStore(thread_store));
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalThreadStore>().0.clone()
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalThreadStore>().map(|g| g.0.clone())
    }

    pub fn new(cx: &mut Context<Self>) -> Self {
        let this = Self {
            threads: Vec::new(),
        };
        this.reload(cx);
        this
    }

    pub fn thread_from_session_id(&self, session_id: &acp::SessionId) -> Option<&DbThreadMetadata> {
        self.threads.iter().find(|thread| &thread.id == session_id)
    }

    pub fn load_thread(
        &mut self,
        id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<DbThread>>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.background_spawn(async move {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.load_thread(id).await
        })
    }

    pub fn save_thread(
        &mut self,
        id: acp::SessionId,
        thread: crate::DbThread,
        folder_paths: PathList,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.save_thread(id, thread, folder_paths).await?;
            this.update(cx, |this, cx| this.reload(cx))
        })
    }

    /// Forks a thread at the given user message, creating a new thread
    /// with conversation history up to (but not including) that message.
    pub fn fork_thread(
        &mut self,
        source_id: acp::SessionId,
        message_id: UserMessageId,
        cx: &mut Context<Self>,
    ) -> Task<Result<(acp::SessionId, SharedString)>> {
        let folder_paths = self
            .thread_from_session_id(&source_id)
            .map(|meta| meta.folder_paths.clone())
            .unwrap_or_default();

        let database_future = ThreadsDatabase::connect(cx);

        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;

            let mut thread = database
                .load_thread(source_id)
                .await?
                .ok_or_else(|| anyhow!("Thread not found"))?;

            let position = thread
                .messages
                .iter()
                .position(|msg| {
                    matches!(msg, Message::User(user_msg) if user_msg.id == message_id)
                })
                .ok_or_else(|| anyhow!("Message not found"))?;

            // Remove token usage entries for messages being dropped.
            for msg in &thread.messages[position..] {
                if let Message::User(user_msg) = msg {
                    thread.request_token_usage.remove(&user_msg.id);
                }
            }

            thread.messages.truncate(position);

            let new_id = acp::SessionId::new(uuid::Uuid::new_v4().to_string());

            thread.title = SharedString::from(format!("Fork of {}", thread.title));
            thread.updated_at = Utc::now();
            thread.detailed_summary = None;
            thread.draft_prompt = None;
            thread.ui_scroll_position = None;

            let title = thread.title.clone();

            database
                .save_thread(new_id.clone(), thread, folder_paths)
                .await?;

            this.update(cx, |this, cx| this.reload(cx))?;

            Ok((new_id, title))
        })
    }

    pub fn delete_thread(
        &mut self,
        id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.delete_thread(id.clone()).await?;
            this.update(cx, |this, cx| this.reload(cx))
        })
    }

    pub fn delete_threads(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.delete_threads().await?;
            this.update(cx, |this, cx| this.reload(cx))
        })
    }

    pub fn reload(&self, cx: &mut Context<Self>) {
        let database_connection = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_connection.await.map_err(|err| anyhow!(err))?;
            let all_threads = database.list_threads().await?;
            this.update(cx, |this, cx| {
                this.threads.clear();
                for thread in all_threads {
                    if thread.parent_session_id.is_some() {
                        continue;
                    }
                    this.threads.push(thread);
                }
                cx.notify();
            })
        })
        .detach_and_log_err(cx);
    }

    pub fn is_empty(&self) -> bool {
        self.threads.is_empty()
    }

    pub fn entries(&self) -> impl Iterator<Item = DbThreadMetadata> + '_ {
        self.threads.iter().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};
    use collections::HashMap;
    use gpui::TestAppContext;
    use std::sync::Arc;

    fn session_id(value: &str) -> acp::SessionId {
        acp::SessionId::new(Arc::<str>::from(value))
    }

    fn make_thread(title: &str, updated_at: DateTime<Utc>) -> DbThread {
        DbThread {
            title: title.to_string().into(),
            messages: Vec::new(),
            updated_at,
            detailed_summary: None,
            initial_project_snapshot: None,
            cumulative_token_usage: Default::default(),
            request_token_usage: HashMap::default(),
            model: None,
            profile: None,
            imported: false,
            subagent_context: None,
            speed: None,
            thinking_enabled: false,
            thinking_effort: None,
            draft_prompt: None,
            ui_scroll_position: None,
        }
    }

    fn make_thread_with_messages(
        title: &str,
        updated_at: DateTime<Utc>,
        messages: Vec<Message>,
    ) -> DbThread {
        let mut thread = make_thread(title, updated_at);
        thread.messages = messages;
        thread
    }

    fn user_message() -> (UserMessageId, Message) {
        let id = UserMessageId::new();
        let msg = Message::User(crate::UserMessage {
            id: id.clone(),
            content: vec![crate::UserMessageContent::Text("user text".to_string())],
        });
        (id, msg)
    }

    fn agent_message() -> Message {
        Message::Agent(crate::AgentMessage {
            content: vec![crate::AgentMessageContent::Text("agent text".to_string())],
            tool_results: Default::default(),
            reasoning_details: None,
        })
    }

    #[gpui::test]
    async fn test_entries_are_sorted_by_updated_at(cx: &mut TestAppContext) {
        let thread_store = cx.new(|cx| ThreadStore::new(cx));
        cx.run_until_parked();

        let older_id = session_id("thread-a");
        let newer_id = session_id("thread-b");

        let older_thread = make_thread(
            "Thread A",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        let newer_thread = make_thread(
            "Thread B",
            Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap(),
        );

        let save_older = thread_store.update(cx, |store, cx| {
            store.save_thread(older_id.clone(), older_thread, PathList::default(), cx)
        });
        save_older.await.unwrap();

        let save_newer = thread_store.update(cx, |store, cx| {
            store.save_thread(newer_id.clone(), newer_thread, PathList::default(), cx)
        });
        save_newer.await.unwrap();

        cx.run_until_parked();

        let entries: Vec<_> = thread_store.read_with(cx, |store, _cx| store.entries().collect());
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id, newer_id);
        assert_eq!(entries[1].id, older_id);
    }

    #[gpui::test]
    async fn test_delete_threads_clears_entries(cx: &mut TestAppContext) {
        let thread_store = cx.new(|cx| ThreadStore::new(cx));
        cx.run_until_parked();

        let thread_id = session_id("thread-a");
        let thread = make_thread(
            "Thread A",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );

        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(thread_id, thread, PathList::default(), cx)
        });
        save_task.await.unwrap();

        cx.run_until_parked();
        assert!(!thread_store.read_with(cx, |store, _cx| store.is_empty()));

        let delete_task = thread_store.update(cx, |store, cx| store.delete_threads(cx));
        delete_task.await.unwrap();
        cx.run_until_parked();

        assert!(thread_store.read_with(cx, |store, _cx| store.is_empty()));
    }

    #[gpui::test]
    async fn test_delete_thread_removes_only_target(cx: &mut TestAppContext) {
        let thread_store = cx.new(|cx| ThreadStore::new(cx));
        cx.run_until_parked();

        let first_id = session_id("thread-a");
        let second_id = session_id("thread-b");

        let first_thread = make_thread(
            "Thread A",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        let second_thread = make_thread(
            "Thread B",
            Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap(),
        );

        let save_first = thread_store.update(cx, |store, cx| {
            store.save_thread(first_id.clone(), first_thread, PathList::default(), cx)
        });
        save_first.await.unwrap();
        let save_second = thread_store.update(cx, |store, cx| {
            store.save_thread(second_id.clone(), second_thread, PathList::default(), cx)
        });
        save_second.await.unwrap();
        cx.run_until_parked();

        let delete_task =
            thread_store.update(cx, |store, cx| store.delete_thread(first_id.clone(), cx));
        delete_task.await.unwrap();
        cx.run_until_parked();

        let entries: Vec<_> = thread_store.read_with(cx, |store, _cx| store.entries().collect());
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, second_id);
    }

    #[gpui::test]
    async fn test_save_thread_refreshes_ordering(cx: &mut TestAppContext) {
        let thread_store = cx.new(|cx| ThreadStore::new(cx));
        cx.run_until_parked();

        let first_id = session_id("thread-a");
        let second_id = session_id("thread-b");

        let first_thread = make_thread(
            "Thread A",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        let second_thread = make_thread(
            "Thread B",
            Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap(),
        );

        let save_first = thread_store.update(cx, |store, cx| {
            store.save_thread(first_id.clone(), first_thread, PathList::default(), cx)
        });
        save_first.await.unwrap();
        let save_second = thread_store.update(cx, |store, cx| {
            store.save_thread(second_id.clone(), second_thread, PathList::default(), cx)
        });
        save_second.await.unwrap();
        cx.run_until_parked();

        let updated_first = make_thread(
            "Thread A",
            Utc.with_ymd_and_hms(2024, 1, 3, 0, 0, 0).unwrap(),
        );
        let update_task = thread_store.update(cx, |store, cx| {
            store.save_thread(first_id.clone(), updated_first, PathList::default(), cx)
        });
        update_task.await.unwrap();
        cx.run_until_parked();

        let entries: Vec<_> = thread_store.read_with(cx, |store, _cx| store.entries().collect());
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id, first_id);
        assert_eq!(entries[1].id, second_id);
    }

    #[gpui::test]
    async fn test_fork_thread_creates_truncated_copy(cx: &mut TestAppContext) {
        let thread_store = cx.new(|cx| ThreadStore::new(cx));
        cx.run_until_parked();

        let original_id = session_id("thread-a");
        let (_, msg1) = user_message();
        let (_, msg2) = user_message();
        let (msg3_id, msg3) = user_message();
        let messages = vec![
            msg1,
            agent_message(),
            msg2,
            agent_message(),
            msg3,
            agent_message(),
        ];
        let original_thread = make_thread_with_messages(
            "My Thread",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            messages,
        );

        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(original_id.clone(), original_thread, PathList::default(), cx)
        });
        save_task.await.unwrap();
        cx.run_until_parked();

        // Fork at msg3, which should keep messages before it (msg1, response, msg2, response).
        let (new_id, new_title) = thread_store
            .update(cx, |store, cx| {
                store.fork_thread(original_id.clone(), msg3_id, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();

        assert_ne!(new_id, original_id);
        assert_eq!(new_title.as_ref(), "Fork of My Thread");

        let entries: Vec<_> = thread_store.read_with(cx, |store, _cx| store.entries().collect());
        assert_eq!(entries.len(), 2);
        // The fork has a newer updated_at, so it should sort first.
        assert_eq!(entries[0].id, new_id);
        assert_eq!(entries[1].id, original_id);

        // Load the forked thread and verify it has only the first 4 messages.
        let forked = thread_store
            .update(cx, |store, cx| store.load_thread(new_id, cx))
            .await
            .unwrap()
            .expect("forked thread should exist");

        assert_eq!(forked.messages.len(), 4);
        assert_eq!(forked.title.as_ref(), "Fork of My Thread");
    }

    #[gpui::test]
    async fn test_fork_thread_preserves_original(cx: &mut TestAppContext) {
        let thread_store = cx.new(|cx| ThreadStore::new(cx));
        cx.run_until_parked();

        let original_id = session_id("thread-a");
        let original_time = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();
        let (_, msg1) = user_message();
        let (msg2_id, msg2) = user_message();
        let messages = vec![
            msg1,
            agent_message(),
            msg2,
            agent_message(),
        ];
        let original_thread =
            make_thread_with_messages("Important Chat", original_time, messages);

        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(original_id.clone(), original_thread, PathList::default(), cx)
        });
        save_task.await.unwrap();
        cx.run_until_parked();

        // Fork at msg2, keeping only msg1 and its response.
        let (new_id, _) = thread_store
            .update(cx, |store, cx| {
                store.fork_thread(original_id.clone(), msg2_id, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();

        // Load both threads and verify the original is untouched.
        let original_loaded = thread_store
            .update(cx, |store, cx| store.load_thread(original_id.clone(), cx))
            .await
            .unwrap()
            .expect("original thread should exist");
        let forked_loaded = thread_store
            .update(cx, |store, cx| store.load_thread(new_id, cx))
            .await
            .unwrap()
            .expect("forked thread should exist");

        assert_eq!(original_loaded.title.as_ref(), "Important Chat");
        assert_eq!(original_loaded.updated_at, original_time);
        assert_eq!(original_loaded.messages.len(), 4);

        assert_eq!(forked_loaded.title.as_ref(), "Fork of Important Chat");
        assert!(forked_loaded.updated_at > original_time);
        assert_eq!(forked_loaded.messages.len(), 2);
    }
}
