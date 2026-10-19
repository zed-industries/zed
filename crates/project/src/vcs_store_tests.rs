use super::*;
use extension::*;
use gpui::{EntityId, TestAppContext, UpdateGlobal};
use language::{LanguageName, LanguageRegistry};
use lsp::LanguageServerName;
use serde_json::json;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
use task::{SpawnInTerminal, ZedDebugConfig};

struct FixtureExtension {
    manifest: Arc<ExtensionManifest>,
    directory: Arc<Path>,
    scenario: String,
    callbacks: AtomicUsize,
}

impl FixtureExtension {
    fn new(id: &str, directory: &Path, scenario: &str) -> Arc<Self> {
        Arc::new(Self {
            manifest: Arc::new(
                serde_json::from_value(json!({
                    "id": id, "name": id, "version": "0.1.0", "schema_version": 1,
                    "vcs_providers": {"fixture": {"root_markers": [".fixture-vcs"]}}
                }))
                .unwrap(),
            ),
            directory: Arc::from(directory),
            scenario: scenario.into(),
            callbacks: AtomicUsize::new(0),
        })
    }

    fn register(self: &Arc<Self>, cx: &mut TestAppContext) {
        cx.update(|cx| {
            ExtensionHostProxy::global(cx).register_vcs_provider(self.clone(), "fixture".into(), cx)
        });
    }
}

macro_rules! fixture_extension {
    ($(async fn $method:ident($($parameters:tt)*) -> $result:ty;)*) => {
        #[async_trait::async_trait]
        impl Extension for FixtureExtension {
            fn manifest(&self) -> Arc<ExtensionManifest> { self.manifest.clone() }
            fn work_dir(&self) -> Arc<Path> { self.directory.clone() }
            async fn vcs_provider_command(&self, _: Arc<str>, _: Arc<dyn WorktreeDelegate>) -> Result<Command> {
                self.callbacks.fetch_add(1, Ordering::SeqCst);
                let python = std::env::var_os("VCS_FIXTURE_PYTHON").map(PathBuf::from).unwrap_or_else(|| which::which("python3").expect("public fixture Python"));
                Ok(Command {
                    command: python,
                    args: vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../vcs/tests/fixture_provider.py").to_string_lossy().into_owned(),
                        "--scenario".into(), self.scenario.clone(), "--events".into(), self.directory.join(format!("{}.jsonl", self.manifest.id)).to_string_lossy().into_owned()],
                    env: Vec::new(),
                })
            }
            $(async fn $method($($parameters)*) -> $result { bail!("fixture implements only VCS") })*
        }
    };
}

fixture_extension! {
    async fn language_server_command(&self, _: LanguageServerName, _: LanguageName, _: Arc<dyn WorktreeDelegate>, _: EntityId) -> Result<Command>;
    async fn language_server_initialization_options(&self, _: LanguageServerName, _: LanguageName, _: Arc<dyn WorktreeDelegate>, _: EntityId) -> Result<Option<String>>;
    async fn language_server_workspace_configuration(&self, _: LanguageServerName, _: Arc<dyn WorktreeDelegate>, _: EntityId) -> Result<Option<String>>;
    async fn language_server_initialization_options_schema(&self, _: LanguageServerName, _: Arc<dyn WorktreeDelegate>, _: EntityId) -> Result<Option<String>>;
    async fn language_server_workspace_configuration_schema(&self, _: LanguageServerName, _: Arc<dyn WorktreeDelegate>, _: EntityId) -> Result<Option<String>>;
    async fn language_server_additional_initialization_options(&self, _: LanguageServerName, _: LanguageServerName, _: Arc<dyn WorktreeDelegate>, _: EntityId) -> Result<Option<String>>;
    async fn language_server_additional_workspace_configuration(&self, _: LanguageServerName, _: LanguageServerName, _: Arc<dyn WorktreeDelegate>, _: EntityId) -> Result<Option<String>>;
    async fn labels_for_completions(&self, _: LanguageServerName, _: Vec<Completion>) -> Result<Vec<Option<CodeLabel>>>;
    async fn labels_for_symbols(&self, _: LanguageServerName, _: Vec<Symbol>) -> Result<Vec<Option<CodeLabel>>>;
    async fn complete_slash_command_argument(&self, _: SlashCommand, _: Vec<String>) -> Result<Vec<SlashCommandArgumentCompletion>>;
    async fn run_slash_command(&self, _: SlashCommand, _: Vec<String>, _: Option<Arc<dyn WorktreeDelegate>>) -> Result<SlashCommandOutput>;
    async fn context_server_command(&self, _: Arc<str>, _: Arc<dyn ProjectDelegate>) -> Result<Command>;
    async fn context_server_configuration(&self, _: Arc<str>, _: Arc<dyn ProjectDelegate>) -> Result<Option<ContextServerConfiguration>>;
    async fn suggest_docs_packages(&self, _: Arc<str>) -> Result<Vec<String>>;
    async fn index_docs(&self, _: Arc<str>, _: Arc<str>, _: Arc<dyn KeyValueStoreDelegate>) -> Result<()>;
    async fn get_dap_binary(&self, _: Arc<str>, _: DebugTaskDefinition, _: Option<PathBuf>, _: Arc<dyn WorktreeDelegate>) -> Result<DebugAdapterBinary>;
    async fn dap_request_kind(&self, _: Arc<str>, _: serde_json::Value) -> Result<StartDebuggingRequestArgumentsRequest>;
    async fn dap_config_to_scenario(&self, _: ZedDebugConfig) -> Result<DebugScenario>;
    async fn dap_locator_create_scenario(&self, _: String, _: BuildTaskTemplate, _: String, _: String) -> Result<Option<DebugScenario>>;
    async fn run_dap_locator(&self, _: String, _: SpawnInTerminal) -> Result<DebugRequest>;
}

