use std::{rc::Rc, time::Duration};

use acp_thread::{
    AcpThread, AgentConnection, AgentModelList, AgentModelSelector, AgentSessionConfigOptions,
    AuthRequired, LoadError,
};
use agent_client_protocol::schema::v2 as acp_v2;
use anyhow::{Result, anyhow};
use collections::HashSet;
use futures::{FutureExt as _, channel::oneshot, future::Shared};
use gpui::{App, AsyncApp, Entity, SharedString, Task, WeakEntity};
use project::{
    AgentId, Project,
    external_agent_models::{
        ExternalAgentModel, ExternalAgentModelCatalog, ExternalAgentModelsState,
        set_external_agent_model_source,
    },
};
use util::ResultExt as _;

use crate::agent_connection_store::AgentConnectedState;
use crate::{Agent, AgentPanel};

const PROBE_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) fn init(cx: &mut App) {
    set_external_agent_model_source(refresh, cx);
}

fn refresh(project: Entity<Project>, catalog: Entity<ExternalAgentModelCatalog>, cx: &mut App) {
    let agent_ids: Vec<AgentId> = project
        .read(cx)
        .agent_server_store()
        .read(cx)
        .external_agents()
        .cloned()
        .collect();

    if project.read(cx).is_via_collab() {
        fail_all(
            &catalog,
            &agent_ids,
            "External agents are not yet supported in shared projects.",
            cx,
        );
        return;
    }

    let Some(panel) = find_agent_panel(&project, cx) else {
        fail_all(&catalog, &agent_ids, "Agent Panel is not available", cx);
        return;
    };

    let open_threads: Vec<Entity<AcpThread>> = panel
        .read(cx)
        .conversation_views()
        .into_iter()
        .filter_map(|view| view.read(cx).root_thread(cx))
        .filter(|thread| {
            thread
                .read(cx)
                .connection()
                .clone()
                .downcast::<agent::NativeAgentConnection>()
                .is_none()
        })
        .collect();

    for agent_id in agent_ids {
        if matches!(
            catalog.read(cx).state(&agent_id),
            Some(ExternalAgentModelsState::Loading)
        ) {
            continue;
        }

        if let Some(thread) = open_threads
            .iter()
            .find(|thread| thread.read(cx).connection().agent_id() == agent_id)
        {
            read_open_thread(agent_id, thread.downgrade(), &catalog, cx);
            continue;
        }

        if matches!(
            catalog.read(cx).state(&agent_id),
            Some(ExternalAgentModelsState::Ready(_)) | Some(ExternalAgentModelsState::AuthRequired)
        ) {
            continue;
        }

        probe_agent(agent_id, &panel, &project, &catalog, cx);
    }
}

fn fail_all(
    catalog: &Entity<ExternalAgentModelCatalog>,
    agent_ids: &[AgentId],
    message: &'static str,
    cx: &mut App,
) {
    catalog.update(cx, |catalog, cx| {
        for agent_id in agent_ids {
            catalog.set_state(
                agent_id.clone(),
                ExternalAgentModelsState::Failed(message.into()),
                cx,
            );
        }
    });
}

fn find_agent_panel(project: &Entity<Project>, cx: &App) -> Option<Entity<AgentPanel>> {
    let app_state = workspace::AppState::try_global(cx)?;
    app_state
        .workspace_store
        .read(cx)
        .workspaces()
        .filter_map(|workspace| workspace.upgrade())
        .find_map(|workspace| {
            let workspace = workspace.read(cx);
            (workspace.project().entity_id() == project.entity_id())
                .then(|| workspace.panel::<AgentPanel>(cx))
                .flatten()
        })
}

fn read_open_thread(
    agent_id: AgentId,
    thread: WeakEntity<AcpThread>,
    catalog: &Entity<ExternalAgentModelCatalog>,
    cx: &mut App,
) {
    catalog.update(cx, |catalog, cx| {
        let task = cx.spawn({
            let agent_id = agent_id.clone();
            async move |catalog, cx| {
                let result = read_open_thread_models(&thread, cx).await;
                catalog
                    .update(cx, |catalog, cx| {
                        catalog.set_state(agent_id, result_to_state(result), cx)
                    })
                    .ok();
            }
        });
        catalog.set_task(agent_id, task);
    });
}

async fn read_open_thread_models(
    thread: &WeakEntity<AcpThread>,
    cx: &mut AsyncApp,
) -> Result<Vec<ExternalAgentModel>> {
    let (connection, session_id) = thread.read_with(cx, |thread, _cx| {
        (thread.connection().clone(), thread.session_id().clone())
    })?;
    read_models(&connection, &session_id, cx).await
}

fn probe_agent(
    agent_id: AgentId,
    panel: &Entity<AgentPanel>,
    project: &Entity<Project>,
    catalog: &Entity<ExternalAgentModelCatalog>,
    cx: &mut App,
) {
    catalog.update(cx, |catalog, cx| {
        catalog.set_state(agent_id.clone(), ExternalAgentModelsState::Loading, cx);
    });

    let agent = Agent::Custom {
        id: agent_id.clone(),
    };
    let fs = project.read(cx).fs().clone();
    let thread_store = panel.read(cx).thread_store().clone();
    let server = agent.server(fs, thread_store);
    let entry = panel
        .read(cx)
        .connection_store()
        .clone()
        .update(cx, |store, cx| store.request_connection(agent, server, cx));
    let connection_wait = entry.read(cx).wait_for_connection();

    let project = project.clone();
    catalog.update(cx, |catalog, cx| {
        let task = cx.spawn({
            let agent_id = agent_id.clone();
            async move |catalog, cx| {
                let result = run_probe(connection_wait, project, cx).await;
                catalog
                    .update(cx, |catalog, cx| {
                        catalog.set_state(agent_id, result_to_state(result), cx)
                    })
                    .ok();
            }
        });
        catalog.set_task(agent_id, task);
    });
}

