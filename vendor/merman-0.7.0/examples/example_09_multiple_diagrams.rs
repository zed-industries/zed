use merman::render::HeadlessRenderer;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let diagrams = [
        (
            "host-flow",
            "flowchart TD\n  A[Editor] --> B[Preview]\n  B --> C[Export]\n",
        ),
        (
            "host-sequence",
            "sequenceDiagram\n  participant UI\n  participant Engine\n  UI->>Engine: render\n  Engine-->>UI: svg\n",
        ),
    ];

    // Caller-owned ids keep root SVG and internal definition ids distinct across diagrams.
    let out_dir = Path::new("target/merman-multiple-diagrams");
    fs::create_dir_all(out_dir)?;

    for (diagram_id, source) in diagrams {
        let renderer = HeadlessRenderer::new()
            .with_strict_parsing()
            .with_diagram_id(diagram_id);
        let Some(svg) = renderer.render_svg_sync(source)? else {
            return Err(format!("no Mermaid diagram detected for {diagram_id}").into());
        };
        let path = out_dir.join(format!("{diagram_id}.svg"));
        fs::write(&path, svg)?;
        eprintln!("wrote {}", path.display());
    }

    Ok(())
}
