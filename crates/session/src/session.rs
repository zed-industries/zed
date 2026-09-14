use anyhow::Context as _;
use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Context, Subscription, Task, WindowId};
use std::collections::HashMap;
use util::ResultExt;

pub struct Session {
    session_id: String,
    old_session_id: Option<String>,
    old_window_ids: Option<Vec<WindowId>>,
    max_window_id: u64,
}

const SESSION_ID_KEY: &str = "session_id";
const SESSION_WINDOW_STACK_KEY: &str = "session_window_stack";

impl Session {
    pub async fn new(
        launch_id: String,
        db: KeyValueStore,
        preserve_previous: bool,
        max_window_id: Option<u64>,
    ) -> anyhow::Result<Self> {
        let old_session_id = db
            .read_kvp(SESSION_ID_KEY)
            .context("reading recovery session ID")?;
        let old_window_ids = db
            .read_kvp(SESSION_WINDOW_STACK_KEY)
            .context("reading recovery window stack")?
            .and_then(|json| serde_json::from_str::<Vec<u64>>(&json).log_err())
            .map(|ids| ids.into_iter().map(WindowId::from).collect::<Vec<_>>());
        let max_window_id = max_window_id
            .into_iter()
            .chain(old_window_ids.iter().flatten().map(WindowId::as_u64))
            .max()
            .unwrap_or(0);
        anyhow::ensure!(max_window_id < u64::MAX, "serialized window IDs exhausted");
        let session_id = if preserve_previous {
            old_session_id.clone().unwrap_or(launch_id)
        } else {
            launch_id
        };

        if old_session_id.as_deref() != Some(session_id.as_str()) {
            db.write_kvp(SESSION_ID_KEY.to_string(), session_id.clone())
                .await
                .context("writing recovery session ID")?;
        }

        Ok(Self {
            session_id,
            old_session_id,
            old_window_ids,
            max_window_id,
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            old_session_id: None,
            old_window_ids: None,
            max_window_id: 0,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_with_old_session(old_session_id: String) -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            old_session_id: Some(old_session_id),
            old_window_ids: None,
            max_window_id: 0,
        }
    }

    pub fn id(&self) -> &str {
        &self.session_id
    }
}

pub struct AppSession {
    session: Session,
    window_ids: HashMap<WindowId, WindowId>,
    pending_window_ids: Vec<WindowId>,
    max_window_id: u64,
    _serialization_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl AppSession {
    pub fn new(session: Session, cx: &Context<Self>) -> Self {
        let session_handle = cx.weak_entity();
        let _subscriptions = vec![
            cx.on_app_quit(Self::app_will_quit),
            cx.on_window_closed(move |cx, window_id| {
                session_handle
                    .update(cx, |session, _| {
                        session.window_ids.remove(&window_id);
                    })
                    .log_err();
            }),
        ];

        let _serialization_task = if cfg!(not(any(test, feature = "test-support"))) {
            let db = KeyValueStore::global(cx);
            cx.spawn(async move |this, cx| {
                // Disabled in tests: the infinite loop bypasses "parking forbidden" checks,
                // causing tests to hang instead of panicking.
                {
                    let mut current_window_stack = Vec::new();
                    loop {
                        let Ok(windows) = this.read_with(cx, |this, cx| this.window_stack(cx))
                        else {
                            return;
                        };
                        if let Some(windows) = windows
                            && !windows.is_empty()
                            && windows != current_window_stack
                        {
                            store_window_stack(db.clone(), &windows).await;
                            current_window_stack = windows;
                        }

                        cx.background_executor()
                            .timer(std::time::Duration::from_millis(500))
                            .await;
                    }
                }
            })
        } else {
            Task::ready(())
        };

        let pending_window_ids = if session.old_session_id.as_deref() == Some(session.id()) {
            session.old_window_ids.clone().unwrap_or_default()
        } else {
            Vec::new()
        };
        Self {
            max_window_id: session.max_window_id,
            session,
            window_ids: HashMap::new(),
            pending_window_ids,
            _subscriptions,
            _serialization_task,
        }
    }

    pub fn id(&self) -> &str {
        self.session.id()
    }

    pub fn last_session_id(&self) -> Option<&str> {
        self.session.old_session_id.as_deref()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn replace_session_for_test(&mut self, session: Session) {
        self.max_window_id = self.max_window_id.max(session.max_window_id);
        self.pending_window_ids = if session.old_session_id.as_deref() == Some(session.id()) {
            session
                .old_window_ids
                .iter()
                .flatten()
                .filter(|pending| {
                    !self
                        .window_ids
                        .values()
                        .any(|window_id| window_id == *pending)
                })
                .copied()
                .collect()
        } else {
            Vec::new()
        };
        self.session = session;
    }

    pub fn last_session_window_stack(&self) -> Option<Vec<WindowId>> {
        self.session.old_window_ids.clone()
    }

    pub fn register_window(
        &mut self,
        runtime_id: WindowId,
        preferred_id: Option<WindowId>,
    ) -> WindowId {
        if let Some(window_id) = self.window_ids.get(&runtime_id) {
            return *window_id;
        }
        let window_id = preferred_id
            .filter(|preferred| {
                !self
                    .window_ids
                    .values()
                    .any(|window_id| window_id == preferred)
            })
            .unwrap_or_else(|| {
                WindowId::from(
                    runtime_id.as_u64().max(
                        self.max_window_id
                            .checked_add(1)
                            .expect("serialized window IDs exhausted"),
                    ),
                )
            });
        self.max_window_id = self.max_window_id.max(window_id.as_u64());
        self.pending_window_ids
            .retain(|pending| *pending != window_id);
        self.window_ids.insert(runtime_id, window_id);
        window_id
    }

    fn app_will_quit(&mut self, cx: &mut Context<Self>) -> Task<()> {
        if let Some(window_stack) = self.window_stack(cx)
            && !window_stack.is_empty()
        {
            let db = KeyValueStore::global(cx);
            cx.background_spawn(async move { store_window_stack(db, &window_stack).await })
        } else {
            Task::ready(())
        }
    }

    fn window_stack(&self, cx: &App) -> Option<Vec<u64>> {
        Some(
            self.serialized_window_stack(
                cx.window_stack()?
                    .into_iter()
                    .map(|window| window.window_id()),
            ),
        )
    }

    fn serialized_window_stack(&self, windows: impl IntoIterator<Item = WindowId>) -> Vec<u64> {
        windows
            .into_iter()
            .filter_map(|window| self.window_ids.get(&window))
            .chain(&self.pending_window_ids)
            .map(WindowId::as_u64)
            .collect()
    }
}

async fn store_window_stack(db: KeyValueStore, windows: &[u64]) {
    if let Ok(window_ids_json) = serde_json::to_string(windows) {
        db.write_kvp(SESSION_WINDOW_STACK_KEY.to_string(), window_ids_json)
            .await
            .log_err();
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AppSession, SESSION_ID_KEY, SESSION_WINDOW_STACK_KEY, Session, store_window_stack,
    };
    use db::kvp::KeyValueStore;
    use gpui::{AppContext as _, Empty, TestAppContext, WindowId};

    #[gpui::test]
    async fn test_recovery_session_survives_interrupted_startup(cx: &mut TestAppContext) {
        for (restored_count, database_name) in
            [(0, "interrupted-startup-0"), (1, "interrupted-startup-1")]
        {
            let saved_windows = [4_294_967_297, 4_294_967_298];
            let db = session_database(database_name, &saved_windows).await;

            let interrupted = Session::new(
                String::from("interrupted"),
                db.clone(),
                true,
                Some(saved_windows[1]),
            )
            .await
            .expect("failed to resume session");
            let session = cx.new(|cx| AppSession::new(interrupted, cx));
            let stack = session.update(cx, |session, _| {
                for index in 0..restored_count {
                    assert_eq!(
                        session.register_window(
                            WindowId::from(saved_windows[index]),
                            Some(WindowId::from(saved_windows[1 - index])),
                        ),
                        WindowId::from(saved_windows[1 - index]),
                    );
                }
                session.serialized_window_stack(
                    saved_windows
                        .iter()
                        .take(restored_count)
                        .copied()
                        .map(WindowId::from),
                )
            });
            assert_eq!(
                stack,
                if restored_count == 0 {
                    vec![4_294_967_297, 4_294_967_298]
                } else {
                    vec![4_294_967_298, 4_294_967_297]
                },
            );
            store_window_stack(db.clone(), &stack).await;
            drop(session);

            let resumed = Session::new(
                String::from("resumed"),
                db.clone(),
                true,
                Some(saved_windows[1]),
            )
            .await
            .expect("failed to resume interrupted session");
            assert_eq!(resumed.id(), "original");
            assert_eq!(resumed.old_session_id.as_deref(), Some("original"));
            assert_eq!(
                db.read_kvp(SESSION_ID_KEY)
                    .expect("failed to read session ID"),
                Some(String::from("original")),
            );
        }
    }

    #[gpui::test]
    async fn test_explicit_new_session_preserves_window_stack(cx: &mut TestAppContext) {
        let db = session_database("explicit-new-session", &[4_294_967_300]).await;
        let session = Session::new(String::from("new"), db.clone(), false, None)
            .await
            .expect("failed to start new session");
        assert_eq!(session.id(), "new");
        assert_eq!(session.old_session_id.as_deref(), Some("original"));
        assert_eq!(session.max_window_id, 4_294_967_300);
        assert_eq!(
            db.read_kvp(SESSION_ID_KEY)
                .expect("failed to read session ID"),
            Some(String::from("new")),
        );
        assert_eq!(
            db.read_kvp(SESSION_WINDOW_STACK_KEY)
                .expect("failed to read window stack"),
            Some(String::from("[4294967300]")),
        );
        let session = cx.new(|cx| AppSession::new(session, cx));
        session.update(cx, |session, _| {
            assert_eq!(session.serialized_window_stack([]), Vec::<u64>::new());
            assert_eq!(
                session
                    .register_window(WindowId::from(4_294_967_297), None)
                    .as_u64(),
                4_294_967_301,
            );
        });
    }

    #[gpui::test]
    fn test_restored_window_mapping_and_fresh_ids(cx: &mut TestAppContext) {
        let session = cx.new(|cx| AppSession::new(Session::test(), cx));
        session.update(cx, |session, _| {
            let fresh = WindowId::from(4_294_967_300);
            assert_eq!(session.register_window(fresh, None), fresh);
            let first = WindowId::from(4_294_967_297);
            let second = WindowId::from(4_294_967_298);
            let third = WindowId::from(4_294_967_299);
            assert_eq!(session.register_window(first, Some(second)), second);
            assert_eq!(session.register_window(second, Some(first)), first);
            assert_eq!(session.register_window(first, None), second);
            assert_eq!(
                session.serialized_window_stack([first, second, third]),
                vec![4_294_967_298, 4_294_967_297],
            );
            assert_eq!(session.register_window(third, None).as_u64(), 4_294_967_301);
            assert_eq!(
                session
                    .register_window(WindowId::from(4_294_967_301), Some(first))
                    .as_u64(),
                4_294_967_302,
            );
            assert_eq!(
                session
                    .register_window(WindowId::from(4_294_967_400), None)
                    .as_u64(),
                4_294_967_400,
            );
            assert_eq!(session.serialized_window_stack([]), Vec::<u64>::new());
        });
    }

    #[gpui::test]
    fn test_window_close_releases_ids_and_keeps_pending_windows(cx: &mut TestAppContext) {
        let saved = [
            WindowId::from(4_294_967_297),
            WindowId::from(4_294_967_298),
            WindowId::from(4_294_967_299),
        ];
        let session = cx.new(|cx| {
            let mut session = Session::test();
            session.old_session_id = Some(session.session_id.clone());
            session.old_window_ids = Some(saved.to_vec());
            session.max_window_id = saved[2].as_u64();
            AppSession::new(session, cx)
        });
        let first = cx.add_window(|_, _| Empty);
        let second = cx.add_window(|_, _| Empty);
        session.update(cx, |session, _| {
            session.register_window(first.window_id(), Some(saved[2]));
            session.register_window(second.window_id(), Some(saved[0]));
            assert_eq!(
                session.serialized_window_stack([first.window_id(), second.window_id()]),
                vec![4_294_967_299, 4_294_967_297, 4_294_967_298],
            );
        });
        first
            .update(cx, |_, window, _| window.remove_window())
            .expect("failed to close restored window");
        cx.run_until_parked();
        assert_eq!(
            session.read_with(cx, |session, _| session.window_ids.len()),
            1
        );
        assert_eq!(
            session.read_with(cx, |session, _| {
                session.serialized_window_stack([second.window_id()])
            }),
            vec![4_294_967_297, 4_294_967_298],
        );
        let third = cx.add_window(|_, _| Empty);
        session.update(cx, |session, _| {
            session.register_window(third.window_id(), Some(saved[1]));
            assert_eq!(session.pending_window_ids, Vec::<WindowId>::new());
            assert_eq!(
                session.serialized_window_stack([third.window_id(), second.window_id()]),
                vec![4_294_967_298, 4_294_967_297],
            );
        });
        let fourth = cx.add_window(|_, _| Empty);
        assert_eq!(
            session.update(cx, |session, _| {
                session.register_window(fourth.window_id(), Some(saved[2]))
            }),
            saved[2],
        );
    }

    #[gpui::test]
    async fn test_exhausted_window_ids_do_not_publish_session() {
        for (from_stack, database_name) in [
            (false, "exhausted-window-ids-false"),
            (true, "exhausted-window-ids-true"),
        ] {
            let db =
                session_database(database_name, if from_stack { &[u64::MAX] } else { &[] }).await;
            let error = Session::new(
                String::from("new"),
                db.clone(),
                false,
                (!from_stack).then_some(u64::MAX),
            )
            .await
            .err()
            .expect("exhausted window IDs should fail startup");
            assert_eq!(error.to_string(), "serialized window IDs exhausted");
            assert_eq!(
                db.read_kvp(SESSION_ID_KEY)
                    .expect("failed to read session ID"),
                Some(String::from("original")),
            );
        }
    }

    #[gpui::test]
    async fn test_session_pointer_errors_propagate() {
        let db = KeyValueStore::open_test_db("session-pointer-errors").await;
        db.write(|connection| {
            connection.exec(
                "CREATE TRIGGER reject_session BEFORE INSERT ON kv_store
                 WHEN NEW.key = 'session_id'
                 BEGIN SELECT RAISE(FAIL, 'session write rejected'); END",
            )?()
        })
        .await
        .expect("failed to install write failure");
        let error = Session::new(String::from("new"), db.clone(), true, None)
            .await
            .err()
            .expect("session pointer write should fail");
        assert_eq!(error.to_string(), "writing recovery session ID");
        assert_eq!(
            db.read_kvp(SESSION_ID_KEY)
                .expect("failed to read session ID"),
            None
        );

        db.write(|connection| connection.exec("DROP TABLE kv_store")?())
            .await
            .expect("failed to install read failure");
        let error = Session::new(String::from("new"), db, true, None)
            .await
            .err()
            .expect("session pointer read should fail");
        assert_eq!(error.to_string(), "reading recovery session ID");
    }

    async fn session_database(name: &'static str, windows: &[u64]) -> KeyValueStore {
        let db = KeyValueStore::open_test_db(name).await;
        let session = Session::new(String::from("original"), db.clone(), true, None)
            .await
            .expect("failed to create original session");
        assert_eq!(session.id(), "original");
        assert_eq!(session.old_session_id, None);
        store_window_stack(db.clone(), windows).await;
        db
    }
}
