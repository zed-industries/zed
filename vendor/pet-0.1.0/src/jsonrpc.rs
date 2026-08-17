// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::find::find_and_report_envs;
use crate::find::find_python_environments_in_workspace_folder_recursive;
use crate::find::identify_python_executables_using_locators;
use crate::find::SearchScope;
use crate::locators::create_locators;
use log::{error, info, trace, warn};
use pet::initialize_tracing;
use pet::resolve::resolve_environment;
use pet_conda::Conda;
use pet_conda::CondaLocator;
use pet_core::python_environment::PythonEnvironment;
use pet_core::python_environment::PythonEnvironmentKind;
use pet_core::telemetry::refresh_performance::RefreshPerformance;
use pet_core::telemetry::TelemetryEvent;
use pet_core::{
    os_environment::{Environment, EnvironmentApi},
    reporter::Reporter,
    Configuration, Locator, RefreshStatePersistence, RefreshStateSyncScope,
};
use pet_env_var_path::get_search_paths_from_env_variables;
use pet_fs::glob::{expand_glob_pattern, expand_glob_patterns, is_recursive_glob_pattern};
use pet_fs::path::norm_case;
use pet_jsonrpc::{
    send_error, send_reply,
    server::{start_server, HandlersKeyedByMethodName},
};
use pet_poetry::Poetry;
use pet_poetry::PoetryLocator;
use pet_python_utils::cache::clear_cache;
use pet_python_utils::cache::set_cache_directory;
use pet_reporter::collect;
use pet_reporter::{cache::CacheReporter, jsonrpc};
use pet_telemetry::report_inaccuracies_identified_after_resolving;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{self, Value};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use std::{
    ops::Deref,
    panic::{self, AssertUnwindSafe},
    path::PathBuf,
    sync::{Arc, Condvar, Mutex, RwLock},
    thread,
    time::{Instant, SystemTime},
};
use tracing::info_span;

#[derive(Debug, Clone, Default)]
struct ConfigurationState {
    generation: u64,
    config: Configuration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RefreshKey {
    options: RefreshOptions,
    config_generation: u64,
}

impl RefreshKey {
    fn new(options: &RefreshOptions, config_generation: u64) -> Self {
        Self {
            options: options.clone(),
            config_generation,
        }
    }
}

#[derive(Debug)]
struct ActiveRefresh {
    key: RefreshKey,
    request_ids: Vec<u32>,
}

#[derive(Debug, Default)]
enum RefreshCoordinatorState {
    #[default]
    Idle,
    Running(ActiveRefresh),
    Completing(ActiveRefresh),
}

#[derive(Debug, Default)]
struct RefreshCoordinator {
    state: Mutex<RefreshCoordinatorState>,
    changed: Condvar,
}

enum RefreshRegistration {
    Start,
    Joined,
    Wait,
}

impl RefreshCoordinator {
    fn register_request(&self, request_id: u32, key: RefreshKey) -> RefreshRegistration {
        let mut state = self
            .state
            .lock()
            .expect("refresh coordinator mutex poisoned");
        match &mut *state {
            RefreshCoordinatorState::Idle => {
                *state = RefreshCoordinatorState::Running(ActiveRefresh {
                    key,
                    request_ids: vec![request_id],
                });
                RefreshRegistration::Start
            }
            RefreshCoordinatorState::Running(active) if active.key == key => {
                active.request_ids.push(request_id);
                RefreshRegistration::Joined
            }
            RefreshCoordinatorState::Completing(active) if active.key == key => {
                active.request_ids.push(request_id);
                RefreshRegistration::Joined
            }
            RefreshCoordinatorState::Running(_) | RefreshCoordinatorState::Completing(_) => {
                RefreshRegistration::Wait
            }
        }
    }

    fn wait_until_idle(&self) {
        let state = self
            .state
            .lock()
            .expect("refresh coordinator mutex poisoned");
        let _guard = self
            .changed
            .wait_while(state, |state| {
                !matches!(state, RefreshCoordinatorState::Idle)
            })
            .expect("refresh coordinator condvar poisoned");
    }

    fn begin_completion(&self, key: &RefreshKey) {
        let mut state = self
            .state
            .lock()
            .expect("refresh coordinator mutex poisoned");
        match std::mem::replace(&mut *state, RefreshCoordinatorState::Idle) {
            RefreshCoordinatorState::Running(active) if active.key == *key => {
                *state = RefreshCoordinatorState::Completing(active);
            }
            RefreshCoordinatorState::Running(active) => {
                *state = RefreshCoordinatorState::Running(active);
                panic!("attempted to finish refresh with unexpected key");
            }
            RefreshCoordinatorState::Completing(active) => {
                *state = RefreshCoordinatorState::Completing(active);
                panic!("attempted to begin refresh completion while already completing")
            }
            RefreshCoordinatorState::Idle => {
                panic!("attempted to finish refresh while coordinator was idle")
            }
        }
    }

    fn drain_completing_request_ids(&self, key: &RefreshKey) -> Vec<u32> {
        let mut state = self
            .state
            .lock()
            .expect("refresh coordinator mutex poisoned");
        match &mut *state {
            RefreshCoordinatorState::Completing(active) if active.key == *key => {
                std::mem::take(&mut active.request_ids)
            }
            RefreshCoordinatorState::Completing(_) => {
                panic!("attempted to drain completion requests with unexpected key")
            }
            RefreshCoordinatorState::Running(_) => {
                panic!("attempted to drain completion requests before beginning completion")
            }
            RefreshCoordinatorState::Idle => Vec::new(),
        }
    }

    fn complete_request(&self, key: &RefreshKey) -> bool {
        let mut state = self
            .state
            .lock()
            .expect("refresh coordinator mutex poisoned");
        match &mut *state {
            RefreshCoordinatorState::Completing(active) if active.key == *key => {
                if active.request_ids.is_empty() {
                    *state = RefreshCoordinatorState::Idle;
                    self.changed.notify_all();
                    true
                } else {
                    false
                }
            }
            RefreshCoordinatorState::Completing(_) => {
                panic!("attempted to complete refresh with unexpected key")
            }
            RefreshCoordinatorState::Running(_) => {
                panic!("attempted to complete refresh before beginning completion")
            }
            RefreshCoordinatorState::Idle => {
                panic!("attempted to complete refresh while coordinator was idle")
            }
        }
    }

    fn force_complete_request(&self, key: &RefreshKey) {
        let mut state = self
            .state
            .lock()
            .expect("refresh coordinator mutex poisoned");
        match &*state {
            RefreshCoordinatorState::Completing(active) if active.key == *key => {
                *state = RefreshCoordinatorState::Idle;
                self.changed.notify_all();
            }
            RefreshCoordinatorState::Running(active) if active.key == *key => {
                // Recovery path: if begin_completion() panicked, the state was
                // restored to Running before the unwind. Transition to Idle so
                // waiters are not stuck forever.
                *state = RefreshCoordinatorState::Idle;
                self.changed.notify_all();
            }
            RefreshCoordinatorState::Idle => {}
            RefreshCoordinatorState::Completing(active) => {
                // Mismatched key — another refresh owns this state. Log and
                // leave it alone; the owning refresh will clean up.
                error!(
                    "force_complete_request called with mismatched key while coordinator was Completing; caller key: {:?}, active key: {:?}",
                    key,
                    active.key
                );
            }
            RefreshCoordinatorState::Running(active) => {
                // Mismatched key — another refresh owns this state. Log and
                // leave it alone; the owning refresh will clean up.
                error!(
                    "force_complete_request called with mismatched key while coordinator was Running; caller key: {:?}, active key: {:?}",
                    key,
                    active.key
                );
            }
        }
    }
}

/// Safety guard created when a refresh thread takes ownership of the `Running`
/// state.  If the thread exits the `Start` arm without ever constructing a
/// `RefreshCompletionGuard` (e.g., because `begin_completion` panics), this
/// guard calls `force_complete_request` to transition the coordinator back to
/// `Idle`, preventing a permanent deadlock.
struct RefreshSafetyGuard<'a> {
    coordinator: &'a RefreshCoordinator,
    key: RefreshKey,
    disarmed: bool,
}

impl<'a> RefreshSafetyGuard<'a> {
    fn new(coordinator: &'a RefreshCoordinator, key: RefreshKey) -> Self {
        Self {
            coordinator,
            key,
            disarmed: false,
        }
    }

    /// Disarm the safety guard once a `RefreshCompletionGuard` takes over
    /// responsibility for the state transition.
    fn disarm(&mut self) {
        self.disarmed = true;
    }
}

impl Drop for RefreshSafetyGuard<'_> {
    fn drop(&mut self) {
        if !self.disarmed {
            self.coordinator.force_complete_request(&self.key);
        }
    }
}

struct RefreshLocators {
    locators: Arc<Vec<Arc<dyn Locator>>>,
    conda_locator: Arc<Conda>,
    poetry_locator: Arc<Poetry>,
}

struct RefreshExecution {
    result: RefreshResult,
    perf: RefreshPerformance,
    reporter: Arc<CacheReporter>,
    configuration: Arc<RwLock<ConfigurationState>>,
    refresh_generation: u64,
    conda_locator: Arc<Conda>,
    poetry_locator: Arc<Poetry>,
    conda_executable: Option<PathBuf>,
    poetry_executable: Option<PathBuf>,
}

struct RefreshCompletionGuard<'a> {
    coordinator: &'a RefreshCoordinator,
    key: RefreshKey,
    completed: bool,
}

impl<'a> RefreshCompletionGuard<'a> {
    fn begin(coordinator: &'a RefreshCoordinator, key: &RefreshKey) -> Self {
        coordinator.begin_completion(key);
        Self {
            coordinator,
            key: key.clone(),
            completed: false,
        }
    }

    fn drain_request_ids(&self) -> Vec<u32> {
        self.coordinator.drain_completing_request_ids(&self.key)
    }

    fn finish_if_no_pending(&mut self) -> bool {
        let completed = self.coordinator.complete_request(&self.key);
        if completed {
            self.completed = true;
        }
        completed
    }
}

impl Drop for RefreshCompletionGuard<'_> {
    fn drop(&mut self) {
        if !self.completed {
            self.coordinator.force_complete_request(&self.key);
        }
    }
}

fn send_refresh_replies_for_waiters(
    completion_guard: &RefreshCompletionGuard<'_>,
    result: &RefreshResult,
) {
    for request_id in completion_guard.drain_request_ids() {
        send_reply(request_id, Some(result.clone()));
    }
}

fn send_refresh_errors_for_waiters(completion_guard: &RefreshCompletionGuard<'_>, message: &str) {
    for request_id in completion_guard.drain_request_ids() {
        send_error(Some(request_id), -4, message.to_string());
    }
}

fn finish_refresh_replies(
    completion_guard: &mut RefreshCompletionGuard<'_>,
    result: &RefreshResult,
) {
    loop {
        send_refresh_replies_for_waiters(completion_guard, result);
        if completion_guard.finish_if_no_pending() {
            return;
        }
    }
}

fn finish_refresh_errors(completion_guard: &mut RefreshCompletionGuard<'_>, message: &str) {
    loop {
        send_refresh_errors_for_waiters(completion_guard, message);
        if completion_guard.finish_if_no_pending() {
            return;
        }
    }
}

fn sync_refresh_locator_state_if_current<F>(
    configuration: &RwLock<ConfigurationState>,
    refresh_generation: u64,
    sync: F,
) -> Result<(), u64>
where
    F: FnOnce(),
{
    let state = configuration.read().unwrap();
    if state.generation != refresh_generation {
        return Err(state.generation);
    }

    sync();
    Ok(())
}

struct GenerationGuardedReporter {
    reporter: Arc<dyn Reporter>,
    configuration: Arc<RwLock<ConfigurationState>>,
    refresh_generation: u64,
}

impl GenerationGuardedReporter {
    fn new(
        reporter: Arc<dyn Reporter>,
        configuration: Arc<RwLock<ConfigurationState>>,
        refresh_generation: u64,
    ) -> Self {
        Self {
            reporter,
            configuration,
            refresh_generation,
        }
    }

