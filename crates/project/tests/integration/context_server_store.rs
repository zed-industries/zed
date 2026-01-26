use anyhow::Result;
use context_server::test::create_fake_transport;
use context_server::{ContextServer, ContextServerId};
use gpui::{AppContext, AsyncApp, Entity, Subscription, Task, TestAppContext, UpdateGlobal as _};
use http_client::{FakeHttpClient, Response};
use project::context_server_store::registry::ContextServerDescriptorRegistry;
use project::context_server_store::*;
use project::project_settings::ContextServerSettings;
use project::worktree_store::WorktreeStore;
use project::{
    FakeFs, Project, context_server_store::registry::ContextServerDescriptor,
    project_settings::ProjectSettings,
};
use serde_json::json;
use settings::{ContextServerCommand, Settings, SettingsStore};
use std::sync::Arc;
use std::{cell::RefCell, path::PathBuf, rc::Rc};
use util::path;

#[gpui::test]
async fn test_context_server_status(cx: &mut TestAppContext) {
    const SERVER_1_ID: &str = "mcp-1";
    const SERVER_2_ID: &str = "mcp-2";

    let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

    let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
    let store = cx.new(|cx| {
        ContextServerStore::test(
            registry.clone(),
            project.read(cx).worktree_store(),
            Some(project.downgrade()),
            cx,
        )
    });

    let server_1_id = ContextServerId(SERVER_1_ID.into());
    let server_2_id = ContextServerId(SERVER_2_ID.into());

    let server_1 = Arc::new(ContextServer::new(
        server_1_id.clone(),
        Arc::new(create_fake_transport(SERVER_1_ID, cx.executor())),
    ));
    let server_2 = Arc::new(ContextServer::new(
        server_2_id.clone(),
        Arc::new(create_fake_transport(SERVER_2_ID, cx.executor())),
    ));

    store.update(cx, |store, cx| store.test_start_server(server_1, cx));

    cx.run_until_parked();

    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_1_id),
            Some(ContextServerStatus::Running)
        );
        assert_eq!(store.read(cx).status_for_server(&server_2_id), None);
    });

    store.update(cx, |store, cx| {
        store.test_start_server(server_2.clone(), cx)
    });

    cx.run_until_parked();

    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_1_id),
            Some(ContextServerStatus::Running)
        );
        assert_eq!(
            store.read(cx).status_for_server(&server_2_id),
            Some(ContextServerStatus::Running)
        );
    });

    store
        .update(cx, |store, cx| store.stop_server(&server_2_id, cx))
        .unwrap();

    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_1_id),
            Some(ContextServerStatus::Running)
        );
        assert_eq!(
            store.read(cx).status_for_server(&server_2_id),
            Some(ContextServerStatus::Stopped)
        );
    });
}

#[gpui::test]
async fn test_context_server_status_events(cx: &mut TestAppContext) {
    const SERVER_1_ID: &str = "mcp-1";
    const SERVER_2_ID: &str = "mcp-2";

    let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

    let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
    let store = cx.new(|cx| {
        ContextServerStore::test(
            registry.clone(),
            project.read(cx).worktree_store(),
            Some(project.downgrade()),
            cx,
        )
    });

    let server_1_id = ContextServerId(SERVER_1_ID.into());
    let server_2_id = ContextServerId(SERVER_2_ID.into());

    let server_1 = Arc::new(ContextServer::new(
        server_1_id.clone(),
        Arc::new(create_fake_transport(SERVER_1_ID, cx.executor())),
    ));
    let server_2 = Arc::new(ContextServer::new(
        server_2_id.clone(),
        Arc::new(create_fake_transport(SERVER_2_ID, cx.executor())),
    ));

    let _server_events = assert_server_events(
        &store,
        vec![
            (server_1_id.clone(), ContextServerStatus::Starting),
            (server_1_id, ContextServerStatus::Running),
            (server_2_id.clone(), ContextServerStatus::Starting),
            (server_2_id.clone(), ContextServerStatus::Running),
            (server_2_id.clone(), ContextServerStatus::Stopped),
        ],
        cx,
    );

    store.update(cx, |store, cx| store.test_start_server(server_1, cx));

    cx.run_until_parked();

    store.update(cx, |store, cx| {
        store.test_start_server(server_2.clone(), cx)
    });

    cx.run_until_parked();

    store
        .update(cx, |store, cx| store.stop_server(&server_2_id, cx))
        .unwrap();
}

