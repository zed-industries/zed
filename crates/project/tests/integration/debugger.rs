mod go_locator {
    use collections::HashMap;
    use dap::{DapLocator, adapters::DebugAdapterName};
    use gpui::TestAppContext;
    use project::debugger::locators::go::{DelveLaunchRequest, GoLocator};
    use task::{HideStrategy, RevealStrategy, RevealTarget, SaveStrategy, Shell, TaskTemplate};
    #[gpui::test]
    async fn test_create_scenario_for_go_build(_: &mut TestAppContext) {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go build".into(),
            command: "go".into(),
            args: vec!["build".into(), ".".into()],
            env: Default::default(),
            cwd: Some("${ZED_WORKTREE_ROOT}".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            tags: vec![],
            show_summary: true,
            show_command: true,
            save: SaveStrategy::default(),
            hooks: Default::default(),
        };

        let scenario = locator
            .create_scenario(&task, "test label", &DebugAdapterName("Delve".into()))
            .await;

        assert!(scenario.is_none());
    }

    #[gpui::test]
    async fn test_skip_non_go_commands_with_non_delve_adapter(_: &mut TestAppContext) {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "cargo build".into(),
            command: "cargo".into(),
            args: vec!["build".into()],
            env: Default::default(),
            cwd: Some("${ZED_WORKTREE_ROOT}".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            tags: vec![],
            show_summary: true,
            show_command: true,
            save: SaveStrategy::default(),
            hooks: Default::default(),
        };

        let scenario = locator
            .create_scenario(
                &task,
                "test label",
                &DebugAdapterName("SomeOtherAdapter".into()),
            )
            .await;
        assert!(scenario.is_none());

        let scenario = locator
            .create_scenario(&task, "test label", &DebugAdapterName("Delve".into()))
            .await;
        assert!(scenario.is_none());
    }
    #[gpui::test]
    async fn test_go_locator_run(_: &mut TestAppContext) {
        let locator = GoLocator;
        let delve = DebugAdapterName("Delve".into());

        let task = TaskTemplate {
            label: "go run with flags".into(),
            command: "go".into(),
            args: vec![
                "run".to_string(),
                "-race".to_string(),
                "-ldflags".to_string(),
                "-X main.version=1.0".to_string(),
                "./cmd/myapp".to_string(),
                "--config".to_string(),
                "production.yaml".to_string(),
                "--verbose".to_string(),
            ],
            env: {
                let mut env = HashMap::default();
                env.insert("GO_ENV".to_string(), "production".to_string());
                env
            },
            cwd: Some("/project/root".into()),
            ..Default::default()
        };

        let scenario = locator
            .create_scenario(&task, "test run label", &delve)
            .await
            .unwrap();

        let config: DelveLaunchRequest = serde_json::from_value(scenario.config).unwrap();

        assert_eq!(
            config,
            DelveLaunchRequest {
                request: "launch".to_string(),
                mode: "debug".to_string(),
                program: "./cmd/myapp".to_string(),
                build_flags: vec![
                    "-race".to_string(),
                    "-ldflags".to_string(),
                    "-X main.version=1.0".to_string()
                ],
                args: vec![
                    "--config".to_string(),
                    "production.yaml".to_string(),
                    "--verbose".to_string(),
                ],
                env: {
                    let mut env = HashMap::default();
                    env.insert("GO_ENV".to_string(), "production".to_string());
                    env
                },
                cwd: Some("/project/root".to_string()),
            }
        );
    }

    #[gpui::test]
    async fn test_go_locator_test(_: &mut TestAppContext) {
        let locator = GoLocator;
        let delve = DebugAdapterName("Delve".into());

        // Test with tags and run flag
        let task_with_tags = TaskTemplate {
            label: "test".into(),
            command: "go".into(),
            args: vec![
                "test".to_string(),
                "-tags".to_string(),
                "integration,unit".to_string(),
                "-run".to_string(),
                "Foo".to_string(),
                ".".to_string(),
            ],
            ..Default::default()
        };
        let result = locator
            .create_scenario(&task_with_tags, "", &delve)
            .await
            .unwrap();

        let config: DelveLaunchRequest = serde_json::from_value(result.config).unwrap();

        assert_eq!(
            config,
            DelveLaunchRequest {
                request: "launch".to_string(),
                mode: "test".to_string(),
                program: ".".to_string(),
                build_flags: vec!["-tags".to_string(), "integration,unit".to_string(),],
                args: vec![
                    "-test.run".to_string(),
                    "Foo".to_string(),
                    "-test.v".to_string()
                ],
                env: Default::default(),
                cwd: None,
            }
        );
    }

    #[gpui::test]
    async fn test_go_locator_unescapes_nested_subtest_regex(_: &mut TestAppContext) {
        let locator = GoLocator;
        let delve = DebugAdapterName("Delve".into());

        // Delve receives the `-run` regex with no shell, so GoLocator must strip
        // the escaping itself.
        let task = TaskTemplate {
            label: "test subtest".into(),
            command: "go".into(),
            args: vec![
                "test".to_string(),
                "-v".to_string(),
                "-run".to_string(),
                "\\^TestFoo\\$/\\^simple_subtest\\$".to_string(),
            ],
            ..Default::default()
        };
        let result = locator.create_scenario(&task, "", &delve).await.unwrap();
        let config: DelveLaunchRequest = serde_json::from_value(result.config).unwrap();
        assert_eq!(
            config,
            DelveLaunchRequest {
                request: "launch".to_string(),
                mode: "test".to_string(),
                program: ".".to_string(),
                build_flags: vec![],
                args: vec![
                    "-test.v".to_string(),
                    "-test.run".to_string(),
                    "^TestFoo$/^simple_subtest$".to_string(),
                ],
                env: Default::default(),
                cwd: None,
            }
        );
    }

    #[gpui::test]
    async fn test_skip_unsupported_go_commands(_: &mut TestAppContext) {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go clean".into(),
            command: "go".into(),
            args: vec!["clean".into()],
            env: Default::default(),
            cwd: Some("${ZED_WORKTREE_ROOT}".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            tags: vec![],
            show_summary: true,
            show_command: true,
            save: SaveStrategy::default(),
            hooks: Default::default(),
        };

        let scenario = locator
            .create_scenario(&task, "test label", &DebugAdapterName("Delve".into()))
            .await;
        assert!(scenario.is_none());
    }
}

