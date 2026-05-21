use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

use anyhow::{Context as _, Result, ensure};
use collections::BTreeMap;
use dap::client::SessionId;
use fs::FakeFs;
use futures::{FutureExt as _, StreamExt as _, future};
use gpui::proptest::prelude::*;
use gpui::{App, AppContext as _, Entity, TestAppContext};
use language::{Buffer, FakeLspAdapter, rust_lang};
use project::{
    Event, Project, ProjectPath, TaskSourceKind, WorktreeId,
    bookmark_store::SerializedBookmark,
    debugger::{
        breakpoint_store::{
            Breakpoint, BreakpointEditAction, BreakpointState, BreakpointWithPosition,
            SourceBreakpoint,
        },
        dap_store::DapStoreEvent,
    },
    git_store::{GitStore, RepositoryId},
    project_settings::SettingsObserverEvent,
};
use rand::{Rng, SeedableRng, rngs::StdRng};
use serde_json::json;
use settings::SettingsStore;
use task::{ResolvedTask, TaskTemplate};
use util::{path, rel_path::rel_path};

fn init_test(cx: &mut TestAppContext) {
    zlog::init_test();
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
}

/// Build two `Project`s sharing a single `Host` (via `HostRegistry`),
/// with disjoint worktree roots, on a small FakeFs.
async fn two_projects_with_disjoint_worktrees(
    cx: &mut TestAppContext,
) -> (Arc<FakeFs>, Entity<Project>, Entity<Project>) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/repos"),
        json!({
            "alpha": {
                "src": { "lib.rs": "fn alpha() {}\nfn helper() {}\nfn aux() {}\n" },
            },
            "beta": {
                "src": { "lib.rs": "fn beta() {}\nfn helper() {}\nfn aux() {}\n" },
            },
        }),
    )
    .await;

    let project_a = Project::test(fs.clone(), [path!("/repos/alpha").as_ref()], cx).await;
    let project_b = Project::test(fs.clone(), [path!("/repos/beta").as_ref()], cx).await;
    cx.run_until_parked();

    // Sanity-check the Phase 2 invariant: same FakeFs → same Host
    // (any host-shared store entity will do as a proxy).
    let store_a = project_a.read_with(cx, |p, cx| p.bookmark_store(cx));
    let store_b = project_b.read_with(cx, |p, cx| p.bookmark_store(cx));
    assert_eq!(
        store_a.entity_id(),
        store_b.entity_id(),
        "both projects must share a single Host for this audit"
    );

    (fs, project_a, project_b)
}

async fn open_buffer(
    project: &Entity<Project>,
    abs_path: &str,
    cx: &mut TestAppContext,
) -> Entity<Buffer> {
    project
        .update(cx, |project, cx| {
            project.open_local_buffer(Path::new(abs_path), cx)
        })
        .await
        .unwrap()
}

async fn two_projects_with_disjoint_git_worktrees(
    cx: &mut TestAppContext,
) -> (Arc<FakeFs>, Entity<Project>, Entity<Project>) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/repos"),
        json!({
            "alpha": {
                ".git": {},
                "src": { "lib.rs": "fn alpha() {}\n" },
            },
            "beta": {
                ".git": {},
                "src": { "lib.rs": "fn beta() {}\n" },
            },
        }),
    )
    .await;

    let project_a = Project::test(fs.clone(), [path!("/repos/alpha").as_ref()], cx).await;
    let project_b = Project::test(fs.clone(), [path!("/repos/beta").as_ref()], cx).await;
    cx.run_until_parked();
    project_a
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    project_b
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.run_until_parked();

    let git_store_a = project_a.read_with(cx, |project, cx| project.git_store(cx));
    let git_store_b = project_b.read_with(cx, |project, cx| project.git_store(cx));
    assert_eq!(
        git_store_a.entity_id(),
        git_store_b.entity_id(),
        "both projects must share a single GitStore for this regression"
    );

    (fs, project_a, project_b)
}

#[gpui::test]
async fn test_project_drop_releases_repository(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_git_worktrees(cx).await;

    let git_store = project_b.read_with(cx, |project, cx| project.git_store(cx));
    let initial_repo_count = git_store.read_with(cx, |gs, _| gs.repositories().len());
    assert_eq!(initial_repo_count, 2);

    drop(project_a);
    // `cx.run_until_parked()` alone doesn't trigger the effect flush
    // that releases dropped entities; round-trip through `cx.update`.
    cx.update(|_| {});
    cx.run_until_parked();

    let remaining = git_store.read_with(cx, |gs, _| gs.repositories().len());
    assert_eq!(
        remaining, 1,
        "After dropping Project A, only Project B's repository should remain in the host GitStore",
    );
}

#[gpui::test]
async fn test_project_claiming_existing_repository_sets_active_repository(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/repos"),
        json!({
            "alpha": {
                ".git": {},
                "src": { "lib.rs": "fn alpha() {}\n" },
            },
        }),
    )
    .await;

    let project_a = Project::test(fs.clone(), [path!("/repos/alpha").as_ref()], cx).await;
    cx.run_until_parked();
    project_a
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;

    let project_b = Project::test(fs, std::iter::empty::<&Path>(), cx).await;
    let (worktree, _) = project_b
        .update(cx, |project, cx| {
            project.find_or_create_worktree(Path::new(path!("/repos/alpha")), true, cx)
        })
        .await
        .expect("failed to claim existing worktree");
    worktree
        .read_with(cx, |worktree, _| {
            worktree
                .as_local()
                .expect("property test worktree should be local")
                .scan_complete()
        })
        .await;
    project_b
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    cx.run_until_parked();

    let (repository_count, active_repo_path) = project_b.read_with(cx, |project, cx| {
        (
            project.repositories().len(),
            project
                .active_repository(cx)
                .map(|repo| repo.read(cx).work_directory_abs_path.clone()),
        )
    });

    assert_eq!(repository_count, 1);
    assert_eq!(
        active_repo_path.as_deref(),
        Some(Path::new(path!("/repos/alpha"))),
        "a project that back-claims an existing repository should make it active",
    );
}

