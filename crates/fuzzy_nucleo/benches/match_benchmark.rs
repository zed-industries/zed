use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use fuzzy::CharBag;
use std::sync::atomic::AtomicBool;
use util::{paths::PathStyle, rel_path::RelPath};

const DIRS: &[&str] = &[
    "src",
    "crates/gpui/src",
    "crates/editor/src",
    "crates/fuzzy_nucleo/src",
    "crates/workspace/src",
    "crates/project/src",
    "crates/language/src",
    "crates/terminal/src",
    "crates/assistant/src",
    "crates/theme/src",
    "tests/integration",
    "tests/unit",
    "docs/architecture",
    "scripts",
    "assets/icons",
    "assets/fonts",
    "crates/git/src",
    "crates/rpc/src",
    "crates/settings/src",
    "crates/diagnostics/src",
    "crates/search/src",
    "crates/collab/src",
    "crates/db/src",
    "crates/lsp/src",
];

const FILENAMES: &[&str] = &[
    "parser.rs",
    "main.rs",
    "executor.rs",
    "editor.rs",
    "strings.rs",
    "workspace.rs",
    "project.rs",
    "buffer.rs",
    "colors.rs",
    "panel.rs",
    "renderer.rs",
    "dispatcher.rs",
    "matcher.rs",
    "paths.rs",
    "context.rs",
    "toolbar.rs",
    "statusbar.rs",
    "keymap.rs",
    "config.rs",
    "settings.rs",
    "diagnostics.rs",
    "completion.rs",
    "hover.rs",
    "references.rs",
    "inlay_hints.rs",
    "git_blame.rs",
    "terminal.rs",
    "search.rs",
    "replace.rs",
    "outline.rs",
    "breadcrumbs.rs",
    "tab_bar.rs",
    "Cargo.toml",
    "README.md",
    "build.sh",
    "LICENSE",
    "overview.md",
    "string_helpers.rs",
    "test_helpers.rs",
    "fixtures.json",
    "schema.sql",
];

const QUERY_WORDS: &[&str] = &[
    "par",
    "edi",
    "buf",
    "set",
    "mat",
    "con",
    "ren",
    "dis",
    "sea",
    "ter",
    "col",
    "hov",
    "out",
    "rep",
    "key",
    "too",
    "pan",
    "str",
    "dia",
    "com",
    "executor",
    "workspace",
    "settings",
    "terminal",
    "breadcrumbs",
    "git_blame",
    "fixtures",
    "schema",
    "config",
    "toolbar",
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

/// Deterministic query generation from QUERY_WORDS using a simple LCG.
/// Returns `count` single-word queries and `count` two-word queries.
fn generate_queries(count: usize) -> (Vec<&'static str>, Vec<String>) {
    let mut state: u64 = 0xDEAD_BEEF;
    let mut next = || -> usize {
        // LCG: simple, fast, deterministic
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (state >> 33) as usize
    };

    let single: Vec<&str> = (0..count)
        .map(|_| QUERY_WORDS[next() % QUERY_WORDS.len()])
        .collect();

    let two_word: Vec<String> = (0..count)
        .map(|_| {
            let a = QUERY_WORDS[next() % QUERY_WORDS.len()];
            let b = QUERY_WORDS[next() % QUERY_WORDS.len()];
            format!("{a} {b}")
        })
        .collect();

    (single, two_word)
}

fn bench_string_matching(criterion: &mut Criterion) {
    let cancel = AtomicBool::new(false);

    let dispatcher = std::sync::Arc::new(gpui::TestDispatcher::new(0));
    let background_executor = gpui::BackgroundExecutor::new(dispatcher.clone());
    let foreground_executor = gpui::ForegroundExecutor::new(dispatcher);

    let sizes = [100, 1000, 10_000];
    let query_count = 200;
    let (single_queries, two_word_queries) = generate_queries(query_count);

    for (label, queries) in [
        (
            "1-word",
            single_queries
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<&str>>(),
        ),
        (
            "2-word",
            two_word_queries
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
        ),
    ] {
        let mut group = criterion.benchmark_group(label);
        for size in sizes {
            let candidates = generate_candidates(size);
            let fuzzy_candidates = to_fuzzy_candidates(&candidates);

            let mut query_idx = 0usize;
            group.bench_function(BenchmarkId::new("nucleo", size), |b| {
                b.iter(|| {
                    let query = queries[query_idx % queries.len()];
                    query_idx += 1;
                    foreground_executor.block_on(fuzzy_nucleo::match_strings(
                        &candidates,
                        query,
                        false,
                        true,
                        size,
                        &cancel,
                        background_executor.clone(),
                    ))
                })
            });

            let mut query_idx = 0usize;
            group.bench_function(BenchmarkId::new("fuzzy", size), |b| {
                b.iter(|| {
                    let query = queries[query_idx % queries.len()];
                    query_idx += 1;
                    foreground_executor.block_on(fuzzy::match_strings(
                        &fuzzy_candidates,
                        query,
                        false,
                        true,
                        size,
                        &cancel,
                        background_executor.clone(),
                    ))
                })
            });
        }
        group.finish();
    }
}

fn generate_path_strings(count: usize) -> &'static [String] {
    let paths: Box<[String]> = (0..count)
        .map(|id| {
            let dir = DIRS[id % DIRS.len()];
            let file = FILENAMES[id / DIRS.len() % FILENAMES.len()];
            format!("{dir}/{file}")
        })
        .collect();
    Box::leak(paths)
}

