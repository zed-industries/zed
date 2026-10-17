use db::{anyhow::Result, kvp::KeyValueStore};
use gpui::{App, AppContext as _, Context, Subscription, Task, WindowId};
use util::ResultExt;

pub struct Session {
    session_id: String,
    old_session_id: Option<String>,
    old_window_ids: Option<Vec<WindowId>>,
}

const SESSION_ID_KEY: &str = "session_id";
const SESSION_WINDOW_STACK_KEY: &str = "session_window_stack";

impl Session {
    pub async fn new(session_id: String, db: KeyValueStore) -> Self {
        let old_session_id = db.read_kvp(SESSION_ID_KEY).ok().flatten();

        let old_window_ids = db
            .read_kvp(SESSION_WINDOW_STACK_KEY)
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str::<Vec<u64>>(&json).ok())
            .map(|vec: Vec<u64>| {
                vec.into_iter()
                    .map(WindowId::from)
                    .collect::<Vec<WindowId>>()
            });

        Self {
            session_id,
            old_session_id,
            old_window_ids,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            old_session_id: None,
            old_window_ids: None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_with_old_session(old_session_id: String) -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            old_session_id: Some(old_session_id),
            old_window_ids: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.session_id
    }
}

pub struct AppSession {
    session: Session,
    committed: bool,
    _serialization_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl AppSession {
    pub fn new(session: Session, cx: &Context<Self>) -> Self {
        let _subscriptions = vec![cx.on_app_quit(Self::app_will_quit)];

        let _serialization_task = if cfg!(not(any(test, feature = "test-support"))) {
            let db = KeyValueStore::global(cx);
            cx.spawn(async move |session, cx| {
                // Disabled in tests: the infinite loop bypasses "parking forbidden" checks,
                // causing tests to hang instead of panicking.
                {
                    let mut current_window_stack = Vec::new();
                    loop {
                        let Ok(windows) = session.read_with(cx, Self::window_stack) else {
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

        Self {
            session,
            committed: false,
            _subscriptions,
            _serialization_task,
        }
    }

    pub fn commit(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        if self.committed {
            return Task::ready(Ok(()));
        }
        let db = KeyValueStore::global(cx);
        let session_id = self.session.session_id.clone();
        cx.spawn(async move |session, cx| {
            db.write_kvp(SESSION_ID_KEY.to_owned(), session_id).await?;
            session.update(cx, |session, _| session.committed = true)
        })
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

    pub fn id(&self) -> &str {
        self.session.id()
    }

    pub fn last_session_id(&self) -> Option<&str> {
        self.session.old_session_id.as_deref()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn replace_session_for_test(&mut self, session: Session) {
        self.session = session;
        self.committed = false;
    }

    pub fn last_session_window_stack(&self) -> Option<Vec<WindowId>> {
        self.session.old_window_ids.clone()
    }

    fn window_stack(&self, cx: &App) -> Option<Vec<u64>> {
        if !self.committed {
            return None;
        }
        Some(
            cx.window_stack()?
                .into_iter()
                .map(|window| window.window_id().as_u64())
                .collect(),
        )
    }
}

async fn store_window_stack(db: KeyValueStore, windows: &[u64]) {
    if let Ok(window_ids_json) = serde_json::to_string(windows) {
        db.write_kvp(SESSION_WINDOW_STACK_KEY.to_string(), window_ids_json)
            .await
            .log_err();
    }
}
