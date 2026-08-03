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
    DisableAiSettings, FakeFs, Project, context_server_store::registry::ContextServerDescriptor,
    project_settings::ProjectSettings,
};
use serde_json::json;
use settings::settings_content::SaturatingBool;
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

#[gpui::test]
async fn test_context_server_respects_disable_ai(cx: &mut TestAppContext) {
    const SERVER_1_ID: &str = "mcp-1";

    let server_1_id = ContextServerId(SERVER_1_ID.into());

    // Set up SettingsStore with disable_ai: true in user settings BEFORE creating project
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        DisableAiSettings::register(cx);
        // Set disable_ai via user settings (not override_global) so it persists through recompute_values
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |content| {
                content.project.disable_ai = Some(SaturatingBool(true));
            });
        });
    });

    // Now create the project (ContextServerStore will see disable_ai = true)
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({"code.rs": ""})).await;
    let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

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

    cx.run_until_parked();

    // Verify that no server started because AI is disabled
    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_1_id),
            None,
            "Server should not start when disable_ai is true"
        );
    });

    // Enable AI and verify server starts
    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_1_id.clone(), ContextServerStatus::Starting),
                (server_1_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |content| {
                    content.project.disable_ai = Some(SaturatingBool(false));
                });
            });
        });
        cx.run_until_parked();
    }

    // Disable AI again and verify server stops
    {
        let _server_events = assert_server_events(
            &store,
            vec![(server_1_id.clone(), ContextServerStatus::Stopped)],
            cx,
        );
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |content| {
                    content.project.disable_ai = Some(SaturatingBool(true));
                });
            });
        });
        cx.run_until_parked();
    }

    // Verify server is stopped
    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_1_id),
            Some(ContextServerStatus::Stopped),
            "Server should be stopped when disable_ai is true"
        );
    });
}

#[gpui::test]
async fn test_context_server_refreshed_when_worktree_added(cx: &mut TestAppContext) {
    const SERVER_1_ID: &str = "mcp-1";

    let server_1_id = ContextServerId(SERVER_1_ID.into());

    let (fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;
    fs.insert_tree(path!("/second"), json!({"other.rs": ""}))
        .await;

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

    // Witness that adding a worktree triggers the store to refresh available
    // servers (via `cx.notify` after `maintain_servers`). Without the
    // `WorktreeStoreEvent::WorktreeAdded` subscription in `ContextServerStore`,
    // this counter would remain zero.
    let notify_count = Rc::new(RefCell::new(0usize));
    let _notify_subscription = cx.update(|cx| {
        let count = notify_count.clone();
        cx.observe(&store, move |_, _| {
            *count.borrow_mut() += 1;
        })
    });

    {
        let _server_events = assert_server_events(&store, vec![], cx);
        let _ = project.update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/second"), true, cx)
        });
        cx.run_until_parked();
    }

    cx.update(|cx| {
        assert!(
            *notify_count.borrow() > 0,
            "Adding a worktree should trigger the context server store to refresh"
        );
        assert!(
            store.read(cx).server_ids().contains(&server_1_id),
            "Configured server list should still include the server after a worktree is added"
        );
        assert_eq!(
            store.read(cx).status_for_server(&server_1_id),
            Some(ContextServerStatus::Running),
            "Server should still be running after a worktree is added"
        );
    });
}

#[gpui::test]
async fn test_stdio_server_restarts_when_project_root_becomes_available(cx: &mut TestAppContext) {
    const SERVER_ID: &str = "mcp-1";
    let server_id = ContextServerId(SERVER_ID.into());

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/"),
        json!({"lonely.rs": "", "project": {"code.rs": ""}}),
    )
    .await;

    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    });

    // Open a single-file worktree, whose `root_dir()` is `None`, so the server is
    // spawned with no working directory even though it is configured.
    let project = Project::test(fs.clone(), [path!("/lonely.rs").as_ref()], cx).await;

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

    // Configure the server globally so it starts against the file worktree.
    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );
        set_context_server_configuration(
            vec![(
                server_id.0.clone(),
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
        cx.run_until_parked();
    }

    // Adding a directory worktree makes the project root resolvable. Since the
    // server was started with working directory `None`, it must be restarted so
    // it picks up the new root — otherwise it keeps running under Zed's own cwd.
    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_id.clone(), ContextServerStatus::Stopped),
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );
        project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path!("/project"), true, cx)
            })
            .await
            .expect("Failed to add worktree");
        cx.run_until_parked();
    }

    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_id),
            Some(ContextServerStatus::Running),
            "Server should be running again after restarting with the project root"
        );
    });
}

