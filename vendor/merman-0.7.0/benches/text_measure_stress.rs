use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use merman_render::text::{TextMeasurer, TextStyle, VendoredFontMetricsTextMeasurer, WrapMode};
use std::hint::black_box;

const FLOWCHART_FONT_FAMILY: &str = "\"trebuchet ms\", verdana, arial, sans-serif";

const TEXT_CASES: &[(&str, &str, Option<f64>)] = &[
    ("node_label", "Node Label", Some(200.0)),
    ("edge_label", "Edge Label", Some(200.0)),
    ("subgraph_title", "Subgraph Title", Some(200.0)),
    (
        "wrapped_cluster_title",
        "A very long cluster title with punctuation: (a/b/c)",
        Some(120.0),
    ),
    ("special_characters", "special characters", Some(200.0)),
];

fn flowchart_style(font_weight: Option<&str>) -> TextStyle {
    TextStyle {
        font_family: Some(FLOWCHART_FONT_FAMILY.to_string()),
        font_size: 16.0,
        font_weight: font_weight.map(str::to_string),
    }
}

fn bench_text_measure_stress(c: &mut Criterion) {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let styles = [
        ("plain", flowchart_style(None)),
        ("bold", flowchart_style(Some("bold"))),
    ];

    let mut group = c.benchmark_group("text_measure_stress");
    group.sample_size(50);

    for (style_name, style) in &styles {
        for &(case_name, text, wrap_width) in TEXT_CASES {
            group.bench_function(
                BenchmarkId::new(format!("computed_length_{style_name}"), case_name),
                |b| {
                    b.iter(|| {
                        black_box(measurer.measure_svg_text_computed_length_px(
                            black_box(text),
                            black_box(style),
                        ));
                    });
                },
            );

            group.bench_function(
                BenchmarkId::new(format!("wrapped_svg_like_{style_name}"), case_name),
                |b| {
                    b.iter(|| {
                        black_box(measurer.measure_wrapped(
                            black_box(text),
                            black_box(style),
                            black_box(wrap_width),
                            WrapMode::SvgLike,
                        ));
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(benches, bench_text_measure_stress);
criterion_main!(benches);
