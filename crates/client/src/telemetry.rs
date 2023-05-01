use crate::{ZED_SECRET_CLIENT_TOKEN, ZED_SERVER_URL};
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    executor::Background,
    serde_json::{self, value::Map, Value},
    AppContext, Task,
};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::Serialize;
use serde_json::json;
use settings::TelemetrySettings;
use std::{
    io::Write,
    mem,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tempfile::NamedTempFile;
use util::http::HttpClient;
use util::{channel::ReleaseChannel, post_inc, ResultExt, TryFutureExt};
use uuid::Uuid;

pub struct Telemetry {
    http_client: Arc<dyn HttpClient>,
    executor: Arc<Background>,
    state: Mutex<TelemetryState>,
}

#[derive(Default)]
struct TelemetryState {
    metrics_id: Option<Arc<str>>,      // Per logged-in user
    installation_id: Option<Arc<str>>, // Per app installation
    app_version: Option<Arc<str>>,
    release_channel: Option<&'static str>,
    os_version: Option<Arc<str>>,
    os_name: &'static str,
    mixpanel_events_queue: Vec<MixpanelEvent>,
    clickhouse_events_queue: Vec<ClickhouseEventWrapper>,
    next_mixpanel_event_id: usize,
    flush_mixpanel_events_task: Option<Task<()>>,
    flush_clickhouse_events_task: Option<Task<()>>,
    log_file: Option<NamedTempFile>,
    is_staff: Option<bool>,
}

const MIXPANEL_EVENTS_URL: &'static str = "https://api.mixpanel.com/track";
const MIXPANEL_ENGAGE_URL: &'static str = "https://api.mixpanel.com/engage#profile-set";
const CLICKHOUSE_EVENTS_URL_PATH: &'static str = "/api/events";

lazy_static! {
    static ref MIXPANEL_TOKEN: Option<String> = std::env::var("ZED_MIXPANEL_TOKEN")
        .ok()
        .or_else(|| option_env!("ZED_MIXPANEL_TOKEN").map(|key| key.to_string()));
    static ref CLICKHOUSE_EVENTS_URL: String =
        format!("{}{}", *ZED_SERVER_URL, CLICKHOUSE_EVENTS_URL_PATH);
}

#[derive(Serialize, Debug)]
struct ClickhouseEventRequestBody {
    token: &'static str,
    installation_id: Option<Arc<str>>,
    app_version: Option<Arc<str>>,
    os_name: &'static str,
    os_version: Option<Arc<str>>,
    release_channel: Option<&'static str>,
    events: Vec<ClickhouseEventWrapper>,
}

#[derive(Serialize, Debug)]
struct ClickhouseEventWrapper {
    time: u128,
    signed_in: bool,
    #[serde(flatten)]
    event: ClickhouseEvent,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum ClickhouseEvent {
    Editor {
        operation: &'static str,
        file_extension: Option<String>,
        vim_mode: bool,
        copilot_enabled: bool,
        copilot_enabled_for_language: bool,
    },
}

#[derive(Serialize, Debug)]
struct MixpanelEvent {
    event: String,
    properties: MixpanelEventProperties,
}

#[derive(Serialize, Debug)]
struct MixpanelEventProperties {
    // Mixpanel required fields
    #[serde(skip_serializing_if = "str::is_empty")]
    token: &'static str,
    time: u128,
    #[serde(rename = "distinct_id")]
    installation_id: Option<Arc<str>>,
    #[serde(rename = "$insert_id")]
    insert_id: usize,
    // Custom fields
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    event_properties: Option<Map<String, Value>>,
    #[serde(rename = "OS Name")]
    os_name: &'static str,
    #[serde(rename = "OS Version")]
    os_version: Option<Arc<str>>,
    #[serde(rename = "Release Channel")]
    release_channel: Option<&'static str>,
    #[serde(rename = "App Version")]
    app_version: Option<Arc<str>>,
    #[serde(rename = "Signed In")]
    signed_in: bool,
}

#[derive(Serialize)]
struct MixpanelEngageRequest {
    #[serde(rename = "$token")]
    token: &'static str,
    #[serde(rename = "$distinct_id")]
    installation_id: Arc<str>,
    #[serde(rename = "$set")]
    set: Value,
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
        let release_channel = if cx.has_global::<ReleaseChannel>() {
            Some(cx.global::<ReleaseChannel>().display_name())
        } else {
            None
        };
        let this = Arc::new(Self {
            http_client: client,
            executor: cx.background().clone(),
            state: Mutex::new(TelemetryState {
                os_version: platform.os_version().ok().map(|v| v.to_string().into()),
                os_name: platform.os_name().into(),
                app_version: platform.app_version().ok().map(|v| v.to_string().into()),
                release_channel,
                installation_id: None,
                metrics_id: None,
                mixpanel_events_queue: Default::default(),
                clickhouse_events_queue: Default::default(),
                flush_mixpanel_events_task: Default::default(),
                flush_clickhouse_events_task: Default::default(),
                next_mixpanel_event_id: 0,
                log_file: None,
                is_staff: None,
            }),
        });

        if MIXPANEL_TOKEN.is_some() {
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

    pub fn start(self: &Arc<Self>) {
        let this = self.clone();
        self.executor
            .spawn(
                async move {
                    let installation_id =
                        if let Ok(Some(installation_id)) = KEY_VALUE_STORE.read_kvp("device_id") {
                            installation_id
                        } else {
                            let installation_id = Uuid::new_v4().to_string();
                            KEY_VALUE_STORE
                                .write_kvp("device_id".to_string(), installation_id.clone())
                                .await?;
                            installation_id
                        };

                    let installation_id: Arc<str> = installation_id.into();
                    let mut state = this.state.lock();
                    state.installation_id = Some(installation_id.clone());

                    for event in &mut state.mixpanel_events_queue {
                        event
                            .properties
                            .installation_id
                            .get_or_insert_with(|| installation_id.clone());
                    }

                    let has_mixpanel_events = !state.mixpanel_events_queue.is_empty();
                    let has_clickhouse_events = !state.clickhouse_events_queue.is_empty();
                    drop(state);

                    if has_mixpanel_events {
                        this.flush_mixpanel_events();
                    }

                    if has_clickhouse_events {
                        this.flush_clickhouse_events();
                    }

                    anyhow::Ok(())
                }
                .log_err(),
            )
            .detach();
    }

    /// This method takes the entire TelemetrySettings struct in order to force client code
    /// to pull the struct out of the settings global. Do not remove!
    pub fn set_authenticated_user_info(
        self: &Arc<Self>,
        metrics_id: Option<String>,
        is_staff: bool,
        telemetry_settings: TelemetrySettings,
    ) {
        if !telemetry_settings.metrics() {
            return;
        }

        let this = self.clone();
        let mut state = self.state.lock();
        let installation_id = state.installation_id.clone();
        let metrics_id: Option<Arc<str>> = metrics_id.map(|id| id.into());
        state.metrics_id = metrics_id.clone();
        state.is_staff = Some(is_staff);
        drop(state);

        if let Some((token, installation_id)) = MIXPANEL_TOKEN.as_ref().zip(installation_id) {
            self.executor
                .spawn(
                    async move {
                        let json_bytes = serde_json::to_vec(&[MixpanelEngageRequest {
                            token,
                            installation_id,
                            set: json!({
                                "Staff": is_staff,
                                "ID": metrics_id,
                                "App": true
                            }),
                        }])?;

                        this.http_client
                            .post_json(MIXPANEL_ENGAGE_URL, json_bytes.into())
                            .await?;
                        anyhow::Ok(())
                    }
                    .log_err(),
                )
                .detach();
        }
    }

    pub fn report_clickhouse_event(
        self: &Arc<Self>,
        event: ClickhouseEvent,
        telemetry_settings: TelemetrySettings,
    ) {
        if !telemetry_settings.metrics() {
            return;
        }

        let mut state = self.state.lock();
        let signed_in = state.metrics_id.is_some();
        state.clickhouse_events_queue.push(ClickhouseEventWrapper {
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            signed_in,
            event,
        });

        if state.installation_id.is_some() {
            if state.mixpanel_events_queue.len() >= MAX_QUEUE_LEN {
                drop(state);
                self.flush_clickhouse_events();
            } else {
                let this = self.clone();
                let executor = self.executor.clone();
                state.flush_clickhouse_events_task = Some(self.executor.spawn(async move {
                    executor.timer(DEBOUNCE_INTERVAL).await;
                    this.flush_clickhouse_events();
                }));
            }
        }
    }

    pub fn report_mixpanel_event(
        self: &Arc<Self>,
        kind: &str,
        properties: Value,
        telemetry_settings: TelemetrySettings,
    ) {
        if !telemetry_settings.metrics() {
            return;
        }

        let mut state = self.state.lock();
        let event = MixpanelEvent {
            event: kind.into(),
            properties: MixpanelEventProperties {
                token: "",
                time: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                installation_id: state.installation_id.clone(),
                insert_id: post_inc(&mut state.next_mixpanel_event_id),
                event_properties: if let Value::Object(properties) = properties {
                    Some(properties)
                } else {
                    None
                },
                os_name: state.os_name,
                os_version: state.os_version.clone(),
                release_channel: state.release_channel,
                app_version: state.app_version.clone(),
                signed_in: state.metrics_id.is_some(),
            },
        };
        state.mixpanel_events_queue.push(event);
        if state.installation_id.is_some() {
            if state.mixpanel_events_queue.len() >= MAX_QUEUE_LEN {
                drop(state);
                self.flush_mixpanel_events();
            } else {
                let this = self.clone();
                let executor = self.executor.clone();
                state.flush_mixpanel_events_task = Some(self.executor.spawn(async move {
                    executor.timer(DEBOUNCE_INTERVAL).await;
                    this.flush_mixpanel_events();
                }));
            }
        }
    }

    pub fn metrics_id(self: &Arc<Self>) -> Option<Arc<str>> {
        self.state.lock().metrics_id.clone()
    }

    pub fn installation_id(self: &Arc<Self>) -> Option<Arc<str>> {
        self.state.lock().installation_id.clone()
    }

    pub fn is_staff(self: &Arc<Self>) -> Option<bool> {
        self.state.lock().is_staff
    }

    fn flush_mixpanel_events(self: &Arc<Self>) {
        let mut state = self.state.lock();
        let mut events = mem::take(&mut state.mixpanel_events_queue);
        state.flush_mixpanel_events_task.take();
        drop(state);

        if let Some(token) = MIXPANEL_TOKEN.as_ref() {
            let this = self.clone();
            self.executor
                .spawn(
                    async move {
                        let mut json_bytes = Vec::new();

                        if let Some(file) = &mut this.state.lock().log_file {
                            let file = file.as_file_mut();
                            for event in &mut events {
                                json_bytes.clear();
                                serde_json::to_writer(&mut json_bytes, event)?;
                                file.write_all(&json_bytes)?;
                                file.write(b"\n")?;

                                event.properties.token = token;
                            }
                        }

                        json_bytes.clear();
                        serde_json::to_writer(&mut json_bytes, &events)?;
                        this.http_client
                            .post_json(MIXPANEL_EVENTS_URL, json_bytes.into())
                            .await?;
                        anyhow::Ok(())
                    }
                    .log_err(),
                )
                .detach();
        }
    }

    fn flush_clickhouse_events(self: &Arc<Self>) {
        let mut state = self.state.lock();
        let mut events = mem::take(&mut state.clickhouse_events_queue);
        state.flush_clickhouse_events_task.take();
        drop(state);

        let this = self.clone();
        self.executor
            .spawn(
                async move {
                    let mut json_bytes = Vec::new();

                    if let Some(file) = &mut this.state.lock().log_file {
                        let file = file.as_file_mut();
                        for event in &mut events {
                            json_bytes.clear();
                            serde_json::to_writer(&mut json_bytes, event)?;
                            file.write_all(&json_bytes)?;
                            file.write(b"\n")?;
                        }
                    }

                    {
                        let state = this.state.lock();
                        json_bytes.clear();
                        serde_json::to_writer(
                            &mut json_bytes,
                            &ClickhouseEventRequestBody {
                                token: ZED_SECRET_CLIENT_TOKEN,
                                installation_id: state.installation_id.clone(),
                                app_version: state.app_version.clone(),
                                os_name: state.os_name,
                                os_version: state.os_version.clone(),
                                release_channel: state.release_channel,
                                events,
                            },
                        )?;
                    }

                    this.http_client
                        .post_json(CLICKHOUSE_EVENTS_URL.as_str(), json_bytes.into())
                        .await?;
                    anyhow::Ok(())
                }
                .log_err(),
            )
            .detach();
    }
}