#[gpui::test]
async fn test_server_ids_includes_disabled_servers(cx: &mut TestAppContext) {
    const ENABLED_SERVER_ID: &str = "enabled-server";
    const DISABLED_SERVER_ID: &str = "disabled-server";

    let enabled_server_id = ContextServerId(ENABLED_SERVER_ID.into());
    let disabled_server_id = ContextServerId(DISABLED_SERVER_ID.into());

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

    // Configure one enabled and one disabled server
    set_context_server_configuration(
        vec![
            (
                enabled_server_id.0.clone(),
                settings::ContextServerSettingsContent::Stdio {
                    enabled: true,
                    remote: false,
                    command: ContextServerCommand {
                        path: "somebinary".into(),
                        args: vec![],
                        env: None,
                        timeout: None,
                    },
                },
            ),
            (
                disabled_server_id.0.clone(),
                settings::ContextServerSettingsContent::Stdio {
                    enabled: false,
                    remote: false,
                    command: ContextServerCommand {
                        path: "somebinary".into(),
                        args: vec![],
                        env: None,
                        timeout: None,
                    },
                },
            ),
        ],
        cx,
    );

    cx.run_until_parked();

    // Verify that server_ids includes both enabled and disabled servers
    cx.update(|cx| {
        let server_ids = store.read(cx).server_ids().to_vec();
        assert!(
            server_ids.contains(&enabled_server_id),
            "server_ids should include enabled server"
        );
        assert!(
            server_ids.contains(&disabled_server_id),
            "server_ids should include disabled server"
        );
    });

    // Verify that the enabled server is running and the disabled server is not
    cx.read(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&enabled_server_id),
            Some(ContextServerStatus::Running),
            "enabled server should be running"
        );
        // Disabled server should not be in the servers map (status returns None)
        // but should still be in server_ids
        assert_eq!(
            store.read(cx).status_for_server(&disabled_server_id),
            None,
            "disabled server should not have a status (not in servers map)"
        );
    });
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
                oauth: None,
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

// A server may accept `initialize` unauthenticated (returns 200) yet only send
// `WWW-Authenticate` on a later request such as `tools/list` / `tools/call`.
// That post-initialize 401 must initiate the OAuth flow instead of surfacing as
// an opaque request failure while the server stays "Running".
#[gpui::test]
async fn test_http_server_authenticates_on_post_init_401(cx: &mut TestAppContext) {
    use context_server::transport::TransportError;

    const SERVER_ID: &str = "auth-server";
    let server_id = ContextServerId(SERVER_ID.into());

    set_fake_mcp_http_client(cx, |message| {
        if message.contains("\"method\":\"initialize\"") {
            Ok(initialize_response())
        } else if message.contains("notifications/initialized") {
            Ok(notification_accepted_response())
        } else {
            Ok(unauthorized_response())
        }
    });

    let (_fs, project) = setup_context_server_test(cx, json!({ "code.rs": "" }), vec![]).await;
    let store = project.read_with(cx, |project, _| project.context_server_store());

    set_http_context_server_configuration(&server_id, cx);

    {
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

    let client = store.read_with(cx, |store, _| {
        store
            .get_running_server(&server_id)
            .expect("server should be running")
            .client()
            .expect("running server should have a client")
    });

    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::AuthRequired),
            ],
            cx,
        );

        // The 401 tears down the client, and the request that carried it fails
        // with the typed error.
        let error = client
            .request::<context_server::types::requests::ListTools>(())
            .await
            .expect_err("request challenged with a 401 should fail");
        assert!(
            matches!(
                error.downcast_ref::<TransportError>(),
                Some(TransportError::AuthRequired { .. })
            ),
            "expected an AuthRequired error, got: {error}"
        );

        cx.run_until_parked();
    }

    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_id),
            Some(ContextServerStatus::AuthRequired),
            "server should require authentication after a post-initialize 401"
        );
    });
}

