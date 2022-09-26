use crate::{http::HttpClient, ZED_SECRET_CLIENT_TOKEN};
use gpui::{
    executor::Background,
    serde_json::{self, value::Map, Value},
    AppContext, Task,
};
use isahc::Request;
use parking_lot::Mutex;
use serde::Serialize;
use std::{
    mem,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use util::{post_inc, ResultExt};

pub struct Telemetry {
    client: Arc<dyn HttpClient>,
    executor: Arc<Background>,
    session_id: u128,
    state: Mutex<TelemetryState>,
}

#[derive(Default)]
struct TelemetryState {
    user_id: Option<Arc<str>>,
    device_id: Option<Arc<str>>,
    app_version: Option<Arc<str>>,
    os_version: Option<Arc<str>>,
    os_name: &'static str,
    queue: Vec<AmplitudeEvent>,
    next_event_id: usize,
    flush_task: Option<Task<()>>,
}

const AMPLITUDE_EVENTS_URL: &'static str = "https//api2.amplitude.com/batch";

#[derive(Serialize)]
struct AmplitudeEventBatch {
    api_key: &'static str,
    events: Vec<AmplitudeEvent>,
}

#[derive(Serialize)]
struct AmplitudeEvent {
    user_id: Option<Arc<str>>,
    device_id: Option<Arc<str>>,
    event_type: String,
    event_properties: Option<Map<String, Value>>,
    user_properties: Option<Map<String, Value>>,
    os_name: &'static str,
    os_version: Option<Arc<str>>,
    app_version: Option<Arc<str>>,
    event_id: usize,
    session_id: u128,
    time: u128,
}

#[cfg(debug_assertions)]
const MAX_QUEUE_LEN: usize = 1;

#[cfg(not(debug_assertions))]
const MAX_QUEUE_LEN: usize = 10;

const DEBOUNCE_INTERVAL: Duration = Duration::from_secs(30);

impl Telemetry {
    pub fn new(client: Arc<dyn HttpClient>, cx: &AppContext) -> Arc<Self> {
        let platform = cx.platform();
        Arc::new(Self {
            client,
            executor: cx.background().clone(),
            session_id: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            state: Mutex::new(TelemetryState {
                os_version: platform
                    .os_version()
                    .log_err()
                    .map(|v| v.to_string().into()),
                os_name: platform.os_name().into(),
                app_version: platform
                    .app_version()
                    .log_err()
                    .map(|v| v.to_string().into()),
                device_id: None,
                queue: Default::default(),
                flush_task: Default::default(),
                next_event_id: 0,
                user_id: None,
            }),
        })
    }

    pub fn log_event(self: &Arc<Self>, kind: &str, properties: Value) {
        let mut state = self.state.lock();
        let event = AmplitudeEvent {
            event_type: kind.to_string(),
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            session_id: self.session_id,
            event_properties: if let Value::Object(properties) = properties {
                Some(properties)
            } else {
                None
            },
            user_properties: None,
            user_id: state.user_id.clone(),
            device_id: state.device_id.clone(),
            os_name: state.os_name,
            os_version: state.os_version.clone(),
            app_version: state.app_version.clone(),
            event_id: post_inc(&mut state.next_event_id),
        };
        state.queue.push(event);
        if state.queue.len() >= MAX_QUEUE_LEN {
            drop(state);
            self.flush();
        } else {
            let this = self.clone();
            let executor = self.executor.clone();
            state.flush_task = Some(self.executor.spawn(async move {
                executor.timer(DEBOUNCE_INTERVAL).await;
                this.flush();
            }));
        }
    }

    fn flush(&self) {
        let mut state = self.state.lock();
        let events = mem::take(&mut state.queue);
        let client = self.client.clone();
        state.flush_task.take();
        self.executor
            .spawn(async move {
                let body = serde_json::to_vec(&AmplitudeEventBatch {
                    api_key: ZED_SECRET_CLIENT_TOKEN,
                    events,
                })
                .log_err()?;
                let request = Request::post(AMPLITUDE_EVENTS_URL)
                    .header("Content-Type", "application/json")
                    .body(body.into())
                    .log_err()?;
                client.send(request).await.log_err();
                Some(())
            })
            .detach();
    }
}
