mod event_coalescer;

use crate::{ChannelId, TelemetrySettings};
use chrono::{DateTime, Utc};
use clock::SystemClock;
use collections::{HashMap, HashSet};
use futures::Future;
use gpui::{AppContext, BackgroundExecutor, Task};
use http_client::{self, HttpClient, HttpClientWithUrl, Method};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use release_channel::ReleaseChannel;
use settings::{Settings, SettingsStore};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::{env, mem, path::PathBuf, sync::Arc, time::Duration};
use sysinfo::{CpuRefreshKind, Pid, ProcessRefreshKind, RefreshKind, System};
use telemetry_events::{
    ActionEvent, AppEvent, AssistantEvent, AssistantKind, CallEvent, CpuEvent, EditEvent,
    EditorEvent, Event, EventRequestBody, EventWrapper, ExtensionEvent, InlineCompletionEvent,
    MemoryEvent, ReplEvent, SettingEvent,
};
use tempfile::NamedTempFile;
#[cfg(not(debug_assertions))]
use util::ResultExt;
use util::TryFutureExt;
use worktree::{UpdatedEntriesSet, WorktreeId};

use self::event_coalescer::EventCoalescer;

pub struct Telemetry {
    clock: Arc<dyn SystemClock>,
    http_client: Arc<HttpClientWithUrl>,
    executor: BackgroundExecutor,
    state: Arc<Mutex<TelemetryState>>,
}

struct TelemetryState {
    settings: TelemetrySettings,
    metrics_id: Option<Arc<str>>,      // Per logged-in user
    installation_id: Option<Arc<str>>, // Per app installation (different for dev, nightly, preview, and stable)
    session_id: Option<String>,        // Per app launch
    release_channel: Option<&'static str>,
    architecture: &'static str,
    events_queue: Vec<EventWrapper>,
    flush_events_task: Option<Task<()>>,
    log_file: Option<NamedTempFile>,
    is_staff: Option<bool>,
    first_event_date_time: Option<DateTime<Utc>>,
    event_coalescer: EventCoalescer,
    max_queue_size: usize,
    worktree_id_map: WorktreeIdMap,

    os_name: String,
    app_version: String,
    os_version: Option<String>,
}

#[derive(Debug)]
struct WorktreeIdMap(HashMap<String, ProjectCache>);

#[derive(Debug)]
struct ProjectCache {
    name: String,
    worktree_ids_reported: HashSet<WorktreeId>,
}

impl ProjectCache {
    fn new(name: String) -> Self {
        Self {
            name,
            worktree_ids_reported: HashSet::default(),
        }
    }
}

#[cfg(debug_assertions)]
const MAX_QUEUE_LEN: usize = 5;

#[cfg(not(debug_assertions))]
const MAX_QUEUE_LEN: usize = 50;

#[cfg(debug_assertions)]
const FLUSH_INTERVAL: Duration = Duration::from_secs(1);

#[cfg(not(debug_assertions))]
const FLUSH_INTERVAL: Duration = Duration::from_secs(60 * 5);
static ZED_CLIENT_CHECKSUM_SEED: Lazy<Option<Vec<u8>>> = Lazy::new(|| {
    option_env!("ZED_CLIENT_CHECKSUM_SEED")
        .map(|s| s.as_bytes().into())
        .or_else(|| {
            env::var("ZED_CLIENT_CHECKSUM_SEED")
                .ok()
                .map(|s| s.as_bytes().into())
        })
});

pub fn os_name() -> String {
    #[cfg(target_os = "macos")]
    {
        "macOS".to_string()
    }
    #[cfg(target_os = "linux")]
    {
        format!("Linux {}", gpui::guess_compositor())
    }

    #[cfg(target_os = "windows")]
    {
        "Windows".to_string()
    }
}

/// Note: This might do blocking IO! Only call from background threads
pub fn os_version() -> String {
    #[cfg(target_os = "macos")]
    {
        use cocoa::base::nil;
        use cocoa::foundation::NSProcessInfo;

        unsafe {
            let process_info = cocoa::foundation::NSProcessInfo::processInfo(nil);
            let version = process_info.operatingSystemVersion();
            gpui::SemanticVersion::new(
                version.majorVersion as usize,
                version.minorVersion as usize,
                version.patchVersion as usize,
            )
            .to_string()
        }
    }
    #[cfg(target_os = "linux")]
    {
        use std::path::Path;

        let content = if let Ok(file) = std::fs::read_to_string(&Path::new("/etc/os-release")) {
            file
        } else if let Ok(file) = std::fs::read_to_string(&Path::new("/usr/lib/os-release")) {
            file
        } else {
            log::error!("Failed to load /etc/os-release, /usr/lib/os-release");
            "".to_string()
        };
        let mut name = "unknown".to_string();
        let mut version = "unknown".to_string();

        for line in content.lines() {
            if line.starts_with("ID=") {
                name = line.trim_start_matches("ID=").trim_matches('"').to_string();
            }
            if line.starts_with("VERSION_ID=") {
                version = line
                    .trim_start_matches("VERSION_ID=")
                    .trim_matches('"')
                    .to_string();
            }
        }

        format!("{} {}", name, version)
    }

    #[cfg(target_os = "windows")]
    {
        let mut info = unsafe { std::mem::zeroed() };
        let status = unsafe { windows::Wdk::System::SystemServices::RtlGetVersion(&mut info) };
        if status.is_ok() {
            gpui::SemanticVersion::new(
                info.dwMajorVersion as _,
                info.dwMinorVersion as _,
                info.dwBuildNumber as _,
            )
            .to_string()
        } else {
            "unknown".to_string()
        }
    }
}