// The first 401 may even arrive on a notification (`notifications/initialized`
// here): the send fails with no request in flight to carry a typed error back,
// and the client is dead by the time anything notices. Watching the transport
// shutdown must still move the server into `AuthRequired`.
#[gpui::test]
async fn test_http_server_authenticates_on_notification_401(cx: &mut TestAppContext) {
    const SERVER_ID: &str = "auth-server";
    let server_id = ContextServerId(SERVER_ID.into());

    set_fake_mcp_http_client(cx, |message| {
        if message.contains("\"method\":\"initialize\"") {
            Ok(initialize_response())
        } else {
            Ok(unauthorized_response())
        }
    });

    let (_fs, project) = setup_context_server_test(cx, json!({ "code.rs": "" }), vec![]).await;
    let store = project.read_with(cx, |project, _| project.context_server_store());

    set_http_context_server_configuration(&server_id, cx);

    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::Running),
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::AuthRequired),
            ],
            cx,
        );
        cx.run_until_parked();
    }

    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_id),
            Some(ContextServerStatus::AuthRequired),
            "server should require authentication after a 401 on a notification"
        );
    });
}

// A transport failure that is not an authentication challenge must not touch
// the server's state: no spurious auth flow, and (as before the transport
// watch existed) the server stays `Running`.
#[gpui::test]
async fn test_http_server_ignores_non_auth_transport_failure(cx: &mut TestAppContext) {
    const SERVER_ID: &str = "flaky-server";
    let server_id = ContextServerId(SERVER_ID.into());

    set_fake_mcp_http_client(cx, |message| {
        if message.contains("\"method\":\"initialize\"") {
            Ok(initialize_response())
        } else if message.contains("notifications/initialized") {
            Ok(notification_accepted_response())
        } else {
            Err(anyhow::anyhow!("connection reset"))
        }
    });

    let (_fs, project) = setup_context_server_test(cx, json!({ "code.rs": "" }), vec![]).await;
    let store = project.read_with(cx, |project, _| project.context_server_store());

    set_http_context_server_configuration(&server_id, cx);

    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );
        cx.run_until_parked();

        let client = store.read_with(cx, |store, _| {
            store
                .get_running_server(&server_id)
                .expect("server should be running")
                .client()
                .expect("running server should have a client")
        });

        client
            .request::<context_server::types::requests::ListTools>(())
            .await
            .expect_err("request should fail when the transport errors");

        cx.run_until_parked();
        // Dropping the events guard asserts no further status change happened.
    }

    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_id),
            Some(ContextServerStatus::Running),
            "a non-auth transport failure should not change the server state"
        );
    });
}

// A server may also require authentication on `initialize` itself. The
// challenge is read from the transport slot rather than the returned error, so
// the 401 is recognized even if another error (e.g. the request timeout) wins
// the race to become the reported startup failure.
#[gpui::test]
async fn test_http_server_authenticates_on_initialize_401(cx: &mut TestAppContext) {
    const SERVER_ID: &str = "auth-server";
    let server_id = ContextServerId(SERVER_ID.into());

    set_fake_mcp_http_client(cx, |_message| Ok(unauthorized_response()));

    let (_fs, project) = setup_context_server_test(cx, json!({ "code.rs": "" }), vec![]).await;
    let store = project.read_with(cx, |project, _| project.context_server_store());

    set_http_context_server_configuration(&server_id, cx);

    {
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::AuthRequired),
            ],
            cx,
        );
        cx.run_until_parked();
    }

    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_id),
            Some(ContextServerStatus::AuthRequired),
            "server should require authentication after a 401 on initialize"
        );
    });
}