struct Fixture {
    directory: tempfile::TempDir,
    project: Entity<Project>,
    store: Entity<VcsStore>,
    fs: Arc<fs::FakeFs>,
    worktree_id: WorktreeId,
}

async fn fixture(cx: &mut TestAppContext, nested_git: bool) -> Fixture {
    cx.executor().allow_parking();
    cx.update(|cx| {
        let settings = settings::SettingsStore::test(cx);
        cx.set_global(settings);
        settings::SettingsStore::update_global(cx, |settings, cx| {
            settings.update_user_settings(cx, |content| {
                content.session.get_or_insert_default().trust_all_worktrees = Some(false);
            })
        });
        crate::trusted_worktrees::init(Default::default(), cx);
    });
    let directory = tempfile::tempdir().unwrap();
    let fs = fs::FakeFs::new(cx.background_executor.clone());
    let mut tree = json!({".fixture-vcs": "", "src": {"example.txt": "working text\n"}});
    if nested_git {
        tree["nested"] = json!({".git": {}, "file.txt": "Git content"});
    }
    fs.insert_tree(directory.path(), tree).await;
    let languages = Arc::new(LanguageRegistry::test(cx.executor()));
    let client = cx.update(|cx| {
        client::Client::new(
            Arc::new(clock::FakeSystemClock::new()),
            http_client::FakeHttpClient::with_404_response(),
            cx,
        )
    });
    let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
    let project = cx.update(|cx| {
        Project::local(
            client,
            node_runtime::NodeRuntime::unavailable(),
            user_store,
            languages,
            fs.clone(),
            Some(HashMap::default()),
            crate::LocalProjectFlags {
                init_worktree_trust: true,
                ..Default::default()
            },
            cx,
        )
    });
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(directory.path(), true, cx)
        })
        .await
        .unwrap();
    worktree
        .read_with(cx, |worktree, _| {
            worktree.as_local().unwrap().scan_complete()
        })
        .await;
    let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
    let store = project.read_with(cx, |project, _| project.vcs_store().unwrap().clone());
    Fixture {
        directory,
        project,
        store,
        fs,
        worktree_id,
    }
}

fn trust(fixture: &Fixture, cx: &mut TestAppContext) {
    trust_worktree(fixture, fixture.worktree_id, cx);
}

fn trust_worktree(fixture: &Fixture, worktree_id: WorktreeId, cx: &mut TestAppContext) {
    let worktree_store = fixture
        .project
        .read_with(cx, |project, _| project.worktree_store());
    cx.update(|cx| {
        TrustedWorktrees::try_get_global(cx)
            .unwrap()
            .update(cx, |trust, cx| {
                trust.trust(
                    &worktree_store,
                    collections::HashSet::from_iter([
                        crate::trusted_worktrees::PathTrust::Worktree(worktree_id),
                    ]),
                    cx,
                )
            });
    });
}

async fn open_worktree(fixture: &Fixture, path: &Path, cx: &mut TestAppContext) -> WorktreeId {
    let (worktree, _) = fixture
        .project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path, true, cx)
        })
        .await
        .expect("open additional worktree");
    worktree
        .read_with(cx, |worktree, _| {
            worktree.as_local().unwrap().scan_complete()
        })
        .await;
    worktree.read_with(cx, |worktree, _| worktree.id())
}

async fn ready(fixture: &Fixture, cx: &mut TestAppContext) -> VcsRepositorySnapshot {
    cx.condition(&fixture.store, |store, _| {
        store
            .repositories()
            .first()
            .is_some_and(|repository| repository.snapshot.is_some() && !repository.loading)
    })
    .await;
    fixture
        .store
        .read_with(cx, |store, _| store.repositories().remove(0))
}

async fn stop(fixture: &Fixture, cx: &mut TestAppContext) {
    let connections: Vec<_> = fixture.store.read_with(cx, |store, _| {
        store
            .sessions
            .values()
            .filter_map(|session| session.connection.clone())
            .collect()
    });
    for connection in connections {
        connection.shutdown().await.unwrap();
    }
    fixture.store.update(cx, |store, cx| store.stop_all(cx));
}

