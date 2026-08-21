use benchmarks::bench_utils::RandomCharIter;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use editor::{EditorStyle, MultiBuffer, display_map::*};
use gpui::{AppContext as _, BenchAppContext, HighlightStyle, Hsla, font, px};
use multi_buffer::MultiBufferOffset;
use project::project_settings::DiagnosticSeverity;
use rand::{Rng, SeedableRng, rngs::StdRng};
use settings::SettingsStore;
use std::{num::NonZeroU32, time::Duration};
use text::Bias;

/// Builds a benchmark app context backed by the same real headless platform
/// `#[gpui::bench]` uses. The benchmarks below are plain `criterion_group!`
/// functions rather than `#[gpui::bench]` because they need Criterion
/// features the macro doesn't expose (a custom `measurement_time`, and
/// several named benchmarks built from one fixture-building loop), but they
/// must stay just as production-capable, rather than falling back to
/// `TestAppContext`/`TestDispatcher`.
fn bench_app_context<'a, 'measurement>(
    name: &'static str,
    bencher: &'a mut criterion::Bencher<'measurement>,
) -> BenchAppContext<'a, 'measurement> {
    BenchAppContext::new(
        gpui::bench_platform(
            Some(Box::new(|| gpui_platform::current_headless_renderer())),
            gpui_platform::current_platform(true).text_system(),
        ),
        Some(name),
        bencher,
    )
}

fn to_tab_point_benchmark(c: &mut Criterion) {
    const LENGTH: usize = 1024;

    let mut group = c.benchmark_group("To tab point");
    group.bench_with_input(
        BenchmarkId::new("to_tab_point", LENGTH),
        &LENGTH,
        |bencher, &length| {
            let mut cx = bench_app_context("to_tab_point_benchmark", bencher);

            let mut rng = StdRng::seed_from_u64(1);
            let text = RandomCharIter::new(&mut rng)
                .take(length)
                .collect::<String>();
            let buffer = cx.update(|cx| MultiBuffer::build_simple_for_benchmarks(&text, cx));

            let buffer_snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
            let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
            let (_, fold_snapshot) = FoldMap::new(inlay_snapshot.clone());
            let fold_point = fold_snapshot.to_fold_point(
                inlay_snapshot.to_point(InlayOffset(
                    rng.random_range(MultiBufferOffset(0)..MultiBufferOffset(length)),
                )),
                Bias::Left,
            );
            let (_, snapshot) = TabMap::new(fold_snapshot, NonZeroU32::new(4).unwrap());

            cx.bench_iter(|_| {
                snapshot.fold_point_to_tab_point(fold_point);
            });
            cx.teardown();
        },
    );

    group.finish();
}

fn to_fold_point_benchmark(c: &mut Criterion) {
    const LENGTH: usize = 1024;

    let mut group = c.benchmark_group("To fold point");
    group.bench_with_input(
        BenchmarkId::new("to_fold_point", LENGTH),
        &LENGTH,
        |bencher, &length| {
            let mut cx = bench_app_context("to_fold_point_benchmark", bencher);

            let mut rng = StdRng::seed_from_u64(1);
            let text = RandomCharIter::new(&mut rng)
                .take(length)
                .collect::<String>();
            let buffer = cx.update(|cx| MultiBuffer::build_simple_for_benchmarks(&text, cx));

            let buffer_snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
            let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
            let (_, fold_snapshot) = FoldMap::new(inlay_snapshot.clone());

            let fold_point = fold_snapshot.to_fold_point(
                inlay_snapshot.to_point(InlayOffset(
                    rng.random_range(MultiBufferOffset(0)..MultiBufferOffset(length)),
                )),
                Bias::Left,
            );

            let (_, snapshot) = TabMap::new(fold_snapshot, NonZeroU32::new(4).unwrap());
            let tab_point = snapshot.fold_point_to_tab_point(fold_point);

            cx.bench_iter(|_| {
                snapshot.tab_point_to_fold_point(tab_point, Bias::Left);
            });
            cx.teardown();
        },
    );

    group.finish();
}

