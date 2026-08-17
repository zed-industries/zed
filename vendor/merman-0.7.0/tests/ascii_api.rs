#![cfg(feature = "ascii")]

use merman::RenderSemanticModel;
use merman::ascii::{
    AsciiRenderOptions, HeadlessAsciiRenderer, render_ascii_sync, render_class, render_er,
    render_model, render_xychart,
};

fn render_model_for(source: &str) -> RenderSemanticModel {
    merman::Engine::new()
        .parse_diagram_for_render_model_sync(source, merman::ParseOptions::strict())
        .unwrap()
        .unwrap()
        .model
}

fn deeply_nested_flowchart(depth: usize) -> String {
    let mut lines = vec!["flowchart TB".to_string()];
    for i in 0..depth {
        lines.push(format!("subgraph n{i}"));
    }
    lines.push("A".to_string());
    for _ in 0..depth {
        lines.push("end".to_string());
    }
    lines.join("\n")
}

#[test]
fn render_ascii_sync_renders_flowchart_from_mermaid_text() {
    let engine = merman::Engine::new();
    let rendered = render_ascii_sync(
        &engine,
        "flowchart LR\nA --> B",
        merman::ParseOptions::strict(),
        &AsciiRenderOptions::ascii(),
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        rendered,
        "+---+     +---+\n|   |     |   |\n| A |---->| B |\n|   |     |   |\n+---+     +---+\n"
    );
}

#[test]
fn render_ascii_sync_renders_shipped_reference_diagram_families() {
    let engine = merman::Engine::new();
    let cases = [
        ("classDiagram\nclass Animal", "Animal"),
        ("erDiagram\nCUSTOMER", "CUSTOMER"),
        (
            r#"xychart
title "Sales"
x-axis [Jan, Feb]
y-axis 0 --> 10
bar [2, 8]
"#,
            "Sales",
        ),
    ];

    for (source, expected) in cases {
        let rendered = render_ascii_sync(
            &engine,
            source,
            merman::ParseOptions::strict(),
            &AsciiRenderOptions::ascii(),
        )
        .unwrap()
        .unwrap();

        assert!(
            rendered.contains(expected),
            "expected {expected:?} in rendered output:\n{rendered}"
        );
    }
}

#[test]
fn direct_ascii_exports_render_shipped_typed_models() {
    let options = AsciiRenderOptions::ascii();

    let RenderSemanticModel::Class(class_model) = render_model_for("classDiagram\nclass Animal")
    else {
        panic!("expected class render model");
    };
    let rendered = render_class(&class_model, &options).unwrap();
    assert!(rendered.contains("Animal"));

    let RenderSemanticModel::Er(er_model) = render_model_for("erDiagram\nCUSTOMER") else {
        panic!("expected ER render model");
    };
    let rendered = render_er(&er_model, &options).unwrap();
    assert!(rendered.contains("CUSTOMER"));

    let RenderSemanticModel::XyChart(xychart_model) = render_model_for(
        r#"xychart
x-axis [A, B]
y-axis 0 --> 10
bar [4, 8]
"#,
    ) else {
        panic!("expected XYChart render model");
    };
    let rendered = render_xychart(&xychart_model, &options).unwrap();
    assert!(rendered.contains("###"));
}

#[test]
fn render_ascii_sync_applies_mermaid_ascii_padding_directives() {
    let engine = merman::Engine::new();
    let rendered = render_ascii_sync(
        &engine,
        "paddingX=2\npaddingY=1\ngraph LR\nA --> B",
        merman::ParseOptions::strict(),
        &AsciiRenderOptions::ascii(),
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        rendered,
        "+---+  +---+\n|   |  |   |\n| A |->| B |\n|   |  |   |\n+---+  +---+\n"
    );
}

#[test]
fn headless_ascii_renderer_renders_sequence_with_unicode_defaults() {
    let renderer = HeadlessAsciiRenderer::new().with_strict_parsing();
    let rendered = renderer
        .render_ascii_sync("sequenceDiagram\nparticipant A\nparticipant B\nA->>B: Hello")
        .unwrap()
        .unwrap();

    assert!(rendered.contains("┌"));
    assert!(rendered.contains("Hello"));
    assert!(rendered.contains("►"));
}

#[test]
fn render_ascii_sync_returns_none_when_no_diagram_is_detected() {
    let engine = merman::Engine::new();
    let rendered = render_ascii_sync(
        &engine,
        "this is just prose",
        merman::ParseOptions::lenient(),
        &AsciiRenderOptions::default(),
    )
    .unwrap();

    assert!(rendered.is_none());
}

#[test]
fn render_ascii_model_handles_deep_flowchart_subgraph_chain_with_small_stack() {
    const DEPTH: usize = 512;
    let source = deeply_nested_flowchart(DEPTH);
    let model = render_model_for(&source);
    let handle = std::thread::Builder::new()
        .name("ascii-deep-flowchart-subgraph".to_string())
        .stack_size(64 * 1024)
        .spawn(move || {
            let mut options = AsciiRenderOptions::ascii();
            options.max_grid_cells = 10_000_000;
            let rendered = render_model(&model, &options)
                .expect("deep Flowchart ASCII render should not return an error");
            assert!(rendered.contains('A'));
        })
        .expect("spawn deep Flowchart ASCII render test");
    handle
        .join()
        .expect("deep Flowchart ASCII render should not overflow the stack");
}