// Restarting a server reuses its transport (e.g. via the MCP settings page),
// so a challenge recorded by a previous client generation must not leak into
// the next one: after a successful restart, a non-auth transport failure must
// not trip a spurious auth flow on the stale challenge.
#[gpui::test]
async fn test_http_server_restart_clears_stale_auth_challenge(cx: &mut TestAppContext) {
    use std::sync::atomic::{AtomicBool, Ordering};

    const SERVER_ID: &str = "auth-server";
    let server_id = ContextServerId(SERVER_ID.into());

    let restarted = Arc::new(AtomicBool::new(false));
    set_fake_mcp_http_client(cx, {
        let restarted = restarted.clone();
        move |message| {
            if message.contains("\"method\":\"initialize\"") {
                Ok(initialize_response())
            } else if message.contains("notifications/initialized") {
                Ok(notification_accepted_response())
            } else if restarted.load(Ordering::SeqCst) {
                Err(anyhow::anyhow!("connection reset"))
            } else {
                Ok(unauthorized_response())
            }
        }
    });

    let (_fs, project) = setup_context_server_test(cx, json!({ "code.rs": "" }), vec![]).await;
    let store = project.read_with(cx, |project, _| project.context_server_store());

    set_http_context_server_configuration(&server_id, cx);
    cx.run_until_parked();

    // A post-initialize 401 records a challenge on the transport and moves the
    // server into AuthRequired.
    let client = store.read_with(cx, |store, _| {
        store
            .get_running_server(&server_id)
            .expect("server should be running")
            .client()
            .expect("running server should have a client")
    });
    client
        .request::<context_server::types::requests::ListTools>(())
        .await
        .expect_err("request challenged with a 401 should fail");
    // Drop our handle so the dead client fully goes away, as it does in
    // production once the store has stopped it: a lingering client would
    // compete with its successor for the reused transport's response channel.
    drop(client);
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_id),
            Some(ContextServerStatus::AuthRequired),
        );
    });

    // The user restarts the same server instance instead of authenticating,
    // and the server no longer challenges.
    restarted.store(true, Ordering::SeqCst);
    store.update(cx, |store, cx| {
        let server = store.get_server(&server_id).expect("server should exist");
        store.start_server(server, cx);
    });
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_id),
            Some(ContextServerStatus::Running),
        );
    });

    let client = store.read_with(cx, |store, _| {
        store
            .get_running_server(&server_id)
            .expect("server should be running")
            .client()
            .expect("running server should have a client")
    });
    client
        .request::<context_server::types::requests::ListTools>(())
        .await
        .expect_err("request should fail when the transport errors");
    cx.run_until_parked();

    cx.update(|cx| {
        assert_eq!(
            store.read(cx).status_for_server(&server_id),
            Some(ContextServerStatus::Running),
            "a stale challenge from a previous client generation must not trigger auth"
        );
    });
}

fn set_http_context_server_configuration(server_id: &ContextServerId, cx: &mut TestAppContext) {
    set_context_server_configuration(
        vec![(
            server_id.0.clone(),
            settings::ContextServerSettingsContent::Http {
                enabled: true,
                url: "https://mcp.example.com/mcp".to_string(),
                headers: Default::default(),
                timeout: None,
                oauth: None,
            },
        )],
        cx,
    );
}

/// A fake HTTP client that serves the OAuth discovery documents (CIMD-capable,
/// so no dynamic client registration is needed) and routes MCP endpoint POSTs
/// to `respond_to_mcp_message` by the JSON-RPC message in the request body.
fn set_fake_mcp_http_client(
    cx: &mut TestAppContext,
    respond_to_mcp_message: impl Fn(&str) -> Result<Response<http_client::AsyncBody>>
    + Send
    + Sync
    + 'static,
) {
    let respond_to_mcp_message = Arc::new(respond_to_mcp_message);
    let client = FakeHttpClient::create(move |request| {
        let respond_to_mcp_message = respond_to_mcp_message.clone();
        async move {
            let uri = request.uri().to_string();
            let discovery_document = if uri.contains("oauth-protected-resource") {
                Some(json!({
                    "resource": "https://mcp.example.com",
                    "authorization_servers": ["https://auth.example.com"],
                    "scopes_supported": ["mcp:read"]
                }))
            } else if uri.contains("oauth-authorization-server") {
                Some(json!({
                    "issuer": "https://auth.example.com",
                    "authorization_endpoint": "https://auth.example.com/authorize",
                    "token_endpoint": "https://auth.example.com/token",
                    "code_challenge_methods_supported": ["S256"],
                    "client_id_metadata_document_supported": true
                }))
            } else {
                None
            };
            if let Some(document) = discovery_document {
                return Ok(json_response(document));
            }

            let mut body = request.into_body();
            let mut message = String::new();
            futures::AsyncReadExt::read_to_string(&mut body, &mut message).await?;
            respond_to_mcp_message(&message)
        }
    });
    cx.update(|cx| cx.set_http_client(client));
}