async fn provider_event(path: PathBuf, event: &'static str) {
    // The child writes this handshake outside GPUI's simulated clock.
    smol::unblock(move || {
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        loop {
            if std::fs::read_to_string(&path)
                .unwrap_or_default()
                .lines()
                .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
                .any(|value| value["event"] == event)
            {
                return;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "fixture handshake {event} timed out"
            );
            std::thread::sleep(Duration::from_millis(5));
        }
    })
    .await;
}

async fn wait_diff_base(diff: &Entity<BufferDiff>, expected: &str, cx: &TestAppContext) {
    let expected = expected.to_owned();
    let changed = diff.condition::<buffer_diff::BufferDiffEvent>(cx, move |diff, cx| {
        diff.base_text(cx).text() == expected
    });
    let timeout = cx.background_executor.timer(Duration::from_secs(3));
    futures::pin_mut!(changed, timeout);
    match futures::future::select(changed, timeout).await {
        futures::future::Either::Left(_) => {}
        futures::future::Either::Right(_) => panic!(
            "diff baseline did not update: {:?}",
            diff.read_with(cx, |diff, cx| diff.base_text(cx).text())
        ),
    }
}

#[gpui::test]
async fn trust_precedes_callback_and_spawn_and_read_only_rejects_git_init(cx: &mut TestAppContext) {
    let fixture = fixture(cx, false).await;
    let extension = FixtureExtension::new("trust-fixture", fixture.directory.path(), "normal");
    extension.register(cx);
    let immediate_initialization = fixture.project.read_with(cx, |project, cx| {
        project.git_init(Arc::from(fixture.directory.path()), "main".into(), cx)
    });
    assert!(immediate_initialization.await.is_err());
    cx.executor().advance_clock(Duration::from_secs(1));
    cx.run_until_parked();
    assert_eq!(extension.callbacks.load(Ordering::SeqCst), 0);
    assert!(
        !fixture
            .directory
            .path()
            .join("trust-fixture.jsonl")
            .exists()
    );
    trust(&fixture, cx);
    let repository = ready(&fixture, cx).await;
    assert_eq!(extension.callbacks.load(Ordering::SeqCst), 1);
    assert_eq!(repository.snapshot.unwrap().revision_id, "r42");
    let initialize_repository = fixture.project.read_with(cx, |project, cx| {
        project.git_init(Arc::from(fixture.directory.path()), "main".into(), cx)
    });
    assert!(initialize_repository.await.is_err());
    fixture.store.update(cx, |store, cx| store.refresh_all(cx));
    let during_refresh = fixture.project.read_with(cx, |project, cx| {
        project.git_init(Arc::from(fixture.directory.path()), "main".into(), cx)
    });
    assert!(during_refresh.await.is_err());
    assert!(
        fixture
            .fs
            .metadata(&fixture.directory.path().join(".git"))
            .await
            .unwrap()
            .is_none()
    );
    fixture.fs.insert_tree("/ordinary", json!({})).await;
    let ordinary = fixture.project.read_with(cx, |project, cx| {
        project.git_init(Arc::from(Path::new("/ordinary")), "main".into(), cx)
    });
    ordinary.await.unwrap();
    assert!(
        fixture
            .fs
            .metadata(Path::new("/ordinary/.git"))
            .await
            .unwrap()
            .is_some()
    );
    stop(&fixture, cx).await;
    let events =
        std::fs::read_to_string(fixture.directory.path().join("trust-fixture.jsonl")).unwrap();
    assert!(events.contains("shutdown-received"));
    assert!(!events.contains("vcs/commit"));
}

#[gpui::test]
async fn external_repository_preserves_nested_git_ownership_and_refuses_ambiguity(
    cx: &mut TestAppContext,
) {
    let fixture = fixture(cx, true).await;
    trust(&fixture, cx);
    FixtureExtension::new("first", fixture.directory.path(), "normal").register(cx);
    let repository = ready(&fixture, cx).await;
    fixture
        .project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    fixture.store.read_with(cx, |store, cx| {
        let external = ProjectPath {
            worktree_id: fixture.worktree_id,
            path: Arc::from(RelPath::from_unix_str("src/example.txt").unwrap()),
        };
        let nested = ProjectPath {
            worktree_id: fixture.worktree_id,
            path: Arc::from(RelPath::from_unix_str("nested/file.txt").unwrap()),
        };
        assert_eq!(
            store.repository_for_path(&external, cx),
            Some(repository.key.clone())
        );
        assert!(store.repository_for_path(&nested, cx).is_none());
        assert!(
            store
                .git
                .read(cx)
                .repository_and_path_for_project_path(&nested, cx)
                .is_some()
        );
    });
    FixtureExtension::new("second", fixture.directory.path(), "normal").register(cx);
    cx.condition(&fixture.store, |store, _| {
        store.repositories().len() == 2
            && store
                .repositories()
                .iter()
                .all(|repository| repository.error.is_some())
    })
    .await;
    fixture.store.read_with(cx, |store, cx| {
        let path = ProjectPath {
            worktree_id: fixture.worktree_id,
            path: Arc::from(RelPath::from_unix_str("src/example.txt").unwrap()),
        };
        assert!(store.repository_for_path(&path, cx).is_none());
        assert!(store.changes(&repository.key, cx).is_empty());
    });
    stop(&fixture, cx).await;
}

