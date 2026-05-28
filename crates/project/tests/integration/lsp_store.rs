use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use fs::FakeFs;
use futures::{FutureExt, StreamExt, lock::OwnedMutexGuard};
use gpui::{AsyncApp, TestAppContext, UpdateGlobal};
use language::{
    BinaryDownloadsDisabled, BinaryStatus, CodeLabel, DynLspInstaller, FakeLspAdapter, HighlightId,
    LanguageName, LanguageServerBinaryLocations, LspAdapter, LspAdapterDelegate, Toolchain,
    rust_lang,
};
use lsp::{LanguageServerBinary, LanguageServerBinaryOptions, LanguageServerName, Uri};
use project::{Project, lsp_store::*};
use serde_json::json;
use settings::{LocalSettingsKind, LocalSettingsPath, Settings, SettingsStore};
use util::{path, rel_path::RelPath};

use crate::init_test;

#[derive(Clone, Default)]
struct DownloadOnlyLspAdapter {
    fetch_count: Arc<AtomicUsize>,
}

#[async_trait::async_trait(?Send)]
impl DynLspInstaller for DownloadOnlyLspAdapter {
    async fn try_fetch_server_binary(
        &self,
        _: &Arc<dyn LspAdapterDelegate>,
        _: PathBuf,
        _: bool,
        _: &mut AsyncApp,
    ) -> anyhow::Result<LanguageServerBinary> {
        unreachable!()
    }

    fn get_language_server_command(
        self: Arc<Self>,
        delegate: Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        binary_options: LanguageServerBinaryOptions,
        _: OwnedMutexGuard<Option<(bool, LanguageServerBinary)>>,
        _: AsyncApp,
    ) -> LanguageServerBinaryLocations {
        async move {
            if !binary_options.allow_binary_downloads {
                let reason =
                    BinaryDownloadsDisabled::new(format!("language server {}", self.name().0));
                delegate.update_status(
                    self.name(),
                    BinaryStatus::Disabled {
                        reason: reason.to_string(),
                    },
                );
                return (Err(reason.into()), None);
            }

            self.fetch_count.fetch_add(1, Ordering::SeqCst);
            (
                Ok(LanguageServerBinary {
                    path: "/downloaded/lsp".into(),
                    arguments: Vec::new(),
                    env: None,
                }),
                None,
            )
        }
        .boxed_local()
    }
}

impl LspAdapter for DownloadOnlyLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName::new_static("download-only-language-server")
    }

    fn language_ids(&self) -> collections::HashMap<LanguageName, String> {
        collections::HashMap::from_iter([("Rust".into(), "rust".to_string())])
    }
}

#[gpui::test]
async fn test_allow_binary_downloads_false_holds_lsp_until_allowed(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    cx.update(|cx| project::binary_downloads::init(cx));

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(false);
            });
        });
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let adapter = DownloadOnlyLspAdapter::default();
    let adapter_name = adapter.name();
    language_registry.register_lsp_adapter("Rust".into(), Arc::new(adapter));
    let mut fake_servers = language_registry.register_fake_lsp_server(
        adapter_name.clone(),
        lsp::ServerCapabilities::default(),
        None,
    );

    let (_buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_secs(1)).fuse();
    futures::select! {
        _ = next_server => panic!("language server started while downloads were disabled"),
        _ = timeout => {}
    }

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(true);
            });
        });
    });

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_secs(1)).fuse();
    futures::select! {
        server = next_server => assert_eq!(server.is_some(), true),
        _ = timeout => panic!("timed out waiting for language server after enabling downloads"),
    }
}