#[gpui::test(iterations = 25)]
async fn test_context_server_concurrent_starts(cx: &mut TestAppContext) {
    const SERVER_1_ID: &str = "mcp-1";

    let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

    let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
    let store = cx.new(|cx| {
        ContextServerStore::test(
            registry.clone(),
            project.read(cx).worktree_store(),
            Some(project.downgrade()),
            cx,
        )
    });

    let server_id = ContextServerId(SERVER_1_ID.into());

    let server_with_same_id_1 = Arc::new(ContextServer::new(
        server_id.clone(),
        Arc::new(create_fake_transport(SERVER_1_ID, cx.executor())),
    ));
    let server_with_same_id_2 = Arc::new(ContextServer::new(
        server_id.clone(),
        Arc::new(create_fake_transport(SERVER_1_ID, cx.executor())),
    ));

    // If we start another server with the same id, we should report that we stopped the previous one
    let _server_events = assert_server_events(
        &store,
        vec![
            (server_id.clone(), ContextServerStatus::Starting),
            (server_id.clone(), ContextServerStatus::Stopped),
            (server_id.clone(), ContextServerStatus::Starting),
            (server_id.clone(), ContextServerStatus::Running),
        ],
        cx,
    );

    store.update(cx, |store, cx| {
        store.test_start_server(server_with_same_id_1.clone(), cx)
    });
    store.update(cx, |store, cx| {
        store.test_start_server(server_with_same_id_2.clone(), cx)
    });

    cx.run_until_parked();

    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_id),
            Some(ContextServerStatus::Running)
        );
    });
}

