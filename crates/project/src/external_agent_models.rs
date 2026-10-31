use std::rc::Rc;

use collections::{HashMap, IndexMap};
use gpui::{
    App, AppContext as _, Context, Entity, EntityId, Global, SharedString, Subscription, Task,
    WeakEntity,
};
use settings::Settings as _;

use crate::agent_server_store::{
    AgentServerStore, AllAgentServersSettings, CustomAgentServerSettings,
};
use crate::{AgentId, AgentRegistryStore, Project};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalAgentModel {
    pub id: SharedString,
    pub name: SharedString,
    pub group: Option<SharedString>,
}

#[derive(Debug)]
pub enum ExternalAgentModelsState {
    Loading,
    Ready(Vec<ExternalAgentModel>),
    AuthRequired,
    Failed(SharedString),
}

/// The settings and registry data an agent had when its catalog entry was
/// probed. An `AgentServersUpdated` event only invalidates an agent whose
/// fingerprint differs from what the store reports afterwards.
#[derive(Debug, Clone, PartialEq)]
struct AgentFingerprint {
    settings: Option<CustomAgentServerSettings>,
    registry_version: Option<SharedString>,
}

fn agent_fingerprint(agent_id: &AgentId, cx: &App) -> AgentFingerprint {
    AgentFingerprint {
        settings: AllAgentServersSettings::get_global(cx)
            .get(agent_id.0.as_ref())
            .cloned(),
        registry_version: AgentRegistryStore::try_global(cx).and_then(|store| {
            store
                .read(cx)
                .agent(agent_id)
                .map(|agent| agent.version().clone())
        }),
    }
}

struct StoreWatch {
    project: WeakEntity<Project>,
    _updated: Subscription,
    _released: Subscription,
}

/// App-wide catalog of the models exposed by installed external agents, keyed
/// by `AgentId`. A single catalog serves every project and window; it
/// subscribes to each local project's `AgentServerStore` for
/// `AgentServersUpdated` invalidation.
pub struct ExternalAgentModelCatalog {
    states: IndexMap<AgentId, ExternalAgentModelsState>,
    tasks: HashMap<AgentId, Task<()>>,
    fingerprints: HashMap<AgentId, AgentFingerprint>,
    store_watches: HashMap<EntityId, StoreWatch>,
    refresh_on_agent_changes: bool,
}