async fn run_probe(
    connection_wait: Shared<Task<Result<AgentConnectedState, LoadError>>>,
    project: Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<Vec<ExternalAgentModel>> {
    let timeout = cx.background_executor().timer(PROBE_TIMEOUT).fuse();
    let probe = async {
        let connected = connection_wait
            .await
            .map_err(|error| anyhow::Error::new(error))?;
        let (result_tx, result_rx) = oneshot::channel();
        cx.spawn(async move |cx| {
            let result = run_probe_session(connected.connection, project, cx).await;
            result_tx.send(result).ok();
        })
        .detach();
        result_rx
            .await
            .map_err(|_| anyhow!("probe session task was dropped"))?
    }
    .fuse();
    futures::pin_mut!(probe);
    match futures::future::select(probe, timeout).await {
        futures::future::Either::Left((result, _)) => result,
        futures::future::Either::Right(_) => Err(anyhow!("Timed out after 30 seconds")),
    }
}

async fn run_probe_session(
    connection: Rc<dyn AgentConnection>,
    project: Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<Vec<ExternalAgentModel>> {
    let new_session = cx.update(|cx| {
        let work_dirs = project.read(cx).default_path_list(cx);
        connection.clone().new_session(project, work_dirs, cx)
    });
    let thread = new_session.await?;
    let session_id = thread.read_with(cx, |thread, _cx| thread.session_id().clone());
    let models = read_models(&connection, &session_id, cx).await;
    close_probe_session(&connection, &session_id, cx).await;
    models
}

async fn read_models(
    connection: &Rc<dyn AgentConnection>,
    session_id: &acp_v2::SessionId,
    cx: &mut AsyncApp,
) -> Result<Vec<ExternalAgentModel>> {
    enum ModelSource {
        ConfigOptions(Rc<dyn AgentSessionConfigOptions>),
        Selector(Rc<dyn AgentModelSelector>),
        None,
    }

    let source = cx.update(|cx| {
        if let Some(options) = connection.session_config_options(session_id, cx) {
            ModelSource::ConfigOptions(options)
        } else if let Some(selector) = connection.model_selector(session_id) {
            ModelSource::Selector(selector)
        } else {
            ModelSource::None
        }
    });

    let models = match source {
        ModelSource::ConfigOptions(options) => models_from_config_options(&*options),
        ModelSource::Selector(selector) => {
            let list = cx.update(|cx| selector.list_models(cx)).await?;
            models_from_model_list(&list)
        }
        ModelSource::None => Vec::new(),
    };
    Ok(dedup_models(models))
}

async fn close_probe_session(
    connection: &Rc<dyn AgentConnection>,
    session_id: &acp_v2::SessionId,
    cx: &mut AsyncApp,
) {
    if connection.supports_close_session() {
        let close = cx.update(|cx| connection.clone().close_session(session_id, cx));
        close.await.log_err();
    }
    if let Some(session_list) = cx.update(|cx| connection.session_list(cx)) {
        if session_list.supports_delete() {
            let delete = cx.update(|cx| session_list.delete_session(session_id, cx));
            delete.await.log_err();
        }
    }
}

fn models_from_config_options(
    config_options: &dyn AgentSessionConfigOptions,
) -> Vec<ExternalAgentModel> {
    let Some(option) = config_options.config_options().into_iter().find(|option| {
        option.category == Some(acp_v2::SessionConfigOptionCategory::Model)
            && matches!(option.kind, acp_v2::SessionConfigKind::Select(_))
    }) else {
        return Vec::new();
    };
    let acp_v2::SessionConfigKind::Select(select) = &option.kind else {
        return Vec::new();
    };
    match &select.options {
        acp_v2::SessionConfigSelectOptions::Ungrouped(options) => options
            .iter()
            .map(|option| ExternalAgentModel {
                id: SharedString::from(option.value.0.clone()),
                name: option.name.clone().into(),
                group: None,
            })
            .collect(),
        acp_v2::SessionConfigSelectOptions::Grouped(groups) => groups
            .iter()
            .flat_map(|group| {
                group.options.iter().map(|option| ExternalAgentModel {
                    id: SharedString::from(option.value.0.clone()),
                    name: option.name.clone().into(),
                    group: Some(group.name.clone().into()),
                })
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn models_from_model_list(list: &AgentModelList) -> Vec<ExternalAgentModel> {
    fn map_model(
        info: &acp_thread::AgentModelInfo,
        group: Option<SharedString>,
    ) -> Option<ExternalAgentModel> {
        if info.disabled.is_some() {
            return None;
        }
        Some(ExternalAgentModel {
            id: SharedString::from(info.id.as_str()),
            name: info.name.clone(),
            group,
        })
    }
    match list {
        AgentModelList::Flat(models) => models
            .iter()
            .filter_map(|info| map_model(info, None))
            .collect(),
        AgentModelList::Grouped(groups) => groups
            .iter()
            .flat_map(|(name, models)| {
                models
                    .iter()
                    .filter_map(|info| map_model(info, Some(name.0.clone())))
            })
            .collect(),
    }
}

fn dedup_models(models: Vec<ExternalAgentModel>) -> Vec<ExternalAgentModel> {
    let mut seen = HashSet::default();
    models
        .into_iter()
        .filter(|model| seen.insert(model.id.clone()))
        .collect()
}

fn result_to_state(result: Result<Vec<ExternalAgentModel>>) -> ExternalAgentModelsState {
    match result {
        Ok(models) => ExternalAgentModelsState::Ready(models),
        Err(error) => {
            if AuthRequired::from_error(&error).is_some() {
                ExternalAgentModelsState::AuthRequired
            } else {
                ExternalAgentModelsState::Failed(error.to_string().into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::collections::VecDeque;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use acp_thread::{
        AgentModelGroupName, AgentModelId, AgentModelInfo, AgentSessionList,
        AgentSessionListRequest, AgentSessionListResponse, StubSessionCounter,
    };
    use action_log::ActionLog;
    use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp};
    use agent_servers::{AgentServer, AgentServerDelegate};
    use collections::IndexMap;
    use fs::FakeFs;
    use gpui::{AppContext as _, TestAppContext, UpdateGlobal, VisualTestContext};
    use language_model::DisabledReason;
    use parking_lot::Mutex;
    use project::{AgentServersUpdated, external_agent_models};
    use settings::{CustomAgentServerSettings, SettingsStore};
    use workspace::{MultiWorkspace, PathList, Workspace};

    use crate::test_support::StubAgentServer;
    use crate::thread_metadata_store::ThreadMetadataStore;

    use super::*;

    fn init_test(cx: &mut TestAppContext, fs: Arc<FakeFs>) {
        crate::test_support::init_test(cx);
        cx.update(|cx| {
            <dyn fs::Fs>::set_global(fs, cx);
            language_model::LanguageModelRegistry::test(cx);
            agent::ThreadStore::init_global(cx);
            crate::thread_metadata_store::init(cx);
            init(cx);
        });
    }

    fn install_custom_agent(id: &str, cx: &mut TestAppContext) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |content| {
                    content.agent_servers.get_or_insert_default().insert(
                        id.to_string(),
                        CustomAgentServerSettings::Custom {
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

    async fn setup_workspace_with_panel(
        project: Entity<Project>,
        cx: &mut TestAppContext,
    ) -> (gpui::AnyWindowHandle, Entity<Workspace>, Entity<AgentPanel>) {
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let multi_workspace = window_handle
            .root(cx)
            .expect("MultiWorkspace root should exist");
        let workspace = multi_workspace.read_with(cx, |multi_workspace, _cx| {
            multi_workspace.workspace().clone()
        });
        let app_state = workspace.read_with(cx, |workspace, _cx| workspace.app_state().clone());
        cx.update(|cx| workspace::AppState::set_global(app_state, cx));

        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);
        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| crate::AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        cx.run_until_parked();
        (window_handle.into(), workspace, panel)
    }

    fn seed_connection(
        panel: &Entity<AgentPanel>,
        connection: CatalogTestConnection,
        cx: &mut VisualTestContext,
    ) {
        let server: Rc<dyn agent_servers::AgentServer> =
            Rc::new(StubAgentServer::new(connection.clone()).with_connection_agent_id());
        let agent = Agent::Custom {
            id: server.agent_id(),
        };
        panel
            .read_with(cx, |panel, _cx| panel.connection_store().clone())
            .update(cx, |store, cx| {
                store.request_connection(agent, server, cx);
            });
        cx.run_until_parked();
        assert_eq!(connection.new_session_call_count(), 0);
    }

    fn refresh(
        project: &Entity<Project>,
        cx: &mut VisualTestContext,
    ) -> Entity<ExternalAgentModelCatalog> {
        let catalog = cx
            .update(|_window, cx| external_agent_models::refresh_external_agent_models(project, cx))
            .expect("catalog should exist once the agent_ui source is registered");
        cx.run_until_parked();
        catalog
    }

    fn catalog_state(
        catalog: &Entity<ExternalAgentModelCatalog>,
        agent_id: &str,
        cx: &mut VisualTestContext,
    ) -> Option<String> {
        catalog.read_with(cx, |catalog, _cx| {
            catalog
                .state(&AgentId::new(agent_id))
                .map(|state| match state {
                    ExternalAgentModelsState::Loading => "loading".to_string(),
                    ExternalAgentModelsState::Ready(_) => "ready".to_string(),
                    ExternalAgentModelsState::AuthRequired => "auth_required".to_string(),
                    ExternalAgentModelsState::Failed(message) => format!("failed:{message}"),
                })
        })
    }

    fn ready_models(
        catalog: &Entity<ExternalAgentModelCatalog>,
        agent_id: &str,
        cx: &mut VisualTestContext,
    ) -> Vec<(String, String, Option<String>)> {
        catalog.read_with(cx, |catalog, _cx| {
            let Some(ExternalAgentModelsState::Ready(models)) =
                catalog.state(&AgentId::new(agent_id))
            else {
                panic!("expected Ready state for {agent_id}");
            };
            models
                .iter()
                .map(|model| {
                    (
                        model.id.to_string(),
                        model.name.to_string(),
                        model.group.as_ref().map(|group| group.to_string()),
                    )
                })
                .collect()
        })
    }

    #[gpui::test]
    async fn ungrouped_model_select_produces_deduped_ready_models(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("catalog-agent", cx);
        let (_window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(_window, cx);

        let connection = CatalogTestConnection::new("catalog-agent").with_config_options(vec![
            acp::SessionConfigOption::select(
                "mode",
                "Mode",
                "default",
                vec![acp::SessionConfigSelectOption::new("default", "Default")],
            )
            .category(acp::SessionConfigOptionCategory::Mode),
            acp::SessionConfigOption::select(
                "model",
                "Model",
                "model-1",
                vec![
                    acp::SessionConfigSelectOption::new("model-1", "Model One"),
                    acp::SessionConfigSelectOption::new("model-2", "Model Two"),
                    acp::SessionConfigSelectOption::new("model-1", "Duplicate Entry"),
                ],
            )
            .category(acp::SessionConfigOptionCategory::Model),
        ]);
        seed_connection(&panel, connection, cx);

        let catalog = refresh(&project, cx);
        assert_eq!(
            catalog_state(&catalog, "catalog-agent", cx).as_deref(),
            Some("ready")
        );
        assert_eq!(
            ready_models(&catalog, "catalog-agent", cx),
            vec![
                ("model-1".to_string(), "Model One".to_string(), None),
                ("model-2".to_string(), "Model Two".to_string(), None),
            ]
        );
    }

    #[gpui::test]
    async fn grouped_model_select_preserves_group_names(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("catalog-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let connection = CatalogTestConnection::new("catalog-agent").with_config_options(vec![
            acp::SessionConfigOption::select(
                "model",
                "Model",
                "fast",
                acp::SessionConfigSelectOptions::Grouped(vec![
                    acp::SessionConfigSelectGroup::new(
                        "fast-group",
                        "Fast Models",
                        vec![acp::SessionConfigSelectOption::new("fast", "Fast")],
                    ),
                    acp::SessionConfigSelectGroup::new(
                        "smart-group",
                        "Smart Models",
                        vec![
                            acp::SessionConfigSelectOption::new("smart", "Smart"),
                            acp::SessionConfigSelectOption::new("smarter", "Smarter"),
                        ],
                    ),
                ]),
            )
            .category(acp::SessionConfigOptionCategory::Model),
        ]);
        seed_connection(&panel, connection, cx);

        let catalog = refresh(&project, cx);
        assert_eq!(
            ready_models(&catalog, "catalog-agent", cx),
            vec![
                (
                    "fast".to_string(),
                    "Fast".to_string(),
                    Some("Fast Models".to_string())
                ),
                (
                    "smart".to_string(),
                    "Smart".to_string(),
                    Some("Smart Models".to_string())
                ),
                (
                    "smarter".to_string(),
                    "Smarter".to_string(),
                    Some("Smart Models".to_string())
                ),
            ]
        );
    }

    #[gpui::test]
    async fn config_options_without_model_category_produces_empty_list(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("catalog-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let connection = CatalogTestConnection::new("catalog-agent")
            .with_config_options(vec![
                acp::SessionConfigOption::select(
                    "mode",
                    "Mode",
                    "default",
                    vec![acp::SessionConfigSelectOption::new("default", "Default")],
                )
                .category(acp::SessionConfigOptionCategory::Mode),
            ])
            .with_model_list(AgentModelList::Flat(vec![stub_model("unused", "Unused")]));
        seed_connection(&panel, connection, cx);

        let catalog = refresh(&project, cx);
        assert_eq!(ready_models(&catalog, "catalog-agent", cx), Vec::new());
    }

    #[gpui::test]
    async fn model_selector_fallback_skips_disabled_and_keeps_groups(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("flat-agent", cx);
        install_custom_agent("grouped-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        seed_connection(
            &panel,
            CatalogTestConnection::new("flat-agent").with_model_list(AgentModelList::Flat(vec![
                stub_model("usable", "Usable"),
                stub_model_disabled("disabled", "Disabled"),
            ])),
            cx,
        );
        let mut groups = IndexMap::default();
        groups.insert(
            AgentModelGroupName("Family".into()),
            vec![stub_model_disabled("old", "Old"), stub_model("new", "New")],
        );
        seed_connection(
            &panel,
            CatalogTestConnection::new("grouped-agent")
                .with_model_list(AgentModelList::Grouped(groups)),
            cx,
        );

        let catalog = refresh(&project, cx);
        assert_eq!(
            ready_models(&catalog, "flat-agent", cx),
            vec![("usable".to_string(), "Usable".to_string(), None)]
        );
        assert_eq!(
            ready_models(&catalog, "grouped-agent", cx),
            vec![(
                "new".to_string(),
                "New".to_string(),
                Some("Family".to_string())
            )]
        );
    }

    #[gpui::test]
    async fn auth_errors_produce_auth_required_without_authenticate(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("struct-auth", cx);
        install_custom_agent("protocol-auth", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let struct_connection = CatalogTestConnection::new("struct-auth");
        struct_connection.fail_next_new_session(anyhow!(AuthRequired::new()));
        let protocol_connection = CatalogTestConnection::new("protocol-auth");
        protocol_connection.fail_next_new_session(anyhow!(acp::Error::auth_required()));
        seed_connection(&panel, struct_connection.clone(), cx);
        seed_connection(&panel, protocol_connection.clone(), cx);

        let catalog = refresh(&project, cx);
        assert_eq!(
            catalog_state(&catalog, "struct-auth", cx).as_deref(),
            Some("auth_required")
        );
        assert_eq!(
            catalog_state(&catalog, "protocol-auth", cx).as_deref(),
            Some("auth_required")
        );
        assert_eq!(struct_connection.authenticate_call_count(), 0);
        assert_eq!(protocol_connection.authenticate_call_count(), 0);
    }

    #[gpui::test]
    async fn hanging_new_session_times_out_and_late_session_is_closed(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("slow-agent", cx);
        install_custom_agent("fast-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let slow_connection = CatalogTestConnection::new("slow-agent").with_close_session();
        let release_session = slow_connection.defer_next_new_session();
        let fast_connection = CatalogTestConnection::new("fast-agent")
            .with_model_list(AgentModelList::Flat(vec![stub_model("model", "Model")]));
        seed_connection(&panel, slow_connection.clone(), cx);
        seed_connection(&panel, fast_connection, cx);

        let catalog = cx
            .update(|_window, cx| {
                external_agent_models::refresh_external_agent_models(&project, cx)
            })
            .expect("catalog");
        cx.run_until_parked();
        assert_eq!(slow_connection.new_session_call_count(), 1);
        assert_eq!(
            catalog_state(&catalog, "slow-agent", cx).as_deref(),
            Some("loading")
        );

        cx.executor()
            .advance_clock(PROBE_TIMEOUT + Duration::from_secs(1));
        cx.run_until_parked();

        assert_eq!(
            catalog_state(&catalog, "slow-agent", cx).as_deref(),
            Some("failed:Timed out after 30 seconds")
        );
        assert_eq!(
            catalog_state(&catalog, "fast-agent", cx).as_deref(),
            Some("ready")
        );

        // A late-resolving session must still be closed but must not write its
        // result into the catalog.
        release_session
            .send(Ok(()))
            .expect("probe should still be waiting");
        cx.run_until_parked();
        assert_eq!(slow_connection.close_session_call_count(), 1);
        assert_eq!(
            catalog_state(&catalog, "slow-agent", cx).as_deref(),
            Some("failed:Timed out after 30 seconds")
        );
    }

    #[gpui::test]
    async fn hanging_connect_times_out_and_refresh_reuses_pending_entry(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("hang-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let server = DeferredConnectServer::new(
            CatalogTestConnection::new("hang-agent")
                .with_model_list(AgentModelList::Flat(vec![stub_model("m", "M")])),
        );
        let release_connect = server.defer_next_connect();
        panel
            .read_with(cx, |panel, _cx| panel.connection_store().clone())
            .update(cx, |store, cx| {
                store.request_connection(
                    Agent::Custom {
                        id: AgentId::new("hang-agent"),
                    },
                    Rc::new(server.clone()),
                    cx,
                );
            });
        cx.run_until_parked();
        assert_eq!(server.connect_call_count(), 1);

        let catalog = refresh(&project, cx);
        assert_eq!(
            catalog_state(&catalog, "hang-agent", cx).as_deref(),
            Some("loading")
        );

        cx.executor()
            .advance_clock(PROBE_TIMEOUT + Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(
            catalog_state(&catalog, "hang-agent", cx).as_deref(),
            Some("failed:Timed out after 30 seconds")
        );

        // The retry waits on the same pending connection-store entry; it
        // must not spawn the agent again.
        let catalog = refresh(&project, cx);
        assert_eq!(server.connect_call_count(), 1);
        assert_eq!(
            catalog_state(&catalog, "hang-agent", cx).as_deref(),
            Some("loading")
        );

        release_connect.send(()).expect("connect still pending");
        cx.run_until_parked();
        assert_eq!(
            catalog_state(&catalog, "hang-agent", cx).as_deref(),
            Some("ready")
        );
        assert_eq!(
            ready_models(&catalog, "hang-agent", cx),
            vec![("m".to_string(), "M".to_string(), None)]
        );
    }

    #[gpui::test]
    async fn refresh_skips_agents_whose_probe_is_loading(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("catalog-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let connection = CatalogTestConnection::new("catalog-agent")
            .with_model_list(AgentModelList::Flat(vec![stub_model("m", "M")]));
        let release_session = connection.defer_next_new_session();
        seed_connection(&panel, connection.clone(), cx);

        let catalog = refresh(&project, cx);
        assert_eq!(connection.new_session_call_count(), 1);
        assert_eq!(
            catalog_state(&catalog, "catalog-agent", cx).as_deref(),
            Some("loading")
        );

        // Reopening the picker while the probe is in flight must not start
        // a second session for that agent.
        let catalog = refresh(&project, cx);
        assert_eq!(connection.new_session_call_count(), 1);
        assert_eq!(
            catalog_state(&catalog, "catalog-agent", cx).as_deref(),
            Some("loading")
        );

        release_session.send(Ok(())).expect("probe still waiting");
        cx.run_until_parked();
        assert_eq!(
            catalog_state(&catalog, "catalog-agent", cx).as_deref(),
            Some("ready")
        );
    }

    #[gpui::test]
    async fn agent_servers_updated_discards_in_flight_probe_result(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("catalog-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let connection = CatalogTestConnection::new("catalog-agent").with_close_session();
        let release_session = connection.defer_next_new_session();
        seed_connection(&panel, connection.clone(), cx);

        let catalog = refresh(&project, cx);
        assert_eq!(connection.new_session_call_count(), 1);
        assert_eq!(
            catalog_state(&catalog, "catalog-agent", cx).as_deref(),
            Some("loading")
        );

        // Uninstalling/changing agents mid-probe clears the catalog task; the
        // detached session lifecycle must still close the session, but the
        // late result must not be written back into the cleared catalog.
        let agent_server_store =
            project.read_with(cx, |project, _cx| project.agent_server_store().clone());
        agent_server_store.update(cx, |_store, cx| cx.emit(AgentServersUpdated));
        assert_eq!(catalog_state(&catalog, "catalog-agent", cx), None);

        release_session.send(Ok(())).expect("probe still waiting");
        cx.run_until_parked();
        assert_eq!(connection.close_session_call_count(), 1);
        assert_eq!(catalog_state(&catalog, "catalog-agent", cx), None);
    }

    #[gpui::test]
    async fn open_panel_session_supplies_models_without_new_session(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("open-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let connection = CatalogTestConnection::new("open-agent").with_config_options(vec![
            acp::SessionConfigOption::select(
                "model",
                "Model",
                "open-model",
                vec![acp::SessionConfigSelectOption::new(
                    "open-model",
                    "Open Model",
                )],
            )
            .category(acp::SessionConfigOptionCategory::Model),
        ]);
        crate::test_support::open_thread_with_custom_connection(&panel, connection.clone(), cx);
        assert_eq!(connection.new_session_call_count(), 1);

        let catalog = refresh(&project, cx);
        assert_eq!(connection.new_session_call_count(), 1);
        assert_eq!(
            ready_models(&catalog, "open-agent", cx),
            vec![("open-model".to_string(), "Open Model".to_string(), None)]
        );
    }

    #[gpui::test]
    async fn stable_states_are_not_reprobed_but_failed_entries_retry(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("retry-agent", cx);
        install_custom_agent("stable-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let retry_connection = CatalogTestConnection::new("retry-agent")
            .with_model_list(AgentModelList::Flat(vec![stub_model("m", "M")]));
        retry_connection.fail_next_new_session(anyhow!("boom"));
        let stable_connection = CatalogTestConnection::new("stable-agent")
            .with_model_list(AgentModelList::Flat(vec![stub_model("m", "M")]));
        seed_connection(&panel, retry_connection.clone(), cx);
        seed_connection(&panel, stable_connection.clone(), cx);

        let catalog = refresh(&project, cx);
        assert_eq!(
            catalog_state(&catalog, "retry-agent", cx).as_deref(),
            Some("failed:boom")
        );
        assert_eq!(retry_connection.new_session_call_count(), 1);
        assert_eq!(stable_connection.new_session_call_count(), 1);

        let catalog = refresh(&project, cx);
        assert_eq!(stable_connection.new_session_call_count(), 1);
        assert_eq!(retry_connection.new_session_call_count(), 2);
        assert_eq!(
            catalog_state(&catalog, "retry-agent", cx).as_deref(),
            Some("ready")
        );
    }

    #[gpui::test]
    async fn agent_servers_updated_clears_catalog_and_refresh_reprobes(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("catalog-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let connection = CatalogTestConnection::new("catalog-agent")
            .with_model_list(AgentModelList::Flat(vec![stub_model("m", "M")]));
        seed_connection(&panel, connection.clone(), cx);

        let catalog = refresh(&project, cx);
        assert_eq!(
            catalog_state(&catalog, "catalog-agent", cx).as_deref(),
            Some("ready")
        );
        assert_eq!(connection.new_session_call_count(), 1);

        let agent_server_store =
            project.read_with(cx, |project, _cx| project.agent_server_store().clone());
        agent_server_store.update(cx, |_store, cx| cx.emit(AgentServersUpdated));
        assert_eq!(catalog_state(&catalog, "catalog-agent", cx), None);

        let catalog = refresh(&project, cx);
        assert_eq!(connection.new_session_call_count(), 2);
        assert_eq!(
            catalog_state(&catalog, "catalog-agent", cx).as_deref(),
            Some("ready")
        );
    }

    #[gpui::test]
    async fn probe_session_is_closed_and_deleted_when_supported(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("catalog-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let session_list = CatalogTestSessionList::deleting();
        let connection = CatalogTestConnection::new("catalog-agent")
            .with_close_session()
            .with_session_list(session_list.clone())
            .with_model_list(AgentModelList::Flat(vec![stub_model("m", "M")]));
        seed_connection(&panel, connection.clone(), cx);

        let catalog = refresh(&project, cx);
        assert_eq!(
            catalog_state(&catalog, "catalog-agent", cx).as_deref(),
            Some("ready")
        );
        assert_eq!(connection.close_session_call_count(), 1);
        assert_eq!(session_list.delete_call_count(), 1);
    }

    #[gpui::test]
    async fn probe_does_not_mutate_sessions_or_create_ui(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("catalog-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        let options = CatalogTestConfigOptions::new(vec![
            acp::SessionConfigOption::select(
                "model",
                "Model",
                "m",
                vec![acp::SessionConfigSelectOption::new("m", "M")],
            )
            .category(acp::SessionConfigOptionCategory::Model),
        ]);
        let connection = CatalogTestConnection::new("catalog-agent")
            .with_config_options_handle(options.clone())
            .with_model_list(AgentModelList::Flat(vec![stub_model("m", "M")]));
        seed_connection(&panel, connection.clone(), cx);

        let metadata_entries =
            cx.update(|_window, cx| ThreadMetadataStore::global(cx).read(cx).entries().count());

        refresh(&project, cx);

        assert_eq!(connection.prompt_call_count(), 0);
        assert_eq!(connection.cancel_call_count(), 0);
        assert_eq!(connection.authenticate_call_count(), 0);
        assert_eq!(connection.select_model_call_count(), 0);
        assert_eq!(options.set_call_count(), 0);
        panel.read_with(cx, |panel, _cx| {
            assert!(panel.conversation_views().is_empty());
        });
        let metadata_entries_after =
            cx.update(|_window, cx| ThreadMetadataStore::global(cx).read(cx).entries().count());
        assert_eq!(metadata_entries, metadata_entries_after);
    }

    #[gpui::test]
    async fn collab_project_fails_without_requesting_connection(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("catalog-agent", cx);
        let (window, _workspace, panel) = setup_workspace_with_panel(project.clone(), cx).await;
        project.update(cx, |project, _cx| project.mark_as_collab_for_testing());
        let cx = &mut VisualTestContext::from_window(window, cx);

        let catalog = refresh(&project, cx);
        assert_eq!(
            catalog_state(&catalog, "catalog-agent", cx).as_deref(),
            Some("failed:External agents are not yet supported in shared projects.")
        );
        assert!(
            panel
                .read_with(cx, |panel, _cx| panel.connection_store().clone())
                .read_with(cx, |store, _cx| {
                    store
                        .entry(&Agent::Custom {
                            id: AgentId::new("catalog-agent"),
                        })
                        .is_none()
                })
        );
    }

    #[gpui::test]
    async fn missing_agent_panel_reports_unavailable(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        init_test(cx, fs.clone());
        let project = Project::test(fs, [], cx).await;
        install_custom_agent("catalog-agent", cx);
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let multi_workspace = window_handle
            .root(cx)
            .expect("MultiWorkspace root should exist");
        let workspace = multi_workspace.read_with(cx, |multi_workspace, _cx| {
            multi_workspace.workspace().clone()
        });
        let app_state = workspace.read_with(cx, |workspace, _cx| workspace.app_state().clone());
        cx.update(|cx| workspace::AppState::set_global(app_state, cx));
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        let catalog = refresh(&project, cx);
        assert_eq!(
            catalog_state(&catalog, "catalog-agent", cx).as_deref(),
            Some("failed:Agent Panel is not available")
        );
    }

    fn stub_model(id: &str, name: &str) -> AgentModelInfo {
        AgentModelInfo {
            id: AgentModelId::new(id),
            name: name.into(),
            description: None,
            icon: None,
            is_latest: false,
            cost: None,
            disabled: None,
        }
    }

    fn stub_model_disabled(id: &str, name: &str) -> AgentModelInfo {
        AgentModelInfo {
            disabled: Some(DisabledReason("unavailable".into())),
            ..stub_model(id, name)
        }
    }

    #[derive(Clone)]
    struct CatalogTestConfigOptions {
        options: Vec<acp::SessionConfigOption>,
        set_calls: Arc<AtomicUsize>,
    }

    impl CatalogTestConfigOptions {
        fn new(options: Vec<acp::SessionConfigOption>) -> Self {
            Self {
                options,
                set_calls: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn set_call_count(&self) -> usize {
            self.set_calls.load(Ordering::SeqCst)
        }
    }

    impl AgentSessionConfigOptions for CatalogTestConfigOptions {
        fn config_options(&self) -> Vec<acp::SessionConfigOption> {
            self.options.clone()
        }

        fn set_config_option(
            &self,
            _config_id: acp::SessionConfigId,
            _value: acp::SessionConfigOptionValue,
            _cx: &mut App,
        ) -> Task<Result<Vec<acp::SessionConfigOption>>> {
            self.set_calls.fetch_add(1, Ordering::SeqCst);
            Task::ready(Ok(self.options.clone()))
        }
    }

    struct CatalogTestModelSelector {
        list: AgentModelList,
        select_calls: Arc<AtomicUsize>,
    }

    impl AgentModelSelector for CatalogTestModelSelector {
        fn list_models(&self, _cx: &mut App) -> Task<Result<AgentModelList>> {
            Task::ready(Ok(self.list.clone()))
        }

        fn select_model(&self, _model_id: AgentModelId, _cx: &mut App) -> Task<Result<()>> {
            self.select_calls.fetch_add(1, Ordering::SeqCst);
            Task::ready(Ok(()))
        }

        fn selected_model(&self, _cx: &mut App) -> Task<Result<AgentModelInfo>> {
            let model = match &self.list {
                AgentModelList::Flat(models) => models.first().cloned(),
                AgentModelList::Grouped(groups) => {
                    groups.values().find_map(|models| models.first().cloned())
                }
            };
            Task::ready(model.ok_or_else(|| anyhow!("no models configured")))
        }
    }

    #[derive(Clone)]
    struct CatalogTestSessionList {
        supports_delete: bool,
        delete_calls: Arc<Mutex<Vec<acp::SessionId>>>,
    }

    impl CatalogTestSessionList {
        fn deleting() -> Self {
            Self {
                supports_delete: true,
                delete_calls: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn delete_call_count(&self) -> usize {
            self.delete_calls.lock().len()
        }
    }

    impl AgentSessionList for CatalogTestSessionList {
        fn list_sessions(
            &self,
            _request: AgentSessionListRequest,
            _cx: &mut App,
        ) -> Task<Result<AgentSessionListResponse>> {
            Task::ready(Ok(AgentSessionListResponse::new(Vec::new())))
        }

        fn supports_delete(&self) -> bool {
            self.supports_delete
        }

        fn delete_session(&self, session_id: &acp::SessionId, _cx: &mut App) -> Task<Result<()>> {
            self.delete_calls.lock().push(session_id.clone());
            Task::ready(Ok(()))
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    #[derive(Clone)]
    struct DeferredConnectServer {
        connection: CatalogTestConnection,
        connect_calls: Arc<AtomicUsize>,
        deferred_connects: Arc<Mutex<VecDeque<oneshot::Receiver<()>>>>,
    }

    impl DeferredConnectServer {
        fn new(connection: CatalogTestConnection) -> Self {
            Self {
                connection,
                connect_calls: Arc::new(AtomicUsize::new(0)),
                deferred_connects: Arc::new(Mutex::new(VecDeque::new())),
            }
        }

        fn defer_next_connect(&self) -> oneshot::Sender<()> {
            let (sender, receiver) = oneshot::channel();
            self.deferred_connects.lock().push_back(receiver);
            sender
        }

        fn connect_call_count(&self) -> usize {
            self.connect_calls.load(Ordering::SeqCst)
        }
    }

    impl AgentServer for DeferredConnectServer {
        fn logo(&self) -> ui::IconName {
            ui::IconName::ZedAgent
        }

        fn agent_id(&self) -> AgentId {
            self.connection.agent_id()
        }

        fn connect(
            &self,
            _delegate: AgentServerDelegate,
            _project: Entity<Project>,
            cx: &mut App,
        ) -> Task<Result<Rc<dyn AgentConnection>>> {
            self.connect_calls.fetch_add(1, Ordering::SeqCst);
            let connection = self.connection.clone();
            if let Some(receiver) = self.deferred_connects.lock().pop_front() {
                return cx.foreground_executor().spawn(async move {
                    receiver
                        .await
                        .map_err(|_| anyhow!("connect sender dropped"))?;
                    Ok(Rc::new(connection) as Rc<dyn AgentConnection>)
                });
            }
            Task::ready(Ok(Rc::new(connection)))
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    #[derive(Clone)]
    struct CatalogTestConnection {
        agent_id: AgentId,
        prompt_capabilities_rx: watch::Receiver<acp::PromptCapabilities>,
        config_options: Option<CatalogTestConfigOptions>,
        model_list: Option<AgentModelList>,
        session_list: Option<CatalogTestSessionList>,
        supports_close_session: bool,
        new_session_calls: Arc<AtomicUsize>,
        deferred_new_sessions: Arc<Mutex<VecDeque<oneshot::Receiver<Result<()>>>>>,
        new_session_errors: Arc<Mutex<VecDeque<anyhow::Error>>>,
        close_session_calls: Arc<Mutex<Vec<acp::SessionId>>>,
        authenticate_calls: Arc<AtomicUsize>,
        prompt_calls: Arc<AtomicUsize>,
        cancel_calls: Arc<AtomicUsize>,
        select_model_calls: Arc<AtomicUsize>,
    }

    impl CatalogTestConnection {
        fn new(agent_id: &str) -> Self {
            Self {
                agent_id: AgentId::new(agent_id),
                prompt_capabilities_rx: watch::Receiver::constant(
                    acp::PromptCapabilities::new()
                        .image(acp::PromptImageCapabilities::new())
                        .audio(acp::PromptAudioCapabilities::new())
                        .embedded_context(acp::PromptEmbeddedContextCapabilities::new()),
                ),
                config_options: None,
                model_list: None,
                session_list: None,
                supports_close_session: false,
                new_session_calls: Arc::new(AtomicUsize::new(0)),
                deferred_new_sessions: Arc::new(Mutex::new(VecDeque::new())),
                new_session_errors: Arc::new(Mutex::new(VecDeque::new())),
                close_session_calls: Arc::new(Mutex::new(Vec::new())),
                authenticate_calls: Arc::new(AtomicUsize::new(0)),
                prompt_calls: Arc::new(AtomicUsize::new(0)),
                cancel_calls: Arc::new(AtomicUsize::new(0)),
                select_model_calls: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn with_config_options(mut self, options: Vec<acp::SessionConfigOption>) -> Self {
            self.config_options = Some(CatalogTestConfigOptions::new(options));
            self
        }

        fn with_config_options_handle(mut self, options: CatalogTestConfigOptions) -> Self {
            self.config_options = Some(options);
            self
        }

        fn with_model_list(mut self, list: AgentModelList) -> Self {
            self.model_list = Some(list);
            self
        }

        fn with_session_list(mut self, session_list: CatalogTestSessionList) -> Self {
            self.session_list = Some(session_list);
            self
        }

        fn with_close_session(mut self) -> Self {
            self.supports_close_session = true;
            self
        }

        fn defer_next_new_session(&self) -> oneshot::Sender<Result<()>> {
            let (sender, receiver) = oneshot::channel();
            self.deferred_new_sessions.lock().push_back(receiver);
            sender
        }

        fn fail_next_new_session(&self, error: anyhow::Error) {
            self.new_session_errors.lock().push_back(error);
        }

        fn new_session_call_count(&self) -> usize {
            self.new_session_calls.load(Ordering::SeqCst)
        }

        fn close_session_call_count(&self) -> usize {
            self.close_session_calls.lock().len()
        }

        fn authenticate_call_count(&self) -> usize {
            self.authenticate_calls.load(Ordering::SeqCst)
        }

        fn prompt_call_count(&self) -> usize {
            self.prompt_calls.load(Ordering::SeqCst)
        }

        fn cancel_call_count(&self) -> usize {
            self.cancel_calls.load(Ordering::SeqCst)
        }

        fn select_model_call_count(&self) -> usize {
            self.select_model_calls.load(Ordering::SeqCst)
        }

        fn create_test_thread(
            self: Rc<Self>,
            project: Entity<Project>,
            work_dirs: PathList,
            cx: &mut App,
        ) -> Entity<AcpThread> {
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            cx.new(|cx| {
                AcpThread::new(
                    None,
                    Some("Catalog probe".into()),
                    Some(work_dirs),
                    self.clone(),
                    project,
                    action_log,
                    acp::SessionId::new(StubSessionCounter::next(cx).to_string()),
                    self.prompt_capabilities_rx.clone(),
                    cx,
                )
            })
        }
    }

    impl AgentConnection for CatalogTestConnection {
        fn agent_id(&self) -> AgentId {
            self.agent_id.clone()
        }

        fn telemetry_id(&self) -> SharedString {
            self.agent_id.0.clone()
        }

        fn auth_methods(&self) -> &[acp::AuthMethod] {
            &[]
        }

        fn authenticate(&self, _method: acp::AuthMethodId, _cx: &mut App) -> Task<Result<()>> {
            self.authenticate_calls.fetch_add(1, Ordering::SeqCst);
            Task::ready(Ok(()))
        }

        fn new_session(
            self: Rc<Self>,
            project: Entity<Project>,
            work_dirs: PathList,
            cx: &mut App,
        ) -> Task<Result<Entity<AcpThread>>> {
            self.new_session_calls.fetch_add(1, Ordering::SeqCst);
            if let Some(error) = self.new_session_errors.lock().pop_front() {
                return Task::ready(Err(error));
            }
            if let Some(receiver) = self.deferred_new_sessions.lock().pop_front() {
                let connection = self.clone();
                return cx.spawn(async move |cx| {
                    receiver.await.map_err(anyhow::Error::new)??;
                    Ok(cx.update(|cx| connection.create_test_thread(project, work_dirs, cx)))
                });
            }
            Task::ready(Ok(self.create_test_thread(project, work_dirs, cx)))
        }

        fn session_config_options(
            &self,
            _session_id: &acp::SessionId,
            _cx: &App,
        ) -> Option<Rc<dyn AgentSessionConfigOptions>> {
            self.config_options
                .clone()
                .map(|options| Rc::new(options) as Rc<dyn AgentSessionConfigOptions>)
        }

        fn model_selector(
            &self,
            _session_id: &acp::SessionId,
        ) -> Option<Rc<dyn AgentModelSelector>> {
            self.model_list.clone().map(|list| {
                Rc::new(CatalogTestModelSelector {
                    list,
                    select_calls: self.select_model_calls.clone(),
                }) as Rc<dyn AgentModelSelector>
            })
        }

        fn supports_close_session(&self) -> bool {
            self.supports_close_session
        }

        fn close_session(
            self: Rc<Self>,
            session_id: &acp::SessionId,
            _cx: &mut App,
        ) -> Task<Result<()>> {
            self.close_session_calls.lock().push(session_id.clone());
            Task::ready(Ok(()))
        }

        fn session_list(&self, _cx: &mut App) -> Option<Rc<dyn AgentSessionList>> {
            self.session_list
                .clone()
                .map(|list| Rc::new(list) as Rc<dyn AgentSessionList>)
        }

        fn prompt(
            &self,
            _params: acp::PromptRequest,
            _cx: &mut App,
        ) -> Task<Result<acp_v1::PromptResponse>> {
            self.prompt_calls.fetch_add(1, Ordering::SeqCst);
            Task::ready(Err(anyhow!("prompts are not supported")))
        }

        fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) -> Result<()> {
            self.cancel_calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }
}