#[gpui::test]
async fn test_context_server_maintain_servers_loop(cx: &mut TestAppContext) {
    const SERVER_1_ID: &str = "mcp-1";
    const SERVER_2_ID: &str = "mcp-2";

    let server_1_id = ContextServerId(SERVER_1_ID.into());
    let server_2_id = ContextServerId(SERVER_2_ID.into());

    let fake_descriptor_1 = Arc::new(FakeContextServerDescriptor::new(SERVER_1_ID));

    let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

    let executor = cx.executor();
    let store = project.read_with(cx, |project, _| project.context_server_store());
    store.update(cx, |store, cx| {
        store.set_context_server_factory(Box::new(move |id, _| {
            Arc::new(ContextServer::new(
                id.clone(),
                Arc::new(create_fake_transport(id.0.to_string(), executor.clone())),
            ))
        }));
        store.registry().update(cx, |registry, cx| {
            registry.register_context_server_descriptor(SERVER_1_ID.into(), fake_descriptor_1, cx);
        });
    });

    set_context_server_configuration(
        vec![(
            server_1_id.0.clone(),
            settings::ContextServerSettingsContent::Extension {
                enabled: true,
                remote: false,
                settings: json!({
                    "somevalue": true
                }),
            },
        )],
        cx,
    );

    // Ensure that mcp-1 starts up
    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_1_id.clone(), ContextServerStatus::Starting),
                (server_1_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );
        cx.run_until_parked();
    }

    // Ensure that mcp-1 is restarted when the configuration was changed
    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_1_id.clone(), ContextServerStatus::Stopped),
                (server_1_id.clone(), ContextServerStatus::Starting),
                (server_1_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );
        set_context_server_configuration(
            vec![(
                server_1_id.0.clone(),
                settings::ContextServerSettingsContent::Extension {
                    enabled: true,
                    remote: false,
                    settings: json!({
                        "somevalue": false
                    }),
                },
            )],
            cx,
        );

        cx.run_until_parked();
    }

    // Ensure that mcp-1 is not restarted when the configuration was not changed
    {
        let _server_events = assert_server_events(&store, vec![], cx);
        set_context_server_configuration(
            vec![(
                server_1_id.0.clone(),
                settings::ContextServerSettingsContent::Extension {
                    enabled: true,
                    remote: false,
                    settings: json!({
                        "somevalue": false
                    }),
                },
            )],
            cx,
        );

        cx.run_until_parked();
    }

    // Ensure that mcp-2 is started once it is added to the settings
    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_2_id.clone(), ContextServerStatus::Starting),
                (server_2_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );
        set_context_server_configuration(
            vec![
                (
                    server_1_id.0.clone(),
                    settings::ContextServerSettingsContent::Extension {
                        enabled: true,
                        remote: false,
                        settings: json!({
                            "somevalue": false
                        }),
                    },
                ),
                (
                    server_2_id.0.clone(),
                    settings::ContextServerSettingsContent::Stdio {
                        enabled: true,
                        remote: false,
                        command: ContextServerCommand {
                            path: "somebinary".into(),
                            args: vec!["arg".to_string()],
                            env: None,
                            timeout: None,
                        },
                    },
                ),
            ],
            cx,
        );

        cx.run_until_parked();
    }

    // Ensure that mcp-2 is restarted once the args have changed
    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_2_id.clone(), ContextServerStatus::Stopped),
                (server_2_id.clone(), ContextServerStatus::Starting),
                (server_2_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );
        set_context_server_configuration(
            vec![
                (
                    server_1_id.0.clone(),
                    settings::ContextServerSettingsContent::Extension {
                        enabled: true,
                        remote: false,
                        settings: json!({
                            "somevalue": false
                        }),
                    },
                ),
                (
                    server_2_id.0.clone(),
                    settings::ContextServerSettingsContent::Stdio {
                        enabled: true,
                        remote: false,
                        command: ContextServerCommand {
                            path: "somebinary".into(),
                            args: vec!["anotherArg".to_string()],
                            env: None,
                            timeout: None,
                        },
                    },
                ),
            ],
            cx,
        );

        cx.run_until_parked();
    }

    // Ensure that mcp-2 is removed once it is removed from the settings
    {
        let _server_events = assert_server_events(
            &store,
            vec![(server_2_id.clone(), ContextServerStatus::Stopped)],
            cx,
        );
        set_context_server_configuration(
            vec![(
                server_1_id.0.clone(),
                settings::ContextServerSettingsContent::Extension {
                    enabled: true,
                    remote: false,
                    settings: json!({
                        "somevalue": false
                    }),
                },
            )],
            cx,
        );

        cx.run_until_parked();

        cx.update(|cx| {
            assert_eq!(store.read(cx).status_for_server(&server_2_id), None);
        });
    }

    // Ensure that nothing happens if the settings do not change
    {
        let _server_events = assert_server_events(&store, vec![], cx);
        set_context_server_configuration(
            vec![(
                server_1_id.0.clone(),
                settings::ContextServerSettingsContent::Extension {
                    enabled: true,
                    remote: false,
                    settings: json!({
                        "somevalue": false
                    }),
                },
            )],
            cx,
        );

        cx.run_until_parked();

        cx.update(|cx| {
            assert_eq!(
                store.read(cx).status_for_server(&server_1_id),
                Some(ContextServerStatus::Running)
            );
            assert_eq!(store.read(cx).status_for_server(&server_2_id), None);
        });
    }
}

#[gpui::test]
async fn test_context_server_enabled_disabled(cx: &mut TestAppContext) {
    const SERVER_1_ID: &str = "mcp-1";

    let server_1_id = ContextServerId(SERVER_1_ID.into());

    let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

    let executor = cx.executor();
    let store = project.read_with(cx, |project, _| project.context_server_store());
    store.update(cx, |store, _| {
        store.set_context_server_factory(Box::new(move |id, _| {
            Arc::new(ContextServer::new(
                id.clone(),
                Arc::new(create_fake_transport(id.0.to_string(), executor.clone())),
            ))
        }));
    });

    set_context_server_configuration(
        vec![(
            server_1_id.0.clone(),
            settings::ContextServerSettingsContent::Stdio {
                enabled: true,
                remote: false,
                command: ContextServerCommand {
                    path: "somebinary".into(),
                    args: vec!["arg".to_string()],
                    env: None,
                    timeout: None,
                },
            },
        )],
        cx,
    );

    // Ensure that mcp-1 starts up
    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_1_id.clone(), ContextServerStatus::Starting),
                (server_1_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );
        cx.run_until_parked();
    }

    // Ensure that mcp-1 is stopped once it is disabled.
    {
        let _server_events = assert_server_events(
            &store,
            vec![(server_1_id.clone(), ContextServerStatus::Stopped)],
            cx,
        );
        set_context_server_configuration(
            vec![(
                server_1_id.0.clone(),
                settings::ContextServerSettingsContent::Stdio {
                    enabled: false,
                    remote: false,
                    command: ContextServerCommand {
                        path: "somebinary".into(),
                        args: vec!["arg".to_string()],
                        env: None,
                        timeout: None,
                    },
                },
            )],
            cx,
        );

        cx.run_until_parked();
    }

    // Ensure that mcp-1 is started once it is enabled again.
    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_1_id.clone(), ContextServerStatus::Starting),
                (server_1_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );
        set_context_server_configuration(
            vec![(
                server_1_id.0.clone(),
                settings::ContextServerSettingsContent::Stdio {
                    enabled: true,
                    remote: false,
                    command: ContextServerCommand {
                        path: "somebinary".into(),
                        args: vec!["arg".to_string()],
                        timeout: None,
                        env: None,
                    },
                },
            )],
            cx,
        );

        cx.run_until_parked();
    }
}

