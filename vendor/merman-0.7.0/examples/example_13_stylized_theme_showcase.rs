mod support;

use merman::render::{HeadlessRenderer, HostThemeOutput, HostThemePreset, HostThemeProfile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = support::read_mermaid_or_default(
        "example_13_stylized_theme_showcase",
        r#"flowchart TD
    subgraph Editor["Editor Preview"]
      A[Parse Mermaid] --> B{Theme Profile}
      B -->|preset| C[Common editor themes]
      B -->|custom CSS| D[Stylized showcase]
    end
    C --> E[Resvg-safe SVG]
    D --> E
"#,
    )?;

    let mut profile = HostThemeProfile::from_preset(HostThemePreset::AyuDark);
    profile.output = {
        let mut output = HostThemeOutput::resvg_safe_editor();
        output.scoped_css = Some(
            r#"
.node rect,
.node circle,
.node polygon,
.node path {
  stroke-width: 2px;
  filter: drop-shadow(0 0 6px rgba(89, 194, 255, 0.45));
}
.node .label,
.cluster-label {
  letter-spacing: 0.02em;
}
.flowchart-link {
  stroke-width: 2.5px;
  filter: drop-shadow(0 0 4px rgba(255, 180, 84, 0.5));
}
.cluster rect {
  fill: rgba(31, 36, 48, 0.82);
  stroke: #ffb454;
  stroke-dasharray: 6 4;
}
.edgeLabel rect {
  fill: #0b0e14;
  opacity: 0.92;
}
.merman-foreignobject-fallback-text {
  fill: #e6f1ff;
  font-weight: 700;
}
"#
            .to_string(),
        );
        output
    };

    let renderer = HeadlessRenderer::new()
        .with_host_theme(&profile)
        .with_vendored_text_measurer()
        .with_diagram_id("stylized-theme-showcase");

    let Some(svg) = renderer.render_svg_sync(&input)? else {
        return Err("no Mermaid diagram detected".into());
    };

    let document = roxmltree::Document::parse(&svg)?;
    if document
        .descendants()
        .any(|node| node.is_element() && node.tag_name().name() == "foreignObject")
    {
        return Err("stylized theme showcase should not emit duplicate native HTML labels".into());
    }

    print!("{svg}");
    Ok(())
}
