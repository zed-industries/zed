use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use docker_client::{ComposeProject, Container, DockerClient, DockerEndpoint, EndpointKind, Image};
use gpui::{Context, EventEmitter, Task};
use settings::{Settings, SettingsStore};

use crate::DockerSettings;

/// The minimum interval between polls of a remote (`Ssh`) endpoint, so the
/// autopoll loop never hammers a remote daemon even if `poll_interval_seconds`
/// is configured very low.
const MIN_REMOTE_POLL_INTERVAL: Duration = Duration::from_secs(15);

/// How often the autopoll loop wakes up to re-check `poll_interval_seconds`
/// while autopolling is disabled (`poll_interval_seconds == 0`). This keeps
/// the loop idle without busy-looping, while still noticing promptly if a
/// settings change re-enables autopolling.
const DISABLED_POLL_RECHECK_INTERVAL: Duration = Duration::from_secs(60);

/// Events emitted by [`DockerEndpointStore`] so the panel can surface changes
/// (e.g. new/updated container lists) to the workspace.
#[derive(Clone, Debug)]
pub enum EndpointStoreEvent {
    Changed,
}

/// A single container/image/compose action that can be dispatched against an
/// endpoint. Carries everything needed both to invoke the [`DockerClient`]
/// method and to render the exact command string in [`crate::ConfirmModal`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DockerAction {
    StartContainer {
        id: String,
    },
    StopContainer {
        id: String,
    },
    RestartContainer {
        id: String,
    },
    PullImage {
        reference: String,
    },
    RemoveImage {
        id: String,
    },
    ComposeUp {
        project: String,
        service: Option<String>,
    },
    ComposeDown {
        project: String,
    },
    ComposeRestart {
        project: String,
        service: Option<String>,
    },
}

impl DockerAction {
    /// Whether this action can discard running state or delete data, and
    /// therefore requires confirmation via [`crate::ConfirmModal`] before it
    /// reaches [`DockerEndpointStore::dispatch_action`] on a writable
    /// endpoint. This is orthogonal to the `read_only` gate in
    /// `dispatch_action`, which blocks EVERY `DockerAction` (destructive or
    /// not) on a read-only endpoint; `is_destructive` only decides whether a
    /// writable endpoint additionally needs a confirmation step first.
    ///
    /// NOTE: `start_container` is intentionally NOT destructive: bringing up
    /// an already-stopped container can't discard running state or data, so
    /// it dispatches without confirmation on a writable endpoint (though it's
    /// still blocked like any other action on a read-only one). `pull_image`
    /// is also non-destructive: it only downloads an image.
    pub fn is_destructive(&self) -> bool {
        match self {
            DockerAction::StartContainer { .. } | DockerAction::PullImage { .. } => false,
            DockerAction::StopContainer { .. }
            | DockerAction::RestartContainer { .. }
            | DockerAction::RemoveImage { .. }
            | DockerAction::ComposeUp { .. }
            | DockerAction::ComposeDown { .. }
            | DockerAction::ComposeRestart { .. } => true,
        }
    }

    /// The exact `docker` command line this action will run, matching
    /// [`docker_client::cli::CliDockerClient`]'s argument construction. Shown
    /// verbatim in [`crate::ConfirmModal`] so the user knows precisely what
    /// will execute before confirming.
    pub fn command_string(&self) -> String {
        match self {
            DockerAction::StartContainer { id } => format!("docker start {id}"),
            DockerAction::StopContainer { id } => format!("docker stop {id}"),
            DockerAction::RestartContainer { id } => format!("docker restart {id}"),
            DockerAction::PullImage { reference } => format!("docker pull {reference}"),
            DockerAction::RemoveImage { id } => format!("docker rmi {id}"),
            DockerAction::ComposeUp { project, service } => match service {
                Some(service) => format!("docker compose -p {project} up -d {service}"),
                None => format!("docker compose -p {project} up -d"),
            },
            DockerAction::ComposeDown { project } => format!("docker compose -p {project} down"),
            DockerAction::ComposeRestart { project, service } => match service {
                Some(service) => format!("docker compose -p {project} restart {service}"),
                None => format!("docker compose -p {project} restart"),
            },
        }
    }
}

/// Constructs a [`DockerClient`] used to talk to every endpoint.
///
/// Production wires this to [`default_client_factory`], which builds a
/// `CliDockerClient`. Tests inject a factory returning a `FakeDockerClient`.
pub type ClientFactory = Arc<dyn Fn() -> Arc<dyn DockerClient> + Send + Sync>;

