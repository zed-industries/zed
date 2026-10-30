use std::rc::Rc;

use collections::{HashMap, IndexMap};
use gpui::{
    App, AppContext as _, Context, Entity, EntityId, Global, SharedString, Subscription, Task,
    WeakEntity,
};

use crate::{AgentId, Project};

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

pub struct ExternalAgentModelCatalog {
    states: IndexMap<AgentId, ExternalAgentModelsState>,
    tasks: HashMap<AgentId, Task<()>>,
    _agent_servers_updated: Subscription,
}

impl ExternalAgentModelCatalog {
    fn new(project: &Entity<Project>, cx: &mut Context<Self>) -> Self {
        let agent_server_store = project.read(cx).agent_server_store().clone();
        let _agent_servers_updated = cx
            .subscribe(&agent_server_store, |catalog, _store, _event, cx| {
                catalog.clear(cx)
            });
        Self {
            states: IndexMap::default(),
            tasks: HashMap::default(),
            _agent_servers_updated,
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
        self.states.insert(agent_id, state);
        cx.notify();
    }

    pub fn set_task(&mut self, agent_id: AgentId, task: Task<()>) {
        self.tasks.insert(agent_id, task);
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.states.clear();
        self.tasks.clear();
        cx.notify();
    }
}

type ExternalAgentModelSource =
    Rc<dyn Fn(Entity<Project>, Entity<ExternalAgentModelCatalog>, &mut App)>;

#[derive(Default)]
struct GlobalExternalAgentModelCatalogs {
    catalogs: HashMap<EntityId, (WeakEntity<Project>, Entity<ExternalAgentModelCatalog>)>,
    source: Option<ExternalAgentModelSource>,
}

impl Global for GlobalExternalAgentModelCatalogs {}

/// Registers the source responsible for populating per-project catalogs with
/// the models exposed by installed external agents. Called once by the crate
/// that can create agent sessions (currently `agent_ui`).
pub fn set_external_agent_model_source(
    source: impl Fn(Entity<Project>, Entity<ExternalAgentModelCatalog>, &mut App) + 'static,
    cx: &mut App,
) {
    cx.default_global::<GlobalExternalAgentModelCatalogs>()
        .source = Some(Rc::new(source));
}

/// Returns the catalog for this project if a model source has been registered
/// and a refresh created it. Never invokes the source.
pub fn external_agent_model_catalog(
    project: &Entity<Project>,
    cx: &App,
) -> Option<Entity<ExternalAgentModelCatalog>> {
    let (_, catalog) = cx
        .try_global::<GlobalExternalAgentModelCatalogs>()?
        .catalogs
        .get(&project.entity_id())?;
    Some(catalog.clone())
}

/// Returns the catalog for this project, creating it on first use, and asks the
/// registered source to refresh it exactly once. Returns `None` when no source
/// has been registered.
pub fn refresh_external_agent_models(
    project: &Entity<Project>,
    cx: &mut App,
) -> Option<Entity<ExternalAgentModelCatalog>> {
    let source = cx
        .try_global::<GlobalExternalAgentModelCatalogs>()
        .and_then(|catalogs| catalogs.source.clone())?;

    cx.global_mut::<GlobalExternalAgentModelCatalogs>()
        .catalogs
        .retain(|_entity_id, (project, _catalog)| project.upgrade().is_some());

    let existing = cx
        .global::<GlobalExternalAgentModelCatalogs>()
        .catalogs
        .get(&project.entity_id())
        .map(|(_, catalog)| catalog.clone());

    let catalog = if let Some(catalog) = existing {
        catalog
    } else {
        let catalog = cx.new(|cx| ExternalAgentModelCatalog::new(project, cx));
        cx.global_mut::<GlobalExternalAgentModelCatalogs>()
            .catalogs
            .insert(project.entity_id(), (project.downgrade(), catalog.clone()));
        catalog
    };

    source(project.clone(), catalog.clone(), cx);
    Some(catalog)
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    use fs::FakeFs;
    use gpui::TestAppContext;
    use settings::SettingsStore;

    use crate::AgentServersUpdated;

    use super::*;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    #[gpui::test]
    async fn no_source_returns_none(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        cx.update(|cx| {
            assert!(refresh_external_agent_models(&project, cx).is_none());
            assert!(external_agent_model_catalog(&project, cx).is_none());
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
            .update(|cx| external_agent_model_catalog(&project, cx))
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
    async fn agent_servers_updated_clears_states_and_pending_tasks(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        cx.update(|cx| {
            set_external_agent_model_source(
                |_project, catalog, cx| {
                    catalog.update(cx, |catalog, cx| {
                        catalog.set_state(
                            AgentId::new("agent-one"),
                            ExternalAgentModelsState::Ready(vec![]),
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
        assert!(catalog.read_with(cx, |catalog, _cx| {
            catalog.state(&AgentId::new("agent-one")).is_some()
        }));

        let agent_server_store =
            project.read_with(cx, |project, _cx| project.agent_server_store().clone());
        agent_server_store.update(cx, |_store, cx| cx.emit(AgentServersUpdated));

        catalog.read_with(cx, |catalog, _cx| {
            assert!(catalog.state(&AgentId::new("agent-one")).is_none());
            assert!(catalog.state(&AgentId::new("agent-two")).is_none());
            assert!(catalog.tasks.is_empty());
        });
    }

    #[gpui::test]
    async fn released_project_catalog_entry_is_purged(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;
        let project_entity_id = project.entity_id();

        cx.update(|cx| {
            set_external_agent_model_source(|_project, _catalog, _cx| {}, cx);
        });
        let catalog = cx
            .update(|cx| refresh_external_agent_models(&project, cx))
            .expect("catalog");

        let other_project = Project::test(fs, [], cx).await;
        drop(project);
        // Entity release is processed when the app flushes pending effects.
        cx.update(|_cx| {});

        cx.update(|cx| {
            refresh_external_agent_models(&other_project, cx);
        });
        cx.update(|cx| {
            assert!(
                !cx.global::<GlobalExternalAgentModelCatalogs>()
                    .catalogs
                    .contains_key(&project_entity_id)
            );
        });
        drop(catalog);
    }
}