fn create_highlight_endpoints_benchmark(c: &mut Criterion) {
    const LINE_COUNT: usize = 20_000;
    const LINE_VIEW_PORT_COUNT: usize = 100;
    const HIGHLIGHTS_PER_LINE: usize = 4;

    let mut group = c.benchmark_group("Create highlight endpoints");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(10));
    group.bench_with_input(
        BenchmarkId::new("text_highlights", LINE_VIEW_PORT_COUNT),
        &LINE_VIEW_PORT_COUNT,
        |bencher, _| {
            let mut cx = bench_app_context("create_highlight_endpoints_benchmark", bencher);
            cx.update(|cx| {
                let store = SettingsStore::benchmarks(cx);
                cx.set_global(store);
                editor::init(cx);
            });

            let mut text = String::new();
            let mut highlight_ranges = Vec::with_capacity(LINE_COUNT * HIGHLIGHTS_PER_LINE);
            for line in 0..LINE_COUNT {
                text.push_str("fn item_");
                text.push_str(&format!("{line:05}"));
                text.push_str("() { ");

                let start = text.len();
                text.push_str("alpha_highlight");
                highlight_ranges.push(MultiBufferOffset(start)..MultiBufferOffset(text.len()));

                text.push_str(" + ");
                let start = text.len();
                text.push_str("beta_highlight");
                highlight_ranges.push(MultiBufferOffset(start)..MultiBufferOffset(text.len()));

                text.push_str(" + ");
                let start = text.len();
                text.push_str("gamma_highlight");
                highlight_ranges.push(MultiBufferOffset(start)..MultiBufferOffset(text.len()));

                text.push_str(" + ");
                let start = text.len();
                text.push_str("delta_highlight");
                highlight_ranges.push(MultiBufferOffset(start)..MultiBufferOffset(text.len()));

                text.push_str("; }\n");
            }

            let buffer = cx.update(|cx| MultiBuffer::build_simple_for_benchmarks(&text, cx));
            let buffer_snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
            let highlight_ranges = highlight_ranges
                .into_iter()
                .map(|range| {
                    buffer_snapshot.anchor_before(range.start)
                        ..buffer_snapshot.anchor_before(range.end)
                })
                .collect();

            let map = cx.new(|cx| {
                DisplayMap::new(
                    buffer,
                    font("Courier"),
                    px(16.0),
                    None,
                    1,
                    1,
                    FoldPlaceholder::default(),
                    DiagnosticSeverity::Warning,
                    cx,
                )
            });
            cx.update(|cx| {
                map.update(cx, |map, cx| {
                    map.highlight_text(
                        HighlightKey::Editor,
                        highlight_ranges,
                        HighlightStyle {
                            color: Some(Hsla::blue()),
                            ..Default::default()
                        },
                        false,
                        cx,
                    );
                });
            });
            let snapshot = cx.update(|cx| map.update(cx, |map, cx| map.snapshot(cx)));

            cx.bench_iter(|_| {
                black_box(snapshot.chunks(
                    DisplayRow(400)..DisplayRow(400 + LINE_VIEW_PORT_COUNT as u32),
                    language::LanguageAwareStyling {
                        tree_sitter: false,
                        diagnostics: false,
                    },
                    Default::default(),
                ));
            });
            cx.teardown();
        },
    );
    group.finish();
}

fn highlighted_chunks_benchmark(c: &mut Criterion) {
    const LINE_COUNT: usize = 500;

    let corpora = [
        (
            "ascii",
            "    let chunks = snapshot.highlighted_chunks(rows.clone(), language_aware, style);",
        ),
        (
            "unicode",
            "の設定を変更する — émojis 🧑‍✈️ und Ümläute überall, здесь тоже текст",
        ),
        (
            "sparse_invisibles",
            "normal text here\u{200b}and some more text that goes on for a while without issues",
        ),
        (
            "dense_invisibles",
            "a\u{200b}b\u{ad}c\u{2060}d\u{feff}e\u{200b}f\u{ad}g\u{2060}h",
        ),
    ];

    let mut group = c.benchmark_group("Highlighted chunks");
    for (name, line) in corpora {
        let text = std::iter::repeat_n(line, LINE_COUNT)
            .collect::<Vec<_>>()
            .join("\n");
        group.bench_with_input(
            BenchmarkId::new("highlighted_chunks", name),
            &text,
            |bencher, text| {
                let mut cx = bench_app_context("highlighted_chunks_benchmark", bencher);
                cx.update(|cx| {
                    let store = SettingsStore::benchmarks(cx);
                    cx.set_global(store);
                    editor::init(cx);
                });

                let buffer = cx.update(|cx| MultiBuffer::build_simple_for_benchmarks(text, cx));
                let map = cx.new(|cx| {
                    DisplayMap::new(
                        buffer,
                        font("Courier"),
                        px(16.0),
                        None,
                        1,
                        1,
                        FoldPlaceholder::default(),
                        DiagnosticSeverity::Warning,
                        cx,
                    )
                });
                let snapshot = cx.update(|cx| map.update(cx, |map, cx| map.snapshot(cx)));
                let editor_style = EditorStyle::default();

                cx.bench_iter(|_| {
                    let mut total_len = 0usize;
                    let chunks = snapshot.highlighted_chunks(
                        DisplayRow(0)..DisplayRow(LINE_COUNT as u32),
                        language::LanguageAwareStyling {
                            tree_sitter: false,
                            diagnostics: false,
                        },
                        &editor_style,
                    );
                    for chunk in chunks {
                        total_len += black_box(chunk.text).len();
                    }
                    black_box(total_len);
                });
                cx.teardown();
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    to_tab_point_benchmark,
    to_fold_point_benchmark,
    create_highlight_endpoints_benchmark,
    highlighted_chunks_benchmark
);
criterion_main!(benches);
