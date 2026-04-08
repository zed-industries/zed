use std::sync::atomic::AtomicBool;
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use fuzzy::CharBag;
use util::{paths::PathStyle, rel_path::RelPath};

fn generate_candidates(count: usize) -> Vec<fuzzy_nucleo::StringMatchCandidate> {
    let sample_strings = [
        "src/lib/parser.rs",
        "src/bin/main.rs",
        "tests/parser_test.rs",
        "crates/gpui/src/executor.rs",
        "crates/editor/src/editor.rs",
        "crates/fuzzy_nucleo/src/strings.rs",
        "README.md",
        "Cargo.toml",
        "src/components/sidebar/panel.tsx",
        "src/utils/string_helpers.rs",
        "crates/workspace/src/workspace.rs",
        "crates/project/src/project.rs",
        "docs/architecture/overview.md",
        "scripts/build.sh",
        "src/theme/colors.rs",
        "crates/language/src/buffer.rs",
    ];
    (0..count)
        .map(|id| {
            let base = &sample_strings[id % sample_strings.len()];
            if id < sample_strings.len() {
                fuzzy_nucleo::StringMatchCandidate::new(id, base)
            } else {
                fuzzy_nucleo::StringMatchCandidate::new(id, &format!("{base}/{id}"))
            }
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

const PATH_SAMPLES: &[&str] = &[
    "src/lib/parser.rs",
    "src/bin/main.rs",
    "tests/parser_test.rs",
    "crates/gpui/src/executor.rs",
    "crates/editor/src/editor.rs",
    "crates/fuzzy_nucleo/src/strings.rs",
    "README.md",
    "Cargo.toml",
    "src/components/sidebar/panel.tsx",
    "src/utils/string_helpers.rs",
    "crates/workspace/src/workspace.rs",
    "crates/project/src/project.rs",
    "docs/architecture/overview.md",
    "scripts/build.sh",
    "src/theme/colors.rs",
    "crates/language/src/buffer.rs",
];

fn generate_nucleo_path_candidates(count: usize) -> Vec<fuzzy_nucleo::PathMatchCandidate<'static>> {
    (0..count)
        .map(|id| {
            let path = PATH_SAMPLES[id % PATH_SAMPLES.len()];
            fuzzy_nucleo::PathMatchCandidate {
                is_dir: false,
                path: RelPath::unix(path).unwrap(),
            }
        })
        .collect()
}

fn generate_fuzzy_path_candidates(count: usize) -> Vec<fuzzy::PathMatchCandidate<'static>> {
    (0..count)
        .map(|id| {
            let path = PATH_SAMPLES[id % PATH_SAMPLES.len()];
            fuzzy::PathMatchCandidate {
                is_dir: false,
                path: RelPath::unix(path).unwrap(),
                char_bag: CharBag::from(path),
            }
        })
        .collect()
}

fn bench_path_matching(criterion: &mut Criterion) {
    let sizes = [100, 1000, 10_000];
    let queries = ["par", "src editor", "executor.rs"];

    for query in queries {
        let mut group = criterion.benchmark_group(format!("path/{query}"));
        for size in sizes {
            group.bench_function(BenchmarkId::new("nucleo", size), |b| {
                b.iter_batched(
                    || generate_nucleo_path_candidates(size),
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
                    || generate_fuzzy_path_candidates(size),
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
