mod support;

use merman::render::HeadlessRenderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This uses a Mermaid init directive; host-owned defaults should use `with_site_config`.
    let input = support::read_mermaid_or_default(
        "example_07_theme_css",
        r##"%%{init: {"theme": "base", "themeVariables": {"primaryColor": "#e0f2fe", "primaryBorderColor": "#0284c7", "primaryTextColor": "#0f172a", "lineColor": "#16a34a"}, "themeCSS": ".node rect { stroke-width: 3px; } .edgeLabel { font-weight: 600; }"}}%%
flowchart TD
    A[Theme variables] -->|plus themeCSS| B[Headless SVG]
"##,
    )?;

    let renderer = HeadlessRenderer::new()
        .with_strict_parsing()
        .with_diagram_id("theme-css-example");
    let Some(svg) = renderer.render_svg_sync(&input)? else {
        return Err("no Mermaid diagram detected".into());
    };

    print!("{svg}");
    Ok(())
}