mod python_locator {
    use dap::{DapLocator, adapters::DebugAdapterName};
    use serde_json::json;

    use project::debugger::locators::python::*;
    use task::{DebugScenario, TaskTemplate};

    #[gpui::test]
    async fn test_python_locator() {
        let adapter = DebugAdapterName("Debugpy".into());
        let build_task = TaskTemplate {
            label: "run module '$ZED_FILE'".into(),
            command: "$ZED_CUSTOM_PYTHON_ACTIVE_ZED_TOOLCHAIN".into(),
            args: vec!["-m".into(), "$ZED_CUSTOM_PYTHON_MODULE_NAME".into()],
            env: Default::default(),
            cwd: Some("$ZED_WORKTREE_ROOT".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: task::RevealStrategy::Always,
            reveal_target: task::RevealTarget::Dock,
            hide: task::HideStrategy::Never,
            tags: vec!["python-module-main-method".into()],
            shell: task::Shell::System,
            show_summary: false,
            show_command: false,
            save: task::SaveStrategy::default(),
            hooks: Default::default(),
        };

        let expected_scenario = DebugScenario {
            adapter: "Debugpy".into(),
            label: "run module 'main.py'".into(),
            build: None,
            config: json!({
                "request": "launch",
                "python": "$ZED_CUSTOM_PYTHON_ACTIVE_ZED_TOOLCHAIN",
                "args": [],
                "cwd": "$ZED_WORKTREE_ROOT",
                "module": "$ZED_CUSTOM_PYTHON_MODULE_NAME",
            }),
            tcp_connection: None,
        };

        assert_eq!(
            PythonLocator
                .create_scenario(&build_task, "run module 'main.py'", &adapter)
                .await
                .expect("Failed to create a scenario"),
            expected_scenario
        );
    }
}

mod hover {
    use std::{path::Path, sync::Arc};