#[gpui::test]
async fn test_allow_binary_downloads_can_be_enabled_for_a_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    cx.update(|cx| project::binary_downloads::init(cx));

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(false);
            });
        });
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let worktree_id = project.update(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    cx.update_global::<SettingsStore, _>(|store, cx| {
        store
            .set_local_settings(
                worktree_id,
                LocalSettingsPath::InWorktree(Arc::from(RelPath::empty())),
                LocalSettingsKind::Settings,
                Some(r#"{ "allow_binary_downloads": true }"#),
                cx,
            )
            .unwrap();
    });
    project.read_with(cx, |_, cx| {
        assert_eq!(
            project::project_settings::ProjectSettings::get(
                Some(settings::SettingsLocation {
                    worktree_id,
                    path: RelPath::empty(),
                }),
                cx,
            )
            .allow_binary_downloads,
            true,
        );
    });

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let adapter = DownloadOnlyLspAdapter::default();
    let adapter_name = adapter.name();
    language_registry.register_lsp_adapter("Rust".into(), Arc::new(adapter));
    let mut fake_servers = language_registry.register_fake_lsp_server(
        adapter_name,
        lsp::ServerCapabilities::default(),
        None,
    );

    let (_buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_secs(1)).fuse();
    futures::select! {
        server = next_server => assert_eq!(server.is_some(), true),
        _ = timeout => panic!("timed out waiting for language server"),
    }
}

#[gpui::test]
async fn test_removing_invisible_worktree_cleans_reused_lsp_bookkeeping(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;
    fs.insert_tree(
        path!("/the-registry"),
        json!({ "dep": { "src": { "dep.rs": "pub fn dep() {}" } } }),
    )
    .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp("Rust", FakeLspAdapter::default());

    let (_visible_buffer, _visible_handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();
    fake_servers.next().await.unwrap();
    cx.run_until_parked();

    let server_id = project.read_with(cx, |project, cx| {
        project
            .lsp_store()
            .read(cx)
            .language_server_statuses()
            .next()
            .unwrap()
            .0
    });
    let external_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_via_lsp(
                Uri::from_file_path(path!("/the-registry/dep/src/dep.rs")).unwrap(),
                server_id,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let invisible_worktree_id =
        external_buffer.read_with(cx, |buffer, cx| buffer.file().unwrap().worktree_id(cx));
    project.read_with(cx, |project, cx| {
        let worktree = project.worktree_for_id(invisible_worktree_id, cx).unwrap();
        assert!(!worktree.read(cx).is_visible());
        assert!(
            project
                .lsp_store()
                .read(cx)
                .has_language_server_seed_for_worktree(invisible_worktree_id)
        );
    });

    project.update(cx, |project, cx| {
        project.remove_worktree(invisible_worktree_id, cx);
    });
    cx.run_until_parked();

    project.read_with(cx, |project, cx| {
        let lsp_store = project.lsp_store();
        let lsp_store = lsp_store.read(cx);
        assert!(
            lsp_store
                .language_server_statuses()
                .any(|(status_server_id, _)| status_server_id == server_id)
        );
        assert!(!lsp_store.has_language_server_seed_for_worktree(invisible_worktree_id));
    });
}

#[test]
fn test_glob_literal_prefix() {
    assert_eq!(glob_literal_prefix(Path::new("**/*.js")), Path::new(""));
    assert_eq!(
        glob_literal_prefix(Path::new("node_modules/**/*.js")),
        Path::new("node_modules")
    );
    assert_eq!(
        glob_literal_prefix(Path::new("foo/{bar,baz}.js")),
        Path::new("foo")
    );
    assert_eq!(
        glob_literal_prefix(Path::new("foo/bar/baz.js")),
        Path::new("foo/bar/baz.js")
    );

    #[cfg(target_os = "windows")]
    {
        assert_eq!(glob_literal_prefix(Path::new("**\\*.js")), Path::new(""));
        assert_eq!(
            glob_literal_prefix(Path::new("node_modules\\**/*.js")),
            Path::new("node_modules")
        );
        assert_eq!(
            glob_literal_prefix(Path::new("foo/{bar,baz}.js")),
            Path::new("foo")
        );
        assert_eq!(
            glob_literal_prefix(Path::new("foo\\bar\\baz.js")),
            Path::new("foo/bar/baz.js")
        );
    }
}

#[test]
fn test_multi_len_chars_normalization() {
    let mut label = CodeLabel::new(
        "myElˇ (parameter) myElˇ: {\n    foo: string;\n}".to_string(),
        0..6,
        vec![(0..6, HighlightId::new(1))],
    );
    ensure_uniform_list_compatible_label(&mut label);
    assert_eq!(
        label,
        CodeLabel::new(
            "myElˇ (parameter) myElˇ: { foo: string; }".to_string(),
            0..6,
            vec![(0..6, HighlightId::new(1))],
        )
    );
}

#[test]
fn test_trailing_newline_in_completion_documentation() {
    let doc =
        lsp::Documentation::String("Inappropriate argument value (of correct type).\n".to_string());
    let completion_doc: CompletionDocumentation = doc.into();
    assert!(
        matches!(completion_doc, CompletionDocumentation::SingleLine(s) if s == "Inappropriate argument value (of correct type).")
    );

    let doc = lsp::Documentation::String("  some value  \n".to_string());
    let completion_doc: CompletionDocumentation = doc.into();
    assert!(matches!(
        completion_doc,
        CompletionDocumentation::SingleLine(s) if s == "some value"
    ));
}
