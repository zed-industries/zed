use std::time::Duration;

use db::kvp::KEY_VALUE_STORE;
use gpui::{AnyWindowHandle, AppContext, Subscription, Task, WindowId};
use util::ResultExt;
use uuid::Uuid;

pub struct Session {
    session_id: String,
    old_session_id: Option<String>,
    old_window_ids: Option<Vec<WindowId>>,
    _serialization_task: Option<Task<()>>,
    _on_app_quit: Option<Subscription>,
}

const SESSION_ID_KEY: &'static str = "session_id";
const SESSION_ORDERED_WINDOWS_KEY: &'static str = "session_ordered_window_ids";

impl Session {
    pub async fn new() -> Self {
        let old_session_id = KEY_VALUE_STORE.read_kvp(&SESSION_ID_KEY).ok().flatten();

        let session_id = Uuid::new_v4().to_string();

        KEY_VALUE_STORE
            .write_kvp(SESSION_ID_KEY.to_string(), session_id.clone())
            .await
            .log_err();

        let old_window_ids = KEY_VALUE_STORE
            .read_kvp(&SESSION_ORDERED_WINDOWS_KEY)
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
            _serialization_task: None,
            _on_app_quit: None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            old_session_id: None,
            old_window_ids: None,
            _serialization_task: Some(Task::ready(())),
            _on_app_quit: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.session_id
    }

    pub fn last_session_id(&self) -> Option<&str> {
        self.old_session_id.as_deref()
    }

    pub fn last_session_windows_order(&self) -> Option<Vec<WindowId>> {
        self.old_window_ids.clone()
    }

    pub fn start_serialization(&mut self, cx: &mut AppContext) {
        self._on_app_quit = Some(cx.on_app_quit(|cx| {
            if let Some(windows) = cx.windows_with_platform_ordering() {
                cx.background_executor().spawn(store_window_order(windows))
            } else {
                Task::ready(())
            }
        }));
        self._serialization_task = Some(cx.spawn(|cx| async move {
            loop {
                if let Some(windows) = cx
                    .update(|cx| cx.windows_with_platform_ordering())
                    .ok()
                    .flatten()
                {
                    store_window_order(windows).await;
                }

                cx.background_executor()
                    .timer(Duration::from_millis(100))
                    .await;
            }
        }))
    }
}

async fn store_window_order(windows: Vec<AnyWindowHandle>) {
    let window_ids = windows
        .into_iter()
        .map(|window| window.window_id().as_u64())
        .collect::<Vec<_>>();

    if let Ok(window_ids_json) = serde_json::to_string(&window_ids) {
        KEY_VALUE_STORE
            .write_kvp(SESSION_ORDERED_WINDOWS_KEY.to_string(), window_ids_json)
            .await
            .log_err();
    }
}