#[gpui::test]
async fn test_active_repository_is_scoped_to_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_git_worktrees(cx).await;

    let active_repo_a = project_a.read_with(cx, |project, cx| {
        project
            .active_repository(cx)
            .map(|repo| repo.read(cx).work_directory_abs_path.clone())
    });
    let active_repo_b = project_b.read_with(cx, |project, cx| {
        project
            .active_repository(cx)
            .map(|repo| repo.read(cx).work_directory_abs_path.clone())
    });

    assert_eq!(
        active_repo_a.as_deref(),
        Some(Path::new(path!("/repos/alpha"))),
        "project_a should use its own repository as active"
    );
    assert_eq!(
        active_repo_b.as_deref(),
        Some(Path::new(path!("/repos/beta"))),
        "project_b should use its own repository as active, not the shared GitStore's active repository"
    );
}

const GIT_STORE_PROPERTY_REPO_PATHS: [&str; 4] = [
    path!("/repos/alpha"),
    path!("/repos/beta"),
    path!("/repos/gamma"),
    path!("/repos/delta"),
];
const GIT_STORE_PROPERTY_MAX_LIVE_PROJECTS: usize = 4;

#[derive(Clone, Copy, Debug)]
enum GitStorePropertyOperationKind {
    OpenProject,
    AddWorktree,
    DropProject,
}

fn git_store_property_tree() -> serde_json::Value {
    json!({
        "alpha": {
            ".git": {},
            "src": { "lib.rs": "fn alpha() {}\n" },
        },
        "beta": {
            ".git": {},
            "src": { "lib.rs": "fn beta() {}\n" },
        },
        "gamma": {
            ".git": {},
            "src": { "lib.rs": "fn gamma() {}\n" },
        },
        "delta": {
            ".git": {},
            "src": { "lib.rs": "fn delta() {}\n" },
        },
    })
}

struct HostRepositoryState {
    worktrees_by_repository_id: HashMap<RepositoryId, HashSet<WorktreeId>>,
    path_by_repository_id: HashMap<RepositoryId, Arc<Path>>,
}

struct GitStorePropertyWorld {
    fs: Arc<FakeFs>,
    projects: Vec<Entity<Project>>,
}

impl HostRepositoryState {
    fn new(git_store: &Entity<GitStore>, cx: &App) -> Result<Self> {
        let mut worktrees_by_repository_id =
            HashMap::<RepositoryId, HashSet<WorktreeId>>::default();
        let mut path_by_repository_id = HashMap::<RepositoryId, Arc<Path>>::default();
        let git_store = git_store.read(cx);

        for (repository_id, repository) in git_store.repositories() {
            let snapshot = repository.read(cx).snapshot();
            path_by_repository_id.insert(repository_id, snapshot.work_directory_abs_path);
            let worktree_ids = git_store
                .worktree_ids_for_repository(repository_id)
                .with_context(|| {
                    format!(
                        "local repository {:?} has no worktree associations",
                        repository_id
                    )
                })?;
            ensure!(
                !worktree_ids.is_empty(),
                "local repository {:?} has an empty worktree association set",
                repository_id
            );
            worktrees_by_repository_id.insert(repository_id, worktree_ids.clone());
        }

        Ok(Self {
            worktrees_by_repository_id,
            path_by_repository_id,
        })
    }
}

impl GitStorePropertyWorld {
    async fn new(cx: &mut TestAppContext) -> Entity<Self> {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/repos"), git_store_property_tree())
            .await;

        cx.new(|_| Self {
            fs,
            projects: Vec::new(),
        })
    }

    async fn open_project(this: &Entity<Self>, path_index: usize, cx: &mut TestAppContext) {
        let fs = this.read_with(cx, |this, _| this.fs.clone());
        let path = Path::new(GIT_STORE_PROPERTY_REPO_PATHS[path_index]);

        // Create an empty project first so we can register it with the
        // world before any worktree-driven `GitStore` events start firing.
        let project = Project::test(fs, std::iter::empty::<&Path>(), cx).await;
        this.update(cx, |this, _| {
            this.projects.push(project.clone());
        });

        let (tree, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path, true, cx)
            })
            .await
            .expect("failed to open property-test worktree");
        tree.read_with(cx, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
    }

    async fn add_worktree(
        this: &Entity<Self>,
        project_index: usize,
        path_index: usize,
        cx: &mut TestAppContext,
    ) {
        let project = this.read_with(cx, |this, _| this.projects[project_index].clone());
        let path = Path::new(GIT_STORE_PROPERTY_REPO_PATHS[path_index]);
        project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path, true, cx)
            })
            .await
            .expect("failed to add property-test worktree");
    }

    fn drop_project(this: &Entity<Self>, project_index: usize, cx: &mut TestAppContext) {
        let project = this.update(cx, |this, _| this.projects.swap_remove(project_index));
        drop(project);
        // `cx.run_until_parked()` only drains the dispatcher; it does not
        // trigger the effect-flush that releases dropped entities. Without
        // this `cx.update` round-trip, the dropped `Project`'s
        // `observe_release` chain (and the WorktreeStore + GitStore
        // cleanup it cascades into) never fires.
        cx.update(|_| {});
    }

    fn choose_operation(&self, choice: usize, cx: &App) -> GitStorePropertyOperationKind {
        let mut valid_operations = Vec::new();

        if self.projects.len() < GIT_STORE_PROPERTY_MAX_LIVE_PROJECTS {
            valid_operations.push(GitStorePropertyOperationKind::OpenProject);
        }
        if self.project_with_missing_worktree(cx).is_some() {
            valid_operations.push(GitStorePropertyOperationKind::AddWorktree);
        }
        if self.projects.len() > 1 {
            valid_operations.push(GitStorePropertyOperationKind::DropProject);
        }

        valid_operations[choice % valid_operations.len()]
    }

    fn project_with_missing_worktree(&self, cx: &App) -> Option<usize> {
        self.projects
            .iter()
            .enumerate()
            .find_map(|(project_index, project)| {
                let worktree_paths = project_worktree_paths(project, cx);
                GIT_STORE_PROPERTY_REPO_PATHS
                    .iter()
                    .any(|path| !worktree_paths.contains(Path::new(path)))
                    .then_some(project_index)
            })
    }

    fn choose_missing_worktree(&self, project_index: usize, rng: &mut StdRng, cx: &App) -> usize {
        let project = &self.projects[project_index];
        let worktree_paths = project_worktree_paths(project, cx);
        let missing_paths = GIT_STORE_PROPERTY_REPO_PATHS
            .iter()
            .enumerate()
            .filter_map(|(path_index, path)| {
                (!worktree_paths.contains(Path::new(path))).then_some(path_index)
            })
            .collect::<Vec<_>>();
        missing_paths[rng.random_range(0..missing_paths.len())]
    }
}

