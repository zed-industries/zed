use crate::{DbThread, DbThreadMetadata, ThreadsDatabase};
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use gpui::{App, Context, Entity, Task, prelude::*};
use project::Project;
use std::rc::Rc;

// TODO: Remove once ACP thread loading is fully handled elsewhere.
pub fn load_agent_thread(
    session_id: acp::SessionId,
    thread_store: Entity<ThreadStore>,
    project: Entity<Project>,
    cx: &mut App,
) -> Task<Result<Entity<crate::Thread>>> {
    use agent_servers::{AgentServer, AgentServerDelegate};

    let server = Rc::new(crate::NativeAgentServer::new(
        project.read(cx).fs().clone(),
        thread_store,
    ));
    let delegate = AgentServerDelegate::new(
        project.read(cx).agent_server_store().clone(),
        project.clone(),
        None,
        None,
    );
    let connection = server.connect(None, delegate, cx);
    cx.spawn(async move |cx| {
        let (agent, _) = connection.await?;
        let agent = agent.downcast::<crate::NativeAgentConnection>().unwrap();
        cx.update(|cx| agent.load_thread(session_id, cx)).await
    })
}

pub struct ThreadStore {
    threads: Vec<DbThreadMetadata>,
}

impl ThreadStore {
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
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.save_thread(id, thread).await?;
            this.update(cx, |this, cx| this.reload(cx))
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
            let threads = database.list_threads().await?;
            this.update(cx, |this, cx| {
                this.threads = threads;
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
            completion_mode: None,
            profile: None,
            imported: false,
        }
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
            store.save_thread(older_id.clone(), older_thread, cx)
        });
        save_older.await.unwrap();

        let save_newer = thread_store.update(cx, |store, cx| {
            store.save_thread(newer_id.clone(), newer_thread, cx)
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

        let save_task =
            thread_store.update(cx, |store, cx| store.save_thread(thread_id, thread, cx));
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
            store.save_thread(first_id.clone(), first_thread, cx)
        });
        save_first.await.unwrap();
        let save_second = thread_store.update(cx, |store, cx| {
            store.save_thread(second_id.clone(), second_thread, cx)
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
            store.save_thread(first_id.clone(), first_thread, cx)
        });
        save_first.await.unwrap();
        let save_second = thread_store.update(cx, |store, cx| {
            store.save_thread(second_id.clone(), second_thread, cx)
        });
        save_second.await.unwrap();
        cx.run_until_parked();

        let updated_first = make_thread(
            "Thread A",
            Utc.with_ymd_and_hms(2024, 1, 3, 0, 0, 0).unwrap(),
        );
        let update_task = thread_store.update(cx, |store, cx| {
            store.save_thread(first_id.clone(), updated_first, cx)
        });
        update_task.await.unwrap();
        cx.run_until_parked();

        let entries: Vec<_> = thread_store.read_with(cx, |store, _cx| store.entries().collect());
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id, first_id);
        assert_eq!(entries[1].id, second_id);
    }
}
