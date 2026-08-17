#[cfg(feature = "render")]
#[test]
fn xychart_render_svg_sync_uses_typed_render_path() {
    let engine = merman_core::Engine::new();
    let parse_options = merman_core::ParseOptions::strict();
    let layout = merman::render::LayoutOptions {
        viewport_width: 800.0,
        viewport_height: 600.0,
        text_measurer: std::sync::Arc::new(
            merman::render::VendoredFontMetricsTextMeasurer::default(),
        ),
        math_renderer: None,
        use_manatee_layout: true,
    };
    let svg_opts = merman::render::SvgRenderOptions {
        diagram_id: Some("typed_xychart".to_string()),
        ..Default::default()
    };
    let input = r#"
xychart
title "Typed XYChart"
x-axis [A, B]
y-axis 1 --> 3
bar [1, 2]
"#;

    let svg = merman::render::render_svg_sync(&engine, input, parse_options, &layout, &svg_opts)
        .expect("render svg")
        .expect("diagram detected");

    assert!(svg.contains("typed_xychart"));
    assert!(svg.contains("xychart"));
}
