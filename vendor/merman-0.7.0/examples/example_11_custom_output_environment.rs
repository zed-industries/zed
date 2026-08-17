mod support;

use merman::MermaidConfig;
use merman::render::{
    CssOverridePostprocessor, HeadlessRenderer, RootBackgroundPostprocessor,
    ScopedCssPostprocessor, SvgPipeline,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = support::read_mermaid_or_default(
        "example_11_custom_output_environment",
        r#"flowchart TD
    Host[Editor host] --> Renderer[HeadlessRenderer]
    Renderer --> Svg[resvg-safe SVG]
    Svg --> Preview[Preview surface]
"#,
    )?;

    // Host-owned Mermaid defaults stay outside the diagram source.
    let site_config = MermaidConfig::from_value(serde_json::json!({
        "theme": "base",
        "darkMode": true,
        "fontFamily": "system-ui",
        "themeCSS": ".node rect { stroke-width: 4px !important; }",
        "themeVariables": {
            "background": "#111827",
            "mainBkg": "#1f2937",
            "primaryColor": "#1f2937",
            "primaryTextColor": "#e5e7eb",
            "primaryBorderColor": "#64748b",
            "lineColor": "#94a3b8",
            "textColor": "#e5e7eb",
            "edgeLabelBackground": "#111827",
            "fontFamily": "system-ui"
        }
    }));

    // Keep host CSS scoped to the configured root SVG id.
    let host_css = r#"
.node rect,
.node polygon,
.node path {
  stroke: #38bdf8;
  stroke-width: 2px;
}

.edgePath .path {
  stroke: #94a3b8;
}

.merman-foreignobject-fallback-text {
  fill: #e5e7eb;
}
"#;

    // Stable ids and vendored text metrics make embedded previews deterministic across machines.
    let renderer = HeadlessRenderer::new()
        .with_site_config(site_config)
        .with_vendored_text_measurer()
        .with_diagram_id("custom-output-environment-example");
    // The host pipeline is explicit so default rendering remains Mermaid-parity oriented.
    let pipeline = SvgPipeline::resvg_safe()
        .with_postprocessor(CssOverridePostprocessor::strip_existing_important())
        .with_postprocessor(RootBackgroundPostprocessor::new("#111827"))
        .with_postprocessor(ScopedCssPostprocessor::new(host_css));

    let Some(svg) = renderer.render_svg_with_pipeline_sync(&input, &pipeline)? else {
        return Err("no Mermaid diagram detected".into());
    };

    // Example-level contract check: this output should be safe for resvg-like preview surfaces.
    if svg.contains("<foreignObject") || svg.contains("!important") {
        return Err(
            "custom output environment should produce raster-safe host-controlled SVG".into(),
        );
    }

    print!("{svg}");
    Ok(())
}