fn set_context_server_configuration(
    context_servers: Vec<(Arc<str>, settings::ContextServerSettingsContent)>,
    cx: &mut TestAppContext,
) {
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |content| {
                content.project.context_servers.clear();
                for (id, config) in context_servers {
                    content.project.context_servers.insert(id, config);
                }
            });
        })
    });
}

#[gpui::test]
async fn test_remote_context_server(cx: &mut TestAppContext) {
    const SERVER_ID: &str = "remote-server";
    let server_id = ContextServerId(SERVER_ID.into());
    let server_url = "http://example.com/api";

    let client = FakeHttpClient::create(|_| async move {
        use http_client::AsyncBody;

        let response = Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(
                serde_json::to_string(&json!({
                    "jsonrpc": "2.0",
                    "id": 0,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {},
                        "serverInfo": {
                            "name": "test-server",
                            "version": "1.0.0"
                        }
                    }
                }))
                .unwrap(),
            ))
            .unwrap();
        Ok(response)
    });
    cx.update(|cx| cx.set_http_client(client));

    let (_fs, project) = setup_context_server_test(cx, json!({ "code.rs": "" }), vec![]).await;

    let store = project.read_with(cx, |project, _| project.context_server_store());

    set_context_server_configuration(
        vec![(
            server_id.0.clone(),
            settings::ContextServerSettingsContent::Http {
                enabled: true,
                url: server_url.to_string(),
                headers: Default::default(),
                timeout: None,
            },
        )],
        cx,
    );

    let _server_events = assert_server_events(
        &store,
        vec![
            (server_id.clone(), ContextServerStatus::Starting),
            (server_id.clone(), ContextServerStatus::Running),
        ],
        cx,
    );
    cx.run_until_parked();
}

struct ServerEvents {
    received_event_count: Rc<RefCell<usize>>,
    expected_event_count: usize,
    _subscription: Subscription,
}

impl Drop for ServerEvents {
    fn drop(&mut self) {
        let actual_event_count = *self.received_event_count.borrow();
        assert_eq!(
            actual_event_count, self.expected_event_count,
            "
               Expected to receive {} context server store events, but received {} events",
            self.expected_event_count, actual_event_count
        );
    }
}

#[gpui::test]
async fn test_context_server_global_timeout(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(r#"{"context_server_timeout": 90}"#, cx)
                .expect("Failed to set test user settings");
        });
    });

    let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

    let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
    let store = cx.new(|cx| {
        ContextServerStore::test(
            registry.clone(),
            project.read(cx).worktree_store(),
            Some(project.downgrade()),
            cx,
        )
    });

    let mut async_cx = cx.to_async();
    let result = ContextServerStore::create_context_server(
        store.downgrade(),
        ContextServerId("test-server".into()),
        Arc::new(ContextServerConfiguration::Http {
            url: url::Url::parse("http://localhost:8080").expect("Failed to parse test URL"),
            headers: Default::default(),
            timeout: None,
        }),
        &mut async_cx,
    )
    .await;

    assert!(
        result.is_ok(),
        "Server should be created successfully with global timeout"
    );
}

