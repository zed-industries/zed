use criterion::{Criterion, black_box, criterion_group, criterion_main};

const LANGUAGE_NAMES: &[&str] = &[
    "bash",
    "c",
    "cpp",
    "css",
    "diff",
    "gitcommit",
    "go",
    "gomod",
    "gowork",
    "javascript",
    "jsdoc",
    "json",
    "jsonc",
    "markdown",
    "markdown-inline",
    "python",
    "regex",
    "rust",
    "tsx",
    "typescript",
    "yaml",
];

fn load_queries(criterion: &mut Criterion) {
    criterion.bench_function("load_queries/rust", |bencher| {
        bencher.iter(|| grammars::load_queries(black_box("rust")));
    });

    criterion.bench_function("load_queries/all", |bencher| {
        bencher.iter(|| {
            for language_name in black_box(LANGUAGE_NAMES) {
                black_box(grammars::load_queries(language_name));
            }
        });
    });
}

criterion_group!(benches, load_queries);
criterion_main!(benches);
