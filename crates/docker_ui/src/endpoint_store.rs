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

/// Events emitted by [`DockerEndpointStore`] so the panel can surface changes
/// (e.g. new/updated container lists) to the workspace.
#[derive(Clone, Debug)]
pub enum EndpointStoreEvent {
    Changed,
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
        Self {
            endpoints,
            client_factory,
            last_polled: HashMap::new(),
            _settings_subscription: settings_subscription,
            _poll_task: poll_task,
        }
    }

    pub fn endpoints(&self) -> &[EndpointState] {
        &self.endpoints
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

        let before = self.endpoints.len();
        self.endpoints.retain(|state| {
            configs
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
                    if &existing.endpoint != config {
                        existing.endpoint = config.clone();
                        existing.status = EndpointStatus::Idle;
                        existing.containers = None;
                        existing.images = None;
                        existing.compose = None;
                        existing.generation = existing.generation.wrapping_add(1);
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
    fn start_poll_task(cx: &mut Context<Self>) -> Task<()> {
        cx.spawn(async move |this, cx| {
            loop {
                let interval = this
                    .read_with(cx, |_, cx| {
                        Duration::from_secs(DockerSettings::get_global(cx).poll_interval_seconds)
                    })
                    .unwrap_or(Duration::from_secs(5));
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
        let factory: ClientFactory =
            Arc::new(|| Arc::new(FakeDockerClient::new()) as Arc<dyn DockerClient>);
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        store.read_with(cx, |s, _| {
            assert!(s.endpoints().iter().any(|e| e.endpoint.name == "local"))
        });
    }

    #[gpui::test]
    async fn autopoll_refreshes_after_interval(cx: &mut TestAppContext) {
        init_test(cx); // default.json poll_interval_seconds = 5
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
}
