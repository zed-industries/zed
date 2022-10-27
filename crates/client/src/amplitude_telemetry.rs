use crate::http::HttpClient;
use db::Db;
use gpui::{
    executor::Background,
    serde_json::{self, value::Map, Value},
    AppContext, Task,
};
use isahc::Request;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::Serialize;
use serde_json::json;
use std::{
    io::Write,
    mem,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tempfile::NamedTempFile;
use util::{post_inc, ResultExt, TryFutureExt};
use uuid::Uuid;

pub struct AmplitudeTelemetry {
    http_client: Arc<dyn HttpClient>,
    executor: Arc<Background>,
    session_id: u128,
    state: Mutex<AmplitudeTelemetryState>,
}

#[derive(Default)]
struct AmplitudeTelemetryState {
    metrics_id: Option<Arc<str>>,
    device_id: Option<Arc<str>>,
    app_version: Option<Arc<str>>,
    os_version: Option<Arc<str>>,
    os_name: &'static str,
    queue: Vec<AmplitudeEvent>,
    next_event_id: usize,
    flush_task: Option<Task<()>>,
    log_file: Option<NamedTempFile>,
}

const AMPLITUDE_EVENTS_URL: &'static str = "https://api2.amplitude.com/batch";

lazy_static! {
    static ref AMPLITUDE_API_KEY: Option<String> = std::env::var("ZED_AMPLITUDE_API_KEY")
        .ok()
        .or_else(|| option_env!("ZED_AMPLITUDE_API_KEY").map(|key| key.to_string()));
}

#[derive(Serialize)]
struct AmplitudeEventBatch {
    api_key: &'static str,
    events: Vec<AmplitudeEvent>,
}

#[derive(Serialize)]
struct AmplitudeEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<Arc<str>>,
    device_id: Option<Arc<str>>,
    event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_properties: Option<Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_properties: Option<Map<String, Value>>,
    os_name: &'static str,
    os_version: Option<Arc<str>>,
    app_version: Option<Arc<str>>,
    platform: &'static str,
    event_id: usize,
    session_id: u128,
    time: u128,
}

#[cfg(debug_assertions)]
const MAX_QUEUE_LEN: usize = 1;

#[cfg(not(debug_assertions))]
const MAX_QUEUE_LEN: usize = 10;

#[cfg(debug_assertions)]
const DEBOUNCE_INTERVAL: Duration = Duration::from_secs(1);

#[cfg(not(debug_assertions))]
const DEBOUNCE_INTERVAL: Duration = Duration::from_secs(30);

impl AmplitudeTelemetry {
    pub fn new(client: Arc<dyn HttpClient>, cx: &AppContext) -> Arc<Self> {
        let platform = cx.platform();
        let this = Arc::new(Self {
            http_client: client,
            executor: cx.background().clone(),
            session_id: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            state: Mutex::new(AmplitudeTelemetryState {
                os_version: platform.os_version().ok().map(|v| v.to_string().into()),
                os_name: platform.os_name().into(),
                app_version: platform.app_version().ok().map(|v| v.to_string().into()),
                device_id: None,
                queue: Default::default(),
                flush_task: Default::default(),
                next_event_id: 0,
                log_file: None,
                metrics_id: None,
            }),
        });

        if AMPLITUDE_API_KEY.is_some() {
            this.executor
                .spawn({
                    let this = this.clone();
                    async move {
                        if let Some(tempfile) = NamedTempFile::new().log_err() {
                            this.state.lock().log_file = Some(tempfile);
                        }
                    }
                })
                .detach();
        }

        this
    }

    pub fn log_file_path(&self) -> Option<PathBuf> {
        Some(self.state.lock().log_file.as_ref()?.path().to_path_buf())
    }

    pub fn start(self: &Arc<Self>, db: Db) {
        let this = self.clone();
        self.executor
            .spawn(
                async move {
                    let device_id = if let Ok(Some(device_id)) = db.read_kvp("device_id") {
                        device_id
                    } else {
                        let device_id = Uuid::new_v4().to_string();
                        db.write_kvp("device_id", &device_id)?;
                        device_id
                    };

                    let device_id = Some(Arc::from(device_id));
                    let mut state = this.state.lock();
                    state.device_id = device_id.clone();
                    for event in &mut state.queue {
                        event.device_id = device_id.clone();
                    }
                    if !state.queue.is_empty() {
                        drop(state);
                        this.flush();
                    }

                    anyhow::Ok(())
                }
                .log_err(),
            )
            .detach();
    }

    pub fn set_authenticated_user_info(
        self: &Arc<Self>,
        metrics_id: Option<String>,
        is_staff: bool,
    ) {
        let is_signed_in = metrics_id.is_some();
        self.state.lock().metrics_id = metrics_id.map(|s| s.into());
        if is_signed_in {
            self.report_event_with_user_properties(
                "$identify",
                Default::default(),
                json!({ "$set": { "staff": is_staff } }),
            )
        }
    }

    pub fn report_event(self: &Arc<Self>, kind: &str, properties: Value) {
        self.report_event_with_user_properties(kind, properties, Default::default());
    }

    fn report_event_with_user_properties(
        self: &Arc<Self>,
        kind: &str,
        properties: Value,
        user_properties: Value,
    ) {
        if AMPLITUDE_API_KEY.is_none() {
            return;
        }

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
            user_properties: if let Value::Object(user_properties) = user_properties {
                Some(user_properties)
            } else {
                None
            },
            user_id: state.metrics_id.clone(),
            device_id: state.device_id.clone(),
            os_name: state.os_name,
            platform: "Zed",
            os_version: state.os_version.clone(),
            app_version: state.app_version.clone(),
            event_id: post_inc(&mut state.next_event_id),
        };
        state.queue.push(event);
        if state.device_id.is_some() {
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
    }

    fn flush(self: &Arc<Self>) {
        let mut state = self.state.lock();
        let events = mem::take(&mut state.queue);
        state.flush_task.take();
        drop(state);

        if let Some(api_key) = AMPLITUDE_API_KEY.as_ref() {
            let this = self.clone();
            self.executor
                .spawn(
                    async move {
                        let mut json_bytes = Vec::new();

                        if let Some(file) = &mut this.state.lock().log_file {
                            let file = file.as_file_mut();
                            for event in &events {
                                json_bytes.clear();
                                serde_json::to_writer(&mut json_bytes, event)?;
                                file.write_all(&json_bytes)?;
                                file.write(b"\n")?;
                            }
                        }

                        let batch = AmplitudeEventBatch { api_key, events };
                        json_bytes.clear();
                        serde_json::to_writer(&mut json_bytes, &batch)?;
                        let request =
                            Request::post(AMPLITUDE_EVENTS_URL).body(json_bytes.into())?;
                        this.http_client.send(request).await?;
                        Ok(())
                    }
                    .log_err(),
                )
                .detach();
        }
    }
}
