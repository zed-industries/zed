mod event_coalescer;

use crate::{TelemetrySettings, ZED_SERVER_URL};
use chrono::{DateTime, Utc};
use futures::Future;
use gpui::{AppContext, AppMetadata, BackgroundExecutor, Task};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::Serialize;
use settings::{Settings, SettingsStore};
use std::{env, io::Write, mem, path::PathBuf, sync::Arc, time::Duration};
use sysinfo::{
    CpuRefreshKind, Pid, PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt,
};
use tempfile::NamedTempFile;
use util::http::HttpClient;
#[cfg(not(debug_assertions))]
use util::ResultExt;
use util::{channel::ReleaseChannel, TryFutureExt};

use self::event_coalescer::EventCoalescer;

pub struct Telemetry {
    http_client: Arc<dyn HttpClient>,
    executor: BackgroundExecutor,
    state: Arc<Mutex<TelemetryState>>,
}

struct TelemetryState {
    settings: TelemetrySettings,
    metrics_id: Option<Arc<str>>,      // Per logged-in user
    installation_id: Option<Arc<str>>, // Per app installation (different for dev, nightly, preview, and stable)
    session_id: Option<Arc<str>>,      // Per app launch
    release_channel: Option<&'static str>,
    app_metadata: AppMetadata,
    architecture: &'static str,
    events_queue: Vec<EventWrapper>,
    flush_events_task: Option<Task<()>>,
    log_file: Option<NamedTempFile>,
    is_staff: Option<bool>,
    first_event_date_time: Option<DateTime<Utc>>,
    event_coalescer: EventCoalescer,
    max_queue_size: usize,
}

const EVENTS_URL_PATH: &'static str = "/api/events";

lazy_static! {
    static ref EVENTS_URL: String = format!("{}{}", *ZED_SERVER_URL, EVENTS_URL_PATH);
}

#[derive(Serialize, Debug)]
struct EventRequestBody {
    installation_id: Option<Arc<str>>,
    session_id: Option<Arc<str>>,
    is_staff: Option<bool>,
    app_version: Option<String>,
    os_name: &'static str,
    os_version: Option<String>,
    architecture: &'static str,
    release_channel: Option<&'static str>,
    events: Vec<EventWrapper>,
}

