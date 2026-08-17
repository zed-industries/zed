use super::*;

// Error diagram SVG renderer implementation (split from parity.rs).

pub(super) fn render_error_diagram_svg(
    layout: &ErrorDiagramLayout,
    _semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let diagram_id = options.diagram_id.as_deref().unwrap_or("merman");

    let mut out = String::new();
    let viewbox_attr = format!(
        "0 0 {} {}",
        fmt(layout.viewbox_width),
        fmt(layout.viewbox_height)
    );
    let style_attr = format!("max-width: {}px;", fmt(layout.max_width_px));
    root_svg::push_svg_root_open(
        &mut out,
        root_svg::SvgRootAttrs {
            width: root_svg::SvgRootWidth::Percent100,
            style_attr: Some(style_attr.as_str()),
            viewbox_attr: Some(viewbox_attr.as_str()),
            style_viewbox_order: root_svg::SvgRootStyleViewBoxOrder::ViewBoxThenStyle,
            trailing_newline: false,
            ..root_svg::SvgRootAttrs::new(diagram_id, "error")
        },
    );
    let css = info_css_with_config(diagram_id, effective_config);
    let _ = write!(
        &mut out,
        r#"<style xmlns="http://www.w3.org/1999/xhtml">{}</style>"#,
        css
    );
    out.push_str(r#"<g/>"#);
    out.push_str(r#"<g>"#);
    out.push_str(r#"<path class="error-icon" d="m411.313,123.313c6.25-6.25 6.25-16.375 0-22.625s-16.375-6.25-22.625,0l-32,32-9.375,9.375-20.688-20.688c-12.484-12.5-32.766-12.5-45.25,0l-16,16c-1.261,1.261-2.304,2.648-3.31,4.051-21.739-8.561-45.324-13.426-70.065-13.426-105.867,0-192,86.133-192,192s86.133,192 192,192 192-86.133 192-192c0-24.741-4.864-48.327-13.426-70.065 1.402-1.007 2.79-2.049 4.051-3.31l16-16c12.5-12.492 12.5-32.758 0-45.25l-20.688-20.688 9.375-9.375 32.001-31.999zm-219.313,100.687c-52.938,0-96,43.063-96,96 0,8.836-7.164,16-16,16s-16-7.164-16-16c0-70.578 57.422-128 128-128 8.836,0 16,7.164 16,16s-7.164,16-16,16z"/>"#);
    out.push_str(r#"<path class="error-icon" d="m459.02,148.98c-6.25-6.25-16.375-6.25-22.625,0s-6.25,16.375 0,22.625l16,16c3.125,3.125 7.219,4.688 11.313,4.688 4.094,0 8.188-1.563 11.313-4.688 6.25-6.25 6.25-16.375 0-22.625l-16.001-16z"/>"#);
    out.push_str(r#"<path class="error-icon" d="m340.395,75.605c3.125,3.125 7.219,4.688 11.313,4.688 4.094,0 8.188-1.563 11.313-4.688 6.25-6.25 6.25-16.375 0-22.625l-16-16c-6.25-6.25-16.375-6.25-22.625,0s-6.25,16.375 0,22.625l15.999,16z"/>"#);
    out.push_str(r#"<path class="error-icon" d="m400,64c8.844,0 16-7.164 16-16v-32c0-8.836-7.156-16-16-16-8.844,0-16,7.164-16,16v32c0,8.836 7.156,16 16,16z"/>"#);
    out.push_str(r#"<path class="error-icon" d="m496,96.586h-32c-8.844,0-16,7.164-16,16 0,8.836 7.156,16 16,16h32c8.844,0 16-7.164 16-16 0-8.836-7.156-16-16-16z"/>"#);
    out.push_str(r#"<path class="error-icon" d="m436.98,75.605c3.125,3.125 7.219,4.688 11.313,4.688 4.094,0 8.188-1.563 11.313-4.688l32-32c6.25-6.25 6.25-16.375 0-22.625s-16.375-6.25-22.625,0l-32,32c-6.251,6.25-6.251,16.375-0.001,22.625z"/>"#);
    out.push_str(r#"<text class="error-text" x="1440" y="250" font-size="150px" style="text-anchor: middle;">Syntax error in text</text>"#);
    let _ = write!(
        &mut out,
        r#"<text class="error-text" x="1250" y="400" font-size="100px" style="text-anchor: middle;">mermaid version {}</text>"#,
        crate::error::UPSTREAM_MERMAID_VERSION
    );
    out.push_str("</g></svg>\n");
    Ok(out)
}