#[gpui::test]
async fn linked_git_worktree_keeps_git_ownership_with_external_provider_registered(
    cx: &mut TestAppContext,
) {
    let fixture = fixture(cx, false).await;
    let main = tempfile::tempdir().expect("main Git repository directory");
    let linked = tempfile::tempdir().expect("linked Git checkout directory");
    let git_directory = main.path().join(".git/worktrees/feature");
    fixture
        .fs
        .insert_tree(
            main.path(),
            json!({
                ".git": {
                    "HEAD": "ref: refs/heads/main\n",
                    "worktrees": {"feature": {
                        "HEAD": "ref: refs/heads/feature\n",
                        "commondir": "../..\n",
                        "gitdir": format!("{}\n", linked.path().join(".git").display())
                    }}
                }
            }),
        )
        .await;
    fixture
        .fs
        .insert_tree(
            linked.path(),
            json!({
                ".fixture-vcs": "",
                ".git": format!("gitdir: {}\n", git_directory.display()),
                "src": {"example.txt": "linked Git working content\n"}
            }),
        )
        .await;
    assert_eq!(
        crate::git_store::resolve_git_worktree_to_main_repo(fixture.fs.as_ref(), linked.path())
            .await,
        Some(main.path().to_path_buf())
    );
    let linked_id = open_worktree(&fixture, linked.path(), cx).await;
    fixture
        .project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    trust(&fixture, cx);
    trust_worktree(&fixture, linked_id, cx);
    let extension = FixtureExtension::new("linked-arbitration", fixture.directory.path(), "normal");
    extension.register(cx);
    let external = ready(&fixture, cx).await;
    fixture.store.read_with(cx, |store, cx| {
        let linked_path = ProjectPath {
            worktree_id: linked_id,
            path: Arc::from(RelPath::from_unix_str("src/example.txt").unwrap()),
        };
        assert!(
            store
                .git
                .read(cx)
                .repository_and_path_for_project_path(&linked_path, cx)
                .is_some()
        );
        assert!(store.repository_for_path(&linked_path, cx).is_none());
        assert_eq!(store.repositories().len(), 1);
        assert_eq!(external.key.worktree_id, fixture.worktree_id);
        assert!(!store.sessions.keys().any(|key| key.0 == linked_id));
    });
    assert_eq!(extension.callbacks.load(Ordering::SeqCst), 1);
    assert!(
        !fixture
            .fs
            .metadata(&linked.path().join(".git"))
            .await
            .unwrap()
            .unwrap()
            .is_dir
    );
    stop(&fixture, cx).await;
}