    fn report_if_current<F, S>(&self, report: F, on_stale: S)
    where
        F: FnOnce(&dyn Reporter),
        S: FnOnce(),
    {
        let state = self.configuration.read().unwrap();
        if state.generation == self.refresh_generation {
            report(self.reporter.as_ref());
            return;
        }

        drop(state);
        on_stale();
    }
}

impl Reporter for GenerationGuardedReporter {
    fn report_manager(&self, manager: &pet_core::manager::EnvManager) {
        self.report_if_current(
            |reporter| reporter.report_manager(manager),
            || {
                trace!(
                    "Skipping manager notification for stale generation {}",
                    self.refresh_generation
                )
            },
        );
    }

    fn report_environment(&self, env: &PythonEnvironment) {
        self.report_if_current(
            |reporter| reporter.report_environment(env),
            || {
                trace!(
                    "Skipping environment notification for stale generation {}: {:?}",
                    self.refresh_generation,
                    env.executable
                        .clone()
                        .unwrap_or(env.prefix.clone().unwrap_or_default())
                )
            },
        );
    }

    fn report_telemetry(&self, event: &TelemetryEvent) {
        self.report_if_current(
            |reporter| reporter.report_telemetry(event),
            || {
                trace!(
                    "Skipping telemetry notification for stale generation {}: {:?}",
                    self.refresh_generation,
                    event
                )
            },
        );
    }
}

pub struct Context {
    configuration: Arc<RwLock<ConfigurationState>>,
    // Serializes overlapping `configure` RPCs so that two configures cannot
    // interleave their `locator.configure()` calls. Lock ordering: always
    // acquire `configure_in_progress` BEFORE `configuration.write()`. Refresh
    // paths never touch this mutex, so no deadlock cycle is possible. See
    // #461 for why this is decoupled from the configuration `RwLock`: holding
    // the write lock across locator I/O regresses refresh latency.
    configure_in_progress: Arc<Mutex<()>>,
    locators: Arc<Vec<Arc<dyn Locator>>>,
    conda_locator: Arc<Conda>,
    os_environment: Arc<dyn Environment>,
    refresh_coordinator: RefreshCoordinator,
}

const MISSING_ENVS_AVAILABLE: u64 = u64::MAX;
const MISSING_ENVS_COMPLETED: u64 = u64::MAX - 1;

static MISSING_ENVS_REPORTING_STATE: AtomicU64 = AtomicU64::new(MISSING_ENVS_AVAILABLE);
static NEXT_REFRESH_ID: AtomicU64 = AtomicU64::new(1);

pub fn start_jsonrpc_server() {
    // Initialize tracing for performance profiling (controlled by RUST_LOG env var)
    // Note: This includes log compatibility, so we don't call jsonrpc::initialize_logger
    initialize_tracing(false);

    // These are globals for the the lifetime of the server.
    // Hence passed around as Arcs via the context.
    let environment = EnvironmentApi::new();
    let conda_locator = Arc::new(Conda::from(&environment));
    let poetry_locator = Arc::new(Poetry::from(&environment));
    let context = Context {
        locators: create_locators(conda_locator.clone(), poetry_locator.clone(), &environment),
        conda_locator,
        configuration: Arc::new(RwLock::new(ConfigurationState::default())),
        configure_in_progress: Arc::new(Mutex::new(())),
        os_environment: Arc::new(environment),
        refresh_coordinator: RefreshCoordinator::default(),
    };

    let mut handlers = HandlersKeyedByMethodName::new(Arc::new(context));
    handlers.add_request_handler("info", handle_info);
    handlers.add_request_handler("configure", handle_configure);
    handlers.add_request_handler("refresh", handle_refresh);
    handlers.add_request_handler("resolve", handle_resolve);
    handlers.add_request_handler("find", handle_find);
    handlers.add_request_handler("condaInfo", handle_conda_telemetry);
    handlers.add_request_handler("clear", handle_clear_cache);
    start_server(&handlers)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub pet_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
}

impl InfoResponse {
    fn current() -> Self {
        Self {
            pet_version: env!("CARGO_PKG_VERSION").to_string(),
            build_id: option_env!("PET_BUILD_ID")
                .filter(|value| !value.is_empty())
                .map(ToString::to_string),
            commit_sha: option_env!("PET_COMMIT_SHA")
                .filter(|value| !value.is_empty())
                .map(ToString::to_string),
        }
    }
}