    use dap::{
        DapRegistry, EvaluateArgumentsContext, Variable,
        adapters::{DebugAdapterName, DebugTaskDefinition},
        client::DebugAdapterClient,
        requests::{Evaluate, Variables},
    };
    use fs::FakeFs;
    use futures::StreamExt as _;
    use gpui::{BackgroundExecutor, Entity, TestAppContext};
    use language::{Buffer, FakeLspAdapter, ToPointUtf16, rust_lang};
    use parking_lot::Mutex;
    use project::{
        Project,
        debugger::{
            breakpoint_store::ActiveStackFrame,
            session::{Session, SessionQuirks, ThreadId},
        },
    };
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use task::SharedTaskContext;
    use util::path;

    use crate::init_test;

    const SOURCE: &str = "fn main() {
    let value = 42;
    let x = value + 1;
    println!(\"{}\", x);
}
";

    async fn init_project(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) -> Entity<Project> {
        init_test(cx);
        cx.executor().allow_parking();
        cx.update(|cx| DapRegistry::global(cx).add_adapter(Arc::new(dap::FakeAdapter::new())));
        let fs = FakeFs::new(executor);
        fs.insert_tree(path!("/project"), json!({ "main.rs": SOURCE }))
            .await;
        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        project
            .read_with(cx, |project, _| project.languages().clone())
            .add(rust_lang());
        project
    }

    async fn boot_session<T: Fn(&Arc<DebugAdapterClient>) + 'static>(
        project: &Entity<Project>,
        configure: T,
        cx: &mut TestAppContext,
    ) -> Entity<Session> {
        let worktree = project.update(cx, |project, cx| {
            project
                .worktrees(cx)
                .next()
                .expect("test project should have one worktree")
        });
        let _subscription = project::debugger::test::intercept_debug_sessions(cx, configure);
        let dap_store = project.read_with(cx, |project, _| project.dap_store());
        let (session, boot_task) = dap_store.update(cx, |dap_store, cx| {
            let session = dap_store.new_session(
                None,
                DebugAdapterName(dap::FakeAdapter::ADAPTER_NAME.into()),
                SharedTaskContext::default(),
                None,
                SessionQuirks::default(),
                cx,
            );
            let boot_task = dap_store.boot_session(
                session.clone(),
                DebugTaskDefinition {
                    adapter: dap::FakeAdapter::ADAPTER_NAME.into(),
                    label: "test".into(),
                    config: json!({ "request": "launch" }),
                    tcp_connection: None,
                },
                worktree,
                cx,
            );
            (session, boot_task)
        });
        boot_task.await.expect("session should boot");
        cx.run_until_parked();
        session
    }

    fn set_active_stack_frame(
        project: &Entity<Project>,
        session: &Entity<Session>,
        buffer: &Entity<Buffer>,
        cx: &mut TestAppContext,
    ) {
        let session_id = session.read_with(cx, |session, _| session.session_id());
        let position = buffer.read_with(cx, |buffer, _| buffer.snapshot().anchor_before(0));
        project.update(cx, |project, cx| {
            project.breakpoint_store().update(cx, |store, cx| {
                store.set_active_position(
                    ActiveStackFrame {
                        session_id,
                        thread_id: ThreadId(1),
                        stack_frame_id: 1,
                        path: Arc::<Path>::from(Path::new(path!("/project/main.rs"))),
                        position,
                    },
                    cx,
                );
            });
        });
    }