#[gpui::test]
async fn multiple_external_scopes_keep_baselines_and_lifetimes_independent(
    cx: &mut TestAppContext,
) {
    let fixture = fixture(cx, false).await;
    let second = tempfile::tempdir().expect("second external scope");
    fixture
        .fs
        .insert_tree(
            second.path(),
            json!({
                ".fixture-vcs": "", "src": {"example.txt": "second working content\n"}
            }),
        )
        .await;
    let second_id = open_worktree(&fixture, second.path(), cx).await;
    trust(&fixture, cx);
    trust_worktree(&fixture, second_id, cx);
    let extension = FixtureExtension::new("multiple-scopes", fixture.directory.path(), "normal");
    extension.register(cx);
    cx.condition(&fixture.store, |store, _| {
        let repositories = store.repositories();
        repositories.len() == 2
            && repositories
                .iter()
                .all(|repository| repository.snapshot.is_some() && !repository.loading)
    })
    .await;
    let repositories = fixture.store.read_with(cx, |store, _| store.repositories());
    let first = repositories
        .iter()
        .find(|repository| repository.key.worktree_id == fixture.worktree_id)
        .expect("first scope")
        .clone();
    let second_repository = repositories
        .iter()
        .find(|repository| repository.key.worktree_id == second_id)
        .expect("second scope")
        .clone();
    assert_eq!(first.descriptor.id, second_repository.descriptor.id);
    assert_ne!(first.key, second_repository.key);
    assert_ne!(
        first.descriptor.scope_uri,
        second_repository.descriptor.scope_uri
    );
    let (first_connection, second_connection) = fixture.store.read_with(cx, |store, cx| {
        for repository in [&first, &second_repository] {
            let path = ProjectPath {
                worktree_id: repository.key.worktree_id,
                path: Arc::from(RelPath::from_unix_str("src/example.txt").unwrap()),
            };
            assert_eq!(
                store.repository_for_path(&path, cx),
                Some(repository.key.clone())
            );
        }
        (
            store.repositories[&first.key].connection.clone(),
            store.repositories[&second_repository.key]
                .connection
                .clone(),
        )
    });
    assert!(!Arc::ptr_eq(&first_connection, &second_connection));
    let first_diff = fixture
        .store
        .update(cx, |store, cx| {
            store.open_diff(first.key.clone(), "src/example.txt".into(), cx)
        })
        .await
        .unwrap();
    let second_diff = fixture
        .store
        .update(cx, |store, cx| {
            store.open_diff(second_repository.key.clone(), "src/example.txt".into(), cx)
        })
        .await
        .unwrap();
    fixture
        .store
        .update(cx, |store, cx| {
            store.set_baseline(&first.key, "previous".into(), cx)
        })
        .unwrap();
    wait_diff_base(&first_diff.diff, "previous — r41\n", cx).await;
    assert_eq!(
        second_diff
            .diff
            .read_with(cx, |diff, cx| diff.base_text(cx).text()),
        "baseline — r42\n"
    );
    assert_eq!(
        second_diff.buffer.read_with(cx, |buffer, _| buffer.text()),
        "second working content\n"
    );
    fixture.store.read_with(cx, |store, _| {
        assert_eq!(
            store.repositories[&first.key].view.selected_baseline,
            "previous"
        );
        assert_eq!(
            store.repositories[&second_repository.key]
                .view
                .selected_baseline,
            "submitted"
        );
    });
    fixture.project.update(cx, |project, cx| {
        project.remove_worktree(fixture.worktree_id, cx)
    });
    cx.condition(&fixture.store, |store, _| {
        let repositories = store.repositories();
        repositories.len() == 1
            && repositories
                .first()
                .is_some_and(|repository| repository.key == second_repository.key)
    })
    .await;
    first_connection.shutdown().await.unwrap();
    fixture.store.read_with(cx, |store, _| {
        let remaining = &store.repositories[&second_repository.key];
        assert!(Arc::ptr_eq(&remaining.connection, &second_connection));
        assert!(remaining.connection.is_alive());
        assert_eq!(remaining.view.selected_baseline, "submitted");
    });
    assert_eq!(extension.callbacks.load(Ordering::SeqCst), 2);
    stop(&fixture, cx).await;
}

#[gpui::test]
async fn baseline_change_updates_existing_live_diff_and_close_reopen_uses_new_session(
    cx: &mut TestAppContext,
) {
    let fixture = fixture(cx, false).await;
    trust(&fixture, cx);
    let extension = FixtureExtension::new("reopen", fixture.directory.path(), "normal");
    extension.register(cx);
    let repository = ready(&fixture, cx).await;
    let opened = fixture
        .store
        .update(cx, |store, cx| {
            store.open_diff(repository.key.clone(), "src/example.txt".into(), cx)
        })
        .await
        .unwrap();
    opened.buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "unsaved ")], None, cx);
    });
    let before = opened.buffer.read_with(cx, |buffer, _| buffer.text());
    fixture
        .store
        .update(cx, |store, cx| {
            store.set_baseline(&repository.key, "previous".into(), cx)
        })
        .unwrap();
    ready(&fixture, cx).await;
    wait_diff_base(&opened.diff, "previous — r41\n", cx).await;
    assert_eq!(
        opened.buffer.read_with(cx, |buffer, _| buffer.text()),
        before
    );
    let old_connection = fixture.store.read_with(cx, |store, _| {
        store
            .repositories
            .get(&repository.key)
            .unwrap()
            .connection
            .clone()
    });
    fixture.project.update(cx, |project, cx| {
        project.remove_worktree(fixture.worktree_id, cx)
    });
    cx.condition(&fixture.store, |store, _| store.repositories().is_empty())
        .await;
    provider_event(
        fixture.directory.path().join("reopen.jsonl"),
        "exit-received",
    )
    .await;
    assert!(!old_connection.is_alive());
    let (worktree, _) = fixture
        .project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(fixture.directory.path(), true, cx)
        })
        .await
        .unwrap();
    worktree
        .read_with(cx, |worktree, _| {
            worktree.as_local().unwrap().scan_complete()
        })
        .await;
    let worktree_store = fixture
        .project
        .read_with(cx, |project, _| project.worktree_store());
    cx.update(|cx| {
        TrustedWorktrees::try_get_global(cx)
            .unwrap()
            .update(cx, |trust, cx| {
                trust.trust(
                    &worktree_store,
                    collections::HashSet::from_iter([
                        crate::trusted_worktrees::PathTrust::Worktree(worktree.read(cx).id()),
                    ]),
                    cx,
                )
            })
    });
    let reopened = ready(&fixture, cx).await;
    assert_ne!(reopened.key.worktree_id, repository.key.worktree_id);
    assert_eq!(reopened.selected_baseline, "submitted");
    assert_eq!(extension.callbacks.load(Ordering::SeqCst), 2);
    stop(&fixture, cx).await;
}