pub fn handle_info(_context: Arc<Context>, id: u32, _params: Value) {
    send_reply(id, Some(InfoResponse::current()));
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureOptions {
    /// These are paths like workspace folders, where we can look for environments.
    /// Glob patterns are supported (e.g., "/home/user/projects/*").
    pub workspace_directories: Option<Vec<PathBuf>>,
    pub conda_executable: Option<PathBuf>,
    pub pipenv_executable: Option<PathBuf>,
    pub poetry_executable: Option<PathBuf>,
    /// Custom locations where environments can be found. Generally global locations where virtualenvs & the like can be found.
    /// Workspace directories should not be included into this list.
    /// Glob patterns are supported (e.g., "/home/user/envs/*").
    pub environment_directories: Option<Vec<PathBuf>>,
    /// Directory to cache the Python environment details.
    pub cache_directory: Option<PathBuf>,
}

/// Threshold for glob expansion duration before emitting a warning.
/// The client has a 30-second timeout for configure requests.
const GLOB_EXPANSION_WARN_THRESHOLD: Duration = Duration::from_secs(5);

fn expand_configure_directory_patterns(kind: &str, patterns: Vec<PathBuf>) -> Vec<PathBuf> {
    patterns
        .into_iter()
        .flat_map(|pattern| {
            let start = Instant::now();
            let expanded = expand_glob_pattern(&pattern.to_string_lossy());
            let elapsed = start.elapsed();
            trace!(
                "Expanded {} pattern '{}' in {:?}",
                kind,
                pattern.display(),
                elapsed
            );
            if elapsed >= GLOB_EXPANSION_WARN_THRESHOLD {
                warn!(
                    "Expanding {} pattern '{}' took {:?}, this may cause client timeouts",
                    kind,
                    pattern.display(),
                    elapsed
                );
            }
            expanded
        })
        .filter(|path| path.is_dir())
        .collect()
}

fn recursive_environment_patterns(patterns: &[PathBuf]) -> impl Iterator<Item = &PathBuf> {
    patterns
        .iter()
        .filter(|pattern| is_recursive_glob_pattern(&pattern.to_string_lossy()))
}

fn warn_for_recursive_environment_patterns(patterns: &[PathBuf]) {
    for pattern in recursive_environment_patterns(patterns) {
        warn!(
            "Recursive environmentDirectories pattern '{}' can make configure slow; prefer non-recursive container-directory patterns such as '<root>/envs' or '<root>/*/envs'",
            pattern.display()
        );
    }
}
pub fn handle_configure(context: Arc<Context>, id: u32, params: Value) {
    match serde_json::from_value::<ConfigureOptions>(params.clone()) {
        Ok(mut configure_options) => {
            info!("Received configure request");
            // Start in a new thread, we can have multiple requests.
            thread::spawn(move || {
                let now = Instant::now();

                // Warn before any expansion so a slow workspace pattern cannot delay
                // the actionable environmentDirectories diagnostic.
                if let Some(patterns) = configure_options.environment_directories.as_deref() {
                    warn_for_recursive_environment_patterns(patterns);
                }

                // Expand glob patterns before acquiring the write lock so we
                // don't block readers/writers while traversing the filesystem.
                let workspace_directories =
                    configure_options
                        .workspace_directories
                        .take()
                        .map(|patterns| {
                            expand_configure_directory_patterns("workspaceDirectories", patterns)
                        });
                let environment_directories =
                    configure_options
                        .environment_directories
                        .take()
                        .map(|patterns| {
                            expand_configure_directory_patterns("environmentDirectories", patterns)
                        });
                let glob_elapsed = now.elapsed();
                trace!("Glob expansion completed in {:?}", glob_elapsed);
                if glob_elapsed >= GLOB_EXPANSION_WARN_THRESHOLD {
                    warn!(
                        "Glob expansion took {:?}, this may cause client timeouts",
                        glob_elapsed
                    );
                }

                if let Err(message) = apply_configure_options(
                    context.configuration.as_ref(),
                    &context.configure_in_progress,
                    &context.locators,
                    configure_options,
                    workspace_directories,
                    environment_directories,
                ) {
                    error!("Configure failed: {message}");
                    send_error(Some(id), -4, message);
                    return;
                }
                info!("Configure completed in {:?}", now.elapsed());
                send_reply(id, None::<()>);
            });
        }
        Err(e) => {
            error!("Failed to parse configure options {:?}: {}", params, e);
            send_error(
                Some(id),
                -4,
                format!("Failed to parse configure options {params:?}: {e}"),
            );
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshOptions {
    /// If provided, then limit the search to this kind of environments.
    pub search_kind: Option<PythonEnvironmentKind>,
    /// If provided, then limit the search paths to these.
    /// Note: Search paths can also include Python exes or Python env folders.
    /// Traditionally, search paths are workspace folders.
    /// Glob patterns are supported (e.g., "/home/user/*/venv", "**/.venv").
    pub search_paths: Option<Vec<PathBuf>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResult {
    duration: u128,
    refresh_id: u64,
}

impl RefreshResult {
    pub fn new(duration: Duration, refresh_id: u64) -> RefreshResult {
        RefreshResult {
            duration: duration.as_millis(),
            refresh_id,
        }
    }
}
fn normalize_refresh_params(params: Value) -> Value {
    match params {
        Value::Null => json!({}),
        Value::Array(values) if values.is_empty() => json!({}),
        _ => params,
    }
}

fn canonicalize_refresh_options(mut options: RefreshOptions) -> RefreshOptions {
    if let Some(search_paths) = options.search_paths.take() {
        let mut expanded = expand_glob_patterns(&search_paths)
            .into_iter()
            .map(norm_case)
            .collect::<Vec<PathBuf>>();
        expanded.sort();
        expanded.dedup();
        options.search_paths = Some(expanded);
    }

    options
}

fn parse_refresh_options(params: Value) -> Result<RefreshOptions, serde_json::Error> {
    serde_json::from_value::<Option<RefreshOptions>>(normalize_refresh_params(params))
        .map(|options| canonicalize_refresh_options(options.unwrap_or_default()))
}

fn apply_configure_options(
    configuration: &RwLock<ConfigurationState>,
    configure_in_progress: &Mutex<()>,
    locators: &Arc<Vec<Arc<dyn Locator>>>,
    configure_options: ConfigureOptions,
    workspace_directories: Option<Vec<PathBuf>>,
    environment_directories: Option<Vec<PathBuf>>,
) -> Result<(), String> {
    // Phase A — Prepare: serialize concurrent configures, then briefly
    // take a read lock to snapshot the current config and compute the next
    // one. No lock is held while locators are invoked, so refresh threads
    // (which take `configuration.read()`) are not blocked by per-locator
    // I/O. See #461.
    let _configure_guard = configure_in_progress.lock().unwrap();

    let (previous_config, mut next_config, next_generation) = {
        // Read-only snapshot. `configure_in_progress` already ensures no
        // other configure thread is racing, and refresh threads only ever
        // take read locks, so a read lock here suffices.
        let state = configuration.read().unwrap();
        let previous_config = state.config.clone();
        let next_config = state.config.clone();
        let next_generation = state.generation + 1;
        (previous_config, next_config, next_generation)
    };

    next_config.workspace_directories = workspace_directories;
    next_config.conda_executable = configure_options.conda_executable;
    next_config.environment_directories = environment_directories;
    next_config.pipenv_executable = configure_options.pipenv_executable;
    next_config.poetry_executable = configure_options.poetry_executable;
    // We will not support changing the cache directories once set.
    // No point, supporting such a use case.
    let cache_directory = configure_options.cache_directory;
    if let Some(cache_directory) = cache_directory.clone() {
        next_config.cache_directory = Some(cache_directory);
    }

    trace!(
        "Configuring locators with generation {}: {:?}",
        next_generation,
        next_config
    );

    // Phase B — Configure locators with no configuration lock held so that
    // concurrent refreshes can read the (still-old) generation without
    // blocking on locator I/O.
    let mut configured_locator_count = 0;
    for locator in locators.iter() {
        if let Err(panic_payload) = panic::catch_unwind(AssertUnwindSafe(|| {
            locator.configure(&next_config);
        })) {
            let panic_message = panic_payload_message(panic_payload.as_ref());
            error!(
                "Locator configuration panicked for generation {}: {}",
                next_generation, panic_message
            );
            rollback_locator_config(
                locators,
                &previous_config,
                next_generation,
                configured_locator_count + 1,
            );
            return Err(format!(
                "Locator configuration failed for generation {next_generation}: {panic_message}"
            ));
        }
        configured_locator_count += 1;
    }

    if let Some(cache_directory) = cache_directory {
        if let Err(panic_payload) = panic::catch_unwind(AssertUnwindSafe(|| {
            set_cache_directory(cache_directory);
        })) {
            let panic_message = panic_payload_message(panic_payload.as_ref());
            error!(
                "Cache directory configuration panicked for generation {}: {}",
                next_generation, panic_message
            );
            rollback_locator_config(
                locators,
                &previous_config,
                next_generation,
                configured_locator_count,
            );
            return Err(format!(
                "Cache directory configuration failed for generation {next_generation}: {panic_message}"
            ));
        }
    }

    // Phase C — Publish: re-take the write lock and atomically install the
    // new config + generation. Refresh threads only ever observe the new
    // generation after every locator has been configured.
    {
        let mut state = configuration.write().unwrap();
        state.config = next_config;
        state.generation = next_generation;
        // Reset missing-env reporting so that the next refresh after
        // reconfiguration can trigger it again (Fixes #395). Done inside the
        // write lock to avoid a TOCTOU window with concurrent refresh threads
        // reading the generation.
        MISSING_ENVS_REPORTING_STATE.store(MISSING_ENVS_AVAILABLE, Ordering::Release);
    }

    Ok(())
}

fn rollback_locator_config(
    locators: &Arc<Vec<Arc<dyn Locator>>>,
    previous_config: &Configuration,
    failed_generation: u64,
    configured_locator_count: usize,
) {
    if let Err(panic_payload) = panic::catch_unwind(AssertUnwindSafe(|| {
        for locator in locators.iter().take(configured_locator_count) {
            locator.configure(previous_config);
        }
    })) {
        error!(
            "Rollback after failed locator configuration for generation {} also panicked: {}. Aborting process to avoid continuing with inconsistent locator state.",
            failed_generation,
            panic_payload_message(panic_payload.as_ref())
        );
        std::process::abort();
    }
}

fn panic_payload_message(panic_payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(message) = panic_payload.downcast_ref::<&str>() {
        return (*message).to_string();
    }
    if let Some(message) = panic_payload.downcast_ref::<String>() {
        return message.clone();
    }

    "unknown panic payload".to_string()
}

fn configure_locators(locators: &Arc<Vec<Arc<dyn Locator>>>, config: &Configuration) {
    for locator in locators.iter() {
        locator.configure(config);
    }
}

fn create_refresh_locators(
    environment: &dyn Environment,
    shared_conda_locator: &Conda,
) -> RefreshLocators {
    let conda_locator = Arc::new(Conda::from_shared_environment_cache(
        environment,
        shared_conda_locator,
    ));
    let poetry_locator = Arc::new(Poetry::from(environment));
    let locators = create_locators(conda_locator.clone(), poetry_locator.clone(), environment);

    RefreshLocators {
        locators,
        conda_locator,
        poetry_locator,
    }
}

fn sync_refresh_locator_state(
    target_locators: &[Arc<dyn Locator>],
    source_locators: &[Arc<dyn Locator>],
    search_scope: Option<&SearchScope>,
) {
    let sync_scope = refresh_state_sync_scope(search_scope);

    assert_eq!(
        target_locators.len(),
        source_locators.len(),
        "refresh locator graphs drifted"
    );

    for (target, source) in target_locators.iter().zip(source_locators.iter()) {
        assert_eq!(
            target.get_kind(),
            source.get_kind(),
            "refresh locator order drifted"
        );

        if !matches!(target.refresh_state(), RefreshStatePersistence::Stateless) {
            trace!(
                "Applying refresh state contract for locator {:?}: {:?}",
                target.get_kind(),
                target.refresh_state()
            );
        }

        target.sync_refresh_state_from(source.as_ref(), &sync_scope);
    }
}

fn refresh_state_sync_scope(search_scope: Option<&SearchScope>) -> RefreshStateSyncScope {
    match search_scope {
        Some(SearchScope::Workspace) => RefreshStateSyncScope::Workspace,
        Some(SearchScope::Global(kind)) => RefreshStateSyncScope::GlobalFiltered(*kind),
        None => RefreshStateSyncScope::Full,
    }
}

fn is_current_generation(
    configuration: &RwLock<ConfigurationState>,
    refresh_generation: u64,
) -> bool {
    configuration.read().unwrap().generation == refresh_generation
}

fn try_begin_missing_env_reporting(
    configuration: &RwLock<ConfigurationState>,
    refresh_generation: u64,
) -> bool {
    try_begin_missing_env_reporting_with_state(
        &MISSING_ENVS_REPORTING_STATE,
        configuration,
        refresh_generation,
    )
}

fn try_begin_missing_env_reporting_with_state(
    reporting_state: &AtomicU64,
    configuration: &RwLock<ConfigurationState>,
    refresh_generation: u64,
) -> bool {
    loop {
        let current_state = reporting_state.load(Ordering::Acquire);
        if current_state == MISSING_ENVS_COMPLETED {
            return false;
        }
        if current_state != MISSING_ENVS_AVAILABLE && current_state >= refresh_generation {
            return false;
        }

        if reporting_state
            .compare_exchange(
                current_state,
                refresh_generation,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
        {
            if is_current_generation(configuration, refresh_generation) {
                return true;
            }

            release_missing_env_reporting_if_stale_with_state(
                reporting_state,
                configuration,
                refresh_generation,
            );
            return false;
        }
    }
}

fn release_missing_env_reporting_if_stale(
    configuration: &RwLock<ConfigurationState>,
    refresh_generation: u64,
) {
    release_missing_env_reporting_if_stale_with_state(
        &MISSING_ENVS_REPORTING_STATE,
        configuration,
        refresh_generation,
    );
}

fn release_missing_env_reporting_if_stale_with_state(
    reporting_state: &AtomicU64,
    configuration: &RwLock<ConfigurationState>,
    refresh_generation: u64,
) {
    if !is_current_generation(configuration, refresh_generation) {
        let _ = reporting_state.compare_exchange(
            refresh_generation,
            MISSING_ENVS_AVAILABLE,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
    }
}

fn complete_missing_env_reporting(refresh_generation: u64) {
    complete_missing_env_reporting_with_state(&MISSING_ENVS_REPORTING_STATE, refresh_generation);
}

fn complete_missing_env_reporting_with_state(reporting_state: &AtomicU64, refresh_generation: u64) {
    let _ = reporting_state.compare_exchange(
        refresh_generation,
        MISSING_ENVS_COMPLETED,
        Ordering::AcqRel,
        Ordering::Acquire,
    );
}
fn execute_refresh(
    context: &Context,
    refresh_options: &RefreshOptions,
    configuration_state: &ConfigurationState,
) -> RefreshExecution {
    let refresh_id = NEXT_REFRESH_ID.fetch_add(1, Ordering::Relaxed);
    let refresh_locators = create_refresh_locators(
        context.os_environment.deref(),
        context.conda_locator.as_ref(),
    );
    let reporter = Arc::new(CacheReporter::new(Arc::new(
        GenerationGuardedReporter::new(
            Arc::new(jsonrpc::create_reporter(refresh_options.search_kind)),
            context.configuration.clone(),
            configuration_state.generation,
        ),
    )));

    let (config, search_scope) =
        build_refresh_config(refresh_options, configuration_state.config.clone());
    if refresh_options.search_paths.is_some() {
        trace!(
            "Expanded search paths to {} workspace dirs, {} executables",
            config
                .workspace_directories
                .as_ref()
                .map(|v| v.len())
                .unwrap_or(0),
            config.executables.as_ref().map(|v| v.len()).unwrap_or(0)
        );
    }

    configure_locators(&refresh_locators.locators, &config);

    trace!(
        "Start refreshing environments, generation: {}, config: {:?}",
        configuration_state.generation,
        config
    );
    let summary = find_and_report_envs(
        reporter.as_ref(),
        config,
        &refresh_locators.locators,
        context.os_environment.deref(),
        search_scope.clone(),
        Some(refresh_id),
    );
    let summary = summary.lock().expect("summary mutex poisoned");
    for locator in summary.locators.iter() {
        info!("Locator {:?} took {:?}", locator.0, locator.1);
    }
    for item in summary.breakdown.iter() {
        info!("Locator {} took {:?}", item.0, item.1);
    }
    trace!("Finished refreshing environments in {:?}", summary.total);

    // Refresh runs on a transient locator graph, so apply each locator's refresh-state
    // contract back into the long-lived shared locator graph only if the generation
    // still matches the configuration snapshot this refresh started with.
    if let Err(current_generation) = sync_refresh_locator_state_if_current(
        context.configuration.as_ref(),
        configuration_state.generation,
        || {
            sync_refresh_locator_state(
                context.locators.as_ref(),
                refresh_locators.locators.as_ref(),
                search_scope.as_ref(),
            );
        },
    ) {
        warn!(
            "Skipping refresh state sync for stale generation {} because current generation is {}",
            configuration_state.generation, current_generation
        );
    }

    let perf = RefreshPerformance {
        total: summary.total.as_millis(),
        locators: summary
            .locators
            .clone()
            .iter()
            .map(|(k, v)| (format!("{k:?}"), v.as_millis()))
            .collect::<BTreeMap<String, u128>>(),
        breakdown: summary
            .breakdown
            .clone()
            .iter()
            .map(|(k, v)| (k.to_string(), v.as_millis()))
            .collect::<BTreeMap<String, u128>>(),
    };

    RefreshExecution {
        result: RefreshResult::new(summary.total, refresh_id),
        perf,
        reporter,
        configuration: context.configuration.clone(),
        refresh_generation: configuration_state.generation,
        conda_locator: refresh_locators.conda_locator,
        poetry_locator: refresh_locators.poetry_locator,
        conda_executable: configuration_state.config.conda_executable.clone(),
        poetry_executable: configuration_state.config.poetry_executable.clone(),
    }
}

fn report_refresh_follow_up(execution: RefreshExecution) {
    execution
        .reporter
        .report_telemetry(&TelemetryEvent::RefreshPerformance(execution.perf));

    if try_begin_missing_env_reporting(
        execution.configuration.as_ref(),
        execution.refresh_generation,
    ) {
        let conda_locator = execution.conda_locator.clone();
        let conda_executable = execution.conda_executable.clone();
        let poetry_locator = execution.poetry_locator.clone();
        let poetry_executable = execution.poetry_executable.clone();
        let reporter_ref = execution.reporter.clone();
        let configuration = execution.configuration.clone();
        let refresh_generation = execution.refresh_generation;
        thread::spawn(move || {
            if !is_current_generation(configuration.as_ref(), refresh_generation) {
                release_missing_env_reporting_if_stale(configuration.as_ref(), refresh_generation);
                return Some(());
            }

            conda_locator.find_and_report_missing_envs(reporter_ref.as_ref(), conda_executable);
            if !is_current_generation(configuration.as_ref(), refresh_generation) {
                release_missing_env_reporting_if_stale(configuration.as_ref(), refresh_generation);
                return Some(());
            }

            poetry_locator.find_and_report_missing_envs(reporter_ref.as_ref(), poetry_executable);
            if is_current_generation(configuration.as_ref(), refresh_generation) {
                complete_missing_env_reporting(refresh_generation);
            } else {
                release_missing_env_reporting_if_stale(configuration.as_ref(), refresh_generation);
            }

            Some(())
        });
    }
}

pub fn handle_refresh(context: Arc<Context>, id: u32, params: Value) {
    match parse_refresh_options(params.clone()) {
        Ok(refresh_options) => {
            // Start in a new thread, we can have multiple requests.
            thread::spawn(move || {
                let _span = info_span!("handle_refresh",
                    search_kind = ?refresh_options.search_kind,
                    has_search_paths = refresh_options.search_paths.is_some()
                )
                .entered();

                loop {
                    let configuration_state = context.configuration.read().unwrap().clone();
                    let refresh_key =
                        RefreshKey::new(&refresh_options, configuration_state.generation);

                    match context
                        .refresh_coordinator
                        .register_request(id, refresh_key.clone())
                    {
                        RefreshRegistration::Joined => return,
                        RefreshRegistration::Wait => {
                            context.refresh_coordinator.wait_until_idle();
                        }
                        RefreshRegistration::Start => {
                            // Safety guard: if anything in this arm panics
                            // (including begin_completion), force the
                            // coordinator back to Idle so waiters are not
                            // stuck forever.
                            // Move refresh_key into the guard to avoid an
                            // extra clone of potentially large search_paths.
                            let mut safety_guard =
                                RefreshSafetyGuard::new(&context.refresh_coordinator, refresh_key);

                            let refresh_result = panic::catch_unwind(AssertUnwindSafe(|| {
                                execute_refresh(
                                    context.as_ref(),
                                    &refresh_options,
                                    &configuration_state,
                                )
                            }));

                            match refresh_result {
                                Ok(execution) => {
                                    let refresh_result = execution.result.clone();
                                    let mut completion_guard = RefreshCompletionGuard::begin(
                                        &context.refresh_coordinator,
                                        &safety_guard.key,
                                    );
                                    safety_guard.disarm();
                                    finish_refresh_replies(&mut completion_guard, &refresh_result);
                                    report_refresh_follow_up(execution);
                                }
                                Err(_) => {
                                    error!(
                                        "Refresh panicked for generation {} and options {:?}",
                                        configuration_state.generation, refresh_options
                                    );
                                    let mut completion_guard = RefreshCompletionGuard::begin(
                                        &context.refresh_coordinator,
                                        &safety_guard.key,
                                    );
                                    safety_guard.disarm();
                                    finish_refresh_errors(
                                        &mut completion_guard,
                                        "Refresh failed unexpectedly",
                                    );
                                }
                            }
                            return;
                        }
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to parse refresh {params:?}: {e}");
            send_error(
                Some(id),
                -4,
                format!("Failed to parse refresh {params:?}: {e}"),
            );
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResolveOptions {
    pub executable: PathBuf,
}

pub fn handle_resolve(context: Arc<Context>, id: u32, params: Value) {
    match serde_json::from_value::<ResolveOptions>(params.clone()) {
        Ok(request_options) => {
            let executable = request_options.executable.clone();
            // Start in a new thread, we can have multiple resolve requests.
            let environment = context.os_environment.clone();
            thread::spawn(move || {
                let now = SystemTime::now();
                trace!("Resolving env {:?}", executable);
                if let Some(result) =
                    resolve_environment(&executable, &context.locators, environment.deref())
                {
                    if let Some(resolved) = result.resolved {
                        // Gather telemetry of this resolved env and see what we got wrong.
                        let jsonrpc_reporter = jsonrpc::create_reporter(None);
                        let _ = report_inaccuracies_identified_after_resolving(
                            &jsonrpc_reporter,
                            &result.discovered,
                            &resolved,
                        );

                        trace!(
                            "Resolved env ({:?}) {executable:?} as {resolved:?}",
                            now.elapsed()
                        );
                        send_reply(id, resolved.into());
                    } else {
                        error!(
                            "Failed to resolve env {executable:?}, returning discovered env {:?}",
                            result.discovered
                        );
                        send_reply(id, result.discovered.into());
                    }
                } else {
                    error!("Failed to resolve env {executable:?}");
                    send_error(
                        Some(id),
                        -4,
                        format!("Failed to resolve env {executable:?}"),
                    );
                }
            });
        }
        Err(e) => {
            error!("Failed to parse resolve {params:?}: {e}");
            send_error(
                Some(id),
                -4,
                format!("Failed to parse resolve {params:?}: {e}"),
            );
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindOptions {
    /// Search path, can be a directory or a Python executable as well.
    /// If passing a directory, the assumption is that its a project directory (workspace folder).
    /// This is important, because any poetry/pipenv environment found will have the project directory set.
    pub search_path: PathBuf,
}

pub fn handle_find(context: Arc<Context>, id: u32, params: Value) {
    thread::spawn(
        move || match serde_json::from_value::<FindOptions>(params.clone()) {
            Ok(find_options) => {
                let now = Instant::now();
                trace!("Finding environments in {:?}", find_options.search_path);
                let global_env_search_paths: Vec<PathBuf> =
                    get_search_paths_from_env_variables(context.os_environment.as_ref());

                let collect_reporter = Arc::new(collect::create_reporter());
                let reporter = CacheReporter::new(collect_reporter.clone());
                if find_options.search_path.is_file() {
                    identify_python_executables_using_locators(
                        vec![find_options.search_path.clone()],
                        &context.locators,
                        &reporter,
                        &global_env_search_paths,
                    );
                } else {
                    find_python_environments_in_workspace_folder_recursive(
                        &find_options.search_path,
                        &reporter,
                        &context.locators,
                        &global_env_search_paths,
                        context
                            .configuration
                            .read()
                            .unwrap()
                            .config
                            .clone()
                            .environment_directories
                            .as_deref()
                            .unwrap_or(&[]),
                    );
                }

                let envs = collect_reporter
                    .environments
                    .lock()
                    .expect("environments mutex poisoned")
                    .clone();
                trace!(
                    "Find completed in {:?}, found {} environments in {:?}",
                    now.elapsed(),
                    envs.len(),
                    find_options.search_path
                );
                if envs.is_empty() {
                    send_reply(id, None::<Vec<PythonEnvironment>>);
                } else {
                    send_reply(id, envs.into());
                }
            }
            Err(e) => {
                error!("Failed to parse find {params:?}: {e}");
                send_error(
                    Some(id),
                    -4,
                    format!("Failed to parse find {params:?}: {e}"),
                );
            }
        },
    );
}

pub fn handle_conda_telemetry(context: Arc<Context>, id: u32, _params: Value) {
    thread::spawn(move || {
        trace!("Gathering conda telemetry");
        let conda_locator = context.conda_locator.clone();
        let conda_executable = context
            .configuration
            .read()
            .unwrap()
            .config
            .conda_executable
            .clone();
        let info = conda_locator.get_info_for_telemetry(conda_executable);
        trace!("Conda telemetry complete");
        send_reply(id, info.into());
    });
}

pub fn handle_clear_cache(_context: Arc<Context>, id: u32, _params: Value) {
    thread::spawn(move || {
        if let Err(e) = clear_cache() {
            error!("Failed to clear cache {:?}", e);
            send_error(Some(id), -4, format!("Failed to clear cache {e:?}"));
        } else {
            info!("Cleared cache");
            send_reply(id, None::<()>);
        }
    });
}

/// Builds the configuration and search scope based on refresh options.
/// This is extracted from handle_refresh to enable unit testing.
///
/// Returns (modified_config, search_scope)
pub(crate) fn build_refresh_config(
    refresh_options: &RefreshOptions,
    mut config: Configuration,
) -> (Configuration, Option<SearchScope>) {
    let mut search_scope = None;

    // If search_paths is provided, limit search to those paths.
    // If only search_kind is provided (without search_paths), we still search
    // workspace directories because many environment types (like Venv, VirtualEnv)
    // don't have global locations - they only exist in workspace folders.
    // The reporter will filter results to only report the requested kind.
    if let Some(ref search_paths) = refresh_options.search_paths {
        // Clear workspace directories when explicit search paths are provided.
        config.workspace_directories = None;
        // These workspace folders are only for this refresh.
        config.workspace_directories = Some(
            search_paths
                .iter()
                .filter(|p| p.is_dir())
                .cloned()
                .collect(),
        );
        config.executables = Some(
            search_paths
                .iter()
                .filter(|p| p.is_file())
                .cloned()
                .collect(),
        );
        search_scope = Some(SearchScope::Workspace);
    } else if let Some(search_kind) = refresh_options.search_kind {
        // When only search_kind is provided, keep workspace directories so that
        // workspace-based environments (Venv, VirtualEnv, etc.) can be found.
        // The search_scope tells find_and_report_envs to filter locators by kind.
        search_scope = Some(SearchScope::Global(search_kind));
    }

    (config, search_scope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pet_conda::manager::CondaManager;
    use pet_core::manager::EnvManager;
    use pet_core::manager::EnvManagerType;
    use pet_core::LocatorKind;
    use pet_core::RefreshStatePersistence;
    use std::path::PathBuf;
    use std::sync::{mpsc, Barrier, Mutex};
    use std::thread;

    #[test]
    fn recursive_environment_pattern_filter_only_returns_recursive_globs() {
        let patterns = vec![
            PathBuf::from(".venv"),
            PathBuf::from("*/.venv"),
            PathBuf::from("**/.venv"),
            PathBuf::from("/workspace/**/venv"),
        ];

        assert_eq!(
            recursive_environment_patterns(&patterns)
                .cloned()
                .collect::<Vec<_>>(),
            vec![
                PathBuf::from("**/.venv"),
                PathBuf::from("/workspace/**/venv")
            ]
        );
    }

    #[test]
    fn configure_pattern_expansion_filters_non_directories() {
        let temp = tempfile::tempdir().unwrap();
        let directory = temp.path().join("env");
        let file = temp.path().join("python.exe");
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(&file, "").unwrap();

        assert_eq!(
            expand_configure_directory_patterns(
                "environmentDirectories",
                vec![temp.path().join("*")],
            ),
            vec![directory]
        );
    }
    #[derive(Default)]
    struct RecordingReporter {
        environments: Mutex<Vec<PythonEnvironment>>,
        managers: Mutex<Vec<EnvManager>>,
        telemetry: Mutex<Vec<TelemetryEvent>>,
    }

    struct LockCheckingReporter {
        configuration: Arc<RwLock<ConfigurationState>>,
        reported: Mutex<bool>,
    }

    struct BlockingConfigureLocator {
        started: mpsc::Sender<()>,
        release: Mutex<mpsc::Receiver<()>>,
        configured_workspace_directories: Mutex<Option<Vec<PathBuf>>>,
    }

    struct PanicConfigureLocator {
        configured_workspace_directories: Mutex<Option<Vec<PathBuf>>>,
    }

    struct RecordingConfigureLocator {
        configured_workspace_directories: Mutex<Option<Vec<PathBuf>>>,
    }

    impl Reporter for RecordingReporter {
        fn report_manager(&self, manager: &EnvManager) {
            self.managers.lock().unwrap().push(manager.clone());
        }

        fn report_environment(&self, env: &PythonEnvironment) {
            self.environments.lock().unwrap().push(env.clone());
        }

        fn report_telemetry(&self, event: &TelemetryEvent) {
            self.telemetry.lock().unwrap().push(event.clone());
        }
    }

    impl Reporter for LockCheckingReporter {
        fn report_manager(&self, _manager: &EnvManager) {
            assert!(self.configuration.try_write().is_err());
            *self.reported.lock().unwrap() = true;
        }

        fn report_environment(&self, _env: &PythonEnvironment) {
            assert!(self.configuration.try_write().is_err());
            *self.reported.lock().unwrap() = true;
        }

        fn report_telemetry(&self, _event: &TelemetryEvent) {
            assert!(self.configuration.try_write().is_err());
            *self.reported.lock().unwrap() = true;
        }
    }

    impl Locator for BlockingConfigureLocator {
        fn get_kind(&self) -> LocatorKind {
            LocatorKind::Venv
        }

        fn configure(&self, config: &Configuration) {
            self.started.send(()).unwrap();
            self.release.lock().unwrap().recv().unwrap();
            *self.configured_workspace_directories.lock().unwrap() =
                config.workspace_directories.clone();
        }

        fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
            vec![PythonEnvironmentKind::Venv]
        }

        fn try_from(&self, _env: &pet_core::env::PythonEnv) -> Option<PythonEnvironment> {
            None
        }

        fn find(&self, _reporter: &dyn Reporter) {}
    }

    impl Locator for PanicConfigureLocator {
        fn get_kind(&self) -> LocatorKind {
            LocatorKind::Venv
        }

        fn configure(&self, config: &Configuration) {
            *self.configured_workspace_directories.lock().unwrap() =
                config.workspace_directories.clone();
            if config.workspace_directories.is_some() || config.conda_executable.is_some() {
                panic!("configure boom");
            }
        }

        fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
            vec![PythonEnvironmentKind::Venv]
        }

        fn try_from(&self, _env: &pet_core::env::PythonEnv) -> Option<PythonEnvironment> {
            None
        }

        fn find(&self, _reporter: &dyn Reporter) {}
    }

    impl Locator for RecordingConfigureLocator {
        fn get_kind(&self) -> LocatorKind {
            LocatorKind::Venv
        }

        fn configure(&self, config: &Configuration) {
            *self.configured_workspace_directories.lock().unwrap() =
                config.workspace_directories.clone();
        }

        fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
            vec![PythonEnvironmentKind::Venv]
        }

        fn try_from(&self, _env: &pet_core::env::PythonEnv) -> Option<PythonEnvironment> {
            None
        }

        fn find(&self, _reporter: &dyn Reporter) {}
    }

    fn make_refresh_key(generation: u64, options: RefreshOptions) -> RefreshKey {
        RefreshKey::new(&options, generation)
    }

    #[test]
    fn test_parse_refresh_options_normalizes_null_and_array() {
        assert_eq!(
            parse_refresh_options(Value::Null).unwrap(),
            RefreshOptions::default()
        );
        assert_eq!(
            parse_refresh_options(Value::Array(vec![])).unwrap(),
            RefreshOptions::default()
        );
    }

    #[test]
    fn test_info_response_uses_package_version_and_optional_build_metadata() {
        let info = InfoResponse::current();

        assert_eq!(info.pet_version, env!("CARGO_PKG_VERSION"));
        // build_id / commit_sha are populated from env vars set at compile time by CI.
        // For local dev builds they will be None; assert non-empty only when present.
        assert!(info
            .build_id
            .as_deref()
            .is_none_or(|build_id| !build_id.is_empty()));
        assert!(info
            .commit_sha
            .as_deref()
            .is_none_or(|commit_sha| !commit_sha.is_empty()));
    }

    #[test]
    fn test_info_response_serializes_camel_case_and_omits_none() {
        // Guards the JSON wire format (camelCase rename + skip_serializing_if)
        // independently of whether CI env vars were set at compile time.
        let json = serde_json::to_value(InfoResponse {
            pet_version: "1.2.3".to_string(),
            build_id: Some("42".to_string()),
            commit_sha: Some("abc123".to_string()),
        })
        .unwrap();
        assert_eq!(json["petVersion"], "1.2.3");
        assert_eq!(json["buildId"], "42");
        assert_eq!(json["commitSha"], "abc123");

        let json = serde_json::to_value(InfoResponse {
            pet_version: "1.2.3".to_string(),
            build_id: None,
            commit_sha: None,
        })
        .unwrap();
        assert_eq!(json["petVersion"], "1.2.3");
        assert!(json.get("buildId").is_none());
        assert!(json.get("commitSha").is_none());
    }

    #[test]
    fn test_parse_refresh_options_rejects_non_empty_array() {
        assert!(parse_refresh_options(json!([{"searchKind": "Conda"}])).is_err());
    }

    #[test]
    fn test_parse_refresh_options_canonicalizes_search_paths() {
        let temp_dir = tempfile::tempdir().unwrap();
        let alpha = temp_dir.path().join("alpha");
        let beta = temp_dir.path().join("beta");
        std::fs::create_dir(&alpha).unwrap();
        std::fs::create_dir(&beta).unwrap();

        let options = canonicalize_refresh_options(RefreshOptions {
            search_kind: Some(PythonEnvironmentKind::Venv),
            search_paths: Some(vec![beta.clone(), temp_dir.path().join("*"), alpha.clone()]),
        });

        assert_eq!(
            options,
            RefreshOptions {
                search_kind: Some(PythonEnvironmentKind::Venv),
                search_paths: Some(vec![norm_case(alpha), norm_case(beta)]),
            }
        );
    }

    #[test]
    fn test_sync_refresh_locator_state_if_current_matches_generation() {
        let configuration = RwLock::new(ConfigurationState {
            generation: 4,
            config: Configuration::default(),
        });
        let mut synced = false;

        let result = sync_refresh_locator_state_if_current(&configuration, 4, || {
            assert!(configuration.try_write().is_err());
            synced = true;
        });

        assert!(result.is_ok());
        assert!(synced);
    }

    #[test]
    fn test_generation_guarded_reporter_drops_stale_notifications() {
        let configuration = Arc::new(RwLock::new(ConfigurationState {
            generation: 1,
            config: Configuration::default(),
        }));
        let inner = Arc::new(RecordingReporter::default());
        let reporter = GenerationGuardedReporter::new(inner.clone(), configuration.clone(), 1);

        let environment = PythonEnvironment::new(
            Some(PathBuf::from("/tmp/python")),
            Some(PythonEnvironmentKind::Venv),
            Some(PathBuf::from("/tmp")),
            None,
            Some("3.11.0".to_string()),
        );
        let manager = EnvManager {
            executable: PathBuf::from("/tmp/conda"),
            version: Some("24.1.0".to_string()),
            tool: EnvManagerType::Conda,
        };
        let telemetry = TelemetryEvent::RefreshPerformance(RefreshPerformance {
            total: 1,
            locators: BTreeMap::new(),
            breakdown: BTreeMap::new(),
        });

        reporter.report_environment(&environment);
        reporter.report_manager(&manager);
        reporter.report_telemetry(&telemetry);

        assert_eq!(inner.environments.lock().unwrap().len(), 1);
        assert_eq!(inner.managers.lock().unwrap().len(), 1);
        assert_eq!(inner.telemetry.lock().unwrap().len(), 1);

        configuration.write().unwrap().generation = 2;

        reporter.report_environment(&environment);
        reporter.report_manager(&manager);
        reporter.report_telemetry(&telemetry);

        assert_eq!(inner.environments.lock().unwrap().len(), 1);
        assert_eq!(inner.managers.lock().unwrap().len(), 1);
        assert_eq!(inner.telemetry.lock().unwrap().len(), 1);
    }

    #[test]
    fn test_generation_guarded_reporter_holds_lock_while_reporting() {
        let configuration = Arc::new(RwLock::new(ConfigurationState {
            generation: 7,
            config: Configuration::default(),
        }));
        let inner = Arc::new(LockCheckingReporter {
            configuration: configuration.clone(),
            reported: Mutex::new(false),
        });
        let reporter = GenerationGuardedReporter::new(inner.clone(), configuration, 7);

        reporter.report_telemetry(&TelemetryEvent::RefreshPerformance(RefreshPerformance {
            total: 1,
            locators: BTreeMap::new(),
            breakdown: BTreeMap::new(),
        }));

        assert!(*inner.reported.lock().unwrap());
    }

    #[test]
    fn test_configure_publishes_state_after_shared_locators_are_configured() {
        let configuration = Arc::new(RwLock::new(ConfigurationState::default()));
        let configure_in_progress = Arc::new(Mutex::new(()));
        let (started_tx, started_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        let locator = Arc::new(BlockingConfigureLocator {
            started: started_tx,
            release: Mutex::new(release_rx),
            configured_workspace_directories: Mutex::new(None),
        });
        let locators = Arc::new(vec![locator.clone() as Arc<dyn Locator>]);
        let workspace_directories = vec![PathBuf::from("/workspace")];

        let worker = {
            let configuration = configuration.clone();
            let configure_in_progress = configure_in_progress.clone();
            let locators = locators.clone();
            let workspace_directories = workspace_directories.clone();
            thread::spawn(move || {
                apply_configure_options(
                    configuration.as_ref(),
                    &configure_in_progress,
                    &locators,
                    ConfigureOptions {
                        workspace_directories: None,
                        conda_executable: None,
                        pipenv_executable: None,
                        poetry_executable: None,
                        environment_directories: None,
                        cache_directory: None,
                    },
                    Some(workspace_directories),
                    None,
                )
                .unwrap();
                done_tx.send(()).unwrap();
            })
        };

        started_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        // Publish-after-configure invariant (#416): while the locator is
        // still being configured, the new generation must not yet be
        // observable. Under the post-#461 lock discipline a concurrent
        // reader CAN acquire the read lock (refresh must not be blocked by
        // configure), but the generation must still report the previous
        // value.
        {
            let state = configuration.read().unwrap();
            assert_eq!(state.generation, 0);
            assert!(state.config.workspace_directories.is_none());
        }

        release_tx.send(()).unwrap();
        done_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        worker.join().unwrap();

        let state = configuration.read().unwrap();
        assert_eq!(state.generation, 1);
        assert_eq!(
            state.config.workspace_directories,
            Some(workspace_directories.clone())
        );
        assert_eq!(
            *locator.configured_workspace_directories.lock().unwrap(),
            Some(workspace_directories)
        );
    }

    #[test]
    fn test_configure_panic_does_not_publish_state_or_poison_lock() {
        let configuration = RwLock::new(ConfigurationState::default());
        let configure_in_progress = Mutex::new(());
        let panic_locator = Arc::new(PanicConfigureLocator {
            configured_workspace_directories: Mutex::new(None),
        });
        let locators = Arc::new(vec![panic_locator.clone() as Arc<dyn Locator>]);
        let workspace_directories = vec![PathBuf::from("/workspace")];

        let result = apply_configure_options(
            &configuration,
            &configure_in_progress,
            &locators,
            ConfigureOptions {
                workspace_directories: None,
                conda_executable: Some(PathBuf::from("/configured/conda")),
                pipenv_executable: None,
                poetry_executable: None,
                environment_directories: None,
                cache_directory: None,
            },
            Some(workspace_directories),
            None,
        );

        assert!(result.unwrap_err().contains("configure boom"));
        let state = configuration.read().unwrap();
        assert_eq!(state.generation, 0);
        assert!(state.config.workspace_directories.is_none());
        assert!(state.config.conda_executable.is_none());
        assert!(panic_locator
            .configured_workspace_directories
            .lock()
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_configure_panic_rolls_back_previously_configured_locators() {
        let configuration = RwLock::new(ConfigurationState::default());
        let configure_in_progress = Mutex::new(());
        let recording_locator = Arc::new(RecordingConfigureLocator {
            configured_workspace_directories: Mutex::new(None),
        });
        let panic_locator = Arc::new(PanicConfigureLocator {
            configured_workspace_directories: Mutex::new(None),
        });
        let locators = Arc::new(vec![
            recording_locator.clone() as Arc<dyn Locator>,
            panic_locator.clone() as Arc<dyn Locator>,
        ]);

        let result = apply_configure_options(
            &configuration,
            &configure_in_progress,
            &locators,
            ConfigureOptions {
                workspace_directories: None,
                conda_executable: None,
                pipenv_executable: None,
                poetry_executable: None,
                environment_directories: None,
                cache_directory: None,
            },
            Some(vec![PathBuf::from("/workspace")]),
            None,
        );

        assert!(result.is_err());
        assert!(recording_locator
            .configured_workspace_directories
            .lock()
            .unwrap()
            .is_none());
        assert!(panic_locator
            .configured_workspace_directories
            .lock()
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_stale_generation_does_not_begin_missing_env_reporting() {
        let reporting_state = AtomicU64::new(MISSING_ENVS_AVAILABLE);
        let configuration = RwLock::new(ConfigurationState {
            generation: 2,
            config: Configuration::default(),
        });

        assert!(!try_begin_missing_env_reporting_with_state(
            &reporting_state,
            &configuration,
            1,
        ));
        assert_eq!(
            reporting_state.load(Ordering::Acquire),
            MISSING_ENVS_AVAILABLE
        );
    }

    #[test]
    fn test_stale_generation_releases_missing_env_reporting_slot() {
        let reporting_state = AtomicU64::new(2);
        let configuration = RwLock::new(ConfigurationState {
            generation: 3,
            config: Configuration::default(),
        });

        release_missing_env_reporting_if_stale_with_state(&reporting_state, &configuration, 2);

        assert_eq!(
            reporting_state.load(Ordering::Acquire),
            MISSING_ENVS_AVAILABLE
        );
    }

    #[test]
    fn test_newer_generation_can_claim_missing_env_reporting_after_older_reservation() {
        let reporting_state = AtomicU64::new(1);
        let configuration = RwLock::new(ConfigurationState {
            generation: 2,
            config: Configuration::default(),
        });

        assert!(try_begin_missing_env_reporting_with_state(
            &reporting_state,
            &configuration,
            2,
        ));
        assert_eq!(reporting_state.load(Ordering::Acquire), 2);
    }
    #[test]
    fn test_refresh_coordinator_joins_identical_requests() {
        let coordinator = RefreshCoordinator::default();
        let key = make_refresh_key(3, RefreshOptions::default());

        assert!(matches!(
            coordinator.register_request(1, key.clone()),
            RefreshRegistration::Start
        ));
        assert!(matches!(
            coordinator.register_request(2, key.clone()),
            RefreshRegistration::Joined
        ));
        let mut completion_guard = RefreshCompletionGuard::begin(&coordinator, &key);
        assert_eq!(completion_guard.drain_request_ids(), vec![1, 2]);
        assert!(completion_guard.finish_if_no_pending());
    }

    #[test]
    fn test_refresh_coordinator_serializes_incompatible_requests() {
        let coordinator = Arc::new(RefreshCoordinator::default());
        let first_key = make_refresh_key(1, RefreshOptions::default());
        let second_key = make_refresh_key(
            1,
            RefreshOptions {
                search_kind: Some(PythonEnvironmentKind::Venv),
                search_paths: None,
            },
        );

        assert!(matches!(
            coordinator.register_request(1, first_key.clone()),
            RefreshRegistration::Start
        ));

        let (waiting_tx, waiting_rx) = mpsc::channel();
        let worker = {
            let coordinator = coordinator.clone();
            let second_key = second_key.clone();
            thread::spawn(move || {
                let action = coordinator.register_request(2, second_key.clone());
                waiting_tx.send(()).unwrap();
                assert!(matches!(action, RefreshRegistration::Wait));

                coordinator.wait_until_idle();
                assert!(matches!(
                    coordinator.register_request(2, second_key.clone()),
                    RefreshRegistration::Start
                ));
                let mut completion_guard = RefreshCompletionGuard::begin(&coordinator, &second_key);
                let request_ids = completion_guard.drain_request_ids();
                assert!(completion_guard.finish_if_no_pending());
                request_ids
            })
        };

        waiting_rx.recv().unwrap();
        let mut completion_guard = RefreshCompletionGuard::begin(&coordinator, &first_key);
        assert_eq!(completion_guard.drain_request_ids(), vec![1]);
        assert!(completion_guard.finish_if_no_pending());
        assert_eq!(worker.join().unwrap(), vec![2]);
    }

    #[test]
    fn test_conda_refresh_state_sync_replaces_shared_caches() {
        let environment = EnvironmentApi::new();
        let shared = Conda::from(&environment);
        let refreshed = Conda::from(&environment);

        let stale_env_path = PathBuf::from("/stale/env");
        let fresh_env_path = PathBuf::from("/fresh/env");
        let stale_manager_path = PathBuf::from("/stale/conda");
        let fresh_manager_path = PathBuf::from("/fresh/conda");
        let stale_mamba_path = PathBuf::from("/stale/mamba");
        let fresh_mamba_path = PathBuf::from("/fresh/mamba");

        shared.environments.insert(
            stale_env_path.clone(),
            PythonEnvironment::new(
                Some(stale_env_path.join("python")),
                Some(PythonEnvironmentKind::Conda),
                Some(stale_env_path.clone()),
                None,
                Some("3.10.0".to_string()),
            ),
        );
        shared.managers.insert(
            stale_manager_path.clone(),
            CondaManager {
                executable: stale_manager_path.clone(),
                version: Some("23.1.0".to_string()),
                conda_dir: Some(PathBuf::from("/stale")),
                manager_type: EnvManagerType::Conda,
            },
        );
        shared.mamba_managers.insert(
            stale_mamba_path.clone(),
            CondaManager {
                executable: stale_mamba_path.clone(),
                version: Some("1.5.0".to_string()),
                conda_dir: Some(PathBuf::from("/stale")),
                manager_type: EnvManagerType::Mamba,
            },
        );

        refreshed.environments.insert(
            fresh_env_path.clone(),
            PythonEnvironment::new(
                Some(fresh_env_path.join("python")),
                Some(PythonEnvironmentKind::Conda),
                Some(fresh_env_path.clone()),
                None,
                Some("3.11.0".to_string()),
            ),
        );
        refreshed.managers.insert(
            fresh_manager_path.clone(),
            CondaManager {
                executable: fresh_manager_path.clone(),
                version: Some("24.1.0".to_string()),
                conda_dir: Some(PathBuf::from("/fresh")),
                manager_type: EnvManagerType::Conda,
            },
        );
        refreshed.mamba_managers.insert(
            fresh_mamba_path.clone(),
            CondaManager {
                executable: fresh_mamba_path.clone(),
                version: Some("2.0.0".to_string()),
                conda_dir: Some(PathBuf::from("/fresh")),
                manager_type: EnvManagerType::Mamba,
            },
        );

        assert_eq!(
            shared.refresh_state(),
            RefreshStatePersistence::SyncedDiscoveryState
        );

        shared.sync_refresh_state_from(&refreshed, &RefreshStateSyncScope::Full);

        assert_eq!(shared.environments.len(), 1);
        assert!(!shared.environments.contains_key(&stale_env_path));
        assert!(shared.environments.contains_key(&fresh_env_path));

        assert_eq!(shared.managers.len(), 1);
        assert!(!shared.managers.contains_key(&stale_manager_path));
        assert!(shared.managers.contains_key(&fresh_manager_path));

        assert_eq!(shared.mamba_managers.len(), 1);
        assert!(!shared.mamba_managers.contains_key(&stale_mamba_path));
        assert!(shared.mamba_managers.contains_key(&fresh_mamba_path));
    }

    #[test]
    fn test_workspace_refresh_does_not_replace_shared_conda_state() {
        let environment = EnvironmentApi::new();
        let shared = Arc::new(Conda::from(&environment));
        let refreshed = Arc::new(Conda::from(&environment));

        let stale_env_path = PathBuf::from("/stale/env");
        let fresh_env_path = PathBuf::from("/fresh/env");

        shared.environments.insert(
            stale_env_path.clone(),
            PythonEnvironment::new(
                Some(stale_env_path.join("python")),
                Some(PythonEnvironmentKind::Conda),
                Some(stale_env_path.clone()),
                None,
                Some("3.10.0".to_string()),
            ),
        );
        refreshed.environments.insert(
            fresh_env_path.clone(),
            PythonEnvironment::new(
                Some(fresh_env_path.join("python")),
                Some(PythonEnvironmentKind::Conda),
                Some(fresh_env_path.clone()),
                None,
                Some("3.11.0".to_string()),
            ),
        );

        sync_refresh_locator_state(
            &[shared.clone() as Arc<dyn Locator>],
            &[refreshed as Arc<dyn Locator>],
            Some(&SearchScope::Workspace),
        );

        assert_eq!(shared.environments.len(), 1);
        assert!(shared.environments.contains_key(&stale_env_path));
        assert!(!shared.environments.contains_key(&fresh_env_path));
    }

    #[test]
    fn test_kind_filtered_refresh_skips_unrelated_conda_state_sync() {
        let environment = EnvironmentApi::new();
        let shared = Arc::new(Conda::from(&environment));
        let refreshed = Arc::new(Conda::from(&environment));

        let stale_env_path = PathBuf::from("/stale/env");
        let fresh_env_path = PathBuf::from("/fresh/env");

        shared.environments.insert(
            stale_env_path.clone(),
            PythonEnvironment::new(
                Some(stale_env_path.join("python")),
                Some(PythonEnvironmentKind::Conda),
                Some(stale_env_path.clone()),
                None,
                Some("3.10.0".to_string()),
            ),
        );
        refreshed.environments.insert(
            fresh_env_path.clone(),
            PythonEnvironment::new(
                Some(fresh_env_path.join("python")),
                Some(PythonEnvironmentKind::Conda),
                Some(fresh_env_path.clone()),
                None,
                Some("3.11.0".to_string()),
            ),
        );

        sync_refresh_locator_state(
            &[shared.clone() as Arc<dyn Locator>],
            &[refreshed as Arc<dyn Locator>],
            Some(&SearchScope::Global(PythonEnvironmentKind::Venv)),
        );

        assert_eq!(shared.environments.len(), 1);
        assert!(shared.environments.contains_key(&stale_env_path));
        assert!(!shared.environments.contains_key(&fresh_env_path));
    }

    #[test]
    fn test_stale_generation_does_not_sync_refresh_state() {
        let configuration = RwLock::new(ConfigurationState {
            generation: 2,
            config: Configuration::default(),
        });
        let mut synced = false;

        let result = sync_refresh_locator_state_if_current(&configuration, 1, || {
            synced = true;
        });

        assert_eq!(result, Err(2));
        assert!(!synced);
    }

    #[test]
    fn test_locator_graph_refresh_state_contracts_are_explicit() {
        let environment = EnvironmentApi::new();
        let conda_locator = Arc::new(Conda::from(&environment));
        let poetry_locator = Arc::new(Poetry::from(&environment));
        let locators = create_locators(conda_locator, poetry_locator, &environment);

        let actual = locators
            .iter()
            .map(|locator| (locator.get_kind(), locator.refresh_state()))
            .collect::<Vec<_>>();

        let expected = vec![
            #[cfg(windows)]
            (
                LocatorKind::WindowsStore,
                RefreshStatePersistence::SyncedDiscoveryState,
            ),
            #[cfg(windows)]
            (
                LocatorKind::WindowsRegistry,
                RefreshStatePersistence::SyncedDiscoveryState,
            ),
            #[cfg(windows)]
            (
                LocatorKind::WinPython,
                RefreshStatePersistence::SyncedDiscoveryState,
            ),
            (
                LocatorKind::PyEnv,
                RefreshStatePersistence::SelfHydratingCache,
            ),
            (LocatorKind::Pixi, RefreshStatePersistence::Stateless),
            (
                LocatorKind::Conda,
                RefreshStatePersistence::SyncedDiscoveryState,
            ),
            (LocatorKind::Uv, RefreshStatePersistence::ConfiguredOnly),
            (
                LocatorKind::Poetry,
                RefreshStatePersistence::SyncedDiscoveryState,
            ),
            (LocatorKind::PipEnv, RefreshStatePersistence::ConfiguredOnly),
            (LocatorKind::Hatch, RefreshStatePersistence::ConfiguredOnly),
            (
                LocatorKind::VirtualEnvWrapper,
                RefreshStatePersistence::Stateless,
            ),
            (LocatorKind::Venv, RefreshStatePersistence::Stateless),
            (LocatorKind::VirtualEnv, RefreshStatePersistence::Stateless),
            #[cfg(unix)]
            (LocatorKind::Homebrew, RefreshStatePersistence::Stateless),
            #[cfg(target_os = "macos")]
            (LocatorKind::MacXCode, RefreshStatePersistence::Stateless),
            #[cfg(target_os = "macos")]
            (
                LocatorKind::MacCommandLineTools,
                RefreshStatePersistence::Stateless,
            ),
            #[cfg(target_os = "macos")]
            (
                LocatorKind::MacPythonOrg,
                RefreshStatePersistence::Stateless,
            ),
            #[cfg(all(unix, not(target_os = "macos")))]
            (
                LocatorKind::LinuxGlobal,
                RefreshStatePersistence::SelfHydratingCache,
            ),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_refresh_coordinator_does_not_join_different_generations() {
        let coordinator = RefreshCoordinator::default();
        let options = RefreshOptions::default();
        let first_key = make_refresh_key(1, options.clone());
        let second_key = make_refresh_key(2, options);

        assert!(matches!(
            coordinator.register_request(10, first_key.clone()),
            RefreshRegistration::Start
        ));
        assert!(matches!(
            coordinator.register_request(11, second_key.clone()),
            RefreshRegistration::Wait
        ));
        let mut completion_guard = RefreshCompletionGuard::begin(&coordinator, &first_key);
        assert_eq!(completion_guard.drain_request_ids(), vec![10]);
        assert!(completion_guard.finish_if_no_pending());
        assert!(matches!(
            coordinator.register_request(11, second_key.clone()),
            RefreshRegistration::Start
        ));
        let mut completion_guard = RefreshCompletionGuard::begin(&coordinator, &second_key);
        assert_eq!(completion_guard.drain_request_ids(), vec![11]);
        assert!(completion_guard.finish_if_no_pending());
    }

    #[test]
    fn test_refresh_coordinator_reuses_same_key_during_completion() {
        let coordinator = RefreshCoordinator::default();
        let key = make_refresh_key(1, RefreshOptions::default());

        assert!(matches!(
            coordinator.register_request(1, key.clone()),
            RefreshRegistration::Start
        ));

        let mut completion_guard = RefreshCompletionGuard::begin(&coordinator, &key);
        assert_eq!(completion_guard.drain_request_ids(), vec![1]);

        assert!(matches!(
            coordinator.register_request(2, key.clone()),
            RefreshRegistration::Joined
        ));
        assert_eq!(completion_guard.drain_request_ids(), vec![2]);
        assert!(completion_guard.finish_if_no_pending());
    }

    #[test]
    fn test_refresh_coordinator_waits_for_completion_boundary() {
        let coordinator = Arc::new(RefreshCoordinator::default());
        let first_key = make_refresh_key(1, RefreshOptions::default());
        let second_key = make_refresh_key(
            1,
            RefreshOptions {
                search_kind: Some(PythonEnvironmentKind::Venv),
                search_paths: None,
            },
        );

        assert!(matches!(
            coordinator.register_request(1, first_key.clone()),
            RefreshRegistration::Start
        ));
        let mut completion_guard = RefreshCompletionGuard::begin(&coordinator, &first_key);
        assert_eq!(completion_guard.drain_request_ids(), vec![1]);

        let (state_tx, state_rx) = mpsc::channel();
        let worker = {
            let coordinator = coordinator.clone();
            let second_key = second_key.clone();
            thread::spawn(move || {
                assert!(matches!(
                    coordinator.register_request(2, second_key.clone()),
                    RefreshRegistration::Wait
                ));
                state_tx.send("waiting").unwrap();
                coordinator.wait_until_idle();
                state_tx.send("idle").unwrap();
                assert!(matches!(
                    coordinator.register_request(2, second_key.clone()),
                    RefreshRegistration::Start
                ));
                let mut completion_guard = RefreshCompletionGuard::begin(&coordinator, &second_key);
                let request_ids = completion_guard.drain_request_ids();
                assert!(completion_guard.finish_if_no_pending());
                request_ids
            })
        };

        assert_eq!(state_rx.recv().unwrap(), "waiting");
        assert!(state_rx.try_recv().is_err());

        assert!(completion_guard.finish_if_no_pending());

        assert_eq!(state_rx.recv().unwrap(), "idle");
        assert_eq!(worker.join().unwrap(), vec![2]);
    }

    #[test]
    fn test_refresh_completion_guard_releases_waiters_on_unwind() {
        let coordinator = Arc::new(RefreshCoordinator::default());
        let first_key = make_refresh_key(1, RefreshOptions::default());
        let second_key = make_refresh_key(
            1,
            RefreshOptions {
                search_kind: Some(PythonEnvironmentKind::Venv),
                search_paths: None,
            },
        );

        assert!(matches!(
            coordinator.register_request(1, first_key.clone()),
            RefreshRegistration::Start
        ));

        let (state_tx, state_rx) = mpsc::channel();
        let waiter = {
            let coordinator = coordinator.clone();
            let second_key = second_key.clone();
            thread::spawn(move || {
                assert!(matches!(
                    coordinator.register_request(2, second_key.clone()),
                    RefreshRegistration::Wait
                ));
                state_tx.send("waiting").unwrap();
                coordinator.wait_until_idle();
                state_tx.send("idle").unwrap();
                assert!(matches!(
                    coordinator.register_request(2, second_key.clone()),
                    RefreshRegistration::Start
                ));
                let mut completion_guard = RefreshCompletionGuard::begin(&coordinator, &second_key);
                let request_ids = completion_guard.drain_request_ids();
                assert!(completion_guard.finish_if_no_pending());
                request_ids
            })
        };

        assert_eq!(state_rx.recv().unwrap(), "waiting");
        let panic_result = panic::catch_unwind(AssertUnwindSafe(|| {
            let completion_guard = RefreshCompletionGuard::begin(coordinator.as_ref(), &first_key);
            assert_eq!(completion_guard.drain_request_ids(), vec![1]);
            panic!("forced completion panic");
        }));
        assert!(panic_result.is_err());

        assert_eq!(state_rx.recv().unwrap(), "idle");
        assert_eq!(waiter.join().unwrap(), vec![2]);
    }

    /// Test for https://github.com/microsoft/python-environment-tools/issues/151
    /// Verifies that when searchKind is provided (without searchPaths),
    /// workspace_directories are NOT cleared.
    ///
    /// The bug was that handle_refresh cleared workspace_directories when searchKind
    /// was provided, preventing discovery of workspace-based environments like venvs.
    #[test]
    fn test_search_kind_preserves_workspace_directories() {
        let workspace = PathBuf::from("/test/workspace");
        let config = Configuration {
            workspace_directories: Some(vec![workspace.clone()]),
            ..Default::default()
        };

        let refresh_options = RefreshOptions {
            search_kind: Some(PythonEnvironmentKind::Venv),
            search_paths: None,
        };

        let (result_config, search_scope) = build_refresh_config(&refresh_options, config);

        // CRITICAL: workspace_directories must be preserved when only search_kind is provided
        assert_eq!(
            result_config.workspace_directories,
            Some(vec![workspace]),
            "workspace_directories should NOT be cleared when only searchKind is provided"
        );

        // search_scope should be Global with the requested kind
        assert!(
            matches!(
                search_scope,
                Some(SearchScope::Global(PythonEnvironmentKind::Venv))
            ),
            "search_scope should be Global(Venv)"
        );
    }

    /// Test that when searchPaths is provided, workspace_directories ARE replaced.
    #[test]
    fn test_search_paths_replaces_workspace_directories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let search_dir = temp_dir.path().join("search_path");
        std::fs::create_dir(&search_dir).unwrap();

        let original_workspace = PathBuf::from("/original/workspace");
        let config = Configuration {
            workspace_directories: Some(vec![original_workspace]),
            ..Default::default()
        };

        let refresh_options = RefreshOptions {
            search_kind: None,
            search_paths: Some(vec![search_dir.clone()]),
        };

        let (result_config, search_scope) = build_refresh_config(&refresh_options, config);

        // workspace_directories should be replaced with the search_paths directory
        assert_eq!(
            result_config.workspace_directories,
            Some(vec![search_dir]),
            "workspace_directories should be replaced by search_paths"
        );

        assert!(
            matches!(search_scope, Some(SearchScope::Workspace)),
            "search_scope should be Workspace"
        );
    }

    /// Test that when neither searchKind nor searchPaths is provided,
    /// configuration is unchanged.
    #[test]
    fn test_no_options_preserves_config() {
        let workspace = PathBuf::from("/test/workspace");
        let config = Configuration {
            workspace_directories: Some(vec![workspace.clone()]),
            ..Default::default()
        };

        let refresh_options = RefreshOptions {
            search_kind: None,
            search_paths: None,
        };

        let (result_config, search_scope) = build_refresh_config(&refresh_options, config);

        assert_eq!(
            result_config.workspace_directories,
            Some(vec![workspace]),
            "workspace_directories should be preserved when no options provided"
        );

        assert!(
            search_scope.is_none(),
            "search_scope should be None when no options provided"
        );
    }

    #[test]
    fn test_search_paths_use_already_canonicalized_paths() {
        let temp_dir = tempfile::tempdir().unwrap();
        let workspace_dir = temp_dir.path().join("workspace");
        let executable = temp_dir.path().join("python.exe");
        std::fs::create_dir(&workspace_dir).unwrap();
        std::fs::write(&executable, b"python").unwrap();

        let config = Configuration::default();
        let refresh_options = RefreshOptions {
            search_kind: None,
            search_paths: Some(vec![workspace_dir.clone(), executable.clone()]),
        };

        let (result_config, search_scope) = build_refresh_config(&refresh_options, config);

        assert_eq!(
            result_config.workspace_directories,
            Some(vec![workspace_dir])
        );
        assert_eq!(result_config.executables, Some(vec![executable]));
        assert!(matches!(search_scope, Some(SearchScope::Workspace)));
    }

    /// Test for #396: force_complete_request recovers from Running state.
    /// When begin_completion() cannot be reached (e.g., the thread panics before
    /// constructing a RefreshCompletionGuard), force_complete_request must still
    /// transition Running → Idle to unblock waiters.
    #[test]
    fn test_force_complete_request_recovers_from_running_state() {
        let coordinator = RefreshCoordinator::default();
        let key = make_refresh_key(1, RefreshOptions::default());

        // State → Running(key)
        assert!(matches!(
            coordinator.register_request(1, key.clone()),
            RefreshRegistration::Start
        ));

        // Simulate recovery: force_complete_request from Running state.
        coordinator.force_complete_request(&key);

        // Verify we're back to Idle and can start a new refresh.
        assert!(matches!(
            coordinator.register_request(2, key.clone()),
            RefreshRegistration::Start
        ));
    }

    /// Test for #396: RefreshSafetyGuard transitions Running → Idle on drop
    /// when begin_completion is never reached.
    #[test]
    fn test_safety_guard_recovers_running_state_on_drop() {
        let coordinator = Arc::new(RefreshCoordinator::default());
        let key = make_refresh_key(1, RefreshOptions::default());
        let other_key = make_refresh_key(
            1,
            RefreshOptions {
                search_kind: Some(PythonEnvironmentKind::Venv),
                search_paths: None,
            },
        );

        assert!(matches!(
            coordinator.register_request(1, key.clone()),
            RefreshRegistration::Start
        ));

        let (state_tx, state_rx) = mpsc::channel();
        let waiter = {
            let coordinator = coordinator.clone();
            let other_key = other_key.clone();
            thread::spawn(move || {
                // Different key → returns Wait (not Joined).
                assert!(matches!(
                    coordinator.register_request(2, other_key.clone()),
                    RefreshRegistration::Wait
                ));
                state_tx.send("waiting").unwrap();
                coordinator.wait_until_idle();
                state_tx.send("idle").unwrap();
            })
        };

        assert_eq!(state_rx.recv().unwrap(), "waiting");

        // Create and immediately drop the safety guard without disarming it.
        // This simulates the thread dying before begin_completion.
        {
            let _guard = RefreshSafetyGuard::new(&coordinator, key.clone());
        }

        // Waiter should be unblocked.
        assert_eq!(state_rx.recv().unwrap(), "idle");
        waiter.join().unwrap();
    }

    /// Test for #396: RefreshSafetyGuard does NOT interfere when disarmed
    /// (normal path where RefreshCompletionGuard takes over).
    #[test]
    fn test_safety_guard_disarmed_does_not_interfere() {
        let coordinator = RefreshCoordinator::default();
        let key = make_refresh_key(1, RefreshOptions::default());

        assert!(matches!(
            coordinator.register_request(1, key.clone()),
            RefreshRegistration::Start
        ));

        {
            let mut safety_guard = RefreshSafetyGuard::new(&coordinator, key.clone());
            let mut completion_guard = RefreshCompletionGuard::begin(&coordinator, &key);
            safety_guard.disarm();
            let ids = completion_guard.drain_request_ids();
            assert_eq!(ids, vec![1]);
            assert!(completion_guard.finish_if_no_pending());
        }

        // Should be Idle — can start a new refresh.
        assert!(matches!(
            coordinator.register_request(2, key.clone()),
            RefreshRegistration::Start
        ));
    }

    /// Test for #395: configure resets missing-env state so that subsequent
    /// refreshes can trigger reporting again.
    #[test]
    fn test_configure_resets_completed_missing_env_reporting() {
        let reporting_state = AtomicU64::new(MISSING_ENVS_AVAILABLE);
        let configuration = Arc::new(RwLock::new(ConfigurationState {
            generation: 1,
            config: Configuration::default(),
        }));

        assert!(try_begin_missing_env_reporting_with_state(
            &reporting_state,
            configuration.as_ref(),
            1,
        ));
        complete_missing_env_reporting_with_state(&reporting_state, 1);
        assert!(!try_begin_missing_env_reporting_with_state(
            &reporting_state,
            configuration.as_ref(),
            1,
        ));

        {
            let mut state = configuration.write().unwrap();
            state.generation = 2;
            reporting_state.store(MISSING_ENVS_AVAILABLE, Ordering::Release);
        }

        assert!(try_begin_missing_env_reporting_with_state(
            &reporting_state,
            configuration.as_ref(),
            2,
        ));
    }
    /// Test for #461: refresh-side `configuration.read()` callers must not
    /// block on the configure thread while it iterates `locator.configure()`.
    #[test]
    fn test_refresh_not_blocked_by_in_flight_configure() {
        let configuration = Arc::new(RwLock::new(ConfigurationState::default()));
        let configure_in_progress = Arc::new(Mutex::new(()));
        let (started_tx, started_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        let locator = Arc::new(BlockingConfigureLocator {
            started: started_tx,
            release: Mutex::new(release_rx),
            configured_workspace_directories: Mutex::new(None),
        });
        let locators = Arc::new(vec![locator.clone() as Arc<dyn Locator>]);

        let worker = {
            let configuration = configuration.clone();
            let configure_in_progress = configure_in_progress.clone();
            let locators = locators.clone();
            thread::spawn(move || {
                apply_configure_options(
                    configuration.as_ref(),
                    &configure_in_progress,
                    &locators,
                    ConfigureOptions {
                        workspace_directories: None,
                        conda_executable: None,
                        pipenv_executable: None,
                        poetry_executable: None,
                        environment_directories: None,
                        cache_directory: None,
                    },
                    Some(vec![PathBuf::from("/workspace")]),
                    None,
                )
                .unwrap();
                done_tx.send(()).unwrap();
            })
        };

        // Wait for the configure thread to enter the locator's `configure()`.
        // At this point Phase B is in progress with no `configuration` lock
        // held.
        started_rx.recv_timeout(Duration::from_secs(5)).unwrap();

        let (read_done_tx, read_done_rx) = mpsc::channel();
        let read_worker = {
            let configuration = configuration.clone();
            thread::spawn(move || {
                // Exercise the three read sites called out in #461 from another
                // thread. If a regression holds `configuration.write()` during
                // Phase B, this worker blocks and the main test thread can fail
                // with a bounded timeout instead of deadlocking.
                let read_start = Instant::now();
                {
                    // `execute_refresh` snapshot read path.
                    let _state = configuration.read().unwrap();
                }
                // `sync_refresh_locator_state_if_current` read path.
                let _ = sync_refresh_locator_state_if_current(configuration.as_ref(), 0, || {});
                // `GenerationGuardedReporter::report_if_current` read path. The
                // generation here matches the still-old generation (0) so the
                // inner report is invoked, exercising the full read-lock-then-call
                // path.
                let inner = Arc::new(RecordingReporter::default());
                let reporter = GenerationGuardedReporter::new(inner.clone(), configuration, 0);
                let env = PythonEnvironment::new(
                    Some(PathBuf::from("/tmp/python")),
                    Some(PythonEnvironmentKind::Venv),
                    Some(PathBuf::from("/tmp")),
                    None,
                    Some("3.11.0".to_string()),
                );
                reporter.report_environment(&env);

                let _ = read_done_tx.send((
                    read_start.elapsed(),
                    inner.environments.lock().unwrap().len(),
                ));
            })
        };

        let (read_elapsed, reported_count) = match read_done_rx.recv_timeout(Duration::from_secs(2))
        {
            Ok(result) => result,
            Err(error) => {
                let _ = release_tx.send(());
                let _ = done_rx.recv_timeout(Duration::from_secs(5));
                worker.join().unwrap();
                panic!("concurrent refresh reads should not block on configure: {error}");
            }
        };
        read_worker.join().unwrap();
        assert!(
            read_elapsed < Duration::from_secs(2),
            "concurrent refresh reads should not block on configure (took {read_elapsed:?})"
        );
        // The reporter should also have actually reported because the
        // generation has not yet advanced.
        assert_eq!(reported_count, 1);

        // Release the locator and confirm configure completes and publishes.
        release_tx.send(()).unwrap();
        done_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        worker.join().unwrap();

        let state = configuration.read().unwrap();
        assert_eq!(state.generation, 1);
    }

    /// Test for #461: two overlapping `apply_configure_options` calls must
    /// fully serialize — their per-locator `configure()` invocations cannot
    /// interleave.
    #[test]
    fn test_concurrent_configures_serialize() {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum Event {
            Enter(u32),
            Exit(u32),
        }

        struct OrderingLocator {
            id: u32,
            events: Arc<Mutex<Vec<Event>>>,
            entered: mpsc::Sender<u32>,
            release: Arc<Mutex<mpsc::Receiver<()>>>,
        }

        impl Locator for OrderingLocator {
            fn get_kind(&self) -> LocatorKind {
                LocatorKind::Venv
            }
            fn configure(&self, _config: &Configuration) {
                self.events.lock().unwrap().push(Event::Enter(self.id));
                self.entered.send(self.id).unwrap();
                self.release.lock().unwrap().recv().unwrap();
                self.events.lock().unwrap().push(Event::Exit(self.id));
            }
            fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
                vec![PythonEnvironmentKind::Venv]
            }
            fn try_from(&self, _env: &pet_core::env::PythonEnv) -> Option<PythonEnvironment> {
                None
            }
            fn find(&self, _reporter: &dyn Reporter) {}
        }

        let configuration = Arc::new(RwLock::new(ConfigurationState::default()));
        let configure_in_progress = Arc::new(Mutex::new(()));
        let events = Arc::new(Mutex::new(Vec::<Event>::new()));
        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let release = Arc::new(Mutex::new(release_rx));
        let start_barrier = Arc::new(Barrier::new(3));

        let make_locators = |id: u32| -> Arc<Vec<Arc<dyn Locator>>> {
            Arc::new(vec![Arc::new(OrderingLocator {
                id,
                events: events.clone(),
                entered: entered_tx.clone(),
                release: release.clone(),
            }) as Arc<dyn Locator>])
        };

        let locators_a = make_locators(1);
        let locators_b = make_locators(2);

        let worker_a = {
            let configuration = configuration.clone();
            let configure_in_progress = configure_in_progress.clone();
            let start_barrier = start_barrier.clone();
            thread::spawn(move || {
                start_barrier.wait();
                apply_configure_options(
                    configuration.as_ref(),
                    &configure_in_progress,
                    &locators_a,
                    ConfigureOptions {
                        workspace_directories: None,
                        conda_executable: None,
                        pipenv_executable: None,
                        poetry_executable: None,
                        environment_directories: None,
                        cache_directory: None,
                    },
                    Some(vec![PathBuf::from("/workspace/a")]),
                    None,
                )
                .unwrap();
            })
        };
        let worker_b = {
            let configuration = configuration.clone();
            let configure_in_progress = configure_in_progress.clone();
            let start_barrier = start_barrier.clone();
            thread::spawn(move || {
                start_barrier.wait();
                apply_configure_options(
                    configuration.as_ref(),
                    &configure_in_progress,
                    &locators_b,
                    ConfigureOptions {
                        workspace_directories: None,
                        conda_executable: None,
                        pipenv_executable: None,
                        poetry_executable: None,
                        environment_directories: None,
                        cache_directory: None,
                    },
                    Some(vec![PathBuf::from("/workspace/b")]),
                    None,
                )
                .unwrap();
            })
        };

        start_barrier.wait();
        let first_id = entered_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        assert!(configure_in_progress.try_lock().is_err());
        if let Ok(second_id) = entered_rx.recv_timeout(Duration::from_millis(250)) {
            let _ = release_tx.send(());
            let _ = release_tx.send(());
            worker_a.join().unwrap();
            worker_b.join().unwrap();
            panic!(
                "configure calls interleaved: {first_id} and {second_id} were both inside locator.configure()"
            );
        }

        release_tx.send(()).unwrap();
        let second_id = entered_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_ne!(first_id, second_id);
        release_tx.send(()).unwrap();

        worker_a.join().unwrap();
        worker_b.join().unwrap();

        let events = events.lock().unwrap();
        assert_eq!(events.len(), 4, "two configures × enter+exit");
        assert_eq!(
            events.as_slice(),
            &[
                Event::Enter(first_id),
                Event::Exit(first_id),
                Event::Enter(second_id),
                Event::Exit(second_id),
            ]
        );

        // Both publishes occurred.
        let state = configuration.read().unwrap();
        assert_eq!(state.generation, 2);
    }

    /// Test for #461: rollback after a panicking locator must leave the
    /// published config unchanged and release configure serialization.
    #[test]
    fn test_configure_panic_leaves_state_unchanged_and_releases_mutex() {
        // The panic locator panics when `workspace_directories` is set.
        // Place a recording locator first so it is configured, then panics
        // happen on the second locator; rollback must re-configure the
        // first locator with the previous (empty) config.
        let configuration = Arc::new(RwLock::new(ConfigurationState::default()));
        let configure_in_progress = Arc::new(Mutex::new(()));
        let recording = Arc::new(RecordingConfigureLocator {
            configured_workspace_directories: Mutex::new(None),
        });
        let panic_locator = Arc::new(PanicConfigureLocator {
            configured_workspace_directories: Mutex::new(None),
        });
        let locators = Arc::new(vec![
            recording.clone() as Arc<dyn Locator>,
            panic_locator.clone() as Arc<dyn Locator>,
        ]);

        let result = apply_configure_options(
            configuration.as_ref(),
            &configure_in_progress,
            &locators,
            ConfigureOptions {
                workspace_directories: None,
                conda_executable: None,
                pipenv_executable: None,
                poetry_executable: None,
                environment_directories: None,
                cache_directory: None,
            },
            Some(vec![PathBuf::from("/workspace")]),
            None,
        );
        assert!(result.is_err());

        // Generation and config are unchanged.
        let state = configuration.read().unwrap();
        assert_eq!(state.generation, 0);
        assert!(state.config.workspace_directories.is_none());

        // The recording locator was configured forward then rolled back
        // (final observed config has no workspace_directories).
        assert!(recording
            .configured_workspace_directories
            .lock()
            .unwrap()
            .is_none());

        // The configure mutex is released, so a subsequent configure can
        // acquire it without blocking.
        assert!(configure_in_progress.try_lock().is_ok());
    }
}