impl ExternalAgentModelCatalog {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            states: IndexMap::default(),
            tasks: HashMap::default(),
            fingerprints: HashMap::default(),
            store_watches: HashMap::default(),
            refresh_on_agent_changes: false,
        }
    }

    pub fn state(&self, agent_id: &AgentId) -> Option<&ExternalAgentModelsState> {
        self.states.get(agent_id)
    }

    pub fn set_state(
        &mut self,
        agent_id: AgentId,
        state: ExternalAgentModelsState,
        cx: &mut Context<Self>,
    ) {
        if matches!(state, ExternalAgentModelsState::Loading)
            || !self.fingerprints.contains_key(&agent_id)
        {
            self.fingerprints
                .insert(agent_id.clone(), agent_fingerprint(&agent_id, cx));
        }
        self.states.insert(agent_id, state);
        cx.notify();
    }

    pub fn set_task(&mut self, agent_id: AgentId, task: Task<()>) {
        self.tasks.insert(agent_id, task);
    }

    /// When enabled, `AgentServersUpdated` events that actually change the set
    /// of tracked agents re-invoke the registered source. Probing stays off by
    /// default so opening UI cannot start agent processes.
    pub fn enable_refresh_on_agent_changes(&mut self) {
        self.refresh_on_agent_changes = true;
    }

    /// Subscribes the catalog to this project's agent-server store. Remote and
    /// collab projects are never subscribed; a project is subscribed at most
    /// once, and the subscription is dropped when the project is released.
    pub fn ensure_store_subscription(&mut self, project: &Entity<Project>, cx: &mut Context<Self>) {
        if !project.read(cx).is_local() {
            return;
        }
        let store = project.read(cx).agent_server_store().clone();
        let store_id = store.entity_id();
        if self.store_watches.contains_key(&store_id) {
            return;
        }

        let _updated = cx.subscribe(&store, |catalog, store, _event, cx| {
            catalog.handle_agent_servers_updated(store, cx)
        });
        let _released = cx.observe_release(project, move |catalog, _project, _cx| {
            catalog.store_watches.remove(&store_id);
        });
        self.store_watches.insert(
            store_id,
            StoreWatch {
                project: project.downgrade(),
                _updated,
                _released,
            },
        );
    }

    fn handle_agent_servers_updated(
        &mut self,
        store: Entity<AgentServerStore>,
        cx: &mut Context<Self>,
    ) {
        let installed: HashMap<AgentId, AgentFingerprint> = store
            .read(cx)
            .external_agents()
            .map(|agent_id| (agent_id.clone(), agent_fingerprint(agent_id, cx)))
            .collect();

        let mut changed = false;
        self.states.retain(|agent_id, _| {
            let keep = installed.get(agent_id) == self.fingerprints.get(agent_id);
            changed |= !keep;
            keep
        });
        if !changed {
            return;
        }
        self.tasks
            .retain(|agent_id, _| self.states.contains_key(agent_id));
        self.fingerprints
            .retain(|agent_id, _| self.states.contains_key(agent_id));
        cx.notify();

        if self.refresh_on_agent_changes {
            let project = self
                .store_watches
                .get(&store.entity_id())
                .and_then(|watch| watch.project.upgrade());
            if let (Some(source), Some(project)) = (model_source(cx), project) {
                let catalog = cx.entity();
                cx.defer(move |cx| source(project, catalog, cx));
            }
        }
    }
}

type ExternalAgentModelSource =
    Rc<dyn Fn(Entity<Project>, Entity<ExternalAgentModelCatalog>, &mut App)>;

#[derive(Default)]
struct GlobalExternalAgentModelCatalogs {
    catalog: Option<Entity<ExternalAgentModelCatalog>>,
    source: Option<ExternalAgentModelSource>,
}

impl Global for GlobalExternalAgentModelCatalogs {}

fn model_source(cx: &App) -> Option<ExternalAgentModelSource> {
    cx.try_global::<GlobalExternalAgentModelCatalogs>()
        .and_then(|catalogs| catalogs.source.clone())
}

/// Registers the source responsible for populating the catalog with the
/// models exposed by installed external agents. Called once by the crate that
/// can create agent sessions (currently `agent_ui`).
pub fn set_external_agent_model_source(
    source: impl Fn(Entity<Project>, Entity<ExternalAgentModelCatalog>, &mut App) + 'static,
    cx: &mut App,
) {
    cx.default_global::<GlobalExternalAgentModelCatalogs>()
        .source = Some(Rc::new(source));
}

/// Returns the app-wide catalog if one has been created. Never invokes the
/// source.
pub fn external_agent_model_catalog(cx: &App) -> Option<Entity<ExternalAgentModelCatalog>> {
    cx.try_global::<GlobalExternalAgentModelCatalogs>()?
        .catalog
        .clone()
}

/// Returns the app-wide catalog, creating it on first use, and subscribes it
/// to this project's agent-server store when the project is local. Never
/// invokes the source.
pub fn subscribe_external_agent_models(
    project: &Entity<Project>,
    cx: &mut App,
) -> Entity<ExternalAgentModelCatalog> {
    let existing = cx
        .default_global::<GlobalExternalAgentModelCatalogs>()
        .catalog
        .clone();
    let catalog = match existing {
        Some(catalog) => catalog,
        None => {
            let catalog = cx.new(|cx| ExternalAgentModelCatalog::new(cx));
            cx.global_mut::<GlobalExternalAgentModelCatalogs>().catalog = Some(catalog.clone());
            catalog
        }
    };
    catalog.update(cx, |catalog, cx| {
        catalog.ensure_store_subscription(project, cx)
    });
    catalog
}

