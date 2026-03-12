use acp_thread::{AgentSessionInfo, AgentSessionList, AgentSessionListRequest, SessionListUpdate};
use agent_client_protocol as acp;
use gpui::{App, Task};
use std::rc::Rc;
use ui::prelude::*;

pub struct ThreadHistory {
    session_list: Option<Rc<dyn AgentSessionList>>,
    sessions: Vec<AgentSessionInfo>,
    _refresh_task: Task<()>,
    _watch_task: Option<Task<()>>,
}

impl ThreadHistory {
    pub fn new(session_list: Option<Rc<dyn AgentSessionList>>, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            session_list: None,
            sessions: Vec::new(),
            _refresh_task: Task::ready(()),
            _watch_task: None,
        };
        this.set_session_list(session_list, cx);
        this
    }

    pub fn set_session_list(
        &mut self,
        session_list: Option<Rc<dyn AgentSessionList>>,
        cx: &mut Context<Self>,
    ) {
        if let (Some(current), Some(next)) = (&self.session_list, &session_list)
            && Rc::ptr_eq(current, next)
        {
            return;
        }

        self.session_list = session_list;
        self.sessions.clear();
        self._refresh_task = Task::ready(());

        let Some(session_list) = self.session_list.as_ref() else {
            self._watch_task = None;
            cx.notify();
            return;
        };
        let Some(rx) = session_list.watch(cx) else {
            self._watch_task = None;
            self.refresh_sessions(false, cx);
            return;
        };
        session_list.notify_refresh();

        self._watch_task = Some(cx.spawn(async move |this, cx| {
            while let Ok(first_update) = rx.recv().await {
                let mut updates = vec![first_update];
                while let Ok(update) = rx.try_recv() {
                    updates.push(update);
                }

                this.update(cx, |this, cx| {
                    let needs_refresh = updates
                        .iter()
                        .any(|u| matches!(u, SessionListUpdate::Refresh));

                    if needs_refresh {
                        this.refresh_sessions(false, cx);
                    } else {
                        for update in updates {
                            if let SessionListUpdate::SessionInfo { session_id, update } = update {
                                this.apply_info_update(session_id, update, cx);
                            }
                        }
                    }
                })
                .ok();
            }
        }));
    }

    pub(crate) fn refresh_full_history(&mut self, cx: &mut Context<Self>) {
        self.refresh_sessions(true, cx);
    }

    fn apply_info_update(
        &mut self,
        session_id: acp::SessionId,
        info_update: acp::SessionInfoUpdate,
        cx: &mut Context<Self>,
    ) {
        let Some(session) = self
            .sessions
            .iter_mut()
            .find(|s| s.session_id == session_id)
        else {
            return;
        };

        match info_update.title {
            acp::MaybeUndefined::Value(title) => {
                session.title = Some(title.into());
            }
            acp::MaybeUndefined::Null => {
                session.title = None;
            }
            acp::MaybeUndefined::Undefined => {}
        }
        match info_update.updated_at {
            acp::MaybeUndefined::Value(date_str) => {
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str) {
                    session.updated_at = Some(dt.with_timezone(&chrono::Utc));
                }
            }
            acp::MaybeUndefined::Null => {
                session.updated_at = None;
            }
            acp::MaybeUndefined::Undefined => {}
        }
        if let Some(meta) = info_update.meta {
            session.meta = Some(meta);
        }

        cx.notify();
    }

    fn refresh_sessions(&mut self, load_all_pages: bool, cx: &mut Context<Self>) {
        let Some(session_list) = self.session_list.clone() else {
            cx.notify();
            return;
        };

        self._refresh_task = cx.spawn(async move |this, cx| {
            let mut cursor: Option<String> = None;
            let mut is_first_page = true;

            loop {
                let request = AgentSessionListRequest {
                    cursor: cursor.clone(),
                    ..Default::default()
                };
                let task = cx.update(|cx| session_list.list_sessions(request, cx));
                let response = match task.await {
                    Ok(response) => response,
                    Err(error) => {
                        log::error!("Failed to load session history: {error:#}");
                        return;
                    }
                };

                let acp_thread::AgentSessionListResponse {
                    sessions: page_sessions,
                    next_cursor,
                    ..
                } = response;

                this.update(cx, |this, cx| {
                    if is_first_page {
                        this.sessions = page_sessions;
                    } else {
                        this.sessions.extend(page_sessions);
                    }
                    cx.notify();
                })
                .ok();

                is_first_page = false;
                if !load_all_pages {
                    break;
                }

                match next_cursor {
                    Some(next_cursor) => {
                        if cursor.as_ref() == Some(&next_cursor) {
                            log::warn!(
                                "Session list pagination returned the same cursor; stopping to avoid a loop."
                            );
                            break;
                        }
                        cursor = Some(next_cursor);
                    }
                    None => break,
                }
            }
        });
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    pub fn has_session_list(&self) -> bool {
        self.session_list.is_some()
    }

    pub fn refresh(&mut self, _cx: &mut Context<Self>) {
        if let Some(session_list) = &self.session_list {
            session_list.notify_refresh();
        }
    }

    pub fn session_for_id(&self, session_id: &acp::SessionId) -> Option<AgentSessionInfo> {
        self.sessions
            .iter()
            .find(|entry| &entry.session_id == session_id)
            .cloned()
    }

    pub(crate) fn sessions(&self) -> &[AgentSessionInfo] {
        &self.sessions
    }

    pub(crate) fn get_recent_sessions(&self, limit: usize) -> Vec<AgentSessionInfo> {
        self.sessions.iter().take(limit).cloned().collect()
    }

    pub fn supports_delete(&self) -> bool {
        self.session_list
            .as_ref()
            .map(|sl| sl.supports_delete())
            .unwrap_or(false)
    }

    pub(crate) fn delete_session(
        &self,
        session_id: &acp::SessionId,
        cx: &mut App,
    ) -> Task<anyhow::Result<()>> {
        if let Some(session_list) = self.session_list.as_ref() {
            session_list.delete_session(session_id, cx)
        } else {
            Task::ready(Ok(()))
        }
    }

    pub(crate) fn delete_sessions(&self, cx: &mut App) -> Task<anyhow::Result<()>> {
        if let Some(session_list) = self.session_list.as_ref() {
            session_list.delete_sessions(cx)
        } else {
            Task::ready(Ok(()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::AgentSessionListResponse;
    use gpui::TestAppContext;
    use std::{
        any::Any,
        sync::{Arc, Mutex},
    };

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
    }

    #[derive(Clone)]
    struct TestSessionList {
        sessions: Vec<AgentSessionInfo>,
        updates_tx: smol::channel::Sender<SessionListUpdate>,
        updates_rx: smol::channel::Receiver<SessionListUpdate>,
    }

    impl TestSessionList {
        fn new(sessions: Vec<AgentSessionInfo>) -> Self {
            let (tx, rx) = smol::channel::unbounded();
            Self {
                sessions,
                updates_tx: tx,
                updates_rx: rx,
            }
        }

        fn send_update(&self, update: SessionListUpdate) {
            self.updates_tx.try_send(update).ok();
        }
    }

    impl AgentSessionList for TestSessionList {
        fn list_sessions(
            &self,
            _request: AgentSessionListRequest,
            _cx: &mut App,
        ) -> Task<anyhow::Result<AgentSessionListResponse>> {
            Task::ready(Ok(AgentSessionListResponse::new(self.sessions.clone())))
        }

        fn watch(&self, _cx: &mut App) -> Option<smol::channel::Receiver<SessionListUpdate>> {
            Some(self.updates_rx.clone())
        }

        fn notify_refresh(&self) {
            self.send_update(SessionListUpdate::Refresh);
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    #[derive(Clone)]
    struct PaginatedTestSessionList {
        first_page_sessions: Vec<AgentSessionInfo>,
        second_page_sessions: Vec<AgentSessionInfo>,
        requested_cursors: Arc<Mutex<Vec<Option<String>>>>,
        async_responses: bool,
        updates_tx: smol::channel::Sender<SessionListUpdate>,
        updates_rx: smol::channel::Receiver<SessionListUpdate>,
    }

    impl PaginatedTestSessionList {
        fn new(
            first_page_sessions: Vec<AgentSessionInfo>,
            second_page_sessions: Vec<AgentSessionInfo>,
        ) -> Self {
            let (tx, rx) = smol::channel::unbounded();
            Self {
                first_page_sessions,
                second_page_sessions,
                requested_cursors: Arc::new(Mutex::new(Vec::new())),
                async_responses: false,
                updates_tx: tx,
                updates_rx: rx,
            }
        }

        fn with_async_responses(mut self) -> Self {
            self.async_responses = true;
            self
        }

        fn requested_cursors(&self) -> Vec<Option<String>> {
            self.requested_cursors.lock().unwrap().clone()
        }

        fn clear_requested_cursors(&self) {
            self.requested_cursors.lock().unwrap().clear()
        }

        fn send_update(&self, update: SessionListUpdate) {
            self.updates_tx.try_send(update).ok();
        }
    }

    impl AgentSessionList for PaginatedTestSessionList {
        fn list_sessions(
            &self,
            request: AgentSessionListRequest,
            cx: &mut App,
        ) -> Task<anyhow::Result<AgentSessionListResponse>> {
            let requested_cursors = self.requested_cursors.clone();
            let first_page_sessions = self.first_page_sessions.clone();
            let second_page_sessions = self.second_page_sessions.clone();

            let respond = move || {
                requested_cursors
                    .lock()
                    .unwrap()
                    .push(request.cursor.clone());

                match request.cursor.as_deref() {
                    None => AgentSessionListResponse {
                        sessions: first_page_sessions,
                        next_cursor: Some("page-2".to_string()),
                        meta: None,
                    },
                    Some("page-2") => AgentSessionListResponse::new(second_page_sessions),
                    _ => AgentSessionListResponse::new(Vec::new()),
                }
            };

            if self.async_responses {
                cx.foreground_executor().spawn(async move {
                    smol::future::yield_now().await;
                    Ok(respond())
                })
            } else {
                Task::ready(Ok(respond()))
            }
        }

        fn watch(&self, _cx: &mut App) -> Option<smol::channel::Receiver<SessionListUpdate>> {
            Some(self.updates_rx.clone())
        }

        fn notify_refresh(&self) {
            self.send_update(SessionListUpdate::Refresh);
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    fn test_session(session_id: &str, title: &str) -> AgentSessionInfo {
        AgentSessionInfo {
            session_id: acp::SessionId::new(session_id),
            cwd: None,
            title: Some(title.to_string().into()),
            updated_at: None,
            created_at: None,
            meta: None,
        }
    }

    #[gpui::test]
    async fn test_refresh_only_loads_first_page_by_default(cx: &mut TestAppContext) {
        init_test(cx);

        let session_list = Rc::new(PaginatedTestSessionList::new(
            vec![test_session("session-1", "First")],
            vec![test_session("session-2", "Second")],
        ));

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            assert_eq!(history.sessions.len(), 1);
            assert_eq!(
                history.sessions[0].session_id,
                acp::SessionId::new("session-1")
            );
        });
        assert_eq!(session_list.requested_cursors(), vec![None]);
    }

    #[gpui::test]
    async fn test_enabling_full_pagination_loads_all_pages(cx: &mut TestAppContext) {
        init_test(cx);

        let session_list = Rc::new(PaginatedTestSessionList::new(
            vec![test_session("session-1", "First")],
            vec![test_session("session-2", "Second")],
        ));

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();
        session_list.clear_requested_cursors();

        history.update(cx, |history, cx| history.refresh_full_history(cx));
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            assert_eq!(history.sessions.len(), 2);
            assert_eq!(
                history.sessions[0].session_id,
                acp::SessionId::new("session-1")
            );
            assert_eq!(
                history.sessions[1].session_id,
                acp::SessionId::new("session-2")
            );
        });
        assert_eq!(
            session_list.requested_cursors(),
            vec![None, Some("page-2".to_string())]
        );
    }

    #[gpui::test]
    async fn test_standard_refresh_replaces_with_first_page_after_full_history_refresh(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let session_list = Rc::new(PaginatedTestSessionList::new(
            vec![test_session("session-1", "First")],
            vec![test_session("session-2", "Second")],
        ));

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();

        history.update(cx, |history, cx| history.refresh_full_history(cx));
        cx.run_until_parked();
        session_list.clear_requested_cursors();

        history.update(cx, |history, cx| {
            history.refresh(cx);
        });
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            assert_eq!(history.sessions.len(), 1);
            assert_eq!(
                history.sessions[0].session_id,
                acp::SessionId::new("session-1")
            );
        });
        assert_eq!(session_list.requested_cursors(), vec![None]);
    }

    #[gpui::test]
    async fn test_re_entering_full_pagination_reloads_all_pages(cx: &mut TestAppContext) {
        init_test(cx);

        let session_list = Rc::new(PaginatedTestSessionList::new(
            vec![test_session("session-1", "First")],
            vec![test_session("session-2", "Second")],
        ));

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();

        history.update(cx, |history, cx| history.refresh_full_history(cx));
        cx.run_until_parked();
        session_list.clear_requested_cursors();

        history.update(cx, |history, cx| history.refresh_full_history(cx));
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            assert_eq!(history.sessions.len(), 2);
        });
        assert_eq!(
            session_list.requested_cursors(),
            vec![None, Some("page-2".to_string())]
        );
    }

    #[gpui::test]
    async fn test_partial_refresh_batch_drops_non_first_page_sessions(cx: &mut TestAppContext) {
        init_test(cx);

        let second_page_session_id = acp::SessionId::new("session-2");
        let session_list = Rc::new(PaginatedTestSessionList::new(
            vec![test_session("session-1", "First")],
            vec![test_session("session-2", "Second")],
        ));

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();

        history.update(cx, |history, cx| history.refresh_full_history(cx));
        cx.run_until_parked();

        session_list.clear_requested_cursors();

        session_list.send_update(SessionListUpdate::SessionInfo {
            session_id: second_page_session_id.clone(),
            update: acp::SessionInfoUpdate::new().title("Updated Second"),
        });
        session_list.send_update(SessionListUpdate::Refresh);
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            assert_eq!(history.sessions.len(), 1);
            assert_eq!(
                history.sessions[0].session_id,
                acp::SessionId::new("session-1")
            );
            assert!(
                history
                    .sessions
                    .iter()
                    .all(|session| session.session_id != second_page_session_id)
            );
        });
        assert_eq!(session_list.requested_cursors(), vec![None]);
    }

    #[gpui::test]
    async fn test_full_pagination_works_with_async_page_fetches(cx: &mut TestAppContext) {
        init_test(cx);

        let session_list = Rc::new(
            PaginatedTestSessionList::new(
                vec![test_session("session-1", "First")],
                vec![test_session("session-2", "Second")],
            )
            .with_async_responses(),
        );

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();
        session_list.clear_requested_cursors();

        history.update(cx, |history, cx| history.refresh_full_history(cx));
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            assert_eq!(history.sessions.len(), 2);
        });
        assert_eq!(
            session_list.requested_cursors(),
            vec![None, Some("page-2".to_string())]
        );
    }

    #[gpui::test]
    async fn test_apply_info_update_title(cx: &mut TestAppContext) {
        init_test(cx);

        let session_id = acp::SessionId::new("test-session");
        let sessions = vec![AgentSessionInfo {
            session_id: session_id.clone(),
            cwd: None,
            title: Some("Original Title".into()),
            updated_at: None,
            created_at: None,
            meta: None,
        }];
        let session_list = Rc::new(TestSessionList::new(sessions));

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();

        session_list.send_update(SessionListUpdate::SessionInfo {
            session_id: session_id.clone(),
            update: acp::SessionInfoUpdate::new().title("New Title"),
        });
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            let session = history.sessions.iter().find(|s| s.session_id == session_id);
            assert_eq!(
                session.unwrap().title.as_ref().map(|s| s.as_ref()),
                Some("New Title")
            );
        });
    }

    #[gpui::test]
    async fn test_apply_info_update_clears_title_with_null(cx: &mut TestAppContext) {
        init_test(cx);

        let session_id = acp::SessionId::new("test-session");
        let sessions = vec![AgentSessionInfo {
            session_id: session_id.clone(),
            cwd: None,
            title: Some("Original Title".into()),
            updated_at: None,
            created_at: None,
            meta: None,
        }];
        let session_list = Rc::new(TestSessionList::new(sessions));

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();

        session_list.send_update(SessionListUpdate::SessionInfo {
            session_id: session_id.clone(),
            update: acp::SessionInfoUpdate::new().title(None::<String>),
        });
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            let session = history.sessions.iter().find(|s| s.session_id == session_id);
            assert_eq!(session.unwrap().title, None);
        });
    }

    #[gpui::test]
    async fn test_apply_info_update_ignores_undefined_fields(cx: &mut TestAppContext) {
        init_test(cx);

        let session_id = acp::SessionId::new("test-session");
        let sessions = vec![AgentSessionInfo {
            session_id: session_id.clone(),
            cwd: None,
            title: Some("Original Title".into()),
            updated_at: None,
            created_at: None,
            meta: None,
        }];
        let session_list = Rc::new(TestSessionList::new(sessions));

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();

        session_list.send_update(SessionListUpdate::SessionInfo {
            session_id: session_id.clone(),
            update: acp::SessionInfoUpdate::new(),
        });
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            let session = history.sessions.iter().find(|s| s.session_id == session_id);
            assert_eq!(
                session.unwrap().title.as_ref().map(|s| s.as_ref()),
                Some("Original Title")
            );
        });
    }

    #[gpui::test]
    async fn test_multiple_info_updates_applied_in_order(cx: &mut TestAppContext) {
        init_test(cx);

        let session_id = acp::SessionId::new("test-session");
        let sessions = vec![AgentSessionInfo {
            session_id: session_id.clone(),
            cwd: None,
            title: None,
            updated_at: None,
            created_at: None,
            meta: None,
        }];
        let session_list = Rc::new(TestSessionList::new(sessions));

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();

        session_list.send_update(SessionListUpdate::SessionInfo {
            session_id: session_id.clone(),
            update: acp::SessionInfoUpdate::new().title("First Title"),
        });
        session_list.send_update(SessionListUpdate::SessionInfo {
            session_id: session_id.clone(),
            update: acp::SessionInfoUpdate::new().title("Second Title"),
        });
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            let session = history.sessions.iter().find(|s| s.session_id == session_id);
            assert_eq!(
                session.unwrap().title.as_ref().map(|s| s.as_ref()),
                Some("Second Title")
            );
        });
    }

    #[gpui::test]
    async fn test_refresh_supersedes_info_updates(cx: &mut TestAppContext) {
        init_test(cx);

        let session_id = acp::SessionId::new("test-session");
        let sessions = vec![AgentSessionInfo {
            session_id: session_id.clone(),
            cwd: None,
            title: Some("Server Title".into()),
            updated_at: None,
            created_at: None,
            meta: None,
        }];
        let session_list = Rc::new(TestSessionList::new(sessions));

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();

        session_list.send_update(SessionListUpdate::SessionInfo {
            session_id: session_id.clone(),
            update: acp::SessionInfoUpdate::new().title("Local Update"),
        });
        session_list.send_update(SessionListUpdate::Refresh);
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            let session = history.sessions.iter().find(|s| s.session_id == session_id);
            assert_eq!(
                session.unwrap().title.as_ref().map(|s| s.as_ref()),
                Some("Server Title")
            );
        });
    }

    #[gpui::test]
    async fn test_info_update_for_unknown_session_is_ignored(cx: &mut TestAppContext) {
        init_test(cx);

        let session_id = acp::SessionId::new("known-session");
        let sessions = vec![AgentSessionInfo {
            session_id,
            cwd: None,
            title: Some("Original".into()),
            updated_at: None,
            created_at: None,
            meta: None,
        }];
        let session_list = Rc::new(TestSessionList::new(sessions));

        let history = cx.new(|cx| ThreadHistory::new(Some(session_list.clone()), cx));
        cx.run_until_parked();

        session_list.send_update(SessionListUpdate::SessionInfo {
            session_id: acp::SessionId::new("unknown-session"),
            update: acp::SessionInfoUpdate::new().title("Should Be Ignored"),
        });
        cx.run_until_parked();

        history.update(cx, |history, _cx| {
            assert_eq!(history.sessions.len(), 1);
            assert_eq!(
                history.sessions[0].title.as_ref().map(|s| s.as_ref()),
                Some("Original")
            );
        });
    }
}