    #[gpui::test]
    async fn test_hover_merges_debug_value_before_lsp_hover(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        let project = init_project(executor, cx).await;
        let evaluation_contexts = Arc::new(Mutex::new(Vec::new()));
        let languages = project.read_with(cx, |project, _| project.languages().clone());
        let mut fake_servers = languages.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        );
        let session = boot_session(
            &project,
            {
                let evaluation_contexts = evaluation_contexts.clone();
                move |client| {
                    let evaluation_contexts = evaluation_contexts.clone();
                    client.on_request::<Evaluate, _>(move |_, args| {
                        let context = args.context.clone();
                        evaluation_contexts
                            .lock()
                            .push((args.expression.clone(), context.clone()));

                        match context {
                            Some(EvaluateArgumentsContext::Hover) => Err(dap::ErrorResponse {
                                error: Some(dap::Message {
                                    id: 1,
                                    format: "hover unsupported".into(),
                                    variables: None,
                                    send_telemetry: None,
                                    show_user: None,
                                    url: None,
                                    url_label: None,
                                }),
                            }),
                            Some(EvaluateArgumentsContext::Variables) => {
                                Ok(dap::EvaluateResponse {
                                    result: "42".into(),
                                    type_: Some("i32".into()),
                                    presentation_hint: None,
                                    variables_reference: 0,
                                    named_variables: None,
                                    indexed_variables: None,
                                    memory_reference: None,
                                    value_location_reference: None,
                                })
                            }
                            other => panic!("unexpected evaluate context: {other:?}"),
                        }
                    });
                }
            },
            cx,
        )
        .await;

        let (buffer, _handle) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/project/main.rs"), cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();
        fake_servers
            .next()
            .await
            .expect("failed to get language server")
            .set_request_handler::<lsp::request::HoverRequest, _, _>(move |_, _| async move {
                Ok(Some(lsp::Hover {
                    contents: lsp::HoverContents::Scalar(lsp::MarkedString::String(
                        "lsp hover".to_string(),
                    )),
                    range: None,
                }))
            });

        set_active_stack_frame(&project, &session, &buffer, cx);
        let hover_offset = SOURCE.find("value + 1").unwrap();
        let hover_position = buffer.read_with(cx, |buffer, _| hover_offset.to_point_utf16(buffer));
        let hover = project
            .update(cx, |project, cx| project.hover(&buffer, hover_position, cx))
            .await
            .and_then(|mut hovers| hovers.pop())
            .expect("expected merged hover");
        let debugger_value = hover
            .debugger_value
            .expect("expected debugger hover payload");