impl Telemetry {
    pub fn new(
        clock: Arc<dyn SystemClock>,
        client: Arc<HttpClientWithUrl>,
        cx: &mut AppContext,
    ) -> Arc<Self> {
        let release_channel =
            ReleaseChannel::try_global(cx).map(|release_channel| release_channel.display_name());

        TelemetrySettings::register(cx);

        let state = Arc::new(Mutex::new(TelemetryState {
            settings: *TelemetrySettings::get_global(cx),
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
            event_coalescer: EventCoalescer::new(clock.clone()),
            max_queue_size: MAX_QUEUE_LEN,
            worktree_id_map: WorktreeIdMap(HashMap::from_iter([
                (
                    "pnpm-lock.yaml".to_string(),
                    ProjectCache::new("pnpm".to_string()),
                ),
                (
                    "yarn.lock".to_string(),
                    ProjectCache::new("yarn".to_string()),
                ),
                (
                    "package.json".to_string(),
                    ProjectCache::new("node".to_string()),
                ),
            ])),

            os_version: None,
            os_name: os_name(),
            app_version: release_channel::AppVersion::global(cx).to_string(),
        }));

        #[cfg(not(debug_assertions))]
        cx.background_executor()
            .spawn({
                let state = state.clone();
                async move {
                    if let Some(tempfile) =
                        NamedTempFile::new_in(paths::logs_dir().as_path()).log_err()
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
                state.settings = *TelemetrySettings::get_global(cx);
            }
        })
        .detach();

        // TODO: Replace all hardware stuff with nested SystemSpecs json
        let this = Arc::new(Self {
            clock,
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
        state.session_id = Some(session_id);
        state.app_version = release_channel::AppVersion::global(cx).to_string();
        state.os_name = os_name();

        drop(state);

        let this = self.clone();
        cx.background_executor()
            .spawn(async move {
                let mut system = System::new_with_specifics(
                    RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
                );

                let refresh_kind = ProcessRefreshKind::new().with_cpu().with_memory();
                let current_process = Pid::from_u32(std::process::id());
                system.refresh_process_specifics(current_process, refresh_kind);

                // Waiting some amount of time before the first query is important to get a reasonable value
                // https://docs.rs/sysinfo/0.29.10/sysinfo/trait.ProcessExt.html#tymethod.cpu_usage
                const DURATION_BETWEEN_SYSTEM_EVENTS: Duration = Duration::from_secs(4 * 60);

                loop {
                    smol::Timer::after(DURATION_BETWEEN_SYSTEM_EVENTS).await;

                    let current_process = Pid::from_u32(std::process::id());
                    system.refresh_process_specifics(current_process, refresh_kind);
                    let Some(process) = system.process(current_process) else {
                        log::error!(
                            "Failed to find own process {current_process:?} in system process table"
                        );
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
        state.metrics_id.clone_from(&metrics_id);
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
        let event = Event::Editor(EditorEvent {
            file_extension,
            vim_mode,
            operation: operation.into(),
            copilot_enabled,
            copilot_enabled_for_language,
        });

        self.report_event(event)
    }

    pub fn report_inline_completion_event(
        self: &Arc<Self>,
        provider: String,
        suggestion_accepted: bool,
        file_extension: Option<String>,
    ) {
        let event = Event::InlineCompletion(InlineCompletionEvent {
            provider,
            suggestion_accepted,
            file_extension,
        });

        self.report_event(event)
    }

    pub fn report_assistant_event(
        self: &Arc<Self>,
        conversation_id: Option<String>,
        kind: AssistantKind,
        model: String,
        response_latency: Option<Duration>,
        error_message: Option<String>,
    ) {
        let event = Event::Assistant(AssistantEvent {
            conversation_id,
            kind,
            model: model.to_string(),
            response_latency,
            error_message,
        });

        self.report_event(event)
    }

    pub fn report_call_event(
        self: &Arc<Self>,
        operation: &'static str,
        room_id: Option<u64>,
        channel_id: Option<ChannelId>,
    ) {
        let event = Event::Call(CallEvent {
            operation: operation.to_string(),
            room_id,
            channel_id: channel_id.map(|cid| cid.0),
        });

        self.report_event(event)
    }

    pub fn report_cpu_event(self: &Arc<Self>, usage_as_percentage: f32, core_count: u32) {
        let event = Event::Cpu(CpuEvent {
            usage_as_percentage,
            core_count,
        });

        self.report_event(event)
    }

    pub fn report_memory_event(
        self: &Arc<Self>,
        memory_in_bytes: u64,
        virtual_memory_in_bytes: u64,
    ) {
        let event = Event::Memory(MemoryEvent {
            memory_in_bytes,
            virtual_memory_in_bytes,
        });

        self.report_event(event)
    }

    pub fn report_app_event(self: &Arc<Self>, operation: String) -> Event {
        let event = Event::App(AppEvent { operation });

        self.report_event(event.clone());

        event
    }

    pub fn report_setting_event(self: &Arc<Self>, setting: &'static str, value: String) {
        let event = Event::Setting(SettingEvent {
            setting: setting.to_string(),
            value,
        });

        self.report_event(event)
    }

    pub fn report_extension_event(self: &Arc<Self>, extension_id: Arc<str>, version: Arc<str>) {
        self.report_event(Event::Extension(ExtensionEvent {
            extension_id,
            version,
        }))
    }

    pub fn log_edit_event(self: &Arc<Self>, environment: &'static str) {
        let mut state = self.state.lock();
        let period_data = state.event_coalescer.log_event(environment);
        drop(state);

        if let Some((start, end, environment)) = period_data {
            let event = Event::Edit(EditEvent {
                duration: end.timestamp_millis() - start.timestamp_millis(),
                environment: environment.to_string(),
            });

            self.report_event(event);
        }
    }

    pub fn report_action_event(self: &Arc<Self>, source: &'static str, action: String) {
        let event = Event::Action(ActionEvent {
            source: source.to_string(),
            action,
        });

        self.report_event(event)
    }

    pub fn report_discovered_project_events(
        self: &Arc<Self>,
        worktree_id: WorktreeId,
        updated_entries_set: &UpdatedEntriesSet,
    ) {
        let project_names: Vec<String> = {
            let mut state = self.state.lock();
            state
                .worktree_id_map
                .0
                .iter_mut()
                .filter_map(|(project_file_name, project_type_telemetry)| {
                    if project_type_telemetry
                        .worktree_ids_reported
                        .contains(&worktree_id)
                    {
                        return None;
                    }

                    let project_file_found = updated_entries_set.iter().any(|(path, _, _)| {
                        path.as_ref()
                            .file_name()
                            .and_then(|name| name.to_str())
                            .map(|name_str| name_str == project_file_name)
                            .unwrap_or(false)
                    });

                    if !project_file_found {
                        return None;
                    }

                    project_type_telemetry
                        .worktree_ids_reported
                        .insert(worktree_id);

                    Some(project_type_telemetry.name.clone())
                })
                .collect()
        };

        // Done on purpose to avoid calling `self.state.lock()` multiple times
        for project_name in project_names {
            self.report_app_event(format!("open {} project", project_name));
        }
    }

    pub fn report_repl_event(
        self: &Arc<Self>,
        kernel_language: String,
        kernel_status: String,
        repl_session_id: String,
    ) {
        let event = Event::Repl(ReplEvent {
            kernel_language,
            kernel_status,
            repl_session_id,
        });

        self.report_event(event)
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

        let date_time = self.clock.utc_now();

        let milliseconds_since_first_event = match state.first_event_date_time {
            Some(first_event_date_time) => {
                date_time.timestamp_millis() - first_event_date_time.timestamp_millis()
            }
            None => {
                state.first_event_date_time = Some(date_time);
                0
            }
        };

        let signed_in = state.metrics_id.is_some();
        state.events_queue.push(EventWrapper {
            signed_in,
            milliseconds_since_first_event,
            event,
        });

        if state.installation_id.is_some() && state.events_queue.len() >= state.max_queue_size {
            drop(state);
            self.flush_events();
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
                            file.write_all(b"\n")?;
                        }
                    }

                    {
                        let state = this.state.lock();

                        let request_body = EventRequestBody {
                            installation_id: state.installation_id.as_deref().map(Into::into),
                            metrics_id: state.metrics_id.as_deref().map(Into::into),
                            session_id: state.session_id.clone(),
                            is_staff: state.is_staff,
                            app_version: state.app_version.clone(),
                            os_name: state.os_name.clone(),
                            os_version: state.os_version.clone(),
                            architecture: state.architecture.to_string(),

                            release_channel: state.release_channel.map(Into::into),
                            events,
                        };
                        json_bytes.clear();
                        serde_json::to_writer(&mut json_bytes, &request_body)?;
                    }

                    let checksum = calculate_json_checksum(&json_bytes).unwrap_or("".to_string());

                    let request = http_client::Request::builder()
                        .method(Method::POST)
                        .uri(
                            this.http_client
                                .build_zed_api_url("/telemetry/events", &[])?
                                .as_ref(),
                        )
                        .header("Content-Type", "text/plain")
                        .header("x-zed-checksum", checksum)
                        .body(json_bytes.into());

                    let response = this.http_client.send(request?).await?;
                    if response.status() != 200 {
                        log::error!("Failed to send events: HTTP {:?}", response.status());
                    }
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
    use clock::FakeSystemClock;
    use gpui::TestAppContext;
    use http_client::FakeHttpClient;

    #[gpui::test]
    fn test_telemetry_flush_on_max_queue_size(cx: &mut TestAppContext) {
        init_test(cx);
        let clock = Arc::new(FakeSystemClock::new(
            Utc.with_ymd_and_hms(1990, 4, 12, 12, 0, 0).unwrap(),
        ));
        let http = FakeHttpClient::with_200_response();
        let installation_id = Some("installation_id".to_string());
        let session_id = "session_id".to_string();

        cx.update(|cx| {
            let telemetry = Telemetry::new(clock.clone(), http, cx);

            telemetry.state.lock().max_queue_size = 4;
            telemetry.start(installation_id, session_id, cx);

            assert!(is_empty_state(&telemetry));

            let first_date_time = clock.utc_now();
            let operation = "test".to_string();

            let event = telemetry.report_app_event(operation.clone());
            assert_eq!(
                event,
                Event::App(AppEvent {
                    operation: operation.clone(),
                })
            );
            assert_eq!(telemetry.state.lock().events_queue.len(), 1);
            assert!(telemetry.state.lock().flush_events_task.is_some());
            assert_eq!(
                telemetry.state.lock().first_event_date_time,
                Some(first_date_time)
            );

            clock.advance(chrono::Duration::milliseconds(100));

            let event = telemetry.report_app_event(operation.clone());
            assert_eq!(
                event,
                Event::App(AppEvent {
                    operation: operation.clone(),
                })
            );
            assert_eq!(telemetry.state.lock().events_queue.len(), 2);
            assert!(telemetry.state.lock().flush_events_task.is_some());
            assert_eq!(
                telemetry.state.lock().first_event_date_time,
                Some(first_date_time)
            );

            clock.advance(chrono::Duration::milliseconds(100));

            let event = telemetry.report_app_event(operation.clone());
            assert_eq!(
                event,
                Event::App(AppEvent {
                    operation: operation.clone(),
                })
            );
            assert_eq!(telemetry.state.lock().events_queue.len(), 3);
            assert!(telemetry.state.lock().flush_events_task.is_some());
            assert_eq!(
                telemetry.state.lock().first_event_date_time,
                Some(first_date_time)
            );

            clock.advance(chrono::Duration::milliseconds(100));

            // Adding a 4th event should cause a flush
            let event = telemetry.report_app_event(operation.clone());
            assert_eq!(
                event,
                Event::App(AppEvent {
                    operation: operation.clone(),
                })
            );

            assert!(is_empty_state(&telemetry));
        });
    }

    #[gpui::test]
    async fn test_telemetry_flush_on_flush_interval(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let clock = Arc::new(FakeSystemClock::new(
            Utc.with_ymd_and_hms(1990, 4, 12, 12, 0, 0).unwrap(),
        ));
        let http = FakeHttpClient::with_200_response();
        let installation_id = Some("installation_id".to_string());
        let session_id = "session_id".to_string();

        cx.update(|cx| {
            let telemetry = Telemetry::new(clock.clone(), http, cx);
            telemetry.state.lock().max_queue_size = 4;
            telemetry.start(installation_id, session_id, cx);

            assert!(is_empty_state(&telemetry));

            let first_date_time = clock.utc_now();
            let operation = "test".to_string();

            let event = telemetry.report_app_event(operation.clone());
            assert_eq!(
                event,
                Event::App(AppEvent {
                    operation: operation.clone(),
                })
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

pub fn calculate_json_checksum(json: &impl AsRef<[u8]>) -> Option<String> {
    let Some(checksum_seed) = &*ZED_CLIENT_CHECKSUM_SEED else {
        return None;
    };

    let mut summer = Sha256::new();
    summer.update(checksum_seed);
    summer.update(&json);
    summer.update(checksum_seed);
    let mut checksum = String::new();
    for byte in summer.finalize().as_slice() {
        use std::fmt::Write;
        write!(&mut checksum, "{:02x}", byte).unwrap();
    }

    Some(checksum)
}