fn project_worktree_paths(project: &Entity<Project>, cx: &App) -> HashSet<PathBuf> {
    project.read_with(cx, |project, cx| {
        project
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).abs_path().as_ref().to_path_buf())
            .collect()
    })
}

fn verify_git_store_property_invariants(projects: &[Entity<Project>], cx: &App) -> Result<()> {
    if projects.is_empty() {
        return Ok(());
    }

    let git_store = projects[0].read_with(cx, |project, cx| project.git_store(cx));
    for project in &projects[1..] {
        let project_git_store = project.read_with(cx, |project, cx| project.git_store(cx));
        ensure!(
            project_git_store.entity_id() == git_store.entity_id(),
            "all generated projects should share one host GitStore"
        );
    }

    let host_repositories = HostRepositoryState::new(&git_store, cx)?;
    verify_project_repository_ownership(projects, &host_repositories, cx)?;
    verify_host_repositories_have_live_project_owner(projects, &host_repositories, cx)?;
    verify_repository_paths_map_to_shared_repository_ids(&host_repositories)?;

    Ok(())
}

fn verify_project_repository_ownership(
    projects: &[Entity<Project>],
    host_repositories: &HostRepositoryState,
    cx: &App,
) -> Result<()> {
    for project in projects {
        let project_worktree_ids = project.read_with(cx, |project, _| {
            project
                .worktrees(cx)
                .map(|worktree| worktree.read(cx).id())
                .collect::<HashSet<_>>()
        });
        let expected_repository_ids = host_repositories
            .worktrees_by_repository_id
            .iter()
            .filter_map(|(repository_id, worktree_ids)| {
                worktree_ids
                    .iter()
                    .any(|worktree_id| project_worktree_ids.contains(worktree_id))
                    .then_some(*repository_id)
            })
            .collect::<HashSet<_>>();
        let actual_repository_ids = project.read_with(cx, |project, _| {
            project
                .repositories()
                .keys()
                .copied()
                .collect::<HashSet<_>>()
        });
        ensure!(
            actual_repository_ids == expected_repository_ids,
            "project repository ids should match repositories associated with its worktrees. expected: {:?}, actual: {:?}",
            expected_repository_ids,
            actual_repository_ids
        );

        let active_repository_id =
            project.read_with(cx, |project, _| project.active_repository_id());
        if actual_repository_ids.is_empty() {
            ensure!(
                active_repository_id.is_none(),
                "project with no repositories should not have an active repository: {:?}",
                active_repository_id
            );
        } else {
            ensure!(
                active_repository_id.is_some(),
                "project with owned repositories should have an active repository; project repositories: {:?}",
                actual_repository_ids
            );
        }
        if let Some(active_repository_id) = active_repository_id {
            ensure!(
                host_repositories
                    .worktrees_by_repository_id
                    .contains_key(&active_repository_id),
                "active repository {:?} should exist in the host GitStore",
                active_repository_id
            );
            ensure!(
                actual_repository_ids.contains(&active_repository_id),
                "active repository {:?} should be owned by the project; project repositories: {:?}",
                active_repository_id,
                actual_repository_ids
            );
        }
    }

    Ok(())
}

fn verify_host_repositories_have_live_project_owner(
    projects: &[Entity<Project>],
    host_repositories: &HostRepositoryState,
    cx: &App,
) -> Result<()> {
    let project_worktree_ids = projects
        .iter()
        .map(|project| {
            project.read_with(cx, |project, cx| {
                project
                    .worktrees(cx)
                    .map(|worktree| worktree.read(cx).id())
                    .collect::<HashSet<_>>()
            })
        })
        .collect::<Vec<_>>();

    for (repository_id, worktree_ids) in &host_repositories.worktrees_by_repository_id {
        let owned_by_live_project = project_worktree_ids.iter().any(|project_ids| {
            worktree_ids
                .iter()
                .any(|worktree_id| project_ids.contains(worktree_id))
        });
        ensure!(
            owned_by_live_project,
            "host repository {:?} at {:?} should be owned by at least one live project",
            repository_id,
            host_repositories.path_by_repository_id.get(repository_id),
        );
    }

    Ok(())
}

fn verify_repository_paths_map_to_shared_repository_ids(
    host_repositories: &HostRepositoryState,
) -> Result<()> {
    let mut repository_id_by_path = HashMap::<Arc<Path>, RepositoryId>::default();
    for (repository_id, repository_path) in &host_repositories.path_by_repository_id {
        if let Some(existing_repository_id) =
            repository_id_by_path.insert(repository_path.clone(), *repository_id)
        {
            ensure!(
                existing_repository_id == *repository_id,
                "repository path {:?} should map to one shared repository id, but saw {:?} and {:?}",
                repository_path,
                existing_repository_id,
                repository_id
            );
        }
    }

    Ok(())
}