/// The default production client factory: shells out to the `docker` CLI.
pub fn default_client_factory() -> ClientFactory {
    Arc::new(|| Arc::new(docker_client::CliDockerClient) as Arc<dyn DockerClient>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum EndpointStatus {
    Idle,
    Connecting,
    Connected,
    Error(String),
}

pub struct EndpointState {
    pub endpoint: DockerEndpoint,
    pub status: EndpointStatus,
    /// `None` until `refresh` has successfully listed the containers.
    pub containers: Option<Vec<Container>>,
    /// `None` until `refresh` has successfully listed the images.
    pub images: Option<Vec<Image>>,
    /// `None` until `refresh` has successfully listed the compose projects.
    pub compose: Option<Vec<ComposeProject>>,
    /// Bumped every time the endpoint's state is invalidated (a new refresh
    /// begins or a config edit resets it). An in-flight refresh only writes
    /// its result back if this still matches the value it captured at spawn
    /// time, so a superseded attempt can never clobber newer state.
    generation: u64,
    /// Whether this endpoint was auto-imported from `docker context ls`
    /// rather than configured (or the synthetic "local"). Used by
    /// `sync_from_settings` so a settings resync doesn't discard an imported
    /// endpoint just because it's absent from settings.
    is_imported: bool,
}

impl EndpointState {
    fn new(endpoint: DockerEndpoint) -> Self {
        Self {
            endpoint,
            status: EndpointStatus::Idle,
            containers: None,
            images: None,
            compose: None,
            generation: 0,
            is_imported: false,
        }
    }

    fn new_imported(endpoint: DockerEndpoint) -> Self {
        Self {
            is_imported: true,
            ..Self::new(endpoint)
        }
    }
}

/// Holds the configured Docker endpoints and their lazily-loaded container /
/// image / compose data.
///
/// Every load spawns work through `gpui_tokio` and writes the result back
/// into `self` followed by `cx.notify()` so the panel re-renders. A
/// background poll task periodically calls [`Self::refresh_all`] so the
/// panel stays up to date without user interaction.
pub struct DockerEndpointStore {
    endpoints: Vec<EndpointState>,
    client_factory: ClientFactory,
    /// Tracks the last time each named endpoint was polled, so remote (`Ssh`)
    /// endpoints can be throttled independently of the configured interval.
    last_polled: HashMap<String, Instant>,
    _settings_subscription: gpui::Subscription,
    _poll_task: Task<()>,
}

impl DockerEndpointStore {
    pub fn new(client_factory: ClientFactory, cx: &mut Context<Self>) -> Self {
        let mut endpoints: Vec<EndpointState> = DockerSettings::get_global(cx)
            .endpoints
            .iter()
            .cloned()
            .map(EndpointState::new)
            .collect();
        if !endpoints.iter().any(|state| state.endpoint.name == "local") {
            endpoints.insert(
                0,
                EndpointState::new(DockerEndpoint {
                    name: "local".to_string(),
                    kind: EndpointKind::Local,
                    read_only: false,
                }),
            );
        }
        let settings_subscription = cx.observe_global::<SettingsStore>(Self::sync_from_settings);
        let poll_task = Self::start_poll_task(cx);
        let mut this = Self {
            endpoints,
            client_factory,
            last_polled: HashMap::new(),
            _settings_subscription: settings_subscription,
            _poll_task: poll_task,
        };
        this.discover_contexts(cx);
        this
    }

    /// Best-effort auto-import of `docker context ls` endpoints: spawns
    /// [`DockerClient::list_contexts`] and merges any discovered contexts
    /// into the endpoint list via [`docker_client::merge_endpoints`], which
    /// ensures configured endpoints (and the synthetic "local" one, since
    /// it's included in the "configured" side of the merge) always win by
    /// name over an imported context.
    ///
    /// A failure to list contexts must NOT break the panel: it's logged and
    /// otherwise ignored, leaving the endpoint list exactly as it was
    /// (configured + synthetic local).
    fn discover_contexts(&mut self, cx: &mut Context<Self>) {
        let client = (self.client_factory)();
        let task = gpui_tokio::Tokio::spawn_result(cx, async move { client.list_contexts().await });
        cx.spawn(async move |this, cx| {
            let contexts = match task.await {
                Ok(contexts) => contexts,
                Err(error) => {
                    log::warn!("docker_ui: failed to list docker contexts: {error:#}");
                    return;
                }
            };
            this.update(cx, |this, cx| {
                let configured: Vec<DockerEndpoint> = this
                    .endpoints
                    .iter()
                    .map(|state| state.endpoint.clone())
                    .collect();
                let merged = docker_client::merge_endpoints(configured, contexts);
                let mut changed = false;
                for endpoint in merged {
                    if !this
                        .endpoints
                        .iter()
                        .any(|state| state.endpoint.name == endpoint.name)
                    {
                        this.endpoints.push(EndpointState::new_imported(endpoint));
                        changed = true;
                    }
                }
                if changed {
                    cx.emit(EndpointStoreEvent::Changed);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    pub fn endpoints(&self) -> &[EndpointState] {
        &self.endpoints
    }

    /// Builds a fresh [`DockerClient`] for one-off read operations (e.g.
    /// Inspect/Logs in the detail view) that are always allowed and do not
    /// go through [`Self::dispatch_action`]'s read-only gate.
    pub fn make_client(&self) -> Arc<dyn DockerClient> {
        (self.client_factory)()
    }

    /// The [`ClientFactory`] this store was built with, so callers that need
    /// to construct a throwaway client outside the store (e.g.
    /// `DockerEndpointModal`'s Test Connection button) use the same factory
    /// production wires to `CliDockerClient` and tests wire to a fake.
    pub fn client_factory(&self) -> ClientFactory {
        self.client_factory.clone()
    }

    pub fn endpoint(&self, endpoint_name: &str) -> Option<&DockerEndpoint> {
        self.endpoints
            .iter()
            .find(|state| state.endpoint.name == endpoint_name)
            .map(|state| &state.endpoint)
    }

    /// Reconciles `self.endpoints` with the current settings: adds newly
    /// configured endpoints as `Idle`, drops removed ones, and keeps live
    /// state for endpoints whose config is unchanged. The synthetic "local"
    /// endpoint is retained even if settings has no endpoints configured.
    fn sync_from_settings(&mut self, cx: &mut Context<Self>) {
        let mut configs: Vec<DockerEndpoint> = DockerSettings::get_global(cx).endpoints.clone();
        if !configs.iter().any(|endpoint| endpoint.name == "local") {
            configs.insert(
                0,
                DockerEndpoint {
                    name: "local".to_string(),
                    kind: EndpointKind::Local,
                    read_only: false,
                },
            );
        }

        let mut changed = false;

        // Drop endpoints that are neither configured nor synthetic-local NOR
        // an auto-imported context: those came from a prior settings
        // generation and are no longer wanted. Imported endpoints are kept
        // even though they're absent from `configs`, since they're not
        // sourced from settings in the first place; a configured endpoint of
        // the same name still takes precedence below.
        let before = self.endpoints.len();
        self.endpoints.retain(|state| {
            state.is_imported
                || configs
                    .iter()
                    .any(|config| config.name == state.endpoint.name)
        });
        changed |= self.endpoints.len() != before;

        for config in &configs {
            match self
                .endpoints
                .iter_mut()
                .find(|state| state.endpoint.name == config.name)
            {
                Some(existing) => {
                    if &existing.endpoint != config || existing.is_imported {
                        existing.endpoint = config.clone();
                        existing.status = EndpointStatus::Idle;
                        existing.containers = None;
                        existing.images = None;
                        existing.compose = None;
                        existing.generation = existing.generation.wrapping_add(1);
                        existing.is_imported = false;
                        changed = true;
                    }
                }
                None => {
                    self.endpoints.push(EndpointState::new(config.clone()));
                    changed = true;
                }
            }
        }

        if changed {
            cx.emit(EndpointStoreEvent::Changed);
            cx.notify();
        }

        self.discover_contexts(cx);
    }

    fn endpoint_index(&self, endpoint_name: &str) -> Option<usize> {
        self.endpoints
            .iter()
            .position(|state| state.endpoint.name == endpoint_name)
    }

    fn endpoint_mut(&mut self, endpoint_name: &str) -> Option<&mut EndpointState> {
        self.endpoints
            .iter_mut()
            .find(|state| state.endpoint.name == endpoint_name)
    }

    /// Test-only hook for forcing an endpoint's `read_only` flag (or other
    /// fields) without going through settings, so read-only-gate tests don't
    /// need a full settings round-trip to set up their fixture.
    #[cfg(test)]
    pub(crate) fn endpoint_mut_for_test(
        &mut self,
        endpoint_name: &str,
    ) -> Option<&mut EndpointState> {
        self.endpoint_mut(endpoint_name)
    }

    /// Tests the endpoint and lists its containers, images, and compose
    /// projects, writing the results back as they complete.
    pub fn refresh(&mut self, endpoint_name: &str, cx: &mut Context<Self>) {
        let Some(index) = self.endpoint_index(endpoint_name) else {
            return;
        };
        if self.endpoints[index].status == EndpointStatus::Connecting {
            return;
        }
        self.endpoints[index].generation = self.endpoints[index].generation.wrapping_add(1);
        let generation = self.endpoints[index].generation;
        self.endpoints[index].status = EndpointStatus::Connecting;
        cx.notify();

        let endpoint = self.endpoints[index].endpoint.clone();
        let client = (self.client_factory)();
        let endpoint_name = endpoint_name.to_string();

        let test_client = client.clone();
        let test_endpoint = endpoint.clone();
        let test_task = gpui_tokio::Tokio::spawn_result(cx, async move {
            test_client.test_endpoint(&test_endpoint).await
        });

        cx.spawn(async move |this, cx| {
            let result = test_task.await;
            if let Err(error) = result {
                this.update(cx, |this, cx| {
                    this.set_error_for_generation(
                        &endpoint_name,
                        generation,
                        format!("{error:#}"),
                        cx,
                    );
                })
                .ok();
                return;
            }

            let containers_client = client.clone();
            let containers_endpoint = endpoint.clone();
            let images_client = client.clone();
            let images_endpoint = endpoint.clone();
            let compose_client = client.clone();
            let compose_endpoint = endpoint.clone();

            let containers_task = this
                .update(cx, |_, cx| {
                    gpui_tokio::Tokio::spawn_result(cx, async move {
                        containers_client
                            .list_containers(&containers_endpoint)
                            .await
                    })
                })
                .ok();
            let images_task = this
                .update(cx, |_, cx| {
                    gpui_tokio::Tokio::spawn_result(cx, async move {
                        images_client.list_images(&images_endpoint).await
                    })
                })
                .ok();
            let compose_task = this
                .update(cx, |_, cx| {
                    gpui_tokio::Tokio::spawn_result(cx, async move {
                        compose_client
                            .list_compose_projects(&compose_endpoint)
                            .await
                    })
                })
                .ok();

            let (Some(containers_task), Some(images_task), Some(compose_task)) =
                (containers_task, images_task, compose_task)
            else {
                return;
            };

            let containers_result = containers_task.await;
            let images_result = images_task.await;
            let compose_result = compose_task.await;

            this.update(cx, |this, cx| {
                let Some(state) = this.endpoint_mut(&endpoint_name) else {
                    return;
                };
                if state.generation != generation {
                    return;
                }

                let mut first_error = None;
                match containers_result {
                    Ok(containers) => state.containers = Some(containers),
                    Err(error) => {
                        first_error.get_or_insert(format!("{error:#}"));
                    }
                }
                match images_result {
                    Ok(images) => state.images = Some(images),
                    Err(error) => {
                        first_error.get_or_insert(format!("{error:#}"));
                    }
                }
                match compose_result {
                    Ok(compose) => state.compose = Some(compose),
                    Err(error) => {
                        first_error.get_or_insert(format!("{error:#}"));
                    }
                }

                match first_error {
                    Some(message) => {
                        state.status = EndpointStatus::Error(message);
                    }
                    None => {
                        state.status = EndpointStatus::Connected;
                    }
                }
                cx.emit(EndpointStoreEvent::Changed);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Runs `action` against `endpoint_name` and refreshes that endpoint on
    /// success.
    ///
    /// This is the SAFETY-CRITICAL gate and the single chokepoint every
    /// `DockerAction` passes through (start/stop/restart/remove/pull/compose)
    /// on its way to the `DockerClient`: ANY action against a `read_only`
    /// endpoint returns immediately WITHOUT invoking the `DockerClient` at
    /// all, not just destructive ones. Read paths (logs/inspect) never call
    /// this method in the first place, so they remain unaffected. This check
    /// is re-applied here rather than trusted to the caller (e.g. a disabled
    /// button or a confirm modal that failed to gate correctly), so that even
    /// a caller bug can never reach the client on a read-only endpoint. A
    /// read-only endpoint is therefore truly view-only, including
    /// non-destructive mutations like Start/Pull.
    ///
    /// Callers are expected to have already shown a [`crate::ConfirmModal`]
    /// for destructive actions and only invoke this method once the user
    /// confirmed; this method itself does not prompt. The confirm
    /// requirement stays keyed to `DockerAction::is_destructive` and is
    /// unaffected by this gate: it decides whether a writable endpoint needs
    /// confirmation, not whether the action is allowed at all.
    pub fn dispatch_action(
        &mut self,
        endpoint_name: &str,
        action: DockerAction,
        cx: &mut Context<Self>,
    ) {
        let Some(state) = self
            .endpoints
            .iter()
            .find(|s| s.endpoint.name == endpoint_name)
        else {
            return;
        };
        if state.endpoint.read_only {
            log::warn!("blocked docker action on read-only endpoint {endpoint_name}: {action:?}");
            return;
        }

        let endpoint = state.endpoint.clone();
        let client = (self.client_factory)();
        let endpoint_name = endpoint_name.to_string();

        let run_task = gpui_tokio::Tokio::spawn_result(cx, async move {
            match action {
                DockerAction::StartContainer { id } => client.start_container(&endpoint, &id).await,
                DockerAction::StopContainer { id } => client.stop_container(&endpoint, &id).await,
                DockerAction::RestartContainer { id } => {
                    client.restart_container(&endpoint, &id).await
                }
                DockerAction::PullImage { reference } => {
                    client.pull_image(&endpoint, &reference).await
                }
                DockerAction::RemoveImage { id } => client.remove_image(&endpoint, &id).await,
                DockerAction::ComposeUp { project, service } => {
                    client
                        .compose_up(&endpoint, &project, service.as_deref())
                        .await
                }
                DockerAction::ComposeDown { project } => {
                    client.compose_down(&endpoint, &project).await
                }
                DockerAction::ComposeRestart { project, service } => {
                    client
                        .compose_restart(&endpoint, &project, service.as_deref())
                        .await
                }
            }
        });

        cx.spawn(async move |this, cx| {
            let result = run_task.await;
            match result {
                Ok(()) => {
                    this.update(cx, |this, cx| this.refresh(&endpoint_name, cx))
                        .ok();
                }
                Err(error) => {
                    this.update(cx, |this, cx| {
                        this.set_error(&endpoint_name, format!("{error:#}"), cx);
                    })
                    .ok();
                }
            }
        })
        .detach();
    }

    /// Starts the container with `id` on `endpoint_name`. Not destructive (so
    /// never requires confirmation on a writable endpoint), but still blocked
    /// on read-only endpoints by [`Self::dispatch_action`], since it's a
    /// state-mutating action like any other.
    pub fn start_container(&mut self, endpoint_name: &str, id: String, cx: &mut Context<Self>) {
        self.dispatch_action(endpoint_name, DockerAction::StartContainer { id }, cx);
    }

    /// Stops the container with `id` on `endpoint_name`. Destructive: blocked
    /// on read-only endpoints by [`Self::dispatch_action`].
    pub fn stop_container(&mut self, endpoint_name: &str, id: String, cx: &mut Context<Self>) {
        self.dispatch_action(endpoint_name, DockerAction::StopContainer { id }, cx);
    }

    /// Restarts the container with `id` on `endpoint_name`. Destructive:
    /// blocked on read-only endpoints by [`Self::dispatch_action`].
    pub fn restart_container(&mut self, endpoint_name: &str, id: String, cx: &mut Context<Self>) {
        self.dispatch_action(endpoint_name, DockerAction::RestartContainer { id }, cx);
    }

    /// Pulls `reference` on `endpoint_name`. Not destructive (so never
    /// requires confirmation on a writable endpoint), but still blocked on
    /// read-only endpoints by [`Self::dispatch_action`], since it's a
    /// state-mutating action like any other.
    pub fn pull_image(&mut self, endpoint_name: &str, reference: String, cx: &mut Context<Self>) {
        self.dispatch_action(endpoint_name, DockerAction::PullImage { reference }, cx);
    }

    /// Removes the image with `id` on `endpoint_name`. Destructive: blocked
    /// on read-only endpoints by [`Self::dispatch_action`].
    pub fn remove_image(&mut self, endpoint_name: &str, id: String, cx: &mut Context<Self>) {
        self.dispatch_action(endpoint_name, DockerAction::RemoveImage { id }, cx);
    }

    /// Brings up `project` (optionally scoped to `service`) on
    /// `endpoint_name`. Destructive: blocked on read-only endpoints by
    /// [`Self::dispatch_action`].
    pub fn compose_up(
        &mut self,
        endpoint_name: &str,
        project: String,
        service: Option<String>,
        cx: &mut Context<Self>,
    ) {
        self.dispatch_action(
            endpoint_name,
            DockerAction::ComposeUp { project, service },
            cx,
        );
    }

    /// Tears down `project` on `endpoint_name`. Destructive: blocked on
    /// read-only endpoints by [`Self::dispatch_action`].
    pub fn compose_down(&mut self, endpoint_name: &str, project: String, cx: &mut Context<Self>) {
        self.dispatch_action(endpoint_name, DockerAction::ComposeDown { project }, cx);
    }

    /// Restarts `project` (optionally scoped to `service`) on
    /// `endpoint_name`. Destructive: blocked on read-only endpoints by
    /// [`Self::dispatch_action`].
    pub fn compose_restart(
        &mut self,
        endpoint_name: &str,
        project: String,
        service: Option<String>,
        cx: &mut Context<Self>,
    ) {
        self.dispatch_action(
            endpoint_name,
            DockerAction::ComposeRestart { project, service },
            cx,
        );
    }

    /// Refreshes every endpoint that is not currently `Error` or `Connecting`.
    pub fn refresh_all(&mut self, cx: &mut Context<Self>) {
        let names: Vec<String> = self
            .endpoints
            .iter()
            .filter(|state| {
                !matches!(
                    state.status,
                    EndpointStatus::Error(_) | EndpointStatus::Connecting
                )
            })
            .map(|state| state.endpoint.name.clone())
            .collect();
        for name in names {
            self.refresh(&name, cx);
        }
    }

    fn set_error(&mut self, endpoint_name: &str, message: String, cx: &mut Context<Self>) {
        if let Some(state) = self.endpoint_mut(endpoint_name) {
            state.status = EndpointStatus::Error(message);
        }
        cx.emit(EndpointStoreEvent::Changed);
        cx.notify();
    }

    /// Sets the error status only if the endpoint's generation still matches
    /// `generation`, so a stale refresh attempt cannot flip an endpoint that a
    /// newer attempt has already reconfigured or connected.
    fn set_error_for_generation(
        &mut self,
        endpoint_name: &str,
        generation: u64,
        message: String,
        cx: &mut Context<Self>,
    ) {
        if let Some(state) = self.endpoint_mut(endpoint_name) {
            if state.generation != generation {
                return;
            }
        } else {
            return;
        }
        self.set_error(endpoint_name, message, cx);
    }

    /// Starts the recurring autopoll loop. The interval is read fresh from
    /// settings on every tick so a settings change takes effect on the next
    /// sleep without restarting the app. Remote (`Ssh`) endpoints are polled
    /// at most every `max(interval, MIN_REMOTE_POLL_INTERVAL)`: endpoints not
    /// yet due are skipped for that tick via `poll_all_due`, rather than
    /// blocking the whole loop's cadence.
    ///
    /// `poll_interval_seconds == 0` means manual-only: the loop sleeps
    /// `DISABLED_POLL_RECHECK_INTERVAL` and re-checks the setting without
    /// refreshing anything, so it neither busy-loops nor needs restarting
    /// once the user picks a non-zero interval again.
    fn start_poll_task(cx: &mut Context<Self>) -> Task<()> {
        cx.spawn(async move |this, cx| {
            loop {
                let interval_seconds = match this.read_with(cx, |_, cx| {
                    DockerSettings::get_global(cx).poll_interval_seconds
                }) {
                    Ok(interval_seconds) => interval_seconds,
                    Err(_) => return,
                };
                if interval_seconds == 0 {
                    cx.background_executor()
                        .timer(DISABLED_POLL_RECHECK_INTERVAL)
                        .await;
                    continue;
                }
                let interval = Duration::from_secs(interval_seconds);
                cx.background_executor().timer(interval).await;
                let now = cx.background_executor().now();
                let poll_result = this.update(cx, |this, cx| {
                    this.poll_all_due(interval, now, cx);
                });
                if poll_result.is_err() {
                    return;
                }
            }
        })
    }

    /// Refreshes every non-`Error`, non-`Connecting` endpoint that is due:
    /// local endpoints every tick, remote (`Ssh`) endpoints at most every
    /// `max(interval, MIN_REMOTE_POLL_INTERVAL)`.
    fn poll_all_due(&mut self, interval: Duration, now: Instant, cx: &mut Context<Self>) {
        let remote_interval = interval.max(MIN_REMOTE_POLL_INTERVAL);
        let names: Vec<String> = self
            .endpoints
            .iter()
            .filter(|state| {
                !matches!(
                    state.status,
                    EndpointStatus::Error(_) | EndpointStatus::Connecting
                )
            })
            .filter(|state| {
                let is_remote = matches!(state.endpoint.kind, EndpointKind::Ssh { .. });
                if !is_remote {
                    return true;
                }
                match self.last_polled.get(&state.endpoint.name) {
                    Some(last) => now.duration_since(*last) >= remote_interval,
                    None => true,
                }
            })
            .map(|state| state.endpoint.name.clone())
            .collect();

        for name in &names {
            self.last_polled.insert(name.clone(), now);
        }
        for name in names {
            self.refresh(&name, cx);
        }
    }
}

impl EventEmitter<EndpointStoreEvent> for DockerEndpointStore {}

#[cfg(test)]
mod tests {
    use super::*;
    use docker_client::fake::FakeDockerClient;
    use gpui::{AppContext as _, BorrowAppContext as _, TestAppContext};
    use settings::SettingsStore;

    #[test]
    fn is_destructive_covers_exactly_stop_restart_remove_and_compose() {
        assert!(!DockerAction::StartContainer { id: "a".into() }.is_destructive());
        assert!(
            !DockerAction::PullImage {
                reference: "a".into()
            }
            .is_destructive()
        );
        assert!(DockerAction::StopContainer { id: "a".into() }.is_destructive());
        assert!(DockerAction::RestartContainer { id: "a".into() }.is_destructive());
        assert!(DockerAction::RemoveImage { id: "a".into() }.is_destructive());
        assert!(
            DockerAction::ComposeUp {
                project: "a".into(),
                service: None
            }
            .is_destructive()
        );
        assert!(
            DockerAction::ComposeDown {
                project: "a".into()
            }
            .is_destructive()
        );
        assert!(
            DockerAction::ComposeRestart {
                project: "a".into(),
                service: None
            }
            .is_destructive()
        );
    }

    #[test]
    fn command_string_matches_cli_docker_client_args() {
        assert_eq!(
            DockerAction::RestartContainer { id: "api".into() }.command_string(),
            "docker restart api"
        );
        assert_eq!(
            DockerAction::ComposeDown {
                project: "shop".into()
            }
            .command_string(),
            "docker compose -p shop down"
        );
        assert_eq!(
            DockerAction::ComposeUp {
                project: "shop".into(),
                service: Some("web".into())
            }
            .command_string(),
            "docker compose -p shop up -d web"
        );
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            gpui_tokio::init(cx);
            crate::init(cx);
        });
    }

    fn set_one_ssh_endpoint(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.docker.get_or_insert_default().connections =
                        Some(vec![settings::DockerConnectionContent {
                            name: "prod".into(),
                            kind: settings::DockerEndpointKindContent::Ssh,
                            ssh_host: Some("deploy@1.2.3.4".into()),
                            read_only: Some(true),
                        }]);
                });
            });
        });
    }

    fn set_poll_interval_seconds(cx: &mut TestAppContext, poll_interval_seconds: u64) {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .docker
                        .get_or_insert_default()
                        .poll_interval_seconds = Some(poll_interval_seconds);
                });
            });
        });
    }

    /// Drives the deterministic scheduler while giving the real tokio runtime a
    /// chance to complete cross-thread work, until `condition` holds or a bound
    /// is reached. Requires `cx.executor().allow_parking()`.
    async fn wait_until(cx: &mut TestAppContext, condition: impl Fn(&mut TestAppContext) -> bool) {
        for _ in 0..200 {
            cx.run_until_parked();
            if condition(cx) {
                return;
            }
            cx.executor()
                .timer(std::time::Duration::from_millis(5))
                .await;
        }
        cx.run_until_parked();
        assert!(
            condition(cx),
            "condition did not become true within the time bound"
        );
    }

    #[gpui::test]
    async fn refresh_populates_containers_from_client(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        set_one_ssh_endpoint(cx);
        let fake = Arc::new(FakeDockerClient::new_with_container("api"));
        let factory: ClientFactory = Arc::new(move || fake.clone() as Arc<dyn DockerClient>);
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        store.update(cx, |s, cx| s.refresh("prod", cx));
        wait_until(cx, |cx| {
            store.read_with(cx, |s, _| {
                s.endpoints()
                    .iter()
                    .find(|e| e.endpoint.name == "prod")
                    .and_then(|e| e.containers.as_ref())
                    .map_or(false, |c| c.len() == 1)
            })
        })
        .await;
    }

    #[gpui::test]
    fn new_store_prepends_local_endpoint(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let factory: ClientFactory =
            Arc::new(|| Arc::new(FakeDockerClient::new()) as Arc<dyn DockerClient>);
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        store.read_with(cx, |s, _| {
            assert!(s.endpoints().iter().any(|e| e.endpoint.name == "local"))
        });
    }

    #[gpui::test]
    async fn new_store_imports_discovered_context_as_endpoint(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDockerClient::new();
        fake.contexts = vec![docker_client::DockerContext {
            name: "staging".into(),
            docker_endpoint: "ssh://deploy@stg".into(),
        }];
        let fake = Arc::new(fake);
        let factory: ClientFactory = Arc::new(move || fake.clone() as Arc<dyn DockerClient>);
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));

        wait_until(cx, |cx| {
            store.read_with(cx, |s, _| {
                s.endpoints().iter().any(|e| e.endpoint.name == "staging")
            })
        })
        .await;

        store.read_with(cx, |s, _| {
            let staging = s
                .endpoints()
                .iter()
                .find(|e| e.endpoint.name == "staging")
                .expect("staging endpoint should have been imported");
            assert!(matches!(
                staging.endpoint.kind,
                EndpointKind::Ssh { ref host } if host == "deploy@stg"
            ));
            assert!(
                staging.endpoint.read_only,
                "auto-imported contexts must default to read_only: true"
            );
        });
    }

    #[gpui::test]
    async fn new_store_prefers_configured_endpoint_over_clashing_context(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        set_one_ssh_endpoint(cx); // configures "prod" as read_only: true, ssh deploy@1.2.3.4
        let mut fake = FakeDockerClient::new();
        fake.contexts = vec![docker_client::DockerContext {
            name: "prod".into(),
            docker_endpoint: "ssh://other@h2".into(),
        }];
        let fake = Arc::new(fake);
        let factory: ClientFactory = Arc::new(move || fake.clone() as Arc<dyn DockerClient>);
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));

        // Give the (best-effort) context-discovery task a chance to run and
        // merge before asserting nothing changed for "prod".
        cx.executor()
            .timer(std::time::Duration::from_millis(50))
            .await;
        cx.run_until_parked();

        store.read_with(cx, |s, _| {
            let prod_count = s
                .endpoints()
                .iter()
                .filter(|e| e.endpoint.name == "prod")
                .count();
            assert_eq!(prod_count, 1, "clashing context must not duplicate prod");
            let prod = s
                .endpoints()
                .iter()
                .find(|e| e.endpoint.name == "prod")
                .expect("prod endpoint should exist");
            assert!(
                prod.endpoint.read_only,
                "configured prod endpoint must win over the clashing context"
            );
            assert!(matches!(
                prod.endpoint.kind,
                EndpointKind::Ssh { ref host } if host == "deploy@1.2.3.4"
            ));
        });
    }

    #[gpui::test]
    async fn new_store_falls_back_when_list_contexts_fails(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDockerClient::with_error("boom"));
        let factory: ClientFactory = Arc::new(move || fake.clone() as Arc<dyn DockerClient>);
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));

        cx.executor()
            .timer(std::time::Duration::from_millis(50))
            .await;
        cx.run_until_parked();

        store.read_with(cx, |s, _| {
            assert!(s.endpoints().iter().any(|e| e.endpoint.name == "local"));
            assert_eq!(
                s.endpoints().len(),
                1,
                "a failed context listing must fall back to just the synthetic local endpoint"
            );
        });
    }

    #[gpui::test]
    async fn autopoll_refreshes_after_interval(cx: &mut TestAppContext) {
        init_test(cx);
        // Explicitly set a small interval rather than relying on the
        // (now 300s) default, so this test stays fast regardless of what the
        // default is configured to.
        set_poll_interval_seconds(cx, 5);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDockerClient::new()); // records calls
        let recorder = fake.clone();
        let factory: ClientFactory = Arc::new(move || recorder.clone() as Arc<dyn DockerClient>);
        let _store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        let before = fake
            .calls()
            .iter()
            .filter(|c| c.starts_with("list_containers"))
            .count();
        cx.executor()
            .advance_clock(std::time::Duration::from_secs(6));
        // The autopoll tick fires immediately once the clock advances past the
        // interval, but the fake client's response arrives via a real tokio
        // thread, so `wait_until` (rather than a single `run_until_parked`) is
        // needed to observe it.
        wait_until(cx, |_| {
            fake.calls()
                .iter()
                .filter(|c| c.starts_with("list_containers"))
                .count()
                > before
        })
        .await;
        let after = fake
            .calls()
            .iter()
            .filter(|c| c.starts_with("list_containers"))
            .count();
        assert!(
            after > before,
            "autopoll should have refreshed at least once"
        );
    }

    #[gpui::test]
    async fn autopoll_disabled_when_interval_zero(cx: &mut TestAppContext) {
        init_test(cx);
        // 0 = manual only: the autopoll loop must never call `refresh_all`
        // while this is set, no matter how long we advance the clock.
        set_poll_interval_seconds(cx, 0);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDockerClient::new()); // records calls
        let recorder = fake.clone();
        let factory: ClientFactory = Arc::new(move || recorder.clone() as Arc<dyn DockerClient>);
        let _store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        cx.run_until_parked();
        let before = fake
            .calls()
            .iter()
            .filter(|c| c.starts_with("list_containers"))
            .count();

        cx.executor()
            .advance_clock(std::time::Duration::from_secs(600));
        cx.run_until_parked();

        let after = fake
            .calls()
            .iter()
            .filter(|c| c.starts_with("list_containers"))
            .count();
        assert_eq!(
            after, before,
            "autopoll must not refresh while poll_interval_seconds is 0"
        );
    }

    /// SAFETY-CRITICAL: this is the lowest-level proof of the read-only
    /// gate. EVERY action — destructive or not — must bail out of
    /// `dispatch_action` before invoking the `DockerClient`, for a
    /// `read_only` endpoint. This includes non-destructive actions like
    /// `start`/`pull`: a read-only endpoint is fully view-only, not just
    /// protected from destructive mutations.
    #[gpui::test]
    async fn dispatch_action_blocks_all_actions_on_read_only_endpoint(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        set_one_ssh_endpoint(cx); // "prod" is read_only: true
        let fake = Arc::new(FakeDockerClient::new_with_container("api"));
        let factory: ClientFactory = {
            let fake = fake.clone();
            Arc::new(move || fake.clone() as Arc<dyn DockerClient>)
        };
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));

        let all_actions = [
            DockerAction::StartContainer { id: "api".into() },
            DockerAction::PullImage {
                reference: "img:latest".into(),
            },
            DockerAction::StopContainer { id: "api".into() },
            DockerAction::RestartContainer { id: "api".into() },
            DockerAction::RemoveImage { id: "img".into() },
            DockerAction::ComposeUp {
                project: "shop".into(),
                service: None,
            },
            DockerAction::ComposeDown {
                project: "shop".into(),
            },
            DockerAction::ComposeRestart {
                project: "shop".into(),
                service: None,
            },
        ];
        for action in all_actions {
            store.update(cx, |s, cx| s.dispatch_action("prod", action, cx));
        }
        cx.run_until_parked();

        // `list_contexts` is expected: it's the best-effort context-discovery
        // call made once on construction, unrelated to the read-only gate
        // under test here.
        let non_discovery_calls: Vec<_> = fake
            .calls()
            .into_iter()
            .filter(|call| call != "list_contexts")
            .collect();
        assert!(
            non_discovery_calls.is_empty(),
            "no action, destructive or not, should reach the client on a read-only endpoint; calls: {:?}",
            non_discovery_calls
        );
    }

    /// Non-destructive actions (start/pull) are allowed on a WRITABLE
    /// endpoint without going through a confirmation step (unlike destructive
    /// actions, which require `ConfirmModal` at the `DockerPanel` layer
    /// before ever calling `dispatch_action`).
    #[gpui::test]
    async fn dispatch_action_allows_non_destructive_actions_on_writable_endpoint(
        cx: &mut TestAppContext,
    ) {
        init_test(cx); // "local" endpoint defaults to read_only: false
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDockerClient::new_with_container("api"));
        let factory: ClientFactory = {
            let fake = fake.clone();
            Arc::new(move || fake.clone() as Arc<dyn DockerClient>)
        };
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));

        store.update(cx, |s, cx| {
            s.dispatch_action(
                "local",
                DockerAction::StartContainer { id: "api".into() },
                cx,
            )
        });
        wait_until(cx, |_| {
            fake.calls()
                .iter()
                .any(|c| c.starts_with("start_container"))
        })
        .await;

        assert!(
            fake.calls()
                .iter()
                .any(|c| c.starts_with("start_container")),
            "non-destructive actions must be allowed on a writable endpoint"
        );
    }

    #[gpui::test]
    async fn dispatch_action_runs_destructive_actions_on_writable_endpoint(
        cx: &mut TestAppContext,
    ) {
        init_test(cx); // "local" endpoint defaults to read_only: false
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDockerClient::new_with_container("api"));
        let factory: ClientFactory = {
            let fake = fake.clone();
            Arc::new(move || fake.clone() as Arc<dyn DockerClient>)
        };
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));

        store.update(cx, |s, cx| {
            s.dispatch_action(
                "local",
                DockerAction::RestartContainer { id: "api".into() },
                cx,
            )
        });
        wait_until(cx, |_| {
            fake.calls()
                .iter()
                .any(|c| c.starts_with("restart_container"))
        })
        .await;

        assert!(
            fake.calls()
                .iter()
                .any(|c| c.starts_with("restart_container local api")),
            "restart should run with the expected endpoint/id; calls: {:?}",
            fake.calls()
        );
    }
}