fn json_response(body: serde_json::Value) -> Response<http_client::AsyncBody> {
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(http_client::AsyncBody::from(body.to_string()))
        .unwrap()
}

fn initialize_response() -> Response<http_client::AsyncBody> {
    json_response(json!({
        "jsonrpc": "2.0",
        "id": 0,
        "result": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "serverInfo": { "name": "test-server", "version": "1.0.0" }
        }
    }))
}

fn notification_accepted_response() -> Response<http_client::AsyncBody> {
    Response::builder()
        .status(202)
        .body(http_client::AsyncBody::empty())
        .unwrap()
}

fn unauthorized_response() -> Response<http_client::AsyncBody> {
    Response::builder()
        .status(401)
        .header("WWW-Authenticate", "Bearer")
        .body(http_client::AsyncBody::empty())
        .unwrap()
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
            oauth: None,
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
                oauth: None,
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
            oauth: None,
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

#[gpui::test]
async fn test_multi_worktree_context_server_settings(cx: &mut TestAppContext) {
    const SERVER_A: &str = "server-from-project-a";
    const SERVER_B: &str = "server-from-project-b";

    let server_a_id = ContextServerId(SERVER_A.into());
    let server_b_id = ContextServerId(SERVER_B.into());

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project_a"),
        json!({
            ".zed": {
                "settings.json": serde_json::to_string(&json!({
                    "context_servers": {
                        "server-from-project-a": {
                            "command": "server-a-binary",
                            "args": []
                        }
                    }
                })).unwrap()
            },
            "code.rs": ""
        }),
    )
    .await;
    fs.insert_tree(
        path!("/project_b"),
        json!({
            ".zed": {
                "settings.json": serde_json::to_string(&json!({
                    "context_servers": {
                        "server-from-project-b": {
                            "command": "server-b-binary",
                            "args": []
                        }
                    }
                })).unwrap()
            },
            "code.rs": ""
        }),
    )
    .await;

    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    });

    // Create project with only project_a initially
    let project = Project::test(fs.clone(), [path!("/project_a").as_ref()], cx).await;

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

    cx.run_until_parked();

    // Only server-a should be configured
    cx.update(|cx| {
        let configured = store.read(cx).configured_server_ids();
        assert!(
            configured.contains(&server_a_id),
            "server-a should be configured from project_a"
        );
        assert!(
            !configured.contains(&server_b_id),
            "server-b should not be configured yet"
        );
    });

    // Add project_b as a second worktree
    project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/project_b"), true, cx)
        })
        .await
        .expect("Failed to add second worktree");

    cx.run_until_parked();

    // Both servers should now be configured
    cx.update(|cx| {
        let configured = store.read(cx).configured_server_ids();
        assert!(
            configured.contains(&server_a_id),
            "server-a should still be configured from project_a"
        );
        assert!(
            configured.contains(&server_b_id),
            "server-b should now be configured from project_b"
        );
    });
}