        assert_eq!(
            hover
                .contents
                .into_iter()
                .map(|block| block.text)
                .collect::<Vec<_>>(),
            vec!["lsp hover".to_string()]
        );
        assert_eq!(debugger_value.root.name, "value");
        assert_eq!(debugger_value.root.value, "42");
        assert_eq!(debugger_value.root.type_name.as_deref(), Some("i32"));
        assert_eq!(debugger_value.root.variables_reference, 0);
        assert_eq!(
            *evaluation_contexts.lock(),
            vec![
                ("value".to_string(), Some(EvaluateArgumentsContext::Hover),),
                (
                    "value".to_string(),
                    Some(EvaluateArgumentsContext::Variables),
                ),
            ]
        );
    }

    #[gpui::test]
    async fn test_hover_children_are_loaded_on_demand_and_cached(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        let project = init_project(executor, cx).await;
        let variables_request_count = Arc::new(AtomicUsize::new(0));
        let session = boot_session(
            &project,
            {
                let variables_request_count = variables_request_count.clone();
                move |client| {
                    client.on_request::<Evaluate, _>(move |_, args| {
                        assert_eq!(args.expression, "value");
                        assert_eq!(args.context, Some(EvaluateArgumentsContext::Hover));
                        Ok(dap::EvaluateResponse {
                            result: "Point { x: 42 }".into(),
                            type_: Some("Point".into()),
                            presentation_hint: None,
                            variables_reference: 1,
                            named_variables: Some(1),
                            indexed_variables: None,
                            memory_reference: None,
                            value_location_reference: None,
                        })
                    });

                    let variables_request_count = variables_request_count.clone();
                    client.on_request::<Variables, _>(move |_, args| {
                        variables_request_count.fetch_add(1, Ordering::SeqCst);
                        assert_eq!(args.variables_reference, 1);
                        Ok(dap::VariablesResponse {
                            variables: vec![Variable {
                                name: "x".into(),
                                value: "42".into(),
                                type_: Some("i32".into()),
                                presentation_hint: None,
                                evaluate_name: None,
                                variables_reference: 0,
                                named_variables: None,
                                indexed_variables: None,
                                memory_reference: None,
                                declaration_location_reference: None,
                                value_location_reference: None,
                            }],
                        })
                    });
                }
            },
            cx,
        )
        .await;

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/main.rs"), cx)
            })
            .await
            .unwrap();
        buffer.update(cx, |buffer, cx| buffer.set_language(Some(rust_lang()), cx));

        set_active_stack_frame(&project, &session, &buffer, cx);
        let hover_offset = SOURCE.find("value + 1").unwrap();
        let hover_position = buffer.read_with(cx, |buffer, _| hover_offset.to_point_utf16(buffer));
        let debugger_value = project
            .update(cx, |project, cx| project.hover(&buffer, hover_position, cx))
            .await
            .and_then(|mut hovers| hovers.pop())
            .and_then(|hover| hover.debugger_value)
            .expect("expected debugger hover payload");

        let children = project
            .update(cx, |project, cx| {
                project.load_debugger_hover_children(
                    debugger_value.session_id,
                    debugger_value.root.variables_reference,
                    cx,
                )
            })
            .await
            .expect("expected first child load to succeed");
        assert_eq!(variables_request_count.load(Ordering::SeqCst), 1);
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "x");
        assert_eq!(children[0].value, "42");
        assert_eq!(children[0].type_name.as_deref(), Some("i32"));
        assert_eq!(children[0].variables_reference, 0);

        let cached_children = project
            .update(cx, |project, cx| {
                project.load_debugger_hover_children(
                    debugger_value.session_id,
                    debugger_value.root.variables_reference,
                    cx,
                )
            })
            .await
            .expect("expected cached child load to succeed");
        assert_eq!(variables_request_count.load(Ordering::SeqCst), 1);
        assert_eq!(cached_children, children);
    }
}

mod memory {
    use project::debugger::{
        MemoryCell,
        memory::{MemoryIterator, PageAddress, PageContents},
    };

    #[test]
    fn iterate_over_unmapped_memory() {
        let empty_iterator = MemoryIterator::new(0..=127, Default::default());
        let actual = empty_iterator.collect::<Vec<_>>();
        let expected = vec![MemoryCell(None); 128];
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }

    #[test]
    fn iterate_over_partially_mapped_memory() {
        let it = MemoryIterator::new(
            0..=127,
            vec![(PageAddress(5), PageContents::mapped(vec![1]))].into_iter(),
        );
        let actual = it.collect::<Vec<_>>();
        let expected = std::iter::repeat_n(MemoryCell(None), 5)
            .chain(std::iter::once(MemoryCell(Some(1))))
            .chain(std::iter::repeat_n(MemoryCell(None), 122))
            .collect::<Vec<_>>();
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }

    #[test]
    fn reads_from_the_middle_of_a_page() {
        let partial_iter = MemoryIterator::new(
            20..=30,
            vec![(PageAddress(0), PageContents::mapped((0..255).collect()))].into_iter(),
        );
        let actual = partial_iter.collect::<Vec<_>>();
        let expected = (20..=30)
            .map(|val| MemoryCell(Some(val)))
            .collect::<Vec<_>>();
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }
}
