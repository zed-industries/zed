mod support;

use merman::render::{
    HeadlessRenderer,
    raster::{RasterFitBox, RasterOptions},
};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/merman-raster-example.png"));

    let input = support::read_mermaid_or_default(
        "example_05_raster_output",
        r#"flowchart TD
    A[Write Mermaid] --> B[Render SVG]
    B --> C[Raster PNG]
"#,
    )?;

    // Bound raster output explicitly; SVG viewBoxes can be much larger than preview surfaces.
    let raster = RasterOptions::default()
        .with_fit_to(RasterFitBox::contain(960, 540))
        .with_scale(2.0)
        .with_background("white");
    let renderer = HeadlessRenderer::new()
        .with_strict_parsing()
        .with_diagram_id("raster-output-example");
    let Some(bytes) = renderer.render_png_sync(&input, &raster)? else {
        return Err("no Mermaid diagram detected".into());
    };

    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&output, bytes)?;
    eprintln!("wrote {}", output.display());
    Ok(())
}