#[gpui::test]
async fn test_multi_worktree_duplicate_server_first_wins(cx: &mut TestAppContext) {
    const SHARED_SERVER: &str = "shared-server";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project_a"),
        json!({
            ".zed": {
                "settings.json": serde_json::to_string(&json!({
                    "context_servers": {
                        "shared-server": {
                            "command": "binary-from-a",
                            "args": ["arg-a"]
                        }
                    }
                })).unwrap()
            },
            "code.rs": ""
        }),
    )
    .await;
    fs.insert_tree(
        path!("/project_b"),
        json!({
            ".zed": {
                "settings.json": serde_json::to_string(&json!({
                    "context_servers": {
                        "shared-server": {
                            "command": "binary-from-b",
                            "args": ["arg-b"]
                        }
                    }
                })).unwrap()
            },
            "code.rs": ""
        }),
    )
    .await;

    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    });

    // Create project with both worktrees
    let project = Project::test(
        fs.clone(),
        [path!("/project_a").as_ref(), path!("/project_b").as_ref()],
        cx,
    )
    .await;

    cx.run_until_parked();

    let store = project.read_with(cx, |project, _| project.context_server_store());

    // The server should appear exactly once
    cx.update(|cx| {
        let configured = store.read(cx).configured_server_ids();
        let count = configured
            .iter()
            .filter(|id| id.0.as_ref() == SHARED_SERVER)
            .count();
        assert_eq!(count, 1, "duplicate server ID should appear exactly once");
    });
}

#[gpui::test]
async fn test_is_server_enabled(cx: &mut TestAppContext) {
    // We'll be setting up 4 different servers in order to test the following
    // scenarios:
    //
    // 1. Explicit Settings, Enabled
    // 2. Explicit Settings, Disabled
    // 3. No Settings, Registry Descriptor, Enabled
    // 4. No Settings, No Descriptor, Disabled
    const SERVER_1_ID: &str = "mcp-1";
    const SERVER_2_ID: &str = "mcp-2";
    const SERVER_3_ID: &str = "mcp-3";
    const SERVER_4_ID: &str = "mcp-4";

    let (_fs, project) = setup_context_server_test(
        cx,
        json!({"code.rs": ""}),
        vec![
            (
                SERVER_1_ID.into(),
                ContextServerSettings::Extension {
                    enabled: true,
                    remote: false,
                    settings: json!({}),
                },
            ),
            (
                SERVER_2_ID.into(),
                ContextServerSettings::Extension {
                    enabled: false,
                    remote: false,
                    settings: json!({}),
                },
            ),
        ],
    )
    .await;

    let registry = cx.new(|cx| {
        let mut registry = ContextServerDescriptorRegistry::new();
        let descriptor = Arc::new(FakeContextServerDescriptor::new(SERVER_3_ID));
        registry.register_context_server_descriptor(SERVER_3_ID.into(), descriptor, cx);

        registry
    });

    let store = cx.new(|cx| {
        ContextServerStore::test(
            registry.clone(),
            project.read(cx).worktree_store(),
            Some(project.downgrade()),
            cx,
        )
    });

    // Sanity check before proceeding, confirm server 1 and 2 have settings,
    // server 3 is present in the registry while server 4 does not meet any of
    // the conditions.
    cx.update(|cx| {
        let settings = ProjectSettings::get_global(cx);

        assert!(settings.context_servers.contains_key(SERVER_1_ID));
        assert!(settings.context_servers.contains_key(SERVER_2_ID));
        assert!(!settings.context_servers.contains_key(SERVER_3_ID));
        assert!(!settings.context_servers.contains_key(SERVER_4_ID));
    });

    registry.update(cx, |registry, _cx| {
        let descriptors = registry.context_server_descriptors();
        assert_eq!(descriptors.len(), 1);
        assert_eq!(descriptors[0].0.as_ref(), SERVER_3_ID);
    });

    let server_1_id = ContextServerId(SERVER_1_ID.into());
    let server_2_id = ContextServerId(SERVER_2_ID.into());
    let server_3_id = ContextServerId(SERVER_3_ID.into());
    let server_4_id = ContextServerId(SERVER_4_ID.into());

    store.read_with(cx, |store, cx| {
        assert!(store.is_server_enabled(&server_1_id, cx));
        assert!(!store.is_server_enabled(&server_2_id, cx));
        assert!(store.is_server_enabled(&server_3_id, cx));
        assert!(!store.is_server_enabled(&server_4_id, cx));
    })
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
            move |_, event, _| {
                let ServerStatusChangedEvent {
                    server_id: actual_server_id,
                    status: actual_status,
                } = event;
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
