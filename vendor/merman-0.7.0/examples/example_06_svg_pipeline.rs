mod support;

use merman::render::{
    CssOverridePolicy, HeadlessRenderer, RenderResult, ScopedCssPostprocessor, SvgPipeline,
    SvgPostprocessContext, SvgPostprocessor,
};
use std::borrow::Cow;

// Custom postprocessors can observe diagram metadata and append host-owned SVG nodes.
struct AddExampleMetadata;

impl SvgPostprocessor for AddExampleMetadata {
    fn name(&self) -> &'static str {
        "add-example-metadata"
    }

    fn process<'a>(
        &self,
        svg: Cow<'a, str>,
        ctx: &SvgPostprocessContext<'_>,
    ) -> RenderResult<Cow<'a, str>> {
        let metadata = format!(
            r#"<metadata data-merman-example="example_06_svg_pipeline" data-preset="{:?}" data-diagram-type="{}" data-title="{}" data-svg-id="{}"/>"#,
            ctx.preset(),
            escape_attr(ctx.diagram_type().unwrap_or("")),
            escape_attr(ctx.diagram_title().unwrap_or("")),
            escape_attr(ctx.svg_id().unwrap_or(""))
        );

        let Some(idx) = svg.rfind("</svg>") else {
            return Ok(Cow::Owned(format!("{svg}{metadata}")));
        };

        Ok(Cow::Owned(format!(
            "{}{}{}",
            &svg[..idx],
            metadata,
            &svg[idx..]
        )))
    }
}

fn escape_attr(value: &str) -> String {
    // The metadata element is assembled as XML text, so context values must be escaped.
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = support::read_mermaid_or_default(
        "example_06_svg_pipeline",
        r#"flowchart TD
    L7["Layer 7\nHTTP"]
    L6["Layer 6\nEncryption"]
    L7 --> L6
"#,
    )?;

    let renderer = HeadlessRenderer::new().with_diagram_id("svg-pipeline-example");
    let host_css = r#"
.node rect {
  stroke: #2563eb;
  stroke-width: 2px;
}
.merman-foreignobject-fallback-text {
  fill: #111827;
}
"#;
    // Built-in resvg-safe cleanup runs before host styling and custom metadata passes.
    let pipeline = SvgPipeline::resvg_safe()
        .with_postprocessor(
            ScopedCssPostprocessor::new(host_css)
                .with_override_policy(CssOverridePolicy::StripExistingImportant),
        )
        .with_postprocessor(AddExampleMetadata);
    let Some(svg) = renderer.render_svg_with_pipeline_sync(&input, &pipeline)? else {
        return Err("no Mermaid diagram detected".into());
    };

    print!("{svg}");
    Ok(())
}