#[gpui::test]
async fn refresh_preserves_baseline_for_healthy_and_reconnected_provider(cx: &mut TestAppContext) {
    let fixture = fixture(cx, false).await;
    trust(&fixture, cx);
    let extension = FixtureExtension::new("refresh", fixture.directory.path(), "normal");
    extension.register(cx);
    let repository = ready(&fixture, cx).await;
    fixture
        .store
        .update(cx, |store, cx| {
            store.set_baseline(&repository.key, "previous".into(), cx)
        })
        .unwrap();
    ready(&fixture, cx).await;
    let opened = fixture
        .store
        .update(cx, |store, cx| {
            store.open_diff(repository.key.clone(), "src/example.txt".into(), cx)
        })
        .await
        .unwrap();
    fixture.store.update(cx, |store, cx| store.refresh_all(cx));
    let refreshed = ready(&fixture, cx).await;
    assert_eq!(refreshed.selected_baseline, "previous");
    assert_eq!(extension.callbacks.load(Ordering::SeqCst), 1);
    wait_diff_base(&opened.diff, "previous — r41\n", cx).await;
    let connection = fixture.store.read_with(cx, |store, _| {
        store
            .repositories
            .get(&repository.key)
            .unwrap()
            .connection
            .clone()
    });
    connection.shutdown().await.unwrap();
    cx.condition(&fixture.store, |store, _| {
        store
            .repositories()
            .first()
            .is_some_and(|repository| repository.error.is_some())
    })
    .await;
    fixture.store.update(cx, |store, cx| store.refresh_all(cx));
    let reconnected = ready(&fixture, cx).await;
    assert_eq!(reconnected.selected_baseline, "previous");
    assert_eq!(extension.callbacks.load(Ordering::SeqCst), 2);
    wait_diff_base(&opened.diff, "previous — r41\n", cx).await;
    stop(&fixture, cx).await;
}

#[gpui::test]
async fn late_snapshot_cannot_replace_new_baseline(cx: &mut TestAppContext) {
    let fixture = fixture(cx, false).await;
    trust(&fixture, cx);
    let extension = FixtureExtension::new("reordered", fixture.directory.path(), "reordered");
    extension.register(cx);
    provider_event(
        fixture.directory.path().join("reordered.jsonl"),
        "snapshot-held",
    )
    .await;
    let repository = fixture
        .store
        .read_with(cx, |store, _| store.repositories().remove(0));
    assert!(repository.loading);
    fixture
        .store
        .update(cx, |store, cx| {
            store.set_baseline(&repository.key, "previous".into(), cx)
        })
        .unwrap();
    let current = ready(&fixture, cx).await;
    assert_eq!(current.snapshot.as_ref().unwrap().revision_id, "r41");
    let opened = fixture
        .store
        .update(cx, |store, cx| {
            store.open_diff(repository.key.clone(), "src/example.txt".into(), cx)
        })
        .await
        .unwrap();
    assert_eq!(
        opened
            .diff
            .read_with(cx, |diff, cx| diff.base_text(cx).text()),
        "previous — r41\n"
    );
    fixture.store.read_with(cx, |store, _| {
        let current = &store.repositories.get(&repository.key).unwrap().view;
        assert_eq!(current.selected_baseline, "previous");
        assert_eq!(current.snapshot.as_ref().unwrap().id, "snapshot:previous");
    });
    stop(&fixture, cx).await;
}

#[gpui::test]
async fn periodic_refresh_reuses_session_and_skips_in_flight_snapshot(cx: &mut TestAppContext) {
    let fixture = fixture(cx, false).await;
    trust(&fixture, cx);
    let extension = FixtureExtension::new("polling", fixture.directory.path(), "normal");
    extension.register(cx);
    let repository = ready(&fixture, cx).await;
    let initial_generation = fixture.store.read_with(cx, |store, _| {
        store.repositories.get(&repository.key).unwrap().generation
    });
    cx.executor().advance_clock(Duration::from_secs(5));
    cx.condition(&fixture.store, |store, _| {
        store
            .repositories
            .get(&repository.key)
            .is_some_and(|repository| {
                repository.generation > initial_generation && !repository.view.loading
            })
    })
    .await;
    assert_eq!(extension.callbacks.load(Ordering::SeqCst), 1);
    let generation = fixture.store.update(cx, |store, _| {
        let repository = store.repositories.get_mut(&repository.key).unwrap();
        repository.view.loading = true;
        repository.generation
    });
    cx.executor().advance_clock(Duration::from_secs(5));
    cx.run_until_parked();
    assert_eq!(
        fixture.store.read_with(cx, |store, _| store
            .repositories
            .get(&repository.key)
            .unwrap()
            .generation),
        generation
    );
    stop(&fixture, cx).await;
    fixture
        .store
        .read_with(cx, |store, _| assert!(store.poll_task.is_none()));
}

