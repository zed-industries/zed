//! Benchmarks for the brush-parser crate.

#![allow(missing_docs)]

#[cfg(unix)]
mod unix {
    use brush_parser::{Token, parse_tokens};
    use criterion::Criterion;

    fn uncached_tokenize(content: &str) -> Vec<brush_parser::Token> {
        brush_parser::uncached_tokenize_str(content, &brush_parser::TokenizerOptions::default())
            .unwrap()
    }

    fn cacheable_tokenize(content: &str) -> Vec<brush_parser::Token> {
        brush_parser::tokenize_str_with_options(content, &brush_parser::TokenizerOptions::default())
            .unwrap()
    }

    fn parse(tokens: &Vec<Token>) -> brush_parser::ast::Program {
        parse_tokens(
            tokens,
            &brush_parser::ParserOptions::default(),
            &brush_parser::SourceInfo::default(),
        )
        .unwrap()
    }

    const SAMPLE_SCRIPT: &str = r#"
for f in A B C; do
    echo "${f@L}" >&2
done
"#;

    fn benchmark_parsing_script_using_caches(c: &mut Criterion, script_path: &std::path::Path) {
        let contents = std::fs::read_to_string(script_path).unwrap();

        c.bench_function(
            std::format!(
                "parse_{}",
                script_path.file_name().unwrap().to_string_lossy()
            )
            .as_str(),
            |b| b.iter(|| parse(&cacheable_tokenize(contents.as_str()))),
        );
    }

    pub(crate) fn criterion_benchmark(c: &mut Criterion) {
        const POSSIBLE_BASH_COMPLETION_SCRIPT_PATH: &str =
            "/usr/share/bash-completion/bash_completion";

        c.bench_function("tokenize_sample_script", |b| {
            b.iter(|| uncached_tokenize(SAMPLE_SCRIPT));
        });

        let tokens = uncached_tokenize(SAMPLE_SCRIPT);
        c.bench_function("parse_sample_script", |b| b.iter(|| parse(&tokens)));

        let well_known_complicated_script =
            std::path::PathBuf::from(POSSIBLE_BASH_COMPLETION_SCRIPT_PATH);

        if well_known_complicated_script.exists() {
            benchmark_parsing_script_using_caches(c, &well_known_complicated_script);
        }
    }
}

#[cfg(unix)]
criterion::criterion_group! {
    name = benches;
    config = criterion::Criterion::default().with_profiler(pprof::criterion::PProfProfiler::new(100, pprof::criterion::Output::Flamegraph(None)));
    targets = unix::criterion_benchmark
}

#[cfg(unix)]
criterion::criterion_main!(benches);

#[cfg(not(unix))]
fn main() {}