#[gpui::property_test(config = ProptestConfig {
    cases: 10,
    ..Default::default()
})]
async fn test_git_store_multi_tenant_random_invariants(
    #[strategy = any::<u64>()] seed: u64,
    #[strategy = gpui::proptest::collection::vec(0usize..1000, 1..40)] operation_choices: Vec<
        usize,
    >,
    cx: &mut TestAppContext,
) {
    init_test(cx);
    cx.executor().allow_parking();
    let mut rng = StdRng::seed_from_u64(seed);

    let world = GitStorePropertyWorld::new(cx).await;
    GitStorePropertyWorld::open_project(
        &world,
        rng.random_range(0..GIT_STORE_PROPERTY_REPO_PATHS.len()),
        cx,
    )
    .await;
    cx.run_until_parked();

    for choice in operation_choices {
        let op = world.read_with(cx, |world, cx| world.choose_operation(choice, cx));
        let op_description = match op {
            GitStorePropertyOperationKind::OpenProject => {
                let path_index = rng.random_range(0..GIT_STORE_PROPERTY_REPO_PATHS.len());
                let description = format!(
                    "open project {:?}",
                    GIT_STORE_PROPERTY_REPO_PATHS[path_index]
                );
                GitStorePropertyWorld::open_project(&world, path_index, cx).await;
                description
            }
            GitStorePropertyOperationKind::AddWorktree => {
                let (project_index, path_index) = world.read_with(cx, |world, cx| {
                    let project_index = world.project_with_missing_worktree(cx).unwrap();
                    let path_index = world.choose_missing_worktree(project_index, &mut rng, cx);
                    (project_index, path_index)
                });
                let description = format!(
                    "add worktree project={} path={:?}",
                    project_index, GIT_STORE_PROPERTY_REPO_PATHS[path_index]
                );
                GitStorePropertyWorld::add_worktree(&world, project_index, path_index, cx).await;
                description
            }
            GitStorePropertyOperationKind::DropProject => {
                let project_count = world.read_with(cx, |world, _| world.projects.len());
                let project_index = rng.random_range(0..project_count);
                let description = format!("drop project {}", project_index);
                GitStorePropertyWorld::drop_project(&world, project_index, cx);
                description
            }
        };

        cx.run_until_parked();

        world.read_with(cx, |world, cx| {
            verify_git_store_property_invariants(&world.projects, cx)
                .with_context(|| format!("after operation: {}", op_description))
                .unwrap();
        });
    }
}

// ────────────────────────────────────────────────────────────────
// LspStore
// ────────────────────────────────────────────────────────────────

#[gpui::test]
async fn test_language_server_statuses_are_scoped_to_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/repos"),
        json!({
            "alpha": {
                "src": { "lib.rs": "fn alpha() {}\n" },
            },
            "beta": {
                "src": { "lib.rs": "fn beta() {}\n" },
            },
        }),
    )
    .await;

    let project_a = Project::test(fs.clone(), [path!("/repos/alpha").as_ref()], cx).await;
    let project_b = Project::test(fs, [path!("/repos/beta").as_ref()], cx).await;
    cx.run_until_parked();

    let lsp_store_a = project_a.read_with(cx, |project, cx| project.lsp_store(cx));
    let lsp_store_b = project_b.read_with(cx, |project, cx| project.lsp_store(cx));
    assert_eq!(
        lsp_store_a.entity_id(),
        lsp_store_b.entity_id(),
        "both projects must share a single LspStore"
    );

    let language_registry = project_a.read_with(cx, |project, _| project.languages().clone());
    let server_capabilities = || lsp::ServerCapabilities {
        text_document_sync: Some(lsp::TextDocumentSyncCapability::Options(
            lsp::TextDocumentSyncOptions {
                open_close: Some(true),
                ..Default::default()
            },
        )),
        ..Default::default()
    };
    let mut fake_rust_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "rust-analyzer",
            capabilities: server_capabilities(),
            ..Default::default()
        },
    );
    language_registry.add(rust_lang());
    cx.executor().run_until_parked();

    let (_alpha_buffer, _alpha_lsp_handle) = project_a
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/repos/alpha/src/lib.rs"), cx)
        })
        .await
        .unwrap();
    let (_beta_buffer, _beta_lsp_handle) = project_b
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/repos/beta/src/lib.rs"), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let mut fake_rust_server = fake_rust_servers.next().await.unwrap();
    let mut sibling_rust_server = fake_rust_servers.next().await.unwrap();
    let alpha_server_id = fake_rust_server.server.server_id();
    let beta_server_id = sibling_rust_server.server.server_id();

    let rust_open = fake_rust_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;
    assert_eq!(
        rust_open.text_document.uri,
        lsp::Uri::from_file_path(path!("/repos/alpha/src/lib.rs")).unwrap()
    );
    let sibling_rust_open = sibling_rust_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;
    assert_eq!(
        sibling_rust_open.text_document.uri,
        lsp::Uri::from_file_path(path!("/repos/beta/src/lib.rs")).unwrap()
    );

    let alpha_worktree_id = project_a.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let beta_worktree_id = project_b.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });

    let host_status_ids = lsp_store_a.read_with(cx, |lsp_store, _| {
        lsp_store
            .language_server_statuses()
            .map(|(server_id, _)| server_id)
            .collect::<HashSet<_>>()
    });
    assert_eq!(
        host_status_ids,
        HashSet::from([alpha_server_id, beta_server_id])
    );

    let project_a_statuses = project_a.read_with(cx, |project, cx| {
        project
            .language_server_statuses(cx)
            .map(|(server_id, status)| (server_id, status.name.clone(), status.worktree))
            .collect::<Vec<_>>()
    });
    assert_eq!(
        project_a_statuses,
        vec![(
            alpha_server_id,
            "rust-analyzer".into(),
            Some(alpha_worktree_id)
        )]
    );

    let project_b_statuses = project_b.read_with(cx, |project, cx| {
        project
            .language_server_statuses(cx)
            .map(|(server_id, status)| (server_id, status.name.clone(), status.worktree))
            .collect::<Vec<_>>()
    });
    assert_eq!(
        project_b_statuses,
        vec![(
            beta_server_id,
            "rust-analyzer".into(),
            Some(beta_worktree_id)
        )]
    );
}