#[gpui::test]
async fn unchanged_poll_reuses_baseline_but_new_revision_reloads_it(cx: &mut TestAppContext) {
    let fixture = fixture(cx, false).await;
    trust(&fixture, cx);
    let extension =
        FixtureExtension::new("content-cache", fixture.directory.path(), "polling-content");
    extension.register(cx);
    let repository = ready(&fixture, cx).await;
    let opened = fixture
        .store
        .update(cx, |store, cx| {
            store.open_diff(repository.key.clone(), "src/example.txt".into(), cx)
        })
        .await
        .unwrap();
    let events = fixture.directory.path().join("content-cache.jsonl");
    let content_reads = || {
        std::fs::read_to_string(&events)
            .unwrap()
            .lines()
            .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
            .filter(|event| event["method"] == "vcs/readContent")
            .count()
    };
    assert_eq!(content_reads(), 1);
    let mut snapshot_id = repository.snapshot.unwrap().id.clone();
    for _ in 0..2 {
        cx.executor().advance_clock(Duration::from_secs(5));
        cx.condition(&fixture.store, |store, _| {
            store
                .repositories
                .get(&repository.key)
                .is_some_and(|repository| {
                    !repository.view.loading
                        && repository
                            .view
                            .snapshot
                            .as_ref()
                            .is_some_and(|snapshot| snapshot.id != snapshot_id)
                })
        })
        .await;
        snapshot_id = fixture.store.read_with(cx, |store, _| {
            store
                .repositories
                .get(&repository.key)
                .unwrap()
                .view
                .snapshot
                .as_ref()
                .unwrap()
                .id
                .clone()
        });
        let reopened = fixture
            .store
            .update(cx, |store, cx| {
                store.open_buffer_diff(opened.buffer.clone(), cx)
            })
            .await
            .unwrap();
        assert_eq!(reopened, opened.diff);
        assert_eq!(
            content_reads(),
            1,
            "unchanged polls and reopening a buffer must reuse its baseline"
        );
    }
    std::fs::write(events.with_extension("jsonl.revision"), "r43").unwrap();
    cx.executor().advance_clock(Duration::from_secs(5));
    cx.condition(&fixture.store, |store, _| {
        store
            .repositories
            .get(&repository.key)
            .is_some_and(|repository| {
                !repository.view.loading
                    && repository
                        .view
                        .snapshot
                        .as_ref()
                        .is_some_and(|snapshot| snapshot.revision_id == "r43")
            })
    })
    .await;
    wait_diff_base(&opened.diff, "baseline — r43\n", cx).await;
    assert_eq!(
        content_reads(),
        2,
        "changed baseline revision must be loaded"
    );
    stop(&fixture, cx).await;
}

#[gpui::test]
async fn concurrent_diff_readers_share_content_and_keep_pending_edits(cx: &mut TestAppContext) {
    let fixture = fixture(cx, false).await;
    trust(&fixture, cx);
    FixtureExtension::new("coalesced", fixture.directory.path(), "normal").register(cx);
    ready(&fixture, cx).await;
    let buffer = fixture
        .project
        .update(cx, |project, cx| {
            project.open_local_buffer(fixture.directory.path().join("src/example.txt"), cx)
        })
        .await
        .unwrap();
    let (first, second) = fixture.store.update(cx, |store, cx| {
        (
            store.open_buffer_diff(buffer.clone(), cx),
            store.open_buffer_diff(buffer.clone(), cx),
        )
    });
    buffer.update(cx, |buffer, cx| {
        buffer.edit(
            [(0..buffer.len(), "baseline — r42\nunsaved second line\n")],
            None,
            cx,
        );
    });
    let (first, second) = futures::future::join(first, second).await;
    let first = first.unwrap();
    assert_eq!(first, second.unwrap());
    first.read_with(cx, |diff, cx| {
        assert_eq!(diff.base_text(cx).text(), "baseline — r42\n");
        assert_eq!(
            diff.snapshot(cx).buffer_snapshot().text(),
            "baseline — r42\nunsaved second line\n"
        );
        assert_eq!(diff.changed_row_counts(), (1, 0));
    });
    let events = std::fs::read_to_string(fixture.directory.path().join("coalesced.jsonl")).unwrap();
    assert_eq!(
        events
            .lines()
            .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
            .filter(|event| event["method"] == "vcs/readContent")
            .count(),
        1
    );
    assert_eq!(
        fixture
            .fs
            .load(&fixture.directory.path().join("src/example.txt"))
            .await
            .unwrap(),
        "working text\n"
    );
    stop(&fixture, cx).await;
}

