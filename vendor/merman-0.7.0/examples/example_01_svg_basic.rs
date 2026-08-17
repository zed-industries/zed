mod support;

use merman::render::HeadlessRenderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = support::read_mermaid_or_default(
        "example_01_svg_basic",
        r#"flowchart TD
    A[Mermaid source] --> B[HeadlessRenderer]
    B --> C[SVG string]
"#,
    )?;

    // Strict parsing surfaces malformed input early; the stable id helps when inlining the SVG.
    let renderer = HeadlessRenderer::new()
        .with_strict_parsing()
        .with_diagram_id("svg-basic-example");
    let Some(svg) = renderer.render_svg_sync(&input)? else {
        return Err("no Mermaid diagram detected".into());
    };

    print!("{svg}");
    Ok(())
}
