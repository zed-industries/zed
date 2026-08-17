#![cfg(feature = "render")]

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

fn assert_no_empty_style_elements(name: &str, svg: &str) {
    let mut cursor = 0;
    while let Some(rel_start) = svg[cursor..].find("<style") {
        let tag_start = cursor + rel_start;
        let Some(rel_tag_end) = svg[tag_start..].find('>') else {
            panic!("{name}: malformed style start tag");
        };
        let content_start = tag_start + rel_tag_end + 1;
        let Some(rel_close) = svg[content_start..].find("</style>") else {
            panic!("{name}: malformed style element");
        };
        let content_end = content_start + rel_close;
        assert!(
            !svg[content_start..content_end].trim().is_empty(),
            "{name}: rendered SVG contains an empty <style> element"
        );
        cursor = content_end + "</style>".len();
    }
}

#[test]
fn pipeline_bench_fixtures_are_benchmarkable() {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("benches")
        .join("fixtures");
    let mut fixtures = fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("read {}: {err}", fixtures_dir.display()))
        .map(|entry| {
            entry
                .unwrap_or_else(|err| panic!("read {} entry: {err}", fixtures_dir.display()))
                .path()
        })
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("mmd"))
        .collect::<Vec<_>>();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "no pipeline bench fixtures found");

    let engine = merman_core::Engine::new();
    let parse_options = merman_core::ParseOptions::strict();
    let layout = merman::render::LayoutOptions {
        viewport_width: 800.0,
        viewport_height: 600.0,
        text_measurer: Arc::new(merman::render::VendoredFontMetricsTextMeasurer::default()),
        math_renderer: None,
        use_manatee_layout: true,
    };

    for path in fixtures {
        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("<invalid fixture name>")
            .to_string();
        let input = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("{name}: read {}: {err}", path.display()));

        let metadata = engine
            .parse_metadata_sync(&input, parse_options)
            .unwrap_or_else(|err| panic!("{name}: metadata parse failed: {err}"))
            .unwrap_or_else(|| panic!("{name}: metadata parser returned no diagram"));

        engine
            .parse_diagram_with_type_sync(&metadata.diagram_type, &input, parse_options)
            .unwrap_or_else(|err| {
                panic!(
                    "{name}: known-type parse failed for {}: {err}",
                    metadata.diagram_type
                )
            });

        let parsed = engine
            .parse_diagram_for_render_model_sync(&input, parse_options)
            .unwrap_or_else(|err| panic!("{name}: render-model parse failed: {err}"))
            .unwrap_or_else(|| panic!("{name}: render-model parser returned no diagram"));

        let svg_options = merman::render::SvgRenderOptions {
            diagram_id: Some(merman::render::sanitize_svg_id(&name)),
            ..Default::default()
        };
        let svg =
            merman::render::render_svg_sync(&engine, &input, parse_options, &layout, &svg_options)
                .unwrap_or_else(|err| panic!("{name}: end-to-end SVG render failed: {err}"))
                .unwrap_or_else(|| panic!("{name}: render returned no SVG"));

        assert!(
            !svg.is_empty(),
            "{name}: render returned an empty SVG for {:?}",
            parsed.meta.diagram_type
        );
        assert_no_empty_style_elements(&name, &svg);
    }
}
