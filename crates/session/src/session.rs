use std::time::Duration;

use db::kvp::KEY_VALUE_STORE;
use gpui::{AnyWindowHandle, ModelContext, Subscription, Task, WindowId};
use util::ResultExt;
use uuid::Uuid;

pub struct Session {
    session_id: String,
    old_session_id: Option<String>,
    old_window_ids: Option<Vec<WindowId>>,
}

const SESSION_ID_KEY: &'static str = "session_id";
const SESSION_WINDOW_STACK_KEY: &'static str = "session_window_stack";

impl Session {
    pub async fn new() -> Self {
        let old_session_id = KEY_VALUE_STORE.read_kvp(&SESSION_ID_KEY).ok().flatten();

        let session_id = Uuid::new_v4().to_string();

        KEY_VALUE_STORE
            .write_kvp(SESSION_ID_KEY.to_string(), session_id.clone())
            .await
            .log_err();

        let old_window_ids = KEY_VALUE_STORE
            .read_kvp(&SESSION_WINDOW_STACK_KEY)
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str::<Vec<u64>>(&json).ok())
            .map(|vec| {
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
            session_id: Uuid::new_v4().to_string(),
            old_session_id: None,
            old_window_ids: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.session_id
    }
}

pub struct AppSession {
    session: Session,
    _serialization_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl AppSession {
    pub fn new(session: Session, cx: &mut ModelContext<Self>) -> Self {
        let _subscriptions = vec![cx.on_app_quit(Self::app_will_quit)];

        let _serialization_task = Some(cx.spawn(|_, cx| async move {
            loop {
                if let Some(windows) = cx.update(|cx| cx.window_stack()).ok().flatten() {
                    store_window_stack(windows).await;
                }

                cx.background_executor()
                    .timer(Duration::from_millis(100))
                    .await;
            }
        }));

        Self {
            session,
            _subscriptions,
            _serialization_task,
        }
    }

    fn app_will_quit(&mut self, cx: &mut ModelContext<Self>) -> Task<()> {
        if let Some(windows) = cx.window_stack() {
            cx.background_executor().spawn(store_window_stack(windows))
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

    pub fn last_session_window_stack(&self) -> Option<Vec<WindowId>> {
        self.session.old_window_ids.clone()
    }
}

async fn store_window_stack(windows: Vec<AnyWindowHandle>) {
    let window_ids = windows
        .into_iter()
        .map(|window| window.window_id().as_u64())
        .collect::<Vec<_>>();

    if let Ok(window_ids_json) = serde_json::to_string(&window_ids) {
        KEY_VALUE_STORE
            .write_kvp(SESSION_WINDOW_STACK_KEY.to_string(), window_ids_json)
            .await
            .log_err();
    }
}