#[gpui::test]
async fn test_context_server_per_server_timeout_override(cx: &mut TestAppContext) {
    const SERVER_ID: &str = "test-server";

    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(r#"{"context_server_timeout": 60}"#, cx)
                .expect("Failed to set test user settings");
        });
    });

    let (_fs, project) = setup_context_server_test(
        cx,
        json!({"code.rs": ""}),
        vec![(
            SERVER_ID.into(),
            ContextServerSettings::Http {
                enabled: true,
                url: "http://localhost:8080".to_string(),
                headers: Default::default(),
                timeout: Some(120),
            },
        )],
    )
    .await;

    let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
    let store = cx.new(|cx| {
        ContextServerStore::test(
            registry.clone(),
            project.read(cx).worktree_store(),
            Some(project.downgrade()),
            cx,
        )
    });

    let mut async_cx = cx.to_async();
    let result = ContextServerStore::create_context_server(
        store.downgrade(),
        ContextServerId("test-server".into()),
        Arc::new(ContextServerConfiguration::Http {
            url: url::Url::parse("http://localhost:8080").expect("Failed to parse test URL"),
            headers: Default::default(),
            timeout: Some(120),
        }),
        &mut async_cx,
    )
    .await;

    assert!(
        result.is_ok(),
        "Server should be created successfully with per-server timeout override"
    );
}

#[gpui::test]
async fn test_context_server_stdio_timeout(cx: &mut TestAppContext) {
    let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

    let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
    let store = cx.new(|cx| {
        ContextServerStore::test(
            registry.clone(),
            project.read(cx).worktree_store(),
            Some(project.downgrade()),
            cx,
        )
    });

    let mut async_cx = cx.to_async();
    let result = ContextServerStore::create_context_server(
        store.downgrade(),
        ContextServerId("stdio-server".into()),
        Arc::new(ContextServerConfiguration::Custom {
            command: ContextServerCommand {
                path: "/usr/bin/node".into(),
                args: vec!["server.js".into()],
                env: None,
                timeout: Some(180000),
            },
            remote: false,
        }),
        &mut async_cx,
    )
    .await;

    assert!(
        result.is_ok(),
        "Stdio server should be created successfully with timeout"
    );
}

fn assert_server_events(
    store: &Entity<ContextServerStore>,
    expected_events: Vec<(ContextServerId, ContextServerStatus)>,
    cx: &mut TestAppContext,
) -> ServerEvents {
    cx.update(|cx| {
        let mut ix = 0;
        let received_event_count = Rc::new(RefCell::new(0));
        let expected_event_count = expected_events.len();
        let subscription = cx.subscribe(store, {
            let received_event_count = received_event_count.clone();
            move |_, event, _| match event {
                Event::ServerStatusChanged {
                    server_id: actual_server_id,
                    status: actual_status,
                } => {
                    let (expected_server_id, expected_status) = &expected_events[ix];

                    assert_eq!(
                        actual_server_id, expected_server_id,
                        "Expected different server id at index {}",
                        ix
                    );
                    assert_eq!(
                        actual_status, expected_status,
                        "Expected different status at index {}",
                        ix
                    );
                    ix += 1;
                    *received_event_count.borrow_mut() += 1;
                }
            }
        });
        ServerEvents {
            expected_event_count,
            received_event_count,
            _subscription: subscription,
        }
    })
}

async fn setup_context_server_test(
    cx: &mut TestAppContext,
    files: serde_json::Value,
    context_server_configurations: Vec<(Arc<str>, ContextServerSettings)>,
) -> (Arc<FakeFs>, Entity<Project>) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        let mut settings = ProjectSettings::get_global(cx).clone();
        for (id, config) in context_server_configurations {
            settings.context_servers.insert(id, config);
        }
        ProjectSettings::override_global(settings, cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), files).await;
    let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

    (fs, project)
}

struct FakeContextServerDescriptor {
    path: PathBuf,
}

impl FakeContextServerDescriptor {
    fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl ContextServerDescriptor for FakeContextServerDescriptor {
    fn command(
        &self,
        _worktree_store: Entity<WorktreeStore>,
        _cx: &AsyncApp,
    ) -> Task<Result<ContextServerCommand>> {
        Task::ready(Ok(ContextServerCommand {
            path: self.path.clone(),
            args: vec!["arg1".to_string(), "arg2".to_string()],
            env: None,
            timeout: None,
        }))
    }

    fn configuration(
        &self,
        _worktree_store: Entity<WorktreeStore>,
        _cx: &AsyncApp,
    ) -> Task<Result<Option<::extension::ContextServerConfiguration>>> {
        Task::ready(Ok(None))
    }
}
