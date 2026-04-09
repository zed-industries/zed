use std::sync::atomic::AtomicBool;
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use fuzzy::CharBag;
use util::{paths::PathStyle, rel_path::RelPath};

const DIRS: &[&str] = &[
    "src", "crates/gpui/src", "crates/editor/src", "crates/fuzzy_nucleo/src",
    "crates/workspace/src", "crates/project/src", "crates/language/src",
    "crates/terminal/src", "crates/assistant/src", "crates/theme/src",
    "tests/integration", "tests/unit", "docs/architecture", "scripts",
    "assets/icons", "assets/fonts", "crates/git/src", "crates/rpc/src",
    "crates/settings/src", "crates/diagnostics/src", "crates/search/src",
    "crates/collab/src", "crates/db/src", "crates/lsp/src",
];

const FILENAMES: &[&str] = &[
    "parser.rs", "main.rs", "executor.rs", "editor.rs", "strings.rs",
    "workspace.rs", "project.rs", "buffer.rs", "colors.rs", "panel.rs",
    "renderer.rs", "dispatcher.rs", "matcher.rs", "paths.rs", "context.rs",
    "toolbar.rs", "statusbar.rs", "keymap.rs", "config.rs", "settings.rs",
    "diagnostics.rs", "completion.rs", "hover.rs", "references.rs",
    "inlay_hints.rs", "git_blame.rs", "terminal.rs", "search.rs",
    "replace.rs", "outline.rs", "breadcrumbs.rs", "tab_bar.rs",
    "Cargo.toml", "README.md", "build.sh", "LICENSE", "overview.md",
    "string_helpers.rs", "test_helpers.rs", "fixtures.json", "schema.sql",
];

fn generate_candidates(count: usize) -> Vec<fuzzy_nucleo::StringMatchCandidate> {
    (0..count)
        .map(|id| {
            let dir = DIRS[id % DIRS.len()];
            let file = FILENAMES[id / DIRS.len() % FILENAMES.len()];
            fuzzy_nucleo::StringMatchCandidate::new(id, &format!("{dir}/{file}"))
        })
        .collect()
}

fn to_fuzzy_candidates(
    candidates: &[fuzzy_nucleo::StringMatchCandidate],
) -> Vec<fuzzy::StringMatchCandidate> {
    candidates
        .iter()
        .map(|c| fuzzy::StringMatchCandidate::new(c.id, &c.string))
        .collect()
}

fn bench_string_matching(criterion: &mut Criterion) {
    let cancel = AtomicBool::new(false);

    let dispatcher = std::sync::Arc::new(gpui::TestDispatcher::new(0));
    let background_executor = gpui::BackgroundExecutor::new(dispatcher.clone());
    let foreground_executor = gpui::ForegroundExecutor::new(dispatcher);

    let sizes = [100, 1000, 10_000];
    let queries = ["par", "src editor", "fuzzy"];

    for query in queries {
        let mut group = criterion.benchmark_group(query);
        for size in sizes {
            let candidates = generate_candidates(size);
            let fuzzy_candidates = to_fuzzy_candidates(&candidates);
            group.bench_function(BenchmarkId::new("nucleo", size), |b| {
                b.iter(|| {
                    foreground_executor.block_on(fuzzy_nucleo::match_strings(
                        &candidates, query, false, true, size, &cancel, background_executor.clone(),
                    ))
                })
            });
            group.bench_function(BenchmarkId::new("fuzzy", size), |b| {
                b.iter(|| {
                    foreground_executor.block_on(fuzzy::match_strings(
                        &fuzzy_candidates, query, false, true, size, &cancel, background_executor.clone(),
                    ))
                })
            });
        }
        group.finish();
    }
}

fn generate_path_strings(count: usize) -> &'static [&'static str] {
    let paths: Vec<&'static str> = (0..count)
        .map(|id| {
            let dir = DIRS[id % DIRS.len()];
            let file = FILENAMES[id / DIRS.len() % FILENAMES.len()];
            &*String::leak(format!("{dir}/{file}"))
        })
        .collect();
    Vec::leak(paths)
}

fn generate_nucleo_path_candidates(paths: &'static [&'static str]) -> Vec<fuzzy_nucleo::PathMatchCandidate<'static>> {
    paths
        .iter()
        .map(|path| fuzzy_nucleo::PathMatchCandidate {
            is_dir: false,
            path: RelPath::unix(path).unwrap(),
        })
        .collect()
}

fn generate_fuzzy_path_candidates(paths: &'static [&'static str]) -> Vec<fuzzy::PathMatchCandidate<'static>> {
    paths
        .iter()
        .map(|path| fuzzy::PathMatchCandidate {
            is_dir: false,
            path: RelPath::unix(path).unwrap(),
            char_bag: CharBag::from(*path),
        })
        .collect()
}

fn bench_path_matching(criterion: &mut Criterion) {
    let sizes = [100, 1000, 10_000];
    let queries = ["par", "src editor", "executor.rs"];
    let all_path_strings = sizes.map(generate_path_strings);

    for query in queries {
        let mut group = criterion.benchmark_group(format!("path/{query}"));
        for (size_index, &size) in sizes.iter().enumerate() {
            let path_strings = all_path_strings[size_index];
            group.bench_function(BenchmarkId::new("nucleo", size), |b| {
                b.iter_batched(
                    || generate_nucleo_path_candidates(&path_strings),
                    |candidates| {
                        fuzzy_nucleo::match_fixed_path_set(
                            candidates, 0, None, query, false, size, PathStyle::Posix,
                        )
                    },
                    BatchSize::SmallInput,
                )
            });
            group.bench_function(BenchmarkId::new("fuzzy", size), |b| {
                b.iter_batched(
                    || generate_fuzzy_path_candidates(&path_strings),
                    |candidates| {
                        fuzzy::match_fixed_path_set(
                            candidates, 0, None, query, false, size, PathStyle::Posix,
                        )
                    },
                    BatchSize::SmallInput,
                )
            });
        }
        group.finish();
    }
}

criterion_group!(benches, bench_string_matching, bench_path_matching);
criterion_main!(benches);