#[gpui::test]
async fn test_restart_all_language_servers_is_scoped_to_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/repos"),
        json!({
            "alpha": {
                "src": { "lib.rs": "fn alpha() {}\n" },
            },
            "beta": {
                "src": { "lib.rs": "fn beta() {}\n" },
            },
        }),
    )
    .await;

    let project_a = Project::test(fs.clone(), [path!("/repos/alpha").as_ref()], cx).await;
    let project_b = Project::test(fs, [path!("/repos/beta").as_ref()], cx).await;
    cx.run_until_parked();

    let lsp_store_a = project_a.read_with(cx, |project, cx| project.lsp_store(cx));
    let lsp_store_b = project_b.read_with(cx, |project, cx| project.lsp_store(cx));
    assert_eq!(
        lsp_store_a.entity_id(),
        lsp_store_b.entity_id(),
        "both projects must share a single LspStore"
    );

    let language_registry = project_a.read_with(cx, |project, _| project.languages().clone());
    let server_capabilities = || lsp::ServerCapabilities {
        text_document_sync: Some(lsp::TextDocumentSyncCapability::Options(
            lsp::TextDocumentSyncOptions {
                open_close: Some(true),
                ..Default::default()
            },
        )),
        ..Default::default()
    };
    let mut fake_rust_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "rust-analyzer",
            capabilities: server_capabilities(),
            ..Default::default()
        },
    );
    language_registry.add(rust_lang());
    cx.executor().run_until_parked();

    let (_alpha_buffer, _alpha_lsp_handle) = project_a
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/repos/alpha/src/lib.rs"), cx)
        })
        .await
        .unwrap();
    let (_beta_buffer, _beta_lsp_handle) = project_b
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/repos/beta/src/lib.rs"), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let mut fake_rust_server = fake_rust_servers.next().await.unwrap();
    let mut sibling_rust_server = fake_rust_servers.next().await.unwrap();

    let rust_open = fake_rust_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;
    assert_eq!(
        rust_open.text_document.uri,
        lsp::Uri::from_file_path(path!("/repos/alpha/src/lib.rs")).unwrap()
    );
    let sibling_rust_open = sibling_rust_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;
    assert_eq!(
        sibling_rust_open.text_document.uri,
        lsp::Uri::from_file_path(path!("/repos/beta/src/lib.rs")).unwrap()
    );

    let mut rust_shutdown_requests = fake_rust_server
        .set_request_handler::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
    let mut sibling_rust_shutdown_requests = sibling_rust_server
        .set_request_handler::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));

    project_a.update(cx, |project, cx| project.restart_all_language_servers(cx));

    assert!(
        rust_shutdown_requests.next().await.is_some(),
        "restarting all servers for project_a should stop project_a's Rust server"
    );

    cx.executor().run_until_parked();
    assert!(
        sibling_rust_shutdown_requests
            .next()
            .now_or_never()
            .is_none(),
        "restarting all servers for project_a must not shut down project_b's Rust server"
    );
}

// ────────────────────────────────────────────────────────────────
// BookmarkStore
// ────────────────────────────────────────────────────────────────

/// Bookmarks set in `project_a` must not leak into `project_b`'s
/// serialized state (`workspace.serialize_workspace_internal` calls
/// `project.bookmark_store(cx).read(cx).all_serialized_bookmarks(cx)`).
#[gpui::test]
async fn test_bookmark_store_per_project_serialization(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let buffer_a = open_buffer(&project_a, path!("/repos/alpha/src/lib.rs"), cx).await;
    project_a.update(cx, |project, cx| {
        let bookmark_store = project.bookmark_store(cx);
        let snapshot = buffer_a.read(cx).snapshot();
        let anchor = snapshot.anchor_after(text::Point::new(1, 0));
        bookmark_store.update(cx, |store, cx| {
            store.toggle_bookmark(buffer_a.clone(), anchor, cx);
        });
    });

    let a_bookmarks = project_a.read_with(cx, |p, cx| p.serialized_bookmarks(cx));
    let b_bookmarks = project_b.read_with(cx, |p, cx| p.serialized_bookmarks(cx));

    assert!(
        a_bookmarks
            .keys()
            .any(|p| p.ends_with(path!("alpha/src/lib.rs"))),
        "project_a should have its own bookmark"
    );
    assert!(
        b_bookmarks.is_empty(),
        "project_b should have no bookmarks, but saw: {:?}",
        b_bookmarks.keys().collect::<Vec<_>>(),
    );
}

