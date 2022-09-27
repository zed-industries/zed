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
use std::{
    mem,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use util::{post_inc, ResultExt, TryFutureExt};
use uuid::Uuid;

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

const AMPLITUDE_EVENTS_URL: &'static str = "https://api2.amplitude.com/batch";

lazy_static! {
    static ref AMPLITUDE_API_KEY: Option<String> = option_env!("AMPLITUDE_API_KEY")
        .map(|key| key.to_string())
        .or(std::env::var("AMPLITUDE_API_KEY").ok());
}

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

#[cfg(debug_assertions)]
const DEBOUNCE_INTERVAL: Duration = Duration::from_secs(1);

#[cfg(not(debug_assertions))]
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

    pub fn start(self: &Arc<Self>, db: Arc<Db>) {
        let this = self.clone();
        self.executor
            .spawn(
                async move {
                    let device_id = if let Some(device_id) = db
                        .read(["device_id"])?
                        .into_iter()
                        .flatten()
                        .next()
                        .and_then(|bytes| String::from_utf8(bytes).ok())
                    {
                        device_id
                    } else {
                        let device_id = Uuid::new_v4().to_string();
                        db.write([("device_id", device_id.as_bytes())])?;
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

    pub fn set_user_id(&self, user_id: Option<u64>) {
        self.state.lock().user_id = user_id.map(|id| id.to_string().into());
    }

    pub fn report_event(self: &Arc<Self>, kind: &str, properties: Value) {
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
            user_properties: None,
            user_id: state.user_id.clone(),
            device_id: state.device_id.clone(),
            os_name: state.os_name,
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

    fn flush(&self) {
        let mut state = self.state.lock();
        let events = mem::take(&mut state.queue);
        state.flush_task.take();

        if let Some(api_key) = AMPLITUDE_API_KEY.as_ref() {
            let client = self.client.clone();
            self.executor
                .spawn(async move {
                    let batch = AmplitudeEventBatch { api_key, events };
                    let body = serde_json::to_vec(&batch).log_err()?;
                    let request = Request::post(AMPLITUDE_EVENTS_URL)
                        .body(body.into())
                        .log_err()?;
                    client.send(request).await.log_err();
                    Some(())
                })
                .detach();
        }
    }
}