fn generate_nucleo_path_candidates(
    paths: &'static [String],
) -> Vec<fuzzy_nucleo::PathMatchCandidate<'static>> {
    paths
        .iter()
        .map(|path| fuzzy_nucleo::PathMatchCandidate {
            is_dir: false,
            path: RelPath::unix(path).unwrap(),
            char_bag: CharBag::from(path.as_str()),
        })
        .collect()
}

fn generate_fuzzy_path_candidates(
    paths: &'static [String],
) -> Vec<fuzzy::PathMatchCandidate<'static>> {
    paths
        .iter()
        .map(|path| fuzzy::PathMatchCandidate {
            is_dir: false,
            path: RelPath::unix(path).unwrap(),
            char_bag: CharBag::from(path.as_str()),
        })
        .collect()
}

fn bench_path_matching(criterion: &mut Criterion) {
    let sizes = [100, 1000, 10_000];
    let all_path_strings = sizes.map(generate_path_strings);
    let query_count = 200;
    let (single_queries, two_word_queries) = generate_queries(query_count);

    for (label, queries) in [
        (
            "path/1-word",
            single_queries
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<&str>>(),
        ),
        (
            "path/2-word",
            two_word_queries
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
        ),
    ] {
        let mut group = criterion.benchmark_group(label);
        for (size_index, &size) in sizes.iter().enumerate() {
            let path_strings = all_path_strings[size_index];

            let mut query_idx = 0usize;
            group.bench_function(BenchmarkId::new("nucleo", size), |b| {
                b.iter_batched(
                    || {
                        let query = queries[query_idx % queries.len()];
                        query_idx += 1;
                        (generate_nucleo_path_candidates(path_strings), query)
                    },
                    |(candidates, query)| {
                        fuzzy_nucleo::match_fixed_path_set(
                            candidates,
                            0,
                            None,
                            query,
                            false,
                            size,
                            PathStyle::Posix,
                        )
                    },
                    BatchSize::SmallInput,
                )
            });

            let mut query_idx = 0usize;
            group.bench_function(BenchmarkId::new("fuzzy", size), |b| {
                b.iter_batched(
                    || {
                        let query = queries[query_idx % queries.len()];
                        query_idx += 1;
                        (generate_fuzzy_path_candidates(path_strings), query)
                    },
                    |(candidates, query)| {
                        fuzzy::match_fixed_path_set(
                            candidates,
                            0,
                            None,
                            query,
                            false,
                            size,
                            PathStyle::Posix,
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