#[gpui::test]
async fn test_bookmark_store_load_preserves_other_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    // Project B sets a bookmark first.
    let buffer_b = open_buffer(&project_b, path!("/repos/beta/src/lib.rs"), cx).await;
    project_b.update(cx, |project, cx| {
        let bookmark_store = project.bookmark_store(cx);
        let snapshot = buffer_b.read(cx).snapshot();
        let anchor = snapshot.anchor_after(text::Point::new(1, 0));
        bookmark_store.update(cx, |store, cx| {
            store.toggle_bookmark(buffer_b.clone(), anchor, cx);
        });
    });

    // Project A loads its (different) serialized bookmarks.
    let mut serialized: BTreeMap<Arc<Path>, Vec<SerializedBookmark>> = BTreeMap::new();
    serialized.insert(
        Arc::from(Path::new(path!("/repos/alpha/src/lib.rs"))),
        vec![SerializedBookmark(0)],
    );
    project_a
        .update(cx, |project, cx| {
            project.restore_serialized_bookmarks(serialized, cx)
        })
        .await
        .unwrap();

    let b_bookmarks = project_b.read_with(cx, |p, cx| p.serialized_bookmarks(cx));
    assert!(
        b_bookmarks
            .keys()
            .any(|p| p.ends_with(path!("beta/src/lib.rs"))),
        "project_b's bookmark should survive project_a's load, but found: {:?}",
        b_bookmarks.keys().collect::<Vec<_>>(),
    );
}

// ────────────────────────────────────────────────────────────────
// BreakpointStore
// ────────────────────────────────────────────────────────────────
#[gpui::test]
async fn test_breakpoint_store_per_project_serialization(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let buffer_a = open_buffer(&project_a, path!("/repos/alpha/src/lib.rs"), cx).await;
    project_a.update(cx, |project, cx| {
        let breakpoint_store = project.breakpoint_store(cx);
        let snapshot = buffer_a.read(cx).snapshot();
        let position = snapshot.anchor_after(text::Point::new(1, 0));
        breakpoint_store.update(cx, |store, cx| {
            store.toggle_breakpoint(
                buffer_a.clone(),
                BreakpointWithPosition {
                    position,
                    bp: Breakpoint::new_standard(),
                },
                BreakpointEditAction::Toggle,
                cx,
            );
        });
    });

    let a_bps = project_a.read_with(cx, |p, cx| p.serialized_breakpoints(cx));
    let b_bps = project_b.read_with(cx, |p, cx| p.serialized_breakpoints(cx));

    assert!(
        a_bps.keys().any(|p| p.ends_with(path!("alpha/src/lib.rs"))),
        "project_a should have its own breakpoint"
    );
    assert!(
        b_bps.is_empty(),
        "project_b should have no breakpoints, but saw: {:?}",
        b_bps.keys().collect::<Vec<_>>(),
    );
}

#[gpui::test]
async fn test_breakpoint_store_clear_per_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let buffer_a = open_buffer(&project_a, path!("/repos/alpha/src/lib.rs"), cx).await;
    let buffer_b = open_buffer(&project_b, path!("/repos/beta/src/lib.rs"), cx).await;

    for (project, buffer) in [(&project_a, &buffer_a), (&project_b, &buffer_b)] {
        project.update(cx, |project, cx| {
            let breakpoint_store = project.breakpoint_store(cx);
            let snapshot = buffer.read(cx).snapshot();
            let position = snapshot.anchor_after(text::Point::new(0, 0));
            breakpoint_store.update(cx, |store, cx| {
                store.toggle_breakpoint(
                    buffer.clone(),
                    BreakpointWithPosition {
                        position,
                        bp: Breakpoint::new_standard(),
                    },
                    BreakpointEditAction::Toggle,
                    cx,
                );
            });
        });
    }

    project_a.update(cx, |project, cx| project.clear_breakpoints(cx));

    let b_bps = project_b.read_with(cx, |p, cx| p.serialized_breakpoints(cx));
    assert!(
        b_bps.keys().any(|p| p.ends_with(path!("beta/src/lib.rs"))),
        "project_b's breakpoints should survive A's clear, but saw: {:?}",
        b_bps.keys().collect::<Vec<_>>(),
    );
}

#[gpui::test]
async fn test_breakpoint_store_load_preserves_other_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let buffer_b = open_buffer(&project_b, path!("/repos/beta/src/lib.rs"), cx).await;
    project_b.update(cx, |project, cx| {
        let breakpoint_store = project.breakpoint_store(cx);
        let snapshot = buffer_b.read(cx).snapshot();
        let position = snapshot.anchor_after(text::Point::new(0, 0));
        breakpoint_store.update(cx, |store, cx| {
            store.toggle_breakpoint(
                buffer_b.clone(),
                BreakpointWithPosition {
                    position,
                    bp: Breakpoint::new_standard(),
                },
                BreakpointEditAction::Toggle,
                cx,
            );
        });
    });

    let mut serialized: BTreeMap<Arc<Path>, Vec<SourceBreakpoint>> = BTreeMap::new();
    serialized.insert(
        Arc::from(Path::new(path!("/repos/alpha/src/lib.rs"))),
        vec![SourceBreakpoint {
            row: 0,
            path: Arc::from(Path::new(path!("/repos/alpha/src/lib.rs"))),
            message: None,
            condition: None,
            hit_condition: None,
            state: BreakpointState::Enabled,
        }],
    );

    project_a
        .update(cx, |project, cx| {
            project.restore_serialized_breakpoints(serialized, cx)
        })
        .await
        .unwrap();

    let b_bps = project_b.read_with(cx, |p, cx| p.serialized_breakpoints(cx));
    assert!(
        b_bps.keys().any(|p| p.ends_with(path!("beta/src/lib.rs"))),
        "project_b's breakpoints should survive project_a's load, but saw: {:?}",
        b_bps.keys().collect::<Vec<_>>(),
    );
}

// ────────────────────────────────────────────────────────────────
// ImageStore
// ────────────────────────────────────────────────────────────────