#[gpui::test]
async fn superseded_diff_readers_wait_for_the_new_baseline(cx: &mut TestAppContext) {
    let fixture = fixture(cx, false).await;
    trust(&fixture, cx);
    FixtureExtension::new("superseded-content", fixture.directory.path(), "normal").register(cx);
    let repository = ready(&fixture, cx).await;
    let buffer = fixture
        .project
        .update(cx, |project, cx| {
            project.open_local_buffer(fixture.directory.path().join("src/example.txt"), cx)
        })
        .await
        .unwrap();
    let (first, second, latest) = fixture.store.update(cx, |store, cx| {
        let first = store.open_buffer_diff(buffer.clone(), cx);
        let second = store.open_buffer_diff(buffer.clone(), cx);
        store
            .set_baseline(&repository.key, "previous".into(), cx)
            .unwrap();
        let latest = store.open_buffer_diff(buffer.clone(), cx);
        (first, second, latest)
    });
    let (first, second, latest) = futures::join!(first, second, latest);
    let first = first.unwrap();
    assert_eq!(first, second.unwrap());
    assert_eq!(first, latest.unwrap());
    assert_eq!(
        first.read_with(cx, |diff, cx| diff.base_text(cx).text()),
        "previous — r41\n"
    );
    stop(&fixture, cx).await;
}

#[gpui::test]
async fn coalesced_diff_readers_receive_real_provider_errors(cx: &mut TestAppContext) {
    let fixture = fixture(cx, false).await;
    trust(&fixture, cx);
    FixtureExtension::new("content-error", fixture.directory.path(), "polling-content")
        .register(cx);
    let repository = ready(&fixture, cx).await;
    let buffer = fixture
        .project
        .update(cx, |project, cx| {
            project.open_local_buffer(fixture.directory.path().join("src/example.txt"), cx)
        })
        .await
        .unwrap();
    let (first, second) = fixture.store.update(cx, |store, cx| {
        let repository = store.repositories.get_mut(&repository.key).unwrap();
        Arc::make_mut(repository.view.snapshot.as_mut().unwrap()).id = "expired".into();
        (
            store.open_buffer_diff(buffer.clone(), cx),
            store.open_buffer_diff(buffer.clone(), cx),
        )
    });
    let (first, second) = futures::future::join(first, second).await;
    for result in [first, second] {
        assert!(format!("{:#}", result.unwrap_err()).contains("Unknown fixture snapshot"));
    }
    stop(&fixture, cx).await;
}

#[gpui::test]
async fn changing_snapshots_do_not_accumulate_internal_diff_waiters(cx: &mut TestAppContext) {
    let fixture = fixture(cx, false).await;
    trust(&fixture, cx);
    FixtureExtension::new(
        "slow-content",
        fixture.directory.path(),
        "changing-slow-content",
    )
    .register(cx);
    let repository = ready(&fixture, cx).await;
    let buffer = fixture
        .project
        .update(cx, |project, cx| {
            project.open_local_buffer(fixture.directory.path().join("src/example.txt"), cx)
        })
        .await
        .unwrap();
    let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
    let first = fixture
        .store
        .update(cx, |store, cx| store.open_buffer_diff(buffer.clone(), cx));
    provider_event(
        fixture.directory.path().join("slow-content.jsonl"),
        "content-held",
    )
    .await;
    for _ in 0..4 {
        fixture
            .store
            .update(cx, |store, cx| store.refresh(&repository.key, cx));
        ready(&fixture, cx).await;
        fixture.store.read_with(cx, |store, _| {
            assert_eq!(store.diffs.get(&buffer_id).unwrap().waiters.len(), 1);
        });
    }
    drop(first);
    cx.run_until_parked();
    let second = fixture
        .store
        .update(cx, |store, cx| store.open_buffer_diff(buffer, cx));
    fixture.store.read_with(cx, |store, _| {
        assert_eq!(
            store.diffs.get(&buffer_id).unwrap().waiters.len(),
            1,
            "dropped readers must be pruned"
        );
    });
    drop(second);
    stop(&fixture, cx).await;
}

#[gpui::test]
async fn closing_buffer_before_snapshot_completes_resolves_diff_waiter(cx: &mut TestAppContext) {
    let fixture = fixture(cx, false).await;
    trust(&fixture, cx);
    FixtureExtension::new("closed-buffer", fixture.directory.path(), "normal").register(cx);
    let repository = ready(&fixture, cx).await;
    let buffer = cx.new(|cx| Buffer::local("working text\n", cx));
    let weak_buffer = buffer.downgrade();
    let waiting = fixture.store.update(cx, |store, cx| {
        let current = store.repositories.get_mut(&repository.key).unwrap();
        current.view.snapshot = None;
        current.view.loading = true;
        store.ensure_diff(&repository.key, "src/example.txt", &buffer, cx);
        store.load_diff_if_needed(buffer.clone(), cx)
    });
    drop(buffer);
    cx.run_until_parked();
    assert!(weak_buffer.upgrade().is_none());
    fixture
        .store
        .update(cx, |store, cx| store.refresh(&repository.key, cx));
    assert!(format!("{:#}", waiting.await.unwrap_err()).contains("VCS diff buffer closed"));
    stop(&fixture, cx).await;
}