/// Returns the app-wide catalog, creating it on first use, and asks the
/// registered source to refresh it. Returns `None` when no source has been
/// registered.
pub fn refresh_external_agent_models(
    project: &Entity<Project>,
    cx: &mut App,
) -> Option<Entity<ExternalAgentModelCatalog>> {
    let source = model_source(cx)?;
    let catalog = subscribe_external_agent_models(project, cx);
    source(project.clone(), catalog.clone(), cx);
    Some(catalog)
}

#[cfg(test)]
mod tests {
    use std::{
        path::PathBuf,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
    };

    use fs::FakeFs;
    use gpui::{TestAppContext, UpdateGlobal as _};
    use settings::SettingsStore;

    use crate::AgentServersUpdated;
    use crate::agent_registry_store::{RegistryAgent, RegistryAgentMetadata, RegistryNpxAgent};

    use super::*;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    fn install_custom_agent(id: &str, cx: &mut TestAppContext) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |content| {
                    content.agent_servers.get_or_insert_default().insert(
                        id.to_string(),
                        settings::CustomAgentServerSettings::Custom {
                            path: PathBuf::from("/usr/bin/fake-agent"),
                            args: Vec::new(),
                            env: Default::default(),
                            default_mode: None,
                            default_config_options: Default::default(),
                            favorite_config_option_values: Default::default(),
                        },
                    );
                });
            });
        });
    }

    fn install_registry_agent(id: &str, cx: &mut TestAppContext) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |content| {
                    content.agent_servers.get_or_insert_default().insert(
                        id.to_string(),
                        settings::CustomAgentServerSettings::Registry {
                            env: Default::default(),
                            default_mode: None,
                            default_config_options: Default::default(),
                            favorite_config_option_values: Default::default(),
                        },
                    );
                });
            });
        });
    }

    fn uninstall_agent(id: &str, cx: &mut TestAppContext) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |content| {
                    content.agent_servers.get_or_insert_default().0.remove(id);
                });
            });
        });
    }

    fn registry_agent(id: &str, version: &str) -> RegistryAgent {
        RegistryAgent::Npx(RegistryNpxAgent {
            metadata: RegistryAgentMetadata {
                id: AgentId::new(id),
                name: id.into(),
                description: "".into(),
                version: version.into(),
                repository: None,
                website: None,
                license_url: None,
                icon_path: None,
            },
            package: id.into(),
            args: Vec::new(),
            env: Default::default(),
        })
    }

    fn emit_agent_servers_updated(project: &Entity<Project>, cx: &mut TestAppContext) {
        let agent_server_store =
            project.read_with(cx, |project, _cx| project.agent_server_store().clone());
        agent_server_store.update(cx, |_store, cx| cx.emit(AgentServersUpdated));
        cx.run_until_parked();
    }

    fn state_is(
        catalog: &Entity<ExternalAgentModelCatalog>,
        agent_id: &str,
        cx: &mut TestAppContext,
    ) -> Option<String> {
        catalog.read_with(cx, |catalog, _cx| {
            catalog
                .state(&AgentId::new(agent_id))
                .map(|state| match state {
                    ExternalAgentModelsState::Loading => "loading".to_string(),
                    ExternalAgentModelsState::Ready(_) => "ready".to_string(),
                    ExternalAgentModelsState::AuthRequired => "auth_required".to_string(),
                    ExternalAgentModelsState::Failed(_) => "failed".to_string(),
                })
        })
    }

    #[gpui::test]
    async fn no_source_returns_none(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        cx.update(|cx| {
            assert!(refresh_external_agent_models(&project, cx).is_none());
            assert!(external_agent_model_catalog(cx).is_none());
        });
    }

    #[gpui::test]
    async fn source_is_called_once_per_refresh_and_access_does_not_invoke_it(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let call_count = Arc::new(AtomicUsize::new(0));
        let notifications = Arc::new(AtomicUsize::new(0));

        cx.update(|cx| {
            set_external_agent_model_source(
                {
                    let call_count = call_count.clone();
                    move |_project, catalog, cx| {
                        call_count.fetch_add(1, Ordering::SeqCst);
                        catalog.update(cx, |catalog, cx| {
                            catalog.set_state(
                                AgentId::new("agent-one"),
                                ExternalAgentModelsState::Loading,
                                cx,
                            );
                        });
                    }
                },
                cx,
            );
        });

        let catalog = cx
            .update(|cx| refresh_external_agent_models(&project, cx))
            .expect("catalog should be returned once a source is registered");
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        let subscription = cx.update(|cx| {
            cx.observe(&catalog, {
                let notifications = notifications.clone();
                move |_catalog, _cx| {
                    notifications.fetch_add(1, Ordering::SeqCst);
                }
            })
        });

        let accessed = cx
            .update(|cx| external_agent_model_catalog(cx))
            .expect("catalog lookup should reuse the stored catalog");
        assert_eq!(accessed, catalog);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
        assert!(catalog.read_with(cx, |catalog, _cx| {
            matches!(
                catalog.state(&AgentId::new("agent-one")),
                Some(ExternalAgentModelsState::Loading)
            )
        }));

        cx.update(|cx| {
            catalog.update(cx, |catalog, cx| {
                catalog.set_state(
                    AgentId::new("agent-one"),
                    ExternalAgentModelsState::Ready(vec![]),
                    cx,
                );
            });
        });
        cx.update(|cx| {
            catalog.update(cx, |catalog, cx| {
                catalog.set_state(
                    AgentId::new("agent-two"),
                    ExternalAgentModelsState::Failed("boom".into()),
                    cx,
                );
            });
        });
        assert_eq!(notifications.load(Ordering::SeqCst), 2);

        cx.update(|cx| {
            refresh_external_agent_models(&project, cx);
        });
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
        assert_eq!(notifications.load(Ordering::SeqCst), 3);
        drop(subscription);
    }

    #[gpui::test]
    async fn catalog_is_app_wide_and_shared_across_projects(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project_one = Project::test(fs.clone(), [], cx).await;
        let project_two = Project::test(fs, [], cx).await;
        let call_count = Arc::new(AtomicUsize::new(0));

        cx.update(|cx| {
            set_external_agent_model_source(
                {
                    let call_count = call_count.clone();
                    move |_project, catalog, cx| {
                        call_count.fetch_add(1, Ordering::SeqCst);
                        catalog.update(cx, |catalog, cx| {
                            catalog.set_state(
                                AgentId::new("agent-one"),
                                ExternalAgentModelsState::Ready(vec![]),
                                cx,
                            );
                        });
                    }
                },
                cx,
            );
        });

        let catalog = cx
            .update(|cx| refresh_external_agent_models(&project_one, cx))
            .expect("catalog");
        let same_catalog = cx
            .update(|cx| refresh_external_agent_models(&project_two, cx))
            .expect("catalog");
        assert_eq!(catalog, same_catalog);
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
        assert_eq!(
            state_is(&catalog, "agent-one", cx).as_deref(),
            Some("ready")
        );

        // The catalog is subscribed to both local projects' stores; an event
        // from either one reaches it.
        assert_eq!(
            catalog.read_with(cx, |catalog, _cx| catalog.store_watches.len()),
            2
        );
        emit_agent_servers_updated(&project_two, cx);
        assert_eq!(state_is(&catalog, "agent-one", cx), None);
    }

    #[gpui::test]
    async fn agent_servers_updated_preserves_unchanged_states_and_loading(cx: &mut TestAppContext) {
        init_test(cx);
        install_custom_agent("agent-one", cx);
        install_custom_agent("agent-two", cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        cx.run_until_parked();

        cx.update(|cx| {
            set_external_agent_model_source(
                |_project, catalog, cx| {
                    catalog.update(cx, |catalog, cx| {
                        catalog.set_state(
                            AgentId::new("agent-one"),
                            ExternalAgentModelsState::Ready(vec![]),
                            cx,
                        );
                        catalog.set_state(
                            AgentId::new("agent-two"),
                            ExternalAgentModelsState::Loading,
                            cx,
                        );
                        catalog.set_task(AgentId::new("agent-two"), cx.spawn(async |_, _| {}));
                    });
                },
                cx,
            );
        });

        let catalog = cx
            .update(|cx| refresh_external_agent_models(&project, cx))
            .expect("catalog");

        emit_agent_servers_updated(&project, cx);

        catalog.read_with(cx, |catalog, _cx| {
            assert!(catalog.state(&AgentId::new("agent-one")).is_some());
            assert!(matches!(
                catalog.state(&AgentId::new("agent-two")),
                Some(ExternalAgentModelsState::Loading)
            ));
            assert!(catalog.tasks.contains_key(&AgentId::new("agent-two")));
        });
    }

    #[gpui::test]
    async fn settings_change_invalidates_only_that_agent(cx: &mut TestAppContext) {
        init_test(cx);
        install_custom_agent("agent-one", cx);
        install_custom_agent("agent-two", cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        cx.run_until_parked();

        cx.update(|cx| {
            set_external_agent_model_source(
                |_project, catalog, cx| {
                    catalog.update(cx, |catalog, cx| {
                        for agent_id in ["agent-one", "agent-two"] {
                            catalog.set_state(
                                AgentId::new(agent_id),
                                ExternalAgentModelsState::Ready(vec![]),
                                cx,
                            );
                        }
                    });
                },
                cx,
            );
        });
        let catalog = cx
            .update(|cx| refresh_external_agent_models(&project, cx))
            .expect("catalog");

        // Rewriting agent-one's settings emits AgentServersUpdated through the
        // store's settings observer.
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |content| {
                    let entry = content
                        .agent_servers
                        .get_or_insert_default()
                        .0
                        .get_mut("agent-one")
                        .expect("agent-one settings");
                    let settings::CustomAgentServerSettings::Custom { args, .. } = entry else {
                        panic!("agent-one should be a custom agent");
                    };
                    args.push("--new-flag".to_string());
                });
            });
        });
        cx.run_until_parked();

        catalog.read_with(cx, |catalog, _cx| {
            assert!(catalog.state(&AgentId::new("agent-one")).is_none());
            assert!(catalog.state(&AgentId::new("agent-two")).is_some());
        });
    }

    #[gpui::test]
    async fn registry_version_change_invalidates_only_that_agent(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            AgentRegistryStore::init_test_global(cx, vec![registry_agent("reg-agent", "1.0.0")]);
        });
        install_registry_agent("reg-agent", cx);
        install_custom_agent("agent-two", cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        cx.run_until_parked();

        cx.update(|cx| {
            set_external_agent_model_source(
                |_project, catalog, cx| {
                    catalog.update(cx, |catalog, cx| {
                        for agent_id in ["reg-agent", "agent-two"] {
                            catalog.set_state(
                                AgentId::new(agent_id),
                                ExternalAgentModelsState::Ready(vec![]),
                                cx,
                            );
                        }
                    });
                },
                cx,
            );
        });
        let catalog = cx
            .update(|cx| refresh_external_agent_models(&project, cx))
            .expect("catalog");
        assert!(catalog.read_with(cx, |catalog, _cx| {
            catalog.state(&AgentId::new("reg-agent")).is_some()
        }));

        // A registry refresh carrying a new version makes the store re-emit
        // AgentServersUpdated.
        cx.update(|cx| {
            AgentRegistryStore::global(cx).update(cx, |store, cx| {
                store.set_agents(vec![registry_agent("reg-agent", "2.0.0")], cx)
            });
        });
        cx.run_until_parked();

        catalog.read_with(cx, |catalog, _cx| {
            assert!(catalog.state(&AgentId::new("reg-agent")).is_none());
            assert!(catalog.state(&AgentId::new("agent-two")).is_some());
        });
    }

    #[gpui::test]
    async fn removed_agent_is_dropped_from_the_catalog(cx: &mut TestAppContext) {
        init_test(cx);
        install_custom_agent("agent-one", cx);
        install_custom_agent("agent-two", cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        cx.run_until_parked();

        cx.update(|cx| {
            set_external_agent_model_source(
                |_project, catalog, cx| {
                    catalog.update(cx, |catalog, cx| {
                        for agent_id in ["agent-one", "agent-two"] {
                            catalog.set_state(
                                AgentId::new(agent_id),
                                ExternalAgentModelsState::Ready(vec![]),
                                cx,
                            );
                        }
                    });
                },
                cx,
            );
        });
        let catalog = cx
            .update(|cx| refresh_external_agent_models(&project, cx))
            .expect("catalog");

        uninstall_agent("agent-one", cx);
        cx.run_until_parked();

        catalog.read_with(cx, |catalog, _cx| {
            assert!(catalog.state(&AgentId::new("agent-one")).is_none());
            assert!(catalog.state(&AgentId::new("agent-two")).is_some());
        });
    }

    #[gpui::test]
    async fn agent_changes_invoke_source_only_when_enabled(cx: &mut TestAppContext) {
        init_test(cx);
        install_custom_agent("agent-one", cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        cx.run_until_parked();
        let call_count = Arc::new(AtomicUsize::new(0));

        cx.update(|cx| {
            set_external_agent_model_source(
                {
                    let call_count = call_count.clone();
                    move |_project, catalog, cx| {
                        call_count.fetch_add(1, Ordering::SeqCst);
                        catalog.update(cx, |catalog, cx| {
                            catalog.set_state(
                                AgentId::new("agent-one"),
                                ExternalAgentModelsState::Ready(vec![]),
                                cx,
                            );
                        });
                    }
                },
                cx,
            );
        });
        let catalog = cx
            .update(|cx| refresh_external_agent_models(&project, cx))
            .expect("catalog");
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        // Without refresh_on_agent_changes the source is not re-invoked.
        emit_agent_servers_updated(&project, cx);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
        assert_eq!(
            state_is(&catalog, "agent-one", cx).as_deref(),
            Some("ready")
        );

        cx.update(|cx| {
            catalog.update(cx, |catalog, _cx| catalog.enable_refresh_on_agent_changes());
        });
        uninstall_agent("agent-one", cx);
        cx.run_until_parked();

        // The invalidation re-invokes the source, which re-seeds a state for
        // the (now removed) agent — the point under test is the invocation.
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[gpui::test]
    async fn non_local_project_is_not_subscribed(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        project.update(cx, |project, _cx| project.mark_as_collab_for_testing());

        cx.update(|cx| {
            set_external_agent_model_source(|_project, _catalog, _cx| {}, cx);
        });
        let catalog = cx
            .update(|cx| refresh_external_agent_models(&project, cx))
            .expect("catalog");

        assert!(catalog.read_with(cx, |catalog, _cx| catalog.store_watches.is_empty()));

        // An event on the (unsubscribed) collab store does not reach the
        // catalog.
        let agent_server_store =
            project.read_with(cx, |project, _cx| project.agent_server_store().clone());
        agent_server_store.update(cx, |_store, cx| cx.emit(AgentServersUpdated));
        cx.run_until_parked();
        assert!(catalog.read_with(cx, |catalog, _cx| catalog.store_watches.is_empty()));
    }

    #[gpui::test]
    async fn watch_is_removed_when_project_is_dropped(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;

        cx.update(|cx| {
            set_external_agent_model_source(|_project, _catalog, _cx| {}, cx);
        });
        let catalog = cx.update(|cx| subscribe_external_agent_models(&project, cx));
        assert_eq!(
            catalog.read_with(cx, |catalog, _cx| catalog.store_watches.len()),
            1
        );

        drop(project);
        // Entity release is processed when the app flushes pending effects.
        cx.update(|_cx| {});
        cx.run_until_parked();

        assert!(catalog.read_with(cx, |catalog, _cx| catalog.store_watches.is_empty()));
    }
}
