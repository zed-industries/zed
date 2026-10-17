use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use fuzzy::CharBag;
use path::{PathStyle, rel_path::RelPath};
use std::sync::atomic::AtomicBool;

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

/// Deterministic query generation from QUERY_WORDS using a simple LCG.
/// Returns `count` queries of each arity: 1, 2, and 4 space-separated words.
fn generate_queries(count: usize) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut state: u64 = 0xDEAD_BEEF;
    let mut next = || -> usize {
        // LCG: simple, fast, deterministic
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (state >> 33) as usize
    };
    let mut n_word = |n: usize| -> Vec<String> {
        (0..count)
            .map(|_| {
                (0..n)
                    .map(|_| QUERY_WORDS[next() % QUERY_WORDS.len()])
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect()
    };

    (n_word(1), n_word(2), n_word(4))
}

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
        .map(|c| fuzzy::StringMatchCandidate::new(c.id, c.string.as_ref()))
        .collect()
}

fn bench_string_matching(criterion: &mut Criterion) {
    let cancel = AtomicBool::new(false);

    let dispatcher = std::sync::Arc::new(gpui::TestDispatcher::new(0));
    let background_executor = gpui::BackgroundExecutor::new(dispatcher.clone());
    let foreground_executor = gpui::ForegroundExecutor::new(dispatcher);

    let sizes = [100, 1000, 10_000];
    let query_count = 200;
    let (q1, q2, q4) = generate_queries(query_count);

    for (label, queries) in [("1-word", &q1), ("2-word", &q2), ("4-word", &q4)] {
        let mut group = criterion.benchmark_group(label);
        for size in sizes {
            let candidates = generate_candidates(size);
            let fuzzy_candidates = to_fuzzy_candidates(&candidates);

            let mut query_idx = 0usize;
            group.bench_function(BenchmarkId::new("nucleo", size), |b| {
                b.iter_batched(
                    || {
                        let query = queries[query_idx % queries.len()].as_str();
                        query_idx += 1;
                        query
                    },
                    |query| {
                        foreground_executor.block_on(fuzzy_nucleo::match_strings_async(
                            &candidates,
                            query,
                            fuzzy_nucleo::Case::Ignore,
                            fuzzy_nucleo::LengthPenalty::On,
                            size,
                            &cancel,
                            background_executor.clone(),
                        ))
                    },
                    BatchSize::SmallInput,
                )
            });

            let mut query_idx = 0usize;
            group.bench_function(BenchmarkId::new("fuzzy", size), |b| {
                b.iter_batched(
                    || {
                        let query = queries[query_idx % queries.len()].as_str();
                        query_idx += 1;
                        query
                    },
                    |query| {
                        foreground_executor.block_on(fuzzy::match_strings(
                            &fuzzy_candidates,
                            query,
                            false,
                            true,
                            size,
                            &cancel,
                            background_executor.clone(),
                        ))
                    },
                    BatchSize::SmallInput,
                )
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
        .map(|path| {
            fuzzy_nucleo::PathMatchCandidate::new(
                RelPath::from_unix_str(path).unwrap(),
                false,
                None,
            )
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
            path: RelPath::from_unix_str(path).unwrap(),
            char_bag: CharBag::from(path.as_str()),
        })
        .collect()
}

fn capitalize_each_word(query: &str) -> String {
    query
        .split_whitespace()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn bench_path_matching(criterion: &mut Criterion) {
    let sizes = [100, 1000, 10_000];
    let all_path_strings = sizes.map(generate_path_strings);
    let query_count = 200;
    let (q1, q2, q4) = generate_queries(query_count);
    let q1_upper: Vec<String> = q1.iter().map(|q| capitalize_each_word(q)).collect();
    let q2_upper: Vec<String> = q2.iter().map(|q| capitalize_each_word(q)).collect();
    let q4_upper: Vec<String> = q4.iter().map(|q| capitalize_each_word(q)).collect();

    for (label, queries, case) in [
        ("path/1-word", &q1, fuzzy_nucleo::Case::Ignore),
        ("path/2-word", &q2, fuzzy_nucleo::Case::Ignore),
        ("path/4-word", &q4, fuzzy_nucleo::Case::Ignore),
        ("path_smart/1-word", &q1_upper, fuzzy_nucleo::Case::Smart),
        ("path_smart/2-word", &q2_upper, fuzzy_nucleo::Case::Smart),
        ("path_smart/4-word", &q4_upper, fuzzy_nucleo::Case::Smart),
    ] {
        let mut group = criterion.benchmark_group(label);
        for (size_index, &size) in sizes.iter().enumerate() {
            let path_strings = all_path_strings[size_index];

            let mut query_idx = 0usize;
            group.bench_function(BenchmarkId::new("nucleo", size), |b| {
                b.iter_batched(
                    || {
                        let query = queries[query_idx % queries.len()].as_str();
                        query_idx += 1;
                        (generate_nucleo_path_candidates(path_strings), query)
                    },
                    |(candidates, query)| {
                        fuzzy_nucleo::match_fixed_path_set(
                            candidates,
                            0,
                            None,
                            query,
                            case,
                            size,
                            PathStyle::Unix,
                        )
                    },
                    BatchSize::SmallInput,
                )
            });

            let mut query_idx = 0usize;
            group.bench_function(BenchmarkId::new("fuzzy", size), |b| {
                b.iter_batched(
                    || {
                        let query = queries[query_idx % queries.len()].as_str();
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
                            PathStyle::Unix,
                        )
                    },
                    BatchSize::SmallInput,
                )
            });
        }
        group.finish();
    }
}

fn generate_unicode_path_strings(count: usize, mixed: bool, hangul: bool) -> &'static [String] {
    let paths: Box<[String]> = (0..count)
        .map(|index| {
            if hangul {
                format!("fixtures/{index}/\u{1112}\u{1161}\u{11ab}\u{1100}\u{1173}\u{11af}.md")
            } else if mixed {
                match index % 3 {
                    0 => format!("fixtures/{index}/plain.md"),
                    1 => format!("fixtures/{index}/grössen.md"),
                    _ => format!("fixtures/{index}/gro\u{308}ssen.md"),
                }
            } else if index % 100 == 0 {
                format!("fixtures/{index}/gro\u{308}ssen.md")
            } else {
                format!("fixtures/{index}/plain.md")
            }
        })
        .collect();
    Box::leak(paths)
}

fn generate_canonical_decoy_path_strings(count: usize) -> &'static [String] {
    let paths = (0..count)
        .map(|index| format!("fixtures/{index}/q\u{323}-q\u{307}.md"))
        .collect::<Vec<_>>()
        .into_boxed_slice();
    Box::leak(paths)
}

fn bench_unicode_path_matching(criterion: &mut Criterion) {
    const CANDIDATE_COUNT: usize = 10_000;
    let ascii_paths = generate_path_strings(CANDIDATE_COUNT);
    let predominantly_ascii_paths = generate_unicode_path_strings(CANDIDATE_COUNT, false, false);
    let mixed_paths = generate_unicode_path_strings(CANDIDATE_COUNT, true, false);
    let hangul_paths = generate_unicode_path_strings(CANDIDATE_COUNT, false, true);
    let canonical_decoy_paths = generate_canonical_decoy_path_strings(CANDIDATE_COUNT);
    let mut group = criterion.benchmark_group("path/unicode");

    for (name, paths, query) in [
        ("ascii_query_ascii_candidates", ascii_paths, "src"),
        (
            "nfc_query_predominantly_ascii_candidates",
            predominantly_ascii_paths,
            "grö",
        ),
        ("nfc_query_mixed_nfc_nfd_candidates", mixed_paths, "grö"),
        (
            "nfc_query_decomposed_hangul_candidates",
            hangul_paths,
            "한글",
        ),
        (
            "canonical_query_non_equivalent_decoys",
            canonical_decoy_paths,
            "q\u{307}",
        ),
    ] {
        group.bench_function(name, |benchmark| {
            benchmark.iter_batched(
                || generate_nucleo_path_candidates(paths),
                |candidates| {
                    fuzzy_nucleo::match_fixed_path_set(
                        candidates,
                        0,
                        None,
                        query,
                        fuzzy_nucleo::Case::Ignore,
                        CANDIDATE_COUNT,
                        PathStyle::Unix,
                    )
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_string_matching,
    bench_path_matching,
    bench_unicode_path_matching
);
criterion_main!(benches);