/// `ImageStore::images()` returns every opened image across every
/// `Project`. UI surfaces should use the filtered `Project::images`
/// accessor instead so a shared host store doesn't leak sibling
/// Projects' images.
#[gpui::test]
async fn test_image_store_images_per_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/repos"),
        json!({
            "alpha": { "icon.png": "" },
            "beta": { "icon.png": "" },
        }),
    )
    .await;
    // 1x1 white PNG.
    let png = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F,
        0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    fs.insert_file(path!("/repos/alpha/icon.png"), png.clone())
        .await;
    fs.insert_file(path!("/repos/beta/icon.png"), png).await;

    let project_a = Project::test(fs.clone(), [path!("/repos/alpha").as_ref()], cx).await;
    let project_b = Project::test(fs.clone(), [path!("/repos/beta").as_ref()], cx).await;
    cx.run_until_parked();

    let worktree_a = project_a.read_with(cx, |p, cx| p.worktrees(cx).next().unwrap().read(cx).id());
    let _image_a = project_a
        .update(cx, |project, cx| {
            project.open_image(
                ProjectPath {
                    worktree_id: worktree_a,
                    path: rel_path("icon.png").into(),
                },
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    // Project-filtered view: B should see no images, A should see one.
    let b_image_count = project_b.read_with(cx, |p, cx| p.images(cx).len());
    let a_image_count = project_a.read_with(cx, |p, cx| p.images(cx).len());
    assert_eq!(
        b_image_count, 0,
        "project_b should not see project_a's images via Project::images",
    );
    assert_eq!(
        a_image_count, 1,
        "project_a should see exactly its own opened image"
    );

    // The unfiltered host accessor still sees the union — documented as
    // a known leaky bystander; UI must go through `Project::images`.
    let host_image_count =
        project_b.read_with(cx, |p, cx| p.image_store(cx).read(cx).images().count());
    assert_eq!(
        host_image_count, 1,
        "shared host ImageStore is expected to hold the union of all Projects' images"
    );
}

// ─────────────────────────────────────────────────────────────
// SettingsObserver
// ─────────────────────────────────────────────────────────────

/// `SettingsObserverEvent::LocalTasksUpdated` fires on the shared host
/// observer for every worktree on the machine. The error case is
/// surfaced as a `Project::Event::Toast`; in Phase 2 only the Project
/// owning the worktree should toast, otherwise workspace A pops a
/// toast about workspace B's settings file.
#[gpui::test]
async fn test_settings_observer_toast_scoped_to_owning_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let a_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let b_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let _subs = cx.update(|cx| {
        let a_events = a_events.clone();
        let b_events = b_events.clone();
        let sub_a = cx.subscribe(&project_a, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                a_events.borrow_mut().push(notification_id.to_string());
            }
        });
        let sub_b = cx.subscribe(&project_b, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                b_events.borrow_mut().push(notification_id.to_string());
            }
        });
        (sub_a, sub_b)
    });

    // Emit a failure event for a path under project_a's worktree.
    let bad_path = std::path::PathBuf::from(path!("/repos/alpha/.zed/tasks.json"));
    let observer = project_a.read_with(cx, |p, cx| p.settings_observer(cx));
    observer.update(cx, |_, cx| {
        cx.emit(SettingsObserverEvent::LocalTasksUpdated(Err(
            settings::InvalidSettingsError::Tasks {
                path: bad_path.clone(),
                message: "bad json".into(),
            },
        )));
    });
    cx.run_until_parked();

    assert!(
        a_events
            .borrow()
            .iter()
            .any(|id| id.contains("alpha/.zed/tasks.json")),
        "project_a should receive a toast for its own settings file, got: {:?}",
        a_events.borrow(),
    );
    assert!(
        b_events.borrow().is_empty(),
        "project_b should not receive a toast for project_a's settings file, got: {:?}",
        b_events.borrow(),
    );
}

/// Global settings paths (e.g. `~/.config/zed/settings.json`) live
/// outside every worktree and must surface toasts in *every*
/// workspace — the multi-tenant filter must not over-restrict.
#[gpui::test]
async fn test_settings_observer_global_toasts_in_every_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let a_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let b_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let _subs = cx.update(|cx| {
        let a_events = a_events.clone();
        let b_events = b_events.clone();
        let sub_a = cx.subscribe(&project_a, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                a_events.borrow_mut().push(notification_id.to_string());
            }
        });
        let sub_b = cx.subscribe(&project_b, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                b_events.borrow_mut().push(notification_id.to_string());
            }
        });
        (sub_a, sub_b)
    });

    // Path outside every worktree mimics ~/.config/zed/tasks.json.
    let global_path = std::path::PathBuf::from(path!("/global/tasks.json"));
    let observer = project_a.read_with(cx, |p, cx| p.settings_observer(cx));
    observer.update(cx, |_, cx| {
        cx.emit(SettingsObserverEvent::LocalTasksUpdated(Err(
            settings::InvalidSettingsError::Tasks {
                path: global_path,
                message: "bad json".into(),
            },
        )));
    });
    cx.run_until_parked();

    assert!(
        !a_events.borrow().is_empty(),
        "project_a should receive the global toast"
    );
    assert!(
        !b_events.borrow().is_empty(),
        "project_b should also receive the global toast"
    );
}

/// `InvalidSettingsError::LocalSettings` (the parse-failure variant
/// for `.zed/settings.json`) carries a `WorktreeId`. The toast must
/// fire only in the Project owning that worktree.
#[gpui::test]
async fn test_settings_observer_local_settings_scoped_by_worktree_id(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let a_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let b_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let _subs = cx.update(|cx| {
        let a_events = a_events.clone();
        let b_events = b_events.clone();
        let sub_a = cx.subscribe(&project_a, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                a_events.borrow_mut().push(notification_id.to_string());
            }
        });
        let sub_b = cx.subscribe(&project_b, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                b_events.borrow_mut().push(notification_id.to_string());
            }
        });
        (sub_a, sub_b)
    });

    let alpha_worktree_id = project_a.read_with(cx, |p, cx| {
        p.visible_worktrees(cx).next().unwrap().read(cx).id()
    });
    let observer = project_a.read_with(cx, |p, cx| p.settings_observer(cx));
    observer.update(cx, |_, cx| {
        cx.emit(SettingsObserverEvent::LocalSettingsUpdated(Err(
            settings::InvalidSettingsError::LocalSettings {
                worktree_id: alpha_worktree_id,
                path: rel_path(".zed/settings.json").into(),
                message: "bad json".into(),
            },
        )));
    });
    cx.run_until_parked();

    assert!(
        !a_events.borrow().is_empty(),
        "project_a should receive a toast for its own worktree's local-settings file"
    );
    assert!(
        b_events.borrow().is_empty(),
        "project_b should not receive a toast for project_a's local-settings file, got: {:?}",
        b_events.borrow(),
    );
}