#[derive(Serialize, Debug)]
struct EventWrapper {
    signed_in: bool,
    #[serde(flatten)]
    event: Event,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantKind {
    Panel,
    Inline,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Event {
    Editor {
        operation: &'static str,
        file_extension: Option<String>,
        vim_mode: bool,
        copilot_enabled: bool,
        copilot_enabled_for_language: bool,
        milliseconds_since_first_event: i64,
    },
    Copilot {
        suggestion_id: Option<String>,
        suggestion_accepted: bool,
        file_extension: Option<String>,
        milliseconds_since_first_event: i64,
    },
    Call {
        operation: &'static str,
        room_id: Option<u64>,
        channel_id: Option<u64>,
        milliseconds_since_first_event: i64,
    },
    Assistant {
        conversation_id: Option<String>,
        kind: AssistantKind,
        model: &'static str,
        milliseconds_since_first_event: i64,
    },
    Cpu {
        usage_as_percentage: f32,
        core_count: u32,
        milliseconds_since_first_event: i64,
    },
    Memory {
        memory_in_bytes: u64,
        virtual_memory_in_bytes: u64,
        milliseconds_since_first_event: i64,
    },
    App {
        operation: String,
        milliseconds_since_first_event: i64,
    },
    Setting {
        setting: &'static str,
        value: String,
        milliseconds_since_first_event: i64,
    },
    Edit {
        duration: i64,
        environment: &'static str,
        milliseconds_since_first_event: i64,
    },
    Action {
        source: &'static str,
        action: String,
        milliseconds_since_first_event: i64,
    },
}

#[cfg(debug_assertions)]
const MAX_QUEUE_LEN: usize = 5;

#[cfg(not(debug_assertions))]
const MAX_QUEUE_LEN: usize = 50;

#[cfg(debug_assertions)]
const FLUSH_INTERVAL: Duration = Duration::from_secs(1);

#[cfg(not(debug_assertions))]
const FLUSH_INTERVAL: Duration = Duration::from_secs(60 * 5);

impl Telemetry {
    pub fn new(client: Arc<dyn HttpClient>, cx: &mut AppContext) -> Arc<Self> {
        let release_channel = cx
            .try_global::<ReleaseChannel>()
            .map(|release_channel| release_channel.display_name());

        TelemetrySettings::register(cx);

        let state = Arc::new(Mutex::new(TelemetryState {
            settings: TelemetrySettings::get_global(cx).clone(),
            app_metadata: cx.app_metadata(),
            architecture: env::consts::ARCH,
            release_channel,
            installation_id: None,
            metrics_id: None,
            session_id: None,
            events_queue: Vec::new(),
            flush_events_task: None,
            log_file: None,
            is_staff: None,
            first_event_date_time: None,
            event_coalescer: EventCoalescer::new(),
            max_queue_size: MAX_QUEUE_LEN,
        }));

        #[cfg(not(debug_assertions))]
        cx.background_executor()
            .spawn({
                let state = state.clone();
                async move {
                    if let Some(tempfile) =
                        NamedTempFile::new_in(util::paths::CONFIG_DIR.as_path()).log_err()
                    {
                        state.lock().log_file = Some(tempfile);
                    }
                }
            })
            .detach();

        cx.observe_global::<SettingsStore>({
            let state = state.clone();

            move |cx| {
                let mut state = state.lock();
                state.settings = TelemetrySettings::get_global(cx).clone();
            }
        })
        .detach();

        // TODO: Replace all hardware stuff with nested SystemSpecs json
        let this = Arc::new(Self {
            http_client: client,
            executor: cx.background_executor().clone(),
            state,
        });

        // We should only ever have one instance of Telemetry, leak the subscription to keep it alive
        // rather than store in TelemetryState, complicating spawn as subscriptions are not Send
        std::mem::forget(cx.on_app_quit({
            let this = this.clone();
            move |_| this.shutdown_telemetry()
        }));

        this
    }

    #[cfg(any(test, feature = "test-support"))]
    fn shutdown_telemetry(self: &Arc<Self>) -> impl Future<Output = ()> {
        Task::ready(())
    }

    // Skip calling this function in tests.
    // TestAppContext ends up calling this function on shutdown and it panics when trying to find the TelemetrySettings
    #[cfg(not(any(test, feature = "test-support")))]
    fn shutdown_telemetry(self: &Arc<Self>) -> impl Future<Output = ()> {
        self.report_app_event("close".to_string());
        // TODO: close final edit period and make sure it's sent
        Task::ready(())
    }

    pub fn log_file_path(&self) -> Option<PathBuf> {
        Some(self.state.lock().log_file.as_ref()?.path().to_path_buf())
    }

    pub fn start(
        self: &Arc<Self>,
        installation_id: Option<String>,
        session_id: String,
        cx: &mut AppContext,
    ) {
        let mut state = self.state.lock();
        state.installation_id = installation_id.map(|id| id.into());
        state.session_id = Some(session_id.into());
        drop(state);

        let this = self.clone();
        cx.spawn(|_| async move {
            // Avoiding calling `System::new_all()`, as there have been crashes related to it
            let refresh_kind = RefreshKind::new()
                .with_memory() // For memory usage
                .with_processes(ProcessRefreshKind::everything()) // For process usage
                .with_cpu(CpuRefreshKind::everything()); // For core count

            let mut system = System::new_with_specifics(refresh_kind);

            // Avoiding calling `refresh_all()`, just update what we need
            system.refresh_specifics(refresh_kind);

            // Waiting some amount of time before the first query is important to get a reasonable value
            // https://docs.rs/sysinfo/0.29.10/sysinfo/trait.ProcessExt.html#tymethod.cpu_usage
            const DURATION_BETWEEN_SYSTEM_EVENTS: Duration = Duration::from_secs(4 * 60);

            loop {
                smol::Timer::after(DURATION_BETWEEN_SYSTEM_EVENTS).await;

                system.refresh_specifics(refresh_kind);

                let current_process = Pid::from_u32(std::process::id());
                let Some(process) = system.processes().get(&current_process) else {
                    let process = current_process;
                    log::error!("Failed to find own process {process:?} in system process table");
                    // TODO: Fire an error telemetry event
                    return;
                };

                this.report_memory_event(process.memory(), process.virtual_memory());
                this.report_cpu_event(process.cpu_usage(), system.cpus().len() as u32);
            }
        })
        .detach();
    }

    pub fn set_authenticated_user_info(
        self: &Arc<Self>,
        metrics_id: Option<String>,
        is_staff: bool,
    ) {
        let mut state = self.state.lock();

        if !state.settings.metrics {
            return;
        }

        let metrics_id: Option<Arc<str>> = metrics_id.map(|id| id.into());
        state.metrics_id = metrics_id.clone();
        state.is_staff = Some(is_staff);
        drop(state);
    }

    pub fn report_editor_event(
        self: &Arc<Self>,
        file_extension: Option<String>,
        vim_mode: bool,
        operation: &'static str,
        copilot_enabled: bool,
        copilot_enabled_for_language: bool,
    ) {
        let event = Event::Editor {
            file_extension,
            vim_mode,
            operation,
            copilot_enabled,
            copilot_enabled_for_language,
            milliseconds_since_first_event: self.milliseconds_since_first_event(Utc::now()),
        };

        self.report_event(event)
    }

    pub fn report_copilot_event(
        self: &Arc<Self>,
        suggestion_id: Option<String>,
        suggestion_accepted: bool,
        file_extension: Option<String>,
    ) {
        let event = Event::Copilot {
            suggestion_id,
            suggestion_accepted,
            file_extension,
            milliseconds_since_first_event: self.milliseconds_since_first_event(Utc::now()),
        };

        self.report_event(event)
    }

    pub fn report_assistant_event(
        self: &Arc<Self>,
        conversation_id: Option<String>,
        kind: AssistantKind,
        model: &'static str,
    ) {
        let event = Event::Assistant {
            conversation_id,
            kind,
            model,
            milliseconds_since_first_event: self.milliseconds_since_first_event(Utc::now()),
        };

        self.report_event(event)
    }

    pub fn report_call_event(
        self: &Arc<Self>,
        operation: &'static str,
        room_id: Option<u64>,
        channel_id: Option<u64>,
    ) {
        let event = Event::Call {
            operation,
            room_id,
            channel_id,
            milliseconds_since_first_event: self.milliseconds_since_first_event(Utc::now()),
        };

        self.report_event(event)
    }

    pub fn report_cpu_event(self: &Arc<Self>, usage_as_percentage: f32, core_count: u32) {
        let event = Event::Cpu {
            usage_as_percentage,
            core_count,
            milliseconds_since_first_event: self.milliseconds_since_first_event(Utc::now()),
        };

        self.report_event(event)
    }

    pub fn report_memory_event(
        self: &Arc<Self>,
        memory_in_bytes: u64,
        virtual_memory_in_bytes: u64,
    ) {
        let event = Event::Memory {
            memory_in_bytes,
            virtual_memory_in_bytes,
            milliseconds_since_first_event: self.milliseconds_since_first_event(Utc::now()),
        };

        self.report_event(event)
    }

    pub fn report_app_event(self: &Arc<Self>, operation: String) {
        self.report_app_event_with_date_time(operation, Utc::now());
    }

    fn report_app_event_with_date_time(
        self: &Arc<Self>,
        operation: String,
        date_time: DateTime<Utc>,
    ) -> Event {
        let event = Event::App {
            operation,
            milliseconds_since_first_event: self.milliseconds_since_first_event(date_time),
        };

        self.report_event(event.clone());

        event
    }

    pub fn report_setting_event(self: &Arc<Self>, setting: &'static str, value: String) {
        let event = Event::Setting {
            setting,
            value,
            milliseconds_since_first_event: self.milliseconds_since_first_event(Utc::now()),
        };

        self.report_event(event)
    }

    pub fn log_edit_event(self: &Arc<Self>, environment: &'static str) {
        let mut state = self.state.lock();
        let period_data = state.event_coalescer.log_event(environment);
        drop(state);

        if let Some((start, end, environment)) = period_data {
            let event = Event::Edit {
                duration: end.timestamp_millis() - start.timestamp_millis(),
                environment,
                milliseconds_since_first_event: self.milliseconds_since_first_event(Utc::now()),
            };

            self.report_event(event);
        }
    }

    pub fn report_action_event(self: &Arc<Self>, source: &'static str, action: String) {
        let event = Event::Action {
            source,
            action,
            milliseconds_since_first_event: self.milliseconds_since_first_event(Utc::now()),
        };

        self.report_event(event)
    }

    fn milliseconds_since_first_event(self: &Arc<Self>, date_time: DateTime<Utc>) -> i64 {
        let mut state = self.state.lock();

        match state.first_event_date_time {
            Some(first_event_date_time) => {
                date_time.timestamp_millis() - first_event_date_time.timestamp_millis()
            }
            None => {
                state.first_event_date_time = Some(date_time);
                0
            }
        }
    }

    fn report_event(self: &Arc<Self>, event: Event) {
        let mut state = self.state.lock();

        if !state.settings.metrics {
            return;
        }

        if state.flush_events_task.is_none() {
            let this = self.clone();
            let executor = self.executor.clone();
            state.flush_events_task = Some(self.executor.spawn(async move {
                executor.timer(FLUSH_INTERVAL).await;
                this.flush_events();
            }));
        }

        let signed_in = state.metrics_id.is_some();
        state.events_queue.push(EventWrapper { signed_in, event });

        if state.installation_id.is_some() {
            if state.events_queue.len() >= state.max_queue_size {
                drop(state);
                self.flush_events();
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

    pub fn flush_events(self: &Arc<Self>) {
        let mut state = self.state.lock();
        state.first_event_date_time = None;
        let mut events = mem::take(&mut state.events_queue);
        state.flush_events_task.take();
        drop(state);
        if events.is_empty() {
            return;
        }

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
                        let request_body = EventRequestBody {
                            installation_id: state.installation_id.clone(),
                            session_id: state.session_id.clone(),
                            is_staff: state.is_staff.clone(),
                            app_version: state
                                .app_metadata
                                .app_version
                                .map(|version| version.to_string()),
                            os_name: state.app_metadata.os_name,
                            os_version: state
                                .app_metadata
                                .os_version
                                .map(|version| version.to_string()),
                            architecture: state.architecture,

                            release_channel: state.release_channel,
                            events,
                        };
                        json_bytes.clear();
                        serde_json::to_writer(&mut json_bytes, &request_body)?;
                    }

                    this.http_client
                        .post_json(EVENTS_URL.as_str(), json_bytes.into())
                        .await?;
                    anyhow::Ok(())
                }
                .log_err(),
            )
            .detach();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use gpui::TestAppContext;
    use util::http::FakeHttpClient;

    #[gpui::test]
    fn test_telemetry_flush_on_max_queue_size(cx: &mut TestAppContext) {
        init_test(cx);
        let http = FakeHttpClient::with_200_response();
        let installation_id = Some("installation_id".to_string());
        let session_id = "session_id".to_string();

        cx.update(|cx| {
            let telemetry = Telemetry::new(http, cx);

            telemetry.state.lock().max_queue_size = 4;
            telemetry.start(installation_id, session_id, cx);

            assert!(is_empty_state(&telemetry));

            let first_date_time = Utc.with_ymd_and_hms(1990, 4, 12, 12, 0, 0).unwrap();
            let operation = "test".to_string();

            let event =
                telemetry.report_app_event_with_date_time(operation.clone(), first_date_time);
            assert_eq!(
                event,
                Event::App {
                    operation: operation.clone(),
                    milliseconds_since_first_event: 0
                }
            );
            assert_eq!(telemetry.state.lock().events_queue.len(), 1);
            assert!(telemetry.state.lock().flush_events_task.is_some());
            assert_eq!(
                telemetry.state.lock().first_event_date_time,
                Some(first_date_time)
            );

            let mut date_time = first_date_time + chrono::Duration::milliseconds(100);

            let event = telemetry.report_app_event_with_date_time(operation.clone(), date_time);
            assert_eq!(
                event,
                Event::App {
                    operation: operation.clone(),
                    milliseconds_since_first_event: 100
                }
            );
            assert_eq!(telemetry.state.lock().events_queue.len(), 2);
            assert!(telemetry.state.lock().flush_events_task.is_some());
            assert_eq!(
                telemetry.state.lock().first_event_date_time,
                Some(first_date_time)
            );

            date_time += chrono::Duration::milliseconds(100);

            let event = telemetry.report_app_event_with_date_time(operation.clone(), date_time);
            assert_eq!(
                event,
                Event::App {
                    operation: operation.clone(),
                    milliseconds_since_first_event: 200
                }
            );
            assert_eq!(telemetry.state.lock().events_queue.len(), 3);
            assert!(telemetry.state.lock().flush_events_task.is_some());
            assert_eq!(
                telemetry.state.lock().first_event_date_time,
                Some(first_date_time)
            );

            date_time += chrono::Duration::milliseconds(100);

            // Adding a 4th event should cause a flush
            let event = telemetry.report_app_event_with_date_time(operation.clone(), date_time);
            assert_eq!(
                event,
                Event::App {
                    operation: operation.clone(),
                    milliseconds_since_first_event: 300
                }
            );

            assert!(is_empty_state(&telemetry));
        });
    }

    #[gpui::test]
    async fn test_connection_timeout(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        init_test(cx);
        let http = FakeHttpClient::with_200_response();
        let installation_id = Some("installation_id".to_string());
        let session_id = "session_id".to_string();

        cx.update(|cx| {
            let telemetry = Telemetry::new(http, cx);
            telemetry.state.lock().max_queue_size = 4;
            telemetry.start(installation_id, session_id, cx);

            assert!(is_empty_state(&telemetry));

            let first_date_time = Utc.with_ymd_and_hms(1990, 4, 12, 12, 0, 0).unwrap();
            let operation = "test".to_string();

            let event =
                telemetry.report_app_event_with_date_time(operation.clone(), first_date_time);
            assert_eq!(
                event,
                Event::App {
                    operation: operation.clone(),
                    milliseconds_since_first_event: 0
                }
            );
            assert_eq!(telemetry.state.lock().events_queue.len(), 1);
            assert!(telemetry.state.lock().flush_events_task.is_some());
            assert_eq!(
                telemetry.state.lock().first_event_date_time,
                Some(first_date_time)
            );

            let duration = Duration::from_millis(1);

            // Test 1 millisecond before the flush interval limit is met
            executor.advance_clock(FLUSH_INTERVAL - duration);

            assert!(!is_empty_state(&telemetry));

            // Test the exact moment the flush interval limit is met
            executor.advance_clock(duration);

            assert!(is_empty_state(&telemetry));
        });
    }

    // TODO:
    // Test settings
    // Update FakeHTTPClient to keep track of the number of requests and assert on it

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    fn is_empty_state(telemetry: &Telemetry) -> bool {
        telemetry.state.lock().events_queue.is_empty()
            && telemetry.state.lock().flush_events_task.is_none()
            && telemetry.state.lock().first_event_date_time.is_none()
    }
}
