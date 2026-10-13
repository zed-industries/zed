use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Context, Subscription, Task, WindowId};
use util::ResultExt;

pub struct Session {
    session_id: String,
    old_session_id: Option<String>,
    old_window_stack: Option<(String, Vec<WindowId>)>,
    restorable_session_ids: Vec<String>,
}

const SESSION_ID_KEY: &str = "session_id";
const SESSION_WINDOW_STACK_KEY: &str = "session_window_stack";
pub const RESTORABLE_SESSION_IDS_KEY: &str = "restorable_session_ids";

pub fn serialize_restorable_session_ids(session_ids: &[String]) -> String {
    serde_json::to_string(session_ids).unwrap_or_else(|_| "[]".to_owned())
}

impl Session {
    pub fn stored_session_id(db: &KeyValueStore) -> Option<String> {
        db.read_kvp(SESSION_ID_KEY).ok().flatten()
    }

    pub async fn new(session_id: String, db: KeyValueStore) -> Self {
        let old_session_id = Self::stored_session_id(&db);
        let restorable_session_ids = stored_restorable_session_ids(&db, old_session_id.as_deref());
        let next_restorable_session_ids = std::iter::once(session_id.clone())
            .chain(restorable_session_ids.iter().cloned())
            .collect::<Vec<_>>();

        db.write_kvps(vec![
            (SESSION_ID_KEY.to_string(), session_id.clone()),
            (
                RESTORABLE_SESSION_IDS_KEY.to_string(),
                serialize_restorable_session_ids(&next_restorable_session_ids),
            ),
        ])
        .await
        .log_err();

        let old_window_stack =
            stored_window_stack(&db, old_session_id.as_deref()).map(|(session_id, window_ids)| {
                (
                    session_id,
                    window_ids
                        .into_iter()
                        .map(WindowId::from)
                        .collect::<Vec<_>>(),
                )
            });

        Self {
            session_id,
            old_session_id,
            old_window_stack,
            restorable_session_ids,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            old_session_id: None,
            old_window_stack: None,
            restorable_session_ids: Vec::new(),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_with_old_session(old_session_id: String) -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            restorable_session_ids: vec![old_session_id.clone()],
            old_session_id: Some(old_session_id),
            old_window_stack: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.session_id
    }

    pub fn restorable_session_ids(&self) -> &[String] {
        &self.restorable_session_ids
    }
}

pub struct AppSession {
    session: Session,
    _serialization_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl AppSession {
    pub fn new(session: Session, cx: &Context<Self>) -> Self {
        let _subscriptions = vec![cx.on_app_quit(Self::app_will_quit)];

        let _serialization_task = if cfg!(not(any(test, feature = "test-support"))) {
            let db = KeyValueStore::global(cx);
            let session_id = session.session_id.clone();
            cx.spawn(async move |_, cx| {
                // Disabled in tests: the infinite loop bypasses "parking forbidden" checks,
                // causing tests to hang instead of panicking.
                {
                    let mut current_window_stack = Vec::new();
                    loop {
                        if let Some(windows) = cx.update(|cx| window_stack(cx))
                            && !windows.is_empty()
                            && windows != current_window_stack
                        {
                            store_window_stack(db.clone(), &session_id, &windows).await;
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
            _subscriptions,
            _serialization_task,
        }
    }

    fn app_will_quit(&mut self, cx: &mut Context<Self>) -> Task<()> {
        if let Some(window_stack) = window_stack(cx)
            && !window_stack.is_empty()
        {
            let db = KeyValueStore::global(cx);
            let session_id = self.session.session_id.clone();
            cx.background_spawn(
                async move { store_window_stack(db, &session_id, &window_stack).await },
            )
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

    pub fn restorable_session_ids(&self) -> &[String] {
        &self.session.restorable_session_ids
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn replace_session_for_test(&mut self, session: Session) {
        self.session = session;
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn relaunch_for_test(&mut self) {
        let previous_session_id = self.session.session_id.clone();
        self.session = Session {
            session_id: uuid::Uuid::new_v4().to_string(),
            restorable_session_ids: std::iter::once(previous_session_id.clone())
                .chain(self.session.restorable_session_ids.iter().cloned())
                .collect(),
            old_session_id: Some(previous_session_id),
            old_window_stack: None,
        };
    }

    pub fn last_session_window_stack(&self) -> Option<(String, Vec<WindowId>)> {
        self.session.old_window_stack.clone()
    }
}

fn window_stack(cx: &App) -> Option<Vec<u64>> {
    Some(
        cx.window_stack()?
            .into_iter()
            .map(|window| window.window_id().as_u64())
            .collect(),
    )
}

fn stored_restorable_session_ids(db: &KeyValueStore, old_session_id: Option<&str>) -> Vec<String> {
    let Some(old_session_id) = old_session_id else {
        return Vec::new();
    };
    let stored = db
        .read_kvp(RESTORABLE_SESSION_IDS_KEY)
        .ok()
        .flatten()
        .and_then(|json| serde_json::from_str::<Vec<String>>(&json).ok())
        .unwrap_or_default();
    if !stored.iter().any(|session_id| session_id == old_session_id) {
        return vec![old_session_id.to_owned()];
    }
    std::iter::once(old_session_id.to_owned())
        .chain(
            stored
                .into_iter()
                .filter(|session_id| session_id != old_session_id),
        )
        .collect()
}

fn stored_window_stack(
    db: &KeyValueStore,
    legacy_session_id: Option<&str>,
) -> Option<(String, Vec<u64>)> {
    let json = db.read_kvp(SESSION_WINDOW_STACK_KEY).ok().flatten()?;
    match serde_json::from_str::<(String, Vec<u64>)>(&json) {
        Ok(window_stack) => Some(window_stack),
        Err(_) => Some((
            legacy_session_id?.to_owned(),
            serde_json::from_str::<Vec<u64>>(&json).ok()?,
        )),
    }
}

async fn store_window_stack(db: KeyValueStore, session_id: &str, windows: &[u64]) {
    if let Ok(window_ids_json) = serde_json::to_string(&(session_id, windows)) {
        db.write_kvp(SESSION_WINDOW_STACK_KEY.to_string(), window_ids_json)
            .await
            .log_err();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restorable_sessions_survive_interrupted_launches_but_not_downgrades() {
        gpui::block_on(async {
            let db = KeyValueStore::open_test_db("session-recovery").await;
            let mut sessions = Vec::new();
            for session_id in ["first", "second", "third"] {
                let session = Session::new(session_id.to_owned(), db.clone()).await;
                sessions.push(session.restorable_session_ids);
            }
            db.write_kvp(SESSION_ID_KEY.to_string(), "downgraded".to_owned())
                .await
                .unwrap();
            for session_id in ["fourth", "fifth"] {
                let session = Session::new(session_id.to_owned(), db.clone()).await;
                sessions.push(session.restorable_session_ids);
            }

            let mut window_stacks = Vec::new();
            for (stack_session_id, next_session_id) in [("other", "sixth"), ("sixth", "seventh")] {
                store_window_stack(
                    db.clone(),
                    stack_session_id,
                    &[WindowId::from(7u64).as_u64()],
                )
                .await;
                window_stacks.push(
                    Session::new(next_session_id.to_owned(), db.clone())
                        .await
                        .old_window_stack,
                );
            }

            let ids = |ids: &[&str]| ids.iter().map(|id| id.to_string()).collect::<Vec<_>>();
            assert_eq!(
                (sessions, window_stacks),
                (
                    vec![
                        Vec::new(),
                        ids(&["first"]),
                        ids(&["second", "first"]),
                        ids(&["downgraded"]),
                        ids(&["fourth", "downgraded"]),
                    ],
                    vec![
                        Some(("other".to_owned(), vec![WindowId::from(7u64)])),
                        Some(("sixth".to_owned(), vec![WindowId::from(7u64)])),
                    ],
                )
            );
        });
    }
}
