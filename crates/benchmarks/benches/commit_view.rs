use std::{path::Path, sync::Arc};

use benchmarks::bench_utils::random_rust_file;
use client::{Client, UserStore};
use fs::RealFs;
use git::repository::RepoPath;
use gpui::{AppContext as _, BenchAppContext, Entity};
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use project::{
    LocalProjectFlags, Project,
    git_store::{CommitFile, Repository},
};
use rand::{SeedableRng as _, rngs::StdRng};
use settings::SettingsStore;

const FILE_COUNT: usize = 64;
const FILE_LINE_COUNT: usize = 2_000;
const COMMIT_SHA: &str = "0123456789abcdef0123456789abcdef01234567";

/// Measures loading every file of a commit into buffers and diffs, comparing
/// sequential loading against the concurrency the commit view uses.
#[gpui::bench(
    inputs = [1, 16],
    group = "Commit view load files",
    input_name = "concurrency",
    sample_size = 10
)]
fn commit_view_load_files(max_concurrent_loads: &usize, cx: &mut BenchAppContext) {
    init_context(cx);
    let (project, repository, language_registry) = open_own_repository(cx);
    let window_handle = cx.add_empty_window().window_handle();
    let max_concurrent_loads = *max_concurrent_loads;

    cx.bench_batched_task(
        |_| commit_files(),
        |files, cx| {
            let files = std::mem::take(files);
            let project = project.clone();
            let repository = repository.clone();
            let language_registry = language_registry.clone();
            cx.update(|cx| {
                window_handle
                    .update(cx, |_, window, cx| {
                        window.spawn(cx, async move |cx| {
                            git_ui::commit_view::bench_load_commit_files(
                                files,
                                repository,
                                project,
                                COMMIT_SHA.into(),
                                language_registry,
                                max_concurrent_loads,
                                cx,
                            )
                            .await
                            .expect("failed to load commit files")
                        })
                    })
                    .expect("benchmark window should be open")
            })
        },
    );
}

/// Opens this crate's directory as a worktree so the enclosing Zed checkout
/// provides the repository without scanning the whole tree.
fn open_own_repository(
    cx: &mut BenchAppContext,
) -> (Entity<Project>, Entity<Repository>, Arc<LanguageRegistry>) {
    let executor = cx.background_executor().clone();
    let language_registry = Arc::new(LanguageRegistry::new(executor.clone()));
    language_registry.add(language::rust_lang());

    let project = cx.update(|cx| {
        let client = Client::new(
            Arc::new(clock::RealSystemClock),
            http_client::FakeHttpClient::with_404_response(),
            cx,
        );
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        Project::local(
            client,
            NodeRuntime::unavailable(),
            user_store,
            language_registry.clone(),
            RealFs::new(None, executor),
            None,
            LocalProjectFlags::default(),
            cx,
        )
    });
    let worktree_task = cx.update(|cx| {
        project.update(cx, |project, cx| {
            project.find_or_create_worktree(Path::new(env!("CARGO_MANIFEST_DIR")), true, cx)
        })
    });
    cx.run_until(|| worktree_task.is_ready().then_some(()));

    let repository = cx.run_until(|| {
        cx.read(|cx| project.read(cx).repositories(cx).values().next().cloned())
    });
    (project, repository, language_registry)
}

/// Builds a commit that modifies `FILE_COUNT` Rust files, rebuilt per iteration
/// because loading consumes the files.
fn commit_files() -> Vec<CommitFile> {
    let mut rng = StdRng::seed_from_u64(1);
    (0..FILE_COUNT)
        .map(|index| {
            let mut lines = random_rust_file(&mut rng, FILE_LINE_COUNT);
            let old_text = lines.join("\n");
            for line in lines.iter_mut().step_by(50) {
                line.push_str(" // changed");
            }
            CommitFile {
                path: RepoPath::new(&format!("generated/file_{index}.rs"))
                    .expect("generated path should be valid"),
                old_text: Some(old_text),
                new_text: Some(lines.join("\n")),
                is_binary: false,
            }
        })
        .collect()
}

fn init_context(cx: &mut BenchAppContext) {
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
    });
}

gpui::bench_group!(benches, commit_view_load_files);
gpui::bench_main!(benches);
