use crate::{http::HttpClient, ZED_SECRET_CLIENT_TOKEN};
use gpui::{
    executor::Background,
    serde_json::{self, value::Map, Value},
    AppContext, AppVersion, Task,
};
use isahc::Request;
use parking_lot::Mutex;
use serde::Serialize;
use std::{
    mem,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use util::ResultExt;

pub struct Telemetry {
    client: Arc<dyn HttpClient>,
    executor: Arc<Background>,
    state: Mutex<TelemetryState>,
}

#[derive(Default)]
struct TelemetryState {
    metrics_id: Option<i32>,
    device_id: Option<String>,
    app_version: Option<AppVersion>,
    os_version: Option<AppVersion>,
    queue: Vec<Event>,
    flush_task: Option<Task<()>>,
}

#[derive(Serialize)]
struct RecordEventParams {
    token: &'static str,
    metrics_id: Option<i32>,
    device_id: Option<String>,
    app_version: Option<String>,
    os_version: Option<String>,
    events: Vec<Event>,
}

#[derive(Serialize)]
struct Event {
    #[serde(rename = "type")]
    kind: String,
    time: u128,
    properties: Option<Map<String, Value>>,
}

const MAX_QUEUE_LEN: usize = 30;
const EVENTS_URI: &'static str = "https://zed.dev/api/telemetry";
const DEBOUNCE_INTERVAL: Duration = Duration::from_secs(30);

impl Telemetry {
    pub fn new(client: Arc<dyn HttpClient>, cx: &AppContext) -> Arc<Self> {
        let platform = cx.platform();
        Arc::new(Self {
            client,
            executor: cx.background().clone(),
            state: Mutex::new(TelemetryState {
                os_version: platform.os_version().log_err(),
                app_version: platform.app_version().log_err(),
                metrics_id: None,
                device_id: None,
                queue: Default::default(),
                flush_task: Default::default(),
            }),
        })
    }

    pub fn set_metrics_id(&self, metrics_id: Option<i32>) {
        self.state.lock().metrics_id = metrics_id;
    }

    pub fn log_event(self: &Arc<Self>, kind: &str, properties: Value) {
        let mut state = self.state.lock();
        state.queue.push(Event {
            kind: kind.to_string(),
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            properties: if let Value::Object(properties) = properties {
                Some(properties)
            } else {
                None
            },
        });
        if state.queue.len() >= MAX_QUEUE_LEN {
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
        let app_version = state.app_version;
        let os_version = state.os_version;
        let metrics_id = state.metrics_id;
        let device_id = state.device_id.clone();
        state.flush_task.take();
        self.executor
            .spawn(async move {
                let body = serde_json::to_vec(&RecordEventParams {
                    token: ZED_SECRET_CLIENT_TOKEN,
                    events,
                    app_version: app_version.map(|v| v.to_string()),
                    os_version: os_version.map(|v| v.to_string()),
                    metrics_id,
                    device_id,
                })
                .log_err()?;
                let request = Request::post(EVENTS_URI).body(body.into()).log_err()?;
                client.send(request).await.log_err();
                Some(())
            })
            .detach();
    }
}
