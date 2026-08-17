mod support;

use merman::render::HeadlessRenderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = support::read_mermaid_or_default(
        "example_03_layout_json",
        "flowchart TD\n  A[Parse] --> B[Layout]\n  B --> C[Geometry]\n",
    )?;

    // Layout JSON exposes computed geometry and routes before SVG emission.
    let renderer = HeadlessRenderer::new().with_strict_parsing();
    let Some(layout) = renderer.layout_diagram_sync(&input)? else {
        return Err("no Mermaid diagram detected".into());
    };

    println!("{}", serde_json::to_string_pretty(&layout)?);
    Ok(())
}
