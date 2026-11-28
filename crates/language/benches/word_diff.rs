use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use language::{DiffOptions, Language, LanguageConfig, word_diff_ranges};
use std::{hint::black_box, sync::Arc};

fn bench_word_diff_ranges(criterion: &mut Criterion) {
    let before = [
        "    let sample_rate = config.sample_rate.unwrap_or(44100);\n",
        "    let threshold = config.threshold.unwrap_or(0.25);\n",
        "    let window = Window::hann(config.frame_size);\n",
        "    let path = cache_dir.join(\"raw_samples.bin\");\n",
        "    let samples = decode(path, sample_rate, Format::S16)?;\n",
    ];
    let after = [
        "    let sample_rate = config.sample_rate.unwrap_or(48000);\n",
        "    let threshold = config.threshold.unwrap_or(0.15);\n",
        "    let window = Window::hamming(config.frame_size);\n",
        "    let path = cache_dir.join(\"filtered_samples.bin\");\n",
        "    let samples = decode(path, sample_rate, Format::F32)?;\n",
    ];
    let inserted = "    validate_config(&config)?;\n";
    let padding = "filter(samples, gain) + ".repeat(176);
    let long_lines = |offset: usize| {
        (0..5)
            .map(|row| format!("let value_{row} = {padding}mix({}, {row});\n", offset + row))
            .collect::<String>()
    };
    let fixtures = [
        ("1x1_modified", before[0].into(), after[0].into()),
        ("2x2_modified", before[..2].concat(), after[..2].concat()),
        ("5x5_modified", before.concat(), after.concat()),
        (
            "1x2_insertion",
            before[0].into(),
            [inserted, after[0]].concat(),
        ),
        (
            "4x5_insertion",
            before[..4].concat(),
            [after[0], inserted, after[1], after[2], after[3]].concat(),
        ),
        (
            "5x5_shifted",
            "xx aa\nxx bb\nxx cc\nxx dd\nxx ee\n".into(),
            "xx bb\nxx cc\nxx dd\nxx ee\nxx ff\n".into(),
        ),
        ("5x5_long_lines", long_lines(0), long_lines(10)),
    ];
    let scope = Arc::new(Language::new(LanguageConfig::default(), None)).default_scope();
    let mut group = criterion.benchmark_group("word_diff_ranges");
    for (name, before, after) in fixtures {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &(before, after),
            |bencher, (before, after)| {
                bencher.iter(|| {
                    word_diff_ranges(
                        black_box(before),
                        black_box(after),
                        DiffOptions {
                            language_scope: Some(scope.clone()),
                            ..Default::default()
                        },
                    )
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_word_diff_ranges);
criterion_main!(benches);