// ─────────────────────────────────────────────────────────────
// TaskStore (Inventory LRU)
// ─────────────────────────────────────────────────────────────

/// The recently-scheduled-task LRU was moved off the host-shared
/// `Inventory` and onto `Project` for Phase 2: a task scheduled in
/// workspace A must not show up as workspace B's "last scheduled task".
#[gpui::test]
async fn test_task_inventory_last_scheduled_per_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let resolved = make_resolved_task("build", "cargo build");
    let kind = TaskSourceKind::UserInput;

    project_a.update(cx, |project, _| {
        project.task_scheduled(kind.clone(), resolved.clone());
    });

    let a_last = project_a.read_with(cx, |project, _| project.last_scheduled_task(None));
    let b_last = project_b.read_with(cx, |project, _| project.last_scheduled_task(None));

    assert!(
        a_last.is_some_and(|(_, t)| t.original_task().label == "build"),
        "project_a should see its own scheduled task"
    );
    assert!(
        b_last.is_none(),
        "project_b should not see project_a's last scheduled task, but got: {:?}",
        b_last
    );
}

// ─────────────────────────────────────────────────────────────
// DapStore
// ─────────────────────────────────────────────────────────────

/// `DapStoreEvent::Notification` carries an optional `SessionId`. The
/// toast should fire only in the Project that owns that session;
/// host-wide notifications (no session) still show everywhere.
#[gpui::test]
async fn test_dap_store_notification_scoped_by_session_id(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let a_messages = Rc::new(RefCell::new(Vec::<String>::new()));
    let b_messages = Rc::new(RefCell::new(Vec::<String>::new()));
    let _subs = cx.update(|cx| {
        let a_messages = a_messages.clone();
        let b_messages = b_messages.clone();
        let sub_a = cx.subscribe(&project_a, move |_, event: &Event, _| {
            if let Event::Toast { message, .. } = event {
                a_messages.borrow_mut().push(message.clone());
            }
        });
        let sub_b = cx.subscribe(&project_b, move |_, event: &Event, _| {
            if let Event::Toast { message, .. } = event {
                b_messages.borrow_mut().push(message.clone());
            }
        });
        (sub_a, sub_b)
    });

    // project_a claims a session id; the shared host DapStore emits
    // a session-scoped notification.
    let owned_session = SessionId(42);
    project_a.update(cx, |project, _| project.claim_dap_session(owned_session));
    let dap_store = project_a.read_with(cx, |p, cx| p.dap_store(cx));
    dap_store.update(cx, |_, cx| {
        cx.emit(DapStoreEvent::Notification {
            session_id: Some(owned_session),
            message: "breakpoint failure".into(),
        });
    });
    cx.run_until_parked();

    assert!(
        a_messages
            .borrow()
            .iter()
            .any(|m| m == "breakpoint failure"),
        "project_a should see its own session's toast"
    );
    assert!(
        b_messages.borrow().is_empty(),
        "project_b should not see project_a's session toast, got: {:?}",
        b_messages.borrow(),
    );

    // A host-wide notification (no session) reaches both Projects.
    dap_store.update(cx, |_, cx| {
        cx.emit(DapStoreEvent::Notification {
            session_id: None,
            message: "host-wide".into(),
        });
    });
    cx.run_until_parked();
    assert!(
        a_messages.borrow().iter().any(|m| m == "host-wide"),
        "project_a should see host-wide toast"
    );
    assert!(
        b_messages.borrow().iter().any(|m| m == "host-wide"),
        "project_b should see host-wide toast"
    );
}

/// `Project::dap_sessions` and `Project::dap_session_by_id` are
/// filtered through the per-Project `dap_sessions` set so a sibling
/// Project's debug session doesn't leak into our UI surfaces.
#[gpui::test]
async fn test_dap_store_session_ownership_set(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let a_session = SessionId(7);
    let b_session = SessionId(8);
    project_a.update(cx, |project, _| project.claim_dap_session(a_session));
    project_b.update(cx, |project, _| project.claim_dap_session(b_session));

    project_a.read_with(cx, |project, _| {
        assert!(project.owns_dap_session(a_session));
        assert!(!project.owns_dap_session(b_session));
    });
    project_b.read_with(cx, |project, _| {
        assert!(project.owns_dap_session(b_session));
        assert!(!project.owns_dap_session(a_session));
    });

    // Pruning rides on `DapStoreEvent::DebugClientShutdown`. Faking
    // the event is enough to verify the set is updated; we don't need
    // a real session entity since `dap_session_by_id` returns `None`
    // either way.
    let dap_store = project_a.read_with(cx, |p, cx| p.dap_store(cx));
    dap_store.update(cx, |_, cx| {
        cx.emit(DapStoreEvent::DebugClientShutdown(a_session));
    });
    cx.run_until_parked();
    project_a.read_with(cx, |project, _| {
        assert!(
            !project.owns_dap_session(a_session),
            "shutdown event should prune the session from the per-project set"
        );
    });
}

fn make_resolved_task(label: &str, command: &str) -> ResolvedTask {
    let template = TaskTemplate {
        label: label.into(),
        command: command.into(),
        ..Default::default()
    };
    template
        .resolve_task("test", &task::TaskContext::default())
        .expect("task should resolve with empty context")
}
