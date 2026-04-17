use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use fuzzy::CharBag;
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
            fuzzy_nucleo::PathMatchCandidate::new(RelPath::unix(path).unwrap(), false, None)
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

criterion_group!(benches, bench_path_matching);
criterion_main!(benches);
